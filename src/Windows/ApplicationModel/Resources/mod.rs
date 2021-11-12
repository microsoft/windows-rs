#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "ApplicationModel_Resources_Core")]
pub mod Core;
#[cfg(feature = "ApplicationModel_Resources_Management")]
pub mod Management;
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceLoader(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IResourceLoader {
    type Vtable = IResourceLoader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08524908_16ef_45ad_a602_293637d7e61a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoader_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceLoader2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IResourceLoader2 {
    type Vtable = IResourceLoader2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10eb6ec6_8138_48c1_bc65_e1f14207367c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoader2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceLoaderFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IResourceLoaderFactory {
    type Vtable = IResourceLoaderFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc33a3603_69dc_4285_a077_d5c0e47ccbe8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoaderFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceLoaderStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IResourceLoaderStatics {
    type Vtable = IResourceLoaderStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf777ce1_19c8_49c2_953c_47e9227b334e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoaderStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceLoaderStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IResourceLoaderStatics2 {
    type Vtable = IResourceLoaderStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cc04141_6466_4989_9494_0b82dfc53f1f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoaderStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceLoaderStatics3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IResourceLoaderStatics3 {
    type Vtable = IResourceLoaderStatics3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64609dfb_64ac_491b_8100_0e558d61c1d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoaderStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, context: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceLoaderStatics4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IResourceLoaderStatics4 {
    type Vtable = IResourceLoaderStatics4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9fb36c32_6c8c_4316_962e_909539b5c259);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoaderStatics4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ResourceLoader(pub ::windows::core::IInspectable);
impl ResourceLoader {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ResourceLoader, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn GetString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, resource: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), resource.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetStringForUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, uri: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IResourceLoader2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn CreateResourceLoaderByName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(name: Param0) -> ::windows::core::Result<ResourceLoader> {
        Self::IResourceLoaderFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<ResourceLoader>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn GetStringForReference<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(uri: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IResourceLoaderStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GetForCurrentView() -> ::windows::core::Result<ResourceLoader> {
        Self::IResourceLoaderStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ResourceLoader>(result__)
        })
    }
    pub fn GetForCurrentViewWithName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(name: Param0) -> ::windows::core::Result<ResourceLoader> {
        Self::IResourceLoaderStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<ResourceLoader>(result__)
        })
    }
    pub fn GetForViewIndependentUse() -> ::windows::core::Result<ResourceLoader> {
        Self::IResourceLoaderStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ResourceLoader>(result__)
        })
    }
    pub fn GetForViewIndependentUseWithName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(name: Param0) -> ::windows::core::Result<ResourceLoader> {
        Self::IResourceLoaderStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<ResourceLoader>(result__)
        })
    }
    #[cfg(feature = "UI")]
    pub fn GetForUIContext<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::UIContext>>(context: Param0) -> ::windows::core::Result<ResourceLoader> {
        Self::IResourceLoaderStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), context.into_param().abi(), &mut result__).from_abi::<ResourceLoader>(result__)
        })
    }
    pub fn GetDefaultPriPath<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(packagefullname: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IResourceLoaderStatics4(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), packagefullname.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn IResourceLoaderFactory<R, F: FnOnce(&IResourceLoaderFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ResourceLoader, IResourceLoaderFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IResourceLoaderStatics<R, F: FnOnce(&IResourceLoaderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ResourceLoader, IResourceLoaderStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IResourceLoaderStatics2<R, F: FnOnce(&IResourceLoaderStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ResourceLoader, IResourceLoaderStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IResourceLoaderStatics3<R, F: FnOnce(&IResourceLoaderStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ResourceLoader, IResourceLoaderStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IResourceLoaderStatics4<R, F: FnOnce(&IResourceLoaderStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ResourceLoader, IResourceLoaderStatics4> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ResourceLoader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.ResourceLoader;{08524908-16ef-45ad-a602-293637d7e61a})");
}
unsafe impl ::windows::core::Interface for ResourceLoader {
    type Vtable = IResourceLoader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08524908_16ef_45ad_a602_293637d7e61a);
}
impl ::windows::core::RuntimeName for ResourceLoader {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.ResourceLoader";
}
impl ::core::convert::From<ResourceLoader> for ::windows::core::IUnknown {
    fn from(value: ResourceLoader) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ResourceLoader> for ::windows::core::IUnknown {
    fn from(value: &ResourceLoader) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ResourceLoader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ResourceLoader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ResourceLoader> for ::windows::core::IInspectable {
    fn from(value: ResourceLoader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ResourceLoader> for ::windows::core::IInspectable {
    fn from(value: &ResourceLoader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ResourceLoader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ResourceLoader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ResourceLoader {}
unsafe impl ::core::marker::Sync for ResourceLoader {}
