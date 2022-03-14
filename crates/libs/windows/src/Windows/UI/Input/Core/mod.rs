#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerIndependentInputSource(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRadialControllerIndependentInputSource {
    type Vtable = IRadialControllerIndependentInputSource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d577ef6_4cee_11e6_b535_001bdc06ab3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerIndependentInputSource_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Controller: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Core")]
    pub Dispatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    Dispatcher: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerIndependentInputSource2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRadialControllerIndependentInputSource2 {
    type Vtable = IRadialControllerIndependentInputSource2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7073aad8_35f3_4eeb_8751_be4d0a66faf4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerIndependentInputSource2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub DispatcherQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    DispatcherQueue: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadialControllerIndependentInputSourceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRadialControllerIndependentInputSourceStatics {
    type Vtable = IRadialControllerIndependentInputSourceStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d577ef5_4cee_11e6_b535_001bdc06ab3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerIndependentInputSourceStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "ApplicationModel_Core")]
    pub CreateForView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, view: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Core"))]
    CreateForView: usize,
}
#[doc = "*Required features: `\"UI_Input_Core\"`*"]
#[repr(transparent)]
pub struct RadialControllerIndependentInputSource(::windows::core::IUnknown);
impl RadialControllerIndependentInputSource {
    #[doc = "*Required features: `\"UI_Input_Core\"`*"]
    pub fn Controller(&self) -> ::windows::core::Result<super::RadialController> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Controller)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::RadialController>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input_Core\"`, `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::core::Result<super::super::Core::CoreDispatcher> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Dispatcher)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input_Core\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<IRadialControllerIndependentInputSource2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input_Core\"`, `\"ApplicationModel_Core\"`*"]
    #[cfg(feature = "ApplicationModel_Core")]
    pub fn CreateForView<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::ApplicationModel::Core::CoreApplicationView>>(view: Param0) -> ::windows::core::Result<RadialControllerIndependentInputSource> {
        Self::IRadialControllerIndependentInputSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateForView)(::core::mem::transmute_copy(this), view.into_param().abi(), &mut result__).from_abi::<RadialControllerIndependentInputSource>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRadialControllerIndependentInputSourceStatics<R, F: FnOnce(&IRadialControllerIndependentInputSourceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RadialControllerIndependentInputSource, IRadialControllerIndependentInputSourceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RadialControllerIndependentInputSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for RadialControllerIndependentInputSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Core.RadialControllerIndependentInputSource;{3d577ef6-4cee-11e6-b535-001bdc06ab3b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RadialControllerIndependentInputSource {
    type Vtable = IRadialControllerIndependentInputSource_Vtbl;
    const IID: ::windows::core::GUID = <IRadialControllerIndependentInputSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RadialControllerIndependentInputSource {
    const NAME: &'static str = "Windows.UI.Input.Core.RadialControllerIndependentInputSource";
}
impl ::core::convert::From<RadialControllerIndependentInputSource> for ::windows::core::IUnknown {
    fn from(value: RadialControllerIndependentInputSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerIndependentInputSource> for ::windows::core::IUnknown {
    fn from(value: &RadialControllerIndependentInputSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RadialControllerIndependentInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RadialControllerIndependentInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RadialControllerIndependentInputSource> for ::windows::core::IInspectable {
    fn from(value: RadialControllerIndependentInputSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RadialControllerIndependentInputSource> for ::windows::core::IInspectable {
    fn from(value: &RadialControllerIndependentInputSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RadialControllerIndependentInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RadialControllerIndependentInputSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RadialControllerIndependentInputSource {}
unsafe impl ::core::marker::Sync for RadialControllerIndependentInputSource {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
