#[doc = "*Required features: `\"UI_Composition_Desktop\"`*"]
#[repr(transparent)]
pub struct DesktopWindowTarget(::windows::core::IUnknown);
impl DesktopWindowTarget {
    #[doc = "*Required features: `\"UI_Composition_Desktop\"`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>, Param1: ::std::convert::Into<::windows::core::InParam<'a, super::AnimationPropertyInfo>>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).PopulatePropertyInfo)(::windows::core::Interface::as_raw(this), propertyname.into().abi(), propertyinfo.into().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Desktop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Desktop\"`*"]
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).Compositor)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Desktop\"`, `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).Dispatcher)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Desktop\"`*"]
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Desktop\"`*"]
    pub fn StartAnimation<'a, Param0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>, Param1: ::std::convert::Into<::windows::core::InParam<'a, super::CompositionAnimation>>>(&self, propertyname: Param0, animation: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StartAnimation)(::windows::core::Interface::as_raw(this), propertyname.into().abi(), animation.into().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Desktop\"`*"]
    pub fn StopAnimation<'a, Param0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>>(&self, propertyname: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopAnimation)(::windows::core::Interface::as_raw(this), propertyname.into().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Desktop\"`*"]
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).Comment)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Desktop\"`*"]
    pub fn SetComment<'a, Param0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetComment)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Desktop\"`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).ImplicitAnimations)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Desktop\"`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::std::convert::Into<::windows::core::InParam<'a, super::ImplicitAnimationCollection>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetImplicitAnimations)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Desktop\"`*"]
    pub fn StartAnimationGroup<'a, Param0: ::std::convert::TryInto<::windows::core::InParam<'a, super::ICompositionAnimationBase>, Error = E0>, E0: ::std::convert::Into<::windows::core::Error>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StartAnimationGroup)(::windows::core::Interface::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Desktop\"`*"]
    pub fn StopAnimationGroup<'a, Param0: ::std::convert::TryInto<::windows::core::InParam<'a, super::ICompositionAnimationBase>, Error = E0>, E0: ::std::convert::Into<::windows::core::Error>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopAnimationGroup)(::windows::core::Interface::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Desktop\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Desktop\"`*"]
    pub fn TryGetAnimationController<'a, Param0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::HSTRING>>>(&self, propertyname: Param0) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).TryGetAnimationController)(::windows::core::Interface::as_raw(this), propertyname.into().abi(), result__.as_mut_ptr()).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Desktop\"`*"]
    pub fn Root(&self) -> ::windows::core::Result<super::Visual> {
        let this = &::windows::core::Interface::cast::<super::ICompositionTarget>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).Root)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Visual>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Desktop\"`*"]
    pub fn SetRoot<'a, Param0: ::std::convert::Into<::windows::core::InParam<'a, super::Visual>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionTarget>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetRoot)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Desktop\"`*"]
    pub fn IsTopmost(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsTopmost)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
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
unsafe impl ::windows::core::Interface for DesktopWindowTarget {
    type Vtable = IDesktopWindowTarget_Vtbl;
    const IID: ::windows::core::GUID = <IDesktopWindowTarget as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DesktopWindowTarget {
    const NAME: &'static str = "Windows.UI.Composition.Desktop.DesktopWindowTarget";
}
impl ::core::convert::From<DesktopWindowTarget> for ::windows::core::IUnknown {
    fn from(value: DesktopWindowTarget) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DesktopWindowTarget> for ::windows::core::IUnknown {
    fn from(value: &DesktopWindowTarget) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DesktopWindowTarget> for &::windows::core::IUnknown {
    fn from(value: &DesktopWindowTarget) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<DesktopWindowTarget> for ::windows::core::IInspectable {
    fn from(value: DesktopWindowTarget) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DesktopWindowTarget> for ::windows::core::IInspectable {
    fn from(value: &DesktopWindowTarget) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DesktopWindowTarget> for &::windows::core::IInspectable {
    fn from(value: &DesktopWindowTarget) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
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
impl<'a> ::core::convert::TryFrom<&DesktopWindowTarget> for ::windows::core::InParam<'a, super::IAnimationObject> {
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
impl<'a> ::core::convert::TryFrom<&DesktopWindowTarget> for ::windows::core::InParam<'a, super::super::super::Foundation::IClosable> {
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
impl<'a> ::core::convert::From<&DesktopWindowTarget> for ::windows::core::InParam<'a, super::CompositionTarget> {
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
impl<'a> ::core::convert::From<&DesktopWindowTarget> for ::windows::core::InParam<'a, super::CompositionObject> {
    fn from(value: &DesktopWindowTarget) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for DesktopWindowTarget {}
unsafe impl ::core::marker::Sync for DesktopWindowTarget {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDesktopWindowTarget(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDesktopWindowTarget {
    type Vtable = IDesktopWindowTarget_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6329d6ca_3366_490e_9db3_25312929ac51);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowTarget_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub IsTopmost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
