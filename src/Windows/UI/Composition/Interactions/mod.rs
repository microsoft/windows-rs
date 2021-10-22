#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CompositionConditionalValue(::windows::runtime::IInspectable);
impl CompositionConditionalValue {
    pub fn Condition(&self) -> ::windows::runtime::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    pub fn SetCondition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ExpressionAnimation>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Value(&self) -> ::windows::runtime::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::ExpressionAnimation>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::Compositor>>(
        compositor: Param0,
    ) -> ::windows::runtime::Result<CompositionConditionalValue> {
        Self::ICompositionConditionalValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                compositor.into_param().abi(),
                &mut result__,
            )
            .from_abi::<CompositionConditionalValue>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::super::Foundation::IClosable,
        >(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ICompositionConditionalValueStatics<
        R,
        F: FnOnce(&ICompositionConditionalValueStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            CompositionConditionalValue,
            ICompositionConditionalValueStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CompositionConditionalValue {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.Composition.Interactions.CompositionConditionalValue;{43250538-eb73-4561-a71d-1a43eaeb7a9b})" ) ;
}
unsafe impl ::windows::runtime::Interface for CompositionConditionalValue {
    type Vtable = ICompositionConditionalValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1126499640,
        60275,
        17761,
        [167, 29, 26, 67, 234, 235, 122, 155],
    );
}
impl ::windows::runtime::RuntimeName for CompositionConditionalValue {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.CompositionConditionalValue";
}
impl ::std::convert::From<CompositionConditionalValue> for ::windows::runtime::IUnknown {
    fn from(value: CompositionConditionalValue) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CompositionConditionalValue> for ::windows::runtime::IUnknown {
    fn from(value: &CompositionConditionalValue) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CompositionConditionalValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CompositionConditionalValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CompositionConditionalValue> for ::windows::runtime::IInspectable {
    fn from(value: CompositionConditionalValue) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CompositionConditionalValue> for ::windows::runtime::IInspectable {
    fn from(value: &CompositionConditionalValue) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CompositionConditionalValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CompositionConditionalValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<CompositionConditionalValue>
    for super::super::super::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: CompositionConditionalValue) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&CompositionConditionalValue>
    for super::super::super::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &CompositionConditionalValue) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable>
    for CompositionConditionalValue
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable>
    for &CompositionConditionalValue
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<CompositionConditionalValue> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CompositionConditionalValue) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&CompositionConditionalValue> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &CompositionConditionalValue) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for CompositionConditionalValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for &CompositionConditionalValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<CompositionConditionalValue> for super::CompositionObject {
    fn from(value: CompositionConditionalValue) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&CompositionConditionalValue> for super::CompositionObject {
    fn from(value: &CompositionConditionalValue) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for CompositionConditionalValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for &CompositionConditionalValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for CompositionConditionalValue {}
unsafe impl ::std::marker::Sync for CompositionConditionalValue {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CompositionInteractionSourceCollection(::windows::runtime::IInspectable);
impl CompositionInteractionSourceCollection {
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::Foundation::Collections::IIterator<ICompositionInteractionSource>,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::super::Foundation::Collections::IIterable<ICompositionInteractionSource>,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::Collections::IIterator<
                ICompositionInteractionSource,
            >>(result__)
        }
    }
    pub fn Count(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn Add<'a, Param0: ::windows::runtime::IntoParam<'a, ICompositionInteractionSource>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Remove<'a, Param0: ::windows::runtime::IntoParam<'a, ICompositionInteractionSource>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveAll(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::super::Foundation::IClosable,
        >(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CompositionInteractionSourceCollection {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.Composition.Interactions.CompositionInteractionSourceCollection;{1b468e4b-a5bf-47d8-a547-3894155a158c})" ) ;
}
unsafe impl ::windows::runtime::Interface for CompositionInteractionSourceCollection {
    type Vtable = ICompositionInteractionSourceCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        457608779,
        42431,
        18392,
        [165, 71, 56, 148, 21, 90, 21, 140],
    );
}
impl ::windows::runtime::RuntimeName for CompositionInteractionSourceCollection {
    const NAME: &'static str =
        "Windows.UI.Composition.Interactions.CompositionInteractionSourceCollection";
}
impl ::std::convert::From<CompositionInteractionSourceCollection> for ::windows::runtime::IUnknown {
    fn from(value: CompositionInteractionSourceCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CompositionInteractionSourceCollection>
    for ::windows::runtime::IUnknown
{
    fn from(value: &CompositionInteractionSourceCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CompositionInteractionSourceCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CompositionInteractionSourceCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CompositionInteractionSourceCollection>
    for ::windows::runtime::IInspectable
{
    fn from(value: CompositionInteractionSourceCollection) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CompositionInteractionSourceCollection>
    for ::windows::runtime::IInspectable
{
    fn from(value: &CompositionInteractionSourceCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CompositionInteractionSourceCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CompositionInteractionSourceCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<CompositionInteractionSourceCollection>
    for super::super::super::Foundation::Collections::IIterable<ICompositionInteractionSource>
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: CompositionInteractionSourceCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&CompositionInteractionSourceCollection>
    for super::super::super::Foundation::Collections::IIterable<ICompositionInteractionSource>
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &CompositionInteractionSourceCollection,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::super::Foundation::Collections::IIterable<ICompositionInteractionSource>,
    > for CompositionInteractionSourceCollection
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::super::Foundation::Collections::IIterable<ICompositionInteractionSource>,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::super::Foundation::Collections::IIterable<ICompositionInteractionSource>,
    > for &CompositionInteractionSourceCollection
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::super::Foundation::Collections::IIterable<ICompositionInteractionSource>,
    > {
        ::std::convert::TryInto::<
            super::super::super::Foundation::Collections::IIterable<ICompositionInteractionSource>,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<CompositionInteractionSourceCollection>
    for super::super::super::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: CompositionInteractionSourceCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&CompositionInteractionSourceCollection>
    for super::super::super::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &CompositionInteractionSourceCollection,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable>
    for CompositionInteractionSourceCollection
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable>
    for &CompositionInteractionSourceCollection
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<CompositionInteractionSourceCollection> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CompositionInteractionSourceCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&CompositionInteractionSourceCollection> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &CompositionInteractionSourceCollection,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for CompositionInteractionSourceCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for &CompositionInteractionSourceCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<CompositionInteractionSourceCollection> for super::CompositionObject {
    fn from(value: CompositionInteractionSourceCollection) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&CompositionInteractionSourceCollection> for super::CompositionObject {
    fn from(value: &CompositionInteractionSourceCollection) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for CompositionInteractionSourceCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for &CompositionInteractionSourceCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for CompositionInteractionSourceCollection {}
unsafe impl ::std::marker::Sync for CompositionInteractionSourceCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for CompositionInteractionSourceCollection {
    type Item = ICompositionInteractionSource;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &CompositionInteractionSourceCollection {
    type Item = ICompositionInteractionSource;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICompositionConditionalValue(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICompositionConditionalValue {
    type Vtable = ICompositionConditionalValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1126499640,
        60275,
        17761,
        [167, 29, 26, 67, 234, 235, 122, 155],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionConditionalValue_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICompositionConditionalValueStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICompositionConditionalValueStatics {
    type Vtable = ICompositionConditionalValueStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        151800690,
        33895,
        19722,
        [144, 101, 172, 70, 184, 10, 85, 34],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionConditionalValueStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        compositor: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ICompositionInteractionSource(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICompositionInteractionSource {
    type Vtable = ICompositionInteractionSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        70984753,
        1763,
        18778,
        [186, 84, 64, 159, 0, 23, 250, 192],
    );
}
impl ICompositionInteractionSource {}
unsafe impl ::windows::runtime::RuntimeType for ICompositionInteractionSource {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{043b2431-06e3-495a-ba54-409f0017fac0}");
}
impl ::std::convert::From<ICompositionInteractionSource> for ::windows::runtime::IUnknown {
    fn from(value: ICompositionInteractionSource) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICompositionInteractionSource> for ::windows::runtime::IUnknown {
    fn from(value: &ICompositionInteractionSource) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ICompositionInteractionSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ICompositionInteractionSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ICompositionInteractionSource> for ::windows::runtime::IInspectable {
    fn from(value: ICompositionInteractionSource) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ICompositionInteractionSource> for ::windows::runtime::IInspectable {
    fn from(value: &ICompositionInteractionSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ICompositionInteractionSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ICompositionInteractionSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionInteractionSource_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICompositionInteractionSourceCollection(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICompositionInteractionSourceCollection {
    type Vtable = ICompositionInteractionSourceCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        457608779,
        42431,
        18392,
        [165, 71, 56, 148, 21, 90, 21, 140],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionInteractionSourceCollection_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IInteractionSourceConfiguration(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionSourceConfiguration {
    type Vtable = IInteractionSourceConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2810398693,
        43473,
        19714,
        [152, 94, 185, 48, 205, 11, 157, 164],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionSourceConfiguration_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut InteractionSourceRedirectionMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: InteractionSourceRedirectionMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut InteractionSourceRedirectionMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: InteractionSourceRedirectionMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut InteractionSourceRedirectionMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: InteractionSourceRedirectionMode,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IInteractionTracker(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTracker {
    type Vtable = IInteractionTracker_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        713985201,
        4096,
        17430,
        [131, 99, 204, 39, 251, 135, 115, 8],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::super::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::super::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics")))] usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        adjustment: f32,
        positionthreshold: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        adjustment: f32,
        positionthreshold: f32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        modifiers: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        modifiers: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        modifiers: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::super::Foundation::Numerics::Vector3,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        amount: super::super::super::Foundation::Numerics::Vector3,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        velocityinpixelspersecond: super::super::super::Foundation::Numerics::Vector3,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f32,
        centerpoint: super::super::super::Foundation::Numerics::Vector3,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
        centerpoint: super::super::super::Foundation::Numerics::Vector3,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        velocityinpercentpersecond: f32,
        centerpoint: super::super::super::Foundation::Numerics::Vector3,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IInteractionTracker2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTracker2 {
    type Vtable = IInteractionTracker2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        628529726,
        52845,
        17548,
        [131, 134, 146, 98, 13, 36, 7, 86],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        conditionalvalues: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        conditionalvalues: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IInteractionTracker3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTracker3 {
    type Vtable = IInteractionTracker3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3871725474,
        23627,
        17094,
        [132, 183, 246, 148, 65, 177, 128, 145],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker3_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        modifiers: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IInteractionTracker4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTracker4 {
    type Vtable = IInteractionTracker4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3956417212,
        1199,
        19143,
        [132, 125, 6, 234, 54, 232, 10, 22],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker4_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::super::Foundation::Numerics::Vector3,
        option: InteractionTrackerClampingOption,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        amount: super::super::super::Foundation::Numerics::Vector3,
        option: InteractionTrackerClampingOption,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IInteractionTracker5(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTracker5 {
    type Vtable = IInteractionTracker5_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3555679650,
        41556,
        16612,
        [136, 213, 68, 228, 225, 107, 88, 9],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker5_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::super::Foundation::Numerics::Vector3,
        option: InteractionTrackerClampingOption,
        posupdateoption: InteractionTrackerPositionUpdateOption,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerCustomAnimationStateEnteredArgs {
    type Vtable = IInteractionTrackerCustomAnimationStateEnteredArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2367458545,
        55216,
        17228,
        [165, 210, 45, 118, 17, 134, 72, 52],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerCustomAnimationStateEnteredArgs2 {
    type Vtable = IInteractionTrackerCustomAnimationStateEnteredArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1205172663,
        2437,
        24217,
        [176, 36, 47, 50, 195, 128, 193, 164],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IInteractionTrackerIdleStateEnteredArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerIdleStateEnteredArgs {
    type Vtable = IInteractionTrackerIdleStateEnteredArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1342255018,
        5392,
        16706,
        [161, 165, 1, 155, 9, 248, 133, 123],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerIdleStateEnteredArgs_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IInteractionTrackerIdleStateEnteredArgs2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerIdleStateEnteredArgs2 {
    type Vtable = IInteractionTrackerIdleStateEnteredArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4075254253,
        47107,
        20791,
        [148, 53, 28, 150, 228, 135, 33, 233],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerIdleStateEnteredArgs2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaModifier(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInertiaModifier {
    type Vtable = IInteractionTrackerInertiaModifier_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2699217184,
        9908,
        19874,
        [139, 97, 94, 104, 57, 121, 187, 226],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaModifier_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaModifierFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInertiaModifierFactory {
    type Vtable = IInteractionTrackerInertiaModifierFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2570590462,
        51534,
        19334,
        [135, 243, 146, 38, 101, 186, 70, 185],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaModifierFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaMotion(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInertiaMotion {
    type Vtable = IInteractionTrackerInertiaMotion_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        76689372,
        61780,
        19640,
        [191, 51, 204, 27, 166, 17, 230, 219],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaMotion_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaMotionStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInertiaMotionStatics {
    type Vtable = IInteractionTrackerInertiaMotionStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2361933270,
        47739,
        17178,
        [132, 75, 110, 172, 145, 48, 249, 154],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaMotionStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        compositor: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaNaturalMotion(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInertiaNaturalMotion {
    type Vtable = IInteractionTrackerInertiaNaturalMotion_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1890376366,
        10204,
        18669,
        [163, 195, 109, 97, 201, 160, 41, 210],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaNaturalMotion_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaNaturalMotionStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInertiaNaturalMotionStatics {
    type Vtable = IInteractionTrackerInertiaNaturalMotionStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3487192496,
        24126,
        17033,
        [147, 45, 238, 95, 80, 231, 66, 131],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaNaturalMotionStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        compositor: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaRestingValue(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInertiaRestingValue {
    type Vtable = IInteractionTrackerInertiaRestingValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2264394761,
        20630,
        16752,
        [156, 200, 223, 47, 225, 1, 187, 147],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaRestingValue_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaRestingValueStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInertiaRestingValueStatics {
    type Vtable = IInteractionTrackerInertiaRestingValueStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        418203289,
        1861,
        16534,
        [188, 171, 58, 78, 153, 86, 155, 207],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaRestingValueStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        compositor: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInertiaStateEnteredArgs {
    type Vtable = IInteractionTrackerInertiaStateEnteredArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2266008818,
        59391,
        20349,
        [159, 253, 215, 47, 30, 64, 155, 99],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics")))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInertiaStateEnteredArgs2 {
    type Vtable = IInteractionTrackerInertiaStateEnteredArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2984981238,
        49772,
        16886,
        [161, 137, 250, 188, 34, 179, 35, 204],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInertiaStateEnteredArgs3 {
    type Vtable = IInteractionTrackerInertiaStateEnteredArgs3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1219238959,
        18365,
        22959,
        [165, 140, 121, 189, 46, 185, 239, 113],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs3_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IInteractionTrackerInteractingStateEnteredArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInteractingStateEnteredArgs {
    type Vtable = IInteractionTrackerInteractingStateEnteredArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2804300089,
        41339,
        16401,
        [153, 253, 181, 194, 79, 20, 55, 72],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInteractingStateEnteredArgs_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IInteractionTrackerInteractingStateEnteredArgs2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInteractingStateEnteredArgs2 {
    type Vtable = IInteractionTrackerInteractingStateEnteredArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1352028886,
        54408,
        22989,
        [129, 159, 245, 35, 16, 41, 91, 17],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInteractingStateEnteredArgs2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IInteractionTrackerOwner(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerOwner {
    type Vtable = IInteractionTrackerOwner_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3677260531,
        19947,
        20051,
        [178, 156, 176, 108, 159, 150, 214, 81],
    );
}
impl IInteractionTrackerOwner {
    pub fn CustomAnimationStateEntered<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, InteractionTracker>,
        Param1: ::windows::runtime::IntoParam<'a, InteractionTrackerCustomAnimationStateEnteredArgs>,
    >(
        &self,
        sender: Param0,
        args: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                sender.into_param().abi(),
                args.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IdleStateEntered<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, InteractionTracker>,
        Param1: ::windows::runtime::IntoParam<'a, InteractionTrackerIdleStateEnteredArgs>,
    >(
        &self,
        sender: Param0,
        args: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                sender.into_param().abi(),
                args.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn InertiaStateEntered<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, InteractionTracker>,
        Param1: ::windows::runtime::IntoParam<'a, InteractionTrackerInertiaStateEnteredArgs>,
    >(
        &self,
        sender: Param0,
        args: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                sender.into_param().abi(),
                args.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn InteractingStateEntered<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, InteractionTracker>,
        Param1: ::windows::runtime::IntoParam<'a, InteractionTrackerInteractingStateEnteredArgs>,
    >(
        &self,
        sender: Param0,
        args: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                sender.into_param().abi(),
                args.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RequestIgnored<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, InteractionTracker>,
        Param1: ::windows::runtime::IntoParam<'a, InteractionTrackerRequestIgnoredArgs>,
    >(
        &self,
        sender: Param0,
        args: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                sender.into_param().abi(),
                args.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ValuesChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, InteractionTracker>,
        Param1: ::windows::runtime::IntoParam<'a, InteractionTrackerValuesChangedArgs>,
    >(
        &self,
        sender: Param0,
        args: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                sender.into_param().abi(),
                args.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IInteractionTrackerOwner {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{db2e8af3-4deb-4e53-b29c-b06c9f96d651}");
}
impl ::std::convert::From<IInteractionTrackerOwner> for ::windows::runtime::IUnknown {
    fn from(value: IInteractionTrackerOwner) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IInteractionTrackerOwner> for ::windows::runtime::IUnknown {
    fn from(value: &IInteractionTrackerOwner) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IInteractionTrackerOwner
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IInteractionTrackerOwner
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IInteractionTrackerOwner> for ::windows::runtime::IInspectable {
    fn from(value: IInteractionTrackerOwner) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInteractionTrackerOwner> for ::windows::runtime::IInspectable {
    fn from(value: &IInteractionTrackerOwner) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IInteractionTrackerOwner
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IInteractionTrackerOwner
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerOwner_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        args: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        args: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        args: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        args: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        args: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        args: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IInteractionTrackerRequestIgnoredArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerRequestIgnoredArgs {
    type Vtable = IInteractionTrackerRequestIgnoredArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2162000625,
        52773,
        18575,
        [145, 221, 203, 100, 85, 204, 255, 46],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerRequestIgnoredArgs_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IInteractionTrackerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerStatics {
    type Vtable = IInteractionTrackerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3148208055,
        26000,
        17560,
        [141, 108, 235, 98, 181, 20, 201, 42],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        compositor: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        compositor: ::windows::runtime::RawPtr,
        owner: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IInteractionTrackerStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerStatics2 {
    type Vtable = IInteractionTrackerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        904214304,
        18103,
        23728,
        [181, 5, 243, 214, 136, 74, 97, 99],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerStatics2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        boundtracker1: ::windows::runtime::RawPtr,
        boundtracker2: ::windows::runtime::RawPtr,
        axismode: InteractionBindingAxisModes,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        boundtracker1: ::windows::runtime::RawPtr,
        boundtracker2: ::windows::runtime::RawPtr,
        result__: *mut InteractionBindingAxisModes,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IInteractionTrackerValuesChangedArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerValuesChangedArgs {
    type Vtable = IInteractionTrackerValuesChangedArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3474290927,
        54239,
        17665,
        [185, 230, 240, 47, 178, 47, 115, 208],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerValuesChangedArgs_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaModifier(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerVector2InertiaModifier {
    type Vtable = IInteractionTrackerVector2InertiaModifier_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2279639728,
        12422,
        18515,
        [164, 183, 119, 136, 42, 213, 215, 227],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaModifier_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaModifierFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerVector2InertiaModifierFactory {
    type Vtable = IInteractionTrackerVector2InertiaModifierFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1946277572,
        27757,
        18655,
        [188, 62, 23, 30, 34, 126, 125, 127],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaModifierFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaNaturalMotion(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerVector2InertiaNaturalMotion {
    type Vtable = IInteractionTrackerVector2InertiaNaturalMotion_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1595369820,
        5677,
        19463,
        [148, 0, 194, 130, 178, 130, 118, 202],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaNaturalMotion_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaNaturalMotionStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface
    for IInteractionTrackerVector2InertiaNaturalMotionStatics
{
    type Vtable = IInteractionTrackerVector2InertiaNaturalMotionStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2181044808,
        2496,
        17231,
        [129, 137, 20, 28, 102, 223, 54, 47],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaNaturalMotionStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        compositor: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IVisualInteractionSource(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVisualInteractionSource {
    type Vtable = IVisualInteractionSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3389950598,
        55510,
        16657,
        [176, 136, 112, 52, 123, 210, 176, 237],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSource_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut VisualInteractionSourceRedirectionMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: VisualInteractionSourceRedirectionMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut InteractionChainingMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: InteractionChainingMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut InteractionSourceMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: InteractionSourceMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut InteractionChainingMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: InteractionChainingMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut InteractionSourceMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: InteractionSourceMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut InteractionChainingMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: InteractionChainingMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut InteractionSourceMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: InteractionSourceMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pointerpoint: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Input"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IVisualInteractionSource2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVisualInteractionSource2 {
    type Vtable = IVisualInteractionSource2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2861648019,
        42812,
        16717,
        [128, 208, 36, 155, 173, 47, 189, 147],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSource2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        conditionalvalues: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        conditionalvalues: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        conditionalvalues: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        conditionalvalues: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        conditionalvalues: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IVisualInteractionSource3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVisualInteractionSource3 {
    type Vtable = IVisualInteractionSource3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3644976938,
        3420,
        16471,
        [146, 215, 201, 113, 21, 51, 32, 79],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSource3_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IVisualInteractionSourceObjectFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVisualInteractionSourceObjectFactory {
    type Vtable = IVisualInteractionSourceObjectFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2999619964,
        59786,
        16882,
        [179, 201, 137, 28, 146, 102, 200, 246],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSourceObjectFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IVisualInteractionSourceStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVisualInteractionSourceStatics {
    type Vtable = IVisualInteractionSourceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        916022753,
        34373,
        20341,
        [186, 0, 100, 121, 205, 16, 200, 230],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSourceStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        source: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IVisualInteractionSourceStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVisualInteractionSourceStatics2 {
    type Vtable = IVisualInteractionSourceStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2843328562,
        22372,
        21984,
        [188, 31, 7, 120, 120, 109, 207, 222],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSourceStatics2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        source: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct InteractionBindingAxisModes(pub u32);
impl InteractionBindingAxisModes {
    pub const None: InteractionBindingAxisModes = InteractionBindingAxisModes(0u32);
    pub const PositionX: InteractionBindingAxisModes = InteractionBindingAxisModes(1u32);
    pub const PositionY: InteractionBindingAxisModes = InteractionBindingAxisModes(2u32);
    pub const Scale: InteractionBindingAxisModes = InteractionBindingAxisModes(4u32);
}
impl ::std::convert::From<u32> for InteractionBindingAxisModes {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InteractionBindingAxisModes {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InteractionBindingAxisModes {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.UI.Composition.Interactions.InteractionBindingAxisModes;u4)",
    );
}
impl ::std::ops::BitOr for InteractionBindingAxisModes {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for InteractionBindingAxisModes {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for InteractionBindingAxisModes {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for InteractionBindingAxisModes {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for InteractionBindingAxisModes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct InteractionChainingMode(pub i32);
impl InteractionChainingMode {
    pub const Auto: InteractionChainingMode = InteractionChainingMode(0i32);
    pub const Always: InteractionChainingMode = InteractionChainingMode(1i32);
    pub const Never: InteractionChainingMode = InteractionChainingMode(2i32);
}
impl ::std::convert::From<i32> for InteractionChainingMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InteractionChainingMode {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InteractionChainingMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.UI.Composition.Interactions.InteractionChainingMode;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct InteractionSourceConfiguration(::windows::runtime::IInspectable);
impl InteractionSourceConfiguration {
    pub fn PositionXSourceMode(
        &self,
    ) -> ::windows::runtime::Result<InteractionSourceRedirectionMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionSourceRedirectionMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InteractionSourceRedirectionMode>(result__)
        }
    }
    pub fn SetPositionXSourceMode(
        &self,
        value: InteractionSourceRedirectionMode,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn PositionYSourceMode(
        &self,
    ) -> ::windows::runtime::Result<InteractionSourceRedirectionMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionSourceRedirectionMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InteractionSourceRedirectionMode>(result__)
        }
    }
    pub fn SetPositionYSourceMode(
        &self,
        value: InteractionSourceRedirectionMode,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn ScaleSourceMode(&self) -> ::windows::runtime::Result<InteractionSourceRedirectionMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionSourceRedirectionMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InteractionSourceRedirectionMode>(result__)
        }
    }
    pub fn SetScaleSourceMode(
        &self,
        value: InteractionSourceRedirectionMode,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::super::Foundation::IClosable,
        >(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionSourceConfiguration {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.Composition.Interactions.InteractionSourceConfiguration;{a78347e5-a9d1-4d02-985e-b930cd0b9da4})" ) ;
}
unsafe impl ::windows::runtime::Interface for InteractionSourceConfiguration {
    type Vtable = IInteractionSourceConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2810398693,
        43473,
        19714,
        [152, 94, 185, 48, 205, 11, 157, 164],
    );
}
impl ::windows::runtime::RuntimeName for InteractionSourceConfiguration {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionSourceConfiguration";
}
impl ::std::convert::From<InteractionSourceConfiguration> for ::windows::runtime::IUnknown {
    fn from(value: InteractionSourceConfiguration) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InteractionSourceConfiguration> for ::windows::runtime::IUnknown {
    fn from(value: &InteractionSourceConfiguration) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for InteractionSourceConfiguration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &InteractionSourceConfiguration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<InteractionSourceConfiguration> for ::windows::runtime::IInspectable {
    fn from(value: InteractionSourceConfiguration) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InteractionSourceConfiguration> for ::windows::runtime::IInspectable {
    fn from(value: &InteractionSourceConfiguration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for InteractionSourceConfiguration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InteractionSourceConfiguration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<InteractionSourceConfiguration>
    for super::super::super::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionSourceConfiguration) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&InteractionSourceConfiguration>
    for super::super::super::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InteractionSourceConfiguration) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable>
    for InteractionSourceConfiguration
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable>
    for &InteractionSourceConfiguration
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<InteractionSourceConfiguration> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionSourceConfiguration) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InteractionSourceConfiguration> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InteractionSourceConfiguration) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for InteractionSourceConfiguration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for &InteractionSourceConfiguration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<InteractionSourceConfiguration> for super::CompositionObject {
    fn from(value: InteractionSourceConfiguration) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&InteractionSourceConfiguration> for super::CompositionObject {
    fn from(value: &InteractionSourceConfiguration) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for InteractionSourceConfiguration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for &InteractionSourceConfiguration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for InteractionSourceConfiguration {}
unsafe impl ::std::marker::Sync for InteractionSourceConfiguration {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct InteractionSourceMode(pub i32);
impl InteractionSourceMode {
    pub const Disabled: InteractionSourceMode = InteractionSourceMode(0i32);
    pub const EnabledWithInertia: InteractionSourceMode = InteractionSourceMode(1i32);
    pub const EnabledWithoutInertia: InteractionSourceMode = InteractionSourceMode(2i32);
}
impl ::std::convert::From<i32> for InteractionSourceMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InteractionSourceMode {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InteractionSourceMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.UI.Composition.Interactions.InteractionSourceMode;i4)",
    );
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct InteractionSourceRedirectionMode(pub i32);
impl InteractionSourceRedirectionMode {
    pub const Disabled: InteractionSourceRedirectionMode = InteractionSourceRedirectionMode(0i32);
    pub const Enabled: InteractionSourceRedirectionMode = InteractionSourceRedirectionMode(1i32);
}
impl ::std::convert::From<i32> for InteractionSourceRedirectionMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InteractionSourceRedirectionMode {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InteractionSourceRedirectionMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.UI.Composition.Interactions.InteractionSourceRedirectionMode;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct InteractionTracker(::windows::runtime::IInspectable);
impl InteractionTracker {
    pub fn InteractionSources(
        &self,
    ) -> ::windows::runtime::Result<CompositionInteractionSourceCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CompositionInteractionSourceCollection>(result__)
        }
    }
    pub fn IsPositionRoundingSuggested(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn MaxPosition(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetMaxPosition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn MaxScale(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetMaxScale(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn MinPosition(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetMinPosition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn MinScale(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetMinScale(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn NaturalRestingPosition(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn NaturalRestingScale(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn Owner(&self) -> ::windows::runtime::Result<IInteractionTrackerOwner> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IInteractionTrackerOwner>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Position(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    pub fn PositionInertiaDecayRate(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::Foundation::IReference<
            super::super::super::Foundation::Numerics::Vector3,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::IReference<
                super::super::super::Foundation::Numerics::Vector3,
            >>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    pub fn SetPositionInertiaDecayRate<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Foundation::IReference<
                super::super::super::Foundation::Numerics::Vector3,
            >,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PositionVelocityInPixelsPerSecond(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn Scale(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ScaleInertiaDecayRate(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::IReference<f32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetScaleInertiaDecayRate<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IReference<f32>>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ScaleVelocityInPercentPerSecond(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn AdjustPositionXIfGreaterThanThreshold(
        &self,
        adjustment: f32,
        positionthreshold: f32,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                adjustment,
                positionthreshold,
            )
            .ok()
        }
    }
    pub fn AdjustPositionYIfGreaterThanThreshold(
        &self,
        adjustment: f32,
        positionthreshold: f32,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                adjustment,
                positionthreshold,
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigurePositionXInertiaModifiers<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Foundation::Collections::IIterable<
                InteractionTrackerInertiaModifier,
            >,
        >,
    >(
        &self,
        modifiers: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).29)(
                ::std::mem::transmute_copy(this),
                modifiers.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigurePositionYInertiaModifiers<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Foundation::Collections::IIterable<
                InteractionTrackerInertiaModifier,
            >,
        >,
    >(
        &self,
        modifiers: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).30)(
                ::std::mem::transmute_copy(this),
                modifiers.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigureScaleInertiaModifiers<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Foundation::Collections::IIterable<
                InteractionTrackerInertiaModifier,
            >,
        >,
    >(
        &self,
        modifiers: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                modifiers.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryUpdatePosition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryUpdatePositionBy<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>,
    >(
        &self,
        amount: Param0,
    ) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::std::mem::transmute_copy(this),
                amount.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn TryUpdatePositionWithAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(
                ::std::mem::transmute_copy(this),
                animation.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryUpdatePositionWithAdditionalVelocity<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>,
    >(
        &self,
        velocityinpixelspersecond: Param0,
    ) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::std::mem::transmute_copy(this),
                velocityinpixelspersecond.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryUpdateScale<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: f32,
        centerpoint: Param1,
    ) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(
                ::std::mem::transmute_copy(this),
                value,
                centerpoint.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryUpdateScaleWithAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>,
    >(
        &self,
        animation: Param0,
        centerpoint: Param1,
    ) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::std::mem::transmute_copy(this),
                animation.into_param().abi(),
                centerpoint.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryUpdateScaleWithAdditionalVelocity<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>,
    >(
        &self,
        velocityinpercentpersecond: f32,
        centerpoint: Param1,
    ) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).38)(
                ::std::mem::transmute_copy(this),
                velocityinpercentpersecond,
                centerpoint.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::Compositor>>(
        compositor: Param0,
    ) -> ::windows::runtime::Result<InteractionTracker> {
        Self::IInteractionTrackerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                compositor.into_param().abi(),
                &mut result__,
            )
            .from_abi::<InteractionTracker>(result__)
        })
    }
    pub fn CreateWithOwner<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Compositor>,
        Param1: ::windows::runtime::IntoParam<'a, IInteractionTrackerOwner>,
    >(
        compositor: Param0,
        owner: Param1,
    ) -> ::windows::runtime::Result<InteractionTracker> {
        Self::IInteractionTrackerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                compositor.into_param().abi(),
                owner.into_param().abi(),
                &mut result__,
            )
            .from_abi::<InteractionTracker>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigureCenterPointXInertiaModifiers<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>,
        >,
    >(
        &self,
        conditionalvalues: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IInteractionTracker2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                conditionalvalues.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigureCenterPointYInertiaModifiers<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>,
        >,
    >(
        &self,
        conditionalvalues: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IInteractionTracker2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                conditionalvalues.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigureVector2PositionInertiaModifiers<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Foundation::Collections::IIterable<
                InteractionTrackerVector2InertiaModifier,
            >,
        >,
    >(
        &self,
        modifiers: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IInteractionTracker3>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                modifiers.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryUpdatePositionWithOption<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
        option: InteractionTrackerClampingOption,
    ) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IInteractionTracker4>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
                option,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryUpdatePositionByWithOption<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>,
    >(
        &self,
        amount: Param0,
        option: InteractionTrackerClampingOption,
    ) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IInteractionTracker4>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                amount.into_param().abi(),
                option,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn IsInertiaFromImpulse(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IInteractionTracker4>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetBindingMode<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, InteractionTracker>,
        Param1: ::windows::runtime::IntoParam<'a, InteractionTracker>,
    >(
        boundtracker1: Param0,
        boundtracker2: Param1,
        axismode: InteractionBindingAxisModes,
    ) -> ::windows::runtime::Result<()> {
        Self::IInteractionTrackerStatics2(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                boundtracker1.into_param().abi(),
                boundtracker2.into_param().abi(),
                axismode,
            )
            .ok()
        })
    }
    pub fn GetBindingMode<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, InteractionTracker>,
        Param1: ::windows::runtime::IntoParam<'a, InteractionTracker>,
    >(
        boundtracker1: Param0,
        boundtracker2: Param1,
    ) -> ::windows::runtime::Result<InteractionBindingAxisModes> {
        Self::IInteractionTrackerStatics2(|this| unsafe {
            let mut result__: InteractionBindingAxisModes = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                boundtracker1.into_param().abi(),
                boundtracker2.into_param().abi(),
                &mut result__,
            )
            .from_abi::<InteractionBindingAxisModes>(result__)
        })
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryUpdatePositionWithOption2<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
        option: InteractionTrackerClampingOption,
        posupdateoption: InteractionTrackerPositionUpdateOption,
    ) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IInteractionTracker5>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
                option,
                posupdateoption,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::super::Foundation::IClosable,
        >(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IInteractionTrackerStatics<
        R,
        F: FnOnce(&IInteractionTrackerStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            InteractionTracker,
            IInteractionTrackerStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IInteractionTrackerStatics2<
        R,
        F: FnOnce(&IInteractionTrackerStatics2) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            InteractionTracker,
            IInteractionTrackerStatics2,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTracker {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.Composition.Interactions.InteractionTracker;{2a8e8cb1-1000-4416-8363-cc27fb877308})" ) ;
}
unsafe impl ::windows::runtime::Interface for InteractionTracker {
    type Vtable = IInteractionTracker_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        713985201,
        4096,
        17430,
        [131, 99, 204, 39, 251, 135, 115, 8],
    );
}
impl ::windows::runtime::RuntimeName for InteractionTracker {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTracker";
}
impl ::std::convert::From<InteractionTracker> for ::windows::runtime::IUnknown {
    fn from(value: InteractionTracker) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InteractionTracker> for ::windows::runtime::IUnknown {
    fn from(value: &InteractionTracker) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InteractionTracker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InteractionTracker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<InteractionTracker> for ::windows::runtime::IInspectable {
    fn from(value: InteractionTracker) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InteractionTracker> for ::windows::runtime::IInspectable {
    fn from(value: &InteractionTracker) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for InteractionTracker
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InteractionTracker
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<InteractionTracker> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionTracker) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&InteractionTracker> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InteractionTracker) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable>
    for InteractionTracker
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable>
    for &InteractionTracker
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<InteractionTracker> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionTracker) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InteractionTracker> for super::IAnimationObject {
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
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<InteractionTracker> for super::CompositionObject {
    fn from(value: InteractionTracker) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&InteractionTracker> for super::CompositionObject {
    fn from(value: &InteractionTracker) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for InteractionTracker {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &InteractionTracker {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for InteractionTracker {}
unsafe impl ::std::marker::Sync for InteractionTracker {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct InteractionTrackerClampingOption(pub i32);
impl InteractionTrackerClampingOption {
    pub const Auto: InteractionTrackerClampingOption = InteractionTrackerClampingOption(0i32);
    pub const Disabled: InteractionTrackerClampingOption = InteractionTrackerClampingOption(1i32);
}
impl ::std::convert::From<i32> for InteractionTrackerClampingOption {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InteractionTrackerClampingOption {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerClampingOption {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.UI.Composition.Interactions.InteractionTrackerClampingOption;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct InteractionTrackerCustomAnimationStateEnteredArgs(::windows::runtime::IInspectable);
impl InteractionTrackerCustomAnimationStateEnteredArgs {
    pub fn RequestId(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn IsFromBinding(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<
            IInteractionTrackerCustomAnimationStateEnteredArgs2,
        >(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerCustomAnimationStateEnteredArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.Composition.Interactions.InteractionTrackerCustomAnimationStateEnteredArgs;{8d1c8cf1-d7b0-434c-a5d2-2d7611864834})" ) ;
}
unsafe impl ::windows::runtime::Interface for InteractionTrackerCustomAnimationStateEnteredArgs {
    type Vtable = IInteractionTrackerCustomAnimationStateEnteredArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2367458545,
        55216,
        17228,
        [165, 210, 45, 118, 17, 134, 72, 52],
    );
}
impl ::windows::runtime::RuntimeName for InteractionTrackerCustomAnimationStateEnteredArgs {
    const NAME: &'static str =
        "Windows.UI.Composition.Interactions.InteractionTrackerCustomAnimationStateEnteredArgs";
}
impl ::std::convert::From<InteractionTrackerCustomAnimationStateEnteredArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: InteractionTrackerCustomAnimationStateEnteredArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InteractionTrackerCustomAnimationStateEnteredArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &InteractionTrackerCustomAnimationStateEnteredArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for InteractionTrackerCustomAnimationStateEnteredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &InteractionTrackerCustomAnimationStateEnteredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<InteractionTrackerCustomAnimationStateEnteredArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: InteractionTrackerCustomAnimationStateEnteredArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InteractionTrackerCustomAnimationStateEnteredArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &InteractionTrackerCustomAnimationStateEnteredArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for InteractionTrackerCustomAnimationStateEnteredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InteractionTrackerCustomAnimationStateEnteredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for InteractionTrackerCustomAnimationStateEnteredArgs {}
unsafe impl ::std::marker::Sync for InteractionTrackerCustomAnimationStateEnteredArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct InteractionTrackerIdleStateEnteredArgs(::windows::runtime::IInspectable);
impl InteractionTrackerIdleStateEnteredArgs {
    pub fn RequestId(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn IsFromBinding(&self) -> ::windows::runtime::Result<bool> {
        let this =
            &::windows::runtime::Interface::cast::<IInteractionTrackerIdleStateEnteredArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerIdleStateEnteredArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.Composition.Interactions.InteractionTrackerIdleStateEnteredArgs;{50012faa-1510-4142-a1a5-019b09f8857b})" ) ;
}
unsafe impl ::windows::runtime::Interface for InteractionTrackerIdleStateEnteredArgs {
    type Vtable = IInteractionTrackerIdleStateEnteredArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1342255018,
        5392,
        16706,
        [161, 165, 1, 155, 9, 248, 133, 123],
    );
}
impl ::windows::runtime::RuntimeName for InteractionTrackerIdleStateEnteredArgs {
    const NAME: &'static str =
        "Windows.UI.Composition.Interactions.InteractionTrackerIdleStateEnteredArgs";
}
impl ::std::convert::From<InteractionTrackerIdleStateEnteredArgs> for ::windows::runtime::IUnknown {
    fn from(value: InteractionTrackerIdleStateEnteredArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InteractionTrackerIdleStateEnteredArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &InteractionTrackerIdleStateEnteredArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for InteractionTrackerIdleStateEnteredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &InteractionTrackerIdleStateEnteredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<InteractionTrackerIdleStateEnteredArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: InteractionTrackerIdleStateEnteredArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InteractionTrackerIdleStateEnteredArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &InteractionTrackerIdleStateEnteredArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for InteractionTrackerIdleStateEnteredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InteractionTrackerIdleStateEnteredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for InteractionTrackerIdleStateEnteredArgs {}
unsafe impl ::std::marker::Sync for InteractionTrackerIdleStateEnteredArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct InteractionTrackerInertiaModifier(::windows::runtime::IInspectable);
impl InteractionTrackerInertiaModifier {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::super::Foundation::IClosable,
        >(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerInertiaModifier {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.Composition.Interactions.InteractionTrackerInertiaModifier;{a0e2c920-26b4-4da2-8b61-5e683979bbe2})" ) ;
}
unsafe impl ::windows::runtime::Interface for InteractionTrackerInertiaModifier {
    type Vtable = IInteractionTrackerInertiaModifier_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2699217184,
        9908,
        19874,
        [139, 97, 94, 104, 57, 121, 187, 226],
    );
}
impl ::windows::runtime::RuntimeName for InteractionTrackerInertiaModifier {
    const NAME: &'static str =
        "Windows.UI.Composition.Interactions.InteractionTrackerInertiaModifier";
}
impl ::std::convert::From<InteractionTrackerInertiaModifier> for ::windows::runtime::IUnknown {
    fn from(value: InteractionTrackerInertiaModifier) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InteractionTrackerInertiaModifier> for ::windows::runtime::IUnknown {
    fn from(value: &InteractionTrackerInertiaModifier) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for InteractionTrackerInertiaModifier
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &InteractionTrackerInertiaModifier
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<InteractionTrackerInertiaModifier> for ::windows::runtime::IInspectable {
    fn from(value: InteractionTrackerInertiaModifier) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InteractionTrackerInertiaModifier> for ::windows::runtime::IInspectable {
    fn from(value: &InteractionTrackerInertiaModifier) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for InteractionTrackerInertiaModifier
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InteractionTrackerInertiaModifier
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<InteractionTrackerInertiaModifier>
    for super::super::super::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionTrackerInertiaModifier) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&InteractionTrackerInertiaModifier>
    for super::super::super::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InteractionTrackerInertiaModifier) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable>
    for InteractionTrackerInertiaModifier
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable>
    for &InteractionTrackerInertiaModifier
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<InteractionTrackerInertiaModifier> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionTrackerInertiaModifier) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InteractionTrackerInertiaModifier> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InteractionTrackerInertiaModifier) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for InteractionTrackerInertiaModifier
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for &InteractionTrackerInertiaModifier
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<InteractionTrackerInertiaModifier> for super::CompositionObject {
    fn from(value: InteractionTrackerInertiaModifier) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&InteractionTrackerInertiaModifier> for super::CompositionObject {
    fn from(value: &InteractionTrackerInertiaModifier) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for InteractionTrackerInertiaModifier
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for &InteractionTrackerInertiaModifier
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for InteractionTrackerInertiaModifier {}
unsafe impl ::std::marker::Sync for InteractionTrackerInertiaModifier {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct InteractionTrackerInertiaMotion(::windows::runtime::IInspectable);
impl InteractionTrackerInertiaMotion {
    pub fn Condition(&self) -> ::windows::runtime::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    pub fn SetCondition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ExpressionAnimation>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Motion(&self) -> ::windows::runtime::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    pub fn SetMotion<'a, Param0: ::windows::runtime::IntoParam<'a, super::ExpressionAnimation>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::Compositor>>(
        compositor: Param0,
    ) -> ::windows::runtime::Result<InteractionTrackerInertiaMotion> {
        Self::IInteractionTrackerInertiaMotionStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                compositor.into_param().abi(),
                &mut result__,
            )
            .from_abi::<InteractionTrackerInertiaMotion>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::super::Foundation::IClosable,
        >(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IInteractionTrackerInertiaMotionStatics<
        R,
        F: FnOnce(&IInteractionTrackerInertiaMotionStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            InteractionTrackerInertiaMotion,
            IInteractionTrackerInertiaMotionStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerInertiaMotion {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.Composition.Interactions.InteractionTrackerInertiaMotion;{04922fdc-f154-4cb8-bf33-cc1ba611e6db})" ) ;
}
unsafe impl ::windows::runtime::Interface for InteractionTrackerInertiaMotion {
    type Vtable = IInteractionTrackerInertiaMotion_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        76689372,
        61780,
        19640,
        [191, 51, 204, 27, 166, 17, 230, 219],
    );
}
impl ::windows::runtime::RuntimeName for InteractionTrackerInertiaMotion {
    const NAME: &'static str =
        "Windows.UI.Composition.Interactions.InteractionTrackerInertiaMotion";
}
impl ::std::convert::From<InteractionTrackerInertiaMotion> for ::windows::runtime::IUnknown {
    fn from(value: InteractionTrackerInertiaMotion) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InteractionTrackerInertiaMotion> for ::windows::runtime::IUnknown {
    fn from(value: &InteractionTrackerInertiaMotion) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for InteractionTrackerInertiaMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &InteractionTrackerInertiaMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<InteractionTrackerInertiaMotion> for ::windows::runtime::IInspectable {
    fn from(value: InteractionTrackerInertiaMotion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InteractionTrackerInertiaMotion> for ::windows::runtime::IInspectable {
    fn from(value: &InteractionTrackerInertiaMotion) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for InteractionTrackerInertiaMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InteractionTrackerInertiaMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<InteractionTrackerInertiaMotion>
    for super::super::super::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionTrackerInertiaMotion) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&InteractionTrackerInertiaMotion>
    for super::super::super::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InteractionTrackerInertiaMotion) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable>
    for InteractionTrackerInertiaMotion
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable>
    for &InteractionTrackerInertiaMotion
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<InteractionTrackerInertiaMotion> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionTrackerInertiaMotion) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InteractionTrackerInertiaMotion> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InteractionTrackerInertiaMotion) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for InteractionTrackerInertiaMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for &InteractionTrackerInertiaMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<InteractionTrackerInertiaMotion> for InteractionTrackerInertiaModifier {
    fn from(value: InteractionTrackerInertiaMotion) -> Self {
        ::std::convert::Into::<InteractionTrackerInertiaModifier>::into(&value)
    }
}
impl ::std::convert::From<&InteractionTrackerInertiaMotion> for InteractionTrackerInertiaModifier {
    fn from(value: &InteractionTrackerInertiaMotion) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, InteractionTrackerInertiaModifier>
    for InteractionTrackerInertiaMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, InteractionTrackerInertiaModifier> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<InteractionTrackerInertiaModifier>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, InteractionTrackerInertiaModifier>
    for &InteractionTrackerInertiaMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, InteractionTrackerInertiaModifier> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<InteractionTrackerInertiaModifier>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
impl ::std::convert::From<InteractionTrackerInertiaMotion> for super::CompositionObject {
    fn from(value: InteractionTrackerInertiaMotion) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&InteractionTrackerInertiaMotion> for super::CompositionObject {
    fn from(value: &InteractionTrackerInertiaMotion) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for InteractionTrackerInertiaMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for &InteractionTrackerInertiaMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for InteractionTrackerInertiaMotion {}
unsafe impl ::std::marker::Sync for InteractionTrackerInertiaMotion {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct InteractionTrackerInertiaNaturalMotion(::windows::runtime::IInspectable);
impl InteractionTrackerInertiaNaturalMotion {
    pub fn Condition(&self) -> ::windows::runtime::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    pub fn SetCondition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ExpressionAnimation>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn NaturalMotion(&self) -> ::windows::runtime::Result<super::ScalarNaturalMotionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ScalarNaturalMotionAnimation>(result__)
        }
    }
    pub fn SetNaturalMotion<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ScalarNaturalMotionAnimation>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::Compositor>>(
        compositor: Param0,
    ) -> ::windows::runtime::Result<InteractionTrackerInertiaNaturalMotion> {
        Self::IInteractionTrackerInertiaNaturalMotionStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                compositor.into_param().abi(),
                &mut result__,
            )
            .from_abi::<InteractionTrackerInertiaNaturalMotion>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::super::Foundation::IClosable,
        >(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IInteractionTrackerInertiaNaturalMotionStatics<
        R,
        F: FnOnce(&IInteractionTrackerInertiaNaturalMotionStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            InteractionTrackerInertiaNaturalMotion,
            IInteractionTrackerInertiaNaturalMotionStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerInertiaNaturalMotion {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.Composition.Interactions.InteractionTrackerInertiaNaturalMotion;{70acdaae-27dc-48ed-a3c3-6d61c9a029d2})" ) ;
}
unsafe impl ::windows::runtime::Interface for InteractionTrackerInertiaNaturalMotion {
    type Vtable = IInteractionTrackerInertiaNaturalMotion_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1890376366,
        10204,
        18669,
        [163, 195, 109, 97, 201, 160, 41, 210],
    );
}
impl ::windows::runtime::RuntimeName for InteractionTrackerInertiaNaturalMotion {
    const NAME: &'static str =
        "Windows.UI.Composition.Interactions.InteractionTrackerInertiaNaturalMotion";
}
impl ::std::convert::From<InteractionTrackerInertiaNaturalMotion> for ::windows::runtime::IUnknown {
    fn from(value: InteractionTrackerInertiaNaturalMotion) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InteractionTrackerInertiaNaturalMotion>
    for ::windows::runtime::IUnknown
{
    fn from(value: &InteractionTrackerInertiaNaturalMotion) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for InteractionTrackerInertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &InteractionTrackerInertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<InteractionTrackerInertiaNaturalMotion>
    for ::windows::runtime::IInspectable
{
    fn from(value: InteractionTrackerInertiaNaturalMotion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InteractionTrackerInertiaNaturalMotion>
    for ::windows::runtime::IInspectable
{
    fn from(value: &InteractionTrackerInertiaNaturalMotion) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for InteractionTrackerInertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InteractionTrackerInertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<InteractionTrackerInertiaNaturalMotion>
    for super::super::super::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionTrackerInertiaNaturalMotion) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&InteractionTrackerInertiaNaturalMotion>
    for super::super::super::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &InteractionTrackerInertiaNaturalMotion,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable>
    for InteractionTrackerInertiaNaturalMotion
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable>
    for &InteractionTrackerInertiaNaturalMotion
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<InteractionTrackerInertiaNaturalMotion> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionTrackerInertiaNaturalMotion) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InteractionTrackerInertiaNaturalMotion> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &InteractionTrackerInertiaNaturalMotion,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for InteractionTrackerInertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for &InteractionTrackerInertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<InteractionTrackerInertiaNaturalMotion>
    for InteractionTrackerInertiaModifier
{
    fn from(value: InteractionTrackerInertiaNaturalMotion) -> Self {
        ::std::convert::Into::<InteractionTrackerInertiaModifier>::into(&value)
    }
}
impl ::std::convert::From<&InteractionTrackerInertiaNaturalMotion>
    for InteractionTrackerInertiaModifier
{
    fn from(value: &InteractionTrackerInertiaNaturalMotion) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, InteractionTrackerInertiaModifier>
    for InteractionTrackerInertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, InteractionTrackerInertiaModifier> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<InteractionTrackerInertiaModifier>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, InteractionTrackerInertiaModifier>
    for &InteractionTrackerInertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, InteractionTrackerInertiaModifier> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<InteractionTrackerInertiaModifier>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
impl ::std::convert::From<InteractionTrackerInertiaNaturalMotion> for super::CompositionObject {
    fn from(value: InteractionTrackerInertiaNaturalMotion) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&InteractionTrackerInertiaNaturalMotion> for super::CompositionObject {
    fn from(value: &InteractionTrackerInertiaNaturalMotion) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for InteractionTrackerInertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for &InteractionTrackerInertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for InteractionTrackerInertiaNaturalMotion {}
unsafe impl ::std::marker::Sync for InteractionTrackerInertiaNaturalMotion {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct InteractionTrackerInertiaRestingValue(::windows::runtime::IInspectable);
impl InteractionTrackerInertiaRestingValue {
    pub fn Condition(&self) -> ::windows::runtime::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    pub fn SetCondition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ExpressionAnimation>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RestingValue(&self) -> ::windows::runtime::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    pub fn SetRestingValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ExpressionAnimation>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::Compositor>>(
        compositor: Param0,
    ) -> ::windows::runtime::Result<InteractionTrackerInertiaRestingValue> {
        Self::IInteractionTrackerInertiaRestingValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                compositor.into_param().abi(),
                &mut result__,
            )
            .from_abi::<InteractionTrackerInertiaRestingValue>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::super::Foundation::IClosable,
        >(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IInteractionTrackerInertiaRestingValueStatics<
        R,
        F: FnOnce(&IInteractionTrackerInertiaRestingValueStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            InteractionTrackerInertiaRestingValue,
            IInteractionTrackerInertiaRestingValueStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerInertiaRestingValue {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.Composition.Interactions.InteractionTrackerInertiaRestingValue;{86f7ec09-5096-4170-9cc8-df2fe101bb93})" ) ;
}
unsafe impl ::windows::runtime::Interface for InteractionTrackerInertiaRestingValue {
    type Vtable = IInteractionTrackerInertiaRestingValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2264394761,
        20630,
        16752,
        [156, 200, 223, 47, 225, 1, 187, 147],
    );
}
impl ::windows::runtime::RuntimeName for InteractionTrackerInertiaRestingValue {
    const NAME: &'static str =
        "Windows.UI.Composition.Interactions.InteractionTrackerInertiaRestingValue";
}
impl ::std::convert::From<InteractionTrackerInertiaRestingValue> for ::windows::runtime::IUnknown {
    fn from(value: InteractionTrackerInertiaRestingValue) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InteractionTrackerInertiaRestingValue> for ::windows::runtime::IUnknown {
    fn from(value: &InteractionTrackerInertiaRestingValue) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for InteractionTrackerInertiaRestingValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &InteractionTrackerInertiaRestingValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<InteractionTrackerInertiaRestingValue>
    for ::windows::runtime::IInspectable
{
    fn from(value: InteractionTrackerInertiaRestingValue) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InteractionTrackerInertiaRestingValue>
    for ::windows::runtime::IInspectable
{
    fn from(value: &InteractionTrackerInertiaRestingValue) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for InteractionTrackerInertiaRestingValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InteractionTrackerInertiaRestingValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<InteractionTrackerInertiaRestingValue>
    for super::super::super::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionTrackerInertiaRestingValue) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&InteractionTrackerInertiaRestingValue>
    for super::super::super::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InteractionTrackerInertiaRestingValue) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable>
    for InteractionTrackerInertiaRestingValue
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable>
    for &InteractionTrackerInertiaRestingValue
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<InteractionTrackerInertiaRestingValue> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionTrackerInertiaRestingValue) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InteractionTrackerInertiaRestingValue> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InteractionTrackerInertiaRestingValue) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for InteractionTrackerInertiaRestingValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for &InteractionTrackerInertiaRestingValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<InteractionTrackerInertiaRestingValue>
    for InteractionTrackerInertiaModifier
{
    fn from(value: InteractionTrackerInertiaRestingValue) -> Self {
        ::std::convert::Into::<InteractionTrackerInertiaModifier>::into(&value)
    }
}
impl ::std::convert::From<&InteractionTrackerInertiaRestingValue>
    for InteractionTrackerInertiaModifier
{
    fn from(value: &InteractionTrackerInertiaRestingValue) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, InteractionTrackerInertiaModifier>
    for InteractionTrackerInertiaRestingValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, InteractionTrackerInertiaModifier> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<InteractionTrackerInertiaModifier>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, InteractionTrackerInertiaModifier>
    for &InteractionTrackerInertiaRestingValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, InteractionTrackerInertiaModifier> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<InteractionTrackerInertiaModifier>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
impl ::std::convert::From<InteractionTrackerInertiaRestingValue> for super::CompositionObject {
    fn from(value: InteractionTrackerInertiaRestingValue) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&InteractionTrackerInertiaRestingValue> for super::CompositionObject {
    fn from(value: &InteractionTrackerInertiaRestingValue) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for InteractionTrackerInertiaRestingValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for &InteractionTrackerInertiaRestingValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for InteractionTrackerInertiaRestingValue {}
unsafe impl ::std::marker::Sync for InteractionTrackerInertiaRestingValue {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct InteractionTrackerInertiaStateEnteredArgs(::windows::runtime::IInspectable);
impl InteractionTrackerInertiaStateEnteredArgs {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics"))]
    pub fn ModifiedRestingPosition(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::super::Foundation::IReference<
            super::super::super::Foundation::Numerics::Vector3,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::IReference<
                super::super::super::Foundation::Numerics::Vector3,
            >>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ModifiedRestingScale(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::IReference<f32>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn NaturalRestingPosition(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn NaturalRestingScale(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PositionVelocityInPixelsPerSecond(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn RequestId(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn ScaleVelocityInPercentPerSecond(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn IsInertiaFromImpulse(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<
            IInteractionTrackerInertiaStateEnteredArgs2,
        >(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn IsFromBinding(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<
            IInteractionTrackerInertiaStateEnteredArgs3,
        >(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerInertiaStateEnteredArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.Composition.Interactions.InteractionTrackerInertiaStateEnteredArgs;{87108cf2-e7ff-4f7d-9ffd-d72f1e409b63})" ) ;
}
unsafe impl ::windows::runtime::Interface for InteractionTrackerInertiaStateEnteredArgs {
    type Vtable = IInteractionTrackerInertiaStateEnteredArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2266008818,
        59391,
        20349,
        [159, 253, 215, 47, 30, 64, 155, 99],
    );
}
impl ::windows::runtime::RuntimeName for InteractionTrackerInertiaStateEnteredArgs {
    const NAME: &'static str =
        "Windows.UI.Composition.Interactions.InteractionTrackerInertiaStateEnteredArgs";
}
impl ::std::convert::From<InteractionTrackerInertiaStateEnteredArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: InteractionTrackerInertiaStateEnteredArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InteractionTrackerInertiaStateEnteredArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &InteractionTrackerInertiaStateEnteredArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for InteractionTrackerInertiaStateEnteredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &InteractionTrackerInertiaStateEnteredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<InteractionTrackerInertiaStateEnteredArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: InteractionTrackerInertiaStateEnteredArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InteractionTrackerInertiaStateEnteredArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &InteractionTrackerInertiaStateEnteredArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for InteractionTrackerInertiaStateEnteredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InteractionTrackerInertiaStateEnteredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for InteractionTrackerInertiaStateEnteredArgs {}
unsafe impl ::std::marker::Sync for InteractionTrackerInertiaStateEnteredArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct InteractionTrackerInteractingStateEnteredArgs(::windows::runtime::IInspectable);
impl InteractionTrackerInteractingStateEnteredArgs {
    pub fn RequestId(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn IsFromBinding(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<
            IInteractionTrackerInteractingStateEnteredArgs2,
        >(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerInteractingStateEnteredArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.Composition.Interactions.InteractionTrackerInteractingStateEnteredArgs;{a7263939-a17b-4011-99fd-b5c24f143748})" ) ;
}
unsafe impl ::windows::runtime::Interface for InteractionTrackerInteractingStateEnteredArgs {
    type Vtable = IInteractionTrackerInteractingStateEnteredArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2804300089,
        41339,
        16401,
        [153, 253, 181, 194, 79, 20, 55, 72],
    );
}
impl ::windows::runtime::RuntimeName for InteractionTrackerInteractingStateEnteredArgs {
    const NAME: &'static str =
        "Windows.UI.Composition.Interactions.InteractionTrackerInteractingStateEnteredArgs";
}
impl ::std::convert::From<InteractionTrackerInteractingStateEnteredArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: InteractionTrackerInteractingStateEnteredArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InteractionTrackerInteractingStateEnteredArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &InteractionTrackerInteractingStateEnteredArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for InteractionTrackerInteractingStateEnteredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &InteractionTrackerInteractingStateEnteredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<InteractionTrackerInteractingStateEnteredArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: InteractionTrackerInteractingStateEnteredArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InteractionTrackerInteractingStateEnteredArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &InteractionTrackerInteractingStateEnteredArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for InteractionTrackerInteractingStateEnteredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InteractionTrackerInteractingStateEnteredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for InteractionTrackerInteractingStateEnteredArgs {}
unsafe impl ::std::marker::Sync for InteractionTrackerInteractingStateEnteredArgs {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct InteractionTrackerPositionUpdateOption(pub i32);
impl InteractionTrackerPositionUpdateOption {
    pub const Default: InteractionTrackerPositionUpdateOption =
        InteractionTrackerPositionUpdateOption(0i32);
    pub const AllowActiveCustomScaleAnimation: InteractionTrackerPositionUpdateOption =
        InteractionTrackerPositionUpdateOption(1i32);
}
impl ::std::convert::From<i32> for InteractionTrackerPositionUpdateOption {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InteractionTrackerPositionUpdateOption {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerPositionUpdateOption {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.UI.Composition.Interactions.InteractionTrackerPositionUpdateOption;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct InteractionTrackerRequestIgnoredArgs(::windows::runtime::IInspectable);
impl InteractionTrackerRequestIgnoredArgs {
    pub fn RequestId(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerRequestIgnoredArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.Composition.Interactions.InteractionTrackerRequestIgnoredArgs;{80dd82f1-ce25-488f-91dd-cb6455ccff2e})" ) ;
}
unsafe impl ::windows::runtime::Interface for InteractionTrackerRequestIgnoredArgs {
    type Vtable = IInteractionTrackerRequestIgnoredArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2162000625,
        52773,
        18575,
        [145, 221, 203, 100, 85, 204, 255, 46],
    );
}
impl ::windows::runtime::RuntimeName for InteractionTrackerRequestIgnoredArgs {
    const NAME: &'static str =
        "Windows.UI.Composition.Interactions.InteractionTrackerRequestIgnoredArgs";
}
impl ::std::convert::From<InteractionTrackerRequestIgnoredArgs> for ::windows::runtime::IUnknown {
    fn from(value: InteractionTrackerRequestIgnoredArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InteractionTrackerRequestIgnoredArgs> for ::windows::runtime::IUnknown {
    fn from(value: &InteractionTrackerRequestIgnoredArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for InteractionTrackerRequestIgnoredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &InteractionTrackerRequestIgnoredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<InteractionTrackerRequestIgnoredArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: InteractionTrackerRequestIgnoredArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InteractionTrackerRequestIgnoredArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &InteractionTrackerRequestIgnoredArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for InteractionTrackerRequestIgnoredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InteractionTrackerRequestIgnoredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for InteractionTrackerRequestIgnoredArgs {}
unsafe impl ::std::marker::Sync for InteractionTrackerRequestIgnoredArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct InteractionTrackerValuesChangedArgs(::windows::runtime::IInspectable);
impl InteractionTrackerValuesChangedArgs {
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Position(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn RequestId(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn Scale(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerValuesChangedArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.Composition.Interactions.InteractionTrackerValuesChangedArgs;{cf1578ef-d3df-4501-b9e6-f02fb22f73d0})" ) ;
}
unsafe impl ::windows::runtime::Interface for InteractionTrackerValuesChangedArgs {
    type Vtable = IInteractionTrackerValuesChangedArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3474290927,
        54239,
        17665,
        [185, 230, 240, 47, 178, 47, 115, 208],
    );
}
impl ::windows::runtime::RuntimeName for InteractionTrackerValuesChangedArgs {
    const NAME: &'static str =
        "Windows.UI.Composition.Interactions.InteractionTrackerValuesChangedArgs";
}
impl ::std::convert::From<InteractionTrackerValuesChangedArgs> for ::windows::runtime::IUnknown {
    fn from(value: InteractionTrackerValuesChangedArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InteractionTrackerValuesChangedArgs> for ::windows::runtime::IUnknown {
    fn from(value: &InteractionTrackerValuesChangedArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for InteractionTrackerValuesChangedArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &InteractionTrackerValuesChangedArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<InteractionTrackerValuesChangedArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: InteractionTrackerValuesChangedArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InteractionTrackerValuesChangedArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &InteractionTrackerValuesChangedArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for InteractionTrackerValuesChangedArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InteractionTrackerValuesChangedArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for InteractionTrackerValuesChangedArgs {}
unsafe impl ::std::marker::Sync for InteractionTrackerValuesChangedArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct InteractionTrackerVector2InertiaModifier(::windows::runtime::IInspectable);
impl InteractionTrackerVector2InertiaModifier {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::super::Foundation::IClosable,
        >(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerVector2InertiaModifier {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.Composition.Interactions.InteractionTrackerVector2InertiaModifier;{87e08ab0-3086-4853-a4b7-77882ad5d7e3})" ) ;
}
unsafe impl ::windows::runtime::Interface for InteractionTrackerVector2InertiaModifier {
    type Vtable = IInteractionTrackerVector2InertiaModifier_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2279639728,
        12422,
        18515,
        [164, 183, 119, 136, 42, 213, 215, 227],
    );
}
impl ::windows::runtime::RuntimeName for InteractionTrackerVector2InertiaModifier {
    const NAME: &'static str =
        "Windows.UI.Composition.Interactions.InteractionTrackerVector2InertiaModifier";
}
impl ::std::convert::From<InteractionTrackerVector2InertiaModifier>
    for ::windows::runtime::IUnknown
{
    fn from(value: InteractionTrackerVector2InertiaModifier) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InteractionTrackerVector2InertiaModifier>
    for ::windows::runtime::IUnknown
{
    fn from(value: &InteractionTrackerVector2InertiaModifier) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for InteractionTrackerVector2InertiaModifier
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &InteractionTrackerVector2InertiaModifier
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<InteractionTrackerVector2InertiaModifier>
    for ::windows::runtime::IInspectable
{
    fn from(value: InteractionTrackerVector2InertiaModifier) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InteractionTrackerVector2InertiaModifier>
    for ::windows::runtime::IInspectable
{
    fn from(value: &InteractionTrackerVector2InertiaModifier) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for InteractionTrackerVector2InertiaModifier
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InteractionTrackerVector2InertiaModifier
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<InteractionTrackerVector2InertiaModifier>
    for super::super::super::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: InteractionTrackerVector2InertiaModifier,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&InteractionTrackerVector2InertiaModifier>
    for super::super::super::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &InteractionTrackerVector2InertiaModifier,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable>
    for InteractionTrackerVector2InertiaModifier
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable>
    for &InteractionTrackerVector2InertiaModifier
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<InteractionTrackerVector2InertiaModifier> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: InteractionTrackerVector2InertiaModifier,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InteractionTrackerVector2InertiaModifier>
    for super::IAnimationObject
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &InteractionTrackerVector2InertiaModifier,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for InteractionTrackerVector2InertiaModifier
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for &InteractionTrackerVector2InertiaModifier
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<InteractionTrackerVector2InertiaModifier> for super::CompositionObject {
    fn from(value: InteractionTrackerVector2InertiaModifier) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&InteractionTrackerVector2InertiaModifier> for super::CompositionObject {
    fn from(value: &InteractionTrackerVector2InertiaModifier) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for InteractionTrackerVector2InertiaModifier
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for &InteractionTrackerVector2InertiaModifier
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for InteractionTrackerVector2InertiaModifier {}
unsafe impl ::std::marker::Sync for InteractionTrackerVector2InertiaModifier {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct InteractionTrackerVector2InertiaNaturalMotion(::windows::runtime::IInspectable);
impl InteractionTrackerVector2InertiaNaturalMotion {
    pub fn Condition(&self) -> ::windows::runtime::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    pub fn SetCondition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ExpressionAnimation>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn NaturalMotion(
        &self,
    ) -> ::windows::runtime::Result<super::Vector2NaturalMotionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Vector2NaturalMotionAnimation>(result__)
        }
    }
    pub fn SetNaturalMotion<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Vector2NaturalMotionAnimation>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::Compositor>>(
        compositor: Param0,
    ) -> ::windows::runtime::Result<InteractionTrackerVector2InertiaNaturalMotion> {
        Self::IInteractionTrackerVector2InertiaNaturalMotionStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                compositor.into_param().abi(),
                &mut result__,
            )
            .from_abi::<InteractionTrackerVector2InertiaNaturalMotion>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::super::Foundation::IClosable,
        >(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IInteractionTrackerVector2InertiaNaturalMotionStatics<
        R,
        F: FnOnce(
            &IInteractionTrackerVector2InertiaNaturalMotionStatics,
        ) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            InteractionTrackerVector2InertiaNaturalMotion,
            IInteractionTrackerVector2InertiaNaturalMotionStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerVector2InertiaNaturalMotion {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.Composition.Interactions.InteractionTrackerVector2InertiaNaturalMotion;{5f17695c-162d-4c07-9400-c282b28276ca})" ) ;
}
unsafe impl ::windows::runtime::Interface for InteractionTrackerVector2InertiaNaturalMotion {
    type Vtable = IInteractionTrackerVector2InertiaNaturalMotion_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1595369820,
        5677,
        19463,
        [148, 0, 194, 130, 178, 130, 118, 202],
    );
}
impl ::windows::runtime::RuntimeName for InteractionTrackerVector2InertiaNaturalMotion {
    const NAME: &'static str =
        "Windows.UI.Composition.Interactions.InteractionTrackerVector2InertiaNaturalMotion";
}
impl ::std::convert::From<InteractionTrackerVector2InertiaNaturalMotion>
    for ::windows::runtime::IUnknown
{
    fn from(value: InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InteractionTrackerVector2InertiaNaturalMotion>
    for ::windows::runtime::IUnknown
{
    fn from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for InteractionTrackerVector2InertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &InteractionTrackerVector2InertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<InteractionTrackerVector2InertiaNaturalMotion>
    for ::windows::runtime::IInspectable
{
    fn from(value: InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InteractionTrackerVector2InertiaNaturalMotion>
    for ::windows::runtime::IInspectable
{
    fn from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for InteractionTrackerVector2InertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InteractionTrackerVector2InertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<InteractionTrackerVector2InertiaNaturalMotion>
    for super::super::super::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: InteractionTrackerVector2InertiaNaturalMotion,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&InteractionTrackerVector2InertiaNaturalMotion>
    for super::super::super::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &InteractionTrackerVector2InertiaNaturalMotion,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable>
    for InteractionTrackerVector2InertiaNaturalMotion
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable>
    for &InteractionTrackerVector2InertiaNaturalMotion
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<InteractionTrackerVector2InertiaNaturalMotion>
    for super::IAnimationObject
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: InteractionTrackerVector2InertiaNaturalMotion,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InteractionTrackerVector2InertiaNaturalMotion>
    for super::IAnimationObject
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &InteractionTrackerVector2InertiaNaturalMotion,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for InteractionTrackerVector2InertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for &InteractionTrackerVector2InertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<InteractionTrackerVector2InertiaNaturalMotion>
    for InteractionTrackerVector2InertiaModifier
{
    fn from(value: InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        ::std::convert::Into::<InteractionTrackerVector2InertiaModifier>::into(&value)
    }
}
impl ::std::convert::From<&InteractionTrackerVector2InertiaNaturalMotion>
    for InteractionTrackerVector2InertiaModifier
{
    fn from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, InteractionTrackerVector2InertiaModifier>
    for InteractionTrackerVector2InertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, InteractionTrackerVector2InertiaModifier> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            InteractionTrackerVector2InertiaModifier,
        >::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, InteractionTrackerVector2InertiaModifier>
    for &InteractionTrackerVector2InertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, InteractionTrackerVector2InertiaModifier> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            InteractionTrackerVector2InertiaModifier,
        >::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InteractionTrackerVector2InertiaNaturalMotion>
    for super::CompositionObject
{
    fn from(value: InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&InteractionTrackerVector2InertiaNaturalMotion>
    for super::CompositionObject
{
    fn from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for InteractionTrackerVector2InertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for &InteractionTrackerVector2InertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for InteractionTrackerVector2InertiaNaturalMotion {}
unsafe impl ::std::marker::Sync for InteractionTrackerVector2InertiaNaturalMotion {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct VisualInteractionSource(::windows::runtime::IInspectable);
impl VisualInteractionSource {
    pub fn IsPositionXRailsEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsPositionXRailsEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsPositionYRailsEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsPositionYRailsEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn ManipulationRedirectionMode(
        &self,
    ) -> ::windows::runtime::Result<VisualInteractionSourceRedirectionMode> {
        let this = self;
        unsafe {
            let mut result__: VisualInteractionSourceRedirectionMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<VisualInteractionSourceRedirectionMode>(result__)
        }
    }
    pub fn SetManipulationRedirectionMode(
        &self,
        value: VisualInteractionSourceRedirectionMode,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn PositionXChainingMode(&self) -> ::windows::runtime::Result<InteractionChainingMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionChainingMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InteractionChainingMode>(result__)
        }
    }
    pub fn SetPositionXChainingMode(
        &self,
        value: InteractionChainingMode,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn PositionXSourceMode(&self) -> ::windows::runtime::Result<InteractionSourceMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionSourceMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InteractionSourceMode>(result__)
        }
    }
    pub fn SetPositionXSourceMode(
        &self,
        value: InteractionSourceMode,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn PositionYChainingMode(&self) -> ::windows::runtime::Result<InteractionChainingMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionChainingMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InteractionChainingMode>(result__)
        }
    }
    pub fn SetPositionYChainingMode(
        &self,
        value: InteractionChainingMode,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn PositionYSourceMode(&self) -> ::windows::runtime::Result<InteractionSourceMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionSourceMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InteractionSourceMode>(result__)
        }
    }
    pub fn SetPositionYSourceMode(
        &self,
        value: InteractionSourceMode,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn ScaleChainingMode(&self) -> ::windows::runtime::Result<InteractionChainingMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionChainingMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InteractionChainingMode>(result__)
        }
    }
    pub fn SetScaleChainingMode(
        &self,
        value: InteractionChainingMode,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn ScaleSourceMode(&self) -> ::windows::runtime::Result<InteractionSourceMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionSourceMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InteractionSourceMode>(result__)
        }
    }
    pub fn SetScaleSourceMode(
        &self,
        value: InteractionSourceMode,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn Source(&self) -> ::windows::runtime::Result<super::Visual> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Visual>(result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn TryRedirectForManipulation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Input::PointerPoint>,
    >(
        &self,
        pointerpoint: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                pointerpoint.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::Visual>>(
        source: Param0,
    ) -> ::windows::runtime::Result<VisualInteractionSource> {
        Self::IVisualInteractionSourceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                source.into_param().abi(),
                &mut result__,
            )
            .from_abi::<VisualInteractionSource>(result__)
        })
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DeltaPosition(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = &::windows::runtime::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn DeltaScale(&self) -> ::windows::runtime::Result<f32> {
        let this = &::windows::runtime::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Position(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = &::windows::runtime::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PositionVelocity(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = &::windows::runtime::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Vector3 =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn Scale(&self) -> ::windows::runtime::Result<f32> {
        let this = &::windows::runtime::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn ScaleVelocity(&self) -> ::windows::runtime::Result<f32> {
        let this = &::windows::runtime::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigureCenterPointXModifiers<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>,
        >,
    >(
        &self,
        conditionalvalues: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                conditionalvalues.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigureCenterPointYModifiers<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>,
        >,
    >(
        &self,
        conditionalvalues: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                conditionalvalues.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigureDeltaPositionXModifiers<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>,
        >,
    >(
        &self,
        conditionalvalues: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                conditionalvalues.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigureDeltaPositionYModifiers<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>,
        >,
    >(
        &self,
        conditionalvalues: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                conditionalvalues.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ConfigureDeltaScaleModifiers<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Foundation::Collections::IIterable<CompositionConditionalValue>,
        >,
    >(
        &self,
        conditionalvalues: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                conditionalvalues.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn PointerWheelConfig(&self) -> ::windows::runtime::Result<InteractionSourceConfiguration> {
        let this = &::windows::runtime::Interface::cast::<IVisualInteractionSource3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InteractionSourceConfiguration>(result__)
        }
    }
    pub fn CreateFromIVisualElement<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::IVisualElement>,
    >(
        source: Param0,
    ) -> ::windows::runtime::Result<VisualInteractionSource> {
        Self::IVisualInteractionSourceStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                source.into_param().abi(),
                &mut result__,
            )
            .from_abi::<VisualInteractionSource>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::super::Foundation::IClosable,
        >(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IVisualInteractionSourceStatics<
        R,
        F: FnOnce(&IVisualInteractionSourceStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            VisualInteractionSource,
            IVisualInteractionSourceStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IVisualInteractionSourceStatics2<
        R,
        F: FnOnce(&IVisualInteractionSourceStatics2) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            VisualInteractionSource,
            IVisualInteractionSourceStatics2,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for VisualInteractionSource {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.Composition.Interactions.VisualInteractionSource;{ca0e8a86-d8d6-4111-b088-70347bd2b0ed})" ) ;
}
unsafe impl ::windows::runtime::Interface for VisualInteractionSource {
    type Vtable = IVisualInteractionSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3389950598,
        55510,
        16657,
        [176, 136, 112, 52, 123, 210, 176, 237],
    );
}
impl ::windows::runtime::RuntimeName for VisualInteractionSource {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.VisualInteractionSource";
}
impl ::std::convert::From<VisualInteractionSource> for ::windows::runtime::IUnknown {
    fn from(value: VisualInteractionSource) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&VisualInteractionSource> for ::windows::runtime::IUnknown {
    fn from(value: &VisualInteractionSource) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for VisualInteractionSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &VisualInteractionSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<VisualInteractionSource> for ::windows::runtime::IInspectable {
    fn from(value: VisualInteractionSource) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VisualInteractionSource> for ::windows::runtime::IInspectable {
    fn from(value: &VisualInteractionSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for VisualInteractionSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a VisualInteractionSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<VisualInteractionSource> for ICompositionInteractionSource {
    type Error = ::windows::runtime::Error;
    fn try_from(value: VisualInteractionSource) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&VisualInteractionSource> for ICompositionInteractionSource {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &VisualInteractionSource) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICompositionInteractionSource>
    for VisualInteractionSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ICompositionInteractionSource> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICompositionInteractionSource>
    for &VisualInteractionSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ICompositionInteractionSource> {
        ::std::convert::TryInto::<ICompositionInteractionSource>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<VisualInteractionSource>
    for super::super::super::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: VisualInteractionSource) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&VisualInteractionSource>
    for super::super::super::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &VisualInteractionSource) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable>
    for VisualInteractionSource
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable>
    for &VisualInteractionSource
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<VisualInteractionSource> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: VisualInteractionSource) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&VisualInteractionSource> for super::IAnimationObject {
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
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<VisualInteractionSource> for super::CompositionObject {
    fn from(value: VisualInteractionSource) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&VisualInteractionSource> for super::CompositionObject {
    fn from(value: &VisualInteractionSource) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for VisualInteractionSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &VisualInteractionSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for VisualInteractionSource {}
unsafe impl ::std::marker::Sync for VisualInteractionSource {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct VisualInteractionSourceRedirectionMode(pub i32);
impl VisualInteractionSourceRedirectionMode {
    pub const Off: VisualInteractionSourceRedirectionMode =
        VisualInteractionSourceRedirectionMode(0i32);
    pub const CapableTouchpadOnly: VisualInteractionSourceRedirectionMode =
        VisualInteractionSourceRedirectionMode(1i32);
    pub const PointerWheelOnly: VisualInteractionSourceRedirectionMode =
        VisualInteractionSourceRedirectionMode(2i32);
    pub const CapableTouchpadAndPointerWheel: VisualInteractionSourceRedirectionMode =
        VisualInteractionSourceRedirectionMode(3i32);
}
impl ::std::convert::From<i32> for VisualInteractionSourceRedirectionMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VisualInteractionSourceRedirectionMode {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for VisualInteractionSourceRedirectionMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.UI.Composition.Interactions.VisualInteractionSourceRedirectionMode;i4)",
    );
}
