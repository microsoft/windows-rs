#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerIndependentInputSource(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRadialControllerIndependentInputSource {
    type Vtable = IRadialControllerIndependentInputSource_Vtbl;
}
impl ::core::clone::Clone for IRadialControllerIndependentInputSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for IRadialControllerIndependentInputSource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d577ef6_4cee_11e6_b535_001bdc06ab3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerIndependentInputSource_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Controller: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Core")]
    pub Dispatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    Dispatcher: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerIndependentInputSource2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRadialControllerIndependentInputSource2 {
    type Vtable = IRadialControllerIndependentInputSource2_Vtbl;
}
impl ::core::clone::Clone for IRadialControllerIndependentInputSource2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for IRadialControllerIndependentInputSource2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7073aad8_35f3_4eeb_8751_be4d0a66faf4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerIndependentInputSource2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub DispatcherQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    DispatcherQueue: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerIndependentInputSourceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRadialControllerIndependentInputSourceStatics {
    type Vtable = IRadialControllerIndependentInputSourceStatics_Vtbl;
}
impl ::core::clone::Clone for IRadialControllerIndependentInputSourceStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for IRadialControllerIndependentInputSourceStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d577ef5_4cee_11e6_b535_001bdc06ab3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerIndependentInputSourceStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_Core")]
    pub CreateForView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, view: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Core"))]
    CreateForView: usize,
}
#[doc = "*Required features: `\"UI_Input_Core\"`*"]
#[repr(transparent)]
pub struct RadialControllerIndependentInputSource(::windows::core::IUnknown);
impl RadialControllerIndependentInputSource {
    pub fn Controller(&self) -> ::windows::core::Result<super::RadialController> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::RadialController>();
            (::windows::core::Vtable::vtable(this).Controller)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::core::Result<super::super::Core::CoreDispatcher> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Core::CoreDispatcher>();
            (::windows::core::Vtable::vtable(this).Dispatcher)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<IRadialControllerIndependentInputSource2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::System::DispatcherQueue>();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
    #[cfg(feature = "ApplicationModel_Core")]
    pub fn CreateForView(view: &super::super::super::ApplicationModel::Core::CoreApplicationView) -> ::windows::core::Result<RadialControllerIndependentInputSource> {
        Self::IRadialControllerIndependentInputSourceStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<RadialControllerIndependentInputSource>();
            (::windows::core::Vtable::vtable(this).CreateForView)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(view), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRadialControllerIndependentInputSourceStatics<R, F: FnOnce(&IRadialControllerIndependentInputSourceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<RadialControllerIndependentInputSource, IRadialControllerIndependentInputSourceStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for RadialControllerIndependentInputSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RadialControllerIndependentInputSource {}
impl ::core::fmt::Debug for RadialControllerIndependentInputSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadialControllerIndependentInputSource").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for RadialControllerIndependentInputSource {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Core.RadialControllerIndependentInputSource;{3d577ef6-4cee-11e6-b535-001bdc06ab3b})");
}
impl ::core::clone::Clone for RadialControllerIndependentInputSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for RadialControllerIndependentInputSource {
    type Vtable = IRadialControllerIndependentInputSource_Vtbl;
}
unsafe impl ::windows::core::Interface for RadialControllerIndependentInputSource {
    const IID: ::windows::core::GUID = <IRadialControllerIndependentInputSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RadialControllerIndependentInputSource {
    const NAME: &'static str = "Windows.UI.Input.Core.RadialControllerIndependentInputSource";
}
::windows::imp::interface_hierarchy!(RadialControllerIndependentInputSource, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RadialControllerIndependentInputSource {}
unsafe impl ::core::marker::Sync for RadialControllerIndependentInputSource {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
