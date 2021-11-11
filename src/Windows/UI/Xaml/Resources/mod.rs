#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `UI_Xaml_Resources`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CustomXamlResourceLoader(pub ::windows::core::IInspectable);
impl CustomXamlResourceLoader {
    #[doc = "*Required features: `UI_Xaml_Resources`*"]
    pub fn Current() -> ::windows::core::Result<CustomXamlResourceLoader> {
        Self::ICustomXamlResourceLoaderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CustomXamlResourceLoader>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Resources`*"]
    pub fn SetCurrent<'a, Param0: ::windows::core::IntoParam<'a, CustomXamlResourceLoader>>(value: Param0) -> ::windows::core::Result<()> {
        Self::ICustomXamlResourceLoaderStatics(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `UI_Xaml_Resources`*"]
    pub fn new() -> ::windows::core::Result<CustomXamlResourceLoader> {
        Self::ICustomXamlResourceLoaderFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<CustomXamlResourceLoader>(result__)
        })
    }
    pub fn ICustomXamlResourceLoaderStatics<R, F: FnOnce(&ICustomXamlResourceLoaderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CustomXamlResourceLoader, ICustomXamlResourceLoaderStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICustomXamlResourceLoaderFactory<R, F: FnOnce(&ICustomXamlResourceLoaderFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CustomXamlResourceLoader, ICustomXamlResourceLoaderFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for CustomXamlResourceLoader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Resources.CustomXamlResourceLoader;{511a84ab-4a88-419f-852e-54083b90b078})");
}
unsafe impl ::windows::core::Interface for CustomXamlResourceLoader {
    type Vtable = ICustomXamlResourceLoader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x511a84ab_4a88_419f_852e_54083b90b078);
}
impl ::windows::core::RuntimeName for CustomXamlResourceLoader {
    const NAME: &'static str = "Windows.UI.Xaml.Resources.CustomXamlResourceLoader";
}
impl ::core::convert::From<CustomXamlResourceLoader> for ::windows::core::IUnknown {
    fn from(value: CustomXamlResourceLoader) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CustomXamlResourceLoader> for ::windows::core::IUnknown {
    fn from(value: &CustomXamlResourceLoader) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CustomXamlResourceLoader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CustomXamlResourceLoader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CustomXamlResourceLoader> for ::windows::core::IInspectable {
    fn from(value: CustomXamlResourceLoader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CustomXamlResourceLoader> for ::windows::core::IInspectable {
    fn from(value: &CustomXamlResourceLoader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CustomXamlResourceLoader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CustomXamlResourceLoader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CustomXamlResourceLoader {}
unsafe impl ::core::marker::Sync for CustomXamlResourceLoader {}
#[repr(transparent)]
#[doc(hidden)]
pub struct ICustomXamlResourceLoader(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICustomXamlResourceLoader {
    type Vtable = ICustomXamlResourceLoader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x511a84ab_4a88_419f_852e_54083b90b078);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomXamlResourceLoader_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICustomXamlResourceLoaderFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICustomXamlResourceLoaderFactory {
    type Vtable = ICustomXamlResourceLoaderFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5bfd7e49_7886_44f3_8ed3_6fec0463ed69);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomXamlResourceLoaderFactory_abi(
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
pub struct ICustomXamlResourceLoaderOverrides(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICustomXamlResourceLoaderOverrides {
    type Vtable = ICustomXamlResourceLoaderOverrides_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf851e991_af02_46e8_9af8_427b7ebfe9f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomXamlResourceLoaderOverrides_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, resourceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, objecttype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertytype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICustomXamlResourceLoaderStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICustomXamlResourceLoaderStatics {
    type Vtable = ICustomXamlResourceLoaderStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x224ff617_e4dc_4c27_ad32_db93d5d0e5da);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomXamlResourceLoaderStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
