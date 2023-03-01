#[doc(hidden)]
#[repr(transparent)]
pub struct IDesktopWindowTarget(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDesktopWindowTarget {
    type Vtable = IDesktopWindowTarget_Vtbl;
}
impl ::core::clone::Clone for IDesktopWindowTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDesktopWindowTarget {
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
        let this = &::windows::core::ComInterface::cast::<super::IAnimationObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).PopulatePropertyInfo)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), ::core::mem::transmute_copy(propertyinfo)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Compositor>();
            (::windows::core::Interface::vtable(this).Compositor)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Core::CoreDispatcher>();
            (::windows::core::Interface::vtable(this).Dispatcher)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::CompositionPropertySet>();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows::core::HSTRING, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::CompositionAnimation>,
    {
        let this = &::windows::core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StartAnimation)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopAnimation)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Comment)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetComment)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::ImplicitAnimationCollection>();
            (::windows::core::Interface::vtable(this).ImplicitAnimations)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations(&self, value: &super::ImplicitAnimationCollection) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetImplicitAnimations)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows::core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StartAnimationGroup)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows::core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StopAnimationGroup)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::core::ComInterface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::System::DispatcherQueue>();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::ComInterface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::AnimationController>();
            (::windows::core::Interface::vtable(this).TryGetAnimationController)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0>(&self, propertyname: &::windows::core::HSTRING, animation: P0, animationcontroller: &super::AnimationController) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::CompositionAnimation>,
    {
        let this = &::windows::core::ComInterface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StartAnimationWithController)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi(), ::core::mem::transmute_copy(animationcontroller)).ok() }
    }
    pub fn Root(&self) -> ::windows::core::Result<super::Visual> {
        let this = &::windows::core::ComInterface::cast::<super::ICompositionTarget>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Visual>();
            (::windows::core::Interface::vtable(this).Root)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRoot<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::Visual>,
    {
        let this = &::windows::core::ComInterface::cast::<super::ICompositionTarget>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetRoot)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn IsTopmost(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsTopmost)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for DesktopWindowTarget {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Desktop.DesktopWindowTarget;{6329d6ca-3366-490e-9db3-25312929ac51})");
}
impl ::core::clone::Clone for DesktopWindowTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for DesktopWindowTarget {
    type Vtable = IDesktopWindowTarget_Vtbl;
}
unsafe impl ::windows::core::ComInterface for DesktopWindowTarget {
    const IID: ::windows::core::GUID = <IDesktopWindowTarget as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for DesktopWindowTarget {
    const NAME: &'static str = "Windows.UI.Composition.Desktop.DesktopWindowTarget";
}
::windows::imp::interface_hierarchy!(DesktopWindowTarget, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<super::IAnimationObject> for DesktopWindowTarget {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::super::Foundation::IClosable> for DesktopWindowTarget {}
impl ::windows::core::CanTryInto<super::CompositionTarget> for DesktopWindowTarget {}
impl ::windows::core::CanTryInto<super::CompositionObject> for DesktopWindowTarget {}
unsafe impl ::core::marker::Send for DesktopWindowTarget {}
unsafe impl ::core::marker::Sync for DesktopWindowTarget {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
