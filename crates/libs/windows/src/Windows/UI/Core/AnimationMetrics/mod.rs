#[doc(hidden)]
#[repr(transparent)]
pub struct IAnimationDescription(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAnimationDescription {
    type Vtable = IAnimationDescription_Vtbl;
}
unsafe impl ::windows::core::Interface for IAnimationDescription {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d11a549_be3d_41de_b081_05c149962f9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnimationDescription_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Animations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Animations: usize,
    #[cfg(feature = "Foundation")]
    pub StaggerDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StaggerDelay: usize,
    pub StaggerDelayFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DelayLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DelayLimit: usize,
    pub ZOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAnimationDescriptionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAnimationDescriptionFactory {
    type Vtable = IAnimationDescriptionFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IAnimationDescriptionFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6e27abe_c1fb_48b5_9271_ecc70ac86ef0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnimationDescriptionFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effect: AnimationEffect, target: AnimationEffectTarget, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOpacityAnimation(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IOpacityAnimation {
    type Vtable = IOpacityAnimation_Vtbl;
}
unsafe impl ::windows::core::Interface for IOpacityAnimation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x803aabe5_ee7e_455f_84e9_2506afb8d2b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpacityAnimation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub InitialOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InitialOpacity: usize,
    pub FinalOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Core_AnimationMetrics\"`*"]
#[repr(transparent)]
pub struct IPropertyAnimation(::windows::core::IUnknown);
impl IPropertyAnimation {
    pub fn Type(&self) -> ::windows::core::Result<PropertyAnimationType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Delay(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Delay)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Duration)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Control1(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Control1)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Control2(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Control2)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IPropertyAnimation, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IPropertyAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPropertyAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyAnimation {}
impl ::core::fmt::Debug for IPropertyAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IPropertyAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{3a01b4da-4d8c-411e-b615-1ade683a9903}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IPropertyAnimation {
    type Vtable = IPropertyAnimation_Vtbl;
}
unsafe impl ::windows::core::Interface for IPropertyAnimation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a01b4da_4d8c_411e_b615_1ade683a9903);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyAnimation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PropertyAnimationType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Delay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Delay: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Foundation")]
    pub Control1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Control1: usize,
    #[cfg(feature = "Foundation")]
    pub Control2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Control2: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScaleAnimation(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IScaleAnimation {
    type Vtable = IScaleAnimation_Vtbl;
}
unsafe impl ::windows::core::Interface for IScaleAnimation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x023552c7_71ab_428c_9c9f_d31780964995);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScaleAnimation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub InitialScaleX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InitialScaleX: usize,
    #[cfg(feature = "Foundation")]
    pub InitialScaleY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InitialScaleY: usize,
    pub FinalScaleX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub FinalScaleY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub NormalizedOrigin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NormalizedOrigin: usize,
}
#[doc = "*Required features: `\"UI_Core_AnimationMetrics\"`*"]
#[repr(transparent)]
pub struct AnimationDescription(::windows::core::IUnknown);
impl AnimationDescription {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Animations(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<IPropertyAnimation>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Animations)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StaggerDelay(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StaggerDelay)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StaggerDelayFactor(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StaggerDelayFactor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DelayLimit(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DelayLimit)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ZOrder(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ZOrder)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateInstance(effect: AnimationEffect, target: AnimationEffectTarget) -> ::windows::core::Result<AnimationDescription> {
        Self::IAnimationDescriptionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(::windows::core::Vtable::as_raw(this), effect, target, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAnimationDescriptionFactory<R, F: FnOnce(&IAnimationDescriptionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<AnimationDescription, IAnimationDescriptionFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for AnimationDescription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AnimationDescription {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AnimationDescription {}
impl ::core::fmt::Debug for AnimationDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AnimationDescription").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AnimationDescription {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.AnimationMetrics.AnimationDescription;{7d11a549-be3d-41de-b081-05c149962f9b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AnimationDescription {
    type Vtable = IAnimationDescription_Vtbl;
}
unsafe impl ::windows::core::Interface for AnimationDescription {
    const IID: ::windows::core::GUID = <IAnimationDescription as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AnimationDescription {
    const NAME: &'static str = "Windows.UI.Core.AnimationMetrics.AnimationDescription";
}
::windows::core::interface_hierarchy!(AnimationDescription, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AnimationDescription {}
unsafe impl ::core::marker::Sync for AnimationDescription {}
#[doc = "*Required features: `\"UI_Core_AnimationMetrics\"`*"]
#[repr(transparent)]
pub struct OpacityAnimation(::windows::core::IUnknown);
impl OpacityAnimation {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InitialOpacity(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InitialOpacity)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn FinalOpacity(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FinalOpacity)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<PropertyAnimationType> {
        let this = &::windows::core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Delay(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Delay)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Duration)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Control1(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = &::windows::core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Control1)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Control2(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = &::windows::core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Control2)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for OpacityAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for OpacityAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OpacityAnimation {}
impl ::core::fmt::Debug for OpacityAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OpacityAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for OpacityAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.AnimationMetrics.OpacityAnimation;{803aabe5-ee7e-455f-84e9-2506afb8d2b4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for OpacityAnimation {
    type Vtable = IOpacityAnimation_Vtbl;
}
unsafe impl ::windows::core::Interface for OpacityAnimation {
    const IID: ::windows::core::GUID = <IOpacityAnimation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for OpacityAnimation {
    const NAME: &'static str = "Windows.UI.Core.AnimationMetrics.OpacityAnimation";
}
::windows::core::interface_hierarchy!(OpacityAnimation, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<OpacityAnimation> for IPropertyAnimation {
    type Error = ::windows::core::Error;
    fn try_from(value: OpacityAnimation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&OpacityAnimation> for IPropertyAnimation {
    type Error = ::windows::core::Error;
    fn try_from(value: &OpacityAnimation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&OpacityAnimation> for ::windows::core::InParam<IPropertyAnimation> {
    type Error = ::windows::core::Error;
    fn try_from(value: &OpacityAnimation) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for OpacityAnimation {}
unsafe impl ::core::marker::Sync for OpacityAnimation {}
#[doc = "*Required features: `\"UI_Core_AnimationMetrics\"`*"]
#[repr(transparent)]
pub struct PropertyAnimation(::windows::core::IUnknown);
impl PropertyAnimation {
    pub fn Type(&self) -> ::windows::core::Result<PropertyAnimationType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Delay(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Delay)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Duration)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Control1(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Control1)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Control2(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Control2)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for PropertyAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PropertyAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PropertyAnimation {}
impl ::core::fmt::Debug for PropertyAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PropertyAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PropertyAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.AnimationMetrics.PropertyAnimation;{3a01b4da-4d8c-411e-b615-1ade683a9903})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PropertyAnimation {
    type Vtable = IPropertyAnimation_Vtbl;
}
unsafe impl ::windows::core::Interface for PropertyAnimation {
    const IID: ::windows::core::GUID = <IPropertyAnimation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PropertyAnimation {
    const NAME: &'static str = "Windows.UI.Core.AnimationMetrics.PropertyAnimation";
}
::windows::core::interface_hierarchy!(PropertyAnimation, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<PropertyAnimation> for IPropertyAnimation {
    type Error = ::windows::core::Error;
    fn try_from(value: PropertyAnimation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PropertyAnimation> for IPropertyAnimation {
    type Error = ::windows::core::Error;
    fn try_from(value: &PropertyAnimation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&PropertyAnimation> for ::windows::core::InParam<IPropertyAnimation> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PropertyAnimation) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for PropertyAnimation {}
unsafe impl ::core::marker::Sync for PropertyAnimation {}
#[doc = "*Required features: `\"UI_Core_AnimationMetrics\"`*"]
#[repr(transparent)]
pub struct ScaleAnimation(::windows::core::IUnknown);
impl ScaleAnimation {
    pub fn Type(&self) -> ::windows::core::Result<PropertyAnimationType> {
        let this = &::windows::core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Delay(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Delay)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Duration)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Control1(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = &::windows::core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Control1)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Control2(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = &::windows::core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Control2)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InitialScaleX(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InitialScaleX)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InitialScaleY(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InitialScaleY)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn FinalScaleX(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FinalScaleX)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn FinalScaleY(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FinalScaleY)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NormalizedOrigin(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NormalizedOrigin)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ScaleAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ScaleAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ScaleAnimation {}
impl ::core::fmt::Debug for ScaleAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScaleAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ScaleAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.AnimationMetrics.ScaleAnimation;{023552c7-71ab-428c-9c9f-d31780964995})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ScaleAnimation {
    type Vtable = IScaleAnimation_Vtbl;
}
unsafe impl ::windows::core::Interface for ScaleAnimation {
    const IID: ::windows::core::GUID = <IScaleAnimation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ScaleAnimation {
    const NAME: &'static str = "Windows.UI.Core.AnimationMetrics.ScaleAnimation";
}
::windows::core::interface_hierarchy!(ScaleAnimation, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<ScaleAnimation> for IPropertyAnimation {
    type Error = ::windows::core::Error;
    fn try_from(value: ScaleAnimation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ScaleAnimation> for IPropertyAnimation {
    type Error = ::windows::core::Error;
    fn try_from(value: &ScaleAnimation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&ScaleAnimation> for ::windows::core::InParam<IPropertyAnimation> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ScaleAnimation) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for ScaleAnimation {}
unsafe impl ::core::marker::Sync for ScaleAnimation {}
#[doc = "*Required features: `\"UI_Core_AnimationMetrics\"`*"]
#[repr(transparent)]
pub struct TranslationAnimation(::windows::core::IUnknown);
impl TranslationAnimation {
    pub fn Type(&self) -> ::windows::core::Result<PropertyAnimationType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Delay(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Delay)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Duration)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Control1(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Control1)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Control2(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Control2)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for TranslationAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TranslationAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TranslationAnimation {}
impl ::core::fmt::Debug for TranslationAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TranslationAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TranslationAnimation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Core.AnimationMetrics.TranslationAnimation;{3a01b4da-4d8c-411e-b615-1ade683a9903})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for TranslationAnimation {
    type Vtable = IPropertyAnimation_Vtbl;
}
unsafe impl ::windows::core::Interface for TranslationAnimation {
    const IID: ::windows::core::GUID = <IPropertyAnimation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TranslationAnimation {
    const NAME: &'static str = "Windows.UI.Core.AnimationMetrics.TranslationAnimation";
}
::windows::core::interface_hierarchy!(TranslationAnimation, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<TranslationAnimation> for IPropertyAnimation {
    type Error = ::windows::core::Error;
    fn try_from(value: TranslationAnimation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&TranslationAnimation> for IPropertyAnimation {
    type Error = ::windows::core::Error;
    fn try_from(value: &TranslationAnimation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&TranslationAnimation> for ::windows::core::InParam<IPropertyAnimation> {
    type Error = ::windows::core::Error;
    fn try_from(value: &TranslationAnimation) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for TranslationAnimation {}
unsafe impl ::core::marker::Sync for TranslationAnimation {}
#[doc = "*Required features: `\"UI_Core_AnimationMetrics\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AnimationEffect(pub i32);
impl AnimationEffect {
    pub const Expand: Self = Self(0i32);
    pub const Collapse: Self = Self(1i32);
    pub const Reposition: Self = Self(2i32);
    pub const FadeIn: Self = Self(3i32);
    pub const FadeOut: Self = Self(4i32);
    pub const AddToList: Self = Self(5i32);
    pub const DeleteFromList: Self = Self(6i32);
    pub const AddToGrid: Self = Self(7i32);
    pub const DeleteFromGrid: Self = Self(8i32);
    pub const AddToSearchGrid: Self = Self(9i32);
    pub const DeleteFromSearchGrid: Self = Self(10i32);
    pub const AddToSearchList: Self = Self(11i32);
    pub const DeleteFromSearchList: Self = Self(12i32);
    pub const ShowEdgeUI: Self = Self(13i32);
    pub const ShowPanel: Self = Self(14i32);
    pub const HideEdgeUI: Self = Self(15i32);
    pub const HidePanel: Self = Self(16i32);
    pub const ShowPopup: Self = Self(17i32);
    pub const HidePopup: Self = Self(18i32);
    pub const PointerDown: Self = Self(19i32);
    pub const PointerUp: Self = Self(20i32);
    pub const DragSourceStart: Self = Self(21i32);
    pub const DragSourceEnd: Self = Self(22i32);
    pub const TransitionContent: Self = Self(23i32);
    pub const Reveal: Self = Self(24i32);
    pub const Hide: Self = Self(25i32);
    pub const DragBetweenEnter: Self = Self(26i32);
    pub const DragBetweenLeave: Self = Self(27i32);
    pub const SwipeSelect: Self = Self(28i32);
    pub const SwipeDeselect: Self = Self(29i32);
    pub const SwipeReveal: Self = Self(30i32);
    pub const EnterPage: Self = Self(31i32);
    pub const TransitionPage: Self = Self(32i32);
    pub const CrossFade: Self = Self(33i32);
    pub const Peek: Self = Self(34i32);
    pub const UpdateBadge: Self = Self(35i32);
}
impl ::core::marker::Copy for AnimationEffect {}
impl ::core::clone::Clone for AnimationEffect {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AnimationEffect {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AnimationEffect {
    type Abi = Self;
}
impl ::core::fmt::Debug for AnimationEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AnimationEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AnimationEffect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Core.AnimationMetrics.AnimationEffect;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Core_AnimationMetrics\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AnimationEffectTarget(pub i32);
impl AnimationEffectTarget {
    pub const Primary: Self = Self(0i32);
    pub const Added: Self = Self(1i32);
    pub const Affected: Self = Self(2i32);
    pub const Background: Self = Self(3i32);
    pub const Content: Self = Self(4i32);
    pub const Deleted: Self = Self(5i32);
    pub const Deselected: Self = Self(6i32);
    pub const DragSource: Self = Self(7i32);
    pub const Hidden: Self = Self(8i32);
    pub const Incoming: Self = Self(9i32);
    pub const Outgoing: Self = Self(10i32);
    pub const Outline: Self = Self(11i32);
    pub const Remaining: Self = Self(12i32);
    pub const Revealed: Self = Self(13i32);
    pub const RowIn: Self = Self(14i32);
    pub const RowOut: Self = Self(15i32);
    pub const Selected: Self = Self(16i32);
    pub const Selection: Self = Self(17i32);
    pub const Shown: Self = Self(18i32);
    pub const Tapped: Self = Self(19i32);
}
impl ::core::marker::Copy for AnimationEffectTarget {}
impl ::core::clone::Clone for AnimationEffectTarget {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AnimationEffectTarget {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AnimationEffectTarget {
    type Abi = Self;
}
impl ::core::fmt::Debug for AnimationEffectTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AnimationEffectTarget").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AnimationEffectTarget {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Core.AnimationMetrics.AnimationEffectTarget;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Core_AnimationMetrics\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PropertyAnimationType(pub i32);
impl PropertyAnimationType {
    pub const Scale: Self = Self(0i32);
    pub const Translation: Self = Self(1i32);
    pub const Opacity: Self = Self(2i32);
}
impl ::core::marker::Copy for PropertyAnimationType {}
impl ::core::clone::Clone for PropertyAnimationType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PropertyAnimationType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PropertyAnimationType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PropertyAnimationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PropertyAnimationType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PropertyAnimationType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Core.AnimationMetrics.PropertyAnimationType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
