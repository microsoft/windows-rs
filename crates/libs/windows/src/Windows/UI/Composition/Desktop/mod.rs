#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"UI_Composition_Desktop\"`*"]
#[repr(transparent)]
pub struct DesktopWindowTarget(::windows::core::IUnknown);
impl DesktopWindowTarget {
    #[doc = "*Required features: `\"UI_Composition_Desktop\"`*"]
    pub fn PopulatePropertyInfo<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::AnimationPropertyInfo>>(&self, propertyname: Param0, propertyinfo: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).PopulatePropertyInfo)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), propertyinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Desktop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Desktop\"`*"]
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Compositor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Desktop\"`, `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Dispatcher)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Desktop\"`*"]
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Desktop\"`*"]
    pub fn StartAnimation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::CompositionAnimation>>(&self, propertyname: Param0, animation: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StartAnimation)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), animation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Desktop\"`*"]
    pub fn StopAnimation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, propertyname: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopAnimation)(::core::mem::transmute_copy(this), propertyname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Desktop\"`*"]
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Comment)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Desktop\"`*"]
    pub fn SetComment<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetComment)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Desktop\"`*"]
    pub fn ImplicitAnimations(&self) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ImplicitAnimations)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Desktop\"`*"]
    pub fn SetImplicitAnimations<'a, Param0: ::windows::core::IntoParam<'a, super::ImplicitAnimationCollection>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetImplicitAnimations)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Desktop\"`*"]
    pub fn StartAnimationGroup<'a, Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StartAnimationGroup)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Desktop\"`*"]
    pub fn StopAnimationGroup<'a, Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopAnimationGroup)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Desktop\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Desktop\"`*"]
    pub fn TryGetAnimationController<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, propertyname: Param0) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryGetAnimationController)(::core::mem::transmute_copy(this), propertyname.into_param().abi(), &mut result__).from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Desktop\"`*"]
    pub fn Root(&self) -> ::windows::core::Result<super::Visual> {
        let this = &::windows::core::Interface::cast::<super::ICompositionTarget>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Root)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Visual>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition_Desktop\"`*"]
    pub fn SetRoot<'a, Param0: ::windows::core::IntoParam<'a, super::Visual>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionTarget>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetRoot)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition_Desktop\"`*"]
    pub fn IsTopmost(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTopmost)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DesktopWindowTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DesktopWindowTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DesktopWindowTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DesktopWindowTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject> for DesktopWindowTarget {
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject> for &DesktopWindowTarget {
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
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
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for DesktopWindowTarget {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &DesktopWindowTarget {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
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
impl<'a> ::windows::core::IntoParam<'a, super::CompositionTarget> for DesktopWindowTarget {
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionTarget> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionTarget> for &DesktopWindowTarget {
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionTarget> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::CompositionTarget>::into(self))
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
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject> for DesktopWindowTarget {
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject> for &DesktopWindowTarget {
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(self))
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
    pub base: ::windows::core::IInspectableVtbl,
    pub IsTopmost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
