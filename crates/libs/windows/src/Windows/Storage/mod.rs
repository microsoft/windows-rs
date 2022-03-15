#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Storage_AccessCache")]
pub mod AccessCache;
#[cfg(feature = "Storage_BulkAccess")]
pub mod BulkAccess;
#[cfg(feature = "Storage_Compression")]
pub mod Compression;
#[cfg(feature = "Storage_FileProperties")]
pub mod FileProperties;
#[cfg(feature = "Storage_Pickers")]
pub mod Pickers;
#[cfg(feature = "Storage_Provider")]
pub mod Provider;
#[cfg(feature = "Storage_Search")]
pub mod Search;
#[cfg(feature = "Storage_Streams")]
pub mod Streams;
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct AppDataPaths(::windows::core::IUnknown);
impl AppDataPaths {
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Cookies(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Cookies)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Desktop(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Desktop)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Documents(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Documents)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Favorites(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Favorites)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn History(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).History)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn InternetCache(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InternetCache)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn LocalAppData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LocalAppData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn ProgramData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProgramData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn RoamingAppData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RoamingAppData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetForUser<'a, Param0: ::windows::core::IntoParam<'a, super::System::User>>(user: Param0) -> ::windows::core::Result<AppDataPaths> {
        Self::IAppDataPathsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForUser)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<AppDataPaths>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn GetDefault() -> ::windows::core::Result<AppDataPaths> {
        Self::IAppDataPathsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppDataPaths>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppDataPathsStatics<R, F: FnOnce(&IAppDataPathsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppDataPaths, IAppDataPathsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AppDataPaths {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppDataPaths {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppDataPaths {}
impl ::core::fmt::Debug for AppDataPaths {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppDataPaths").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppDataPaths {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.AppDataPaths;{7301d60a-79a2-48c9-9ec0-3fda092f79e1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppDataPaths {
    type Vtable = IAppDataPaths_Vtbl;
    const IID: ::windows::core::GUID = <IAppDataPaths as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppDataPaths {
    const NAME: &'static str = "Windows.Storage.AppDataPaths";
}
impl ::core::convert::From<AppDataPaths> for ::windows::core::IUnknown {
    fn from(value: AppDataPaths) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppDataPaths> for ::windows::core::IUnknown {
    fn from(value: &AppDataPaths) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppDataPaths {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppDataPaths {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppDataPaths> for ::windows::core::IInspectable {
    fn from(value: AppDataPaths) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppDataPaths> for ::windows::core::IInspectable {
    fn from(value: &AppDataPaths) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppDataPaths {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppDataPaths {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppDataPaths {}
unsafe impl ::core::marker::Sync for AppDataPaths {}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct ApplicationData(::windows::core::IUnknown);
impl ApplicationData {
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Version(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Version)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetVersionAsync<'a, Param1: ::windows::core::IntoParam<'a, ApplicationDataSetVersionHandler>>(&self, desiredversion: u32, handler: Param1) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetVersionAsync)(::core::mem::transmute_copy(this), desiredversion, handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ClearAllAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ClearAllAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ClearAsync(&self, locality: ApplicationDataLocality) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ClearAsync)(::core::mem::transmute_copy(this), locality, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn LocalSettings(&self) -> ::windows::core::Result<ApplicationDataContainer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LocalSettings)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationDataContainer>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn RoamingSettings(&self) -> ::windows::core::Result<ApplicationDataContainer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RoamingSettings)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationDataContainer>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn LocalFolder(&self) -> ::windows::core::Result<StorageFolder> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LocalFolder)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn RoamingFolder(&self) -> ::windows::core::Result<StorageFolder> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RoamingFolder)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn TemporaryFolder(&self) -> ::windows::core::Result<StorageFolder> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TemporaryFolder)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DataChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<ApplicationData, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DataChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDataChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDataChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn SignalDataChanged(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SignalDataChanged)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn RoamingStorageQuota(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RoamingStorageQuota)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn LocalCacheFolder(&self) -> ::windows::core::Result<StorageFolder> {
        let this = &::windows::core::Interface::cast::<IApplicationData2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LocalCacheFolder)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn GetPublisherCacheFolder<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, foldername: Param0) -> ::windows::core::Result<StorageFolder> {
        let this = &::windows::core::Interface::cast::<IApplicationData3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetPublisherCacheFolder)(::core::mem::transmute_copy(this), foldername.into_param().abi(), &mut result__).from_abi::<StorageFolder>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ClearPublisherCacheFolderAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, foldername: Param0) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IApplicationData3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ClearPublisherCacheFolderAsync)(::core::mem::transmute_copy(this), foldername.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn SharedLocalFolder(&self) -> ::windows::core::Result<StorageFolder> {
        let this = &::windows::core::Interface::cast::<IApplicationData3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SharedLocalFolder)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Current() -> ::windows::core::Result<ApplicationData> {
        Self::IApplicationDataStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Current)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationData>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn GetForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::System::User>>(user: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<ApplicationData>> {
        Self::IApplicationDataStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForUserAsync)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<ApplicationData>>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc(hidden)]
    pub fn IApplicationDataStatics<R, F: FnOnce(&IApplicationDataStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ApplicationData, IApplicationDataStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IApplicationDataStatics2<R, F: FnOnce(&IApplicationDataStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ApplicationData, IApplicationDataStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ApplicationData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ApplicationData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ApplicationData {}
impl ::core::fmt::Debug for ApplicationData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ApplicationData {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.ApplicationData;{c3da6fb7-b744-4b45-b0b8-223a0938d0dc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ApplicationData {
    type Vtable = IApplicationData_Vtbl;
    const IID: ::windows::core::GUID = <IApplicationData as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ApplicationData {
    const NAME: &'static str = "Windows.Storage.ApplicationData";
}
impl ::core::convert::From<ApplicationData> for ::windows::core::IUnknown {
    fn from(value: ApplicationData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ApplicationData> for ::windows::core::IUnknown {
    fn from(value: &ApplicationData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ApplicationData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ApplicationData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ApplicationData> for ::windows::core::IInspectable {
    fn from(value: ApplicationData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ApplicationData> for ::windows::core::IInspectable {
    fn from(value: &ApplicationData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ApplicationData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ApplicationData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<ApplicationData> for super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ApplicationData) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ApplicationData> for super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ApplicationData) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::IClosable> for ApplicationData {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::IClosable> for &ApplicationData {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ApplicationData {}
unsafe impl ::core::marker::Sync for ApplicationData {}
#[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct ApplicationDataCompositeValue(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl ApplicationDataCompositeValue {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ApplicationDataCompositeValue, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::Foundation::Collections::IIterator<super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IIterable<super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).First)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IIterator<super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Lookup)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Size)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasKey)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Insert<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, key: Param0, value: Param1) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Insert)(::core::mem::transmute_copy(this), key.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Remove<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Remove)(::core::mem::transmute_copy(this), key.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Clear)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MapChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Collections::MapChangedEventHandler<::windows::core::HSTRING, ::windows::core::IInspectable>>>(&self, vhnd: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapChanged)(::core::mem::transmute_copy(this), vhnd.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveMapChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMapChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for ApplicationDataCompositeValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for ApplicationDataCompositeValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for ApplicationDataCompositeValue {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for ApplicationDataCompositeValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationDataCompositeValue").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for ApplicationDataCompositeValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.ApplicationDataCompositeValue;{8a43ed9f-f4e6-4421-acf9-1dab2986820c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for ApplicationDataCompositeValue {
    type Vtable = super::Foundation::Collections::IPropertySet_Vtbl;
    const IID: ::windows::core::GUID = <super::Foundation::Collections::IPropertySet as ::windows::core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for ApplicationDataCompositeValue {
    const NAME: &'static str = "Windows.Storage.ApplicationDataCompositeValue";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for ApplicationDataCompositeValue {
    type Item = super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>;
    type IntoIter = super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &ApplicationDataCompositeValue {
    type Item = super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>;
    type IntoIter = super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ApplicationDataCompositeValue> for ::windows::core::IUnknown {
    fn from(value: ApplicationDataCompositeValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ApplicationDataCompositeValue> for ::windows::core::IUnknown {
    fn from(value: &ApplicationDataCompositeValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ApplicationDataCompositeValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ApplicationDataCompositeValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ApplicationDataCompositeValue> for ::windows::core::IInspectable {
    fn from(value: ApplicationDataCompositeValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ApplicationDataCompositeValue> for ::windows::core::IInspectable {
    fn from(value: &ApplicationDataCompositeValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ApplicationDataCompositeValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ApplicationDataCompositeValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ApplicationDataCompositeValue> for super::Foundation::Collections::IIterable<super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>> {
    type Error = ::windows::core::Error;
    fn try_from(value: ApplicationDataCompositeValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ApplicationDataCompositeValue> for super::Foundation::Collections::IIterable<super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ApplicationDataCompositeValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::Collections::IIterable<super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> for ApplicationDataCompositeValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::Collections::IIterable<super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::Collections::IIterable<super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> for &ApplicationDataCompositeValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::Collections::IIterable<super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
        ::core::convert::TryInto::<super::Foundation::Collections::IIterable<super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ApplicationDataCompositeValue> for super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable> {
    type Error = ::windows::core::Error;
    fn try_from(value: ApplicationDataCompositeValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ApplicationDataCompositeValue> for super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ApplicationDataCompositeValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>> for ApplicationDataCompositeValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>> for &ApplicationDataCompositeValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        ::core::convert::TryInto::<super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ApplicationDataCompositeValue> for super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::IInspectable> {
    type Error = ::windows::core::Error;
    fn try_from(value: ApplicationDataCompositeValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ApplicationDataCompositeValue> for super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::IInspectable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ApplicationDataCompositeValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::IInspectable>> for ApplicationDataCompositeValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::IInspectable>> for &ApplicationDataCompositeValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        ::core::convert::TryInto::<super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::IInspectable>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ApplicationDataCompositeValue> for super::Foundation::Collections::IPropertySet {
    type Error = ::windows::core::Error;
    fn try_from(value: ApplicationDataCompositeValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ApplicationDataCompositeValue> for super::Foundation::Collections::IPropertySet {
    type Error = ::windows::core::Error;
    fn try_from(value: &ApplicationDataCompositeValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::Collections::IPropertySet> for ApplicationDataCompositeValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::Collections::IPropertySet> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::Collections::IPropertySet> for &ApplicationDataCompositeValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::Collections::IPropertySet> {
        ::core::convert::TryInto::<super::Foundation::Collections::IPropertySet>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for ApplicationDataCompositeValue {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for ApplicationDataCompositeValue {}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct ApplicationDataContainer(::windows::core::IUnknown);
impl ApplicationDataContainer {
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Locality(&self) -> ::windows::core::Result<ApplicationDataLocality> {
        let this = self;
        unsafe {
            let mut result__: ApplicationDataLocality = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Locality)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationDataLocality>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Values(&self) -> ::windows::core::Result<super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Values)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Containers(&self) -> ::windows::core::Result<super::Foundation::Collections::IMapView<::windows::core::HSTRING, ApplicationDataContainer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Containers)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IMapView<::windows::core::HSTRING, ApplicationDataContainer>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn CreateContainer<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, disposition: ApplicationDataCreateDisposition) -> ::windows::core::Result<ApplicationDataContainer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateContainer)(::core::mem::transmute_copy(this), name.into_param().abi(), disposition, &mut result__).from_abi::<ApplicationDataContainer>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn DeleteContainer<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).DeleteContainer)(::core::mem::transmute_copy(this), name.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for ApplicationDataContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ApplicationDataContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ApplicationDataContainer {}
impl ::core::fmt::Debug for ApplicationDataContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationDataContainer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ApplicationDataContainer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.ApplicationDataContainer;{c5aefd1e-f467-40ba-8566-ab640a441e1d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ApplicationDataContainer {
    type Vtable = IApplicationDataContainer_Vtbl;
    const IID: ::windows::core::GUID = <IApplicationDataContainer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ApplicationDataContainer {
    const NAME: &'static str = "Windows.Storage.ApplicationDataContainer";
}
impl ::core::convert::From<ApplicationDataContainer> for ::windows::core::IUnknown {
    fn from(value: ApplicationDataContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ApplicationDataContainer> for ::windows::core::IUnknown {
    fn from(value: &ApplicationDataContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ApplicationDataContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ApplicationDataContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ApplicationDataContainer> for ::windows::core::IInspectable {
    fn from(value: ApplicationDataContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ApplicationDataContainer> for ::windows::core::IInspectable {
    fn from(value: &ApplicationDataContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ApplicationDataContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ApplicationDataContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<ApplicationDataContainer> for super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ApplicationDataContainer) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ApplicationDataContainer> for super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ApplicationDataContainer) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::IClosable> for ApplicationDataContainer {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::IClosable> for &ApplicationDataContainer {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ApplicationDataContainer {}
unsafe impl ::core::marker::Sync for ApplicationDataContainer {}
#[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct ApplicationDataContainerSettings(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl ApplicationDataContainerSettings {
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::Foundation::Collections::IIterator<super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IIterable<super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).First)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IIterator<super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Lookup)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Size)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasKey)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Insert<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, key: Param0, value: Param1) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Insert)(::core::mem::transmute_copy(this), key.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Remove<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Remove)(::core::mem::transmute_copy(this), key.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Clear)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MapChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Collections::MapChangedEventHandler<::windows::core::HSTRING, ::windows::core::IInspectable>>>(&self, vhnd: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapChanged)(::core::mem::transmute_copy(this), vhnd.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveMapChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMapChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for ApplicationDataContainerSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for ApplicationDataContainerSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for ApplicationDataContainerSettings {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for ApplicationDataContainerSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationDataContainerSettings").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for ApplicationDataContainerSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.ApplicationDataContainerSettings;{8a43ed9f-f4e6-4421-acf9-1dab2986820c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for ApplicationDataContainerSettings {
    type Vtable = super::Foundation::Collections::IPropertySet_Vtbl;
    const IID: ::windows::core::GUID = <super::Foundation::Collections::IPropertySet as ::windows::core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for ApplicationDataContainerSettings {
    const NAME: &'static str = "Windows.Storage.ApplicationDataContainerSettings";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for ApplicationDataContainerSettings {
    type Item = super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>;
    type IntoIter = super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &ApplicationDataContainerSettings {
    type Item = super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>;
    type IntoIter = super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ApplicationDataContainerSettings> for ::windows::core::IUnknown {
    fn from(value: ApplicationDataContainerSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ApplicationDataContainerSettings> for ::windows::core::IUnknown {
    fn from(value: &ApplicationDataContainerSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ApplicationDataContainerSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ApplicationDataContainerSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ApplicationDataContainerSettings> for ::windows::core::IInspectable {
    fn from(value: ApplicationDataContainerSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ApplicationDataContainerSettings> for ::windows::core::IInspectable {
    fn from(value: &ApplicationDataContainerSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ApplicationDataContainerSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ApplicationDataContainerSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ApplicationDataContainerSettings> for super::Foundation::Collections::IIterable<super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>> {
    type Error = ::windows::core::Error;
    fn try_from(value: ApplicationDataContainerSettings) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ApplicationDataContainerSettings> for super::Foundation::Collections::IIterable<super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ApplicationDataContainerSettings) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::Collections::IIterable<super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> for ApplicationDataContainerSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::Collections::IIterable<super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::Collections::IIterable<super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> for &ApplicationDataContainerSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::Collections::IIterable<super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
        ::core::convert::TryInto::<super::Foundation::Collections::IIterable<super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ApplicationDataContainerSettings> for super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable> {
    type Error = ::windows::core::Error;
    fn try_from(value: ApplicationDataContainerSettings) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ApplicationDataContainerSettings> for super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ApplicationDataContainerSettings) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>> for ApplicationDataContainerSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>> for &ApplicationDataContainerSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        ::core::convert::TryInto::<super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ApplicationDataContainerSettings> for super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::IInspectable> {
    type Error = ::windows::core::Error;
    fn try_from(value: ApplicationDataContainerSettings) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ApplicationDataContainerSettings> for super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::IInspectable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ApplicationDataContainerSettings) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::IInspectable>> for ApplicationDataContainerSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::IInspectable>> for &ApplicationDataContainerSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        ::core::convert::TryInto::<super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::IInspectable>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ApplicationDataContainerSettings> for super::Foundation::Collections::IPropertySet {
    type Error = ::windows::core::Error;
    fn try_from(value: ApplicationDataContainerSettings) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ApplicationDataContainerSettings> for super::Foundation::Collections::IPropertySet {
    type Error = ::windows::core::Error;
    fn try_from(value: &ApplicationDataContainerSettings) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::Collections::IPropertySet> for ApplicationDataContainerSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::Collections::IPropertySet> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::Collections::IPropertySet> for &ApplicationDataContainerSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::Collections::IPropertySet> {
        ::core::convert::TryInto::<super::Foundation::Collections::IPropertySet>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for ApplicationDataContainerSettings {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for ApplicationDataContainerSettings {}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ApplicationDataCreateDisposition(pub i32);
impl ApplicationDataCreateDisposition {
    pub const Always: Self = Self(0i32);
    pub const Existing: Self = Self(1i32);
}
impl ::core::marker::Copy for ApplicationDataCreateDisposition {}
impl ::core::clone::Clone for ApplicationDataCreateDisposition {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ApplicationDataCreateDisposition {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ApplicationDataCreateDisposition {
    type Abi = Self;
}
impl ::core::fmt::Debug for ApplicationDataCreateDisposition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationDataCreateDisposition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ApplicationDataCreateDisposition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.ApplicationDataCreateDisposition;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ApplicationDataLocality(pub i32);
impl ApplicationDataLocality {
    pub const Local: Self = Self(0i32);
    pub const Roaming: Self = Self(1i32);
    pub const Temporary: Self = Self(2i32);
    pub const LocalCache: Self = Self(3i32);
    pub const SharedLocal: Self = Self(4i32);
}
impl ::core::marker::Copy for ApplicationDataLocality {}
impl ::core::clone::Clone for ApplicationDataLocality {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ApplicationDataLocality {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ApplicationDataLocality {
    type Abi = Self;
}
impl ::core::fmt::Debug for ApplicationDataLocality {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationDataLocality").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ApplicationDataLocality {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.ApplicationDataLocality;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct ApplicationDataSetVersionHandler(pub ::windows::core::IUnknown);
impl ApplicationDataSetVersionHandler {
    pub fn new<F: FnMut(&::core::option::Option<SetVersionRequest>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = ApplicationDataSetVersionHandlerBox::<F> { vtable: &ApplicationDataSetVersionHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, SetVersionRequest>>(&self, setversionrequest: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this), setversionrequest.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct ApplicationDataSetVersionHandlerBox<F: FnMut(&::core::option::Option<SetVersionRequest>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const ApplicationDataSetVersionHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<SetVersionRequest>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> ApplicationDataSetVersionHandlerBox<F> {
    const VTABLE: ApplicationDataSetVersionHandler_Vtbl = ApplicationDataSetVersionHandler_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<ApplicationDataSetVersionHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, setversionrequest: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&setversionrequest)).into()
    }
}
impl ::core::clone::Clone for ApplicationDataSetVersionHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ApplicationDataSetVersionHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ApplicationDataSetVersionHandler {}
impl ::core::fmt::Debug for ApplicationDataSetVersionHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationDataSetVersionHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ApplicationDataSetVersionHandler {
    type Vtable = ApplicationDataSetVersionHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa05791e6_cc9f_4687_acab_a364fd785463);
}
unsafe impl ::windows::core::RuntimeType for ApplicationDataSetVersionHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{a05791e6-cc9f-4687-acab-a364fd785463}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ApplicationDataSetVersionHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, setversionrequest: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Storage\"`*"]
pub struct CachedFileManager {}
impl CachedFileManager {
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn DeferUpdates<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>>(file: Param0) -> ::windows::core::Result<()> {
        Self::ICachedFileManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).DeferUpdates)(::core::mem::transmute_copy(this), file.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_Provider\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Provider"))]
    pub fn CompleteUpdatesAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>>(file: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Provider::FileUpdateStatus>> {
        Self::ICachedFileManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CompleteUpdatesAsync)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<Provider::FileUpdateStatus>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICachedFileManagerStatics<R, F: FnOnce(&ICachedFileManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CachedFileManager, ICachedFileManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for CachedFileManager {
    const NAME: &'static str = "Windows.Storage.CachedFileManager";
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CreationCollisionOption(pub i32);
impl CreationCollisionOption {
    pub const GenerateUniqueName: Self = Self(0i32);
    pub const ReplaceExisting: Self = Self(1i32);
    pub const FailIfExists: Self = Self(2i32);
    pub const OpenIfExists: Self = Self(3i32);
}
impl ::core::marker::Copy for CreationCollisionOption {}
impl ::core::clone::Clone for CreationCollisionOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CreationCollisionOption {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CreationCollisionOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for CreationCollisionOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CreationCollisionOption").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CreationCollisionOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.CreationCollisionOption;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage\"`*"]
pub struct DownloadsFolder {}
impl DownloadsFolder {
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFileAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(desiredname: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        Self::IDownloadsFolderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFileAsync)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFolderAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(desiredname: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
        Self::IDownloadsFolderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFolderAsync)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFileWithCollisionOptionAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(desiredname: Param0, option: CreationCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        Self::IDownloadsFolderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFileWithCollisionOptionAsync)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), option, &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFolderWithCollisionOptionAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(desiredname: Param0, option: CreationCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
        Self::IDownloadsFolderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFolderWithCollisionOptionAsync)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), option, &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn CreateFileForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, desiredname: Param1) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        Self::IDownloadsFolderStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFileForUserAsync)(::core::mem::transmute_copy(this), user.into_param().abi(), desiredname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn CreateFolderForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, desiredname: Param1) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
        Self::IDownloadsFolderStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFolderForUserAsync)(::core::mem::transmute_copy(this), user.into_param().abi(), desiredname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn CreateFileForUserWithCollisionOptionAsync<'a, Param0: ::windows::core::IntoParam<'a, super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, desiredname: Param1, option: CreationCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        Self::IDownloadsFolderStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFileForUserWithCollisionOptionAsync)(::core::mem::transmute_copy(this), user.into_param().abi(), desiredname.into_param().abi(), option, &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn CreateFolderForUserWithCollisionOptionAsync<'a, Param0: ::windows::core::IntoParam<'a, super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, desiredname: Param1, option: CreationCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
        Self::IDownloadsFolderStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFolderForUserWithCollisionOptionAsync)(::core::mem::transmute_copy(this), user.into_param().abi(), desiredname.into_param().abi(), option, &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDownloadsFolderStatics<R, F: FnOnce(&IDownloadsFolderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DownloadsFolder, IDownloadsFolderStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IDownloadsFolderStatics2<R, F: FnOnce(&IDownloadsFolderStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DownloadsFolder, IDownloadsFolderStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for DownloadsFolder {
    const NAME: &'static str = "Windows.Storage.DownloadsFolder";
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FileAccessMode(pub i32);
impl FileAccessMode {
    pub const Read: Self = Self(0i32);
    pub const ReadWrite: Self = Self(1i32);
}
impl ::core::marker::Copy for FileAccessMode {}
impl ::core::clone::Clone for FileAccessMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FileAccessMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FileAccessMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for FileAccessMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileAccessMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FileAccessMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.FileAccessMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FileAttributes(pub u32);
impl FileAttributes {
    pub const Normal: Self = Self(0u32);
    pub const ReadOnly: Self = Self(1u32);
    pub const Directory: Self = Self(16u32);
    pub const Archive: Self = Self(32u32);
    pub const Temporary: Self = Self(256u32);
    pub const LocallyIncomplete: Self = Self(512u32);
}
impl ::core::marker::Copy for FileAttributes {}
impl ::core::clone::Clone for FileAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FileAttributes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FileAttributes {
    type Abi = Self;
}
impl ::core::fmt::Debug for FileAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileAttributes").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for FileAttributes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FileAttributes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FileAttributes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FileAttributes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FileAttributes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for FileAttributes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.FileAttributes;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage\"`*"]
pub struct FileIO {}
impl FileIO {
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadTextAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>>(file: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IFileIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReadTextAsync)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadTextWithEncodingAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>>(file: Param0, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IFileIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReadTextWithEncodingAsync)(::core::mem::transmute_copy(this), file.into_param().abi(), encoding, &mut result__).from_abi::<super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WriteTextAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(file: Param0, contents: Param1) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IFileIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WriteTextAsync)(::core::mem::transmute_copy(this), file.into_param().abi(), contents.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteTextWithEncodingAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(file: Param0, contents: Param1, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IFileIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WriteTextWithEncodingAsync)(::core::mem::transmute_copy(this), file.into_param().abi(), contents.into_param().abi(), encoding, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AppendTextAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(file: Param0, contents: Param1) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IFileIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppendTextAsync)(::core::mem::transmute_copy(this), file.into_param().abi(), contents.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn AppendTextWithEncodingAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(file: Param0, contents: Param1, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IFileIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppendTextWithEncodingAsync)(::core::mem::transmute_copy(this), file.into_param().abi(), contents.into_param().abi(), encoding, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadLinesAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>>(file: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<::windows::core::HSTRING>>> {
        Self::IFileIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReadLinesAsync)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<::windows::core::HSTRING>>>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn ReadLinesWithEncodingAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>>(file: Param0, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<::windows::core::HSTRING>>> {
        Self::IFileIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReadLinesWithEncodingAsync)(::core::mem::transmute_copy(this), file.into_param().abi(), encoding, &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<::windows::core::HSTRING>>>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn WriteLinesAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>, Param1: ::windows::core::IntoParam<'a, super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(file: Param0, lines: Param1) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IFileIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WriteLinesAsync)(::core::mem::transmute_copy(this), file.into_param().abi(), lines.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn WriteLinesWithEncodingAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>, Param1: ::windows::core::IntoParam<'a, super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(file: Param0, lines: Param1, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IFileIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WriteLinesWithEncodingAsync)(::core::mem::transmute_copy(this), file.into_param().abi(), lines.into_param().abi(), encoding, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppendLinesAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>, Param1: ::windows::core::IntoParam<'a, super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(file: Param0, lines: Param1) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IFileIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppendLinesAsync)(::core::mem::transmute_copy(this), file.into_param().abi(), lines.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn AppendLinesWithEncodingAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>, Param1: ::windows::core::IntoParam<'a, super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(file: Param0, lines: Param1, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IFileIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppendLinesWithEncodingAsync)(::core::mem::transmute_copy(this), file.into_param().abi(), lines.into_param().abi(), encoding, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadBufferAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>>(file: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Streams::IBuffer>> {
        Self::IFileIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReadBufferAsync)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<Streams::IBuffer>>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteBufferAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>, Param1: ::windows::core::IntoParam<'a, Streams::IBuffer>>(file: Param0, buffer: Param1) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IFileIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WriteBufferAsync)(::core::mem::transmute_copy(this), file.into_param().abi(), buffer.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WriteBytesAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>>(file: Param0, buffer: &[u8]) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IFileIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WriteBytesAsync)(::core::mem::transmute_copy(this), file.into_param().abi(), buffer.len() as u32, ::core::mem::transmute(buffer.as_ptr()), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IFileIOStatics<R, F: FnOnce(&IFileIOStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FileIO, IFileIOStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for FileIO {
    const NAME: &'static str = "Windows.Storage.FileIO";
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppDataPaths(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppDataPaths {
    type Vtable = IAppDataPaths_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7301d60a_79a2_48c9_9ec0_3fda092f79e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDataPaths_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Cookies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Desktop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Documents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Favorites: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub History: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub InternetCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LocalAppData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ProgramData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RoamingAppData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppDataPathsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppDataPathsStatics {
    type Vtable = IAppDataPathsStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8eb2afe_a9d9_4b14_b999_e3921379d903);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDataPathsStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationData(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IApplicationData {
    type Vtable = IApplicationData_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3da6fb7_b744_4b45_b0b8_223a0938d0dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationData_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetVersionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredversion: u32, handler: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetVersionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ClearAllAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClearAllAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ClearAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, locality: ApplicationDataLocality, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClearAsync: usize,
    pub LocalSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RoamingSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub LocalFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RoamingFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub TemporaryFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DataChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DataChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDataChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDataChanged: usize,
    pub SignalDataChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RoamingStorageQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationData2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IApplicationData2 {
    type Vtable = IApplicationData2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e65cd69_0ba3_4e32_be29_b02de6607638);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationData2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub LocalCacheFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationData3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IApplicationData3 {
    type Vtable = IApplicationData3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc222cf4_2772_4c1d_aa2c_c9f743ade8d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationData3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetPublisherCacheFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, foldername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ClearPublisherCacheFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, foldername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClearPublisherCacheFolderAsync: usize,
    pub SharedLocalFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationDataContainer(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IApplicationDataContainer {
    type Vtable = IApplicationDataContainer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5aefd1e_f467_40ba_8566_ab640a441e1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationDataContainer_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Locality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ApplicationDataLocality) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Values: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Values: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Containers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Containers: usize,
    pub CreateContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, disposition: ApplicationDataCreateDisposition, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub DeleteContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationDataStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IApplicationDataStatics {
    type Vtable = IApplicationDataStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5612147b_e843_45e3_94d8_06169e3c8e17);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationDataStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationDataStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IApplicationDataStatics2 {
    type Vtable = IApplicationDataStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd606211_cf49_40a4_a47c_c7f0dbba8107);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationDataStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetForUserAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICachedFileManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICachedFileManagerStatics {
    type Vtable = ICachedFileManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ffc224a_e782_495d_b614_654c4f0b2370);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICachedFileManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DeferUpdates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Provider"))]
    pub CompleteUpdatesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Provider")))]
    CompleteUpdatesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDownloadsFolderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDownloadsFolderStatics {
    type Vtable = IDownloadsFolderStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27862ed0_404e_47df_a1e2_e37308be7b37);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadsFolderStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CreateFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFileAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFolderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateFileWithCollisionOptionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, option: CreationCollisionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFileWithCollisionOptionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateFolderWithCollisionOptionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, option: CreationCollisionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFolderWithCollisionOptionAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDownloadsFolderStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDownloadsFolderStatics2 {
    type Vtable = IDownloadsFolderStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe93045bd_8ef8_4f8e_8d15_ac0e265f390d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadsFolderStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub CreateFileForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    CreateFileForUserAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub CreateFolderForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    CreateFolderForUserAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub CreateFileForUserWithCollisionOptionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, option: CreationCollisionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    CreateFileForUserWithCollisionOptionAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub CreateFolderForUserWithCollisionOptionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, option: CreationCollisionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    CreateFolderForUserWithCollisionOptionAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileIOStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFileIOStatics {
    type Vtable = IFileIOStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x887411eb_7f54_4732_a5f0_5e43e3b8c2f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileIOStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ReadTextAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadTextAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ReadTextWithEncodingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, encoding: Streams::UnicodeEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ReadTextWithEncodingAsync: usize,
    #[cfg(feature = "Foundation")]
    pub WriteTextAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, contents: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WriteTextAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub WriteTextWithEncodingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, contents: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, encoding: Streams::UnicodeEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    WriteTextWithEncodingAsync: usize,
    #[cfg(feature = "Foundation")]
    pub AppendTextAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, contents: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AppendTextAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub AppendTextWithEncodingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, contents: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, encoding: Streams::UnicodeEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    AppendTextWithEncodingAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadLinesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadLinesAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub ReadLinesWithEncodingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, encoding: Streams::UnicodeEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    ReadLinesWithEncodingAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub WriteLinesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, lines: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    WriteLinesAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub WriteLinesWithEncodingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, lines: ::windows::core::RawPtr, encoding: Streams::UnicodeEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    WriteLinesWithEncodingAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub AppendLinesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, lines: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppendLinesAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub AppendLinesWithEncodingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, lines: ::windows::core::RawPtr, encoding: Streams::UnicodeEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    AppendLinesWithEncodingAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ReadBufferAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ReadBufferAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub WriteBufferAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    WriteBufferAsync: usize,
    #[cfg(feature = "Foundation")]
    pub WriteBytesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, buffer_array_size: u32, buffer: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WriteBytesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownFoldersCameraRollStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKnownFoldersCameraRollStatics {
    type Vtable = IKnownFoldersCameraRollStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d115e66_27e8_492f_b8e5_2f90896cd4cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownFoldersCameraRollStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CameraRoll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownFoldersPlaylistsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKnownFoldersPlaylistsStatics {
    type Vtable = IKnownFoldersPlaylistsStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdad5ecd6_306f_4d6a_b496_46ba8eb106ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownFoldersPlaylistsStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Playlists: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownFoldersSavedPicturesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKnownFoldersSavedPicturesStatics {
    type Vtable = IKnownFoldersSavedPicturesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x055c93ea_253d_467c_b6ca_a97da1e9a18d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownFoldersSavedPicturesStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SavedPictures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownFoldersStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKnownFoldersStatics {
    type Vtable = IKnownFoldersStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a2a7520_4802_452d_9ad9_4351ada7ec35);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownFoldersStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub MusicLibrary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub PicturesLibrary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub VideosLibrary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub DocumentsLibrary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub HomeGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RemovableDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub MediaServerDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownFoldersStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKnownFoldersStatics2 {
    type Vtable = IKnownFoldersStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x194bd0cd_cf6e_4d07_9d53_e9163a2536e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownFoldersStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Objects3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub AppCaptures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RecordedCalls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownFoldersStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKnownFoldersStatics3 {
    type Vtable = IKnownFoldersStatics3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5194341_9742_4ed5_823d_fc1401148764);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownFoldersStatics3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetFolderForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, folderid: KnownFolderId, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetFolderForUserAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownFoldersStatics4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKnownFoldersStatics4 {
    type Vtable = IKnownFoldersStatics4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1722e6bf_9ff9_4b21_bed5_90ecb13a192e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownFoldersStatics4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folderid: KnownFolderId, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub RequestAccessForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, folderid: KnownFolderId, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    RequestAccessForUserAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folderid: KnownFolderId, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFolderAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPathIOStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPathIOStatics {
    type Vtable = IPathIOStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f2f3758_8ec7_4381_922b_8f6c07d288f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathIOStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ReadTextAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadTextAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ReadTextWithEncodingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, encoding: Streams::UnicodeEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ReadTextWithEncodingAsync: usize,
    #[cfg(feature = "Foundation")]
    pub WriteTextAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contents: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WriteTextAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub WriteTextWithEncodingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contents: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, encoding: Streams::UnicodeEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    WriteTextWithEncodingAsync: usize,
    #[cfg(feature = "Foundation")]
    pub AppendTextAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contents: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AppendTextAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub AppendTextWithEncodingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contents: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, encoding: Streams::UnicodeEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    AppendTextWithEncodingAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadLinesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadLinesAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub ReadLinesWithEncodingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, encoding: Streams::UnicodeEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    ReadLinesWithEncodingAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub WriteLinesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, lines: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    WriteLinesAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub WriteLinesWithEncodingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, lines: ::windows::core::RawPtr, encoding: Streams::UnicodeEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    WriteLinesWithEncodingAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub AppendLinesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, lines: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppendLinesAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub AppendLinesWithEncodingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, lines: ::windows::core::RawPtr, encoding: Streams::UnicodeEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    AppendLinesWithEncodingAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ReadBufferAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ReadBufferAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub WriteBufferAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    WriteBufferAsync: usize,
    #[cfg(feature = "Foundation")]
    pub WriteBytesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, buffer_array_size: u32, buffer: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WriteBytesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISetVersionDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISetVersionDeferral {
    type Vtable = ISetVersionDeferral_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x033508a2_781a_437a_b078_3f32badcfe47);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISetVersionDeferral_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISetVersionRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISetVersionRequest {
    type Vtable = ISetVersionRequest_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9c76b9b_1056_4e69_8330_162619956f9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISetVersionRequest_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CurrentVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub DesiredVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct IStorageFile(::windows::core::IUnknown);
impl IStorageFile {
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn FileType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FileType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn OpenAsync(&self, accessmode: FileAccessMode) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Streams::IRandomAccessStream>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OpenAsync)(::core::mem::transmute_copy(this), accessmode, &mut result__).from_abi::<super::Foundation::IAsyncOperation<Streams::IRandomAccessStream>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenTransactedWriteAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageStreamTransaction>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OpenTransactedWriteAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageStreamTransaction>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CopyOverloadDefaultNameAndOptions<'a, Param0: ::windows::core::IntoParam<'a, IStorageFolder>>(&self, destinationfolder: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CopyOverloadDefaultNameAndOptions)(::core::mem::transmute_copy(this), destinationfolder.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CopyOverloadDefaultOptions<'a, Param0: ::windows::core::IntoParam<'a, IStorageFolder>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, destinationfolder: Param0, desirednewname: Param1) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CopyOverloadDefaultOptions)(::core::mem::transmute_copy(this), destinationfolder.into_param().abi(), desirednewname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CopyOverload<'a, Param0: ::windows::core::IntoParam<'a, IStorageFolder>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, destinationfolder: Param0, desirednewname: Param1, option: NameCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CopyOverload)(::core::mem::transmute_copy(this), destinationfolder.into_param().abi(), desirednewname.into_param().abi(), option, &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CopyAndReplaceAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>>(&self, filetoreplace: Param0) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CopyAndReplaceAsync)(::core::mem::transmute_copy(this), filetoreplace.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MoveOverloadDefaultNameAndOptions<'a, Param0: ::windows::core::IntoParam<'a, IStorageFolder>>(&self, destinationfolder: Param0) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MoveOverloadDefaultNameAndOptions)(::core::mem::transmute_copy(this), destinationfolder.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MoveOverloadDefaultOptions<'a, Param0: ::windows::core::IntoParam<'a, IStorageFolder>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, destinationfolder: Param0, desirednewname: Param1) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MoveOverloadDefaultOptions)(::core::mem::transmute_copy(this), destinationfolder.into_param().abi(), desirednewname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MoveOverload<'a, Param0: ::windows::core::IntoParam<'a, IStorageFolder>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, destinationfolder: Param0, desirednewname: Param1, option: NameCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MoveOverload)(::core::mem::transmute_copy(this), destinationfolder.into_param().abi(), desirednewname.into_param().abi(), option, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MoveAndReplaceAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>>(&self, filetoreplace: Param0) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MoveAndReplaceAsync)(::core::mem::transmute_copy(this), filetoreplace.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn OpenSequentialReadAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Streams::IInputStream>> {
        let this = &::windows::core::Interface::cast::<Streams::IInputStreamReference>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OpenSequentialReadAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<Streams::IInputStream>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn OpenReadAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Streams::IRandomAccessStreamWithContentType>> {
        let this = &::windows::core::Interface::cast::<Streams::IRandomAccessStreamReference>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OpenReadAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<Streams::IRandomAccessStreamWithContentType>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RenameAsyncOverloadDefaultOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RenameAsyncOverloadDefaultOptions)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RenameAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0, option: NameCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RenameAsync)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), option, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsyncOverloadDefaultOptions(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeleteAsyncOverloadDefaultOptions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync(&self, option: StorageDeleteOption) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeleteAsync)(::core::mem::transmute_copy(this), option, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
    pub fn GetBasicPropertiesAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::BasicProperties>> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetBasicPropertiesAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::BasicProperties>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Path)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Attributes(&self) -> ::windows::core::Result<FileAttributes> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: FileAttributes = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Attributes)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileAttributes>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DateCreated(&self) -> ::windows::core::Result<super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DateCreated)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn IsOfType(&self, r#type: StorageItemTypes) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsOfType)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::convert::From<IStorageFile> for ::windows::core::IUnknown {
    fn from(value: IStorageFile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageFile> for ::windows::core::IUnknown {
    fn from(value: &IStorageFile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IStorageFile> for ::windows::core::IInspectable {
    fn from(value: IStorageFile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageFile> for ::windows::core::IInspectable {
    fn from(value: &IStorageFile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<IStorageFile> for Streams::IInputStreamReference {
    type Error = ::windows::core::Error;
    fn try_from(value: IStorageFile) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&IStorageFile> for Streams::IInputStreamReference {
    type Error = ::windows::core::Error;
    fn try_from(value: &IStorageFile) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, Streams::IInputStreamReference> for IStorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, Streams::IInputStreamReference> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, Streams::IInputStreamReference> for &IStorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, Streams::IInputStreamReference> {
        ::core::convert::TryInto::<Streams::IInputStreamReference>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<IStorageFile> for Streams::IRandomAccessStreamReference {
    type Error = ::windows::core::Error;
    fn try_from(value: IStorageFile) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&IStorageFile> for Streams::IRandomAccessStreamReference {
    type Error = ::windows::core::Error;
    fn try_from(value: &IStorageFile) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, Streams::IRandomAccessStreamReference> for IStorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, Streams::IRandomAccessStreamReference> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, Streams::IRandomAccessStreamReference> for &IStorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, Streams::IRandomAccessStreamReference> {
        ::core::convert::TryInto::<Streams::IRandomAccessStreamReference>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IStorageFile> for IStorageItem {
    type Error = ::windows::core::Error;
    fn try_from(value: IStorageFile) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IStorageFile> for IStorageItem {
    type Error = ::windows::core::Error;
    fn try_from(value: &IStorageFile) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItem> for IStorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItem> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItem> for &IStorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItem> {
        ::core::convert::TryInto::<IStorageItem>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IStorageFile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStorageFile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageFile {}
impl ::core::fmt::Debug for IStorageFile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageFile").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageFile {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{fa3f6186-4214-428c-a64c-14c9ac7315ea}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IStorageFile {
    type Vtable = IStorageFile_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa3f6186_4214_428c_a64c_14c9ac7315ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFile_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub FileType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub OpenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accessmode: FileAccessMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    OpenAsync: usize,
    #[cfg(feature = "Foundation")]
    pub OpenTransactedWriteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenTransactedWriteAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CopyOverloadDefaultNameAndOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destinationfolder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CopyOverloadDefaultNameAndOptions: usize,
    #[cfg(feature = "Foundation")]
    pub CopyOverloadDefaultOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destinationfolder: ::windows::core::RawPtr, desirednewname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CopyOverloadDefaultOptions: usize,
    #[cfg(feature = "Foundation")]
    pub CopyOverload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destinationfolder: ::windows::core::RawPtr, desirednewname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, option: NameCollisionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CopyOverload: usize,
    #[cfg(feature = "Foundation")]
    pub CopyAndReplaceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filetoreplace: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CopyAndReplaceAsync: usize,
    #[cfg(feature = "Foundation")]
    pub MoveOverloadDefaultNameAndOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destinationfolder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MoveOverloadDefaultNameAndOptions: usize,
    #[cfg(feature = "Foundation")]
    pub MoveOverloadDefaultOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destinationfolder: ::windows::core::RawPtr, desirednewname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MoveOverloadDefaultOptions: usize,
    #[cfg(feature = "Foundation")]
    pub MoveOverload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destinationfolder: ::windows::core::RawPtr, desirednewname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, option: NameCollisionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MoveOverload: usize,
    #[cfg(feature = "Foundation")]
    pub MoveAndReplaceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filetoreplace: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MoveAndReplaceAsync: usize,
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct IStorageFile2(::windows::core::IUnknown);
impl IStorageFile2 {
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn OpenWithOptionsAsync(&self, accessmode: FileAccessMode, options: StorageOpenOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Streams::IRandomAccessStream>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OpenWithOptionsAsync)(::core::mem::transmute_copy(this), accessmode, options, &mut result__).from_abi::<super::Foundation::IAsyncOperation<Streams::IRandomAccessStream>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenTransactedWriteWithOptionsAsync(&self, options: StorageOpenOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageStreamTransaction>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OpenTransactedWriteWithOptionsAsync)(::core::mem::transmute_copy(this), options, &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageStreamTransaction>>(result__)
        }
    }
}
impl ::core::convert::From<IStorageFile2> for ::windows::core::IUnknown {
    fn from(value: IStorageFile2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageFile2> for ::windows::core::IUnknown {
    fn from(value: &IStorageFile2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageFile2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStorageFile2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IStorageFile2> for ::windows::core::IInspectable {
    fn from(value: IStorageFile2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageFile2> for ::windows::core::IInspectable {
    fn from(value: &IStorageFile2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStorageFile2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStorageFile2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStorageFile2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStorageFile2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageFile2 {}
impl ::core::fmt::Debug for IStorageFile2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageFile2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageFile2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{954e4bcf-0a77-42fb-b777-c2ed58a52e44}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IStorageFile2 {
    type Vtable = IStorageFile2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x954e4bcf_0a77_42fb_b777_c2ed58a52e44);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFile2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub OpenWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accessmode: FileAccessMode, options: StorageOpenOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    OpenWithOptionsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub OpenTransactedWriteWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: StorageOpenOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenTransactedWriteWithOptionsAsync: usize,
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct IStorageFilePropertiesWithAvailability(::windows::core::IUnknown);
impl IStorageFilePropertiesWithAvailability {
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn IsAvailable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAvailable)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::convert::From<IStorageFilePropertiesWithAvailability> for ::windows::core::IUnknown {
    fn from(value: IStorageFilePropertiesWithAvailability) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageFilePropertiesWithAvailability> for ::windows::core::IUnknown {
    fn from(value: &IStorageFilePropertiesWithAvailability) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageFilePropertiesWithAvailability {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStorageFilePropertiesWithAvailability {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IStorageFilePropertiesWithAvailability> for ::windows::core::IInspectable {
    fn from(value: IStorageFilePropertiesWithAvailability) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageFilePropertiesWithAvailability> for ::windows::core::IInspectable {
    fn from(value: &IStorageFilePropertiesWithAvailability) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStorageFilePropertiesWithAvailability {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStorageFilePropertiesWithAvailability {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStorageFilePropertiesWithAvailability {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStorageFilePropertiesWithAvailability {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageFilePropertiesWithAvailability {}
impl ::core::fmt::Debug for IStorageFilePropertiesWithAvailability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageFilePropertiesWithAvailability").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageFilePropertiesWithAvailability {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{afcbbe9b-582b-4133-9648-e44ca46ee491}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IStorageFilePropertiesWithAvailability {
    type Vtable = IStorageFilePropertiesWithAvailability_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xafcbbe9b_582b_4133_9648_e44ca46ee491);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFilePropertiesWithAvailability_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageFileStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageFileStatics {
    type Vtable = IStorageFileStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5984c710_daf2_43c8_8bb4_a4d3eacfd03f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFileStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetFileFromPathAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFileFromPathAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFileFromApplicationUriAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFileFromApplicationUriAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateStreamedFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displaynamewithextension: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, datarequested: ::windows::core::RawPtr, thumbnail: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateStreamedFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ReplaceWithStreamedFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filetoreplace: ::windows::core::RawPtr, datarequested: ::windows::core::RawPtr, thumbnail: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ReplaceWithStreamedFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateStreamedFileFromUriAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displaynamewithextension: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, uri: ::windows::core::RawPtr, thumbnail: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateStreamedFileFromUriAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ReplaceWithStreamedFileFromUriAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filetoreplace: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, thumbnail: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ReplaceWithStreamedFileFromUriAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageFileStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageFileStatics2 {
    type Vtable = IStorageFileStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c76a781_212e_4af9_8f04_740cae108974);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFileStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetFileFromPathForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetFileFromPathForUserAsync: usize,
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct IStorageFolder(::windows::core::IUnknown);
impl IStorageFolder {
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFileAsyncOverloadDefaultOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFileAsyncOverloadDefaultOptions)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFileAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0, options: CreationCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFileAsync)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), options, &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFolderAsyncOverloadDefaultOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFolderAsyncOverloadDefaultOptions)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFolderAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0, options: CreationCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFolderAsync)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), options, &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFileAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFileAsync)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFolderAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFolderAsync)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetItemAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetItemAsync)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<IStorageItem>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFilesAsyncOverloadDefaultOptionsStartAndCount(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFile>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFilesAsyncOverloadDefaultOptionsStartAndCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFile>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFoldersAsyncOverloadDefaultOptionsStartAndCount(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFolder>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFoldersAsyncOverloadDefaultOptionsStartAndCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFolder>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetItemsAsyncOverloadDefaultStartAndCount(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<IStorageItem>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetItemsAsyncOverloadDefaultStartAndCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<IStorageItem>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RenameAsyncOverloadDefaultOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RenameAsyncOverloadDefaultOptions)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RenameAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0, option: NameCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RenameAsync)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), option, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsyncOverloadDefaultOptions(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeleteAsyncOverloadDefaultOptions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync(&self, option: StorageDeleteOption) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeleteAsync)(::core::mem::transmute_copy(this), option, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
    pub fn GetBasicPropertiesAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::BasicProperties>> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetBasicPropertiesAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::BasicProperties>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Path)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Attributes(&self) -> ::windows::core::Result<FileAttributes> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: FileAttributes = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Attributes)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileAttributes>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DateCreated(&self) -> ::windows::core::Result<super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DateCreated)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn IsOfType(&self, r#type: StorageItemTypes) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsOfType)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::convert::From<IStorageFolder> for ::windows::core::IUnknown {
    fn from(value: IStorageFolder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageFolder> for ::windows::core::IUnknown {
    fn from(value: &IStorageFolder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageFolder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStorageFolder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IStorageFolder> for ::windows::core::IInspectable {
    fn from(value: IStorageFolder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageFolder> for ::windows::core::IInspectable {
    fn from(value: &IStorageFolder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStorageFolder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStorageFolder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IStorageFolder> for IStorageItem {
    type Error = ::windows::core::Error;
    fn try_from(value: IStorageFolder) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IStorageFolder> for IStorageItem {
    type Error = ::windows::core::Error;
    fn try_from(value: &IStorageFolder) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItem> for IStorageFolder {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItem> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItem> for &IStorageFolder {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItem> {
        ::core::convert::TryInto::<IStorageItem>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IStorageFolder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStorageFolder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageFolder {}
impl ::core::fmt::Debug for IStorageFolder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageFolder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageFolder {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{72d1cb78-b3ef-4f75-a80b-6fd9dae2944b}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IStorageFolder {
    type Vtable = IStorageFolder_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72d1cb78_b3ef_4f75_a80b_6fd9dae2944b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFolder_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CreateFileAsyncOverloadDefaultOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFileAsyncOverloadDefaultOptions: usize,
    #[cfg(feature = "Foundation")]
    pub CreateFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: CreationCollisionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFileAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateFolderAsyncOverloadDefaultOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFolderAsyncOverloadDefaultOptions: usize,
    #[cfg(feature = "Foundation")]
    pub CreateFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: CreationCollisionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFolderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFileAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFolderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetItemAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetItemAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFilesAsyncOverloadDefaultOptionsStartAndCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFilesAsyncOverloadDefaultOptionsStartAndCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFoldersAsyncOverloadDefaultOptionsStartAndCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFoldersAsyncOverloadDefaultOptionsStartAndCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetItemsAsyncOverloadDefaultStartAndCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetItemsAsyncOverloadDefaultStartAndCount: usize,
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct IStorageFolder2(::windows::core::IUnknown);
impl IStorageFolder2 {
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryGetItemAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryGetItemAsync)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<IStorageItem>>(result__)
        }
    }
}
impl ::core::convert::From<IStorageFolder2> for ::windows::core::IUnknown {
    fn from(value: IStorageFolder2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageFolder2> for ::windows::core::IUnknown {
    fn from(value: &IStorageFolder2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageFolder2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStorageFolder2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IStorageFolder2> for ::windows::core::IInspectable {
    fn from(value: IStorageFolder2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageFolder2> for ::windows::core::IInspectable {
    fn from(value: &IStorageFolder2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStorageFolder2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStorageFolder2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStorageFolder2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStorageFolder2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageFolder2 {}
impl ::core::fmt::Debug for IStorageFolder2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageFolder2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageFolder2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{e827e8b9-08d9-4a8e-a0ac-fe5ed3cbbbd3}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IStorageFolder2 {
    type Vtable = IStorageFolder2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe827e8b9_08d9_4a8e_a0ac_fe5ed3cbbbd3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFolder2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub TryGetItemAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryGetItemAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageFolder3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageFolder3 {
    type Vtable = IStorageFolder3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f617899_bde1_4124_aeb3_b06ad96f98d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFolder3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TryGetChangeTracker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageFolderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageFolderStatics {
    type Vtable = IStorageFolderStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08f327ff_85d5_48b9_aee9_28511e339f9f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFolderStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetFolderFromPathAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFolderFromPathAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageFolderStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageFolderStatics2 {
    type Vtable = IStorageFolderStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4656dc3_71d2_467d_8b29_371f0f62bf6f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFolderStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetFolderFromPathForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetFolderFromPathForUserAsync: usize,
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct IStorageItem(::windows::core::IUnknown);
impl IStorageItem {
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RenameAsyncOverloadDefaultOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RenameAsyncOverloadDefaultOptions)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RenameAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0, option: NameCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RenameAsync)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), option, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsyncOverloadDefaultOptions(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeleteAsyncOverloadDefaultOptions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync(&self, option: StorageDeleteOption) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeleteAsync)(::core::mem::transmute_copy(this), option, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
    pub fn GetBasicPropertiesAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::BasicProperties>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetBasicPropertiesAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::BasicProperties>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Path)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Attributes(&self) -> ::windows::core::Result<FileAttributes> {
        let this = self;
        unsafe {
            let mut result__: FileAttributes = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Attributes)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileAttributes>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DateCreated(&self) -> ::windows::core::Result<super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DateCreated)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn IsOfType(&self, r#type: StorageItemTypes) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsOfType)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::convert::From<IStorageItem> for ::windows::core::IUnknown {
    fn from(value: IStorageItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageItem> for ::windows::core::IUnknown {
    fn from(value: &IStorageItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStorageItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IStorageItem> for ::windows::core::IInspectable {
    fn from(value: IStorageItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageItem> for ::windows::core::IInspectable {
    fn from(value: &IStorageItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStorageItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStorageItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStorageItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStorageItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageItem {}
impl ::core::fmt::Debug for IStorageItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{4207a996-ca2f-42f7-bde8-8b10457a7f30}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IStorageItem {
    type Vtable = IStorageItem_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4207a996_ca2f_42f7_bde8_8b10457a7f30);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItem_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RenameAsyncOverloadDefaultOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RenameAsyncOverloadDefaultOptions: usize,
    #[cfg(feature = "Foundation")]
    pub RenameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, option: NameCollisionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RenameAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAsyncOverloadDefaultOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsyncOverloadDefaultOptions: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, option: StorageDeleteOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
    pub GetBasicPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_FileProperties")))]
    GetBasicPropertiesAsync: usize,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Attributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FileAttributes) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DateCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DateCreated: usize,
    pub IsOfType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: StorageItemTypes, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct IStorageItem2(::windows::core::IUnknown);
impl IStorageItem2 {
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetParentAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetParentAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn IsEqual<'a, Param0: ::windows::core::IntoParam<'a, IStorageItem>>(&self, item: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEqual)(::core::mem::transmute_copy(this), item.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RenameAsyncOverloadDefaultOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RenameAsyncOverloadDefaultOptions)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RenameAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0, option: NameCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RenameAsync)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), option, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsyncOverloadDefaultOptions(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeleteAsyncOverloadDefaultOptions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync(&self, option: StorageDeleteOption) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeleteAsync)(::core::mem::transmute_copy(this), option, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
    pub fn GetBasicPropertiesAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::BasicProperties>> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetBasicPropertiesAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::BasicProperties>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Path)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Attributes(&self) -> ::windows::core::Result<FileAttributes> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: FileAttributes = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Attributes)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileAttributes>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DateCreated(&self) -> ::windows::core::Result<super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DateCreated)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn IsOfType(&self, r#type: StorageItemTypes) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsOfType)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::convert::From<IStorageItem2> for ::windows::core::IUnknown {
    fn from(value: IStorageItem2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageItem2> for ::windows::core::IUnknown {
    fn from(value: &IStorageItem2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageItem2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStorageItem2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IStorageItem2> for ::windows::core::IInspectable {
    fn from(value: IStorageItem2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageItem2> for ::windows::core::IInspectable {
    fn from(value: &IStorageItem2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStorageItem2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStorageItem2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IStorageItem2> for IStorageItem {
    type Error = ::windows::core::Error;
    fn try_from(value: IStorageItem2) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IStorageItem2> for IStorageItem {
    type Error = ::windows::core::Error;
    fn try_from(value: &IStorageItem2) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItem> for IStorageItem2 {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItem> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItem> for &IStorageItem2 {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItem> {
        ::core::convert::TryInto::<IStorageItem>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IStorageItem2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStorageItem2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageItem2 {}
impl ::core::fmt::Debug for IStorageItem2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageItem2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageItem2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{53f926d2-083c-4283-b45b-81c007237e44}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IStorageItem2 {
    type Vtable = IStorageItem2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53f926d2_083c_4283_b45b_81c007237e44);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItem2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetParentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetParentAsync: usize,
    pub IsEqual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct IStorageItemProperties(::windows::core::IUnknown);
impl IStorageItemProperties {
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: FileProperties::ThumbnailMode) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetThumbnailAsyncOverloadDefaultSizeDefaultOptions)(::core::mem::transmute_copy(this), mode, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsyncOverloadDefaultOptions(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetThumbnailAsyncOverloadDefaultOptions)(::core::mem::transmute_copy(this), mode, requestedsize, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsync(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetThumbnailAsync)(::core::mem::transmute_copy(this), mode, requestedsize, options, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn DisplayType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn FolderRelativeId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FolderRelativeId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn Properties(&self) -> ::windows::core::Result<FileProperties::StorageItemContentProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileProperties::StorageItemContentProperties>(result__)
        }
    }
}
impl ::core::convert::From<IStorageItemProperties> for ::windows::core::IUnknown {
    fn from(value: IStorageItemProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageItemProperties> for ::windows::core::IUnknown {
    fn from(value: &IStorageItemProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageItemProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStorageItemProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IStorageItemProperties> for ::windows::core::IInspectable {
    fn from(value: IStorageItemProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageItemProperties> for ::windows::core::IInspectable {
    fn from(value: &IStorageItemProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStorageItemProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStorageItemProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStorageItemProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStorageItemProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageItemProperties {}
impl ::core::fmt::Debug for IStorageItemProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageItemProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageItemProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{86664478-8029-46fe-a789-1c2f3e2ffb5c}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IStorageItemProperties {
    type Vtable = IStorageItemProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86664478_8029_46fe_a789_1c2f3e2ffb5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemProperties_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub GetThumbnailAsyncOverloadDefaultSizeDefaultOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: FileProperties::ThumbnailMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams")))]
    GetThumbnailAsyncOverloadDefaultSizeDefaultOptions: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub GetThumbnailAsyncOverloadDefaultOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: FileProperties::ThumbnailMode, requestedsize: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams")))]
    GetThumbnailAsyncOverloadDefaultOptions: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub GetThumbnailAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams")))]
    GetThumbnailAsync: usize,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DisplayType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FolderRelativeId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_FileProperties")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    Properties: usize,
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct IStorageItemProperties2(::windows::core::IUnknown);
impl IStorageItemProperties2 {
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: FileProperties::ThumbnailMode) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions)(::core::mem::transmute_copy(this), mode, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetScaledImageAsThumbnailAsyncOverloadDefaultOptions(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetScaledImageAsThumbnailAsyncOverloadDefaultOptions)(::core::mem::transmute_copy(this), mode, requestedsize, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetScaledImageAsThumbnailAsync(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetScaledImageAsThumbnailAsync)(::core::mem::transmute_copy(this), mode, requestedsize, options, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: FileProperties::ThumbnailMode) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetThumbnailAsyncOverloadDefaultSizeDefaultOptions)(::core::mem::transmute_copy(this), mode, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsyncOverloadDefaultOptions(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetThumbnailAsyncOverloadDefaultOptions)(::core::mem::transmute_copy(this), mode, requestedsize, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsync(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetThumbnailAsync)(::core::mem::transmute_copy(this), mode, requestedsize, options, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn DisplayType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn FolderRelativeId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FolderRelativeId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn Properties(&self) -> ::windows::core::Result<FileProperties::StorageItemContentProperties> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileProperties::StorageItemContentProperties>(result__)
        }
    }
}
impl ::core::convert::From<IStorageItemProperties2> for ::windows::core::IUnknown {
    fn from(value: IStorageItemProperties2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageItemProperties2> for ::windows::core::IUnknown {
    fn from(value: &IStorageItemProperties2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageItemProperties2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStorageItemProperties2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IStorageItemProperties2> for ::windows::core::IInspectable {
    fn from(value: IStorageItemProperties2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageItemProperties2> for ::windows::core::IInspectable {
    fn from(value: &IStorageItemProperties2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStorageItemProperties2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStorageItemProperties2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IStorageItemProperties2> for IStorageItemProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: IStorageItemProperties2) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IStorageItemProperties2> for IStorageItemProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: &IStorageItemProperties2) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItemProperties> for IStorageItemProperties2 {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItemProperties> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItemProperties> for &IStorageItemProperties2 {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItemProperties> {
        ::core::convert::TryInto::<IStorageItemProperties>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IStorageItemProperties2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStorageItemProperties2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageItemProperties2 {}
impl ::core::fmt::Debug for IStorageItemProperties2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageItemProperties2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageItemProperties2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{8e86a951-04b9-4bd2-929d-fef3f71621d0}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IStorageItemProperties2 {
    type Vtable = IStorageItemProperties2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8e86a951_04b9_4bd2_929d_fef3f71621d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemProperties2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: FileProperties::ThumbnailMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams")))]
    GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub GetScaledImageAsThumbnailAsyncOverloadDefaultOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: FileProperties::ThumbnailMode, requestedsize: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams")))]
    GetScaledImageAsThumbnailAsyncOverloadDefaultOptions: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub GetScaledImageAsThumbnailAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams")))]
    GetScaledImageAsThumbnailAsync: usize,
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct IStorageItemPropertiesWithProvider(::windows::core::IUnknown);
impl IStorageItemPropertiesWithProvider {
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Provider(&self) -> ::windows::core::Result<StorageProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Provider)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageProvider>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: FileProperties::ThumbnailMode) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetThumbnailAsyncOverloadDefaultSizeDefaultOptions)(::core::mem::transmute_copy(this), mode, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsyncOverloadDefaultOptions(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetThumbnailAsyncOverloadDefaultOptions)(::core::mem::transmute_copy(this), mode, requestedsize, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsync(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetThumbnailAsync)(::core::mem::transmute_copy(this), mode, requestedsize, options, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn DisplayType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn FolderRelativeId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FolderRelativeId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn Properties(&self) -> ::windows::core::Result<FileProperties::StorageItemContentProperties> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileProperties::StorageItemContentProperties>(result__)
        }
    }
}
impl ::core::convert::From<IStorageItemPropertiesWithProvider> for ::windows::core::IUnknown {
    fn from(value: IStorageItemPropertiesWithProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageItemPropertiesWithProvider> for ::windows::core::IUnknown {
    fn from(value: &IStorageItemPropertiesWithProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageItemPropertiesWithProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStorageItemPropertiesWithProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IStorageItemPropertiesWithProvider> for ::windows::core::IInspectable {
    fn from(value: IStorageItemPropertiesWithProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageItemPropertiesWithProvider> for ::windows::core::IInspectable {
    fn from(value: &IStorageItemPropertiesWithProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStorageItemPropertiesWithProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStorageItemPropertiesWithProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IStorageItemPropertiesWithProvider> for IStorageItemProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: IStorageItemPropertiesWithProvider) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IStorageItemPropertiesWithProvider> for IStorageItemProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: &IStorageItemPropertiesWithProvider) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItemProperties> for IStorageItemPropertiesWithProvider {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItemProperties> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItemProperties> for &IStorageItemPropertiesWithProvider {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItemProperties> {
        ::core::convert::TryInto::<IStorageItemProperties>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IStorageItemPropertiesWithProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStorageItemPropertiesWithProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageItemPropertiesWithProvider {}
impl ::core::fmt::Debug for IStorageItemPropertiesWithProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageItemPropertiesWithProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageItemPropertiesWithProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{861bf39b-6368-4dee-b40e-74684a5ce714}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IStorageItemPropertiesWithProvider {
    type Vtable = IStorageItemPropertiesWithProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x861bf39b_6368_4dee_b40e_74684a5ce714);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemPropertiesWithProvider_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Provider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageLibrary(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageLibrary {
    type Vtable = IStorageLibrary_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1edd7103_0e5e_4d6c_b5e8_9318983d6a03);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibrary_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RequestAddFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAddFolderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestRemoveFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestRemoveFolderAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Folders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Folders: usize,
    pub SaveFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DefinitionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DefinitionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDefinitionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDefinitionChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageLibrary2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageLibrary2 {
    type Vtable = IStorageLibrary2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b0ce348_fcb3_4031_afb0_a68d7bd44534);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibrary2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ChangeTracker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageLibrary3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageLibrary3 {
    type Vtable = IStorageLibrary3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a281291_2154_4201_8113_d2c05ce1ad23);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibrary3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub AreFolderSuggestionsAvailableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AreFolderSuggestionsAvailableAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageLibraryChange(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageLibraryChange {
    type Vtable = IStorageLibraryChange_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00980b23_2be2_4909_aa48_159f5203a51e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryChange_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ChangeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorageLibraryChangeType) -> ::windows::core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PreviousPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsOfType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: StorageItemTypes, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetStorageItemAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetStorageItemAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageLibraryChangeReader(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageLibraryChangeReader {
    type Vtable = IStorageLibraryChangeReader_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf205bc83_fca2_41f9_8954_ee2e991eb96f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryChangeReader_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchAsync: usize,
    #[cfg(feature = "Foundation")]
    pub AcceptChangesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AcceptChangesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageLibraryChangeReader2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageLibraryChangeReader2 {
    type Vtable = IStorageLibraryChangeReader2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xabf4868b_fbcc_4a4f_999e_e7ab7c646dbe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryChangeReader2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetLastChangeId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageLibraryChangeTracker(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageLibraryChangeTracker {
    type Vtable = IStorageLibraryChangeTracker_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e157316_6073_44f6_9681_7492d1286c90);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryChangeTracker_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetChangeReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Enable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageLibraryChangeTracker2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageLibraryChangeTracker2 {
    type Vtable = IStorageLibraryChangeTracker2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd051c3b_0f9f_42f9_8fb3_158d82e13821);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryChangeTracker2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub EnableWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Disable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageLibraryChangeTrackerOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageLibraryChangeTrackerOptions {
    type Vtable = IStorageLibraryChangeTrackerOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb52bcd4_1a6d_59c0_ad2a_823a20532483);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryChangeTrackerOptions_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TrackChangeDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetTrackChangeDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageLibraryLastChangeId(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageLibraryLastChangeId {
    type Vtable = IStorageLibraryLastChangeId_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5281826a_bbe1_53bc_82ca_81cc7f039329);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryLastChangeId_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageLibraryLastChangeIdStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageLibraryLastChangeIdStatics {
    type Vtable = IStorageLibraryLastChangeIdStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81a49128_2ca3_5309_b0d1_cf0788e40762);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryLastChangeIdStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Unknown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageLibraryStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageLibraryStatics {
    type Vtable = IStorageLibraryStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4208a6db_684a_49c6_9e59_90121ee050d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetLibraryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, libraryid: KnownLibraryId, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetLibraryAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageLibraryStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageLibraryStatics2 {
    type Vtable = IStorageLibraryStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xffb08ddc_fa75_4695_b9d1_7f81f97832e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetLibraryForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, libraryid: KnownLibraryId, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetLibraryForUserAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageProvider {
    type Vtable = IStorageProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe705eed4_d478_47d6_ba46_1a8ebe114a20);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProvider_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProvider2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageProvider2 {
    type Vtable = IStorageProvider2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x010d1917_3404_414b_9fd7_cd44472eaa39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProvider2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub IsPropertySupportedForPartialFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertycanonicalname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsPropertySupportedForPartialFileAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageStreamTransaction(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageStreamTransaction {
    type Vtable = IStorageStreamTransaction_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf67cf363_a53d_4d94_ae2c_67232d93acdd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageStreamTransaction_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Stream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Stream: usize,
    #[cfg(feature = "Foundation")]
    pub CommitAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CommitAsync: usize,
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct IStreamedFileDataRequest(::windows::core::IUnknown);
impl IStreamedFileDataRequest {
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn FailAndClose(&self, failuremode: StreamedFileFailureMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).FailAndClose)(::core::mem::transmute_copy(this), failuremode).ok() }
    }
}
impl ::core::convert::From<IStreamedFileDataRequest> for ::windows::core::IUnknown {
    fn from(value: IStreamedFileDataRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStreamedFileDataRequest> for ::windows::core::IUnknown {
    fn from(value: &IStreamedFileDataRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStreamedFileDataRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStreamedFileDataRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IStreamedFileDataRequest> for ::windows::core::IInspectable {
    fn from(value: IStreamedFileDataRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStreamedFileDataRequest> for ::windows::core::IInspectable {
    fn from(value: &IStreamedFileDataRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStreamedFileDataRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStreamedFileDataRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStreamedFileDataRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStreamedFileDataRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStreamedFileDataRequest {}
impl ::core::fmt::Debug for IStreamedFileDataRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStreamedFileDataRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IStreamedFileDataRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{1673fcce-dabd-4d50-beee-180b8a8191b6}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IStreamedFileDataRequest {
    type Vtable = IStreamedFileDataRequest_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1673fcce_dabd_4d50_beee_180b8a8191b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamedFileDataRequest_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub FailAndClose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, failuremode: StreamedFileFailureMode) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemAudioProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemAudioProperties {
    type Vtable = ISystemAudioProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f8f38b7_308c_47e1_924d_8645348e5db7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemAudioProperties_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub EncodingBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemDataPaths(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemDataPaths {
    type Vtable = ISystemDataPaths_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe32abf70_d8fa_45ec_a942_d2e26fb60ba5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemDataPaths_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Fonts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ProgramData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Public: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PublicDesktop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PublicDocuments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PublicDownloads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PublicMusic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PublicPictures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PublicVideos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub System: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SystemHost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SystemX86: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SystemX64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SystemArm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub UserProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Windows: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemDataPathsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemDataPathsStatics {
    type Vtable = ISystemDataPathsStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0f96fd0_9920_4bca_b379_f96fdf7caad8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemDataPathsStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemGPSProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemGPSProperties {
    type Vtable = ISystemGPSProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0f46eb4_c174_481a_bc25_921986f6a6f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemGPSProperties_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub LatitudeDecimal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LongitudeDecimal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemImageProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemImageProperties {
    type Vtable = ISystemImageProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x011b2e30_8b39_4308_bea1_e8aa61e47826);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemImageProperties_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub HorizontalSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub VerticalSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemMediaProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemMediaProperties {
    type Vtable = ISystemMediaProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa42b3316_8415_40dc_8c44_98361d235430);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMediaProperties_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Producer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Publisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SubTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Writer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Year: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemMusicProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemMusicProperties {
    type Vtable = ISystemMusicProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb47988d5_67af_4bc3_8d39_5b89022026a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMusicProperties_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AlbumArtist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AlbumTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Artist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Composer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Conductor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DisplayArtist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Genre: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TrackNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemPhotoProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemPhotoProperties {
    type Vtable = ISystemPhotoProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4734fc3d_ab21_4424_b735_f4353a56c8fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemPhotoProperties_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CameraManufacturer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CameraModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DateTaken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Orientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PeopleNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemProperties {
    type Vtable = ISystemProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x917a71c1_85f3_4dd1_b001_a50bfd21c8d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemProperties_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Author: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Comment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ItemNameDisplay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Keywords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Rating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Audio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GPS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Media: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Music: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Photo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Video: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Image: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemVideoProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemVideoProperties {
    type Vtable = ISystemVideoProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2040f715_67f8_4322_9b80_4fa9fefb83e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemVideoProperties_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Director: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FrameHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FrameWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Orientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TotalBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataPaths(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataPaths {
    type Vtable = IUserDataPaths_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9c53912_abc4_46ff_8a2b_dc9d7fa6e52f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataPaths_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CameraRoll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Cookies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Desktop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Documents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Downloads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Favorites: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub History: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub InternetCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LocalAppData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LocalAppDataLow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Music: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Pictures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Profile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Recent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RoamingAppData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SavedPictures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Screenshots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Templates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Videos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataPathsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataPathsStatics {
    type Vtable = IUserDataPathsStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01b29def_e062_48a1_8b0c_f2c7a9ca56c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataPathsStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct KnownFolderId(pub i32);
impl KnownFolderId {
    pub const AppCaptures: Self = Self(0i32);
    pub const CameraRoll: Self = Self(1i32);
    pub const DocumentsLibrary: Self = Self(2i32);
    pub const HomeGroup: Self = Self(3i32);
    pub const MediaServerDevices: Self = Self(4i32);
    pub const MusicLibrary: Self = Self(5i32);
    pub const Objects3D: Self = Self(6i32);
    pub const PicturesLibrary: Self = Self(7i32);
    pub const Playlists: Self = Self(8i32);
    pub const RecordedCalls: Self = Self(9i32);
    pub const RemovableDevices: Self = Self(10i32);
    pub const SavedPictures: Self = Self(11i32);
    pub const Screenshots: Self = Self(12i32);
    pub const VideosLibrary: Self = Self(13i32);
    pub const AllAppMods: Self = Self(14i32);
    pub const CurrentAppMods: Self = Self(15i32);
    pub const DownloadsFolder: Self = Self(16i32);
}
impl ::core::marker::Copy for KnownFolderId {}
impl ::core::clone::Clone for KnownFolderId {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KnownFolderId {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KnownFolderId {
    type Abi = Self;
}
impl ::core::fmt::Debug for KnownFolderId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KnownFolderId").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for KnownFolderId {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.KnownFolderId;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage\"`*"]
pub struct KnownFolders {}
impl KnownFolders {
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn CameraRoll() -> ::windows::core::Result<StorageFolder> {
        Self::IKnownFoldersCameraRollStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CameraRoll)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Playlists() -> ::windows::core::Result<StorageFolder> {
        Self::IKnownFoldersPlaylistsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Playlists)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn SavedPictures() -> ::windows::core::Result<StorageFolder> {
        Self::IKnownFoldersSavedPicturesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SavedPictures)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn MusicLibrary() -> ::windows::core::Result<StorageFolder> {
        Self::IKnownFoldersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MusicLibrary)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn PicturesLibrary() -> ::windows::core::Result<StorageFolder> {
        Self::IKnownFoldersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PicturesLibrary)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn VideosLibrary() -> ::windows::core::Result<StorageFolder> {
        Self::IKnownFoldersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideosLibrary)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn DocumentsLibrary() -> ::windows::core::Result<StorageFolder> {
        Self::IKnownFoldersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DocumentsLibrary)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn HomeGroup() -> ::windows::core::Result<StorageFolder> {
        Self::IKnownFoldersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HomeGroup)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn RemovableDevices() -> ::windows::core::Result<StorageFolder> {
        Self::IKnownFoldersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemovableDevices)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn MediaServerDevices() -> ::windows::core::Result<StorageFolder> {
        Self::IKnownFoldersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MediaServerDevices)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Objects3D() -> ::windows::core::Result<StorageFolder> {
        Self::IKnownFoldersStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Objects3D)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn AppCaptures() -> ::windows::core::Result<StorageFolder> {
        Self::IKnownFoldersStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppCaptures)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn RecordedCalls() -> ::windows::core::Result<StorageFolder> {
        Self::IKnownFoldersStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RecordedCalls)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn GetFolderForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::System::User>>(user: Param0, folderid: KnownFolderId) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
        Self::IKnownFoldersStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFolderForUserAsync)(::core::mem::transmute_copy(this), user.into_param().abi(), folderid, &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync(folderid: KnownFolderId) -> ::windows::core::Result<super::Foundation::IAsyncOperation<KnownFoldersAccessStatus>> {
        Self::IKnownFoldersStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestAccessAsync)(::core::mem::transmute_copy(this), folderid, &mut result__).from_abi::<super::Foundation::IAsyncOperation<KnownFoldersAccessStatus>>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn RequestAccessForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::System::User>>(user: Param0, folderid: KnownFolderId) -> ::windows::core::Result<super::Foundation::IAsyncOperation<KnownFoldersAccessStatus>> {
        Self::IKnownFoldersStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestAccessForUserAsync)(::core::mem::transmute_copy(this), user.into_param().abi(), folderid, &mut result__).from_abi::<super::Foundation::IAsyncOperation<KnownFoldersAccessStatus>>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFolderAsync(folderid: KnownFolderId) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
        Self::IKnownFoldersStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFolderAsync)(::core::mem::transmute_copy(this), folderid, &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKnownFoldersCameraRollStatics<R, F: FnOnce(&IKnownFoldersCameraRollStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KnownFolders, IKnownFoldersCameraRollStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IKnownFoldersPlaylistsStatics<R, F: FnOnce(&IKnownFoldersPlaylistsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KnownFolders, IKnownFoldersPlaylistsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IKnownFoldersSavedPicturesStatics<R, F: FnOnce(&IKnownFoldersSavedPicturesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KnownFolders, IKnownFoldersSavedPicturesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IKnownFoldersStatics<R, F: FnOnce(&IKnownFoldersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KnownFolders, IKnownFoldersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IKnownFoldersStatics2<R, F: FnOnce(&IKnownFoldersStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KnownFolders, IKnownFoldersStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IKnownFoldersStatics3<R, F: FnOnce(&IKnownFoldersStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KnownFolders, IKnownFoldersStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IKnownFoldersStatics4<R, F: FnOnce(&IKnownFoldersStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KnownFolders, IKnownFoldersStatics4> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for KnownFolders {
    const NAME: &'static str = "Windows.Storage.KnownFolders";
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct KnownFoldersAccessStatus(pub i32);
impl KnownFoldersAccessStatus {
    pub const DeniedBySystem: Self = Self(0i32);
    pub const NotDeclaredByApp: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const UserPromptRequired: Self = Self(3i32);
    pub const Allowed: Self = Self(4i32);
    pub const AllowedPerAppFolder: Self = Self(5i32);
}
impl ::core::marker::Copy for KnownFoldersAccessStatus {}
impl ::core::clone::Clone for KnownFoldersAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KnownFoldersAccessStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KnownFoldersAccessStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for KnownFoldersAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KnownFoldersAccessStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for KnownFoldersAccessStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.KnownFoldersAccessStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct KnownLibraryId(pub i32);
impl KnownLibraryId {
    pub const Music: Self = Self(0i32);
    pub const Pictures: Self = Self(1i32);
    pub const Videos: Self = Self(2i32);
    pub const Documents: Self = Self(3i32);
}
impl ::core::marker::Copy for KnownLibraryId {}
impl ::core::clone::Clone for KnownLibraryId {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KnownLibraryId {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KnownLibraryId {
    type Abi = Self;
}
impl ::core::fmt::Debug for KnownLibraryId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KnownLibraryId").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for KnownLibraryId {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.KnownLibraryId;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NameCollisionOption(pub i32);
impl NameCollisionOption {
    pub const GenerateUniqueName: Self = Self(0i32);
    pub const ReplaceExisting: Self = Self(1i32);
    pub const FailIfExists: Self = Self(2i32);
}
impl ::core::marker::Copy for NameCollisionOption {}
impl ::core::clone::Clone for NameCollisionOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NameCollisionOption {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NameCollisionOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for NameCollisionOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NameCollisionOption").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for NameCollisionOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.NameCollisionOption;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage\"`*"]
pub struct PathIO {}
impl PathIO {
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadTextAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(absolutepath: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IPathIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReadTextAsync)(::core::mem::transmute_copy(this), absolutepath.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadTextWithEncodingAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(absolutepath: Param0, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IPathIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReadTextWithEncodingAsync)(::core::mem::transmute_copy(this), absolutepath.into_param().abi(), encoding, &mut result__).from_abi::<super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WriteTextAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(absolutepath: Param0, contents: Param1) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IPathIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WriteTextAsync)(::core::mem::transmute_copy(this), absolutepath.into_param().abi(), contents.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteTextWithEncodingAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(absolutepath: Param0, contents: Param1, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IPathIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WriteTextWithEncodingAsync)(::core::mem::transmute_copy(this), absolutepath.into_param().abi(), contents.into_param().abi(), encoding, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AppendTextAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(absolutepath: Param0, contents: Param1) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IPathIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppendTextAsync)(::core::mem::transmute_copy(this), absolutepath.into_param().abi(), contents.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn AppendTextWithEncodingAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(absolutepath: Param0, contents: Param1, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IPathIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppendTextWithEncodingAsync)(::core::mem::transmute_copy(this), absolutepath.into_param().abi(), contents.into_param().abi(), encoding, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadLinesAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(absolutepath: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<::windows::core::HSTRING>>> {
        Self::IPathIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReadLinesAsync)(::core::mem::transmute_copy(this), absolutepath.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<::windows::core::HSTRING>>>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn ReadLinesWithEncodingAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(absolutepath: Param0, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<::windows::core::HSTRING>>> {
        Self::IPathIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReadLinesWithEncodingAsync)(::core::mem::transmute_copy(this), absolutepath.into_param().abi(), encoding, &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<::windows::core::HSTRING>>>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn WriteLinesAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(absolutepath: Param0, lines: Param1) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IPathIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WriteLinesAsync)(::core::mem::transmute_copy(this), absolutepath.into_param().abi(), lines.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn WriteLinesWithEncodingAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(absolutepath: Param0, lines: Param1, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IPathIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WriteLinesWithEncodingAsync)(::core::mem::transmute_copy(this), absolutepath.into_param().abi(), lines.into_param().abi(), encoding, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppendLinesAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(absolutepath: Param0, lines: Param1) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IPathIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppendLinesAsync)(::core::mem::transmute_copy(this), absolutepath.into_param().abi(), lines.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn AppendLinesWithEncodingAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(absolutepath: Param0, lines: Param1, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IPathIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppendLinesWithEncodingAsync)(::core::mem::transmute_copy(this), absolutepath.into_param().abi(), lines.into_param().abi(), encoding, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadBufferAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(absolutepath: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Streams::IBuffer>> {
        Self::IPathIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReadBufferAsync)(::core::mem::transmute_copy(this), absolutepath.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<Streams::IBuffer>>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteBufferAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, Streams::IBuffer>>(absolutepath: Param0, buffer: Param1) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IPathIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WriteBufferAsync)(::core::mem::transmute_copy(this), absolutepath.into_param().abi(), buffer.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WriteBytesAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(absolutepath: Param0, buffer: &[u8]) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IPathIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WriteBytesAsync)(::core::mem::transmute_copy(this), absolutepath.into_param().abi(), buffer.len() as u32, ::core::mem::transmute(buffer.as_ptr()), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPathIOStatics<R, F: FnOnce(&IPathIOStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PathIO, IPathIOStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for PathIO {
    const NAME: &'static str = "Windows.Storage.PathIO";
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct SetVersionDeferral(::windows::core::IUnknown);
impl SetVersionDeferral {
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Complete)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for SetVersionDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SetVersionDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SetVersionDeferral {}
impl ::core::fmt::Debug for SetVersionDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SetVersionDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SetVersionDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.SetVersionDeferral;{033508a2-781a-437a-b078-3f32badcfe47})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SetVersionDeferral {
    type Vtable = ISetVersionDeferral_Vtbl;
    const IID: ::windows::core::GUID = <ISetVersionDeferral as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SetVersionDeferral {
    const NAME: &'static str = "Windows.Storage.SetVersionDeferral";
}
impl ::core::convert::From<SetVersionDeferral> for ::windows::core::IUnknown {
    fn from(value: SetVersionDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SetVersionDeferral> for ::windows::core::IUnknown {
    fn from(value: &SetVersionDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SetVersionDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SetVersionDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SetVersionDeferral> for ::windows::core::IInspectable {
    fn from(value: SetVersionDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SetVersionDeferral> for ::windows::core::IInspectable {
    fn from(value: &SetVersionDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SetVersionDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SetVersionDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SetVersionDeferral {}
unsafe impl ::core::marker::Sync for SetVersionDeferral {}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct SetVersionRequest(::windows::core::IUnknown);
impl SetVersionRequest {
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn CurrentVersion(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CurrentVersion)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn DesiredVersion(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DesiredVersion)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<SetVersionDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SetVersionDeferral>(result__)
        }
    }
}
impl ::core::clone::Clone for SetVersionRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SetVersionRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SetVersionRequest {}
impl ::core::fmt::Debug for SetVersionRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SetVersionRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SetVersionRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.SetVersionRequest;{b9c76b9b-1056-4e69-8330-162619956f9b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SetVersionRequest {
    type Vtable = ISetVersionRequest_Vtbl;
    const IID: ::windows::core::GUID = <ISetVersionRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SetVersionRequest {
    const NAME: &'static str = "Windows.Storage.SetVersionRequest";
}
impl ::core::convert::From<SetVersionRequest> for ::windows::core::IUnknown {
    fn from(value: SetVersionRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SetVersionRequest> for ::windows::core::IUnknown {
    fn from(value: &SetVersionRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SetVersionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SetVersionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SetVersionRequest> for ::windows::core::IInspectable {
    fn from(value: SetVersionRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SetVersionRequest> for ::windows::core::IInspectable {
    fn from(value: &SetVersionRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SetVersionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SetVersionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SetVersionRequest {}
unsafe impl ::core::marker::Sync for SetVersionRequest {}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StorageDeleteOption(pub i32);
impl StorageDeleteOption {
    pub const Default: Self = Self(0i32);
    pub const PermanentDelete: Self = Self(1i32);
}
impl ::core::marker::Copy for StorageDeleteOption {}
impl ::core::clone::Clone for StorageDeleteOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StorageDeleteOption {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StorageDeleteOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for StorageDeleteOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageDeleteOption").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageDeleteOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.StorageDeleteOption;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct StorageFile(::windows::core::IUnknown);
impl StorageFile {
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn OpenSequentialReadAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Streams::IInputStream>> {
        let this = &::windows::core::Interface::cast::<Streams::IInputStreamReference>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OpenSequentialReadAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<Streams::IInputStream>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn OpenReadAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Streams::IRandomAccessStreamWithContentType>> {
        let this = &::windows::core::Interface::cast::<Streams::IRandomAccessStreamReference>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OpenReadAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<Streams::IRandomAccessStreamWithContentType>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn FileType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FileType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn OpenAsync(&self, accessmode: FileAccessMode) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Streams::IRandomAccessStream>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OpenAsync)(::core::mem::transmute_copy(this), accessmode, &mut result__).from_abi::<super::Foundation::IAsyncOperation<Streams::IRandomAccessStream>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenTransactedWriteAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageStreamTransaction>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OpenTransactedWriteAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageStreamTransaction>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CopyOverloadDefaultNameAndOptions<'a, Param0: ::windows::core::IntoParam<'a, IStorageFolder>>(&self, destinationfolder: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CopyOverloadDefaultNameAndOptions)(::core::mem::transmute_copy(this), destinationfolder.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CopyOverloadDefaultOptions<'a, Param0: ::windows::core::IntoParam<'a, IStorageFolder>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, destinationfolder: Param0, desirednewname: Param1) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CopyOverloadDefaultOptions)(::core::mem::transmute_copy(this), destinationfolder.into_param().abi(), desirednewname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CopyOverload<'a, Param0: ::windows::core::IntoParam<'a, IStorageFolder>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, destinationfolder: Param0, desirednewname: Param1, option: NameCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CopyOverload)(::core::mem::transmute_copy(this), destinationfolder.into_param().abi(), desirednewname.into_param().abi(), option, &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CopyAndReplaceAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>>(&self, filetoreplace: Param0) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CopyAndReplaceAsync)(::core::mem::transmute_copy(this), filetoreplace.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MoveOverloadDefaultNameAndOptions<'a, Param0: ::windows::core::IntoParam<'a, IStorageFolder>>(&self, destinationfolder: Param0) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MoveOverloadDefaultNameAndOptions)(::core::mem::transmute_copy(this), destinationfolder.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MoveOverloadDefaultOptions<'a, Param0: ::windows::core::IntoParam<'a, IStorageFolder>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, destinationfolder: Param0, desirednewname: Param1) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MoveOverloadDefaultOptions)(::core::mem::transmute_copy(this), destinationfolder.into_param().abi(), desirednewname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MoveOverload<'a, Param0: ::windows::core::IntoParam<'a, IStorageFolder>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, destinationfolder: Param0, desirednewname: Param1, option: NameCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MoveOverload)(::core::mem::transmute_copy(this), destinationfolder.into_param().abi(), desirednewname.into_param().abi(), option, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MoveAndReplaceAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>>(&self, filetoreplace: Param0) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MoveAndReplaceAsync)(::core::mem::transmute_copy(this), filetoreplace.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn OpenWithOptionsAsync(&self, accessmode: FileAccessMode, options: StorageOpenOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Streams::IRandomAccessStream>> {
        let this = &::windows::core::Interface::cast::<IStorageFile2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OpenWithOptionsAsync)(::core::mem::transmute_copy(this), accessmode, options, &mut result__).from_abi::<super::Foundation::IAsyncOperation<Streams::IRandomAccessStream>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenTransactedWriteWithOptionsAsync(&self, options: StorageOpenOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageStreamTransaction>> {
        let this = &::windows::core::Interface::cast::<IStorageFile2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OpenTransactedWriteWithOptionsAsync)(::core::mem::transmute_copy(this), options, &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageStreamTransaction>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn IsAvailable(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IStorageFilePropertiesWithAvailability>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAvailable)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFileFromPathAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(path: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        Self::IStorageFileStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFileFromPathAsync)(::core::mem::transmute_copy(this), path.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFileFromApplicationUriAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Uri>>(uri: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        Self::IStorageFileStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFileFromApplicationUriAsync)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn CreateStreamedFileAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, StreamedFileDataRequestedHandler>, Param2: ::windows::core::IntoParam<'a, Streams::IRandomAccessStreamReference>>(displaynamewithextension: Param0, datarequested: Param1, thumbnail: Param2) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        Self::IStorageFileStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateStreamedFileAsync)(::core::mem::transmute_copy(this), displaynamewithextension.into_param().abi(), datarequested.into_param().abi(), thumbnail.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReplaceWithStreamedFileAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>, Param1: ::windows::core::IntoParam<'a, StreamedFileDataRequestedHandler>, Param2: ::windows::core::IntoParam<'a, Streams::IRandomAccessStreamReference>>(filetoreplace: Param0, datarequested: Param1, thumbnail: Param2) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        Self::IStorageFileStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReplaceWithStreamedFileAsync)(::core::mem::transmute_copy(this), filetoreplace.into_param().abi(), datarequested.into_param().abi(), thumbnail.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn CreateStreamedFileFromUriAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::Foundation::Uri>, Param2: ::windows::core::IntoParam<'a, Streams::IRandomAccessStreamReference>>(displaynamewithextension: Param0, uri: Param1, thumbnail: Param2) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        Self::IStorageFileStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateStreamedFileFromUriAsync)(::core::mem::transmute_copy(this), displaynamewithextension.into_param().abi(), uri.into_param().abi(), thumbnail.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReplaceWithStreamedFileFromUriAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>, Param1: ::windows::core::IntoParam<'a, super::Foundation::Uri>, Param2: ::windows::core::IntoParam<'a, Streams::IRandomAccessStreamReference>>(filetoreplace: Param0, uri: Param1, thumbnail: Param2) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        Self::IStorageFileStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReplaceWithStreamedFileFromUriAsync)(::core::mem::transmute_copy(this), filetoreplace.into_param().abi(), uri.into_param().abi(), thumbnail.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn GetFileFromPathForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, path: Param1) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        Self::IStorageFileStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFileFromPathForUserAsync)(::core::mem::transmute_copy(this), user.into_param().abi(), path.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RenameAsyncOverloadDefaultOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RenameAsyncOverloadDefaultOptions)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RenameAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0, option: NameCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RenameAsync)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), option, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsyncOverloadDefaultOptions(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeleteAsyncOverloadDefaultOptions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync(&self, option: StorageDeleteOption) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeleteAsync)(::core::mem::transmute_copy(this), option, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
    pub fn GetBasicPropertiesAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::BasicProperties>> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetBasicPropertiesAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::BasicProperties>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Path)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Attributes(&self) -> ::windows::core::Result<FileAttributes> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: FileAttributes = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Attributes)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileAttributes>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DateCreated(&self) -> ::windows::core::Result<super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DateCreated)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn IsOfType(&self, r#type: StorageItemTypes) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsOfType)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetParentAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
        let this = &::windows::core::Interface::cast::<IStorageItem2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetParentAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn IsEqual<'a, Param0: ::windows::core::IntoParam<'a, IStorageItem>>(&self, item: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IStorageItem2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEqual)(::core::mem::transmute_copy(this), item.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: FileProperties::ThumbnailMode) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetThumbnailAsyncOverloadDefaultSizeDefaultOptions)(::core::mem::transmute_copy(this), mode, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsyncOverloadDefaultOptions(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetThumbnailAsyncOverloadDefaultOptions)(::core::mem::transmute_copy(this), mode, requestedsize, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsync(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetThumbnailAsync)(::core::mem::transmute_copy(this), mode, requestedsize, options, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn DisplayType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn FolderRelativeId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FolderRelativeId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn Properties(&self) -> ::windows::core::Result<FileProperties::StorageItemContentProperties> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileProperties::StorageItemContentProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: FileProperties::ThumbnailMode) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions)(::core::mem::transmute_copy(this), mode, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetScaledImageAsThumbnailAsyncOverloadDefaultOptions(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetScaledImageAsThumbnailAsyncOverloadDefaultOptions)(::core::mem::transmute_copy(this), mode, requestedsize, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetScaledImageAsThumbnailAsync(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetScaledImageAsThumbnailAsync)(::core::mem::transmute_copy(this), mode, requestedsize, options, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Provider(&self) -> ::windows::core::Result<StorageProvider> {
        let this = &::windows::core::Interface::cast::<IStorageItemPropertiesWithProvider>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Provider)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageProvider>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IStorageFileStatics<R, F: FnOnce(&IStorageFileStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageFile, IStorageFileStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IStorageFileStatics2<R, F: FnOnce(&IStorageFileStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageFile, IStorageFileStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for StorageFile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageFile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageFile {}
impl ::core::fmt::Debug for StorageFile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageFile").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageFile {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.StorageFile;{fa3f6186-4214-428c-a64c-14c9ac7315ea})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StorageFile {
    type Vtable = IStorageFile_Vtbl;
    const IID: ::windows::core::GUID = <IStorageFile as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorageFile {
    const NAME: &'static str = "Windows.Storage.StorageFile";
}
impl ::core::convert::From<StorageFile> for ::windows::core::IUnknown {
    fn from(value: StorageFile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageFile> for ::windows::core::IUnknown {
    fn from(value: &StorageFile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorageFile> for ::windows::core::IInspectable {
    fn from(value: StorageFile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageFile> for ::windows::core::IInspectable {
    fn from(value: &StorageFile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<StorageFile> for Streams::IInputStreamReference {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageFile) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&StorageFile> for Streams::IInputStreamReference {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageFile) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, Streams::IInputStreamReference> for StorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, Streams::IInputStreamReference> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, Streams::IInputStreamReference> for &StorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, Streams::IInputStreamReference> {
        ::core::convert::TryInto::<Streams::IInputStreamReference>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<StorageFile> for Streams::IRandomAccessStreamReference {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageFile) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&StorageFile> for Streams::IRandomAccessStreamReference {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageFile) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, Streams::IRandomAccessStreamReference> for StorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, Streams::IRandomAccessStreamReference> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, Streams::IRandomAccessStreamReference> for &StorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, Streams::IRandomAccessStreamReference> {
        ::core::convert::TryInto::<Streams::IRandomAccessStreamReference>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<StorageFile> for IStorageFile {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageFile) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StorageFile> for IStorageFile {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageFile) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageFile> for StorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageFile> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageFile> for &StorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageFile> {
        ::core::convert::TryInto::<IStorageFile>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<StorageFile> for IStorageFile2 {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageFile) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StorageFile> for IStorageFile2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageFile) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageFile2> for StorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageFile2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageFile2> for &StorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageFile2> {
        ::core::convert::TryInto::<IStorageFile2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<StorageFile> for IStorageFilePropertiesWithAvailability {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageFile) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StorageFile> for IStorageFilePropertiesWithAvailability {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageFile) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageFilePropertiesWithAvailability> for StorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageFilePropertiesWithAvailability> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageFilePropertiesWithAvailability> for &StorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageFilePropertiesWithAvailability> {
        ::core::convert::TryInto::<IStorageFilePropertiesWithAvailability>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<StorageFile> for IStorageItem {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageFile) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StorageFile> for IStorageItem {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageFile) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItem> for StorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItem> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItem> for &StorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItem> {
        ::core::convert::TryInto::<IStorageItem>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<StorageFile> for IStorageItem2 {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageFile) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StorageFile> for IStorageItem2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageFile) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItem2> for StorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItem2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItem2> for &StorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItem2> {
        ::core::convert::TryInto::<IStorageItem2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<StorageFile> for IStorageItemProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageFile) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StorageFile> for IStorageItemProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageFile) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItemProperties> for StorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItemProperties> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItemProperties> for &StorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItemProperties> {
        ::core::convert::TryInto::<IStorageItemProperties>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<StorageFile> for IStorageItemProperties2 {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageFile) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StorageFile> for IStorageItemProperties2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageFile) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItemProperties2> for StorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItemProperties2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItemProperties2> for &StorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItemProperties2> {
        ::core::convert::TryInto::<IStorageItemProperties2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<StorageFile> for IStorageItemPropertiesWithProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageFile) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StorageFile> for IStorageItemPropertiesWithProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageFile) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItemPropertiesWithProvider> for StorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItemPropertiesWithProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItemPropertiesWithProvider> for &StorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItemPropertiesWithProvider> {
        ::core::convert::TryInto::<IStorageItemPropertiesWithProvider>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct StorageFolder(::windows::core::IUnknown);
impl StorageFolder {
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFileAsyncOverloadDefaultOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFileAsyncOverloadDefaultOptions)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFileAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0, options: CreationCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFileAsync)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), options, &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFolderAsyncOverloadDefaultOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFolderAsyncOverloadDefaultOptions)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFolderAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0, options: CreationCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFolderAsync)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), options, &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFileAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFileAsync)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFolderAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFolderAsync)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetItemAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetItemAsync)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<IStorageItem>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFilesAsyncOverloadDefaultOptionsStartAndCount(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFile>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFilesAsyncOverloadDefaultOptionsStartAndCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFile>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFoldersAsyncOverloadDefaultOptionsStartAndCount(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFolder>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFoldersAsyncOverloadDefaultOptionsStartAndCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFolder>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetItemsAsyncOverloadDefaultStartAndCount(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<IStorageItem>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetItemsAsyncOverloadDefaultStartAndCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<IStorageItem>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryGetItemAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<IStorageItem>> {
        let this = &::windows::core::Interface::cast::<IStorageFolder2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryGetItemAsync)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<IStorageItem>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn TryGetChangeTracker(&self) -> ::windows::core::Result<StorageLibraryChangeTracker> {
        let this = &::windows::core::Interface::cast::<IStorageFolder3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryGetChangeTracker)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageLibraryChangeTracker>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_Search\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Search"))]
    pub fn GetIndexedStateAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Search::IndexedState>> {
        let this = &::windows::core::Interface::cast::<Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetIndexedStateAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<Search::IndexedState>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateFileQueryOverloadDefault(&self) -> ::windows::core::Result<Search::StorageFileQueryResult> {
        let this = &::windows::core::Interface::cast::<Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFileQueryOverloadDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Search::StorageFileQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateFileQuery(&self, query: Search::CommonFileQuery) -> ::windows::core::Result<Search::StorageFileQueryResult> {
        let this = &::windows::core::Interface::cast::<Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFileQuery)(::core::mem::transmute_copy(this), query, &mut result__).from_abi::<Search::StorageFileQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateFileQueryWithOptions<'a, Param0: ::windows::core::IntoParam<'a, Search::QueryOptions>>(&self, queryoptions: Param0) -> ::windows::core::Result<Search::StorageFileQueryResult> {
        let this = &::windows::core::Interface::cast::<Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFileQueryWithOptions)(::core::mem::transmute_copy(this), queryoptions.into_param().abi(), &mut result__).from_abi::<Search::StorageFileQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateFolderQueryOverloadDefault(&self) -> ::windows::core::Result<Search::StorageFolderQueryResult> {
        let this = &::windows::core::Interface::cast::<Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFolderQueryOverloadDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Search::StorageFolderQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateFolderQuery(&self, query: Search::CommonFolderQuery) -> ::windows::core::Result<Search::StorageFolderQueryResult> {
        let this = &::windows::core::Interface::cast::<Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFolderQuery)(::core::mem::transmute_copy(this), query, &mut result__).from_abi::<Search::StorageFolderQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateFolderQueryWithOptions<'a, Param0: ::windows::core::IntoParam<'a, Search::QueryOptions>>(&self, queryoptions: Param0) -> ::windows::core::Result<Search::StorageFolderQueryResult> {
        let this = &::windows::core::Interface::cast::<Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFolderQueryWithOptions)(::core::mem::transmute_copy(this), queryoptions.into_param().abi(), &mut result__).from_abi::<Search::StorageFolderQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateItemQuery(&self) -> ::windows::core::Result<Search::StorageItemQueryResult> {
        let this = &::windows::core::Interface::cast::<Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateItemQuery)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Search::StorageItemQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateItemQueryWithOptions<'a, Param0: ::windows::core::IntoParam<'a, Search::QueryOptions>>(&self, queryoptions: Param0) -> ::windows::core::Result<Search::StorageItemQueryResult> {
        let this = &::windows::core::Interface::cast::<Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateItemQueryWithOptions)(::core::mem::transmute_copy(this), queryoptions.into_param().abi(), &mut result__).from_abi::<Search::StorageItemQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`, `\"Storage_Search\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Search"))]
    pub fn GetFilesAsync(&self, query: Search::CommonFileQuery, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFile>>> {
        let this = &::windows::core::Interface::cast::<Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFilesAsync)(::core::mem::transmute_copy(this), query, startindex, maxitemstoretrieve, &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFile>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`, `\"Storage_Search\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Search"))]
    pub fn GetFilesAsyncOverloadDefaultStartAndCount(&self, query: Search::CommonFileQuery) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFile>>> {
        let this = &::windows::core::Interface::cast::<Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFilesAsyncOverloadDefaultStartAndCount)(::core::mem::transmute_copy(this), query, &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFile>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`, `\"Storage_Search\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Search"))]
    pub fn GetFoldersAsync(&self, query: Search::CommonFolderQuery, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFolder>>> {
        let this = &::windows::core::Interface::cast::<Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFoldersAsync)(::core::mem::transmute_copy(this), query, startindex, maxitemstoretrieve, &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFolder>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`, `\"Storage_Search\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Search"))]
    pub fn GetFoldersAsyncOverloadDefaultStartAndCount(&self, query: Search::CommonFolderQuery) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFolder>>> {
        let this = &::windows::core::Interface::cast::<Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFoldersAsyncOverloadDefaultStartAndCount)(::core::mem::transmute_copy(this), query, &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFolder>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`, `\"Storage_Search\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Search"))]
    pub fn GetItemsAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<IStorageItem>>> {
        let this = &::windows::core::Interface::cast::<Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetItemsAsync)(::core::mem::transmute_copy(this), startindex, maxitemstoretrieve, &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<IStorageItem>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn AreQueryOptionsSupported<'a, Param0: ::windows::core::IntoParam<'a, Search::QueryOptions>>(&self, queryoptions: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AreQueryOptionsSupported)(::core::mem::transmute_copy(this), queryoptions.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn IsCommonFolderQuerySupported(&self, query: Search::CommonFolderQuery) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCommonFolderQuerySupported)(::core::mem::transmute_copy(this), query, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn IsCommonFileQuerySupported(&self, query: Search::CommonFileQuery) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCommonFileQuerySupported)(::core::mem::transmute_copy(this), query, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFolderFromPathAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(path: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
        Self::IStorageFolderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFolderFromPathAsync)(::core::mem::transmute_copy(this), path.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn GetFolderFromPathForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, path: Param1) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
        Self::IStorageFolderStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFolderFromPathForUserAsync)(::core::mem::transmute_copy(this), user.into_param().abi(), path.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RenameAsyncOverloadDefaultOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RenameAsyncOverloadDefaultOptions)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RenameAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0, option: NameCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RenameAsync)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), option, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsyncOverloadDefaultOptions(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeleteAsyncOverloadDefaultOptions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync(&self, option: StorageDeleteOption) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeleteAsync)(::core::mem::transmute_copy(this), option, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
    pub fn GetBasicPropertiesAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::BasicProperties>> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetBasicPropertiesAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::BasicProperties>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Path)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Attributes(&self) -> ::windows::core::Result<FileAttributes> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: FileAttributes = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Attributes)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileAttributes>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DateCreated(&self) -> ::windows::core::Result<super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DateCreated)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn IsOfType(&self, r#type: StorageItemTypes) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsOfType)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetParentAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
        let this = &::windows::core::Interface::cast::<IStorageItem2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetParentAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn IsEqual<'a, Param0: ::windows::core::IntoParam<'a, IStorageItem>>(&self, item: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IStorageItem2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEqual)(::core::mem::transmute_copy(this), item.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: FileProperties::ThumbnailMode) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetThumbnailAsyncOverloadDefaultSizeDefaultOptions)(::core::mem::transmute_copy(this), mode, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsyncOverloadDefaultOptions(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetThumbnailAsyncOverloadDefaultOptions)(::core::mem::transmute_copy(this), mode, requestedsize, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsync(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetThumbnailAsync)(::core::mem::transmute_copy(this), mode, requestedsize, options, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn DisplayType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn FolderRelativeId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FolderRelativeId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn Properties(&self) -> ::windows::core::Result<FileProperties::StorageItemContentProperties> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileProperties::StorageItemContentProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: FileProperties::ThumbnailMode) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions)(::core::mem::transmute_copy(this), mode, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetScaledImageAsThumbnailAsyncOverloadDefaultOptions(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetScaledImageAsThumbnailAsyncOverloadDefaultOptions)(::core::mem::transmute_copy(this), mode, requestedsize, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetScaledImageAsThumbnailAsync(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetScaledImageAsThumbnailAsync)(::core::mem::transmute_copy(this), mode, requestedsize, options, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Provider(&self) -> ::windows::core::Result<StorageProvider> {
        let this = &::windows::core::Interface::cast::<IStorageItemPropertiesWithProvider>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Provider)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageProvider>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IStorageFolderStatics<R, F: FnOnce(&IStorageFolderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageFolder, IStorageFolderStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IStorageFolderStatics2<R, F: FnOnce(&IStorageFolderStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageFolder, IStorageFolderStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for StorageFolder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageFolder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageFolder {}
impl ::core::fmt::Debug for StorageFolder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageFolder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageFolder {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.StorageFolder;{72d1cb78-b3ef-4f75-a80b-6fd9dae2944b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StorageFolder {
    type Vtable = IStorageFolder_Vtbl;
    const IID: ::windows::core::GUID = <IStorageFolder as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorageFolder {
    const NAME: &'static str = "Windows.Storage.StorageFolder";
}
impl ::core::convert::From<StorageFolder> for ::windows::core::IUnknown {
    fn from(value: StorageFolder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageFolder> for ::windows::core::IUnknown {
    fn from(value: &StorageFolder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageFolder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageFolder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorageFolder> for ::windows::core::IInspectable {
    fn from(value: StorageFolder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageFolder> for ::windows::core::IInspectable {
    fn from(value: &StorageFolder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageFolder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageFolder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<StorageFolder> for IStorageFolder {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageFolder) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StorageFolder> for IStorageFolder {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageFolder) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageFolder> for StorageFolder {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageFolder> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageFolder> for &StorageFolder {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageFolder> {
        ::core::convert::TryInto::<IStorageFolder>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<StorageFolder> for IStorageFolder2 {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageFolder) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StorageFolder> for IStorageFolder2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageFolder) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageFolder2> for StorageFolder {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageFolder2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageFolder2> for &StorageFolder {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageFolder2> {
        ::core::convert::TryInto::<IStorageFolder2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Storage_Search")]
impl ::core::convert::TryFrom<StorageFolder> for Search::IStorageFolderQueryOperations {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageFolder) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Search")]
impl ::core::convert::TryFrom<&StorageFolder> for Search::IStorageFolderQueryOperations {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageFolder) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Search")]
impl<'a> ::windows::core::IntoParam<'a, Search::IStorageFolderQueryOperations> for StorageFolder {
    fn into_param(self) -> ::windows::core::Param<'a, Search::IStorageFolderQueryOperations> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Search")]
impl<'a> ::windows::core::IntoParam<'a, Search::IStorageFolderQueryOperations> for &StorageFolder {
    fn into_param(self) -> ::windows::core::Param<'a, Search::IStorageFolderQueryOperations> {
        ::core::convert::TryInto::<Search::IStorageFolderQueryOperations>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<StorageFolder> for IStorageItem {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageFolder) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StorageFolder> for IStorageItem {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageFolder) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItem> for StorageFolder {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItem> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItem> for &StorageFolder {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItem> {
        ::core::convert::TryInto::<IStorageItem>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<StorageFolder> for IStorageItem2 {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageFolder) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StorageFolder> for IStorageItem2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageFolder) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItem2> for StorageFolder {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItem2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItem2> for &StorageFolder {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItem2> {
        ::core::convert::TryInto::<IStorageItem2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<StorageFolder> for IStorageItemProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageFolder) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StorageFolder> for IStorageItemProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageFolder) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItemProperties> for StorageFolder {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItemProperties> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItemProperties> for &StorageFolder {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItemProperties> {
        ::core::convert::TryInto::<IStorageItemProperties>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<StorageFolder> for IStorageItemProperties2 {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageFolder) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StorageFolder> for IStorageItemProperties2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageFolder) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItemProperties2> for StorageFolder {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItemProperties2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItemProperties2> for &StorageFolder {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItemProperties2> {
        ::core::convert::TryInto::<IStorageItemProperties2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<StorageFolder> for IStorageItemPropertiesWithProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageFolder) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StorageFolder> for IStorageItemPropertiesWithProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageFolder) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItemPropertiesWithProvider> for StorageFolder {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItemPropertiesWithProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageItemPropertiesWithProvider> for &StorageFolder {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageItemPropertiesWithProvider> {
        ::core::convert::TryInto::<IStorageItemPropertiesWithProvider>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StorageItemTypes(pub u32);
impl StorageItemTypes {
    pub const None: Self = Self(0u32);
    pub const File: Self = Self(1u32);
    pub const Folder: Self = Self(2u32);
}
impl ::core::marker::Copy for StorageItemTypes {}
impl ::core::clone::Clone for StorageItemTypes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StorageItemTypes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StorageItemTypes {
    type Abi = Self;
}
impl ::core::fmt::Debug for StorageItemTypes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageItemTypes").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for StorageItemTypes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for StorageItemTypes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for StorageItemTypes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for StorageItemTypes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for StorageItemTypes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for StorageItemTypes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.StorageItemTypes;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct StorageLibrary(::windows::core::IUnknown);
impl StorageLibrary {
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAddFolderAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestAddFolderAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestRemoveFolderAsync<'a, Param0: ::windows::core::IntoParam<'a, StorageFolder>>(&self, folder: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestRemoveFolderAsync)(::core::mem::transmute_copy(this), folder.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Folders(&self) -> ::windows::core::Result<super::Foundation::Collections::IObservableVector<StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Folders)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IObservableVector<StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn SaveFolder(&self) -> ::windows::core::Result<StorageFolder> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SaveFolder)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DefinitionChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<StorageLibrary, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DefinitionChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDefinitionChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDefinitionChanged)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn ChangeTracker(&self) -> ::windows::core::Result<StorageLibraryChangeTracker> {
        let this = &::windows::core::Interface::cast::<IStorageLibrary2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ChangeTracker)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageLibraryChangeTracker>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AreFolderSuggestionsAvailableAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IStorageLibrary3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AreFolderSuggestionsAvailableAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetLibraryAsync(libraryid: KnownLibraryId) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageLibrary>> {
        Self::IStorageLibraryStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetLibraryAsync)(::core::mem::transmute_copy(this), libraryid, &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageLibrary>>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn GetLibraryForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::System::User>>(user: Param0, libraryid: KnownLibraryId) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageLibrary>> {
        Self::IStorageLibraryStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetLibraryForUserAsync)(::core::mem::transmute_copy(this), user.into_param().abi(), libraryid, &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageLibrary>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStorageLibraryStatics<R, F: FnOnce(&IStorageLibraryStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageLibrary, IStorageLibraryStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IStorageLibraryStatics2<R, F: FnOnce(&IStorageLibraryStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageLibrary, IStorageLibraryStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for StorageLibrary {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageLibrary {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageLibrary {}
impl ::core::fmt::Debug for StorageLibrary {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageLibrary").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageLibrary {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.StorageLibrary;{1edd7103-0e5e-4d6c-b5e8-9318983d6a03})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StorageLibrary {
    type Vtable = IStorageLibrary_Vtbl;
    const IID: ::windows::core::GUID = <IStorageLibrary as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorageLibrary {
    const NAME: &'static str = "Windows.Storage.StorageLibrary";
}
impl ::core::convert::From<StorageLibrary> for ::windows::core::IUnknown {
    fn from(value: StorageLibrary) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageLibrary> for ::windows::core::IUnknown {
    fn from(value: &StorageLibrary) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageLibrary {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageLibrary {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorageLibrary> for ::windows::core::IInspectable {
    fn from(value: StorageLibrary) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageLibrary> for ::windows::core::IInspectable {
    fn from(value: &StorageLibrary) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageLibrary {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageLibrary {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct StorageLibraryChange(::windows::core::IUnknown);
impl StorageLibraryChange {
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn ChangeType(&self) -> ::windows::core::Result<StorageLibraryChangeType> {
        let this = self;
        unsafe {
            let mut result__: StorageLibraryChangeType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ChangeType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageLibraryChangeType>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Path)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn PreviousPath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PreviousPath)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn IsOfType(&self, r#type: StorageItemTypes) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsOfType)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetStorageItemAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStorageItemAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<IStorageItem>>(result__)
        }
    }
}
impl ::core::clone::Clone for StorageLibraryChange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageLibraryChange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageLibraryChange {}
impl ::core::fmt::Debug for StorageLibraryChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageLibraryChange").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageLibraryChange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.StorageLibraryChange;{00980b23-2be2-4909-aa48-159f5203a51e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StorageLibraryChange {
    type Vtable = IStorageLibraryChange_Vtbl;
    const IID: ::windows::core::GUID = <IStorageLibraryChange as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorageLibraryChange {
    const NAME: &'static str = "Windows.Storage.StorageLibraryChange";
}
impl ::core::convert::From<StorageLibraryChange> for ::windows::core::IUnknown {
    fn from(value: StorageLibraryChange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageLibraryChange> for ::windows::core::IUnknown {
    fn from(value: &StorageLibraryChange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageLibraryChange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageLibraryChange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorageLibraryChange> for ::windows::core::IInspectable {
    fn from(value: StorageLibraryChange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageLibraryChange> for ::windows::core::IInspectable {
    fn from(value: &StorageLibraryChange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageLibraryChange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageLibraryChange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StorageLibraryChange {}
unsafe impl ::core::marker::Sync for StorageLibraryChange {}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct StorageLibraryChangeReader(::windows::core::IUnknown);
impl StorageLibraryChangeReader {
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadBatchAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageLibraryChange>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReadBatchAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageLibraryChange>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AcceptChangesAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AcceptChangesAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn GetLastChangeId(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<IStorageLibraryChangeReader2>(self)?;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetLastChangeId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
}
impl ::core::clone::Clone for StorageLibraryChangeReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageLibraryChangeReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageLibraryChangeReader {}
impl ::core::fmt::Debug for StorageLibraryChangeReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageLibraryChangeReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageLibraryChangeReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.StorageLibraryChangeReader;{f205bc83-fca2-41f9-8954-ee2e991eb96f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StorageLibraryChangeReader {
    type Vtable = IStorageLibraryChangeReader_Vtbl;
    const IID: ::windows::core::GUID = <IStorageLibraryChangeReader as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorageLibraryChangeReader {
    const NAME: &'static str = "Windows.Storage.StorageLibraryChangeReader";
}
impl ::core::convert::From<StorageLibraryChangeReader> for ::windows::core::IUnknown {
    fn from(value: StorageLibraryChangeReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageLibraryChangeReader> for ::windows::core::IUnknown {
    fn from(value: &StorageLibraryChangeReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageLibraryChangeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageLibraryChangeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorageLibraryChangeReader> for ::windows::core::IInspectable {
    fn from(value: StorageLibraryChangeReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageLibraryChangeReader> for ::windows::core::IInspectable {
    fn from(value: &StorageLibraryChangeReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageLibraryChangeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageLibraryChangeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StorageLibraryChangeReader {}
unsafe impl ::core::marker::Sync for StorageLibraryChangeReader {}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct StorageLibraryChangeTracker(::windows::core::IUnknown);
impl StorageLibraryChangeTracker {
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn GetChangeReader(&self) -> ::windows::core::Result<StorageLibraryChangeReader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetChangeReader)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageLibraryChangeReader>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Enable(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Enable)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Reset)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn EnableWithOptions<'a, Param0: ::windows::core::IntoParam<'a, StorageLibraryChangeTrackerOptions>>(&self, options: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStorageLibraryChangeTracker2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).EnableWithOptions)(::core::mem::transmute_copy(this), options.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Disable(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStorageLibraryChangeTracker2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Disable)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for StorageLibraryChangeTracker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageLibraryChangeTracker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageLibraryChangeTracker {}
impl ::core::fmt::Debug for StorageLibraryChangeTracker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageLibraryChangeTracker").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageLibraryChangeTracker {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.StorageLibraryChangeTracker;{9e157316-6073-44f6-9681-7492d1286c90})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StorageLibraryChangeTracker {
    type Vtable = IStorageLibraryChangeTracker_Vtbl;
    const IID: ::windows::core::GUID = <IStorageLibraryChangeTracker as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorageLibraryChangeTracker {
    const NAME: &'static str = "Windows.Storage.StorageLibraryChangeTracker";
}
impl ::core::convert::From<StorageLibraryChangeTracker> for ::windows::core::IUnknown {
    fn from(value: StorageLibraryChangeTracker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageLibraryChangeTracker> for ::windows::core::IUnknown {
    fn from(value: &StorageLibraryChangeTracker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageLibraryChangeTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageLibraryChangeTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorageLibraryChangeTracker> for ::windows::core::IInspectable {
    fn from(value: StorageLibraryChangeTracker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageLibraryChangeTracker> for ::windows::core::IInspectable {
    fn from(value: &StorageLibraryChangeTracker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageLibraryChangeTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageLibraryChangeTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StorageLibraryChangeTracker {}
unsafe impl ::core::marker::Sync for StorageLibraryChangeTracker {}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct StorageLibraryChangeTrackerOptions(::windows::core::IUnknown);
impl StorageLibraryChangeTrackerOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageLibraryChangeTrackerOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn TrackChangeDetails(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrackChangeDetails)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn SetTrackChangeDetails(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTrackChangeDetails)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for StorageLibraryChangeTrackerOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageLibraryChangeTrackerOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageLibraryChangeTrackerOptions {}
impl ::core::fmt::Debug for StorageLibraryChangeTrackerOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageLibraryChangeTrackerOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageLibraryChangeTrackerOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.StorageLibraryChangeTrackerOptions;{bb52bcd4-1a6d-59c0-ad2a-823a20532483})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StorageLibraryChangeTrackerOptions {
    type Vtable = IStorageLibraryChangeTrackerOptions_Vtbl;
    const IID: ::windows::core::GUID = <IStorageLibraryChangeTrackerOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorageLibraryChangeTrackerOptions {
    const NAME: &'static str = "Windows.Storage.StorageLibraryChangeTrackerOptions";
}
impl ::core::convert::From<StorageLibraryChangeTrackerOptions> for ::windows::core::IUnknown {
    fn from(value: StorageLibraryChangeTrackerOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageLibraryChangeTrackerOptions> for ::windows::core::IUnknown {
    fn from(value: &StorageLibraryChangeTrackerOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageLibraryChangeTrackerOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageLibraryChangeTrackerOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorageLibraryChangeTrackerOptions> for ::windows::core::IInspectable {
    fn from(value: StorageLibraryChangeTrackerOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageLibraryChangeTrackerOptions> for ::windows::core::IInspectable {
    fn from(value: &StorageLibraryChangeTrackerOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageLibraryChangeTrackerOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageLibraryChangeTrackerOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StorageLibraryChangeTrackerOptions {}
unsafe impl ::core::marker::Sync for StorageLibraryChangeTrackerOptions {}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StorageLibraryChangeType(pub i32);
impl StorageLibraryChangeType {
    pub const Created: Self = Self(0i32);
    pub const Deleted: Self = Self(1i32);
    pub const MovedOrRenamed: Self = Self(2i32);
    pub const ContentsChanged: Self = Self(3i32);
    pub const MovedOutOfLibrary: Self = Self(4i32);
    pub const MovedIntoLibrary: Self = Self(5i32);
    pub const ContentsReplaced: Self = Self(6i32);
    pub const IndexingStatusChanged: Self = Self(7i32);
    pub const EncryptionChanged: Self = Self(8i32);
    pub const ChangeTrackingLost: Self = Self(9i32);
}
impl ::core::marker::Copy for StorageLibraryChangeType {}
impl ::core::clone::Clone for StorageLibraryChangeType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StorageLibraryChangeType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StorageLibraryChangeType {
    type Abi = Self;
}
impl ::core::fmt::Debug for StorageLibraryChangeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageLibraryChangeType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageLibraryChangeType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.StorageLibraryChangeType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct StorageLibraryLastChangeId(::windows::core::IUnknown);
impl StorageLibraryLastChangeId {
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Unknown() -> ::windows::core::Result<u64> {
        Self::IStorageLibraryLastChangeIdStatics(|this| unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Unknown)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStorageLibraryLastChangeIdStatics<R, F: FnOnce(&IStorageLibraryLastChangeIdStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageLibraryLastChangeId, IStorageLibraryLastChangeIdStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for StorageLibraryLastChangeId {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageLibraryLastChangeId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageLibraryLastChangeId {}
impl ::core::fmt::Debug for StorageLibraryLastChangeId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageLibraryLastChangeId").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageLibraryLastChangeId {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.StorageLibraryLastChangeId;{5281826a-bbe1-53bc-82ca-81cc7f039329})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StorageLibraryLastChangeId {
    type Vtable = IStorageLibraryLastChangeId_Vtbl;
    const IID: ::windows::core::GUID = <IStorageLibraryLastChangeId as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorageLibraryLastChangeId {
    const NAME: &'static str = "Windows.Storage.StorageLibraryLastChangeId";
}
impl ::core::convert::From<StorageLibraryLastChangeId> for ::windows::core::IUnknown {
    fn from(value: StorageLibraryLastChangeId) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageLibraryLastChangeId> for ::windows::core::IUnknown {
    fn from(value: &StorageLibraryLastChangeId) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageLibraryLastChangeId {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageLibraryLastChangeId {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorageLibraryLastChangeId> for ::windows::core::IInspectable {
    fn from(value: StorageLibraryLastChangeId) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageLibraryLastChangeId> for ::windows::core::IInspectable {
    fn from(value: &StorageLibraryLastChangeId) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageLibraryLastChangeId {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageLibraryLastChangeId {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StorageLibraryLastChangeId {}
unsafe impl ::core::marker::Sync for StorageLibraryLastChangeId {}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StorageOpenOptions(pub u32);
impl StorageOpenOptions {
    pub const None: Self = Self(0u32);
    pub const AllowOnlyReaders: Self = Self(1u32);
    pub const AllowReadersAndWriters: Self = Self(2u32);
}
impl ::core::marker::Copy for StorageOpenOptions {}
impl ::core::clone::Clone for StorageOpenOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StorageOpenOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StorageOpenOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for StorageOpenOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageOpenOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for StorageOpenOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for StorageOpenOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for StorageOpenOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for StorageOpenOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for StorageOpenOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for StorageOpenOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.StorageOpenOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct StorageProvider(::windows::core::IUnknown);
impl StorageProvider {
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsPropertySupportedForPartialFileAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, propertycanonicalname: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IStorageProvider2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPropertySupportedForPartialFileAsync)(::core::mem::transmute_copy(this), propertycanonicalname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
impl ::core::clone::Clone for StorageProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageProvider {}
impl ::core::fmt::Debug for StorageProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.StorageProvider;{e705eed4-d478-47d6-ba46-1a8ebe114a20})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StorageProvider {
    type Vtable = IStorageProvider_Vtbl;
    const IID: ::windows::core::GUID = <IStorageProvider as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorageProvider {
    const NAME: &'static str = "Windows.Storage.StorageProvider";
}
impl ::core::convert::From<StorageProvider> for ::windows::core::IUnknown {
    fn from(value: StorageProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageProvider> for ::windows::core::IUnknown {
    fn from(value: &StorageProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorageProvider> for ::windows::core::IInspectable {
    fn from(value: StorageProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageProvider> for ::windows::core::IInspectable {
    fn from(value: &StorageProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct StorageStreamTransaction(::windows::core::IUnknown);
impl StorageStreamTransaction {
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Stream(&self) -> ::windows::core::Result<Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Stream)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Streams::IRandomAccessStream>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CommitAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CommitAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for StorageStreamTransaction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageStreamTransaction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageStreamTransaction {}
impl ::core::fmt::Debug for StorageStreamTransaction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageStreamTransaction").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageStreamTransaction {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.StorageStreamTransaction;{f67cf363-a53d-4d94-ae2c-67232d93acdd})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StorageStreamTransaction {
    type Vtable = IStorageStreamTransaction_Vtbl;
    const IID: ::windows::core::GUID = <IStorageStreamTransaction as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorageStreamTransaction {
    const NAME: &'static str = "Windows.Storage.StorageStreamTransaction";
}
impl ::core::convert::From<StorageStreamTransaction> for ::windows::core::IUnknown {
    fn from(value: StorageStreamTransaction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageStreamTransaction> for ::windows::core::IUnknown {
    fn from(value: &StorageStreamTransaction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageStreamTransaction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageStreamTransaction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorageStreamTransaction> for ::windows::core::IInspectable {
    fn from(value: StorageStreamTransaction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageStreamTransaction> for ::windows::core::IInspectable {
    fn from(value: &StorageStreamTransaction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageStreamTransaction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageStreamTransaction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<StorageStreamTransaction> for super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageStreamTransaction) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&StorageStreamTransaction> for super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageStreamTransaction) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::IClosable> for StorageStreamTransaction {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::IClosable> for &StorageStreamTransaction {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: `\"Storage\"`, `\"Storage_Streams\"`*"]
#[cfg(feature = "Storage_Streams")]
#[repr(transparent)]
pub struct StreamedFileDataRequest(::windows::core::IUnknown);
#[cfg(feature = "Storage_Streams")]
impl StreamedFileDataRequest {
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteAsync<'a, Param0: ::windows::core::IntoParam<'a, Streams::IBuffer>>(&self, buffer: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WriteAsync)(::core::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn FlushAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FlushAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn FailAndClose(&self, failuremode: StreamedFileFailureMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStreamedFileDataRequest>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).FailAndClose)(::core::mem::transmute_copy(this), failuremode).ok() }
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::clone::Clone for StreamedFileDataRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::cmp::PartialEq for StreamedFileDataRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::cmp::Eq for StreamedFileDataRequest {}
#[cfg(feature = "Storage_Streams")]
impl ::core::fmt::Debug for StreamedFileDataRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StreamedFileDataRequest").field(&self.0).finish()
    }
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::windows::core::RuntimeType for StreamedFileDataRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.StreamedFileDataRequest;{905a0fe6-bc53-11df-8c49-001e4fc686da})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::windows::core::Interface for StreamedFileDataRequest {
    type Vtable = Streams::IOutputStream_Vtbl;
    const IID: ::windows::core::GUID = <Streams::IOutputStream as ::windows::core::Interface>::IID;
}
#[cfg(feature = "Storage_Streams")]
impl ::windows::core::RuntimeName for StreamedFileDataRequest {
    const NAME: &'static str = "Windows.Storage.StreamedFileDataRequest";
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<StreamedFileDataRequest> for ::windows::core::IUnknown {
    fn from(value: StreamedFileDataRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<&StreamedFileDataRequest> for ::windows::core::IUnknown {
    fn from(value: &StreamedFileDataRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StreamedFileDataRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StreamedFileDataRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<StreamedFileDataRequest> for ::windows::core::IInspectable {
    fn from(value: StreamedFileDataRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<&StreamedFileDataRequest> for ::windows::core::IInspectable {
    fn from(value: &StreamedFileDataRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StreamedFileDataRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StreamedFileDataRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::core::convert::TryFrom<StreamedFileDataRequest> for super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: StreamedFileDataRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::core::convert::TryFrom<&StreamedFileDataRequest> for super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &StreamedFileDataRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::IClosable> for StreamedFileDataRequest {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::IClosable> for &StreamedFileDataRequest {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<StreamedFileDataRequest> for Streams::IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: StreamedFileDataRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&StreamedFileDataRequest> for Streams::IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &StreamedFileDataRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, Streams::IOutputStream> for StreamedFileDataRequest {
    fn into_param(self) -> ::windows::core::Param<'a, Streams::IOutputStream> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, Streams::IOutputStream> for &StreamedFileDataRequest {
    fn into_param(self) -> ::windows::core::Param<'a, Streams::IOutputStream> {
        ::core::convert::TryInto::<Streams::IOutputStream>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<StreamedFileDataRequest> for IStreamedFileDataRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: StreamedFileDataRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&StreamedFileDataRequest> for IStreamedFileDataRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &StreamedFileDataRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, IStreamedFileDataRequest> for StreamedFileDataRequest {
    fn into_param(self) -> ::windows::core::Param<'a, IStreamedFileDataRequest> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, IStreamedFileDataRequest> for &StreamedFileDataRequest {
    fn into_param(self) -> ::windows::core::Param<'a, IStreamedFileDataRequest> {
        ::core::convert::TryInto::<IStreamedFileDataRequest>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: `\"Storage\"`, `\"Storage_Streams\"`*"]
#[cfg(feature = "Storage_Streams")]
#[repr(transparent)]
pub struct StreamedFileDataRequestedHandler(pub ::windows::core::IUnknown);
#[cfg(feature = "Storage_Streams")]
impl StreamedFileDataRequestedHandler {
    pub fn new<F: FnMut(&::core::option::Option<StreamedFileDataRequest>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = StreamedFileDataRequestedHandlerBox::<F> { vtable: &StreamedFileDataRequestedHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, StreamedFileDataRequest>>(&self, stream: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this), stream.into_param().abi()).ok() }
    }
}
#[cfg(feature = "Storage_Streams")]
#[repr(C)]
struct StreamedFileDataRequestedHandlerBox<F: FnMut(&::core::option::Option<StreamedFileDataRequest>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const StreamedFileDataRequestedHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
#[cfg(feature = "Storage_Streams")]
impl<F: FnMut(&::core::option::Option<StreamedFileDataRequest>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> StreamedFileDataRequestedHandlerBox<F> {
    const VTABLE: StreamedFileDataRequestedHandler_Vtbl = StreamedFileDataRequestedHandler_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<StreamedFileDataRequestedHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&stream)).into()
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::clone::Clone for StreamedFileDataRequestedHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::cmp::PartialEq for StreamedFileDataRequestedHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::cmp::Eq for StreamedFileDataRequestedHandler {}
#[cfg(feature = "Storage_Streams")]
impl ::core::fmt::Debug for StreamedFileDataRequestedHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StreamedFileDataRequestedHandler").field(&self.0).finish()
    }
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::windows::core::Interface for StreamedFileDataRequestedHandler {
    type Vtable = StreamedFileDataRequestedHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfef6a824_2fe1_4d07_a35b_b77c50b5f4cc);
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::windows::core::RuntimeType for StreamedFileDataRequestedHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{fef6a824-2fe1-4d07-a35b-b77c50b5f4cc}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "Storage_Streams")]
#[repr(C)]
#[doc(hidden)]
pub struct StreamedFileDataRequestedHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Invoke: usize,
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StreamedFileFailureMode(pub i32);
impl StreamedFileFailureMode {
    pub const Failed: Self = Self(0i32);
    pub const CurrentlyUnavailable: Self = Self(1i32);
    pub const Incomplete: Self = Self(2i32);
}
impl ::core::marker::Copy for StreamedFileFailureMode {}
impl ::core::clone::Clone for StreamedFileFailureMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StreamedFileFailureMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StreamedFileFailureMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for StreamedFileFailureMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StreamedFileFailureMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StreamedFileFailureMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.StreamedFileFailureMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct SystemAudioProperties(::windows::core::IUnknown);
impl SystemAudioProperties {
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn EncodingBitrate(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EncodingBitrate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for SystemAudioProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemAudioProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemAudioProperties {}
impl ::core::fmt::Debug for SystemAudioProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemAudioProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SystemAudioProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.SystemAudioProperties;{3f8f38b7-308c-47e1-924d-8645348e5db7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SystemAudioProperties {
    type Vtable = ISystemAudioProperties_Vtbl;
    const IID: ::windows::core::GUID = <ISystemAudioProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SystemAudioProperties {
    const NAME: &'static str = "Windows.Storage.SystemAudioProperties";
}
impl ::core::convert::From<SystemAudioProperties> for ::windows::core::IUnknown {
    fn from(value: SystemAudioProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemAudioProperties> for ::windows::core::IUnknown {
    fn from(value: &SystemAudioProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemAudioProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemAudioProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemAudioProperties> for ::windows::core::IInspectable {
    fn from(value: SystemAudioProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemAudioProperties> for ::windows::core::IInspectable {
    fn from(value: &SystemAudioProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemAudioProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemAudioProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemAudioProperties {}
unsafe impl ::core::marker::Sync for SystemAudioProperties {}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct SystemDataPaths(::windows::core::IUnknown);
impl SystemDataPaths {
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Fonts(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Fonts)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn ProgramData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProgramData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Public(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Public)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn PublicDesktop(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PublicDesktop)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn PublicDocuments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PublicDocuments)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn PublicDownloads(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PublicDownloads)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn PublicMusic(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PublicMusic)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn PublicPictures(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PublicPictures)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn PublicVideos(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PublicVideos)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn System(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).System)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn SystemHost(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SystemHost)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn SystemX86(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SystemX86)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn SystemX64(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SystemX64)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn SystemArm(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SystemArm)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn UserProfiles(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UserProfiles)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Windows(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Windows)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn GetDefault() -> ::windows::core::Result<SystemDataPaths> {
        Self::ISystemDataPathsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemDataPaths>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISystemDataPathsStatics<R, F: FnOnce(&ISystemDataPathsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SystemDataPaths, ISystemDataPathsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SystemDataPaths {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemDataPaths {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemDataPaths {}
impl ::core::fmt::Debug for SystemDataPaths {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemDataPaths").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SystemDataPaths {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.SystemDataPaths;{e32abf70-d8fa-45ec-a942-d2e26fb60ba5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SystemDataPaths {
    type Vtable = ISystemDataPaths_Vtbl;
    const IID: ::windows::core::GUID = <ISystemDataPaths as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SystemDataPaths {
    const NAME: &'static str = "Windows.Storage.SystemDataPaths";
}
impl ::core::convert::From<SystemDataPaths> for ::windows::core::IUnknown {
    fn from(value: SystemDataPaths) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemDataPaths> for ::windows::core::IUnknown {
    fn from(value: &SystemDataPaths) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemDataPaths {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemDataPaths {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemDataPaths> for ::windows::core::IInspectable {
    fn from(value: SystemDataPaths) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemDataPaths> for ::windows::core::IInspectable {
    fn from(value: &SystemDataPaths) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemDataPaths {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemDataPaths {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemDataPaths {}
unsafe impl ::core::marker::Sync for SystemDataPaths {}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct SystemGPSProperties(::windows::core::IUnknown);
impl SystemGPSProperties {
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn LatitudeDecimal(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LatitudeDecimal)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn LongitudeDecimal(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LongitudeDecimal)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for SystemGPSProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemGPSProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemGPSProperties {}
impl ::core::fmt::Debug for SystemGPSProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemGPSProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SystemGPSProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.SystemGPSProperties;{c0f46eb4-c174-481a-bc25-921986f6a6f3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SystemGPSProperties {
    type Vtable = ISystemGPSProperties_Vtbl;
    const IID: ::windows::core::GUID = <ISystemGPSProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SystemGPSProperties {
    const NAME: &'static str = "Windows.Storage.SystemGPSProperties";
}
impl ::core::convert::From<SystemGPSProperties> for ::windows::core::IUnknown {
    fn from(value: SystemGPSProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemGPSProperties> for ::windows::core::IUnknown {
    fn from(value: &SystemGPSProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemGPSProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemGPSProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemGPSProperties> for ::windows::core::IInspectable {
    fn from(value: SystemGPSProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemGPSProperties> for ::windows::core::IInspectable {
    fn from(value: &SystemGPSProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemGPSProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemGPSProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemGPSProperties {}
unsafe impl ::core::marker::Sync for SystemGPSProperties {}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct SystemImageProperties(::windows::core::IUnknown);
impl SystemImageProperties {
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn HorizontalSize(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HorizontalSize)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn VerticalSize(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VerticalSize)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for SystemImageProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemImageProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemImageProperties {}
impl ::core::fmt::Debug for SystemImageProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemImageProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SystemImageProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.SystemImageProperties;{011b2e30-8b39-4308-bea1-e8aa61e47826})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SystemImageProperties {
    type Vtable = ISystemImageProperties_Vtbl;
    const IID: ::windows::core::GUID = <ISystemImageProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SystemImageProperties {
    const NAME: &'static str = "Windows.Storage.SystemImageProperties";
}
impl ::core::convert::From<SystemImageProperties> for ::windows::core::IUnknown {
    fn from(value: SystemImageProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemImageProperties> for ::windows::core::IUnknown {
    fn from(value: &SystemImageProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemImageProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemImageProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemImageProperties> for ::windows::core::IInspectable {
    fn from(value: SystemImageProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemImageProperties> for ::windows::core::IInspectable {
    fn from(value: &SystemImageProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemImageProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemImageProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemImageProperties {}
unsafe impl ::core::marker::Sync for SystemImageProperties {}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct SystemMediaProperties(::windows::core::IUnknown);
impl SystemMediaProperties {
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Duration(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Duration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Producer(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Producer)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Publisher(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Publisher)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn SubTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SubTitle)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Writer(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Writer)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Year(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Year)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for SystemMediaProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemMediaProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemMediaProperties {}
impl ::core::fmt::Debug for SystemMediaProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemMediaProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SystemMediaProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.SystemMediaProperties;{a42b3316-8415-40dc-8c44-98361d235430})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SystemMediaProperties {
    type Vtable = ISystemMediaProperties_Vtbl;
    const IID: ::windows::core::GUID = <ISystemMediaProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SystemMediaProperties {
    const NAME: &'static str = "Windows.Storage.SystemMediaProperties";
}
impl ::core::convert::From<SystemMediaProperties> for ::windows::core::IUnknown {
    fn from(value: SystemMediaProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemMediaProperties> for ::windows::core::IUnknown {
    fn from(value: &SystemMediaProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemMediaProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemMediaProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemMediaProperties> for ::windows::core::IInspectable {
    fn from(value: SystemMediaProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemMediaProperties> for ::windows::core::IInspectable {
    fn from(value: &SystemMediaProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemMediaProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemMediaProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemMediaProperties {}
unsafe impl ::core::marker::Sync for SystemMediaProperties {}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct SystemMusicProperties(::windows::core::IUnknown);
impl SystemMusicProperties {
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn AlbumArtist(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AlbumArtist)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn AlbumTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AlbumTitle)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Artist(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Artist)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Composer(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Composer)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Conductor(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Conductor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn DisplayArtist(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayArtist)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Genre(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Genre)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn TrackNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrackNumber)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for SystemMusicProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemMusicProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemMusicProperties {}
impl ::core::fmt::Debug for SystemMusicProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemMusicProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SystemMusicProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.SystemMusicProperties;{b47988d5-67af-4bc3-8d39-5b89022026a1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SystemMusicProperties {
    type Vtable = ISystemMusicProperties_Vtbl;
    const IID: ::windows::core::GUID = <ISystemMusicProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SystemMusicProperties {
    const NAME: &'static str = "Windows.Storage.SystemMusicProperties";
}
impl ::core::convert::From<SystemMusicProperties> for ::windows::core::IUnknown {
    fn from(value: SystemMusicProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemMusicProperties> for ::windows::core::IUnknown {
    fn from(value: &SystemMusicProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemMusicProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemMusicProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemMusicProperties> for ::windows::core::IInspectable {
    fn from(value: SystemMusicProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemMusicProperties> for ::windows::core::IInspectable {
    fn from(value: &SystemMusicProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemMusicProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemMusicProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemMusicProperties {}
unsafe impl ::core::marker::Sync for SystemMusicProperties {}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct SystemPhotoProperties(::windows::core::IUnknown);
impl SystemPhotoProperties {
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn CameraManufacturer(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CameraManufacturer)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn CameraModel(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CameraModel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn DateTaken(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DateTaken)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Orientation(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Orientation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn PeopleNames(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PeopleNames)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for SystemPhotoProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemPhotoProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemPhotoProperties {}
impl ::core::fmt::Debug for SystemPhotoProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemPhotoProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SystemPhotoProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.SystemPhotoProperties;{4734fc3d-ab21-4424-b735-f4353a56c8fc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SystemPhotoProperties {
    type Vtable = ISystemPhotoProperties_Vtbl;
    const IID: ::windows::core::GUID = <ISystemPhotoProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SystemPhotoProperties {
    const NAME: &'static str = "Windows.Storage.SystemPhotoProperties";
}
impl ::core::convert::From<SystemPhotoProperties> for ::windows::core::IUnknown {
    fn from(value: SystemPhotoProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemPhotoProperties> for ::windows::core::IUnknown {
    fn from(value: &SystemPhotoProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemPhotoProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemPhotoProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemPhotoProperties> for ::windows::core::IInspectable {
    fn from(value: SystemPhotoProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemPhotoProperties> for ::windows::core::IInspectable {
    fn from(value: &SystemPhotoProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemPhotoProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemPhotoProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemPhotoProperties {}
unsafe impl ::core::marker::Sync for SystemPhotoProperties {}
#[doc = "*Required features: `\"Storage\"`*"]
pub struct SystemProperties {}
impl SystemProperties {
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Author() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISystemProperties(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Author)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Comment() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISystemProperties(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Comment)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn ItemNameDisplay() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISystemProperties(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ItemNameDisplay)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Keywords() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISystemProperties(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Keywords)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Rating() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISystemProperties(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Rating)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Title() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISystemProperties(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Title)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Audio() -> ::windows::core::Result<SystemAudioProperties> {
        Self::ISystemProperties(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Audio)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemAudioProperties>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn GPS() -> ::windows::core::Result<SystemGPSProperties> {
        Self::ISystemProperties(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GPS)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemGPSProperties>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Media() -> ::windows::core::Result<SystemMediaProperties> {
        Self::ISystemProperties(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Media)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemMediaProperties>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Music() -> ::windows::core::Result<SystemMusicProperties> {
        Self::ISystemProperties(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Music)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemMusicProperties>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Photo() -> ::windows::core::Result<SystemPhotoProperties> {
        Self::ISystemProperties(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Photo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemPhotoProperties>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Video() -> ::windows::core::Result<SystemVideoProperties> {
        Self::ISystemProperties(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Video)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemVideoProperties>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Image() -> ::windows::core::Result<SystemImageProperties> {
        Self::ISystemProperties(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Image)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemImageProperties>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISystemProperties<R, F: FnOnce(&ISystemProperties) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SystemProperties, ISystemProperties> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for SystemProperties {
    const NAME: &'static str = "Windows.Storage.SystemProperties";
}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct SystemVideoProperties(::windows::core::IUnknown);
impl SystemVideoProperties {
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Director(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Director)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn FrameHeight(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FrameHeight)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn FrameWidth(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FrameWidth)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Orientation(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Orientation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn TotalBitrate(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TotalBitrate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for SystemVideoProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemVideoProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemVideoProperties {}
impl ::core::fmt::Debug for SystemVideoProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemVideoProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SystemVideoProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.SystemVideoProperties;{2040f715-67f8-4322-9b80-4fa9fefb83e8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SystemVideoProperties {
    type Vtable = ISystemVideoProperties_Vtbl;
    const IID: ::windows::core::GUID = <ISystemVideoProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SystemVideoProperties {
    const NAME: &'static str = "Windows.Storage.SystemVideoProperties";
}
impl ::core::convert::From<SystemVideoProperties> for ::windows::core::IUnknown {
    fn from(value: SystemVideoProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemVideoProperties> for ::windows::core::IUnknown {
    fn from(value: &SystemVideoProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemVideoProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemVideoProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemVideoProperties> for ::windows::core::IInspectable {
    fn from(value: SystemVideoProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemVideoProperties> for ::windows::core::IInspectable {
    fn from(value: &SystemVideoProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemVideoProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemVideoProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemVideoProperties {}
unsafe impl ::core::marker::Sync for SystemVideoProperties {}
#[doc = "*Required features: `\"Storage\"`*"]
#[repr(transparent)]
pub struct UserDataPaths(::windows::core::IUnknown);
impl UserDataPaths {
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn CameraRoll(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CameraRoll)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Cookies(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Cookies)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Desktop(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Desktop)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Documents(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Documents)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Downloads(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Downloads)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Favorites(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Favorites)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn History(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).History)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn InternetCache(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InternetCache)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn LocalAppData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LocalAppData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn LocalAppDataLow(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LocalAppDataLow)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Music(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Music)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Pictures(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Pictures)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Profile(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Profile)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Recent(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Recent)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn RoamingAppData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RoamingAppData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn SavedPictures(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SavedPictures)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Screenshots(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Screenshots)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Templates(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Templates)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn Videos(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Videos)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetForUser<'a, Param0: ::windows::core::IntoParam<'a, super::System::User>>(user: Param0) -> ::windows::core::Result<UserDataPaths> {
        Self::IUserDataPathsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForUser)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<UserDataPaths>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    pub fn GetDefault() -> ::windows::core::Result<UserDataPaths> {
        Self::IUserDataPathsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataPaths>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUserDataPathsStatics<R, F: FnOnce(&IUserDataPathsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UserDataPaths, IUserDataPathsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for UserDataPaths {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataPaths {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataPaths {}
impl ::core::fmt::Debug for UserDataPaths {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataPaths").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataPaths {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.UserDataPaths;{f9c53912-abc4-46ff-8a2b-dc9d7fa6e52f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UserDataPaths {
    type Vtable = IUserDataPaths_Vtbl;
    const IID: ::windows::core::GUID = <IUserDataPaths as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDataPaths {
    const NAME: &'static str = "Windows.Storage.UserDataPaths";
}
impl ::core::convert::From<UserDataPaths> for ::windows::core::IUnknown {
    fn from(value: UserDataPaths) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataPaths> for ::windows::core::IUnknown {
    fn from(value: &UserDataPaths) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataPaths {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDataPaths {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataPaths> for ::windows::core::IInspectable {
    fn from(value: UserDataPaths) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataPaths> for ::windows::core::IInspectable {
    fn from(value: &UserDataPaths) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataPaths {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDataPaths {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataPaths {}
unsafe impl ::core::marker::Sync for UserDataPaths {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
