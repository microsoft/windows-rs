#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Security_Cryptography_DataProtection\"`*"]
#[repr(transparent)]
pub struct DataProtectionProvider(::windows::core::IUnknown);
impl DataProtectionProvider {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DataProtectionProvider, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Security_Cryptography_DataProtection\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ProtectAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, data: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProtectAsync)(::core::mem::transmute_copy(this), data.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Cryptography_DataProtection\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn UnprotectAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, data: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnprotectAsync)(::core::mem::transmute_copy(this), data.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Cryptography_DataProtection\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ProtectStreamAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IInputStream>, Param1: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IOutputStream>>(&self, src: Param0, dest: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProtectStreamAsync)(::core::mem::transmute_copy(this), src.into_param().abi(), dest.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Cryptography_DataProtection\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn UnprotectStreamAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IInputStream>, Param1: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IOutputStream>>(&self, src: Param0, dest: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnprotectStreamAsync)(::core::mem::transmute_copy(this), src.into_param().abi(), dest.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Cryptography_DataProtection\"`*"]
    pub fn CreateOverloadExplicit<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(protectiondescriptor: Param0) -> ::windows::core::Result<DataProtectionProvider> {
        Self::IDataProtectionProviderFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateOverloadExplicit)(::core::mem::transmute_copy(this), protectiondescriptor.into_param().abi(), &mut result__).from_abi::<DataProtectionProvider>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDataProtectionProviderFactory<R, F: FnOnce(&IDataProtectionProviderFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DataProtectionProvider, IDataProtectionProviderFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DataProtectionProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DataProtectionProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DataProtectionProvider {}
impl ::core::fmt::Debug for DataProtectionProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataProtectionProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DataProtectionProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.DataProtection.DataProtectionProvider;{09639948-ed22-4270-bd1c-6d72c00f8787})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DataProtectionProvider {
    type Vtable = IDataProtectionProvider_Vtbl;
    const IID: ::windows::core::GUID = <IDataProtectionProvider as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DataProtectionProvider {
    const NAME: &'static str = "Windows.Security.Cryptography.DataProtection.DataProtectionProvider";
}
impl ::core::convert::From<DataProtectionProvider> for ::windows::core::IUnknown {
    fn from(value: DataProtectionProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DataProtectionProvider> for ::windows::core::IUnknown {
    fn from(value: &DataProtectionProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DataProtectionProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DataProtectionProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DataProtectionProvider> for ::windows::core::IInspectable {
    fn from(value: DataProtectionProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DataProtectionProvider> for ::windows::core::IInspectable {
    fn from(value: &DataProtectionProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DataProtectionProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DataProtectionProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DataProtectionProvider {}
unsafe impl ::core::marker::Sync for DataProtectionProvider {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataProtectionProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDataProtectionProvider {
    type Vtable = IDataProtectionProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09639948_ed22_4270_bd1c_6d72c00f8787);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataProtectionProvider_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ProtectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ProtectAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub UnprotectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    UnprotectAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ProtectStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, src: ::windows::core::RawPtr, dest: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ProtectStreamAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub UnprotectStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, src: ::windows::core::RawPtr, dest: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    UnprotectStreamAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataProtectionProviderFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDataProtectionProviderFactory {
    type Vtable = IDataProtectionProviderFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xadf33dac_4932_4cdf_ac41_7214333514ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataProtectionProviderFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateOverloadExplicit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protectiondescriptor: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
