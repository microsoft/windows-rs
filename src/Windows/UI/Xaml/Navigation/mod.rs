#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FrameNavigationOptions(pub ::windows::core::IInspectable);
impl FrameNavigationOptions {
    pub fn IsNavigationStackEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsNavigationStackEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    pub fn TransitionInfoOverride(&self) -> ::windows::core::Result<super::Media::Animation::NavigationTransitionInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::Animation::NavigationTransitionInfo>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    pub fn SetTransitionInfoOverride<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Animation::NavigationTransitionInfo>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn new() -> ::windows::core::Result<FrameNavigationOptions> {
        Self::IFrameNavigationOptionsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<FrameNavigationOptions>(result__)
        })
    }
    pub fn IFrameNavigationOptionsFactory<R, F: FnOnce(&IFrameNavigationOptionsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FrameNavigationOptions, IFrameNavigationOptionsFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for FrameNavigationOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Navigation.FrameNavigationOptions;{b539ad2a-9fb7-520a-8f41-57a50c59cf92})");
}
unsafe impl ::windows::core::Interface for FrameNavigationOptions {
    type Vtable = IFrameNavigationOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb539ad2a_9fb7_520a_8f41_57a50c59cf92);
}
impl ::windows::core::RuntimeName for FrameNavigationOptions {
    const NAME: &'static str = "Windows.UI.Xaml.Navigation.FrameNavigationOptions";
}
impl ::core::convert::From<FrameNavigationOptions> for ::windows::core::IUnknown {
    fn from(value: FrameNavigationOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FrameNavigationOptions> for ::windows::core::IUnknown {
    fn from(value: &FrameNavigationOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FrameNavigationOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FrameNavigationOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FrameNavigationOptions> for ::windows::core::IInspectable {
    fn from(value: FrameNavigationOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FrameNavigationOptions> for ::windows::core::IInspectable {
    fn from(value: &FrameNavigationOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FrameNavigationOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FrameNavigationOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for FrameNavigationOptions {}
unsafe impl ::core::marker::Sync for FrameNavigationOptions {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IFrameNavigationOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFrameNavigationOptions {
    type Vtable = IFrameNavigationOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb539ad2a_9fb7_520a_8f41_57a50c59cf92);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameNavigationOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media_Animation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Animation"))] usize,
    #[cfg(feature = "UI_Xaml_Media_Animation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Animation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFrameNavigationOptionsFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFrameNavigationOptionsFactory {
    type Vtable = IFrameNavigationOptionsFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4681e41_7e6d_5c7c_aca0_478681cc6fce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameNavigationOptionsFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, baseinterface: ::windows::core::RawPtr, innerinterface: *mut ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct INavigatingCancelEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INavigatingCancelEventArgs {
    type Vtable = INavigatingCancelEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd1d67ae_eafb_4079_be80_6dc92a03aedf);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigatingCancelEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut NavigationMode) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Interop")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct INavigatingCancelEventArgs2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INavigatingCancelEventArgs2 {
    type Vtable = INavigatingCancelEventArgs2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5407b704_8147_4343_838f_dd1ee908c137);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigatingCancelEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media_Animation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Animation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct INavigationEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INavigationEventArgs {
    type Vtable = INavigationEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6aa9834_6691_44d1_bdf7_58820c27b0d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Interop")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut NavigationMode) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct INavigationEventArgs2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INavigationEventArgs2 {
    type Vtable = INavigationEventArgs2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdbff71d9_979a_4b2e_a49b_3bb17fdef574);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media_Animation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Animation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct INavigationFailedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INavigationFailedEventArgs {
    type Vtable = INavigationFailedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11c1dff7_36c2_4102_b2ef_0217a97289b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationFailedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Interop")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPageStackEntry(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPageStackEntry {
    type Vtable = IPageStackEntry_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef8814a6_9388_4aca_8572_405194069080);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPageStackEntry_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Interop")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media_Animation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Animation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPageStackEntryFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPageStackEntryFactory {
    type Vtable = IPageStackEntryFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4454048a_a8b9_4f78_9b84_1f51f58851ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPageStackEntryFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "UI_Xaml_Interop", feature = "UI_Xaml_Media_Animation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sourcepagetype: ::core::mem::ManuallyDrop<super::Interop::TypeName>, parameter: ::windows::core::RawPtr, navigationtransitioninfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "UI_Xaml_Interop", feature = "UI_Xaml_Media_Animation")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPageStackEntryStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPageStackEntryStatics {
    type Vtable = IPageStackEntryStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaceff8e3_246c_4033_9f01_01cb0da5254e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPageStackEntryStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LoadCompletedEventHandler(::windows::core::IUnknown);
impl LoadCompletedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<NavigationEventArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = LoadCompletedEventHandler_box::<F> { vtable: &LoadCompletedEventHandler_box::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, NavigationEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for LoadCompletedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({aebaf785-43fc-4e2c-95c3-97ae84eabc8e})");
}
unsafe impl ::windows::core::Interface for LoadCompletedEventHandler {
    type Vtable = LoadCompletedEventHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaebaf785_43fc_4e2c_95c3_97ae84eabc8e);
}
#[repr(C)]
#[doc(hidden)]
pub struct LoadCompletedEventHandler_abi(pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[repr(C)]
struct LoadCompletedEventHandler_box<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<NavigationEventArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const LoadCompletedEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<NavigationEventArgs>) -> ::windows::core::Result<()> + 'static> LoadCompletedEventHandler_box<F> {
    const VTABLE: LoadCompletedEventHandler_abi = LoadCompletedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<LoadCompletedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&sender as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&e as *const <NavigationEventArgs as ::windows::core::Abi>::Abi as *const <NavigationEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct NavigatedEventHandler(::windows::core::IUnknown);
impl NavigatedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<NavigationEventArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = NavigatedEventHandler_box::<F> { vtable: &NavigatedEventHandler_box::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, NavigationEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for NavigatedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({7bd1cf54-23cf-4cce-b2f5-4ce78d96896e})");
}
unsafe impl ::windows::core::Interface for NavigatedEventHandler {
    type Vtable = NavigatedEventHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7bd1cf54_23cf_4cce_b2f5_4ce78d96896e);
}
#[repr(C)]
#[doc(hidden)]
pub struct NavigatedEventHandler_abi(pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[repr(C)]
struct NavigatedEventHandler_box<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<NavigationEventArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const NavigatedEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<NavigationEventArgs>) -> ::windows::core::Result<()> + 'static> NavigatedEventHandler_box<F> {
    const VTABLE: NavigatedEventHandler_abi = NavigatedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<NavigatedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&sender as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&e as *const <NavigationEventArgs as ::windows::core::Abi>::Abi as *const <NavigationEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct NavigatingCancelEventArgs(pub ::windows::core::IInspectable);
impl NavigatingCancelEventArgs {
    pub fn Cancel(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetCancel(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn NavigationMode(&self) -> ::windows::core::Result<NavigationMode> {
        let this = self;
        unsafe {
            let mut result__: NavigationMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NavigationMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Interop")]
    pub fn SourcePageType(&self) -> ::windows::core::Result<super::Interop::TypeName> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<super::Interop::TypeName> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Interop::TypeName>(result__)
        }
    }
    pub fn Parameter(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<INavigatingCancelEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    pub fn NavigationTransitionInfo(&self) -> ::windows::core::Result<super::Media::Animation::NavigationTransitionInfo> {
        let this = &::windows::core::Interface::cast::<INavigatingCancelEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::Animation::NavigationTransitionInfo>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for NavigatingCancelEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Navigation.NavigatingCancelEventArgs;{fd1d67ae-eafb-4079-be80-6dc92a03aedf})");
}
unsafe impl ::windows::core::Interface for NavigatingCancelEventArgs {
    type Vtable = INavigatingCancelEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd1d67ae_eafb_4079_be80_6dc92a03aedf);
}
impl ::windows::core::RuntimeName for NavigatingCancelEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Navigation.NavigatingCancelEventArgs";
}
impl ::core::convert::From<NavigatingCancelEventArgs> for ::windows::core::IUnknown {
    fn from(value: NavigatingCancelEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&NavigatingCancelEventArgs> for ::windows::core::IUnknown {
    fn from(value: &NavigatingCancelEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NavigatingCancelEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a NavigatingCancelEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<NavigatingCancelEventArgs> for ::windows::core::IInspectable {
    fn from(value: NavigatingCancelEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&NavigatingCancelEventArgs> for ::windows::core::IInspectable {
    fn from(value: &NavigatingCancelEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NavigatingCancelEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a NavigatingCancelEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for NavigatingCancelEventArgs {}
unsafe impl ::core::marker::Sync for NavigatingCancelEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct NavigatingCancelEventHandler(::windows::core::IUnknown);
impl NavigatingCancelEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<NavigatingCancelEventArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = NavigatingCancelEventHandler_box::<F> { vtable: &NavigatingCancelEventHandler_box::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, NavigatingCancelEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for NavigatingCancelEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({75d6a78f-a302-4489-9898-24ea49182910})");
}
unsafe impl ::windows::core::Interface for NavigatingCancelEventHandler {
    type Vtable = NavigatingCancelEventHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75d6a78f_a302_4489_9898_24ea49182910);
}
#[repr(C)]
#[doc(hidden)]
pub struct NavigatingCancelEventHandler_abi(pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[repr(C)]
struct NavigatingCancelEventHandler_box<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<NavigatingCancelEventArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const NavigatingCancelEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<NavigatingCancelEventArgs>) -> ::windows::core::Result<()> + 'static> NavigatingCancelEventHandler_box<F> {
    const VTABLE: NavigatingCancelEventHandler_abi = NavigatingCancelEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<NavigatingCancelEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&sender as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&e as *const <NavigatingCancelEventArgs as ::windows::core::Abi>::Abi as *const <NavigatingCancelEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NavigationCacheMode(pub i32);
impl NavigationCacheMode {
    pub const Disabled: NavigationCacheMode = NavigationCacheMode(0i32);
    pub const Required: NavigationCacheMode = NavigationCacheMode(1i32);
    pub const Enabled: NavigationCacheMode = NavigationCacheMode(2i32);
}
impl ::core::convert::From<i32> for NavigationCacheMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NavigationCacheMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for NavigationCacheMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Navigation.NavigationCacheMode;i4)");
}
impl ::windows::core::DefaultType for NavigationCacheMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct NavigationEventArgs(pub ::windows::core::IInspectable);
impl NavigationEventArgs {
    pub fn Content(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Parameter(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Interop")]
    pub fn SourcePageType(&self) -> ::windows::core::Result<super::Interop::TypeName> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<super::Interop::TypeName> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Interop::TypeName>(result__)
        }
    }
    pub fn NavigationMode(&self) -> ::windows::core::Result<NavigationMode> {
        let this = self;
        unsafe {
            let mut result__: NavigationMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NavigationMode>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    pub fn NavigationTransitionInfo(&self) -> ::windows::core::Result<super::Media::Animation::NavigationTransitionInfo> {
        let this = &::windows::core::Interface::cast::<INavigationEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::Animation::NavigationTransitionInfo>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for NavigationEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Navigation.NavigationEventArgs;{b6aa9834-6691-44d1-bdf7-58820c27b0d0})");
}
unsafe impl ::windows::core::Interface for NavigationEventArgs {
    type Vtable = INavigationEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6aa9834_6691_44d1_bdf7_58820c27b0d0);
}
impl ::windows::core::RuntimeName for NavigationEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Navigation.NavigationEventArgs";
}
impl ::core::convert::From<NavigationEventArgs> for ::windows::core::IUnknown {
    fn from(value: NavigationEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&NavigationEventArgs> for ::windows::core::IUnknown {
    fn from(value: &NavigationEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NavigationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a NavigationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<NavigationEventArgs> for ::windows::core::IInspectable {
    fn from(value: NavigationEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&NavigationEventArgs> for ::windows::core::IInspectable {
    fn from(value: &NavigationEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NavigationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a NavigationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for NavigationEventArgs {}
unsafe impl ::core::marker::Sync for NavigationEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct NavigationFailedEventArgs(pub ::windows::core::IInspectable);
impl NavigationFailedEventArgs {
    pub fn Exception(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Interop")]
    pub fn SourcePageType(&self) -> ::windows::core::Result<super::Interop::TypeName> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<super::Interop::TypeName> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Interop::TypeName>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for NavigationFailedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Navigation.NavigationFailedEventArgs;{11c1dff7-36c2-4102-b2ef-0217a97289b3})");
}
unsafe impl ::windows::core::Interface for NavigationFailedEventArgs {
    type Vtable = INavigationFailedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11c1dff7_36c2_4102_b2ef_0217a97289b3);
}
impl ::windows::core::RuntimeName for NavigationFailedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Navigation.NavigationFailedEventArgs";
}
impl ::core::convert::From<NavigationFailedEventArgs> for ::windows::core::IUnknown {
    fn from(value: NavigationFailedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&NavigationFailedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &NavigationFailedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NavigationFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a NavigationFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<NavigationFailedEventArgs> for ::windows::core::IInspectable {
    fn from(value: NavigationFailedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&NavigationFailedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &NavigationFailedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NavigationFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a NavigationFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for NavigationFailedEventArgs {}
unsafe impl ::core::marker::Sync for NavigationFailedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct NavigationFailedEventHandler(::windows::core::IUnknown);
impl NavigationFailedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<NavigationFailedEventArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = NavigationFailedEventHandler_box::<F> { vtable: &NavigationFailedEventHandler_box::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, NavigationFailedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for NavigationFailedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({4dab4671-12b2-43c7-b892-9be2dcd3e88d})");
}
unsafe impl ::windows::core::Interface for NavigationFailedEventHandler {
    type Vtable = NavigationFailedEventHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4dab4671_12b2_43c7_b892_9be2dcd3e88d);
}
#[repr(C)]
#[doc(hidden)]
pub struct NavigationFailedEventHandler_abi(pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[repr(C)]
struct NavigationFailedEventHandler_box<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<NavigationFailedEventArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const NavigationFailedEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<NavigationFailedEventArgs>) -> ::windows::core::Result<()> + 'static> NavigationFailedEventHandler_box<F> {
    const VTABLE: NavigationFailedEventHandler_abi = NavigationFailedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<NavigationFailedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&sender as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&e as *const <NavigationFailedEventArgs as ::windows::core::Abi>::Abi as *const <NavigationFailedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NavigationMode(pub i32);
impl NavigationMode {
    pub const New: NavigationMode = NavigationMode(0i32);
    pub const Back: NavigationMode = NavigationMode(1i32);
    pub const Forward: NavigationMode = NavigationMode(2i32);
    pub const Refresh: NavigationMode = NavigationMode(3i32);
}
impl ::core::convert::From<i32> for NavigationMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NavigationMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for NavigationMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Navigation.NavigationMode;i4)");
}
impl ::windows::core::DefaultType for NavigationMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct NavigationStoppedEventHandler(::windows::core::IUnknown);
impl NavigationStoppedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<NavigationEventArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = NavigationStoppedEventHandler_box::<F> { vtable: &NavigationStoppedEventHandler_box::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, NavigationEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for NavigationStoppedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({f0117ddb-12fa-4d8d-8b26-b383d09c2b3c})");
}
unsafe impl ::windows::core::Interface for NavigationStoppedEventHandler {
    type Vtable = NavigationStoppedEventHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0117ddb_12fa_4d8d_8b26_b383d09c2b3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct NavigationStoppedEventHandler_abi(pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[repr(C)]
struct NavigationStoppedEventHandler_box<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<NavigationEventArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const NavigationStoppedEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<NavigationEventArgs>) -> ::windows::core::Result<()> + 'static> NavigationStoppedEventHandler_box<F> {
    const VTABLE: NavigationStoppedEventHandler_abi = NavigationStoppedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<NavigationStoppedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&sender as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&e as *const <NavigationEventArgs as ::windows::core::Abi>::Abi as *const <NavigationEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PageStackEntry(pub ::windows::core::IInspectable);
impl PageStackEntry {
    #[cfg(feature = "UI_Xaml_Interop")]
    pub fn SourcePageType(&self) -> ::windows::core::Result<super::Interop::TypeName> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<super::Interop::TypeName> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Interop::TypeName>(result__)
        }
    }
    pub fn Parameter(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    pub fn NavigationTransitionInfo(&self) -> ::windows::core::Result<super::Media::Animation::NavigationTransitionInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::Animation::NavigationTransitionInfo>(result__)
        }
    }
    #[cfg(all(feature = "UI_Xaml_Interop", feature = "UI_Xaml_Media_Animation"))]
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, super::Interop::TypeName>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param2: ::windows::core::IntoParam<'a, super::Media::Animation::NavigationTransitionInfo>>(sourcepagetype: Param0, parameter: Param1, navigationtransitioninfo: Param2) -> ::windows::core::Result<PageStackEntry> {
        Self::IPageStackEntryFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), sourcepagetype.into_param().abi(), parameter.into_param().abi(), navigationtransitioninfo.into_param().abi(), &mut result__).from_abi::<PageStackEntry>(result__)
        })
    }
    pub fn SourcePageTypeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IPageStackEntryStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IPageStackEntryFactory<R, F: FnOnce(&IPageStackEntryFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PageStackEntry, IPageStackEntryFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPageStackEntryStatics<R, F: FnOnce(&IPageStackEntryStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PageStackEntry, IPageStackEntryStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PageStackEntry {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Navigation.PageStackEntry;{ef8814a6-9388-4aca-8572-405194069080})");
}
unsafe impl ::windows::core::Interface for PageStackEntry {
    type Vtable = IPageStackEntry_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef8814a6_9388_4aca_8572_405194069080);
}
impl ::windows::core::RuntimeName for PageStackEntry {
    const NAME: &'static str = "Windows.UI.Xaml.Navigation.PageStackEntry";
}
impl ::core::convert::From<PageStackEntry> for ::windows::core::IUnknown {
    fn from(value: PageStackEntry) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PageStackEntry> for ::windows::core::IUnknown {
    fn from(value: &PageStackEntry) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PageStackEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PageStackEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PageStackEntry> for ::windows::core::IInspectable {
    fn from(value: PageStackEntry) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PageStackEntry> for ::windows::core::IInspectable {
    fn from(value: &PageStackEntry) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PageStackEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PageStackEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PageStackEntry> for super::DependencyObject {
    fn from(value: PageStackEntry) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&PageStackEntry> for super::DependencyObject {
    fn from(value: &PageStackEntry) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for PageStackEntry {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &PageStackEntry {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(::core::clone::Clone::clone(self)))
    }
}
unsafe impl ::core::marker::Send for PageStackEntry {}
unsafe impl ::core::marker::Sync for PageStackEntry {}
