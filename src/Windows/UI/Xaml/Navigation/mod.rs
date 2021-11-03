#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `UI_Xaml_Navigation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct FrameNavigationOptions(::windows::runtime::IInspectable);
impl FrameNavigationOptions {
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn IsNavigationStackEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn SetIsNavigationStackEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    #[doc = "*Required features: `UI_Xaml_Navigation`, `UI_Xaml_Media_Animation`*"]
    pub fn TransitionInfoOverride(&self) -> ::windows::runtime::Result<super::Media::Animation::NavigationTransitionInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::Animation::NavigationTransitionInfo>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    #[doc = "*Required features: `UI_Xaml_Navigation`, `UI_Xaml_Media_Animation`*"]
    pub fn SetTransitionInfoOverride<'a, Param0: ::windows::runtime::IntoParam<'a, super::Media::Animation::NavigationTransitionInfo>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn new() -> ::windows::runtime::Result<FrameNavigationOptions> {
        Self::IFrameNavigationOptionsFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), ::std::ptr::null_mut(), &mut ::std::option::Option::<::windows::runtime::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<FrameNavigationOptions>(result__)
        })
    }
    pub fn IFrameNavigationOptionsFactory<R, F: FnOnce(&IFrameNavigationOptionsFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<FrameNavigationOptions, IFrameNavigationOptionsFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FrameNavigationOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Navigation.FrameNavigationOptions;{b539ad2a-9fb7-520a-8f41-57a50c59cf92})");
}
unsafe impl ::windows::runtime::Interface for FrameNavigationOptions {
    type Vtable = IFrameNavigationOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3040455978, 40887, 21002, [143, 65, 87, 165, 12, 89, 207, 146]);
}
impl ::windows::runtime::RuntimeName for FrameNavigationOptions {
    const NAME: &'static str = "Windows.UI.Xaml.Navigation.FrameNavigationOptions";
}
impl ::std::convert::From<FrameNavigationOptions> for ::windows::runtime::IUnknown {
    fn from(value: FrameNavigationOptions) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&FrameNavigationOptions> for ::windows::runtime::IUnknown {
    fn from(value: &FrameNavigationOptions) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for FrameNavigationOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &FrameNavigationOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<FrameNavigationOptions> for ::windows::runtime::IInspectable {
    fn from(value: FrameNavigationOptions) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FrameNavigationOptions> for ::windows::runtime::IInspectable {
    fn from(value: &FrameNavigationOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for FrameNavigationOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a FrameNavigationOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for FrameNavigationOptions {}
unsafe impl ::std::marker::Sync for FrameNavigationOptions {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IFrameNavigationOptions(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFrameNavigationOptions {
    type Vtable = IFrameNavigationOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3040455978, 40887, 21002, [143, 65, 87, 165, 12, 89, 207, 146]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameNavigationOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Media_Animation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Animation"))] usize,
    #[cfg(feature = "UI_Xaml_Media_Animation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Animation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFrameNavigationOptionsFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFrameNavigationOptionsFactory {
    type Vtable = IFrameNavigationOptionsFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3563593281, 32365, 23676, [172, 160, 71, 134, 129, 204, 111, 206]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameNavigationOptionsFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, baseinterface: ::windows::runtime::RawPtr, innerinterface: *mut ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct INavigatingCancelEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INavigatingCancelEventArgs {
    type Vtable = INavigatingCancelEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4246562734, 60155, 16505, [190, 128, 109, 201, 42, 3, 174, 223]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigatingCancelEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut NavigationMode) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Interop")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct INavigatingCancelEventArgs2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INavigatingCancelEventArgs2 {
    type Vtable = INavigatingCancelEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1409791748, 33095, 17219, [131, 143, 221, 30, 233, 8, 193, 55]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigatingCancelEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Media_Animation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Animation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct INavigationEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INavigationEventArgs {
    type Vtable = INavigationEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3064633396, 26257, 17617, [189, 247, 88, 130, 12, 39, 176, 208]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Interop")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut NavigationMode) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct INavigationEventArgs2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INavigationEventArgs2 {
    type Vtable = INavigationEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3690951129, 38810, 19246, [164, 155, 59, 177, 127, 222, 245, 116]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Media_Animation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Animation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct INavigationFailedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INavigationFailedEventArgs {
    type Vtable = INavigationFailedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(297918455, 14018, 16642, [178, 239, 2, 23, 169, 114, 137, 179]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationFailedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Interop")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPageStackEntry(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPageStackEntry {
    type Vtable = IPageStackEntry_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4018672806, 37768, 19146, [133, 114, 64, 81, 148, 6, 144, 128]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPageStackEntry_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Interop")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Media_Animation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Animation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPageStackEntryFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPageStackEntryFactory {
    type Vtable = IPageStackEntryFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1146356874, 43193, 20344, [155, 132, 31, 81, 245, 136, 81, 255]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPageStackEntryFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "UI_Xaml_Interop", feature = "UI_Xaml_Media_Animation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourcepagetype: ::std::mem::ManuallyDrop<super::Interop::TypeName>, parameter: ::windows::runtime::RawPtr, navigationtransitioninfo: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "UI_Xaml_Interop", feature = "UI_Xaml_Media_Animation")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPageStackEntryStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPageStackEntryStatics {
    type Vtable = IPageStackEntryStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2901407971, 9324, 16435, [159, 1, 1, 203, 13, 165, 37, 78]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPageStackEntryStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `UI_Xaml_Navigation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LoadCompletedEventHandler(::windows::runtime::IUnknown);
impl LoadCompletedEventHandler {
    pub fn new<F: FnMut(&::std::option::Option<::windows::runtime::IInspectable>, &::std::option::Option<NavigationEventArgs>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = LoadCompletedEventHandler_box::<F> {
            vtable: &LoadCompletedEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, NavigationEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LoadCompletedEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({aebaf785-43fc-4e2c-95c3-97ae84eabc8e})");
}
unsafe impl ::windows::runtime::Interface for LoadCompletedEventHandler {
    type Vtable = LoadCompletedEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2931488645, 17404, 20012, [149, 195, 151, 174, 132, 234, 188, 142]);
}
#[repr(C)]
#[doc(hidden)]
pub struct LoadCompletedEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct LoadCompletedEventHandler_box<F: FnMut(&::std::option::Option<::windows::runtime::IInspectable>, &::std::option::Option<NavigationEventArgs>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const LoadCompletedEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::std::option::Option<::windows::runtime::IInspectable>, &::std::option::Option<NavigationEventArgs>) -> ::windows::runtime::Result<()> + 'static> LoadCompletedEventHandler_box<F> {
    const VTABLE: LoadCompletedEventHandler_abi = LoadCompletedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<LoadCompletedEventHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <::windows::runtime::IInspectable as ::windows::runtime::Abi>::Abi as *const <::windows::runtime::IInspectable as ::windows::runtime::DefaultType>::DefaultType),
            &*(&e as *const <NavigationEventArgs as ::windows::runtime::Abi>::Abi as *const <NavigationEventArgs as ::windows::runtime::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[doc = "*Required features: `UI_Xaml_Navigation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct NavigatedEventHandler(::windows::runtime::IUnknown);
impl NavigatedEventHandler {
    pub fn new<F: FnMut(&::std::option::Option<::windows::runtime::IInspectable>, &::std::option::Option<NavigationEventArgs>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = NavigatedEventHandler_box::<F> {
            vtable: &NavigatedEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, NavigationEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NavigatedEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({7bd1cf54-23cf-4cce-b2f5-4ce78d96896e})");
}
unsafe impl ::windows::runtime::Interface for NavigatedEventHandler {
    type Vtable = NavigatedEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2077347668, 9167, 19662, [178, 245, 76, 231, 141, 150, 137, 110]);
}
#[repr(C)]
#[doc(hidden)]
pub struct NavigatedEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct NavigatedEventHandler_box<F: FnMut(&::std::option::Option<::windows::runtime::IInspectable>, &::std::option::Option<NavigationEventArgs>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const NavigatedEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::std::option::Option<::windows::runtime::IInspectable>, &::std::option::Option<NavigationEventArgs>) -> ::windows::runtime::Result<()> + 'static> NavigatedEventHandler_box<F> {
    const VTABLE: NavigatedEventHandler_abi = NavigatedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<NavigatedEventHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <::windows::runtime::IInspectable as ::windows::runtime::Abi>::Abi as *const <::windows::runtime::IInspectable as ::windows::runtime::DefaultType>::DefaultType),
            &*(&e as *const <NavigationEventArgs as ::windows::runtime::Abi>::Abi as *const <NavigationEventArgs as ::windows::runtime::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[doc = "*Required features: `UI_Xaml_Navigation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct NavigatingCancelEventArgs(::windows::runtime::IInspectable);
impl NavigatingCancelEventArgs {
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn Cancel(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn SetCancel(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn NavigationMode(&self) -> ::windows::runtime::Result<NavigationMode> {
        let this = self;
        unsafe {
            let mut result__: NavigationMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NavigationMode>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Interop")]
    #[doc = "*Required features: `UI_Xaml_Navigation`, `UI_Xaml_Interop`*"]
    pub fn SourcePageType(&self) -> ::windows::runtime::Result<super::Interop::TypeName> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<super::Interop::TypeName> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Interop::TypeName>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn Parameter(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<INavigatingCancelEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    #[doc = "*Required features: `UI_Xaml_Navigation`, `UI_Xaml_Media_Animation`*"]
    pub fn NavigationTransitionInfo(&self) -> ::windows::runtime::Result<super::Media::Animation::NavigationTransitionInfo> {
        let this = &::windows::runtime::Interface::cast::<INavigatingCancelEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::Animation::NavigationTransitionInfo>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NavigatingCancelEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Navigation.NavigatingCancelEventArgs;{fd1d67ae-eafb-4079-be80-6dc92a03aedf})");
}
unsafe impl ::windows::runtime::Interface for NavigatingCancelEventArgs {
    type Vtable = INavigatingCancelEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4246562734, 60155, 16505, [190, 128, 109, 201, 42, 3, 174, 223]);
}
impl ::windows::runtime::RuntimeName for NavigatingCancelEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Navigation.NavigatingCancelEventArgs";
}
impl ::std::convert::From<NavigatingCancelEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: NavigatingCancelEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&NavigatingCancelEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &NavigatingCancelEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for NavigatingCancelEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &NavigatingCancelEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<NavigatingCancelEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: NavigatingCancelEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NavigatingCancelEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &NavigatingCancelEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for NavigatingCancelEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a NavigatingCancelEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for NavigatingCancelEventArgs {}
unsafe impl ::std::marker::Sync for NavigatingCancelEventArgs {}
#[doc = "*Required features: `UI_Xaml_Navigation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct NavigatingCancelEventHandler(::windows::runtime::IUnknown);
impl NavigatingCancelEventHandler {
    pub fn new<F: FnMut(&::std::option::Option<::windows::runtime::IInspectable>, &::std::option::Option<NavigatingCancelEventArgs>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = NavigatingCancelEventHandler_box::<F> {
            vtable: &NavigatingCancelEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, NavigatingCancelEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NavigatingCancelEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({75d6a78f-a302-4489-9898-24ea49182910})");
}
unsafe impl ::windows::runtime::Interface for NavigatingCancelEventHandler {
    type Vtable = NavigatingCancelEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1977001871, 41730, 17545, [152, 152, 36, 234, 73, 24, 41, 16]);
}
#[repr(C)]
#[doc(hidden)]
pub struct NavigatingCancelEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct NavigatingCancelEventHandler_box<F: FnMut(&::std::option::Option<::windows::runtime::IInspectable>, &::std::option::Option<NavigatingCancelEventArgs>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const NavigatingCancelEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::std::option::Option<::windows::runtime::IInspectable>, &::std::option::Option<NavigatingCancelEventArgs>) -> ::windows::runtime::Result<()> + 'static> NavigatingCancelEventHandler_box<F> {
    const VTABLE: NavigatingCancelEventHandler_abi = NavigatingCancelEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<NavigatingCancelEventHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <::windows::runtime::IInspectable as ::windows::runtime::Abi>::Abi as *const <::windows::runtime::IInspectable as ::windows::runtime::DefaultType>::DefaultType),
            &*(&e as *const <NavigatingCancelEventArgs as ::windows::runtime::Abi>::Abi as *const <NavigatingCancelEventArgs as ::windows::runtime::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[doc = "*Required features: `UI_Xaml_Navigation`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NavigationCacheMode(pub i32);
impl NavigationCacheMode {
    pub const Disabled: NavigationCacheMode = NavigationCacheMode(0i32);
    pub const Required: NavigationCacheMode = NavigationCacheMode(1i32);
    pub const Enabled: NavigationCacheMode = NavigationCacheMode(2i32);
}
impl ::std::convert::From<i32> for NavigationCacheMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NavigationCacheMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for NavigationCacheMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Navigation.NavigationCacheMode;i4)");
}
impl ::windows::runtime::DefaultType for NavigationCacheMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Navigation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct NavigationEventArgs(::windows::runtime::IInspectable);
impl NavigationEventArgs {
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn Content(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn Parameter(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Interop")]
    #[doc = "*Required features: `UI_Xaml_Navigation`, `UI_Xaml_Interop`*"]
    pub fn SourcePageType(&self) -> ::windows::runtime::Result<super::Interop::TypeName> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<super::Interop::TypeName> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Interop::TypeName>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn NavigationMode(&self) -> ::windows::runtime::Result<NavigationMode> {
        let this = self;
        unsafe {
            let mut result__: NavigationMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NavigationMode>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Navigation`, `Foundation`*"]
    pub fn Uri(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Navigation`, `Foundation`*"]
    pub fn SetUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    #[doc = "*Required features: `UI_Xaml_Navigation`, `UI_Xaml_Media_Animation`*"]
    pub fn NavigationTransitionInfo(&self) -> ::windows::runtime::Result<super::Media::Animation::NavigationTransitionInfo> {
        let this = &::windows::runtime::Interface::cast::<INavigationEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::Animation::NavigationTransitionInfo>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NavigationEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Navigation.NavigationEventArgs;{b6aa9834-6691-44d1-bdf7-58820c27b0d0})");
}
unsafe impl ::windows::runtime::Interface for NavigationEventArgs {
    type Vtable = INavigationEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3064633396, 26257, 17617, [189, 247, 88, 130, 12, 39, 176, 208]);
}
impl ::windows::runtime::RuntimeName for NavigationEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Navigation.NavigationEventArgs";
}
impl ::std::convert::From<NavigationEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: NavigationEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&NavigationEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &NavigationEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for NavigationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &NavigationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<NavigationEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: NavigationEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NavigationEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &NavigationEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for NavigationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a NavigationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for NavigationEventArgs {}
unsafe impl ::std::marker::Sync for NavigationEventArgs {}
#[doc = "*Required features: `UI_Xaml_Navigation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct NavigationFailedEventArgs(::windows::runtime::IInspectable);
impl NavigationFailedEventArgs {
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn Exception(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn Handled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Interop")]
    #[doc = "*Required features: `UI_Xaml_Navigation`, `UI_Xaml_Interop`*"]
    pub fn SourcePageType(&self) -> ::windows::runtime::Result<super::Interop::TypeName> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<super::Interop::TypeName> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Interop::TypeName>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NavigationFailedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Navigation.NavigationFailedEventArgs;{11c1dff7-36c2-4102-b2ef-0217a97289b3})");
}
unsafe impl ::windows::runtime::Interface for NavigationFailedEventArgs {
    type Vtable = INavigationFailedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(297918455, 14018, 16642, [178, 239, 2, 23, 169, 114, 137, 179]);
}
impl ::windows::runtime::RuntimeName for NavigationFailedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Navigation.NavigationFailedEventArgs";
}
impl ::std::convert::From<NavigationFailedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: NavigationFailedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&NavigationFailedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &NavigationFailedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for NavigationFailedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &NavigationFailedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<NavigationFailedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: NavigationFailedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NavigationFailedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &NavigationFailedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for NavigationFailedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a NavigationFailedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for NavigationFailedEventArgs {}
unsafe impl ::std::marker::Sync for NavigationFailedEventArgs {}
#[doc = "*Required features: `UI_Xaml_Navigation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct NavigationFailedEventHandler(::windows::runtime::IUnknown);
impl NavigationFailedEventHandler {
    pub fn new<F: FnMut(&::std::option::Option<::windows::runtime::IInspectable>, &::std::option::Option<NavigationFailedEventArgs>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = NavigationFailedEventHandler_box::<F> {
            vtable: &NavigationFailedEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, NavigationFailedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NavigationFailedEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({4dab4671-12b2-43c7-b892-9be2dcd3e88d})");
}
unsafe impl ::windows::runtime::Interface for NavigationFailedEventHandler {
    type Vtable = NavigationFailedEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1303070321, 4786, 17351, [184, 146, 155, 226, 220, 211, 232, 141]);
}
#[repr(C)]
#[doc(hidden)]
pub struct NavigationFailedEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct NavigationFailedEventHandler_box<F: FnMut(&::std::option::Option<::windows::runtime::IInspectable>, &::std::option::Option<NavigationFailedEventArgs>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const NavigationFailedEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::std::option::Option<::windows::runtime::IInspectable>, &::std::option::Option<NavigationFailedEventArgs>) -> ::windows::runtime::Result<()> + 'static> NavigationFailedEventHandler_box<F> {
    const VTABLE: NavigationFailedEventHandler_abi = NavigationFailedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<NavigationFailedEventHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <::windows::runtime::IInspectable as ::windows::runtime::Abi>::Abi as *const <::windows::runtime::IInspectable as ::windows::runtime::DefaultType>::DefaultType),
            &*(&e as *const <NavigationFailedEventArgs as ::windows::runtime::Abi>::Abi as *const <NavigationFailedEventArgs as ::windows::runtime::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[doc = "*Required features: `UI_Xaml_Navigation`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NavigationMode(pub i32);
impl NavigationMode {
    pub const New: NavigationMode = NavigationMode(0i32);
    pub const Back: NavigationMode = NavigationMode(1i32);
    pub const Forward: NavigationMode = NavigationMode(2i32);
    pub const Refresh: NavigationMode = NavigationMode(3i32);
}
impl ::std::convert::From<i32> for NavigationMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NavigationMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for NavigationMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Navigation.NavigationMode;i4)");
}
impl ::windows::runtime::DefaultType for NavigationMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Navigation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct NavigationStoppedEventHandler(::windows::runtime::IUnknown);
impl NavigationStoppedEventHandler {
    pub fn new<F: FnMut(&::std::option::Option<::windows::runtime::IInspectable>, &::std::option::Option<NavigationEventArgs>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = NavigationStoppedEventHandler_box::<F> {
            vtable: &NavigationStoppedEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, NavigationEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NavigationStoppedEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({f0117ddb-12fa-4d8d-8b26-b383d09c2b3c})");
}
unsafe impl ::windows::runtime::Interface for NavigationStoppedEventHandler {
    type Vtable = NavigationStoppedEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4027678171, 4858, 19853, [139, 38, 179, 131, 208, 156, 43, 60]);
}
#[repr(C)]
#[doc(hidden)]
pub struct NavigationStoppedEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct NavigationStoppedEventHandler_box<F: FnMut(&::std::option::Option<::windows::runtime::IInspectable>, &::std::option::Option<NavigationEventArgs>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const NavigationStoppedEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::std::option::Option<::windows::runtime::IInspectable>, &::std::option::Option<NavigationEventArgs>) -> ::windows::runtime::Result<()> + 'static> NavigationStoppedEventHandler_box<F> {
    const VTABLE: NavigationStoppedEventHandler_abi = NavigationStoppedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<NavigationStoppedEventHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <::windows::runtime::IInspectable as ::windows::runtime::Abi>::Abi as *const <::windows::runtime::IInspectable as ::windows::runtime::DefaultType>::DefaultType),
            &*(&e as *const <NavigationEventArgs as ::windows::runtime::Abi>::Abi as *const <NavigationEventArgs as ::windows::runtime::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[doc = "*Required features: `UI_Xaml_Navigation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PageStackEntry(::windows::runtime::IInspectable);
impl PageStackEntry {
    #[cfg(feature = "UI_Xaml_Interop")]
    #[doc = "*Required features: `UI_Xaml_Navigation`, `UI_Xaml_Interop`*"]
    pub fn SourcePageType(&self) -> ::windows::runtime::Result<super::Interop::TypeName> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<super::Interop::TypeName> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Interop::TypeName>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn Parameter(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    #[doc = "*Required features: `UI_Xaml_Navigation`, `UI_Xaml_Media_Animation`*"]
    pub fn NavigationTransitionInfo(&self) -> ::windows::runtime::Result<super::Media::Animation::NavigationTransitionInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::Animation::NavigationTransitionInfo>(result__)
        }
    }
    #[cfg(all(feature = "UI_Xaml_Interop", feature = "UI_Xaml_Media_Animation"))]
    #[doc = "*Required features: `UI_Xaml_Navigation`, `UI_Xaml_Interop`, `UI_Xaml_Media_Animation`*"]
    pub fn CreateInstance<'a, Param0: ::windows::runtime::IntoParam<'a, super::Interop::TypeName>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param2: ::windows::runtime::IntoParam<'a, super::Media::Animation::NavigationTransitionInfo>>(sourcepagetype: Param0, parameter: Param1, navigationtransitioninfo: Param2) -> ::windows::runtime::Result<PageStackEntry> {
        Self::IPageStackEntryFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), sourcepagetype.into_param().abi(), parameter.into_param().abi(), navigationtransitioninfo.into_param().abi(), &mut result__).from_abi::<PageStackEntry>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn SourcePageTypeProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IPageStackEntryStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, dp: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), dp.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), dp.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn GetAnimationBaseValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(&self, dp: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), dp.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Xaml_Navigation`, `UI_Core`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn RegisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>, Param1: ::windows::runtime::IntoParam<'a, super::DependencyPropertyChangedCallback>>(&self, dp: Param0, callback: Param1) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject2>(self)?;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), dp.into_param().abi(), callback.into_param().abi(), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn UnregisterPropertyChangedCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(&self, dp: Param0, token: i64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), dp.into_param().abi(), token).ok() }
    }
    pub fn IPageStackEntryFactory<R, F: FnOnce(&IPageStackEntryFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PageStackEntry, IPageStackEntryFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPageStackEntryStatics<R, F: FnOnce(&IPageStackEntryStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PageStackEntry, IPageStackEntryStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PageStackEntry {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Navigation.PageStackEntry;{ef8814a6-9388-4aca-8572-405194069080})");
}
unsafe impl ::windows::runtime::Interface for PageStackEntry {
    type Vtable = IPageStackEntry_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4018672806, 37768, 19146, [133, 114, 64, 81, 148, 6, 144, 128]);
}
impl ::windows::runtime::RuntimeName for PageStackEntry {
    const NAME: &'static str = "Windows.UI.Xaml.Navigation.PageStackEntry";
}
impl ::std::convert::From<PageStackEntry> for ::windows::runtime::IUnknown {
    fn from(value: PageStackEntry) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PageStackEntry> for ::windows::runtime::IUnknown {
    fn from(value: &PageStackEntry) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PageStackEntry {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PageStackEntry {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PageStackEntry> for ::windows::runtime::IInspectable {
    fn from(value: PageStackEntry) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PageStackEntry> for ::windows::runtime::IInspectable {
    fn from(value: &PageStackEntry) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PageStackEntry {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PageStackEntry {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<PageStackEntry> for super::DependencyObject {
    fn from(value: PageStackEntry) -> Self {
        ::std::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::std::convert::From<&PageStackEntry> for super::DependencyObject {
    fn from(value: &PageStackEntry) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for PageStackEntry {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for &PageStackEntry {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::DependencyObject>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for PageStackEntry {}
unsafe impl ::std::marker::Sync for PageStackEntry {}
