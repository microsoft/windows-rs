#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "ApplicationModel_Resources_Core")]
pub mod Core;
#[cfg(feature = "ApplicationModel_Resources_Management")]
pub mod Management;
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IResourceLoader(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IResourceLoader {
    type Vtable = IResourceLoader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(139610376, 5871, 17837, [166, 2, 41, 54, 55, 215, 230, 26]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resource: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IResourceLoader2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IResourceLoader2 {
    type Vtable = IResourceLoader2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(283864774, 33080, 18625, [188, 101, 225, 241, 66, 7, 54, 124]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoader2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IResourceLoaderFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IResourceLoaderFactory {
    type Vtable = IResourceLoaderFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3275372035, 27100, 17029, [160, 119, 213, 192, 228, 124, 203, 232]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoaderFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IResourceLoaderStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IResourceLoaderStatics {
    type Vtable = IResourceLoaderStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3212279009, 6600, 18882, [149, 60, 71, 233, 34, 123, 51, 78]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoaderStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IResourceLoaderStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IResourceLoaderStatics2 {
    type Vtable = IResourceLoaderStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(213926209, 25702, 18825, [148, 148, 11, 130, 223, 197, 63, 31]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoaderStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IResourceLoaderStatics3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IResourceLoaderStatics3 {
    type Vtable = IResourceLoaderStatics3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1684053499, 25772, 18715, [129, 0, 14, 85, 141, 97, 193, 208]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoaderStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, context: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IResourceLoaderStatics4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IResourceLoaderStatics4 {
    type Vtable = IResourceLoaderStatics4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2679335986, 27788, 17174, [150, 46, 144, 149, 57, 181, 194, 89]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoaderStatics4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefullname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `ApplicationModel_Resources`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ResourceLoader(::windows::runtime::IInspectable);
impl ResourceLoader {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ResourceLoader, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_Resources`*"]
    pub fn GetString<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, resource: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), resource.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Resources`, `Foundation`*"]
    pub fn GetStringForUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, uri: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IResourceLoader2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Resources`*"]
    pub fn CreateResourceLoaderByName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(name: Param0) -> ::windows::runtime::Result<ResourceLoader> {
        Self::IResourceLoaderFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<ResourceLoader>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Resources`, `Foundation`*"]
    pub fn GetStringForReference<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(uri: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IResourceLoaderStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Resources`*"]
    pub fn GetForCurrentView() -> ::windows::runtime::Result<ResourceLoader> {
        Self::IResourceLoaderStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ResourceLoader>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Resources`*"]
    pub fn GetForCurrentViewWithName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(name: Param0) -> ::windows::runtime::Result<ResourceLoader> {
        Self::IResourceLoaderStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<ResourceLoader>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Resources`*"]
    pub fn GetForViewIndependentUse() -> ::windows::runtime::Result<ResourceLoader> {
        Self::IResourceLoaderStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ResourceLoader>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Resources`*"]
    pub fn GetForViewIndependentUseWithName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(name: Param0) -> ::windows::runtime::Result<ResourceLoader> {
        Self::IResourceLoaderStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<ResourceLoader>(result__)
        })
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `ApplicationModel_Resources`, `UI`*"]
    pub fn GetForUIContext<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::UIContext>>(context: Param0) -> ::windows::runtime::Result<ResourceLoader> {
        Self::IResourceLoaderStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), context.into_param().abi(), &mut result__).from_abi::<ResourceLoader>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Resources`*"]
    pub fn GetDefaultPriPath<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(packagefullname: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IResourceLoaderStatics4(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), packagefullname.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IResourceLoaderFactory<R, F: FnOnce(&IResourceLoaderFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ResourceLoader, IResourceLoaderFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IResourceLoaderStatics<R, F: FnOnce(&IResourceLoaderStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ResourceLoader, IResourceLoaderStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IResourceLoaderStatics2<R, F: FnOnce(&IResourceLoaderStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ResourceLoader, IResourceLoaderStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IResourceLoaderStatics3<R, F: FnOnce(&IResourceLoaderStatics3) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ResourceLoader, IResourceLoaderStatics3> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IResourceLoaderStatics4<R, F: FnOnce(&IResourceLoaderStatics4) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ResourceLoader, IResourceLoaderStatics4> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ResourceLoader {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.ResourceLoader;{08524908-16ef-45ad-a602-293637d7e61a})");
}
unsafe impl ::windows::runtime::Interface for ResourceLoader {
    type Vtable = IResourceLoader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(139610376, 5871, 17837, [166, 2, 41, 54, 55, 215, 230, 26]);
}
impl ::windows::runtime::RuntimeName for ResourceLoader {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.ResourceLoader";
}
impl ::std::convert::From<ResourceLoader> for ::windows::runtime::IUnknown {
    fn from(value: ResourceLoader) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ResourceLoader> for ::windows::runtime::IUnknown {
    fn from(value: &ResourceLoader) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ResourceLoader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ResourceLoader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ResourceLoader> for ::windows::runtime::IInspectable {
    fn from(value: ResourceLoader) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ResourceLoader> for ::windows::runtime::IInspectable {
    fn from(value: &ResourceLoader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ResourceLoader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ResourceLoader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ResourceLoader {}
unsafe impl ::std::marker::Sync for ResourceLoader {}
