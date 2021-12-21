#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'UI_Xaml_Navigation'*"]
#[repr(transparent)]
pub struct FrameNavigationOptions(::windows::core::IUnknown);
impl FrameNavigationOptions {
    #[doc = "*Required features: 'UI_Xaml_Navigation'*"]
    pub fn IsNavigationStackEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Navigation'*"]
    pub fn SetIsNavigationStackEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Navigation', 'UI_Xaml_Media_Animation'*"]
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    pub fn TransitionInfoOverride(&self) -> ::windows::core::Result<super::Media::Animation::NavigationTransitionInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::Animation::NavigationTransitionInfo>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Navigation', 'UI_Xaml_Media_Animation'*"]
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    pub fn SetTransitionInfoOverride<'a, Param0: ::windows::core::IntoParam<'a, super::Media::Animation::NavigationTransitionInfo>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Navigation'*"]
    pub fn new() -> ::windows::core::Result<FrameNavigationOptions> {
        Self::IFrameNavigationOptionsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<FrameNavigationOptions>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IFrameNavigationOptionsFactory<R, F: FnOnce(&IFrameNavigationOptionsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FrameNavigationOptions, IFrameNavigationOptionsFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for FrameNavigationOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameNavigationOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameNavigationOptions {}
impl ::core::fmt::Debug for FrameNavigationOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FrameNavigationOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FrameNavigationOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Navigation.FrameNavigationOptions;{b539ad2a-9fb7-520a-8f41-57a50c59cf92})");
}
unsafe impl ::windows::core::Interface for FrameNavigationOptions {
    type Vtable = IFrameNavigationOptionsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb539ad2a_9fb7_520a_8f41_57a50c59cf92);
}
impl ::windows::core::RuntimeName for FrameNavigationOptions {
    const NAME: &'static str = "Windows.UI.Xaml.Navigation.FrameNavigationOptions";
}
impl ::core::convert::From<FrameNavigationOptions> for ::windows::core::IUnknown {
    fn from(value: FrameNavigationOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameNavigationOptions> for ::windows::core::IUnknown {
    fn from(value: &FrameNavigationOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FrameNavigationOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &FrameNavigationOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FrameNavigationOptions> for ::windows::core::IInspectable {
    fn from(value: FrameNavigationOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameNavigationOptions> for ::windows::core::IInspectable {
    fn from(value: &FrameNavigationOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FrameNavigationOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &FrameNavigationOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for FrameNavigationOptions {}
unsafe impl ::core::marker::Sync for FrameNavigationOptions {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameNavigationOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFrameNavigationOptions {
    type Vtable = IFrameNavigationOptionsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb539ad2a_9fb7_520a_8f41_57a50c59cf92);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameNavigationOptionsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media_Animation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Animation"))] usize,
    #[cfg(feature = "UI_Xaml_Media_Animation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Animation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameNavigationOptionsFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFrameNavigationOptionsFactory {
    type Vtable = IFrameNavigationOptionsFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4681e41_7e6d_5c7c_aca0_478681cc6fce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameNavigationOptionsFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct INavigatingCancelEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INavigatingCancelEventArgs {
    type Vtable = INavigatingCancelEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd1d67ae_eafb_4079_be80_6dc92a03aedf);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigatingCancelEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NavigationMode) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Interop")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct INavigatingCancelEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INavigatingCancelEventArgs2 {
    type Vtable = INavigatingCancelEventArgs2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5407b704_8147_4343_838f_dd1ee908c137);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigatingCancelEventArgs2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media_Animation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Animation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct INavigationEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INavigationEventArgs {
    type Vtable = INavigationEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6aa9834_6691_44d1_bdf7_58820c27b0d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Interop")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NavigationMode) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct INavigationEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INavigationEventArgs2 {
    type Vtable = INavigationEventArgs2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdbff71d9_979a_4b2e_a49b_3bb17fdef574);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationEventArgs2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media_Animation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Animation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct INavigationFailedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INavigationFailedEventArgs {
    type Vtable = INavigationFailedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11c1dff7_36c2_4102_b2ef_0217a97289b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationFailedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Interop")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPageStackEntry(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPageStackEntry {
    type Vtable = IPageStackEntryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef8814a6_9388_4aca_8572_405194069080);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPageStackEntryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Interop")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media_Animation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Animation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPageStackEntryFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPageStackEntryFactory {
    type Vtable = IPageStackEntryFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4454048a_a8b9_4f78_9b84_1f51f58851ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPageStackEntryFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "UI_Xaml_Interop", feature = "UI_Xaml_Media_Animation"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcepagetype: ::core::mem::ManuallyDrop<super::Interop::TypeName>, parameter: *mut ::core::ffi::c_void, navigationtransitioninfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "UI_Xaml_Interop", feature = "UI_Xaml_Media_Animation")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPageStackEntryStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPageStackEntryStatics {
    type Vtable = IPageStackEntryStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaceff8e3_246c_4033_9f01_01cb0da5254e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPageStackEntryStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'UI_Xaml_Navigation'*"]
#[repr(transparent)]
pub struct LoadCompletedEventHandler(pub ::windows::core::IUnknown);
impl LoadCompletedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<NavigationEventArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = LoadCompletedEventHandlerBox::<F> { vtable: &LoadCompletedEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: 'UI_Xaml_Navigation'*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, NavigationEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct LoadCompletedEventHandlerBox<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<NavigationEventArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const LoadCompletedEventHandlerVtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<NavigationEventArgs>) -> ::windows::core::Result<()> + 'static> LoadCompletedEventHandlerBox<F> {
    const VTABLE: LoadCompletedEventHandlerVtbl = LoadCompletedEventHandlerVtbl(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&sender as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&e as *const <NavigationEventArgs as ::windows::core::Abi>::Abi as *const <NavigationEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
    }
}
impl ::core::clone::Clone for LoadCompletedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LoadCompletedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LoadCompletedEventHandler {}
impl ::core::fmt::Debug for LoadCompletedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LoadCompletedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for LoadCompletedEventHandler {
    type Vtable = LoadCompletedEventHandlerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaebaf785_43fc_4e2c_95c3_97ae84eabc8e);
}
unsafe impl ::windows::core::RuntimeType for LoadCompletedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{aebaf785-43fc-4e2c-95c3-97ae84eabc8e}");
}
#[repr(C)]
#[doc(hidden)]
pub struct LoadCompletedEventHandlerVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'UI_Xaml_Navigation'*"]
#[repr(transparent)]
pub struct NavigatedEventHandler(pub ::windows::core::IUnknown);
impl NavigatedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<NavigationEventArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = NavigatedEventHandlerBox::<F> { vtable: &NavigatedEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: 'UI_Xaml_Navigation'*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, NavigationEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct NavigatedEventHandlerBox<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<NavigationEventArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const NavigatedEventHandlerVtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<NavigationEventArgs>) -> ::windows::core::Result<()> + 'static> NavigatedEventHandlerBox<F> {
    const VTABLE: NavigatedEventHandlerVtbl = NavigatedEventHandlerVtbl(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&sender as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&e as *const <NavigationEventArgs as ::windows::core::Abi>::Abi as *const <NavigationEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
    }
}
impl ::core::clone::Clone for NavigatedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NavigatedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NavigatedEventHandler {}
impl ::core::fmt::Debug for NavigatedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NavigatedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for NavigatedEventHandler {
    type Vtable = NavigatedEventHandlerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7bd1cf54_23cf_4cce_b2f5_4ce78d96896e);
}
unsafe impl ::windows::core::RuntimeType for NavigatedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{7bd1cf54-23cf-4cce-b2f5-4ce78d96896e}");
}
#[repr(C)]
#[doc(hidden)]
pub struct NavigatedEventHandlerVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'UI_Xaml_Navigation'*"]
#[repr(transparent)]
pub struct NavigatingCancelEventArgs(::windows::core::IUnknown);
impl NavigatingCancelEventArgs {
    #[doc = "*Required features: 'UI_Xaml_Navigation'*"]
    pub fn Cancel(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Navigation'*"]
    pub fn SetCancel(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Navigation'*"]
    pub fn NavigationMode(&self) -> ::windows::core::Result<NavigationMode> {
        let this = self;
        unsafe {
            let mut result__: NavigationMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NavigationMode>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Navigation', 'UI_Xaml_Interop'*"]
    #[cfg(feature = "UI_Xaml_Interop")]
    pub fn SourcePageType(&self) -> ::windows::core::Result<super::Interop::TypeName> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<super::Interop::TypeName> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Interop::TypeName>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Navigation'*"]
    pub fn Parameter(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<INavigatingCancelEventArgs2>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Navigation', 'UI_Xaml_Media_Animation'*"]
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    pub fn NavigationTransitionInfo(&self) -> ::windows::core::Result<super::Media::Animation::NavigationTransitionInfo> {
        let this = &::windows::core::Interface::cast::<INavigatingCancelEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::Animation::NavigationTransitionInfo>(result__)
        }
    }
}
impl ::core::clone::Clone for NavigatingCancelEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NavigatingCancelEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NavigatingCancelEventArgs {}
impl ::core::fmt::Debug for NavigatingCancelEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NavigatingCancelEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for NavigatingCancelEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Navigation.NavigatingCancelEventArgs;{fd1d67ae-eafb-4079-be80-6dc92a03aedf})");
}
unsafe impl ::windows::core::Interface for NavigatingCancelEventArgs {
    type Vtable = INavigatingCancelEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd1d67ae_eafb_4079_be80_6dc92a03aedf);
}
impl ::windows::core::RuntimeName for NavigatingCancelEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Navigation.NavigatingCancelEventArgs";
}
impl ::core::convert::From<NavigatingCancelEventArgs> for ::windows::core::IUnknown {
    fn from(value: NavigatingCancelEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NavigatingCancelEventArgs> for ::windows::core::IUnknown {
    fn from(value: &NavigatingCancelEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NavigatingCancelEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &NavigatingCancelEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NavigatingCancelEventArgs> for ::windows::core::IInspectable {
    fn from(value: NavigatingCancelEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NavigatingCancelEventArgs> for ::windows::core::IInspectable {
    fn from(value: &NavigatingCancelEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NavigatingCancelEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &NavigatingCancelEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for NavigatingCancelEventArgs {}
unsafe impl ::core::marker::Sync for NavigatingCancelEventArgs {}
#[doc = "*Required features: 'UI_Xaml_Navigation'*"]
#[repr(transparent)]
pub struct NavigatingCancelEventHandler(pub ::windows::core::IUnknown);
impl NavigatingCancelEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<NavigatingCancelEventArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = NavigatingCancelEventHandlerBox::<F> { vtable: &NavigatingCancelEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: 'UI_Xaml_Navigation'*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, NavigatingCancelEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct NavigatingCancelEventHandlerBox<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<NavigatingCancelEventArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const NavigatingCancelEventHandlerVtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<NavigatingCancelEventArgs>) -> ::windows::core::Result<()> + 'static> NavigatingCancelEventHandlerBox<F> {
    const VTABLE: NavigatingCancelEventHandlerVtbl = NavigatingCancelEventHandlerVtbl(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&sender as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&e as *const <NavigatingCancelEventArgs as ::windows::core::Abi>::Abi as *const <NavigatingCancelEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
    }
}
impl ::core::clone::Clone for NavigatingCancelEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NavigatingCancelEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NavigatingCancelEventHandler {}
impl ::core::fmt::Debug for NavigatingCancelEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NavigatingCancelEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for NavigatingCancelEventHandler {
    type Vtable = NavigatingCancelEventHandlerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75d6a78f_a302_4489_9898_24ea49182910);
}
unsafe impl ::windows::core::RuntimeType for NavigatingCancelEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{75d6a78f-a302-4489-9898-24ea49182910}");
}
#[repr(C)]
#[doc(hidden)]
pub struct NavigatingCancelEventHandlerVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'UI_Xaml_Navigation'*"]
#[repr(transparent)]
pub struct NavigationCacheMode(pub i32);
impl NavigationCacheMode {
    pub const Disabled: Self = Self(0i32);
    pub const Required: Self = Self(1i32);
    pub const Enabled: Self = Self(2i32);
}
impl ::core::marker::Copy for NavigationCacheMode {}
impl ::core::clone::Clone for NavigationCacheMode {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NavigationCacheMode {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NavigationCacheMode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NavigationCacheMode {}
impl ::core::fmt::Debug for NavigationCacheMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NavigationCacheMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for NavigationCacheMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Navigation.NavigationCacheMode;i4)");
}
impl ::windows::core::DefaultType for NavigationCacheMode {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Xaml_Navigation'*"]
#[repr(transparent)]
pub struct NavigationEventArgs(::windows::core::IUnknown);
impl NavigationEventArgs {
    #[doc = "*Required features: 'UI_Xaml_Navigation'*"]
    pub fn Content(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Navigation'*"]
    pub fn Parameter(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Navigation', 'UI_Xaml_Interop'*"]
    #[cfg(feature = "UI_Xaml_Interop")]
    pub fn SourcePageType(&self) -> ::windows::core::Result<super::Interop::TypeName> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<super::Interop::TypeName> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Interop::TypeName>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Navigation'*"]
    pub fn NavigationMode(&self) -> ::windows::core::Result<NavigationMode> {
        let this = self;
        unsafe {
            let mut result__: NavigationMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NavigationMode>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Navigation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Navigation', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Navigation', 'UI_Xaml_Media_Animation'*"]
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    pub fn NavigationTransitionInfo(&self) -> ::windows::core::Result<super::Media::Animation::NavigationTransitionInfo> {
        let this = &::windows::core::Interface::cast::<INavigationEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::Animation::NavigationTransitionInfo>(result__)
        }
    }
}
impl ::core::clone::Clone for NavigationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NavigationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NavigationEventArgs {}
impl ::core::fmt::Debug for NavigationEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NavigationEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for NavigationEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Navigation.NavigationEventArgs;{b6aa9834-6691-44d1-bdf7-58820c27b0d0})");
}
unsafe impl ::windows::core::Interface for NavigationEventArgs {
    type Vtable = INavigationEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6aa9834_6691_44d1_bdf7_58820c27b0d0);
}
impl ::windows::core::RuntimeName for NavigationEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Navigation.NavigationEventArgs";
}
impl ::core::convert::From<NavigationEventArgs> for ::windows::core::IUnknown {
    fn from(value: NavigationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NavigationEventArgs> for ::windows::core::IUnknown {
    fn from(value: &NavigationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NavigationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &NavigationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NavigationEventArgs> for ::windows::core::IInspectable {
    fn from(value: NavigationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NavigationEventArgs> for ::windows::core::IInspectable {
    fn from(value: &NavigationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NavigationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &NavigationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for NavigationEventArgs {}
unsafe impl ::core::marker::Sync for NavigationEventArgs {}
#[doc = "*Required features: 'UI_Xaml_Navigation'*"]
#[repr(transparent)]
pub struct NavigationFailedEventArgs(::windows::core::IUnknown);
impl NavigationFailedEventArgs {
    #[doc = "*Required features: 'UI_Xaml_Navigation'*"]
    pub fn Exception(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Navigation'*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Navigation'*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Xaml_Navigation', 'UI_Xaml_Interop'*"]
    #[cfg(feature = "UI_Xaml_Interop")]
    pub fn SourcePageType(&self) -> ::windows::core::Result<super::Interop::TypeName> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<super::Interop::TypeName> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Interop::TypeName>(result__)
        }
    }
}
impl ::core::clone::Clone for NavigationFailedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NavigationFailedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NavigationFailedEventArgs {}
impl ::core::fmt::Debug for NavigationFailedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NavigationFailedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for NavigationFailedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Navigation.NavigationFailedEventArgs;{11c1dff7-36c2-4102-b2ef-0217a97289b3})");
}
unsafe impl ::windows::core::Interface for NavigationFailedEventArgs {
    type Vtable = INavigationFailedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11c1dff7_36c2_4102_b2ef_0217a97289b3);
}
impl ::windows::core::RuntimeName for NavigationFailedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Navigation.NavigationFailedEventArgs";
}
impl ::core::convert::From<NavigationFailedEventArgs> for ::windows::core::IUnknown {
    fn from(value: NavigationFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NavigationFailedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &NavigationFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NavigationFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &NavigationFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NavigationFailedEventArgs> for ::windows::core::IInspectable {
    fn from(value: NavigationFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NavigationFailedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &NavigationFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NavigationFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &NavigationFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for NavigationFailedEventArgs {}
unsafe impl ::core::marker::Sync for NavigationFailedEventArgs {}
#[doc = "*Required features: 'UI_Xaml_Navigation'*"]
#[repr(transparent)]
pub struct NavigationFailedEventHandler(pub ::windows::core::IUnknown);
impl NavigationFailedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<NavigationFailedEventArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = NavigationFailedEventHandlerBox::<F> { vtable: &NavigationFailedEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: 'UI_Xaml_Navigation'*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, NavigationFailedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct NavigationFailedEventHandlerBox<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<NavigationFailedEventArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const NavigationFailedEventHandlerVtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<NavigationFailedEventArgs>) -> ::windows::core::Result<()> + 'static> NavigationFailedEventHandlerBox<F> {
    const VTABLE: NavigationFailedEventHandlerVtbl = NavigationFailedEventHandlerVtbl(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&sender as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&e as *const <NavigationFailedEventArgs as ::windows::core::Abi>::Abi as *const <NavigationFailedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
    }
}
impl ::core::clone::Clone for NavigationFailedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NavigationFailedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NavigationFailedEventHandler {}
impl ::core::fmt::Debug for NavigationFailedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NavigationFailedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for NavigationFailedEventHandler {
    type Vtable = NavigationFailedEventHandlerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4dab4671_12b2_43c7_b892_9be2dcd3e88d);
}
unsafe impl ::windows::core::RuntimeType for NavigationFailedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{4dab4671-12b2-43c7-b892-9be2dcd3e88d}");
}
#[repr(C)]
#[doc(hidden)]
pub struct NavigationFailedEventHandlerVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'UI_Xaml_Navigation'*"]
#[repr(transparent)]
pub struct NavigationMode(pub i32);
impl NavigationMode {
    pub const New: Self = Self(0i32);
    pub const Back: Self = Self(1i32);
    pub const Forward: Self = Self(2i32);
    pub const Refresh: Self = Self(3i32);
}
impl ::core::marker::Copy for NavigationMode {}
impl ::core::clone::Clone for NavigationMode {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NavigationMode {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NavigationMode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NavigationMode {}
impl ::core::fmt::Debug for NavigationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NavigationMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for NavigationMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Navigation.NavigationMode;i4)");
}
impl ::windows::core::DefaultType for NavigationMode {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Xaml_Navigation'*"]
#[repr(transparent)]
pub struct NavigationStoppedEventHandler(pub ::windows::core::IUnknown);
impl NavigationStoppedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<NavigationEventArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = NavigationStoppedEventHandlerBox::<F> { vtable: &NavigationStoppedEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: 'UI_Xaml_Navigation'*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, NavigationEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct NavigationStoppedEventHandlerBox<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<NavigationEventArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const NavigationStoppedEventHandlerVtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<NavigationEventArgs>) -> ::windows::core::Result<()> + 'static> NavigationStoppedEventHandlerBox<F> {
    const VTABLE: NavigationStoppedEventHandlerVtbl = NavigationStoppedEventHandlerVtbl(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&sender as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&e as *const <NavigationEventArgs as ::windows::core::Abi>::Abi as *const <NavigationEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
    }
}
impl ::core::clone::Clone for NavigationStoppedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NavigationStoppedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NavigationStoppedEventHandler {}
impl ::core::fmt::Debug for NavigationStoppedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NavigationStoppedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for NavigationStoppedEventHandler {
    type Vtable = NavigationStoppedEventHandlerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0117ddb_12fa_4d8d_8b26_b383d09c2b3c);
}
unsafe impl ::windows::core::RuntimeType for NavigationStoppedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{f0117ddb-12fa-4d8d-8b26-b383d09c2b3c}");
}
#[repr(C)]
#[doc(hidden)]
pub struct NavigationStoppedEventHandlerVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'UI_Xaml_Navigation'*"]
#[repr(transparent)]
pub struct PageStackEntry(::windows::core::IUnknown);
impl PageStackEntry {
    #[doc = "*Required features: 'UI_Xaml_Navigation', 'UI_Xaml_Interop'*"]
    #[cfg(feature = "UI_Xaml_Interop")]
    pub fn SourcePageType(&self) -> ::windows::core::Result<super::Interop::TypeName> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<super::Interop::TypeName> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Interop::TypeName>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Navigation'*"]
    pub fn Parameter(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Navigation', 'UI_Xaml_Media_Animation'*"]
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    pub fn NavigationTransitionInfo(&self) -> ::windows::core::Result<super::Media::Animation::NavigationTransitionInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Media::Animation::NavigationTransitionInfo>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Xaml_Navigation', 'UI_Xaml_Interop', 'UI_Xaml_Media_Animation'*"]
    #[cfg(all(feature = "UI_Xaml_Interop", feature = "UI_Xaml_Media_Animation"))]
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, super::Interop::TypeName>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param2: ::windows::core::IntoParam<'a, super::Media::Animation::NavigationTransitionInfo>>(sourcepagetype: Param0, parameter: Param1, navigationtransitioninfo: Param2) -> ::windows::core::Result<PageStackEntry> {
        Self::IPageStackEntryFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), sourcepagetype.into_param().abi(), parameter.into_param().abi(), navigationtransitioninfo.into_param().abi(), &mut result__).from_abi::<PageStackEntry>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Xaml_Navigation'*"]
    pub fn SourcePageTypeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IPageStackEntryStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPageStackEntryFactory<R, F: FnOnce(&IPageStackEntryFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PageStackEntry, IPageStackEntryFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IPageStackEntryStatics<R, F: FnOnce(&IPageStackEntryStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PageStackEntry, IPageStackEntryStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PageStackEntry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PageStackEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PageStackEntry {}
impl ::core::fmt::Debug for PageStackEntry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PageStackEntry").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PageStackEntry {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Navigation.PageStackEntry;{ef8814a6-9388-4aca-8572-405194069080})");
}
unsafe impl ::windows::core::Interface for PageStackEntry {
    type Vtable = IPageStackEntryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef8814a6_9388_4aca_8572_405194069080);
}
impl ::windows::core::RuntimeName for PageStackEntry {
    const NAME: &'static str = "Windows.UI.Xaml.Navigation.PageStackEntry";
}
impl ::core::convert::From<PageStackEntry> for ::windows::core::IUnknown {
    fn from(value: PageStackEntry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PageStackEntry> for ::windows::core::IUnknown {
    fn from(value: &PageStackEntry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PageStackEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PageStackEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PageStackEntry> for ::windows::core::IInspectable {
    fn from(value: PageStackEntry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PageStackEntry> for ::windows::core::IInspectable {
    fn from(value: &PageStackEntry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PageStackEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PageStackEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PageStackEntry> for super::DependencyObject {
    fn from(value: PageStackEntry) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PageStackEntry> for super::DependencyObject {
    fn from(value: &PageStackEntry) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for PageStackEntry {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &PageStackEntry {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for PageStackEntry {}
unsafe impl ::core::marker::Sync for PageStackEntry {}
