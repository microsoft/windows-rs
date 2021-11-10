#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `UI_Xaml_Resources`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CustomXamlResourceLoader(pub ::windows::runtime::IInspectable);
impl CustomXamlResourceLoader {
    #[doc = "*Required features: `UI_Xaml_Resources`*"]
    pub fn Current() -> ::windows::runtime::Result<CustomXamlResourceLoader> {
        Self::ICustomXamlResourceLoaderStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CustomXamlResourceLoader>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Resources`*"]
    pub fn SetCurrent<'a, Param0: ::windows::runtime::IntoParam<'a, CustomXamlResourceLoader>>(value: Param0) -> ::windows::runtime::Result<()> {
        Self::ICustomXamlResourceLoaderStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Resources`*"]
    pub fn new() -> ::windows::runtime::Result<CustomXamlResourceLoader> {
        Self::ICustomXamlResourceLoaderFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::runtime::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<CustomXamlResourceLoader>(result__)
        })
    }
    pub fn ICustomXamlResourceLoaderStatics<R, F: FnOnce(&ICustomXamlResourceLoaderStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CustomXamlResourceLoader, ICustomXamlResourceLoaderStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICustomXamlResourceLoaderFactory<R, F: FnOnce(&ICustomXamlResourceLoaderFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CustomXamlResourceLoader, ICustomXamlResourceLoaderFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CustomXamlResourceLoader {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Resources.CustomXamlResourceLoader;{511a84ab-4a88-419f-852e-54083b90b078})");
}
unsafe impl ::windows::runtime::Interface for CustomXamlResourceLoader {
    type Vtable = ICustomXamlResourceLoader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x511a84ab_4a88_419f_852e_54083b90b078);
}
impl ::windows::runtime::RuntimeName for CustomXamlResourceLoader {
    const NAME: &'static str = "Windows.UI.Xaml.Resources.CustomXamlResourceLoader";
}
impl ::core::convert::From<CustomXamlResourceLoader> for ::windows::runtime::IUnknown {
    fn from(value: CustomXamlResourceLoader) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CustomXamlResourceLoader> for ::windows::runtime::IUnknown {
    fn from(value: &CustomXamlResourceLoader) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CustomXamlResourceLoader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CustomXamlResourceLoader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CustomXamlResourceLoader> for ::windows::runtime::IInspectable {
    fn from(value: CustomXamlResourceLoader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CustomXamlResourceLoader> for ::windows::runtime::IInspectable {
    fn from(value: &CustomXamlResourceLoader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CustomXamlResourceLoader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CustomXamlResourceLoader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CustomXamlResourceLoader {}
unsafe impl ::core::marker::Sync for CustomXamlResourceLoader {}
#[repr(transparent)]
#[doc(hidden)]
pub struct ICustomXamlResourceLoader(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICustomXamlResourceLoader {
    type Vtable = ICustomXamlResourceLoader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x511a84ab_4a88_419f_852e_54083b90b078);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomXamlResourceLoader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICustomXamlResourceLoaderFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICustomXamlResourceLoaderFactory {
    type Vtable = ICustomXamlResourceLoaderFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5bfd7e49_7886_44f3_8ed3_6fec0463ed69);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomXamlResourceLoaderFactory_abi(
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
pub struct ICustomXamlResourceLoaderOverrides(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICustomXamlResourceLoaderOverrides {
    type Vtable = ICustomXamlResourceLoaderOverrides_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf851e991_af02_46e8_9af8_427b7ebfe9f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomXamlResourceLoaderOverrides_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resourceid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, objecttype: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, propertyname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, propertytype: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICustomXamlResourceLoaderStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICustomXamlResourceLoaderStatics {
    type Vtable = ICustomXamlResourceLoaderStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x224ff617_e4dc_4c27_ad32_db93d5d0e5da);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomXamlResourceLoaderStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
