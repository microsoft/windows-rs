#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct CompositionConditionalValue(::windows::core::IUnknown);
impl CompositionConditionalValue {
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).PopulatePropertyInfo)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Condition(&self) -> ::windows::core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Condition)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetCondition<'a, Param0: ::windows::core::IntoParam<'a, super::ExpressionAnimation>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCondition)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Value(&self) -> ::windows::core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Value)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, super::ExpressionAnimation>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetValue)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::Compositor>>(compositor: Param0) -> ::windows::core::Result<CompositionConditionalValue> {
        Self::ICompositionConditionalValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), compositor.into_param().abi(), &mut result__).from_abi::<CompositionConditionalValue>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Compositor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Dispatcher)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StartAnimation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StartAnimation)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StopAnimation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, propertyname: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopAnimation)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Comment)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetComment<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetComment)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ImplicitAnimations)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::core::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetImplicitAnimations)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StartAnimationGroup)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopAnimationGroup)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, propertyname: Param0) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryGetAnimationController)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc(hidden)]
    pub fn ICompositionConditionalValueStatics<R, F: FnOnce(&ICompositionConditionalValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CompositionConditionalValue, ICompositionConditionalValueStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CompositionConditionalValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CompositionConditionalValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CompositionConditionalValue {}
impl ::core::fmt::Debug for CompositionConditionalValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompositionConditionalValue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CompositionConditionalValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.CompositionConditionalValue;{43250538-eb73-4561-a71d-1a43eaeb7a9b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CompositionConditionalValue {
    type Vtable = ICompositionConditionalValue_Vtbl;
    const IID: ::windows::core::GUID = <ICompositionConditionalValue as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CompositionConditionalValue {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.CompositionConditionalValue";
}
impl ::core::convert::From<CompositionConditionalValue> for ::windows::core::IUnknown {
    fn from(value: CompositionConditionalValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompositionConditionalValue> for ::windows::core::IUnknown {
    fn from(value: &CompositionConditionalValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CompositionConditionalValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CompositionConditionalValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CompositionConditionalValue> for ::windows::core::IInspectable {
    fn from(value: CompositionConditionalValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompositionConditionalValue> for ::windows::core::IInspectable {
    fn from(value: &CompositionConditionalValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CompositionConditionalValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CompositionConditionalValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<CompositionConditionalValue> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: CompositionConditionalValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CompositionConditionalValue> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &CompositionConditionalValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject> for CompositionConditionalValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject> for &CompositionConditionalValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CompositionConditionalValue> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CompositionConditionalValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CompositionConditionalValue> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CompositionConditionalValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for CompositionConditionalValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &CompositionConditionalValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<CompositionConditionalValue> for super::CompositionObject {
    fn from(value: CompositionConditionalValue) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CompositionConditionalValue> for super::CompositionObject {
    fn from(value: &CompositionConditionalValue) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject> for CompositionConditionalValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject> for &CompositionConditionalValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for CompositionConditionalValue {}
unsafe impl ::core::marker::Sync for CompositionConditionalValue {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct CompositionInteractionSourceCollection(::windows::core::IUnknown);
impl CompositionInteractionSourceCollection {
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).PopulatePropertyInfo)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Count(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Count)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Add<'a, Param0: ::windows::core::IntoParam<'a, ICompositionInteractionSource>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Add)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Remove<'a, Param0: ::windows::core::IntoParam<'a, ICompositionInteractionSource>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Remove)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn RemoveAll(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAll)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Compositor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Dispatcher)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StartAnimation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StartAnimation)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StopAnimation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, propertyname: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopAnimation)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Comment)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetComment<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetComment)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ImplicitAnimations)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::core::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetImplicitAnimations)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StartAnimationGroup)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopAnimationGroup)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, propertyname: Param0) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryGetAnimationController)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<ICompositionInteractionSource>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<ICompositionInteractionSource>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).First)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<ICompositionInteractionSource>>(result__)
        }
    }
}
impl ::core::clone::Clone for CompositionInteractionSourceCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CompositionInteractionSourceCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CompositionInteractionSourceCollection {}
impl ::core::fmt::Debug for CompositionInteractionSourceCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompositionInteractionSourceCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CompositionInteractionSourceCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.CompositionInteractionSourceCollection;{1b468e4b-a5bf-47d8-a547-3894155a158c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CompositionInteractionSourceCollection {
    type Vtable = ICompositionInteractionSourceCollection_Vtbl;
    const IID: ::windows::core::GUID = <ICompositionInteractionSourceCollection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CompositionInteractionSourceCollection {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.CompositionInteractionSourceCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for CompositionInteractionSourceCollection {
    type Item = ICompositionInteractionSource;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &CompositionInteractionSourceCollection {
    type Item = ICompositionInteractionSource;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl ::core::convert::From<CompositionInteractionSourceCollection> for ::windows::core::IUnknown {
    fn from(value: CompositionInteractionSourceCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompositionInteractionSourceCollection> for ::windows::core::IUnknown {
    fn from(value: &CompositionInteractionSourceCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CompositionInteractionSourceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CompositionInteractionSourceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CompositionInteractionSourceCollection> for ::windows::core::IInspectable {
    fn from(value: CompositionInteractionSourceCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompositionInteractionSourceCollection> for ::windows::core::IInspectable {
    fn from(value: &CompositionInteractionSourceCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CompositionInteractionSourceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CompositionInteractionSourceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<CompositionInteractionSourceCollection> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: CompositionInteractionSourceCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CompositionInteractionSourceCollection> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &CompositionInteractionSourceCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject> for CompositionInteractionSourceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject> for &CompositionInteractionSourceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CompositionInteractionSourceCollection> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CompositionInteractionSourceCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CompositionInteractionSourceCollection> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CompositionInteractionSourceCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for CompositionInteractionSourceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &CompositionInteractionSourceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<CompositionInteractionSourceCollection> for super::super::super::Foundation::Collections::IIterable<ICompositionInteractionSource> {
    type Error = ::windows::core::Error;
    fn try_from(value: CompositionInteractionSourceCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&CompositionInteractionSourceCollection> for super::super::super::Foundation::Collections::IIterable<ICompositionInteractionSource> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CompositionInteractionSourceCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<ICompositionInteractionSource>> for CompositionInteractionSourceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<ICompositionInteractionSource>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<ICompositionInteractionSource>> for &CompositionInteractionSourceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<ICompositionInteractionSource>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<ICompositionInteractionSource>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<CompositionInteractionSourceCollection> for super::CompositionObject {
    fn from(value: CompositionInteractionSourceCollection) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CompositionInteractionSourceCollection> for super::CompositionObject {
    fn from(value: &CompositionInteractionSourceCollection) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject> for CompositionInteractionSourceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject> for &CompositionInteractionSourceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for CompositionInteractionSourceCollection {}
unsafe impl ::core::marker::Sync for CompositionInteractionSourceCollection {}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompositionConditionalValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICompositionConditionalValue {
    type Vtable = ICompositionConditionalValue_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43250538_eb73_4561_a71d_1a43eaeb7a9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionConditionalValue_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Condition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompositionConditionalValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICompositionConditionalValueStatics {
    type Vtable = ICompositionConditionalValueStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x090c4b72_8467_4d0a_9065_ac46b80a5522);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionConditionalValueStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct ICompositionInteractionSource(::windows::core::IUnknown);
impl ICompositionInteractionSource {}
impl ::core::convert::From<ICompositionInteractionSource> for ::windows::core::IUnknown {
    fn from(value: ICompositionInteractionSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICompositionInteractionSource> for ::windows::core::IUnknown {
    fn from(value: &ICompositionInteractionSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICompositionInteractionSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICompositionInteractionSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICompositionInteractionSource> for ::windows::core::IInspectable {
    fn from(value: ICompositionInteractionSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICompositionInteractionSource> for ::windows::core::IInspectable {
    fn from(value: &ICompositionInteractionSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICompositionInteractionSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ICompositionInteractionSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICompositionInteractionSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICompositionInteractionSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICompositionInteractionSource {}
impl ::core::fmt::Debug for ICompositionInteractionSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICompositionInteractionSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICompositionInteractionSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{043b2431-06e3-495a-ba54-409f0017fac0}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ICompositionInteractionSource {
    type Vtable = ICompositionInteractionSource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x043b2431_06e3_495a_ba54_409f0017fac0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionInteractionSource_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompositionInteractionSourceCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICompositionInteractionSourceCollection {
    type Vtable = ICompositionInteractionSourceCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b468e4b_a5bf_47d8_a547_3894155a158c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionInteractionSourceCollection_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionSourceConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInteractionSourceConfiguration {
    type Vtable = IInteractionSourceConfiguration_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa78347e5_a9d1_4d02_985e_b930cd0b9da4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionSourceConfiguration_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PositionXSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceRedirectionMode) -> ::windows::core::HRESULT,
    pub SetPositionXSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InteractionSourceRedirectionMode) -> ::windows::core::HRESULT,
    pub PositionYSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceRedirectionMode) -> ::windows::core::HRESULT,
    pub SetPositionYSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InteractionSourceRedirectionMode) -> ::windows::core::HRESULT,
    pub ScaleSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceRedirectionMode) -> ::windows::core::HRESULT,
    pub SetScaleSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InteractionSourceRedirectionMode) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTracker(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInteractionTracker {
    type Vtable = IInteractionTracker_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a8e8cb1_1000_4416_8363_cc27fb877308);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub InteractionSources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub IsPositionRoundingSuggested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub MaxPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    MaxPosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetMaxPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetMaxPosition: usize,
    pub MaxScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetMaxScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub MinPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    MinPosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetMinPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetMinPosition: usize,
    pub MinScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetMinScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub NaturalRestingPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    NaturalRestingPosition: usize,
    pub NaturalRestingScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub Owner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Position: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub PositionInertiaDecayRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PositionInertiaDecayRate: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetPositionInertiaDecayRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetPositionInertiaDecayRate: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub PositionVelocityInPixelsPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PositionVelocityInPixelsPerSecond: usize,
    pub Scale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ScaleInertiaDecayRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScaleInertiaDecayRate: usize,
    #[cfg(feature = "Foundation")]
    pub SetScaleInertiaDecayRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetScaleInertiaDecayRate: usize,
    pub ScaleVelocityInPercentPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub AdjustPositionXIfGreaterThanThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adjustment: f32, positionthreshold: f32) -> ::windows::core::HRESULT,
    pub AdjustPositionYIfGreaterThanThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adjustment: f32, positionthreshold: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigurePositionXInertiaModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modifiers: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigurePositionXInertiaModifiers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigurePositionYInertiaModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modifiers: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigurePositionYInertiaModifiers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigureScaleInertiaModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modifiers: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigureScaleInertiaModifiers: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryUpdatePosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryUpdatePosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryUpdatePositionBy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amount: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryUpdatePositionBy: usize,
    pub TryUpdatePositionWithAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryUpdatePositionWithAdditionalVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, velocityinpixelspersecond: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryUpdatePositionWithAdditionalVelocity: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryUpdateScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32, centerpoint: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryUpdateScale: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryUpdateScaleWithAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr, centerpoint: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryUpdateScaleWithAnimation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryUpdateScaleWithAdditionalVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, velocityinpercentpersecond: f32, centerpoint: super::super::super::Foundation::Numerics::Vector3, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryUpdateScaleWithAdditionalVelocity: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTracker2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInteractionTracker2 {
    type Vtable = IInteractionTracker2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25769a3e_ce6d_448c_8386_92620d240756);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigureCenterPointXInertiaModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conditionalvalues: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigureCenterPointXInertiaModifiers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigureCenterPointYInertiaModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conditionalvalues: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigureCenterPointYInertiaModifiers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTracker3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInteractionTracker3 {
    type Vtable = IInteractionTracker3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6c5d7a2_5c4b_42c6_84b7_f69441b18091);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigureVector2PositionInertiaModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modifiers: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigureVector2PositionInertiaModifiers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTracker4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInteractionTracker4 {
    type Vtable = IInteractionTracker4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xebd222bc_04af_4ac7_847d_06ea36e80a16);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryUpdatePositionWithOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryUpdatePositionWithOption: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryUpdatePositionByWithOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amount: super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryUpdatePositionByWithOption: usize,
    pub IsInertiaFromImpulse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTracker5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInteractionTracker5 {
    type Vtable = IInteractionTracker5_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd3ef5da2_a254_40e4_88d5_44e4e16b5809);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker5_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryUpdatePositionWithOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Vector3, option: InteractionTrackerClampingOption, posupdateoption: InteractionTrackerPositionUpdateOption, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryUpdatePositionWithOption: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInteractionTrackerCustomAnimationStateEnteredArgs {
    type Vtable = IInteractionTrackerCustomAnimationStateEnteredArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d1c8cf1_d7b0_434c_a5d2_2d7611864834);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub RequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInteractionTrackerCustomAnimationStateEnteredArgs2 {
    type Vtable = IInteractionTrackerCustomAnimationStateEnteredArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47d579b7_0985_5e99_b024_2f32c380c1a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsFromBinding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerIdleStateEnteredArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInteractionTrackerIdleStateEnteredArgs {
    type Vtable = IInteractionTrackerIdleStateEnteredArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50012faa_1510_4142_a1a5_019b09f8857b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerIdleStateEnteredArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub RequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerIdleStateEnteredArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInteractionTrackerIdleStateEnteredArgs2 {
    type Vtable = IInteractionTrackerIdleStateEnteredArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e771ed_b803_5137_9435_1c96e48721e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerIdleStateEnteredArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsFromBinding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaModifier(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInteractionTrackerInertiaModifier {
    type Vtable = IInteractionTrackerInertiaModifier_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0e2c920_26b4_4da2_8b61_5e683979bbe2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaModifier_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaModifierFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInteractionTrackerInertiaModifierFactory {
    type Vtable = IInteractionTrackerInertiaModifierFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x993818fe_c94e_4b86_87f3_922665ba46b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaModifierFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaMotion(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInteractionTrackerInertiaMotion {
    type Vtable = IInteractionTrackerInertiaMotion_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04922fdc_f154_4cb8_bf33_cc1ba611e6db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaMotion_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Condition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Motion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetMotion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaMotionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInteractionTrackerInertiaMotionStatics {
    type Vtable = IInteractionTrackerInertiaMotionStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8cc83dd6_ba7b_431a_844b_6eac9130f99a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaMotionStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaNaturalMotion(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInteractionTrackerInertiaNaturalMotion {
    type Vtable = IInteractionTrackerInertiaNaturalMotion_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70acdaae_27dc_48ed_a3c3_6d61c9a029d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaNaturalMotion_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Condition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub NaturalMotion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetNaturalMotion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaNaturalMotionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInteractionTrackerInertiaNaturalMotionStatics {
    type Vtable = IInteractionTrackerInertiaNaturalMotionStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcfda55b0_5e3e_4289_932d_ee5f50e74283);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaNaturalMotionStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaRestingValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInteractionTrackerInertiaRestingValue {
    type Vtable = IInteractionTrackerInertiaRestingValue_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86f7ec09_5096_4170_9cc8_df2fe101bb93);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaRestingValue_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Condition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RestingValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetRestingValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaRestingValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInteractionTrackerInertiaRestingValueStatics {
    type Vtable = IInteractionTrackerInertiaRestingValueStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18ed4699_0745_4096_bcab_3a4e99569bcf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaRestingValueStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaStateEnteredArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInteractionTrackerInertiaStateEnteredArgs {
    type Vtable = IInteractionTrackerInertiaStateEnteredArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87108cf2_e7ff_4f7d_9ffd_d72f1e409b63);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub ModifiedRestingPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ModifiedRestingPosition: usize,
    #[cfg(feature = "Foundation")]
    pub ModifiedRestingScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ModifiedRestingScale: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub NaturalRestingPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    NaturalRestingPosition: usize,
    pub NaturalRestingScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub PositionVelocityInPixelsPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PositionVelocityInPixelsPerSecond: usize,
    pub RequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub ScaleVelocityInPercentPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaStateEnteredArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInteractionTrackerInertiaStateEnteredArgs2 {
    type Vtable = IInteractionTrackerInertiaStateEnteredArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1eb32f6_c26c_41f6_a189_fabc22b323cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsInertiaFromImpulse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInertiaStateEnteredArgs3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInteractionTrackerInertiaStateEnteredArgs3 {
    type Vtable = IInteractionTrackerInertiaStateEnteredArgs3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x48ac1c2f_47bd_59af_a58c_79bd2eb9ef71);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsFromBinding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInteractingStateEnteredArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInteractionTrackerInteractingStateEnteredArgs {
    type Vtable = IInteractionTrackerInteractingStateEnteredArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7263939_a17b_4011_99fd_b5c24f143748);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInteractingStateEnteredArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub RequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerInteractingStateEnteredArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInteractionTrackerInteractingStateEnteredArgs2 {
    type Vtable = IInteractionTrackerInteractingStateEnteredArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x509652d6_d488_59cd_819f_f52310295b11);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInteractingStateEnteredArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsFromBinding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct IInteractionTrackerOwner(::windows::core::IUnknown);
impl IInteractionTrackerOwner {
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn CustomAnimationStateEntered<'a, Param0: ::windows::core::IntoParam<'a, InteractionTracker>, Param1: ::windows::core::IntoParam<'a, InteractionTrackerCustomAnimationStateEnteredArgs>>(&self, sender: Param0, args: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).CustomAnimationStateEntered)(::core::mem::transmute_copy(this), sender.into_param().abi(), args.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn IdleStateEntered<'a, Param0: ::windows::core::IntoParam<'a, InteractionTracker>, Param1: ::windows::core::IntoParam<'a, InteractionTrackerIdleStateEnteredArgs>>(&self, sender: Param0, args: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).IdleStateEntered)(::core::mem::transmute_copy(this), sender.into_param().abi(), args.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn InertiaStateEntered<'a, Param0: ::windows::core::IntoParam<'a, InteractionTracker>, Param1: ::windows::core::IntoParam<'a, InteractionTrackerInertiaStateEnteredArgs>>(&self, sender: Param0, args: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).InertiaStateEntered)(::core::mem::transmute_copy(this), sender.into_param().abi(), args.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn InteractingStateEntered<'a, Param0: ::windows::core::IntoParam<'a, InteractionTracker>, Param1: ::windows::core::IntoParam<'a, InteractionTrackerInteractingStateEnteredArgs>>(&self, sender: Param0, args: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).InteractingStateEntered)(::core::mem::transmute_copy(this), sender.into_param().abi(), args.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn RequestIgnored<'a, Param0: ::windows::core::IntoParam<'a, InteractionTracker>, Param1: ::windows::core::IntoParam<'a, InteractionTrackerRequestIgnoredArgs>>(&self, sender: Param0, args: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RequestIgnored)(::core::mem::transmute_copy(this), sender.into_param().abi(), args.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn ValuesChanged<'a, Param0: ::windows::core::IntoParam<'a, InteractionTracker>, Param1: ::windows::core::IntoParam<'a, InteractionTrackerValuesChangedArgs>>(&self, sender: Param0, args: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ValuesChanged)(::core::mem::transmute_copy(this), sender.into_param().abi(), args.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IInteractionTrackerOwner> for ::windows::core::IUnknown {
    fn from(value: IInteractionTrackerOwner) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInteractionTrackerOwner> for ::windows::core::IUnknown {
    fn from(value: &IInteractionTrackerOwner) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInteractionTrackerOwner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInteractionTrackerOwner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IInteractionTrackerOwner> for ::windows::core::IInspectable {
    fn from(value: IInteractionTrackerOwner) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInteractionTrackerOwner> for ::windows::core::IInspectable {
    fn from(value: &IInteractionTrackerOwner) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IInteractionTrackerOwner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IInteractionTrackerOwner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IInteractionTrackerOwner {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInteractionTrackerOwner {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInteractionTrackerOwner {}
impl ::core::fmt::Debug for IInteractionTrackerOwner {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInteractionTrackerOwner").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IInteractionTrackerOwner {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{db2e8af3-4deb-4e53-b29c-b06c9f96d651}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IInteractionTrackerOwner {
    type Vtable = IInteractionTrackerOwner_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb2e8af3_4deb_4e53_b29c_b06c9f96d651);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerOwner_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CustomAnimationStateEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub IdleStateEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub InertiaStateEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub InteractingStateEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RequestIgnored: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ValuesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerRequestIgnoredArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInteractionTrackerRequestIgnoredArgs {
    type Vtable = IInteractionTrackerRequestIgnoredArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80dd82f1_ce25_488f_91dd_cb6455ccff2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerRequestIgnoredArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub RequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInteractionTrackerStatics {
    type Vtable = IInteractionTrackerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbba5d7b7_6590_4498_8d6c_eb62b514c92a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateWithOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, owner: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInteractionTrackerStatics2 {
    type Vtable = IInteractionTrackerStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x35e53720_46b7_5cb0_b505_f3d6884a6163);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetBindingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, boundtracker1: ::windows::core::RawPtr, boundtracker2: ::windows::core::RawPtr, axismode: InteractionBindingAxisModes) -> ::windows::core::HRESULT,
    pub GetBindingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, boundtracker1: ::windows::core::RawPtr, boundtracker2: ::windows::core::RawPtr, result__: *mut InteractionBindingAxisModes) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerValuesChangedArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInteractionTrackerValuesChangedArgs {
    type Vtable = IInteractionTrackerValuesChangedArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf1578ef_d3df_4501_b9e6_f02fb22f73d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerValuesChangedArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Position: usize,
    pub RequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub Scale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerVector2InertiaModifier(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInteractionTrackerVector2InertiaModifier {
    type Vtable = IInteractionTrackerVector2InertiaModifier_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87e08ab0_3086_4853_a4b7_77882ad5d7e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaModifier_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerVector2InertiaModifierFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInteractionTrackerVector2InertiaModifierFactory {
    type Vtable = IInteractionTrackerVector2InertiaModifierFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7401d6c4_6c6d_48df_bc3e_171e227e7d7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaModifierFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerVector2InertiaNaturalMotion(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInteractionTrackerVector2InertiaNaturalMotion {
    type Vtable = IInteractionTrackerVector2InertiaNaturalMotion_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f17695c_162d_4c07_9400_c282b28276ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaNaturalMotion_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Condition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub NaturalMotion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetNaturalMotion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInteractionTrackerVector2InertiaNaturalMotionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInteractionTrackerVector2InertiaNaturalMotionStatics {
    type Vtable = IInteractionTrackerVector2InertiaNaturalMotionStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82001a48_09c0_434f_8189_141c66df362f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaNaturalMotionStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualInteractionSource(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVisualInteractionSource {
    type Vtable = IVisualInteractionSource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca0e8a86_d8d6_4111_b088_70347bd2b0ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSource_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsPositionXRailsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsPositionXRailsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsPositionYRailsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsPositionYRailsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ManipulationRedirectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VisualInteractionSourceRedirectionMode) -> ::windows::core::HRESULT,
    pub SetManipulationRedirectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: VisualInteractionSourceRedirectionMode) -> ::windows::core::HRESULT,
    pub PositionXChainingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InteractionChainingMode) -> ::windows::core::HRESULT,
    pub SetPositionXChainingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InteractionChainingMode) -> ::windows::core::HRESULT,
    pub PositionXSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceMode) -> ::windows::core::HRESULT,
    pub SetPositionXSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InteractionSourceMode) -> ::windows::core::HRESULT,
    pub PositionYChainingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InteractionChainingMode) -> ::windows::core::HRESULT,
    pub SetPositionYChainingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InteractionChainingMode) -> ::windows::core::HRESULT,
    pub PositionYSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceMode) -> ::windows::core::HRESULT,
    pub SetPositionYSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InteractionSourceMode) -> ::windows::core::HRESULT,
    pub ScaleChainingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InteractionChainingMode) -> ::windows::core::HRESULT,
    pub SetScaleChainingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InteractionChainingMode) -> ::windows::core::HRESULT,
    pub ScaleSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InteractionSourceMode) -> ::windows::core::HRESULT,
    pub SetScaleSourceMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InteractionSourceMode) -> ::windows::core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Input")]
    pub TryRedirectForManipulation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerpoint: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Input"))]
    TryRedirectForManipulation: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualInteractionSource2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVisualInteractionSource2 {
    type Vtable = IVisualInteractionSource2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa914893_a73c_414d_80d0_249bad2fbd93);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSource2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub DeltaPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DeltaPosition: usize,
    pub DeltaScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Position: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub PositionVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PositionVelocity: usize,
    pub Scale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub ScaleVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigureCenterPointXModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conditionalvalues: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigureCenterPointXModifiers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigureCenterPointYModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conditionalvalues: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigureCenterPointYModifiers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigureDeltaPositionXModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conditionalvalues: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigureDeltaPositionXModifiers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigureDeltaPositionYModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conditionalvalues: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigureDeltaPositionYModifiers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ConfigureDeltaScaleModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conditionalvalues: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ConfigureDeltaScaleModifiers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualInteractionSource3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVisualInteractionSource3 {
    type Vtable = IVisualInteractionSource3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd941ef2a_0d5c_4057_92d7_c9711533204f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSource3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PointerWheelConfig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualInteractionSourceObjectFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVisualInteractionSourceObjectFactory {
    type Vtable = IVisualInteractionSourceObjectFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2ca917c_e98a_41f2_b3c9_891c9266c8f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSourceObjectFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualInteractionSourceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVisualInteractionSourceStatics {
    type Vtable = IVisualInteractionSourceStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x369965e1_8645_4f75_ba00_6479cd10c8e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSourceStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualInteractionSourceStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVisualInteractionSourceStatics2 {
    type Vtable = IVisualInteractionSourceStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa979c032_5764_55e0_bc1f_0778786dcfde);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSourceStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateFromIVisualElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InteractionBindingAxisModes(pub u32);
impl InteractionBindingAxisModes {
    pub const None: Self = Self(0u32);
    pub const PositionX: Self = Self(1u32);
    pub const PositionY: Self = Self(2u32);
    pub const Scale: Self = Self(4u32);
}
impl ::core::marker::Copy for InteractionBindingAxisModes {}
impl ::core::clone::Clone for InteractionBindingAxisModes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InteractionBindingAxisModes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for InteractionBindingAxisModes {
    type Abi = Self;
}
impl ::core::fmt::Debug for InteractionBindingAxisModes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionBindingAxisModes").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for InteractionBindingAxisModes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for InteractionBindingAxisModes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for InteractionBindingAxisModes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for InteractionBindingAxisModes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for InteractionBindingAxisModes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionBindingAxisModes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.InteractionBindingAxisModes;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InteractionChainingMode(pub i32);
impl InteractionChainingMode {
    pub const Auto: Self = Self(0i32);
    pub const Always: Self = Self(1i32);
    pub const Never: Self = Self(2i32);
}
impl ::core::marker::Copy for InteractionChainingMode {}
impl ::core::clone::Clone for InteractionChainingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InteractionChainingMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for InteractionChainingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for InteractionChainingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionChainingMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionChainingMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.InteractionChainingMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionSourceConfiguration(::windows::core::IUnknown);
impl InteractionSourceConfiguration {
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).PopulatePropertyInfo)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Compositor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Dispatcher)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StartAnimation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StartAnimation)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StopAnimation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, propertyname: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopAnimation)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Comment)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetComment<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetComment)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ImplicitAnimations)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::core::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetImplicitAnimations)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StartAnimationGroup)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopAnimationGroup)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, propertyname: Param0) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryGetAnimationController)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn PositionXSourceMode(&self) -> ::windows::core::Result<InteractionSourceRedirectionMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionSourceRedirectionMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PositionXSourceMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InteractionSourceRedirectionMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetPositionXSourceMode(&self, value: InteractionSourceRedirectionMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPositionXSourceMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn PositionYSourceMode(&self) -> ::windows::core::Result<InteractionSourceRedirectionMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionSourceRedirectionMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PositionYSourceMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InteractionSourceRedirectionMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetPositionYSourceMode(&self, value: InteractionSourceRedirectionMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPositionYSourceMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn ScaleSourceMode(&self) -> ::windows::core::Result<InteractionSourceRedirectionMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionSourceRedirectionMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ScaleSourceMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InteractionSourceRedirectionMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetScaleSourceMode(&self, value: InteractionSourceRedirectionMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetScaleSourceMode)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for InteractionSourceConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InteractionSourceConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionSourceConfiguration {}
impl ::core::fmt::Debug for InteractionSourceConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionSourceConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionSourceConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionSourceConfiguration;{a78347e5-a9d1-4d02-985e-b930cd0b9da4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for InteractionSourceConfiguration {
    type Vtable = IInteractionSourceConfiguration_Vtbl;
    const IID: ::windows::core::GUID = <IInteractionSourceConfiguration as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InteractionSourceConfiguration {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionSourceConfiguration";
}
impl ::core::convert::From<InteractionSourceConfiguration> for ::windows::core::IUnknown {
    fn from(value: InteractionSourceConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionSourceConfiguration> for ::windows::core::IUnknown {
    fn from(value: &InteractionSourceConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InteractionSourceConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InteractionSourceConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InteractionSourceConfiguration> for ::windows::core::IInspectable {
    fn from(value: InteractionSourceConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionSourceConfiguration> for ::windows::core::IInspectable {
    fn from(value: &InteractionSourceConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InteractionSourceConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InteractionSourceConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<InteractionSourceConfiguration> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionSourceConfiguration) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionSourceConfiguration> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionSourceConfiguration) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject> for InteractionSourceConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject> for &InteractionSourceConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<InteractionSourceConfiguration> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionSourceConfiguration) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&InteractionSourceConfiguration> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionSourceConfiguration) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for InteractionSourceConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &InteractionSourceConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<InteractionSourceConfiguration> for super::CompositionObject {
    fn from(value: InteractionSourceConfiguration) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InteractionSourceConfiguration> for super::CompositionObject {
    fn from(value: &InteractionSourceConfiguration) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject> for InteractionSourceConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject> for &InteractionSourceConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for InteractionSourceConfiguration {}
unsafe impl ::core::marker::Sync for InteractionSourceConfiguration {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InteractionSourceMode(pub i32);
impl InteractionSourceMode {
    pub const Disabled: Self = Self(0i32);
    pub const EnabledWithInertia: Self = Self(1i32);
    pub const EnabledWithoutInertia: Self = Self(2i32);
}
impl ::core::marker::Copy for InteractionSourceMode {}
impl ::core::clone::Clone for InteractionSourceMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InteractionSourceMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for InteractionSourceMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for InteractionSourceMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionSourceMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionSourceMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.InteractionSourceMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InteractionSourceRedirectionMode(pub i32);
impl InteractionSourceRedirectionMode {
    pub const Disabled: Self = Self(0i32);
    pub const Enabled: Self = Self(1i32);
}
impl ::core::marker::Copy for InteractionSourceRedirectionMode {}
impl ::core::clone::Clone for InteractionSourceRedirectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InteractionSourceRedirectionMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for InteractionSourceRedirectionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for InteractionSourceRedirectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionSourceRedirectionMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionSourceRedirectionMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.InteractionSourceRedirectionMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTracker(::windows::core::IUnknown);
impl InteractionTracker {
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).PopulatePropertyInfo)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Compositor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Dispatcher)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StartAnimation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StartAnimation)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StopAnimation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, propertyname: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopAnimation)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Comment)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetComment<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetComment)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ImplicitAnimations)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::core::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetImplicitAnimations)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StartAnimationGroup)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopAnimationGroup)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, propertyname: Param0) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryGetAnimationController)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn InteractionSources(&self) -> ::windows::core::Result<CompositionInteractionSourceCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InteractionSources)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CompositionInteractionSourceCollection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn IsPositionRoundingSuggested(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPositionRoundingSuggested)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn MaxPosition(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaxPosition)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetMaxPosition<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMaxPosition)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn MaxScale(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaxScale)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetMaxScale(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMaxScale)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn MinPosition(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MinPosition)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetMinPosition<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMinPosition)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn MinScale(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MinScale)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetMinScale(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMinScale)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn NaturalRestingPosition(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NaturalRestingPosition)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn NaturalRestingScale(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NaturalRestingScale)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Owner(&self) -> ::windows::core::Result<IInteractionTrackerOwner> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Owner)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IInteractionTrackerOwner>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PositionInertiaDecayRate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PositionInertiaDecayRate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetPositionInertiaDecayRate<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPositionInertiaDecayRate)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PositionVelocityInPixelsPerSecond(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PositionVelocityInPixelsPerSecond)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Scale(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Scale)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ScaleInertiaDecayRate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ScaleInertiaDecayRate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<f32>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetScaleInertiaDecayRate<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<f32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetScaleInertiaDecayRate)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn ScaleVelocityInPercentPerSecond(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ScaleVelocityInPercentPerSecond)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn AdjustPositionXIfGreaterThanThreshold(&self, adjustment: f32, positionthreshold: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AdjustPositionXIfGreaterThanThreshold)(::core::mem::transmute_copy(this), adjustment, positionthreshold).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn AdjustPositionYIfGreaterThanThreshold(&self, adjustment: f32, positionthreshold: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AdjustPositionYIfGreaterThanThreshold)(::core::mem::transmute_copy(this), adjustment, positionthreshold).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigurePositionXInertiaModifiers<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier>>>(&self, modifiers: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ConfigurePositionXInertiaModifiers)(::core::mem::transmute_copy(this), modifiers.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigurePositionYInertiaModifiers<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier>>>(&self, modifiers: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ConfigurePositionYInertiaModifiers)(::core::mem::transmute_copy(this), modifiers.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigureScaleInertiaModifiers<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier>>>(&self, modifiers: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ConfigureScaleInertiaModifiers)(::core::mem::transmute_copy(this), modifiers.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryUpdatePosition<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryUpdatePosition)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryUpdatePositionBy<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, amount: Param0) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryUpdatePositionBy)(::core::mem::transmute_copy(this), amount.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn TryUpdatePositionWithAnimation<'a, Param0: ::windows::core::IntoParam<'a, super::CompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryUpdatePositionWithAnimation)(::core::mem::transmute_copy(this), animation.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryUpdatePositionWithAdditionalVelocity<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, velocityinpixelspersecond: Param0) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryUpdatePositionWithAdditionalVelocity)(::core::mem::transmute_copy(this), velocityinpixelspersecond.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryUpdateScale<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, value: f32, centerpoint: Param1) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryUpdateScale)(::core::mem::transmute_copy(this), value, centerpoint.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryUpdateScaleWithAnimation<'a, Param0: ::windows::core::IntoParam<'a, super::CompositionAnimation>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, animation: Param0, centerpoint: Param1) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryUpdateScaleWithAnimation)(::core::mem::transmute_copy(this), animation.into_param().abi(), centerpoint.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryUpdateScaleWithAdditionalVelocity<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, velocityinpercentpersecond: f32, centerpoint: Param1) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryUpdateScaleWithAdditionalVelocity)(::core::mem::transmute_copy(this), velocityinpercentpersecond, centerpoint.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigureCenterPointXInertiaModifiers<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>>(&self, conditionalvalues: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInteractionTracker2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ConfigureCenterPointXInertiaModifiers)(::core::mem::transmute_copy(this), conditionalvalues.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigureCenterPointYInertiaModifiers<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>>(&self, conditionalvalues: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInteractionTracker2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ConfigureCenterPointYInertiaModifiers)(::core::mem::transmute_copy(this), conditionalvalues.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigureVector2PositionInertiaModifiers<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<InteractionTrackerVector2InertiaModifier>>>(&self, modifiers: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInteractionTracker3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ConfigureVector2PositionInertiaModifiers)(::core::mem::transmute_copy(this), modifiers.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryUpdatePositionWithOption<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, value: Param0, option: InteractionTrackerClampingOption) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IInteractionTracker4>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryUpdatePositionWithOption)(::core::mem::transmute_copy(this), value.into_param().abi(), option, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryUpdatePositionByWithOption<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, amount: Param0, option: InteractionTrackerClampingOption) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IInteractionTracker4>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryUpdatePositionByWithOption)(::core::mem::transmute_copy(this), amount.into_param().abi(), option, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn IsInertiaFromImpulse(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInteractionTracker4>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsInertiaFromImpulse)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryUpdatePositionWithOption2<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, value: Param0, option: InteractionTrackerClampingOption, posupdateoption: InteractionTrackerPositionUpdateOption) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IInteractionTracker5>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryUpdatePositionWithOption)(::core::mem::transmute_copy(this), value.into_param().abi(), option, posupdateoption, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::Compositor>>(compositor: Param0) -> ::windows::core::Result<InteractionTracker> {
        Self::IInteractionTrackerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), compositor.into_param().abi(), &mut result__).from_abi::<InteractionTracker>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn CreateWithOwner<'a, Param0: ::windows::core::IntoParam<'a, super::Compositor>, Param1: ::windows::core::IntoParam<'a, IInteractionTrackerOwner>>(compositor: Param0, owner: Param1) -> ::windows::core::Result<InteractionTracker> {
        Self::IInteractionTrackerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithOwner)(::core::mem::transmute_copy(this), compositor.into_param().abi(), owner.into_param().abi(), &mut result__).from_abi::<InteractionTracker>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetBindingMode<'a, Param0: ::windows::core::IntoParam<'a, InteractionTracker>, Param1: ::windows::core::IntoParam<'a, InteractionTracker>>(boundtracker1: Param0, boundtracker2: Param1, axismode: InteractionBindingAxisModes) -> ::windows::core::Result<()> {
        Self::IInteractionTrackerStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).SetBindingMode)(::core::mem::transmute_copy(this), boundtracker1.into_param().abi(), boundtracker2.into_param().abi(), axismode).ok() })
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn GetBindingMode<'a, Param0: ::windows::core::IntoParam<'a, InteractionTracker>, Param1: ::windows::core::IntoParam<'a, InteractionTracker>>(boundtracker1: Param0, boundtracker2: Param1) -> ::windows::core::Result<InteractionBindingAxisModes> {
        Self::IInteractionTrackerStatics2(|this| unsafe {
            let mut result__: InteractionBindingAxisModes = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetBindingMode)(::core::mem::transmute_copy(this), boundtracker1.into_param().abi(), boundtracker2.into_param().abi(), &mut result__).from_abi::<InteractionBindingAxisModes>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInteractionTrackerStatics<R, F: FnOnce(&IInteractionTrackerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InteractionTracker, IInteractionTrackerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IInteractionTrackerStatics2<R, F: FnOnce(&IInteractionTrackerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InteractionTracker, IInteractionTrackerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for InteractionTracker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InteractionTracker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTracker {}
impl ::core::fmt::Debug for InteractionTracker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTracker").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionTracker {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTracker;{2a8e8cb1-1000-4416-8363-cc27fb877308})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for InteractionTracker {
    type Vtable = IInteractionTracker_Vtbl;
    const IID: ::windows::core::GUID = <IInteractionTracker as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InteractionTracker {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTracker";
}
impl ::core::convert::From<InteractionTracker> for ::windows::core::IUnknown {
    fn from(value: InteractionTracker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTracker> for ::windows::core::IUnknown {
    fn from(value: &InteractionTracker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InteractionTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InteractionTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InteractionTracker> for ::windows::core::IInspectable {
    fn from(value: InteractionTracker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTracker> for ::windows::core::IInspectable {
    fn from(value: &InteractionTracker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InteractionTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InteractionTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<InteractionTracker> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTracker) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTracker> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTracker) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject> for InteractionTracker {
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject> for &InteractionTracker {
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<InteractionTracker> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTracker) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&InteractionTracker> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTracker) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for InteractionTracker {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &InteractionTracker {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<InteractionTracker> for super::CompositionObject {
    fn from(value: InteractionTracker) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InteractionTracker> for super::CompositionObject {
    fn from(value: &InteractionTracker) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject> for InteractionTracker {
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject> for &InteractionTracker {
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for InteractionTracker {}
unsafe impl ::core::marker::Sync for InteractionTracker {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InteractionTrackerClampingOption(pub i32);
impl InteractionTrackerClampingOption {
    pub const Auto: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
}
impl ::core::marker::Copy for InteractionTrackerClampingOption {}
impl ::core::clone::Clone for InteractionTrackerClampingOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InteractionTrackerClampingOption {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for InteractionTrackerClampingOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for InteractionTrackerClampingOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerClampingOption").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionTrackerClampingOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.InteractionTrackerClampingOption;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerCustomAnimationStateEnteredArgs(::windows::core::IUnknown);
impl InteractionTrackerCustomAnimationStateEnteredArgs {
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn RequestId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn IsFromBinding(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInteractionTrackerCustomAnimationStateEnteredArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsFromBinding)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for InteractionTrackerCustomAnimationStateEnteredArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InteractionTrackerCustomAnimationStateEnteredArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTrackerCustomAnimationStateEnteredArgs {}
impl ::core::fmt::Debug for InteractionTrackerCustomAnimationStateEnteredArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerCustomAnimationStateEnteredArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionTrackerCustomAnimationStateEnteredArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerCustomAnimationStateEnteredArgs;{8d1c8cf1-d7b0-434c-a5d2-2d7611864834})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for InteractionTrackerCustomAnimationStateEnteredArgs {
    type Vtable = IInteractionTrackerCustomAnimationStateEnteredArgs_Vtbl;
    const IID: ::windows::core::GUID = <IInteractionTrackerCustomAnimationStateEnteredArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InteractionTrackerCustomAnimationStateEnteredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerCustomAnimationStateEnteredArgs";
}
impl ::core::convert::From<InteractionTrackerCustomAnimationStateEnteredArgs> for ::windows::core::IUnknown {
    fn from(value: InteractionTrackerCustomAnimationStateEnteredArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerCustomAnimationStateEnteredArgs> for ::windows::core::IUnknown {
    fn from(value: &InteractionTrackerCustomAnimationStateEnteredArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InteractionTrackerCustomAnimationStateEnteredArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InteractionTrackerCustomAnimationStateEnteredArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InteractionTrackerCustomAnimationStateEnteredArgs> for ::windows::core::IInspectable {
    fn from(value: InteractionTrackerCustomAnimationStateEnteredArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerCustomAnimationStateEnteredArgs> for ::windows::core::IInspectable {
    fn from(value: &InteractionTrackerCustomAnimationStateEnteredArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InteractionTrackerCustomAnimationStateEnteredArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InteractionTrackerCustomAnimationStateEnteredArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerCustomAnimationStateEnteredArgs {}
unsafe impl ::core::marker::Sync for InteractionTrackerCustomAnimationStateEnteredArgs {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerIdleStateEnteredArgs(::windows::core::IUnknown);
impl InteractionTrackerIdleStateEnteredArgs {
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn RequestId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn IsFromBinding(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInteractionTrackerIdleStateEnteredArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsFromBinding)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for InteractionTrackerIdleStateEnteredArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InteractionTrackerIdleStateEnteredArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTrackerIdleStateEnteredArgs {}
impl ::core::fmt::Debug for InteractionTrackerIdleStateEnteredArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerIdleStateEnteredArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionTrackerIdleStateEnteredArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerIdleStateEnteredArgs;{50012faa-1510-4142-a1a5-019b09f8857b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for InteractionTrackerIdleStateEnteredArgs {
    type Vtable = IInteractionTrackerIdleStateEnteredArgs_Vtbl;
    const IID: ::windows::core::GUID = <IInteractionTrackerIdleStateEnteredArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InteractionTrackerIdleStateEnteredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerIdleStateEnteredArgs";
}
impl ::core::convert::From<InteractionTrackerIdleStateEnteredArgs> for ::windows::core::IUnknown {
    fn from(value: InteractionTrackerIdleStateEnteredArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerIdleStateEnteredArgs> for ::windows::core::IUnknown {
    fn from(value: &InteractionTrackerIdleStateEnteredArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InteractionTrackerIdleStateEnteredArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InteractionTrackerIdleStateEnteredArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InteractionTrackerIdleStateEnteredArgs> for ::windows::core::IInspectable {
    fn from(value: InteractionTrackerIdleStateEnteredArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerIdleStateEnteredArgs> for ::windows::core::IInspectable {
    fn from(value: &InteractionTrackerIdleStateEnteredArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InteractionTrackerIdleStateEnteredArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InteractionTrackerIdleStateEnteredArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerIdleStateEnteredArgs {}
unsafe impl ::core::marker::Sync for InteractionTrackerIdleStateEnteredArgs {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerInertiaModifier(::windows::core::IUnknown);
impl InteractionTrackerInertiaModifier {
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).PopulatePropertyInfo)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Compositor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Dispatcher)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StartAnimation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StartAnimation)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StopAnimation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, propertyname: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopAnimation)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Comment)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetComment<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetComment)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ImplicitAnimations)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::core::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetImplicitAnimations)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StartAnimationGroup)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopAnimationGroup)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, propertyname: Param0) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryGetAnimationController)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
}
impl ::core::clone::Clone for InteractionTrackerInertiaModifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InteractionTrackerInertiaModifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTrackerInertiaModifier {}
impl ::core::fmt::Debug for InteractionTrackerInertiaModifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerInertiaModifier").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionTrackerInertiaModifier {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerInertiaModifier;{a0e2c920-26b4-4da2-8b61-5e683979bbe2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for InteractionTrackerInertiaModifier {
    type Vtable = IInteractionTrackerInertiaModifier_Vtbl;
    const IID: ::windows::core::GUID = <IInteractionTrackerInertiaModifier as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InteractionTrackerInertiaModifier {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerInertiaModifier";
}
impl ::core::convert::From<InteractionTrackerInertiaModifier> for ::windows::core::IUnknown {
    fn from(value: InteractionTrackerInertiaModifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaModifier> for ::windows::core::IUnknown {
    fn from(value: &InteractionTrackerInertiaModifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InteractionTrackerInertiaModifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InteractionTrackerInertiaModifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InteractionTrackerInertiaModifier> for ::windows::core::IInspectable {
    fn from(value: InteractionTrackerInertiaModifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaModifier> for ::windows::core::IInspectable {
    fn from(value: &InteractionTrackerInertiaModifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InteractionTrackerInertiaModifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InteractionTrackerInertiaModifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<InteractionTrackerInertiaModifier> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTrackerInertiaModifier) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerInertiaModifier> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerInertiaModifier) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject> for InteractionTrackerInertiaModifier {
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject> for &InteractionTrackerInertiaModifier {
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<InteractionTrackerInertiaModifier> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTrackerInertiaModifier) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&InteractionTrackerInertiaModifier> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerInertiaModifier) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for InteractionTrackerInertiaModifier {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &InteractionTrackerInertiaModifier {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<InteractionTrackerInertiaModifier> for super::CompositionObject {
    fn from(value: InteractionTrackerInertiaModifier) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaModifier> for super::CompositionObject {
    fn from(value: &InteractionTrackerInertiaModifier) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject> for InteractionTrackerInertiaModifier {
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject> for &InteractionTrackerInertiaModifier {
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerInertiaModifier {}
unsafe impl ::core::marker::Sync for InteractionTrackerInertiaModifier {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerInertiaMotion(::windows::core::IUnknown);
impl InteractionTrackerInertiaMotion {
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).PopulatePropertyInfo)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Compositor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Dispatcher)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StartAnimation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StartAnimation)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StopAnimation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, propertyname: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopAnimation)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Comment)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetComment<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetComment)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ImplicitAnimations)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::core::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetImplicitAnimations)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StartAnimationGroup)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopAnimationGroup)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, propertyname: Param0) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryGetAnimationController)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Condition(&self) -> ::windows::core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Condition)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetCondition<'a, Param0: ::windows::core::IntoParam<'a, super::ExpressionAnimation>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCondition)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Motion(&self) -> ::windows::core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Motion)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetMotion<'a, Param0: ::windows::core::IntoParam<'a, super::ExpressionAnimation>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMotion)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::Compositor>>(compositor: Param0) -> ::windows::core::Result<InteractionTrackerInertiaMotion> {
        Self::IInteractionTrackerInertiaMotionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), compositor.into_param().abi(), &mut result__).from_abi::<InteractionTrackerInertiaMotion>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInteractionTrackerInertiaMotionStatics<R, F: FnOnce(&IInteractionTrackerInertiaMotionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InteractionTrackerInertiaMotion, IInteractionTrackerInertiaMotionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for InteractionTrackerInertiaMotion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InteractionTrackerInertiaMotion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTrackerInertiaMotion {}
impl ::core::fmt::Debug for InteractionTrackerInertiaMotion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerInertiaMotion").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionTrackerInertiaMotion {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerInertiaMotion;{04922fdc-f154-4cb8-bf33-cc1ba611e6db})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for InteractionTrackerInertiaMotion {
    type Vtable = IInteractionTrackerInertiaMotion_Vtbl;
    const IID: ::windows::core::GUID = <IInteractionTrackerInertiaMotion as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InteractionTrackerInertiaMotion {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerInertiaMotion";
}
impl ::core::convert::From<InteractionTrackerInertiaMotion> for ::windows::core::IUnknown {
    fn from(value: InteractionTrackerInertiaMotion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaMotion> for ::windows::core::IUnknown {
    fn from(value: &InteractionTrackerInertiaMotion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InteractionTrackerInertiaMotion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InteractionTrackerInertiaMotion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InteractionTrackerInertiaMotion> for ::windows::core::IInspectable {
    fn from(value: InteractionTrackerInertiaMotion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaMotion> for ::windows::core::IInspectable {
    fn from(value: &InteractionTrackerInertiaMotion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InteractionTrackerInertiaMotion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InteractionTrackerInertiaMotion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<InteractionTrackerInertiaMotion> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTrackerInertiaMotion) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerInertiaMotion> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerInertiaMotion) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject> for InteractionTrackerInertiaMotion {
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject> for &InteractionTrackerInertiaMotion {
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<InteractionTrackerInertiaMotion> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTrackerInertiaMotion) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&InteractionTrackerInertiaMotion> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerInertiaMotion) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for InteractionTrackerInertiaMotion {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &InteractionTrackerInertiaMotion {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<InteractionTrackerInertiaMotion> for InteractionTrackerInertiaModifier {
    fn from(value: InteractionTrackerInertiaMotion) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaMotion> for InteractionTrackerInertiaModifier {
    fn from(value: &InteractionTrackerInertiaMotion) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, InteractionTrackerInertiaModifier> for InteractionTrackerInertiaMotion {
    fn into_param(self) -> ::windows::core::Param<'a, InteractionTrackerInertiaModifier> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, InteractionTrackerInertiaModifier> for &InteractionTrackerInertiaMotion {
    fn into_param(self) -> ::windows::core::Param<'a, InteractionTrackerInertiaModifier> {
        ::windows::core::Param::Owned(::core::convert::Into::<InteractionTrackerInertiaModifier>::into(self))
    }
}
impl ::core::convert::From<InteractionTrackerInertiaMotion> for super::CompositionObject {
    fn from(value: InteractionTrackerInertiaMotion) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaMotion> for super::CompositionObject {
    fn from(value: &InteractionTrackerInertiaMotion) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject> for InteractionTrackerInertiaMotion {
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject> for &InteractionTrackerInertiaMotion {
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerInertiaMotion {}
unsafe impl ::core::marker::Sync for InteractionTrackerInertiaMotion {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerInertiaNaturalMotion(::windows::core::IUnknown);
impl InteractionTrackerInertiaNaturalMotion {
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).PopulatePropertyInfo)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Compositor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Dispatcher)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StartAnimation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StartAnimation)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StopAnimation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, propertyname: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopAnimation)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Comment)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetComment<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetComment)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ImplicitAnimations)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::core::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetImplicitAnimations)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StartAnimationGroup)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopAnimationGroup)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, propertyname: Param0) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryGetAnimationController)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Condition(&self) -> ::windows::core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Condition)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetCondition<'a, Param0: ::windows::core::IntoParam<'a, super::ExpressionAnimation>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCondition)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn NaturalMotion(&self) -> ::windows::core::Result<super::ScalarNaturalMotionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NaturalMotion)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ScalarNaturalMotionAnimation>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetNaturalMotion<'a, Param0: ::windows::core::IntoParam<'a, super::ScalarNaturalMotionAnimation>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetNaturalMotion)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::Compositor>>(compositor: Param0) -> ::windows::core::Result<InteractionTrackerInertiaNaturalMotion> {
        Self::IInteractionTrackerInertiaNaturalMotionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), compositor.into_param().abi(), &mut result__).from_abi::<InteractionTrackerInertiaNaturalMotion>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInteractionTrackerInertiaNaturalMotionStatics<R, F: FnOnce(&IInteractionTrackerInertiaNaturalMotionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InteractionTrackerInertiaNaturalMotion, IInteractionTrackerInertiaNaturalMotionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for InteractionTrackerInertiaNaturalMotion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InteractionTrackerInertiaNaturalMotion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTrackerInertiaNaturalMotion {}
impl ::core::fmt::Debug for InteractionTrackerInertiaNaturalMotion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerInertiaNaturalMotion").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionTrackerInertiaNaturalMotion {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerInertiaNaturalMotion;{70acdaae-27dc-48ed-a3c3-6d61c9a029d2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for InteractionTrackerInertiaNaturalMotion {
    type Vtable = IInteractionTrackerInertiaNaturalMotion_Vtbl;
    const IID: ::windows::core::GUID = <IInteractionTrackerInertiaNaturalMotion as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InteractionTrackerInertiaNaturalMotion {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerInertiaNaturalMotion";
}
impl ::core::convert::From<InteractionTrackerInertiaNaturalMotion> for ::windows::core::IUnknown {
    fn from(value: InteractionTrackerInertiaNaturalMotion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaNaturalMotion> for ::windows::core::IUnknown {
    fn from(value: &InteractionTrackerInertiaNaturalMotion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InteractionTrackerInertiaNaturalMotion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InteractionTrackerInertiaNaturalMotion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InteractionTrackerInertiaNaturalMotion> for ::windows::core::IInspectable {
    fn from(value: InteractionTrackerInertiaNaturalMotion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaNaturalMotion> for ::windows::core::IInspectable {
    fn from(value: &InteractionTrackerInertiaNaturalMotion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InteractionTrackerInertiaNaturalMotion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InteractionTrackerInertiaNaturalMotion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<InteractionTrackerInertiaNaturalMotion> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTrackerInertiaNaturalMotion) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerInertiaNaturalMotion> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerInertiaNaturalMotion) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject> for InteractionTrackerInertiaNaturalMotion {
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject> for &InteractionTrackerInertiaNaturalMotion {
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<InteractionTrackerInertiaNaturalMotion> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTrackerInertiaNaturalMotion) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&InteractionTrackerInertiaNaturalMotion> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerInertiaNaturalMotion) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for InteractionTrackerInertiaNaturalMotion {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &InteractionTrackerInertiaNaturalMotion {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<InteractionTrackerInertiaNaturalMotion> for InteractionTrackerInertiaModifier {
    fn from(value: InteractionTrackerInertiaNaturalMotion) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaNaturalMotion> for InteractionTrackerInertiaModifier {
    fn from(value: &InteractionTrackerInertiaNaturalMotion) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, InteractionTrackerInertiaModifier> for InteractionTrackerInertiaNaturalMotion {
    fn into_param(self) -> ::windows::core::Param<'a, InteractionTrackerInertiaModifier> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, InteractionTrackerInertiaModifier> for &InteractionTrackerInertiaNaturalMotion {
    fn into_param(self) -> ::windows::core::Param<'a, InteractionTrackerInertiaModifier> {
        ::windows::core::Param::Owned(::core::convert::Into::<InteractionTrackerInertiaModifier>::into(self))
    }
}
impl ::core::convert::From<InteractionTrackerInertiaNaturalMotion> for super::CompositionObject {
    fn from(value: InteractionTrackerInertiaNaturalMotion) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaNaturalMotion> for super::CompositionObject {
    fn from(value: &InteractionTrackerInertiaNaturalMotion) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject> for InteractionTrackerInertiaNaturalMotion {
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject> for &InteractionTrackerInertiaNaturalMotion {
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerInertiaNaturalMotion {}
unsafe impl ::core::marker::Sync for InteractionTrackerInertiaNaturalMotion {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerInertiaRestingValue(::windows::core::IUnknown);
impl InteractionTrackerInertiaRestingValue {
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).PopulatePropertyInfo)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Compositor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Dispatcher)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StartAnimation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StartAnimation)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StopAnimation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, propertyname: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopAnimation)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Comment)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetComment<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetComment)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ImplicitAnimations)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::core::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetImplicitAnimations)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StartAnimationGroup)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopAnimationGroup)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, propertyname: Param0) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryGetAnimationController)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Condition(&self) -> ::windows::core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Condition)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetCondition<'a, Param0: ::windows::core::IntoParam<'a, super::ExpressionAnimation>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCondition)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn RestingValue(&self) -> ::windows::core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RestingValue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetRestingValue<'a, Param0: ::windows::core::IntoParam<'a, super::ExpressionAnimation>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRestingValue)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::Compositor>>(compositor: Param0) -> ::windows::core::Result<InteractionTrackerInertiaRestingValue> {
        Self::IInteractionTrackerInertiaRestingValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), compositor.into_param().abi(), &mut result__).from_abi::<InteractionTrackerInertiaRestingValue>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInteractionTrackerInertiaRestingValueStatics<R, F: FnOnce(&IInteractionTrackerInertiaRestingValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InteractionTrackerInertiaRestingValue, IInteractionTrackerInertiaRestingValueStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for InteractionTrackerInertiaRestingValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InteractionTrackerInertiaRestingValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTrackerInertiaRestingValue {}
impl ::core::fmt::Debug for InteractionTrackerInertiaRestingValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerInertiaRestingValue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionTrackerInertiaRestingValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerInertiaRestingValue;{86f7ec09-5096-4170-9cc8-df2fe101bb93})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for InteractionTrackerInertiaRestingValue {
    type Vtable = IInteractionTrackerInertiaRestingValue_Vtbl;
    const IID: ::windows::core::GUID = <IInteractionTrackerInertiaRestingValue as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InteractionTrackerInertiaRestingValue {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerInertiaRestingValue";
}
impl ::core::convert::From<InteractionTrackerInertiaRestingValue> for ::windows::core::IUnknown {
    fn from(value: InteractionTrackerInertiaRestingValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaRestingValue> for ::windows::core::IUnknown {
    fn from(value: &InteractionTrackerInertiaRestingValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InteractionTrackerInertiaRestingValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InteractionTrackerInertiaRestingValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InteractionTrackerInertiaRestingValue> for ::windows::core::IInspectable {
    fn from(value: InteractionTrackerInertiaRestingValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaRestingValue> for ::windows::core::IInspectable {
    fn from(value: &InteractionTrackerInertiaRestingValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InteractionTrackerInertiaRestingValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InteractionTrackerInertiaRestingValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<InteractionTrackerInertiaRestingValue> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTrackerInertiaRestingValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerInertiaRestingValue> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerInertiaRestingValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject> for InteractionTrackerInertiaRestingValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject> for &InteractionTrackerInertiaRestingValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<InteractionTrackerInertiaRestingValue> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTrackerInertiaRestingValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&InteractionTrackerInertiaRestingValue> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerInertiaRestingValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for InteractionTrackerInertiaRestingValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &InteractionTrackerInertiaRestingValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<InteractionTrackerInertiaRestingValue> for InteractionTrackerInertiaModifier {
    fn from(value: InteractionTrackerInertiaRestingValue) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaRestingValue> for InteractionTrackerInertiaModifier {
    fn from(value: &InteractionTrackerInertiaRestingValue) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, InteractionTrackerInertiaModifier> for InteractionTrackerInertiaRestingValue {
    fn into_param(self) -> ::windows::core::Param<'a, InteractionTrackerInertiaModifier> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, InteractionTrackerInertiaModifier> for &InteractionTrackerInertiaRestingValue {
    fn into_param(self) -> ::windows::core::Param<'a, InteractionTrackerInertiaModifier> {
        ::windows::core::Param::Owned(::core::convert::Into::<InteractionTrackerInertiaModifier>::into(self))
    }
}
impl ::core::convert::From<InteractionTrackerInertiaRestingValue> for super::CompositionObject {
    fn from(value: InteractionTrackerInertiaRestingValue) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaRestingValue> for super::CompositionObject {
    fn from(value: &InteractionTrackerInertiaRestingValue) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject> for InteractionTrackerInertiaRestingValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject> for &InteractionTrackerInertiaRestingValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerInertiaRestingValue {}
unsafe impl ::core::marker::Sync for InteractionTrackerInertiaRestingValue {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerInertiaStateEnteredArgs(::windows::core::IUnknown);
impl InteractionTrackerInertiaStateEnteredArgs {
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn ModifiedRestingPosition(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ModifiedRestingPosition)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::Numerics::Vector3>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ModifiedRestingScale(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ModifiedRestingScale)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<f32>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn NaturalRestingPosition(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NaturalRestingPosition)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn NaturalRestingScale(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NaturalRestingScale)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PositionVelocityInPixelsPerSecond(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PositionVelocityInPixelsPerSecond)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn RequestId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn ScaleVelocityInPercentPerSecond(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ScaleVelocityInPercentPerSecond)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn IsInertiaFromImpulse(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInteractionTrackerInertiaStateEnteredArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsInertiaFromImpulse)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn IsFromBinding(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInteractionTrackerInertiaStateEnteredArgs3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsFromBinding)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for InteractionTrackerInertiaStateEnteredArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InteractionTrackerInertiaStateEnteredArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTrackerInertiaStateEnteredArgs {}
impl ::core::fmt::Debug for InteractionTrackerInertiaStateEnteredArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerInertiaStateEnteredArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionTrackerInertiaStateEnteredArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerInertiaStateEnteredArgs;{87108cf2-e7ff-4f7d-9ffd-d72f1e409b63})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for InteractionTrackerInertiaStateEnteredArgs {
    type Vtable = IInteractionTrackerInertiaStateEnteredArgs_Vtbl;
    const IID: ::windows::core::GUID = <IInteractionTrackerInertiaStateEnteredArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InteractionTrackerInertiaStateEnteredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerInertiaStateEnteredArgs";
}
impl ::core::convert::From<InteractionTrackerInertiaStateEnteredArgs> for ::windows::core::IUnknown {
    fn from(value: InteractionTrackerInertiaStateEnteredArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaStateEnteredArgs> for ::windows::core::IUnknown {
    fn from(value: &InteractionTrackerInertiaStateEnteredArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InteractionTrackerInertiaStateEnteredArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InteractionTrackerInertiaStateEnteredArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InteractionTrackerInertiaStateEnteredArgs> for ::windows::core::IInspectable {
    fn from(value: InteractionTrackerInertiaStateEnteredArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaStateEnteredArgs> for ::windows::core::IInspectable {
    fn from(value: &InteractionTrackerInertiaStateEnteredArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InteractionTrackerInertiaStateEnteredArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InteractionTrackerInertiaStateEnteredArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerInertiaStateEnteredArgs {}
unsafe impl ::core::marker::Sync for InteractionTrackerInertiaStateEnteredArgs {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerInteractingStateEnteredArgs(::windows::core::IUnknown);
impl InteractionTrackerInteractingStateEnteredArgs {
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn RequestId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn IsFromBinding(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInteractionTrackerInteractingStateEnteredArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsFromBinding)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for InteractionTrackerInteractingStateEnteredArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InteractionTrackerInteractingStateEnteredArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTrackerInteractingStateEnteredArgs {}
impl ::core::fmt::Debug for InteractionTrackerInteractingStateEnteredArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerInteractingStateEnteredArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionTrackerInteractingStateEnteredArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerInteractingStateEnteredArgs;{a7263939-a17b-4011-99fd-b5c24f143748})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for InteractionTrackerInteractingStateEnteredArgs {
    type Vtable = IInteractionTrackerInteractingStateEnteredArgs_Vtbl;
    const IID: ::windows::core::GUID = <IInteractionTrackerInteractingStateEnteredArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InteractionTrackerInteractingStateEnteredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerInteractingStateEnteredArgs";
}
impl ::core::convert::From<InteractionTrackerInteractingStateEnteredArgs> for ::windows::core::IUnknown {
    fn from(value: InteractionTrackerInteractingStateEnteredArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerInteractingStateEnteredArgs> for ::windows::core::IUnknown {
    fn from(value: &InteractionTrackerInteractingStateEnteredArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InteractionTrackerInteractingStateEnteredArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InteractionTrackerInteractingStateEnteredArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InteractionTrackerInteractingStateEnteredArgs> for ::windows::core::IInspectable {
    fn from(value: InteractionTrackerInteractingStateEnteredArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerInteractingStateEnteredArgs> for ::windows::core::IInspectable {
    fn from(value: &InteractionTrackerInteractingStateEnteredArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InteractionTrackerInteractingStateEnteredArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InteractionTrackerInteractingStateEnteredArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerInteractingStateEnteredArgs {}
unsafe impl ::core::marker::Sync for InteractionTrackerInteractingStateEnteredArgs {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InteractionTrackerPositionUpdateOption(pub i32);
impl InteractionTrackerPositionUpdateOption {
    pub const Default: Self = Self(0i32);
    pub const AllowActiveCustomScaleAnimation: Self = Self(1i32);
}
impl ::core::marker::Copy for InteractionTrackerPositionUpdateOption {}
impl ::core::clone::Clone for InteractionTrackerPositionUpdateOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InteractionTrackerPositionUpdateOption {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for InteractionTrackerPositionUpdateOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for InteractionTrackerPositionUpdateOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerPositionUpdateOption").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionTrackerPositionUpdateOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.InteractionTrackerPositionUpdateOption;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerRequestIgnoredArgs(::windows::core::IUnknown);
impl InteractionTrackerRequestIgnoredArgs {
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn RequestId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
}
impl ::core::clone::Clone for InteractionTrackerRequestIgnoredArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InteractionTrackerRequestIgnoredArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTrackerRequestIgnoredArgs {}
impl ::core::fmt::Debug for InteractionTrackerRequestIgnoredArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerRequestIgnoredArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionTrackerRequestIgnoredArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerRequestIgnoredArgs;{80dd82f1-ce25-488f-91dd-cb6455ccff2e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for InteractionTrackerRequestIgnoredArgs {
    type Vtable = IInteractionTrackerRequestIgnoredArgs_Vtbl;
    const IID: ::windows::core::GUID = <IInteractionTrackerRequestIgnoredArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InteractionTrackerRequestIgnoredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerRequestIgnoredArgs";
}
impl ::core::convert::From<InteractionTrackerRequestIgnoredArgs> for ::windows::core::IUnknown {
    fn from(value: InteractionTrackerRequestIgnoredArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerRequestIgnoredArgs> for ::windows::core::IUnknown {
    fn from(value: &InteractionTrackerRequestIgnoredArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InteractionTrackerRequestIgnoredArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InteractionTrackerRequestIgnoredArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InteractionTrackerRequestIgnoredArgs> for ::windows::core::IInspectable {
    fn from(value: InteractionTrackerRequestIgnoredArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerRequestIgnoredArgs> for ::windows::core::IInspectable {
    fn from(value: &InteractionTrackerRequestIgnoredArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InteractionTrackerRequestIgnoredArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InteractionTrackerRequestIgnoredArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerRequestIgnoredArgs {}
unsafe impl ::core::marker::Sync for InteractionTrackerRequestIgnoredArgs {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerValuesChangedArgs(::windows::core::IUnknown);
impl InteractionTrackerValuesChangedArgs {
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn RequestId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Scale(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Scale)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
}
impl ::core::clone::Clone for InteractionTrackerValuesChangedArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InteractionTrackerValuesChangedArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTrackerValuesChangedArgs {}
impl ::core::fmt::Debug for InteractionTrackerValuesChangedArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerValuesChangedArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionTrackerValuesChangedArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerValuesChangedArgs;{cf1578ef-d3df-4501-b9e6-f02fb22f73d0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for InteractionTrackerValuesChangedArgs {
    type Vtable = IInteractionTrackerValuesChangedArgs_Vtbl;
    const IID: ::windows::core::GUID = <IInteractionTrackerValuesChangedArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InteractionTrackerValuesChangedArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerValuesChangedArgs";
}
impl ::core::convert::From<InteractionTrackerValuesChangedArgs> for ::windows::core::IUnknown {
    fn from(value: InteractionTrackerValuesChangedArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerValuesChangedArgs> for ::windows::core::IUnknown {
    fn from(value: &InteractionTrackerValuesChangedArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InteractionTrackerValuesChangedArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InteractionTrackerValuesChangedArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InteractionTrackerValuesChangedArgs> for ::windows::core::IInspectable {
    fn from(value: InteractionTrackerValuesChangedArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerValuesChangedArgs> for ::windows::core::IInspectable {
    fn from(value: &InteractionTrackerValuesChangedArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InteractionTrackerValuesChangedArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InteractionTrackerValuesChangedArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerValuesChangedArgs {}
unsafe impl ::core::marker::Sync for InteractionTrackerValuesChangedArgs {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerVector2InertiaModifier(::windows::core::IUnknown);
impl InteractionTrackerVector2InertiaModifier {
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).PopulatePropertyInfo)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Compositor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Dispatcher)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StartAnimation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StartAnimation)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StopAnimation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, propertyname: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopAnimation)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Comment)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetComment<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetComment)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ImplicitAnimations)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::core::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetImplicitAnimations)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StartAnimationGroup)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopAnimationGroup)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, propertyname: Param0) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryGetAnimationController)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
}
impl ::core::clone::Clone for InteractionTrackerVector2InertiaModifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InteractionTrackerVector2InertiaModifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTrackerVector2InertiaModifier {}
impl ::core::fmt::Debug for InteractionTrackerVector2InertiaModifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerVector2InertiaModifier").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionTrackerVector2InertiaModifier {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerVector2InertiaModifier;{87e08ab0-3086-4853-a4b7-77882ad5d7e3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for InteractionTrackerVector2InertiaModifier {
    type Vtable = IInteractionTrackerVector2InertiaModifier_Vtbl;
    const IID: ::windows::core::GUID = <IInteractionTrackerVector2InertiaModifier as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InteractionTrackerVector2InertiaModifier {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerVector2InertiaModifier";
}
impl ::core::convert::From<InteractionTrackerVector2InertiaModifier> for ::windows::core::IUnknown {
    fn from(value: InteractionTrackerVector2InertiaModifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerVector2InertiaModifier> for ::windows::core::IUnknown {
    fn from(value: &InteractionTrackerVector2InertiaModifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InteractionTrackerVector2InertiaModifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InteractionTrackerVector2InertiaModifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InteractionTrackerVector2InertiaModifier> for ::windows::core::IInspectable {
    fn from(value: InteractionTrackerVector2InertiaModifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerVector2InertiaModifier> for ::windows::core::IInspectable {
    fn from(value: &InteractionTrackerVector2InertiaModifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InteractionTrackerVector2InertiaModifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InteractionTrackerVector2InertiaModifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<InteractionTrackerVector2InertiaModifier> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTrackerVector2InertiaModifier) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerVector2InertiaModifier> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerVector2InertiaModifier) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject> for InteractionTrackerVector2InertiaModifier {
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject> for &InteractionTrackerVector2InertiaModifier {
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<InteractionTrackerVector2InertiaModifier> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTrackerVector2InertiaModifier) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&InteractionTrackerVector2InertiaModifier> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerVector2InertiaModifier) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for InteractionTrackerVector2InertiaModifier {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &InteractionTrackerVector2InertiaModifier {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<InteractionTrackerVector2InertiaModifier> for super::CompositionObject {
    fn from(value: InteractionTrackerVector2InertiaModifier) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerVector2InertiaModifier> for super::CompositionObject {
    fn from(value: &InteractionTrackerVector2InertiaModifier) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject> for InteractionTrackerVector2InertiaModifier {
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject> for &InteractionTrackerVector2InertiaModifier {
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerVector2InertiaModifier {}
unsafe impl ::core::marker::Sync for InteractionTrackerVector2InertiaModifier {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct InteractionTrackerVector2InertiaNaturalMotion(::windows::core::IUnknown);
impl InteractionTrackerVector2InertiaNaturalMotion {
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).PopulatePropertyInfo)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Compositor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Dispatcher)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StartAnimation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StartAnimation)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StopAnimation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, propertyname: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopAnimation)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Comment)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetComment<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetComment)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ImplicitAnimations)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::core::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetImplicitAnimations)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StartAnimationGroup)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopAnimationGroup)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, propertyname: Param0) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryGetAnimationController)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Condition(&self) -> ::windows::core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Condition)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetCondition<'a, Param0: ::windows::core::IntoParam<'a, super::ExpressionAnimation>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCondition)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn NaturalMotion(&self) -> ::windows::core::Result<super::Vector2NaturalMotionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NaturalMotion)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Vector2NaturalMotionAnimation>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetNaturalMotion<'a, Param0: ::windows::core::IntoParam<'a, super::Vector2NaturalMotionAnimation>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetNaturalMotion)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::Compositor>>(compositor: Param0) -> ::windows::core::Result<InteractionTrackerVector2InertiaNaturalMotion> {
        Self::IInteractionTrackerVector2InertiaNaturalMotionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), compositor.into_param().abi(), &mut result__).from_abi::<InteractionTrackerVector2InertiaNaturalMotion>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInteractionTrackerVector2InertiaNaturalMotionStatics<R, F: FnOnce(&IInteractionTrackerVector2InertiaNaturalMotionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InteractionTrackerVector2InertiaNaturalMotion, IInteractionTrackerVector2InertiaNaturalMotionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for InteractionTrackerVector2InertiaNaturalMotion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InteractionTrackerVector2InertiaNaturalMotion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InteractionTrackerVector2InertiaNaturalMotion {}
impl ::core::fmt::Debug for InteractionTrackerVector2InertiaNaturalMotion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InteractionTrackerVector2InertiaNaturalMotion").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionTrackerVector2InertiaNaturalMotion {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.InteractionTrackerVector2InertiaNaturalMotion;{5f17695c-162d-4c07-9400-c282b28276ca})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for InteractionTrackerVector2InertiaNaturalMotion {
    type Vtable = IInteractionTrackerVector2InertiaNaturalMotion_Vtbl;
    const IID: ::windows::core::GUID = <IInteractionTrackerVector2InertiaNaturalMotion as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InteractionTrackerVector2InertiaNaturalMotion {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerVector2InertiaNaturalMotion";
}
impl ::core::convert::From<InteractionTrackerVector2InertiaNaturalMotion> for ::windows::core::IUnknown {
    fn from(value: InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerVector2InertiaNaturalMotion> for ::windows::core::IUnknown {
    fn from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InteractionTrackerVector2InertiaNaturalMotion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InteractionTrackerVector2InertiaNaturalMotion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InteractionTrackerVector2InertiaNaturalMotion> for ::windows::core::IInspectable {
    fn from(value: InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InteractionTrackerVector2InertiaNaturalMotion> for ::windows::core::IInspectable {
    fn from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InteractionTrackerVector2InertiaNaturalMotion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InteractionTrackerVector2InertiaNaturalMotion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<InteractionTrackerVector2InertiaNaturalMotion> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTrackerVector2InertiaNaturalMotion) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerVector2InertiaNaturalMotion> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject> for InteractionTrackerVector2InertiaNaturalMotion {
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject> for &InteractionTrackerVector2InertiaNaturalMotion {
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<InteractionTrackerVector2InertiaNaturalMotion> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTrackerVector2InertiaNaturalMotion) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&InteractionTrackerVector2InertiaNaturalMotion> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for InteractionTrackerVector2InertiaNaturalMotion {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &InteractionTrackerVector2InertiaNaturalMotion {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<InteractionTrackerVector2InertiaNaturalMotion> for InteractionTrackerVector2InertiaModifier {
    fn from(value: InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerVector2InertiaNaturalMotion> for InteractionTrackerVector2InertiaModifier {
    fn from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, InteractionTrackerVector2InertiaModifier> for InteractionTrackerVector2InertiaNaturalMotion {
    fn into_param(self) -> ::windows::core::Param<'a, InteractionTrackerVector2InertiaModifier> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, InteractionTrackerVector2InertiaModifier> for &InteractionTrackerVector2InertiaNaturalMotion {
    fn into_param(self) -> ::windows::core::Param<'a, InteractionTrackerVector2InertiaModifier> {
        ::windows::core::Param::Owned(::core::convert::Into::<InteractionTrackerVector2InertiaModifier>::into(self))
    }
}
impl ::core::convert::From<InteractionTrackerVector2InertiaNaturalMotion> for super::CompositionObject {
    fn from(value: InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerVector2InertiaNaturalMotion> for super::CompositionObject {
    fn from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject> for InteractionTrackerVector2InertiaNaturalMotion {
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject> for &InteractionTrackerVector2InertiaNaturalMotion {
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerVector2InertiaNaturalMotion {}
unsafe impl ::core::marker::Sync for InteractionTrackerVector2InertiaNaturalMotion {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
pub struct VisualInteractionSource(::windows::core::IUnknown);
impl VisualInteractionSource {
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).PopulatePropertyInfo)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Compositor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Dispatcher)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StartAnimation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StartAnimation)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StopAnimation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, propertyname: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopAnimation)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Comment)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetComment<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetComment)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ImplicitAnimations)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::core::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetImplicitAnimations)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StartAnimationGroup)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopAnimationGroup)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, propertyname: Param0) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryGetAnimationController)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn IsPositionXRailsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPositionXRailsEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetIsPositionXRailsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsPositionXRailsEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn IsPositionYRailsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPositionYRailsEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetIsPositionYRailsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsPositionYRailsEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn ManipulationRedirectionMode(&self) -> ::windows::core::Result<VisualInteractionSourceRedirectionMode> {
        let this = self;
        unsafe {
            let mut result__: VisualInteractionSourceRedirectionMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ManipulationRedirectionMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VisualInteractionSourceRedirectionMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetManipulationRedirectionMode(&self, value: VisualInteractionSourceRedirectionMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetManipulationRedirectionMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn PositionXChainingMode(&self) -> ::windows::core::Result<InteractionChainingMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionChainingMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PositionXChainingMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InteractionChainingMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetPositionXChainingMode(&self, value: InteractionChainingMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPositionXChainingMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn PositionXSourceMode(&self) -> ::windows::core::Result<InteractionSourceMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionSourceMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PositionXSourceMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InteractionSourceMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetPositionXSourceMode(&self, value: InteractionSourceMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPositionXSourceMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn PositionYChainingMode(&self) -> ::windows::core::Result<InteractionChainingMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionChainingMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PositionYChainingMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InteractionChainingMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetPositionYChainingMode(&self, value: InteractionChainingMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPositionYChainingMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn PositionYSourceMode(&self) -> ::windows::core::Result<InteractionSourceMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionSourceMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PositionYSourceMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InteractionSourceMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetPositionYSourceMode(&self, value: InteractionSourceMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPositionYSourceMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn ScaleChainingMode(&self) -> ::windows::core::Result<InteractionChainingMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionChainingMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ScaleChainingMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InteractionChainingMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetScaleChainingMode(&self, value: InteractionChainingMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetScaleChainingMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn ScaleSourceMode(&self) -> ::windows::core::Result<InteractionSourceMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionSourceMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ScaleSourceMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InteractionSourceMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn SetScaleSourceMode(&self, value: InteractionSourceMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetScaleSourceMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Source(&self) -> ::windows::core::Result<super::Visual> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Source)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Visual>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"UI_Input\"`*"]
    #[cfg(feature = "UI_Input")]
    pub fn TryRedirectForManipulation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Input::PointerPoint>>(&self, pointerpoint: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).TryRedirectForManipulation)(::core::mem::transmute_copy(this), pointerpoint.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DeltaPosition(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = &::windows::core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeltaPosition)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn DeltaScale(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeltaScale)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = &::windows::core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PositionVelocity(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = &::windows::core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PositionVelocity)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Scale(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Scale)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn ScaleVelocity(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ScaleVelocity)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigureCenterPointXModifiers<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>>(&self, conditionalvalues: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ConfigureCenterPointXModifiers)(::core::mem::transmute_copy(this), conditionalvalues.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigureCenterPointYModifiers<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>>(&self, conditionalvalues: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ConfigureCenterPointYModifiers)(::core::mem::transmute_copy(this), conditionalvalues.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigureDeltaPositionXModifiers<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>>(&self, conditionalvalues: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ConfigureDeltaPositionXModifiers)(::core::mem::transmute_copy(this), conditionalvalues.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigureDeltaPositionYModifiers<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>>(&self, conditionalvalues: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ConfigureDeltaPositionYModifiers)(::core::mem::transmute_copy(this), conditionalvalues.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigureDeltaScaleModifiers<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>>>(&self, conditionalvalues: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ConfigureDeltaScaleModifiers)(::core::mem::transmute_copy(this), conditionalvalues.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn PointerWheelConfig(&self) -> ::windows::core::Result<InteractionSourceConfiguration> {
        let this = &::windows::core::Interface::cast::<IVisualInteractionSource3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerWheelConfig)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InteractionSourceConfiguration>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::Visual>>(source: Param0) -> ::windows::core::Result<VisualInteractionSource> {
        Self::IVisualInteractionSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), source.into_param().abi(), &mut result__).from_abi::<VisualInteractionSource>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
    pub fn CreateFromIVisualElement<'a, Param0: ::windows::core::IntoParam<'a, super::IVisualElement>>(source: Param0) -> ::windows::core::Result<VisualInteractionSource> {
        Self::IVisualInteractionSourceStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromIVisualElement)(::core::mem::transmute_copy(this), source.into_param().abi(), &mut result__).from_abi::<VisualInteractionSource>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVisualInteractionSourceStatics<R, F: FnOnce(&IVisualInteractionSourceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VisualInteractionSource, IVisualInteractionSourceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IVisualInteractionSourceStatics2<R, F: FnOnce(&IVisualInteractionSourceStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VisualInteractionSource, IVisualInteractionSourceStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for VisualInteractionSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VisualInteractionSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VisualInteractionSource {}
impl ::core::fmt::Debug for VisualInteractionSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VisualInteractionSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VisualInteractionSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Interactions.VisualInteractionSource;{ca0e8a86-d8d6-4111-b088-70347bd2b0ed})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for VisualInteractionSource {
    type Vtable = IVisualInteractionSource_Vtbl;
    const IID: ::windows::core::GUID = <IVisualInteractionSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for VisualInteractionSource {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.VisualInteractionSource";
}
impl ::core::convert::From<VisualInteractionSource> for ::windows::core::IUnknown {
    fn from(value: VisualInteractionSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VisualInteractionSource> for ::windows::core::IUnknown {
    fn from(value: &VisualInteractionSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VisualInteractionSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VisualInteractionSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VisualInteractionSource> for ::windows::core::IInspectable {
    fn from(value: VisualInteractionSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VisualInteractionSource> for ::windows::core::IInspectable {
    fn from(value: &VisualInteractionSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VisualInteractionSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VisualInteractionSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<VisualInteractionSource> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: VisualInteractionSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VisualInteractionSource> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &VisualInteractionSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject> for VisualInteractionSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject> for &VisualInteractionSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<VisualInteractionSource> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: VisualInteractionSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&VisualInteractionSource> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &VisualInteractionSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for VisualInteractionSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &VisualInteractionSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<VisualInteractionSource> for ICompositionInteractionSource {
    type Error = ::windows::core::Error;
    fn try_from(value: VisualInteractionSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VisualInteractionSource> for ICompositionInteractionSource {
    type Error = ::windows::core::Error;
    fn try_from(value: &VisualInteractionSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICompositionInteractionSource> for VisualInteractionSource {
    fn into_param(self) -> ::windows::core::Param<'a, ICompositionInteractionSource> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICompositionInteractionSource> for &VisualInteractionSource {
    fn into_param(self) -> ::windows::core::Param<'a, ICompositionInteractionSource> {
        ::core::convert::TryInto::<ICompositionInteractionSource>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<VisualInteractionSource> for super::CompositionObject {
    fn from(value: VisualInteractionSource) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&VisualInteractionSource> for super::CompositionObject {
    fn from(value: &VisualInteractionSource) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject> for VisualInteractionSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject> for &VisualInteractionSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for VisualInteractionSource {}
unsafe impl ::core::marker::Sync for VisualInteractionSource {}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VisualInteractionSourceRedirectionMode(pub i32);
impl VisualInteractionSourceRedirectionMode {
    pub const Off: Self = Self(0i32);
    pub const CapableTouchpadOnly: Self = Self(1i32);
    pub const PointerWheelOnly: Self = Self(2i32);
    pub const CapableTouchpadAndPointerWheel: Self = Self(3i32);
}
impl ::core::marker::Copy for VisualInteractionSourceRedirectionMode {}
impl ::core::clone::Clone for VisualInteractionSourceRedirectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VisualInteractionSourceRedirectionMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VisualInteractionSourceRedirectionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for VisualInteractionSourceRedirectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VisualInteractionSourceRedirectionMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VisualInteractionSourceRedirectionMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.VisualInteractionSourceRedirectionMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
