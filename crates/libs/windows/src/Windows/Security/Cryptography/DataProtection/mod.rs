#[doc(hidden)]
#[repr(transparent)]
pub struct IDataProtectionProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDataProtectionProvider {
    type Vtable = IDataProtectionProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IDataProtectionProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09639948_ed22_4270_bd1c_6d72c00f8787);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataProtectionProvider_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ProtectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ProtectAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub UnprotectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    UnprotectAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ProtectStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, src: *mut ::core::ffi::c_void, dest: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ProtectStreamAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub UnprotectStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, src: *mut ::core::ffi::c_void, dest: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    UnprotectStreamAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataProtectionProviderFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDataProtectionProviderFactory {
    type Vtable = IDataProtectionProviderFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IDataProtectionProviderFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xadf33dac_4932_4cdf_ac41_7214333514ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataProtectionProviderFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateOverloadExplicit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protectiondescriptor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Security_Cryptography_DataProtection\"`*"]
#[repr(transparent)]
pub struct DataProtectionProvider(::windows::core::IUnknown);
impl DataProtectionProvider {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<DataProtectionProvider, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ProtectAsync<P0, E0>(&self, data: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProtectAsync)(::windows::core::Vtable::as_raw(this), data.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn UnprotectAsync<P0, E0>(&self, data: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnprotectAsync)(::windows::core::Vtable::as_raw(this), data.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ProtectStreamAsync<P0, E0, P1, E1>(&self, src: P0, dest: P1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Storage::Streams::IInputStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Storage::Streams::IOutputStream>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProtectStreamAsync)(::windows::core::Vtable::as_raw(this), src.try_into().map_err(|e| e.into())?.abi(), dest.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn UnprotectStreamAsync<P0, E0, P1, E1>(&self, src: P0, dest: P1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Storage::Streams::IInputStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Storage::Streams::IOutputStream>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnprotectStreamAsync)(::windows::core::Vtable::as_raw(this), src.try_into().map_err(|e| e.into())?.abi(), dest.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateOverloadExplicit(protectiondescriptor: &::windows::core::HSTRING) -> ::windows::core::Result<DataProtectionProvider> {
        Self::IDataProtectionProviderFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateOverloadExplicit)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(protectiondescriptor), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDataProtectionProviderFactory<R, F: FnOnce(&IDataProtectionProviderFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<DataProtectionProvider, IDataProtectionProviderFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
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
unsafe impl ::windows::core::Vtable for DataProtectionProvider {
    type Vtable = IDataProtectionProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for DataProtectionProvider {
    const IID: ::windows::core::GUID = <IDataProtectionProvider as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DataProtectionProvider {
    const NAME: &'static str = "Windows.Security.Cryptography.DataProtection.DataProtectionProvider";
}
::windows::core::interface_hierarchy!(DataProtectionProvider, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for DataProtectionProvider {}
unsafe impl ::core::marker::Sync for DataProtectionProvider {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
