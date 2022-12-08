#[doc(hidden)]
#[repr(transparent)]
pub struct IDesktopWindowTarget(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDesktopWindowTarget {
    type Vtable = IDesktopWindowTarget_Vtbl;
}
unsafe impl ::windows::core::Interface for IDesktopWindowTarget {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6329d6ca_3366_490e_9db3_25312929ac51);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowTarget_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsTopmost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Composition_Desktop\"`*"]
#[repr(transparent)]
pub struct DesktopWindowTarget(::windows::core::IUnknown);
impl DesktopWindowTarget {
    pub fn PopulatePropertyInfo(&self, propertyname: &::windows::core::HSTRING, propertyinfo: &super::AnimationPropertyInfo) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).PopulatePropertyInfo)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), ::core::mem::transmute_copy(propertyinfo)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Compositor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Dispatcher)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows::core::HSTRING, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StartAnimation)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.into().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StopAnimation)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Comment)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetComment)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImplicitAnimations)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations(&self, value: &super::ImplicitAnimationCollection) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetImplicitAnimations)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn StartAnimationGroup<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::ICompositionAnimationBase>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StartAnimationGroup)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::ICompositionAnimationBase>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StopAnimationGroup)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetAnimationController)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0>(&self, propertyname: &::windows::core::HSTRING, animation: P0, animationcontroller: &super::AnimationController) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StartAnimationWithController)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.into().abi(), ::core::mem::transmute_copy(animationcontroller)).ok() }
    }
    pub fn Root(&self) -> ::windows::core::Result<super::Visual> {
        let this = &::windows::core::Interface::cast::<super::ICompositionTarget>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Root)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetRoot<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Visual>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionTarget>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetRoot)(::windows::core::Vtable::as_raw(this), value.into().abi()).ok() }
    }
    pub fn IsTopmost(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsTopmost)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for DesktopWindowTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DesktopWindowTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DesktopWindowTarget {}
impl ::core::fmt::Debug for DesktopWindowTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DesktopWindowTarget").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DesktopWindowTarget {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Desktop.DesktopWindowTarget;{6329d6ca-3366-490e-9db3-25312929ac51})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for DesktopWindowTarget {
    type Vtable = IDesktopWindowTarget_Vtbl;
}
unsafe impl ::windows::core::Interface for DesktopWindowTarget {
    const IID: ::windows::core::GUID = <IDesktopWindowTarget as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DesktopWindowTarget {
    const NAME: &'static str = "Windows.UI.Composition.Desktop.DesktopWindowTarget";
}
::windows::core::interface_hierarchy!(DesktopWindowTarget, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<DesktopWindowTarget> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: DesktopWindowTarget) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DesktopWindowTarget> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &DesktopWindowTarget) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&DesktopWindowTarget> for ::windows::core::InParam<super::IAnimationObject> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DesktopWindowTarget) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<DesktopWindowTarget> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: DesktopWindowTarget) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&DesktopWindowTarget> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &DesktopWindowTarget) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&DesktopWindowTarget> for ::windows::core::InParam<super::super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DesktopWindowTarget) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::From<DesktopWindowTarget> for super::CompositionTarget {
    fn from(value: DesktopWindowTarget) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DesktopWindowTarget> for super::CompositionTarget {
    fn from(value: &DesktopWindowTarget) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl ::core::convert::From<&DesktopWindowTarget> for ::windows::core::InParam<super::CompositionTarget> {
    fn from(value: &DesktopWindowTarget) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<DesktopWindowTarget> for super::CompositionObject {
    fn from(value: DesktopWindowTarget) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DesktopWindowTarget> for super::CompositionObject {
    fn from(value: &DesktopWindowTarget) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl ::core::convert::From<&DesktopWindowTarget> for ::windows::core::InParam<super::CompositionObject> {
    fn from(value: &DesktopWindowTarget) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for DesktopWindowTarget {}
unsafe impl ::core::marker::Sync for DesktopWindowTarget {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
