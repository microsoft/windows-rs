#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Security_Cryptography_DataProtection`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DataProtectionProvider(pub ::windows::runtime::IInspectable);
impl DataProtectionProvider {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DataProtectionProvider, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Security_Cryptography_DataProtection`, `Foundation`, `Storage_Streams`*"]
    pub fn ProtectAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, data: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), data.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Security_Cryptography_DataProtection`, `Foundation`, `Storage_Streams`*"]
    pub fn UnprotectAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, data: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), data.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Security_Cryptography_DataProtection`, `Foundation`, `Storage_Streams`*"]
    pub fn ProtectStreamAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Streams::IInputStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Streams::IOutputStream>>(&self, src: Param0, dest: Param1) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), src.into_param().abi(), dest.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Security_Cryptography_DataProtection`, `Foundation`, `Storage_Streams`*"]
    pub fn UnprotectStreamAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Streams::IInputStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Streams::IOutputStream>>(&self, src: Param0, dest: Param1) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), src.into_param().abi(), dest.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `Security_Cryptography_DataProtection`*"]
    pub fn CreateOverloadExplicit<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(protectiondescriptor: Param0) -> ::windows::runtime::Result<DataProtectionProvider> {
        Self::IDataProtectionProviderFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), protectiondescriptor.into_param().abi(), &mut result__).from_abi::<DataProtectionProvider>(result__)
        })
    }
    pub fn IDataProtectionProviderFactory<R, F: FnOnce(&IDataProtectionProviderFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DataProtectionProvider, IDataProtectionProviderFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DataProtectionProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.DataProtection.DataProtectionProvider;{09639948-ed22-4270-bd1c-6d72c00f8787})");
}
unsafe impl ::windows::runtime::Interface for DataProtectionProvider {
    type Vtable = IDataProtectionProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(157522248, 60706, 17008, [189, 28, 109, 114, 192, 15, 135, 135]);
}
impl ::windows::runtime::RuntimeName for DataProtectionProvider {
    const NAME: &'static str = "Windows.Security.Cryptography.DataProtection.DataProtectionProvider";
}
impl ::std::convert::From<DataProtectionProvider> for ::windows::runtime::IUnknown {
    fn from(value: DataProtectionProvider) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&DataProtectionProvider> for ::windows::runtime::IUnknown {
    fn from(value: &DataProtectionProvider) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DataProtectionProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DataProtectionProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<DataProtectionProvider> for ::windows::runtime::IInspectable {
    fn from(value: DataProtectionProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DataProtectionProvider> for ::windows::runtime::IInspectable {
    fn from(value: &DataProtectionProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DataProtectionProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DataProtectionProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for DataProtectionProvider {}
unsafe impl ::std::marker::Sync for DataProtectionProvider {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IDataProtectionProvider(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataProtectionProvider {
    type Vtable = IDataProtectionProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(157522248, 60706, 17008, [189, 28, 109, 114, 192, 15, 135, 135]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataProtectionProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, src: ::windows::runtime::RawPtr, dest: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, src: ::windows::runtime::RawPtr, dest: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDataProtectionProviderFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDataProtectionProviderFactory {
    type Vtable = IDataProtectionProviderFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2918399404, 18738, 19679, [172, 65, 114, 20, 51, 53, 20, 202]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataProtectionProviderFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, protectiondescriptor: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
