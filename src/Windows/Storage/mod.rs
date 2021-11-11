#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
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
#[doc = "*Required features: `Storage`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppDataPaths(pub ::windows::core::IInspectable);
impl AppDataPaths {
    #[doc = "*Required features: `Storage`*"]
    pub fn Cookies(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Desktop(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Documents(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Favorites(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn History(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn InternetCache(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn LocalAppData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn ProgramData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn RoamingAppData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Storage`, `System`*"]
    pub fn GetForUser<'a, Param0: ::windows::core::IntoParam<'a, super::System::User>>(user: Param0) -> ::windows::core::Result<AppDataPaths> {
        Self::IAppDataPathsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<AppDataPaths>(result__)
        })
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn GetDefault() -> ::windows::core::Result<AppDataPaths> {
        Self::IAppDataPathsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppDataPaths>(result__)
        })
    }
    pub fn IAppDataPathsStatics<R, F: FnOnce(&IAppDataPathsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppDataPaths, IAppDataPathsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AppDataPaths {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.AppDataPaths;{7301d60a-79a2-48c9-9ec0-3fda092f79e1})");
}
unsafe impl ::windows::core::Interface for AppDataPaths {
    type Vtable = IAppDataPaths_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7301d60a_79a2_48c9_9ec0_3fda092f79e1);
}
impl ::windows::core::RuntimeName for AppDataPaths {
    const NAME: &'static str = "Windows.Storage.AppDataPaths";
}
impl ::core::convert::From<AppDataPaths> for ::windows::core::IUnknown {
    fn from(value: AppDataPaths) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppDataPaths> for ::windows::core::IUnknown {
    fn from(value: &AppDataPaths) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppDataPaths {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppDataPaths {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppDataPaths> for ::windows::core::IInspectable {
    fn from(value: AppDataPaths) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppDataPaths> for ::windows::core::IInspectable {
    fn from(value: &AppDataPaths) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppDataPaths {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppDataPaths {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppDataPaths {}
unsafe impl ::core::marker::Sync for AppDataPaths {}
#[doc = "*Required features: `Storage`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ApplicationData(pub ::windows::core::IInspectable);
impl ApplicationData {
    #[doc = "*Required features: `Storage`*"]
    pub fn Version(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn SetVersionAsync<'a, Param1: ::windows::core::IntoParam<'a, ApplicationDataSetVersionHandler>>(&self, desiredversion: u32, handler: Param1) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), desiredversion, handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn ClearAllAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn ClearAsync(&self, locality: ApplicationDataLocality) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), locality, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn LocalSettings(&self) -> ::windows::core::Result<ApplicationDataContainer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationDataContainer>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn RoamingSettings(&self) -> ::windows::core::Result<ApplicationDataContainer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationDataContainer>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn LocalFolder(&self) -> ::windows::core::Result<StorageFolder> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn RoamingFolder(&self) -> ::windows::core::Result<StorageFolder> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn TemporaryFolder(&self) -> ::windows::core::Result<StorageFolder> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn DataChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<ApplicationData, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn RemoveDataChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn SignalDataChanged(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn RoamingStorageQuota(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn LocalCacheFolder(&self) -> ::windows::core::Result<StorageFolder> {
        let this = &::windows::core::Interface::cast::<IApplicationData2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn GetPublisherCacheFolder<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, foldername: Param0) -> ::windows::core::Result<StorageFolder> {
        let this = &::windows::core::Interface::cast::<IApplicationData3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), foldername.into_param().abi(), &mut result__).from_abi::<StorageFolder>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn ClearPublisherCacheFolderAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, foldername: Param0) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IApplicationData3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), foldername.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn SharedLocalFolder(&self) -> ::windows::core::Result<StorageFolder> {
        let this = &::windows::core::Interface::cast::<IApplicationData3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Current() -> ::windows::core::Result<ApplicationData> {
        Self::IApplicationDataStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationData>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `System`*"]
    pub fn GetForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::System::User>>(user: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<ApplicationData>> {
        Self::IApplicationDataStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<ApplicationData>>(result__)
        })
    }
    pub fn IApplicationDataStatics<R, F: FnOnce(&IApplicationDataStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ApplicationData, IApplicationDataStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IApplicationDataStatics2<R, F: FnOnce(&IApplicationDataStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ApplicationData, IApplicationDataStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ApplicationData {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.ApplicationData;{c3da6fb7-b744-4b45-b0b8-223a0938d0dc})");
}
unsafe impl ::windows::core::Interface for ApplicationData {
    type Vtable = IApplicationData_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3da6fb7_b744_4b45_b0b8_223a0938d0dc);
}
impl ::windows::core::RuntimeName for ApplicationData {
    const NAME: &'static str = "Windows.Storage.ApplicationData";
}
impl ::core::convert::From<ApplicationData> for ::windows::core::IUnknown {
    fn from(value: ApplicationData) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ApplicationData> for ::windows::core::IUnknown {
    fn from(value: &ApplicationData) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ApplicationData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ApplicationData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ApplicationData> for ::windows::core::IInspectable {
    fn from(value: ApplicationData) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ApplicationData> for ::windows::core::IInspectable {
    fn from(value: &ApplicationData) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ApplicationData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ApplicationData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
#[cfg(feature = "Foundation_Collections")]
#[doc = "*Required features: `Storage`, `Foundation_Collections`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ApplicationDataCompositeValue(pub ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ApplicationDataCompositeValue {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ApplicationDataCompositeValue, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::core::Result<super::Foundation::Collections::IIterator<super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IIterable<super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IIterator<super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage`, `Foundation_Collections`*"]
    pub fn Lookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage`, `Foundation_Collections`*"]
    pub fn HasKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage`, `Foundation_Collections`*"]
    pub fn GetView(&self) -> ::windows::core::Result<super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage`, `Foundation_Collections`*"]
    pub fn Insert<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, key: Param0, value: Param1) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), key.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage`, `Foundation_Collections`*"]
    pub fn Remove<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), key.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage`, `Foundation_Collections`*"]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Foundation_Collections`*"]
    pub fn MapChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Collections::MapChangedEventHandler<::windows::core::HSTRING, ::windows::core::IInspectable>>>(&self, vhnd: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), vhnd.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Foundation_Collections`*"]
    pub fn RemoveMapChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for ApplicationDataCompositeValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.ApplicationDataCompositeValue;{8a43ed9f-f4e6-4421-acf9-1dab2986820c})");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for ApplicationDataCompositeValue {
    type Vtable = super::Foundation::Collections::IPropertySet_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a43ed9f_f4e6_4421_acf9_1dab2986820c);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for ApplicationDataCompositeValue {
    const NAME: &'static str = "Windows.Storage.ApplicationDataCompositeValue";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ApplicationDataCompositeValue> for ::windows::core::IUnknown {
    fn from(value: ApplicationDataCompositeValue) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ApplicationDataCompositeValue> for ::windows::core::IUnknown {
    fn from(value: &ApplicationDataCompositeValue) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ApplicationDataCompositeValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ApplicationDataCompositeValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ApplicationDataCompositeValue> for ::windows::core::IInspectable {
    fn from(value: ApplicationDataCompositeValue) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ApplicationDataCompositeValue> for ::windows::core::IInspectable {
    fn from(value: &ApplicationDataCompositeValue) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ApplicationDataCompositeValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ApplicationDataCompositeValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ApplicationDataCompositeValue> for super::Foundation::Collections::IPropertySet {
    fn from(value: ApplicationDataCompositeValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ApplicationDataCompositeValue> for super::Foundation::Collections::IPropertySet {
    fn from(value: &ApplicationDataCompositeValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::Collections::IPropertySet> for ApplicationDataCompositeValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::Collections::IPropertySet> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::Collections::IPropertySet> for &ApplicationDataCompositeValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::Collections::IPropertySet> {
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
unsafe impl ::core::marker::Send for ApplicationDataCompositeValue {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for ApplicationDataCompositeValue {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for ApplicationDataCompositeValue {
    type Item = super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>;
    type IntoIter = super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &ApplicationDataCompositeValue {
    type Item = super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>;
    type IntoIter = super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[doc = "*Required features: `Storage`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ApplicationDataContainer(pub ::windows::core::IInspectable);
impl ApplicationDataContainer {
    #[doc = "*Required features: `Storage`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Locality(&self) -> ::windows::core::Result<ApplicationDataLocality> {
        let this = self;
        unsafe {
            let mut result__: ApplicationDataLocality = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationDataLocality>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage`, `Foundation_Collections`*"]
    pub fn Values(&self) -> ::windows::core::Result<super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage`, `Foundation_Collections`*"]
    pub fn Containers(&self) -> ::windows::core::Result<super::Foundation::Collections::IMapView<::windows::core::HSTRING, ApplicationDataContainer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IMapView<::windows::core::HSTRING, ApplicationDataContainer>>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn CreateContainer<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, disposition: ApplicationDataCreateDisposition) -> ::windows::core::Result<ApplicationDataContainer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), name.into_param().abi(), disposition, &mut result__).from_abi::<ApplicationDataContainer>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn DeleteContainer<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), name.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ApplicationDataContainer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.ApplicationDataContainer;{c5aefd1e-f467-40ba-8566-ab640a441e1d})");
}
unsafe impl ::windows::core::Interface for ApplicationDataContainer {
    type Vtable = IApplicationDataContainer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5aefd1e_f467_40ba_8566_ab640a441e1d);
}
impl ::windows::core::RuntimeName for ApplicationDataContainer {
    const NAME: &'static str = "Windows.Storage.ApplicationDataContainer";
}
impl ::core::convert::From<ApplicationDataContainer> for ::windows::core::IUnknown {
    fn from(value: ApplicationDataContainer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ApplicationDataContainer> for ::windows::core::IUnknown {
    fn from(value: &ApplicationDataContainer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ApplicationDataContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ApplicationDataContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ApplicationDataContainer> for ::windows::core::IInspectable {
    fn from(value: ApplicationDataContainer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ApplicationDataContainer> for ::windows::core::IInspectable {
    fn from(value: &ApplicationDataContainer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ApplicationDataContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ApplicationDataContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
#[cfg(feature = "Foundation_Collections")]
#[doc = "*Required features: `Storage`, `Foundation_Collections`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ApplicationDataContainerSettings(pub ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ApplicationDataContainerSettings {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::core::Result<super::Foundation::Collections::IIterator<super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IIterable<super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IIterator<super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage`, `Foundation_Collections`*"]
    pub fn Lookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage`, `Foundation_Collections`*"]
    pub fn HasKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage`, `Foundation_Collections`*"]
    pub fn GetView(&self) -> ::windows::core::Result<super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage`, `Foundation_Collections`*"]
    pub fn Insert<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, key: Param0, value: Param1) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), key.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage`, `Foundation_Collections`*"]
    pub fn Remove<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), key.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage`, `Foundation_Collections`*"]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Foundation_Collections`*"]
    pub fn MapChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Collections::MapChangedEventHandler<::windows::core::HSTRING, ::windows::core::IInspectable>>>(&self, vhnd: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), vhnd.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Foundation_Collections`*"]
    pub fn RemoveMapChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for ApplicationDataContainerSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.ApplicationDataContainerSettings;{8a43ed9f-f4e6-4421-acf9-1dab2986820c})");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for ApplicationDataContainerSettings {
    type Vtable = super::Foundation::Collections::IPropertySet_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a43ed9f_f4e6_4421_acf9_1dab2986820c);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for ApplicationDataContainerSettings {
    const NAME: &'static str = "Windows.Storage.ApplicationDataContainerSettings";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ApplicationDataContainerSettings> for ::windows::core::IUnknown {
    fn from(value: ApplicationDataContainerSettings) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ApplicationDataContainerSettings> for ::windows::core::IUnknown {
    fn from(value: &ApplicationDataContainerSettings) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ApplicationDataContainerSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ApplicationDataContainerSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ApplicationDataContainerSettings> for ::windows::core::IInspectable {
    fn from(value: ApplicationDataContainerSettings) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ApplicationDataContainerSettings> for ::windows::core::IInspectable {
    fn from(value: &ApplicationDataContainerSettings) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ApplicationDataContainerSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ApplicationDataContainerSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ApplicationDataContainerSettings> for super::Foundation::Collections::IPropertySet {
    fn from(value: ApplicationDataContainerSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ApplicationDataContainerSettings> for super::Foundation::Collections::IPropertySet {
    fn from(value: &ApplicationDataContainerSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::Collections::IPropertySet> for ApplicationDataContainerSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::Collections::IPropertySet> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::Collections::IPropertySet> for &ApplicationDataContainerSettings {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::Collections::IPropertySet> {
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
unsafe impl ::core::marker::Send for ApplicationDataContainerSettings {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for ApplicationDataContainerSettings {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for ApplicationDataContainerSettings {
    type Item = super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>;
    type IntoIter = super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &ApplicationDataContainerSettings {
    type Item = super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>;
    type IntoIter = super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[doc = "*Required features: `Storage`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ApplicationDataCreateDisposition(pub i32);
impl ApplicationDataCreateDisposition {
    pub const Always: ApplicationDataCreateDisposition = ApplicationDataCreateDisposition(0i32);
    pub const Existing: ApplicationDataCreateDisposition = ApplicationDataCreateDisposition(1i32);
}
impl ::core::convert::From<i32> for ApplicationDataCreateDisposition {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ApplicationDataCreateDisposition {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ApplicationDataCreateDisposition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.ApplicationDataCreateDisposition;i4)");
}
impl ::windows::core::DefaultType for ApplicationDataCreateDisposition {
    type DefaultType = Self;
}
#[doc = "*Required features: `Storage`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ApplicationDataLocality(pub i32);
impl ApplicationDataLocality {
    pub const Local: ApplicationDataLocality = ApplicationDataLocality(0i32);
    pub const Roaming: ApplicationDataLocality = ApplicationDataLocality(1i32);
    pub const Temporary: ApplicationDataLocality = ApplicationDataLocality(2i32);
    pub const LocalCache: ApplicationDataLocality = ApplicationDataLocality(3i32);
    pub const SharedLocal: ApplicationDataLocality = ApplicationDataLocality(4i32);
}
impl ::core::convert::From<i32> for ApplicationDataLocality {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ApplicationDataLocality {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ApplicationDataLocality {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.ApplicationDataLocality;i4)");
}
impl ::windows::core::DefaultType for ApplicationDataLocality {
    type DefaultType = Self;
}
#[doc = "*Required features: `Storage`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ApplicationDataSetVersionHandler(::windows::core::IUnknown);
impl ApplicationDataSetVersionHandler {
    pub fn new<F: FnMut(&::core::option::Option<SetVersionRequest>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = ApplicationDataSetVersionHandler_box::<F> {
            vtable: &ApplicationDataSetVersionHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, SetVersionRequest>>(&self, setversionrequest: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), setversionrequest.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ApplicationDataSetVersionHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({a05791e6-cc9f-4687-acab-a364fd785463})");
}
unsafe impl ::windows::core::Interface for ApplicationDataSetVersionHandler {
    type Vtable = ApplicationDataSetVersionHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa05791e6_cc9f_4687_acab_a364fd785463);
}
#[repr(C)]
#[doc(hidden)]
pub struct ApplicationDataSetVersionHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, setversionrequest: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
struct ApplicationDataSetVersionHandler_box<F: FnMut(&::core::option::Option<SetVersionRequest>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const ApplicationDataSetVersionHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<SetVersionRequest>) -> ::windows::core::Result<()> + 'static> ApplicationDataSetVersionHandler_box<F> {
    const VTABLE: ApplicationDataSetVersionHandler_abi = ApplicationDataSetVersionHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<ApplicationDataSetVersionHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(0x8000_4002)
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
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, setversionrequest: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&setversionrequest as *const <SetVersionRequest as ::windows::core::Abi>::Abi as *const <SetVersionRequest as ::windows::core::DefaultType>::DefaultType)).into()
    }
}
#[doc = "*Required features: `Storage`*"]
pub struct CachedFileManager {}
impl CachedFileManager {
    #[doc = "*Required features: `Storage`*"]
    pub fn DeferUpdates<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>>(file: Param0) -> ::windows::core::Result<()> {
        Self::ICachedFileManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), file.into_param().abi()).ok() })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Provider"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_Provider`*"]
    pub fn CompleteUpdatesAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>>(file: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Provider::FileUpdateStatus>> {
        Self::ICachedFileManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<Provider::FileUpdateStatus>>(result__)
        })
    }
    pub fn ICachedFileManagerStatics<R, F: FnOnce(&ICachedFileManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CachedFileManager, ICachedFileManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for CachedFileManager {
    const NAME: &'static str = "Windows.Storage.CachedFileManager";
}
#[doc = "*Required features: `Storage`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CreationCollisionOption(pub i32);
impl CreationCollisionOption {
    pub const GenerateUniqueName: CreationCollisionOption = CreationCollisionOption(0i32);
    pub const ReplaceExisting: CreationCollisionOption = CreationCollisionOption(1i32);
    pub const FailIfExists: CreationCollisionOption = CreationCollisionOption(2i32);
    pub const OpenIfExists: CreationCollisionOption = CreationCollisionOption(3i32);
}
impl ::core::convert::From<i32> for CreationCollisionOption {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CreationCollisionOption {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CreationCollisionOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.CreationCollisionOption;i4)");
}
impl ::windows::core::DefaultType for CreationCollisionOption {
    type DefaultType = Self;
}
#[doc = "*Required features: `Storage`*"]
pub struct DownloadsFolder {}
impl DownloadsFolder {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn CreateFileAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(desiredname: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        Self::IDownloadsFolderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn CreateFolderAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(desiredname: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
        Self::IDownloadsFolderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn CreateFileWithCollisionOptionAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(desiredname: Param0, option: CreationCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        Self::IDownloadsFolderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), option, &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn CreateFolderWithCollisionOptionAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(desiredname: Param0, option: CreationCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
        Self::IDownloadsFolderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), option, &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `System`*"]
    pub fn CreateFileForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, desiredname: Param1) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        Self::IDownloadsFolderStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), user.into_param().abi(), desiredname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `System`*"]
    pub fn CreateFolderForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, desiredname: Param1) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
        Self::IDownloadsFolderStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), user.into_param().abi(), desiredname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `System`*"]
    pub fn CreateFileForUserWithCollisionOptionAsync<'a, Param0: ::windows::core::IntoParam<'a, super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, desiredname: Param1, option: CreationCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        Self::IDownloadsFolderStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), user.into_param().abi(), desiredname.into_param().abi(), option, &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `System`*"]
    pub fn CreateFolderForUserWithCollisionOptionAsync<'a, Param0: ::windows::core::IntoParam<'a, super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, desiredname: Param1, option: CreationCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
        Self::IDownloadsFolderStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), user.into_param().abi(), desiredname.into_param().abi(), option, &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
        })
    }
    pub fn IDownloadsFolderStatics<R, F: FnOnce(&IDownloadsFolderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DownloadsFolder, IDownloadsFolderStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IDownloadsFolderStatics2<R, F: FnOnce(&IDownloadsFolderStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DownloadsFolder, IDownloadsFolderStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for DownloadsFolder {
    const NAME: &'static str = "Windows.Storage.DownloadsFolder";
}
#[doc = "*Required features: `Storage`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct FileAccessMode(pub i32);
impl FileAccessMode {
    pub const Read: FileAccessMode = FileAccessMode(0i32);
    pub const ReadWrite: FileAccessMode = FileAccessMode(1i32);
}
impl ::core::convert::From<i32> for FileAccessMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for FileAccessMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for FileAccessMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.FileAccessMode;i4)");
}
impl ::windows::core::DefaultType for FileAccessMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `Storage`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct FileAttributes(pub u32);
impl FileAttributes {
    pub const Normal: FileAttributes = FileAttributes(0u32);
    pub const ReadOnly: FileAttributes = FileAttributes(1u32);
    pub const Directory: FileAttributes = FileAttributes(16u32);
    pub const Archive: FileAttributes = FileAttributes(32u32);
    pub const Temporary: FileAttributes = FileAttributes(256u32);
    pub const LocallyIncomplete: FileAttributes = FileAttributes(512u32);
}
impl ::core::convert::From<u32> for FileAttributes {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for FileAttributes {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for FileAttributes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.FileAttributes;u4)");
}
impl ::windows::core::DefaultType for FileAttributes {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for FileAttributes {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for FileAttributes {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for FileAttributes {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for FileAttributes {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for FileAttributes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Storage`*"]
pub struct FileIO {}
impl FileIO {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn ReadTextAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>>(file: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IFileIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_Streams`*"]
    pub fn ReadTextWithEncodingAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>>(file: Param0, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IFileIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), file.into_param().abi(), encoding, &mut result__).from_abi::<super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn WriteTextAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(file: Param0, contents: Param1) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IFileIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), file.into_param().abi(), contents.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_Streams`*"]
    pub fn WriteTextWithEncodingAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(file: Param0, contents: Param1, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IFileIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), file.into_param().abi(), contents.into_param().abi(), encoding, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn AppendTextAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(file: Param0, contents: Param1) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IFileIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), file.into_param().abi(), contents.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_Streams`*"]
    pub fn AppendTextWithEncodingAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(file: Param0, contents: Param1, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IFileIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), file.into_param().abi(), contents.into_param().abi(), encoding, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Foundation_Collections`*"]
    pub fn ReadLinesAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>>(file: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<::windows::core::HSTRING>>> {
        Self::IFileIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<::windows::core::HSTRING>>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Foundation_Collections`, `Storage_Streams`*"]
    pub fn ReadLinesWithEncodingAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>>(file: Param0, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<::windows::core::HSTRING>>> {
        Self::IFileIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), file.into_param().abi(), encoding, &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<::windows::core::HSTRING>>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Foundation_Collections`*"]
    pub fn WriteLinesAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>, Param1: ::windows::core::IntoParam<'a, super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(file: Param0, lines: Param1) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IFileIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), file.into_param().abi(), lines.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Foundation_Collections`, `Storage_Streams`*"]
    pub fn WriteLinesWithEncodingAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>, Param1: ::windows::core::IntoParam<'a, super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(file: Param0, lines: Param1, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IFileIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), file.into_param().abi(), lines.into_param().abi(), encoding, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Foundation_Collections`*"]
    pub fn AppendLinesAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>, Param1: ::windows::core::IntoParam<'a, super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(file: Param0, lines: Param1) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IFileIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), file.into_param().abi(), lines.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Foundation_Collections`, `Storage_Streams`*"]
    pub fn AppendLinesWithEncodingAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>, Param1: ::windows::core::IntoParam<'a, super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(file: Param0, lines: Param1, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IFileIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), file.into_param().abi(), lines.into_param().abi(), encoding, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_Streams`*"]
    pub fn ReadBufferAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>>(file: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Streams::IBuffer>> {
        Self::IFileIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<Streams::IBuffer>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_Streams`*"]
    pub fn WriteBufferAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>, Param1: ::windows::core::IntoParam<'a, Streams::IBuffer>>(file: Param0, buffer: Param1) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IFileIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), file.into_param().abi(), buffer.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn WriteBytesAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>>(file: Param0, buffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IFileIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), file.into_param().abi(), buffer.len() as u32, ::core::mem::transmute(buffer.as_ptr()), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    pub fn IFileIOStatics<R, F: FnOnce(&IFileIOStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FileIO, IFileIOStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for FileIO {
    const NAME: &'static str = "Windows.Storage.FileIO";
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppDataPaths(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppDataPaths {
    type Vtable = IAppDataPaths_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7301d60a_79a2_48c9_9ec0_3fda092f79e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDataPaths_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppDataPathsStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppDataPathsStatics {
    type Vtable = IAppDataPathsStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8eb2afe_a9d9_4b14_b999_e3921379d903);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDataPathsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IApplicationData(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IApplicationData {
    type Vtable = IApplicationData_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3da6fb7_b744_4b45_b0b8_223a0938d0dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationData_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, desiredversion: u32, handler: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, locality: ApplicationDataLocality, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IApplicationData2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IApplicationData2 {
    type Vtable = IApplicationData2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e65cd69_0ba3_4e32_be29_b02de6607638);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationData2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IApplicationData3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IApplicationData3 {
    type Vtable = IApplicationData3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc222cf4_2772_4c1d_aa2c_c9f743ade8d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationData3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, foldername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, foldername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IApplicationDataContainer(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IApplicationDataContainer {
    type Vtable = IApplicationDataContainer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5aefd1e_f467_40ba_8566_ab640a441e1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationDataContainer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ApplicationDataLocality) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, disposition: ApplicationDataCreateDisposition, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IApplicationDataStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IApplicationDataStatics {
    type Vtable = IApplicationDataStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5612147b_e843_45e3_94d8_06169e3c8e17);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationDataStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IApplicationDataStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IApplicationDataStatics2 {
    type Vtable = IApplicationDataStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd606211_cf49_40a4_a47c_c7f0dbba8107);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationDataStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICachedFileManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICachedFileManagerStatics {
    type Vtable = ICachedFileManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ffc224a_e782_495d_b614_654c4f0b2370);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICachedFileManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Provider"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Provider")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDownloadsFolderStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDownloadsFolderStatics {
    type Vtable = IDownloadsFolderStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27862ed0_404e_47df_a1e2_e37308be7b37);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadsFolderStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, option: CreationCollisionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, option: CreationCollisionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDownloadsFolderStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDownloadsFolderStatics2 {
    type Vtable = IDownloadsFolderStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe93045bd_8ef8_4f8e_8d15_ac0e265f390d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadsFolderStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, option: CreationCollisionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, option: CreationCollisionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFileIOStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFileIOStatics {
    type Vtable = IFileIOStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x887411eb_7f54_4732_a5f0_5e43e3b8c2f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileIOStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, encoding: Streams::UnicodeEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, contents: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, contents: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, encoding: Streams::UnicodeEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, contents: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, contents: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, encoding: Streams::UnicodeEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, encoding: Streams::UnicodeEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, lines: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, lines: ::windows::core::RawPtr, encoding: Streams::UnicodeEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, lines: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, lines: ::windows::core::RawPtr, encoding: Streams::UnicodeEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, buffer_array_size: u32, buffer: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKnownFoldersCameraRollStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKnownFoldersCameraRollStatics {
    type Vtable = IKnownFoldersCameraRollStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d115e66_27e8_492f_b8e5_2f90896cd4cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownFoldersCameraRollStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKnownFoldersPlaylistsStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKnownFoldersPlaylistsStatics {
    type Vtable = IKnownFoldersPlaylistsStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdad5ecd6_306f_4d6a_b496_46ba8eb106ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownFoldersPlaylistsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKnownFoldersSavedPicturesStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKnownFoldersSavedPicturesStatics {
    type Vtable = IKnownFoldersSavedPicturesStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x055c93ea_253d_467c_b6ca_a97da1e9a18d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownFoldersSavedPicturesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKnownFoldersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKnownFoldersStatics {
    type Vtable = IKnownFoldersStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a2a7520_4802_452d_9ad9_4351ada7ec35);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownFoldersStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKnownFoldersStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKnownFoldersStatics2 {
    type Vtable = IKnownFoldersStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x194bd0cd_cf6e_4d07_9d53_e9163a2536e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownFoldersStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKnownFoldersStatics3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKnownFoldersStatics3 {
    type Vtable = IKnownFoldersStatics3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5194341_9742_4ed5_823d_fc1401148764);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownFoldersStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, folderid: KnownFolderId, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKnownFoldersStatics4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IKnownFoldersStatics4 {
    type Vtable = IKnownFoldersStatics4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1722e6bf_9ff9_4b21_bed5_90ecb13a192e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownFoldersStatics4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, folderid: KnownFolderId, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, folderid: KnownFolderId, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, folderid: KnownFolderId, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPathIOStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPathIOStatics {
    type Vtable = IPathIOStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f2f3758_8ec7_4381_922b_8f6c07d288f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathIOStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, encoding: Streams::UnicodeEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contents: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contents: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, encoding: Streams::UnicodeEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contents: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contents: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, encoding: Streams::UnicodeEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, encoding: Streams::UnicodeEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, lines: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, lines: ::windows::core::RawPtr, encoding: Streams::UnicodeEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, lines: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, lines: ::windows::core::RawPtr, encoding: Streams::UnicodeEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, buffer_array_size: u32, buffer: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISetVersionDeferral(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISetVersionDeferral {
    type Vtable = ISetVersionDeferral_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x033508a2_781a_437a_b078_3f32badcfe47);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISetVersionDeferral_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISetVersionRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISetVersionRequest {
    type Vtable = ISetVersionRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9c76b9b_1056_4e69_8330_162619956f9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISetVersionRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Storage`*"]
pub struct IStorageFile(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageFile {
    type Vtable = IStorageFile_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa3f6186_4214_428c_a64c_14c9ac7315ea);
}
impl IStorageFile {
    #[doc = "*Required features: `Storage`*"]
    pub fn FileType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_Streams`*"]
    pub fn OpenAsync(&self, accessmode: FileAccessMode) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Streams::IRandomAccessStream>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), accessmode, &mut result__).from_abi::<super::Foundation::IAsyncOperation<Streams::IRandomAccessStream>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn OpenTransactedWriteAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageStreamTransaction>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageStreamTransaction>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn CopyOverloadDefaultNameAndOptions<'a, Param0: ::windows::core::IntoParam<'a, IStorageFolder>>(&self, destinationfolder: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), destinationfolder.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn CopyOverloadDefaultOptions<'a, Param0: ::windows::core::IntoParam<'a, IStorageFolder>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, destinationfolder: Param0, desirednewname: Param1) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), destinationfolder.into_param().abi(), desirednewname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn CopyOverload<'a, Param0: ::windows::core::IntoParam<'a, IStorageFolder>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, destinationfolder: Param0, desirednewname: Param1, option: NameCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), destinationfolder.into_param().abi(), desirednewname.into_param().abi(), option, &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn CopyAndReplaceAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>>(&self, filetoreplace: Param0) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), filetoreplace.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn MoveOverloadDefaultNameAndOptions<'a, Param0: ::windows::core::IntoParam<'a, IStorageFolder>>(&self, destinationfolder: Param0) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), destinationfolder.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn MoveOverloadDefaultOptions<'a, Param0: ::windows::core::IntoParam<'a, IStorageFolder>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, destinationfolder: Param0, desirednewname: Param1) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), destinationfolder.into_param().abi(), desirednewname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn MoveOverload<'a, Param0: ::windows::core::IntoParam<'a, IStorageFolder>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, destinationfolder: Param0, desirednewname: Param1, option: NameCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), destinationfolder.into_param().abi(), desirednewname.into_param().abi(), option, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn MoveAndReplaceAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>>(&self, filetoreplace: Param0) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), filetoreplace.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn RenameAsyncOverloadDefaultOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn RenameAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0, option: NameCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), option, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn DeleteAsyncOverloadDefaultOptions(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn DeleteAsync(&self, option: StorageDeleteOption) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), option, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_FileProperties`*"]
    pub fn GetBasicPropertiesAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::BasicProperties>> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::BasicProperties>>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Attributes(&self) -> ::windows::core::Result<FileAttributes> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: FileAttributes = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileAttributes>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn DateCreated(&self) -> ::windows::core::Result<super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn IsOfType(&self, r#type: StorageItemTypes) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_Streams`*"]
    pub fn OpenSequentialReadAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Streams::IInputStream>> {
        let this = &::windows::core::Interface::cast::<Streams::IInputStreamReference>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<Streams::IInputStream>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_Streams`*"]
    pub fn OpenReadAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Streams::IRandomAccessStreamWithContentType>> {
        let this = &::windows::core::Interface::cast::<Streams::IRandomAccessStreamReference>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<Streams::IRandomAccessStreamWithContentType>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageFile {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{fa3f6186-4214-428c-a64c-14c9ac7315ea}");
}
impl ::core::convert::From<IStorageFile> for ::windows::core::IUnknown {
    fn from(value: IStorageFile) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IStorageFile> for ::windows::core::IUnknown {
    fn from(value: &IStorageFile) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IStorageFile> for ::windows::core::IInspectable {
    fn from(value: IStorageFile) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IStorageFile> for ::windows::core::IInspectable {
    fn from(value: &IStorageFile) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFile_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, accessmode: FileAccessMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, destinationfolder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, destinationfolder: ::windows::core::RawPtr, desirednewname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, destinationfolder: ::windows::core::RawPtr, desirednewname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, option: NameCollisionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filetoreplace: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, destinationfolder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, destinationfolder: ::windows::core::RawPtr, desirednewname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, destinationfolder: ::windows::core::RawPtr, desirednewname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, option: NameCollisionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filetoreplace: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Storage`*"]
pub struct IStorageFile2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageFile2 {
    type Vtable = IStorageFile2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x954e4bcf_0a77_42fb_b777_c2ed58a52e44);
}
impl IStorageFile2 {
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_Streams`*"]
    pub fn OpenWithOptionsAsync(&self, accessmode: FileAccessMode, options: StorageOpenOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Streams::IRandomAccessStream>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), accessmode, options, &mut result__).from_abi::<super::Foundation::IAsyncOperation<Streams::IRandomAccessStream>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn OpenTransactedWriteWithOptionsAsync(&self, options: StorageOpenOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageStreamTransaction>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), options, &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageStreamTransaction>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageFile2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{954e4bcf-0a77-42fb-b777-c2ed58a52e44}");
}
impl ::core::convert::From<IStorageFile2> for ::windows::core::IUnknown {
    fn from(value: IStorageFile2) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IStorageFile2> for ::windows::core::IUnknown {
    fn from(value: &IStorageFile2) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageFile2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStorageFile2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IStorageFile2> for ::windows::core::IInspectable {
    fn from(value: IStorageFile2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IStorageFile2> for ::windows::core::IInspectable {
    fn from(value: &IStorageFile2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStorageFile2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStorageFile2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFile2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, accessmode: FileAccessMode, options: StorageOpenOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, options: StorageOpenOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Storage`*"]
pub struct IStorageFilePropertiesWithAvailability(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageFilePropertiesWithAvailability {
    type Vtable = IStorageFilePropertiesWithAvailability_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xafcbbe9b_582b_4133_9648_e44ca46ee491);
}
impl IStorageFilePropertiesWithAvailability {
    #[doc = "*Required features: `Storage`*"]
    pub fn IsAvailable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageFilePropertiesWithAvailability {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{afcbbe9b-582b-4133-9648-e44ca46ee491}");
}
impl ::core::convert::From<IStorageFilePropertiesWithAvailability> for ::windows::core::IUnknown {
    fn from(value: IStorageFilePropertiesWithAvailability) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IStorageFilePropertiesWithAvailability> for ::windows::core::IUnknown {
    fn from(value: &IStorageFilePropertiesWithAvailability) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageFilePropertiesWithAvailability {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStorageFilePropertiesWithAvailability {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IStorageFilePropertiesWithAvailability> for ::windows::core::IInspectable {
    fn from(value: IStorageFilePropertiesWithAvailability) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IStorageFilePropertiesWithAvailability> for ::windows::core::IInspectable {
    fn from(value: &IStorageFilePropertiesWithAvailability) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStorageFilePropertiesWithAvailability {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStorageFilePropertiesWithAvailability {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFilePropertiesWithAvailability_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageFileStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageFileStatics {
    type Vtable = IStorageFileStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5984c710_daf2_43c8_8bb4_a4d3eacfd03f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFileStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, displaynamewithextension: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, datarequested: ::windows::core::RawPtr, thumbnail: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filetoreplace: ::windows::core::RawPtr, datarequested: ::windows::core::RawPtr, thumbnail: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, displaynamewithextension: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, uri: ::windows::core::RawPtr, thumbnail: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filetoreplace: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, thumbnail: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageFileStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageFileStatics2 {
    type Vtable = IStorageFileStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c76a781_212e_4af9_8f04_740cae108974);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFileStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Storage`*"]
pub struct IStorageFolder(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageFolder {
    type Vtable = IStorageFolder_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72d1cb78_b3ef_4f75_a80b_6fd9dae2944b);
}
impl IStorageFolder {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn CreateFileAsyncOverloadDefaultOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn CreateFileAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0, options: CreationCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), options, &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn CreateFolderAsyncOverloadDefaultOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn CreateFolderAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0, options: CreationCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), options, &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn GetFileAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn GetFolderAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn GetItemAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<IStorageItem>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetFilesAsyncOverloadDefaultOptionsStartAndCount(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFile>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFile>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetFoldersAsyncOverloadDefaultOptionsStartAndCount(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFolder>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFolder>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetItemsAsyncOverloadDefaultStartAndCount(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<IStorageItem>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<IStorageItem>>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn RenameAsyncOverloadDefaultOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn RenameAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0, option: NameCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), option, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn DeleteAsyncOverloadDefaultOptions(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn DeleteAsync(&self, option: StorageDeleteOption) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), option, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_FileProperties`*"]
    pub fn GetBasicPropertiesAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::BasicProperties>> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::BasicProperties>>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Attributes(&self) -> ::windows::core::Result<FileAttributes> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: FileAttributes = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileAttributes>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn DateCreated(&self) -> ::windows::core::Result<super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn IsOfType(&self, r#type: StorageItemTypes) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageFolder {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{72d1cb78-b3ef-4f75-a80b-6fd9dae2944b}");
}
impl ::core::convert::From<IStorageFolder> for ::windows::core::IUnknown {
    fn from(value: IStorageFolder) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IStorageFolder> for ::windows::core::IUnknown {
    fn from(value: &IStorageFolder) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageFolder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStorageFolder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IStorageFolder> for ::windows::core::IInspectable {
    fn from(value: IStorageFolder) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IStorageFolder> for ::windows::core::IInspectable {
    fn from(value: &IStorageFolder) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStorageFolder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStorageFolder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFolder_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: CreationCollisionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: CreationCollisionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Storage`*"]
pub struct IStorageFolder2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageFolder2 {
    type Vtable = IStorageFolder2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe827e8b9_08d9_4a8e_a0ac_fe5ed3cbbbd3);
}
impl IStorageFolder2 {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn TryGetItemAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<IStorageItem>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageFolder2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{e827e8b9-08d9-4a8e-a0ac-fe5ed3cbbbd3}");
}
impl ::core::convert::From<IStorageFolder2> for ::windows::core::IUnknown {
    fn from(value: IStorageFolder2) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IStorageFolder2> for ::windows::core::IUnknown {
    fn from(value: &IStorageFolder2) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageFolder2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStorageFolder2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IStorageFolder2> for ::windows::core::IInspectable {
    fn from(value: IStorageFolder2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IStorageFolder2> for ::windows::core::IInspectable {
    fn from(value: &IStorageFolder2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStorageFolder2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStorageFolder2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFolder2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageFolder3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageFolder3 {
    type Vtable = IStorageFolder3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f617899_bde1_4124_aeb3_b06ad96f98d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFolder3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageFolderStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageFolderStatics {
    type Vtable = IStorageFolderStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08f327ff_85d5_48b9_aee9_28511e339f9f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFolderStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageFolderStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageFolderStatics2 {
    type Vtable = IStorageFolderStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4656dc3_71d2_467d_8b29_371f0f62bf6f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFolderStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Storage`*"]
pub struct IStorageItem(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageItem {
    type Vtable = IStorageItem_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4207a996_ca2f_42f7_bde8_8b10457a7f30);
}
impl IStorageItem {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn RenameAsyncOverloadDefaultOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn RenameAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0, option: NameCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), option, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn DeleteAsyncOverloadDefaultOptions(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn DeleteAsync(&self, option: StorageDeleteOption) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), option, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_FileProperties`*"]
    pub fn GetBasicPropertiesAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::BasicProperties>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::BasicProperties>>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Attributes(&self) -> ::windows::core::Result<FileAttributes> {
        let this = self;
        unsafe {
            let mut result__: FileAttributes = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileAttributes>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn DateCreated(&self) -> ::windows::core::Result<super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn IsOfType(&self, r#type: StorageItemTypes) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{4207a996-ca2f-42f7-bde8-8b10457a7f30}");
}
impl ::core::convert::From<IStorageItem> for ::windows::core::IUnknown {
    fn from(value: IStorageItem) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IStorageItem> for ::windows::core::IUnknown {
    fn from(value: &IStorageItem) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStorageItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IStorageItem> for ::windows::core::IInspectable {
    fn from(value: IStorageItem) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IStorageItem> for ::windows::core::IInspectable {
    fn from(value: &IStorageItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStorageItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStorageItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItem_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, option: NameCollisionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, option: StorageDeleteOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_FileProperties")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut FileAttributes) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: StorageItemTypes, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Storage`*"]
pub struct IStorageItem2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageItem2 {
    type Vtable = IStorageItem2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53f926d2_083c_4283_b45b_81c007237e44);
}
impl IStorageItem2 {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn GetParentAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn IsEqual<'a, Param0: ::windows::core::IntoParam<'a, IStorageItem>>(&self, item: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), item.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn RenameAsyncOverloadDefaultOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn RenameAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0, option: NameCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), option, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn DeleteAsyncOverloadDefaultOptions(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn DeleteAsync(&self, option: StorageDeleteOption) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), option, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_FileProperties`*"]
    pub fn GetBasicPropertiesAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::BasicProperties>> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::BasicProperties>>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Attributes(&self) -> ::windows::core::Result<FileAttributes> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: FileAttributes = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileAttributes>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn DateCreated(&self) -> ::windows::core::Result<super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn IsOfType(&self, r#type: StorageItemTypes) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageItem2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{53f926d2-083c-4283-b45b-81c007237e44}");
}
impl ::core::convert::From<IStorageItem2> for ::windows::core::IUnknown {
    fn from(value: IStorageItem2) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IStorageItem2> for ::windows::core::IUnknown {
    fn from(value: &IStorageItem2) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageItem2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStorageItem2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IStorageItem2> for ::windows::core::IInspectable {
    fn from(value: IStorageItem2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IStorageItem2> for ::windows::core::IInspectable {
    fn from(value: &IStorageItem2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStorageItem2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStorageItem2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItem2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, item: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Storage`*"]
pub struct IStorageItemProperties(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageItemProperties {
    type Vtable = IStorageItemProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86664478_8029_46fe_a789_1c2f3e2ffb5c);
}
impl IStorageItemProperties {
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: FileProperties::ThumbnailMode) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), mode, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn GetThumbnailAsyncOverloadDefaultOptions(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), mode, requestedsize, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn GetThumbnailAsync(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), mode, requestedsize, options, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn DisplayType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn FolderRelativeId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    #[doc = "*Required features: `Storage`, `Storage_FileProperties`*"]
    pub fn Properties(&self) -> ::windows::core::Result<FileProperties::StorageItemContentProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileProperties::StorageItemContentProperties>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageItemProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{86664478-8029-46fe-a789-1c2f3e2ffb5c}");
}
impl ::core::convert::From<IStorageItemProperties> for ::windows::core::IUnknown {
    fn from(value: IStorageItemProperties) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IStorageItemProperties> for ::windows::core::IUnknown {
    fn from(value: &IStorageItemProperties) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageItemProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStorageItemProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IStorageItemProperties> for ::windows::core::IInspectable {
    fn from(value: IStorageItemProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IStorageItemProperties> for ::windows::core::IInspectable {
    fn from(value: &IStorageItemProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStorageItemProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStorageItemProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mode: FileProperties::ThumbnailMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mode: FileProperties::ThumbnailMode, requestedsize: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_FileProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Storage`*"]
pub struct IStorageItemProperties2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageItemProperties2 {
    type Vtable = IStorageItemProperties2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8e86a951_04b9_4bd2_929d_fef3f71621d0);
}
impl IStorageItemProperties2 {
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: FileProperties::ThumbnailMode) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), mode, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn GetScaledImageAsThumbnailAsyncOverloadDefaultOptions(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), mode, requestedsize, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn GetScaledImageAsThumbnailAsync(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), mode, requestedsize, options, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: FileProperties::ThumbnailMode) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), mode, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn GetThumbnailAsyncOverloadDefaultOptions(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), mode, requestedsize, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn GetThumbnailAsync(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), mode, requestedsize, options, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn DisplayType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn FolderRelativeId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    #[doc = "*Required features: `Storage`, `Storage_FileProperties`*"]
    pub fn Properties(&self) -> ::windows::core::Result<FileProperties::StorageItemContentProperties> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileProperties::StorageItemContentProperties>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageItemProperties2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{8e86a951-04b9-4bd2-929d-fef3f71621d0}");
}
impl ::core::convert::From<IStorageItemProperties2> for ::windows::core::IUnknown {
    fn from(value: IStorageItemProperties2) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IStorageItemProperties2> for ::windows::core::IUnknown {
    fn from(value: &IStorageItemProperties2) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageItemProperties2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStorageItemProperties2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IStorageItemProperties2> for ::windows::core::IInspectable {
    fn from(value: IStorageItemProperties2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IStorageItemProperties2> for ::windows::core::IInspectable {
    fn from(value: &IStorageItemProperties2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStorageItemProperties2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStorageItemProperties2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemProperties2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mode: FileProperties::ThumbnailMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mode: FileProperties::ThumbnailMode, requestedsize: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Storage`*"]
pub struct IStorageItemPropertiesWithProvider(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageItemPropertiesWithProvider {
    type Vtable = IStorageItemPropertiesWithProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x861bf39b_6368_4dee_b40e_74684a5ce714);
}
impl IStorageItemPropertiesWithProvider {
    #[doc = "*Required features: `Storage`*"]
    pub fn Provider(&self) -> ::windows::core::Result<StorageProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageProvider>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: FileProperties::ThumbnailMode) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), mode, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn GetThumbnailAsyncOverloadDefaultOptions(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), mode, requestedsize, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn GetThumbnailAsync(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), mode, requestedsize, options, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn DisplayType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn FolderRelativeId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    #[doc = "*Required features: `Storage`, `Storage_FileProperties`*"]
    pub fn Properties(&self) -> ::windows::core::Result<FileProperties::StorageItemContentProperties> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileProperties::StorageItemContentProperties>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageItemPropertiesWithProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{861bf39b-6368-4dee-b40e-74684a5ce714}");
}
impl ::core::convert::From<IStorageItemPropertiesWithProvider> for ::windows::core::IUnknown {
    fn from(value: IStorageItemPropertiesWithProvider) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IStorageItemPropertiesWithProvider> for ::windows::core::IUnknown {
    fn from(value: &IStorageItemPropertiesWithProvider) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageItemPropertiesWithProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStorageItemPropertiesWithProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IStorageItemPropertiesWithProvider> for ::windows::core::IInspectable {
    fn from(value: IStorageItemPropertiesWithProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IStorageItemPropertiesWithProvider> for ::windows::core::IInspectable {
    fn from(value: &IStorageItemPropertiesWithProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStorageItemPropertiesWithProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStorageItemPropertiesWithProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemPropertiesWithProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageLibrary(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageLibrary {
    type Vtable = IStorageLibrary_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1edd7103_0e5e_4d6c_b5e8_9318983d6a03);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibrary_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, folder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, eventcookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageLibrary2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageLibrary2 {
    type Vtable = IStorageLibrary2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b0ce348_fcb3_4031_afb0_a68d7bd44534);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibrary2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageLibrary3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageLibrary3 {
    type Vtable = IStorageLibrary3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a281291_2154_4201_8113_d2c05ce1ad23);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibrary3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageLibraryChange(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageLibraryChange {
    type Vtable = IStorageLibraryChange_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00980b23_2be2_4909_aa48_159f5203a51e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryChange_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut StorageLibraryChangeType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: StorageItemTypes, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageLibraryChangeReader(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageLibraryChangeReader {
    type Vtable = IStorageLibraryChangeReader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf205bc83_fca2_41f9_8954_ee2e991eb96f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryChangeReader_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageLibraryChangeReader2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageLibraryChangeReader2 {
    type Vtable = IStorageLibraryChangeReader2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xabf4868b_fbcc_4a4f_999e_e7ab7c646dbe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryChangeReader2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageLibraryChangeTracker(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageLibraryChangeTracker {
    type Vtable = IStorageLibraryChangeTracker_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e157316_6073_44f6_9681_7492d1286c90);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryChangeTracker_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageLibraryChangeTracker2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageLibraryChangeTracker2 {
    type Vtable = IStorageLibraryChangeTracker2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd051c3b_0f9f_42f9_8fb3_158d82e13821);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryChangeTracker2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, options: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageLibraryChangeTrackerOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageLibraryChangeTrackerOptions {
    type Vtable = IStorageLibraryChangeTrackerOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb52bcd4_1a6d_59c0_ad2a_823a20532483);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryChangeTrackerOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageLibraryLastChangeId(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageLibraryLastChangeId {
    type Vtable = IStorageLibraryLastChangeId_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5281826a_bbe1_53bc_82ca_81cc7f039329);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryLastChangeId_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageLibraryLastChangeIdStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageLibraryLastChangeIdStatics {
    type Vtable = IStorageLibraryLastChangeIdStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81a49128_2ca3_5309_b0d1_cf0788e40762);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryLastChangeIdStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageLibraryStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageLibraryStatics {
    type Vtable = IStorageLibraryStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4208a6db_684a_49c6_9e59_90121ee050d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, libraryid: KnownLibraryId, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageLibraryStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageLibraryStatics2 {
    type Vtable = IStorageLibraryStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xffb08ddc_fa75_4695_b9d1_7f81f97832e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, libraryid: KnownLibraryId, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageProvider(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageProvider {
    type Vtable = IStorageProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe705eed4_d478_47d6_ba46_1a8ebe114a20);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageProvider2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageProvider2 {
    type Vtable = IStorageProvider2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x010d1917_3404_414b_9fd7_cd44472eaa39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProvider2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propertycanonicalname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStorageStreamTransaction(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStorageStreamTransaction {
    type Vtable = IStorageStreamTransaction_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf67cf363_a53d_4d94_ae2c_67232d93acdd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageStreamTransaction_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Storage`*"]
pub struct IStreamedFileDataRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStreamedFileDataRequest {
    type Vtable = IStreamedFileDataRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1673fcce_dabd_4d50_beee_180b8a8191b6);
}
impl IStreamedFileDataRequest {
    #[doc = "*Required features: `Storage`*"]
    pub fn FailAndClose(&self, failuremode: StreamedFileFailureMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), failuremode).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IStreamedFileDataRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{1673fcce-dabd-4d50-beee-180b8a8191b6}");
}
impl ::core::convert::From<IStreamedFileDataRequest> for ::windows::core::IUnknown {
    fn from(value: IStreamedFileDataRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IStreamedFileDataRequest> for ::windows::core::IUnknown {
    fn from(value: &IStreamedFileDataRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStreamedFileDataRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStreamedFileDataRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IStreamedFileDataRequest> for ::windows::core::IInspectable {
    fn from(value: IStreamedFileDataRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IStreamedFileDataRequest> for ::windows::core::IInspectable {
    fn from(value: &IStreamedFileDataRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStreamedFileDataRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStreamedFileDataRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamedFileDataRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, failuremode: StreamedFileFailureMode) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemAudioProperties(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISystemAudioProperties {
    type Vtable = ISystemAudioProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f8f38b7_308c_47e1_924d_8645348e5db7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemAudioProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemDataPaths(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISystemDataPaths {
    type Vtable = ISystemDataPaths_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe32abf70_d8fa_45ec_a942_d2e26fb60ba5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemDataPaths_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemDataPathsStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISystemDataPathsStatics {
    type Vtable = ISystemDataPathsStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0f96fd0_9920_4bca_b379_f96fdf7caad8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemDataPathsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemGPSProperties(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISystemGPSProperties {
    type Vtable = ISystemGPSProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0f46eb4_c174_481a_bc25_921986f6a6f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemGPSProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemImageProperties(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISystemImageProperties {
    type Vtable = ISystemImageProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x011b2e30_8b39_4308_bea1_e8aa61e47826);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemImageProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemMediaProperties(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISystemMediaProperties {
    type Vtable = ISystemMediaProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa42b3316_8415_40dc_8c44_98361d235430);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMediaProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemMusicProperties(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISystemMusicProperties {
    type Vtable = ISystemMusicProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb47988d5_67af_4bc3_8d39_5b89022026a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMusicProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemPhotoProperties(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISystemPhotoProperties {
    type Vtable = ISystemPhotoProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4734fc3d_ab21_4424_b735_f4353a56c8fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemPhotoProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemProperties(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISystemProperties {
    type Vtable = ISystemProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x917a71c1_85f3_4dd1_b001_a50bfd21c8d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISystemVideoProperties(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISystemVideoProperties {
    type Vtable = ISystemVideoProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2040f715_67f8_4322_9b80_4fa9fefb83e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemVideoProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataPaths(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUserDataPaths {
    type Vtable = IUserDataPaths_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9c53912_abc4_46ff_8a2b_dc9d7fa6e52f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataPaths_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataPathsStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUserDataPathsStatics {
    type Vtable = IUserDataPathsStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01b29def_e062_48a1_8b0c_f2c7a9ca56c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataPathsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Storage`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KnownFolderId(pub i32);
impl KnownFolderId {
    pub const AppCaptures: KnownFolderId = KnownFolderId(0i32);
    pub const CameraRoll: KnownFolderId = KnownFolderId(1i32);
    pub const DocumentsLibrary: KnownFolderId = KnownFolderId(2i32);
    pub const HomeGroup: KnownFolderId = KnownFolderId(3i32);
    pub const MediaServerDevices: KnownFolderId = KnownFolderId(4i32);
    pub const MusicLibrary: KnownFolderId = KnownFolderId(5i32);
    pub const Objects3D: KnownFolderId = KnownFolderId(6i32);
    pub const PicturesLibrary: KnownFolderId = KnownFolderId(7i32);
    pub const Playlists: KnownFolderId = KnownFolderId(8i32);
    pub const RecordedCalls: KnownFolderId = KnownFolderId(9i32);
    pub const RemovableDevices: KnownFolderId = KnownFolderId(10i32);
    pub const SavedPictures: KnownFolderId = KnownFolderId(11i32);
    pub const Screenshots: KnownFolderId = KnownFolderId(12i32);
    pub const VideosLibrary: KnownFolderId = KnownFolderId(13i32);
    pub const AllAppMods: KnownFolderId = KnownFolderId(14i32);
    pub const CurrentAppMods: KnownFolderId = KnownFolderId(15i32);
    pub const DownloadsFolder: KnownFolderId = KnownFolderId(16i32);
}
impl ::core::convert::From<i32> for KnownFolderId {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for KnownFolderId {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for KnownFolderId {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.KnownFolderId;i4)");
}
impl ::windows::core::DefaultType for KnownFolderId {
    type DefaultType = Self;
}
#[doc = "*Required features: `Storage`*"]
pub struct KnownFolders {}
impl KnownFolders {
    #[doc = "*Required features: `Storage`*"]
    pub fn CameraRoll() -> ::windows::core::Result<StorageFolder> {
        Self::IKnownFoldersCameraRollStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        })
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Playlists() -> ::windows::core::Result<StorageFolder> {
        Self::IKnownFoldersPlaylistsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        })
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn SavedPictures() -> ::windows::core::Result<StorageFolder> {
        Self::IKnownFoldersSavedPicturesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        })
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn MusicLibrary() -> ::windows::core::Result<StorageFolder> {
        Self::IKnownFoldersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        })
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn PicturesLibrary() -> ::windows::core::Result<StorageFolder> {
        Self::IKnownFoldersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        })
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn VideosLibrary() -> ::windows::core::Result<StorageFolder> {
        Self::IKnownFoldersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        })
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn DocumentsLibrary() -> ::windows::core::Result<StorageFolder> {
        Self::IKnownFoldersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        })
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn HomeGroup() -> ::windows::core::Result<StorageFolder> {
        Self::IKnownFoldersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        })
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn RemovableDevices() -> ::windows::core::Result<StorageFolder> {
        Self::IKnownFoldersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        })
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn MediaServerDevices() -> ::windows::core::Result<StorageFolder> {
        Self::IKnownFoldersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        })
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Objects3D() -> ::windows::core::Result<StorageFolder> {
        Self::IKnownFoldersStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        })
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn AppCaptures() -> ::windows::core::Result<StorageFolder> {
        Self::IKnownFoldersStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        })
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn RecordedCalls() -> ::windows::core::Result<StorageFolder> {
        Self::IKnownFoldersStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `System`*"]
    pub fn GetFolderForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::System::User>>(user: Param0, folderid: KnownFolderId) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
        Self::IKnownFoldersStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), user.into_param().abi(), folderid, &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn RequestAccessAsync(folderid: KnownFolderId) -> ::windows::core::Result<super::Foundation::IAsyncOperation<KnownFoldersAccessStatus>> {
        Self::IKnownFoldersStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), folderid, &mut result__).from_abi::<super::Foundation::IAsyncOperation<KnownFoldersAccessStatus>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `System`*"]
    pub fn RequestAccessForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::System::User>>(user: Param0, folderid: KnownFolderId) -> ::windows::core::Result<super::Foundation::IAsyncOperation<KnownFoldersAccessStatus>> {
        Self::IKnownFoldersStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), user.into_param().abi(), folderid, &mut result__).from_abi::<super::Foundation::IAsyncOperation<KnownFoldersAccessStatus>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn GetFolderAsync(folderid: KnownFolderId) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
        Self::IKnownFoldersStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), folderid, &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
        })
    }
    pub fn IKnownFoldersCameraRollStatics<R, F: FnOnce(&IKnownFoldersCameraRollStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KnownFolders, IKnownFoldersCameraRollStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IKnownFoldersPlaylistsStatics<R, F: FnOnce(&IKnownFoldersPlaylistsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KnownFolders, IKnownFoldersPlaylistsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IKnownFoldersSavedPicturesStatics<R, F: FnOnce(&IKnownFoldersSavedPicturesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KnownFolders, IKnownFoldersSavedPicturesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IKnownFoldersStatics<R, F: FnOnce(&IKnownFoldersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KnownFolders, IKnownFoldersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IKnownFoldersStatics2<R, F: FnOnce(&IKnownFoldersStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KnownFolders, IKnownFoldersStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IKnownFoldersStatics3<R, F: FnOnce(&IKnownFoldersStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KnownFolders, IKnownFoldersStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IKnownFoldersStatics4<R, F: FnOnce(&IKnownFoldersStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KnownFolders, IKnownFoldersStatics4> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for KnownFolders {
    const NAME: &'static str = "Windows.Storage.KnownFolders";
}
#[doc = "*Required features: `Storage`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KnownFoldersAccessStatus(pub i32);
impl KnownFoldersAccessStatus {
    pub const DeniedBySystem: KnownFoldersAccessStatus = KnownFoldersAccessStatus(0i32);
    pub const NotDeclaredByApp: KnownFoldersAccessStatus = KnownFoldersAccessStatus(1i32);
    pub const DeniedByUser: KnownFoldersAccessStatus = KnownFoldersAccessStatus(2i32);
    pub const UserPromptRequired: KnownFoldersAccessStatus = KnownFoldersAccessStatus(3i32);
    pub const Allowed: KnownFoldersAccessStatus = KnownFoldersAccessStatus(4i32);
    pub const AllowedPerAppFolder: KnownFoldersAccessStatus = KnownFoldersAccessStatus(5i32);
}
impl ::core::convert::From<i32> for KnownFoldersAccessStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for KnownFoldersAccessStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for KnownFoldersAccessStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.KnownFoldersAccessStatus;i4)");
}
impl ::windows::core::DefaultType for KnownFoldersAccessStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Storage`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KnownLibraryId(pub i32);
impl KnownLibraryId {
    pub const Music: KnownLibraryId = KnownLibraryId(0i32);
    pub const Pictures: KnownLibraryId = KnownLibraryId(1i32);
    pub const Videos: KnownLibraryId = KnownLibraryId(2i32);
    pub const Documents: KnownLibraryId = KnownLibraryId(3i32);
}
impl ::core::convert::From<i32> for KnownLibraryId {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for KnownLibraryId {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for KnownLibraryId {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.KnownLibraryId;i4)");
}
impl ::windows::core::DefaultType for KnownLibraryId {
    type DefaultType = Self;
}
#[doc = "*Required features: `Storage`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NameCollisionOption(pub i32);
impl NameCollisionOption {
    pub const GenerateUniqueName: NameCollisionOption = NameCollisionOption(0i32);
    pub const ReplaceExisting: NameCollisionOption = NameCollisionOption(1i32);
    pub const FailIfExists: NameCollisionOption = NameCollisionOption(2i32);
}
impl ::core::convert::From<i32> for NameCollisionOption {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NameCollisionOption {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for NameCollisionOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.NameCollisionOption;i4)");
}
impl ::windows::core::DefaultType for NameCollisionOption {
    type DefaultType = Self;
}
#[doc = "*Required features: `Storage`*"]
pub struct PathIO {}
impl PathIO {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn ReadTextAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(absolutepath: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IPathIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), absolutepath.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_Streams`*"]
    pub fn ReadTextWithEncodingAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(absolutepath: Param0, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IPathIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), absolutepath.into_param().abi(), encoding, &mut result__).from_abi::<super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn WriteTextAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(absolutepath: Param0, contents: Param1) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IPathIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), absolutepath.into_param().abi(), contents.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_Streams`*"]
    pub fn WriteTextWithEncodingAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(absolutepath: Param0, contents: Param1, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IPathIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), absolutepath.into_param().abi(), contents.into_param().abi(), encoding, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn AppendTextAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(absolutepath: Param0, contents: Param1) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IPathIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), absolutepath.into_param().abi(), contents.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_Streams`*"]
    pub fn AppendTextWithEncodingAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(absolutepath: Param0, contents: Param1, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IPathIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), absolutepath.into_param().abi(), contents.into_param().abi(), encoding, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Foundation_Collections`*"]
    pub fn ReadLinesAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(absolutepath: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<::windows::core::HSTRING>>> {
        Self::IPathIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), absolutepath.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<::windows::core::HSTRING>>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Foundation_Collections`, `Storage_Streams`*"]
    pub fn ReadLinesWithEncodingAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(absolutepath: Param0, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<::windows::core::HSTRING>>> {
        Self::IPathIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), absolutepath.into_param().abi(), encoding, &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<::windows::core::HSTRING>>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Foundation_Collections`*"]
    pub fn WriteLinesAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(absolutepath: Param0, lines: Param1) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IPathIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), absolutepath.into_param().abi(), lines.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Foundation_Collections`, `Storage_Streams`*"]
    pub fn WriteLinesWithEncodingAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(absolutepath: Param0, lines: Param1, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IPathIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), absolutepath.into_param().abi(), lines.into_param().abi(), encoding, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Foundation_Collections`*"]
    pub fn AppendLinesAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(absolutepath: Param0, lines: Param1) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IPathIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), absolutepath.into_param().abi(), lines.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Foundation_Collections`, `Storage_Streams`*"]
    pub fn AppendLinesWithEncodingAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(absolutepath: Param0, lines: Param1, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IPathIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), absolutepath.into_param().abi(), lines.into_param().abi(), encoding, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_Streams`*"]
    pub fn ReadBufferAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(absolutepath: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Streams::IBuffer>> {
        Self::IPathIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), absolutepath.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<Streams::IBuffer>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_Streams`*"]
    pub fn WriteBufferAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, Streams::IBuffer>>(absolutepath: Param0, buffer: Param1) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IPathIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), absolutepath.into_param().abi(), buffer.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn WriteBytesAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(absolutepath: Param0, buffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        Self::IPathIOStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), absolutepath.into_param().abi(), buffer.len() as u32, ::core::mem::transmute(buffer.as_ptr()), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        })
    }
    pub fn IPathIOStatics<R, F: FnOnce(&IPathIOStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PathIO, IPathIOStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for PathIO {
    const NAME: &'static str = "Windows.Storage.PathIO";
}
#[doc = "*Required features: `Storage`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SetVersionDeferral(pub ::windows::core::IInspectable);
impl SetVersionDeferral {
    #[doc = "*Required features: `Storage`*"]
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for SetVersionDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.SetVersionDeferral;{033508a2-781a-437a-b078-3f32badcfe47})");
}
unsafe impl ::windows::core::Interface for SetVersionDeferral {
    type Vtable = ISetVersionDeferral_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x033508a2_781a_437a_b078_3f32badcfe47);
}
impl ::windows::core::RuntimeName for SetVersionDeferral {
    const NAME: &'static str = "Windows.Storage.SetVersionDeferral";
}
impl ::core::convert::From<SetVersionDeferral> for ::windows::core::IUnknown {
    fn from(value: SetVersionDeferral) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SetVersionDeferral> for ::windows::core::IUnknown {
    fn from(value: &SetVersionDeferral) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SetVersionDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SetVersionDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SetVersionDeferral> for ::windows::core::IInspectable {
    fn from(value: SetVersionDeferral) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SetVersionDeferral> for ::windows::core::IInspectable {
    fn from(value: &SetVersionDeferral) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SetVersionDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SetVersionDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SetVersionDeferral {}
unsafe impl ::core::marker::Sync for SetVersionDeferral {}
#[doc = "*Required features: `Storage`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SetVersionRequest(pub ::windows::core::IInspectable);
impl SetVersionRequest {
    #[doc = "*Required features: `Storage`*"]
    pub fn CurrentVersion(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn DesiredVersion(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<SetVersionDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SetVersionDeferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SetVersionRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.SetVersionRequest;{b9c76b9b-1056-4e69-8330-162619956f9b})");
}
unsafe impl ::windows::core::Interface for SetVersionRequest {
    type Vtable = ISetVersionRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9c76b9b_1056_4e69_8330_162619956f9b);
}
impl ::windows::core::RuntimeName for SetVersionRequest {
    const NAME: &'static str = "Windows.Storage.SetVersionRequest";
}
impl ::core::convert::From<SetVersionRequest> for ::windows::core::IUnknown {
    fn from(value: SetVersionRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SetVersionRequest> for ::windows::core::IUnknown {
    fn from(value: &SetVersionRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SetVersionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SetVersionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SetVersionRequest> for ::windows::core::IInspectable {
    fn from(value: SetVersionRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SetVersionRequest> for ::windows::core::IInspectable {
    fn from(value: &SetVersionRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SetVersionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SetVersionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SetVersionRequest {}
unsafe impl ::core::marker::Sync for SetVersionRequest {}
#[doc = "*Required features: `Storage`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct StorageDeleteOption(pub i32);
impl StorageDeleteOption {
    pub const Default: StorageDeleteOption = StorageDeleteOption(0i32);
    pub const PermanentDelete: StorageDeleteOption = StorageDeleteOption(1i32);
}
impl ::core::convert::From<i32> for StorageDeleteOption {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for StorageDeleteOption {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for StorageDeleteOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.StorageDeleteOption;i4)");
}
impl ::windows::core::DefaultType for StorageDeleteOption {
    type DefaultType = Self;
}
#[doc = "*Required features: `Storage`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct StorageFile(pub ::windows::core::IInspectable);
impl StorageFile {
    #[doc = "*Required features: `Storage`*"]
    pub fn FileType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_Streams`*"]
    pub fn OpenAsync(&self, accessmode: FileAccessMode) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Streams::IRandomAccessStream>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), accessmode, &mut result__).from_abi::<super::Foundation::IAsyncOperation<Streams::IRandomAccessStream>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn OpenTransactedWriteAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageStreamTransaction>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageStreamTransaction>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn CopyOverloadDefaultNameAndOptions<'a, Param0: ::windows::core::IntoParam<'a, IStorageFolder>>(&self, destinationfolder: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), destinationfolder.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn CopyOverloadDefaultOptions<'a, Param0: ::windows::core::IntoParam<'a, IStorageFolder>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, destinationfolder: Param0, desirednewname: Param1) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), destinationfolder.into_param().abi(), desirednewname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn CopyOverload<'a, Param0: ::windows::core::IntoParam<'a, IStorageFolder>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, destinationfolder: Param0, desirednewname: Param1, option: NameCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), destinationfolder.into_param().abi(), desirednewname.into_param().abi(), option, &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn CopyAndReplaceAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>>(&self, filetoreplace: Param0) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), filetoreplace.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn MoveOverloadDefaultNameAndOptions<'a, Param0: ::windows::core::IntoParam<'a, IStorageFolder>>(&self, destinationfolder: Param0) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), destinationfolder.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn MoveOverloadDefaultOptions<'a, Param0: ::windows::core::IntoParam<'a, IStorageFolder>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, destinationfolder: Param0, desirednewname: Param1) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), destinationfolder.into_param().abi(), desirednewname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn MoveOverload<'a, Param0: ::windows::core::IntoParam<'a, IStorageFolder>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, destinationfolder: Param0, desirednewname: Param1, option: NameCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), destinationfolder.into_param().abi(), desirednewname.into_param().abi(), option, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn MoveAndReplaceAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>>(&self, filetoreplace: Param0) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), filetoreplace.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_Streams`*"]
    pub fn OpenWithOptionsAsync(&self, accessmode: FileAccessMode, options: StorageOpenOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Streams::IRandomAccessStream>> {
        let this = &::windows::core::Interface::cast::<IStorageFile2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), accessmode, options, &mut result__).from_abi::<super::Foundation::IAsyncOperation<Streams::IRandomAccessStream>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn OpenTransactedWriteWithOptionsAsync(&self, options: StorageOpenOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageStreamTransaction>> {
        let this = &::windows::core::Interface::cast::<IStorageFile2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), options, &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageStreamTransaction>>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn IsAvailable(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IStorageFilePropertiesWithAvailability>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn RenameAsyncOverloadDefaultOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn RenameAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0, option: NameCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), option, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn DeleteAsyncOverloadDefaultOptions(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn DeleteAsync(&self, option: StorageDeleteOption) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), option, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_FileProperties`*"]
    pub fn GetBasicPropertiesAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::BasicProperties>> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::BasicProperties>>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Attributes(&self) -> ::windows::core::Result<FileAttributes> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: FileAttributes = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileAttributes>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn DateCreated(&self) -> ::windows::core::Result<super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn IsOfType(&self, r#type: StorageItemTypes) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn GetParentAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
        let this = &::windows::core::Interface::cast::<IStorageItem2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn IsEqual<'a, Param0: ::windows::core::IntoParam<'a, IStorageItem>>(&self, item: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IStorageItem2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), item.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: FileProperties::ThumbnailMode) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), mode, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn GetThumbnailAsyncOverloadDefaultOptions(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), mode, requestedsize, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn GetThumbnailAsync(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), mode, requestedsize, options, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn DisplayType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn FolderRelativeId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    #[doc = "*Required features: `Storage`, `Storage_FileProperties`*"]
    pub fn Properties(&self) -> ::windows::core::Result<FileProperties::StorageItemContentProperties> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileProperties::StorageItemContentProperties>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: FileProperties::ThumbnailMode) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), mode, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn GetScaledImageAsThumbnailAsyncOverloadDefaultOptions(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), mode, requestedsize, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn GetScaledImageAsThumbnailAsync(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), mode, requestedsize, options, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Provider(&self) -> ::windows::core::Result<StorageProvider> {
        let this = &::windows::core::Interface::cast::<IStorageItemPropertiesWithProvider>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageProvider>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_Streams`*"]
    pub fn OpenSequentialReadAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Streams::IInputStream>> {
        let this = &::windows::core::Interface::cast::<Streams::IInputStreamReference>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<Streams::IInputStream>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_Streams`*"]
    pub fn OpenReadAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Streams::IRandomAccessStreamWithContentType>> {
        let this = &::windows::core::Interface::cast::<Streams::IRandomAccessStreamReference>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<Streams::IRandomAccessStreamWithContentType>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn GetFileFromPathAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(path: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        Self::IStorageFileStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), path.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn GetFileFromApplicationUriAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Uri>>(uri: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        Self::IStorageFileStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_Streams`*"]
    pub fn CreateStreamedFileAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, StreamedFileDataRequestedHandler>, Param2: ::windows::core::IntoParam<'a, Streams::IRandomAccessStreamReference>>(displaynamewithextension: Param0, datarequested: Param1, thumbnail: Param2) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        Self::IStorageFileStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), displaynamewithextension.into_param().abi(), datarequested.into_param().abi(), thumbnail.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_Streams`*"]
    pub fn ReplaceWithStreamedFileAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>, Param1: ::windows::core::IntoParam<'a, StreamedFileDataRequestedHandler>, Param2: ::windows::core::IntoParam<'a, Streams::IRandomAccessStreamReference>>(filetoreplace: Param0, datarequested: Param1, thumbnail: Param2) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        Self::IStorageFileStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), filetoreplace.into_param().abi(), datarequested.into_param().abi(), thumbnail.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_Streams`*"]
    pub fn CreateStreamedFileFromUriAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::Foundation::Uri>, Param2: ::windows::core::IntoParam<'a, Streams::IRandomAccessStreamReference>>(displaynamewithextension: Param0, uri: Param1, thumbnail: Param2) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        Self::IStorageFileStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), displaynamewithextension.into_param().abi(), uri.into_param().abi(), thumbnail.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_Streams`*"]
    pub fn ReplaceWithStreamedFileFromUriAsync<'a, Param0: ::windows::core::IntoParam<'a, IStorageFile>, Param1: ::windows::core::IntoParam<'a, super::Foundation::Uri>, Param2: ::windows::core::IntoParam<'a, Streams::IRandomAccessStreamReference>>(filetoreplace: Param0, uri: Param1, thumbnail: Param2) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        Self::IStorageFileStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), filetoreplace.into_param().abi(), uri.into_param().abi(), thumbnail.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `System`*"]
    pub fn GetFileFromPathForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, path: Param1) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        Self::IStorageFileStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), user.into_param().abi(), path.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        })
    }
    pub fn IStorageFileStatics<R, F: FnOnce(&IStorageFileStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageFile, IStorageFileStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IStorageFileStatics2<R, F: FnOnce(&IStorageFileStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageFile, IStorageFileStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for StorageFile {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.StorageFile;{fa3f6186-4214-428c-a64c-14c9ac7315ea})");
}
unsafe impl ::windows::core::Interface for StorageFile {
    type Vtable = IStorageFile_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa3f6186_4214_428c_a64c_14c9ac7315ea);
}
impl ::windows::core::RuntimeName for StorageFile {
    const NAME: &'static str = "Windows.Storage.StorageFile";
}
impl ::core::convert::From<StorageFile> for ::windows::core::IUnknown {
    fn from(value: StorageFile) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&StorageFile> for ::windows::core::IUnknown {
    fn from(value: &StorageFile) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<StorageFile> for ::windows::core::IInspectable {
    fn from(value: StorageFile) -> Self {
        value.0
    }
}
impl ::core::convert::From<&StorageFile> for ::windows::core::IInspectable {
    fn from(value: &StorageFile) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<StorageFile> for IStorageFile {
    fn from(value: StorageFile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageFile> for IStorageFile {
    fn from(value: &StorageFile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageFile> for StorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageFile> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageFile> for &StorageFile {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageFile> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc = "*Required features: `Storage`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct StorageFolder(pub ::windows::core::IInspectable);
impl StorageFolder {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn CreateFileAsyncOverloadDefaultOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn CreateFileAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0, options: CreationCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), options, &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn CreateFolderAsyncOverloadDefaultOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn CreateFolderAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0, options: CreationCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), options, &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn GetFileAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn GetFolderAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn GetItemAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<IStorageItem>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetFilesAsyncOverloadDefaultOptionsStartAndCount(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFile>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFile>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetFoldersAsyncOverloadDefaultOptionsStartAndCount(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFolder>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFolder>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetItemsAsyncOverloadDefaultStartAndCount(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<IStorageItem>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<IStorageItem>>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn TryGetItemAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<IStorageItem>> {
        let this = &::windows::core::Interface::cast::<IStorageFolder2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<IStorageItem>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn RenameAsyncOverloadDefaultOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn RenameAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, desiredname: Param0, option: NameCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), desiredname.into_param().abi(), option, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn DeleteAsyncOverloadDefaultOptions(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn DeleteAsync(&self, option: StorageDeleteOption) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), option, &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_FileProperties`*"]
    pub fn GetBasicPropertiesAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::BasicProperties>> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::BasicProperties>>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Attributes(&self) -> ::windows::core::Result<FileAttributes> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: FileAttributes = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileAttributes>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn DateCreated(&self) -> ::windows::core::Result<super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn IsOfType(&self, r#type: StorageItemTypes) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn GetParentAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
        let this = &::windows::core::Interface::cast::<IStorageItem2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn IsEqual<'a, Param0: ::windows::core::IntoParam<'a, IStorageItem>>(&self, item: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IStorageItem2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), item.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: FileProperties::ThumbnailMode) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), mode, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn GetThumbnailAsyncOverloadDefaultOptions(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), mode, requestedsize, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn GetThumbnailAsync(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), mode, requestedsize, options, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn DisplayType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn FolderRelativeId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    #[doc = "*Required features: `Storage`, `Storage_FileProperties`*"]
    pub fn Properties(&self) -> ::windows::core::Result<FileProperties::StorageItemContentProperties> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileProperties::StorageItemContentProperties>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: FileProperties::ThumbnailMode) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), mode, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn GetScaledImageAsThumbnailAsyncOverloadDefaultOptions(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), mode, requestedsize, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_FileProperties`, `Storage_Streams`*"]
    pub fn GetScaledImageAsThumbnailAsync(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<IStorageItemProperties2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), mode, requestedsize, options, &mut result__).from_abi::<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Provider(&self) -> ::windows::core::Result<StorageProvider> {
        let this = &::windows::core::Interface::cast::<IStorageItemPropertiesWithProvider>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageProvider>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Search"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_Search`*"]
    pub fn GetIndexedStateAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Search::IndexedState>> {
        let this = &::windows::core::Interface::cast::<Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<Search::IndexedState>>(result__)
        }
    }
    #[cfg(feature = "Storage_Search")]
    #[doc = "*Required features: `Storage`, `Storage_Search`*"]
    pub fn CreateFileQueryOverloadDefault(&self) -> ::windows::core::Result<Search::StorageFileQueryResult> {
        let this = &::windows::core::Interface::cast::<Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Search::StorageFileQueryResult>(result__)
        }
    }
    #[cfg(feature = "Storage_Search")]
    #[doc = "*Required features: `Storage`, `Storage_Search`*"]
    pub fn CreateFileQuery(&self, query: Search::CommonFileQuery) -> ::windows::core::Result<Search::StorageFileQueryResult> {
        let this = &::windows::core::Interface::cast::<Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), query, &mut result__).from_abi::<Search::StorageFileQueryResult>(result__)
        }
    }
    #[cfg(feature = "Storage_Search")]
    #[doc = "*Required features: `Storage`, `Storage_Search`*"]
    pub fn CreateFileQueryWithOptions<'a, Param0: ::windows::core::IntoParam<'a, Search::QueryOptions>>(&self, queryoptions: Param0) -> ::windows::core::Result<Search::StorageFileQueryResult> {
        let this = &::windows::core::Interface::cast::<Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), queryoptions.into_param().abi(), &mut result__).from_abi::<Search::StorageFileQueryResult>(result__)
        }
    }
    #[cfg(feature = "Storage_Search")]
    #[doc = "*Required features: `Storage`, `Storage_Search`*"]
    pub fn CreateFolderQueryOverloadDefault(&self) -> ::windows::core::Result<Search::StorageFolderQueryResult> {
        let this = &::windows::core::Interface::cast::<Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Search::StorageFolderQueryResult>(result__)
        }
    }
    #[cfg(feature = "Storage_Search")]
    #[doc = "*Required features: `Storage`, `Storage_Search`*"]
    pub fn CreateFolderQuery(&self, query: Search::CommonFolderQuery) -> ::windows::core::Result<Search::StorageFolderQueryResult> {
        let this = &::windows::core::Interface::cast::<Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), query, &mut result__).from_abi::<Search::StorageFolderQueryResult>(result__)
        }
    }
    #[cfg(feature = "Storage_Search")]
    #[doc = "*Required features: `Storage`, `Storage_Search`*"]
    pub fn CreateFolderQueryWithOptions<'a, Param0: ::windows::core::IntoParam<'a, Search::QueryOptions>>(&self, queryoptions: Param0) -> ::windows::core::Result<Search::StorageFolderQueryResult> {
        let this = &::windows::core::Interface::cast::<Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), queryoptions.into_param().abi(), &mut result__).from_abi::<Search::StorageFolderQueryResult>(result__)
        }
    }
    #[cfg(feature = "Storage_Search")]
    #[doc = "*Required features: `Storage`, `Storage_Search`*"]
    pub fn CreateItemQuery(&self) -> ::windows::core::Result<Search::StorageItemQueryResult> {
        let this = &::windows::core::Interface::cast::<Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Search::StorageItemQueryResult>(result__)
        }
    }
    #[cfg(feature = "Storage_Search")]
    #[doc = "*Required features: `Storage`, `Storage_Search`*"]
    pub fn CreateItemQueryWithOptions<'a, Param0: ::windows::core::IntoParam<'a, Search::QueryOptions>>(&self, queryoptions: Param0) -> ::windows::core::Result<Search::StorageItemQueryResult> {
        let this = &::windows::core::Interface::cast::<Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), queryoptions.into_param().abi(), &mut result__).from_abi::<Search::StorageItemQueryResult>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Search"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Foundation_Collections`, `Storage_Search`*"]
    pub fn GetFilesAsync(&self, query: Search::CommonFileQuery, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFile>>> {
        let this = &::windows::core::Interface::cast::<Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), query, startindex, maxitemstoretrieve, &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFile>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Search"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Foundation_Collections`, `Storage_Search`*"]
    pub fn GetFilesAsyncOverloadDefaultStartAndCount(&self, query: Search::CommonFileQuery) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFile>>> {
        let this = &::windows::core::Interface::cast::<Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), query, &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFile>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Search"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Foundation_Collections`, `Storage_Search`*"]
    pub fn GetFoldersAsync(&self, query: Search::CommonFolderQuery, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFolder>>> {
        let this = &::windows::core::Interface::cast::<Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), query, startindex, maxitemstoretrieve, &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFolder>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Search"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Foundation_Collections`, `Storage_Search`*"]
    pub fn GetFoldersAsyncOverloadDefaultStartAndCount(&self, query: Search::CommonFolderQuery) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFolder>>> {
        let this = &::windows::core::Interface::cast::<Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), query, &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFolder>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Search"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Foundation_Collections`, `Storage_Search`*"]
    pub fn GetItemsAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<IStorageItem>>> {
        let this = &::windows::core::Interface::cast::<Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), startindex, maxitemstoretrieve, &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<IStorageItem>>>(result__)
        }
    }
    #[cfg(feature = "Storage_Search")]
    #[doc = "*Required features: `Storage`, `Storage_Search`*"]
    pub fn AreQueryOptionsSupported<'a, Param0: ::windows::core::IntoParam<'a, Search::QueryOptions>>(&self, queryoptions: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), queryoptions.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Storage_Search")]
    #[doc = "*Required features: `Storage`, `Storage_Search`*"]
    pub fn IsCommonFolderQuerySupported(&self, query: Search::CommonFolderQuery) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), query, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Storage_Search")]
    #[doc = "*Required features: `Storage`, `Storage_Search`*"]
    pub fn IsCommonFileQuerySupported(&self, query: Search::CommonFileQuery) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), query, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn GetFolderFromPathAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(path: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
        Self::IStorageFolderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), path.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
        })
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn TryGetChangeTracker(&self) -> ::windows::core::Result<StorageLibraryChangeTracker> {
        let this = &::windows::core::Interface::cast::<IStorageFolder3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageLibraryChangeTracker>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `System`*"]
    pub fn GetFolderFromPathForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, path: Param1) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
        Self::IStorageFolderStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), user.into_param().abi(), path.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
        })
    }
    pub fn IStorageFolderStatics<R, F: FnOnce(&IStorageFolderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageFolder, IStorageFolderStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IStorageFolderStatics2<R, F: FnOnce(&IStorageFolderStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageFolder, IStorageFolderStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for StorageFolder {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.StorageFolder;{72d1cb78-b3ef-4f75-a80b-6fd9dae2944b})");
}
unsafe impl ::windows::core::Interface for StorageFolder {
    type Vtable = IStorageFolder_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72d1cb78_b3ef_4f75_a80b_6fd9dae2944b);
}
impl ::windows::core::RuntimeName for StorageFolder {
    const NAME: &'static str = "Windows.Storage.StorageFolder";
}
impl ::core::convert::From<StorageFolder> for ::windows::core::IUnknown {
    fn from(value: StorageFolder) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&StorageFolder> for ::windows::core::IUnknown {
    fn from(value: &StorageFolder) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageFolder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageFolder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<StorageFolder> for ::windows::core::IInspectable {
    fn from(value: StorageFolder) -> Self {
        value.0
    }
}
impl ::core::convert::From<&StorageFolder> for ::windows::core::IInspectable {
    fn from(value: &StorageFolder) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageFolder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageFolder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<StorageFolder> for IStorageFolder {
    fn from(value: StorageFolder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageFolder> for IStorageFolder {
    fn from(value: &StorageFolder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageFolder> for StorageFolder {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageFolder> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageFolder> for &StorageFolder {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageFolder> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc = "*Required features: `Storage`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct StorageItemTypes(pub u32);
impl StorageItemTypes {
    pub const None: StorageItemTypes = StorageItemTypes(0u32);
    pub const File: StorageItemTypes = StorageItemTypes(1u32);
    pub const Folder: StorageItemTypes = StorageItemTypes(2u32);
}
impl ::core::convert::From<u32> for StorageItemTypes {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for StorageItemTypes {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for StorageItemTypes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.StorageItemTypes;u4)");
}
impl ::windows::core::DefaultType for StorageItemTypes {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for StorageItemTypes {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for StorageItemTypes {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for StorageItemTypes {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for StorageItemTypes {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for StorageItemTypes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Storage`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct StorageLibrary(pub ::windows::core::IInspectable);
impl StorageLibrary {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn RequestAddFolderAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn RequestRemoveFolderAsync<'a, Param0: ::windows::core::IntoParam<'a, StorageFolder>>(&self, folder: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), folder.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Storage`, `Foundation_Collections`*"]
    pub fn Folders(&self) -> ::windows::core::Result<super::Foundation::Collections::IObservableVector<StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IObservableVector<StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn SaveFolder(&self) -> ::windows::core::Result<StorageFolder> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolder>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn DefinitionChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<StorageLibrary, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn RemoveDefinitionChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn GetLibraryAsync(libraryid: KnownLibraryId) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageLibrary>> {
        Self::IStorageLibraryStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), libraryid, &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageLibrary>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `System`*"]
    pub fn GetLibraryForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::System::User>>(user: Param0, libraryid: KnownLibraryId) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageLibrary>> {
        Self::IStorageLibraryStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), user.into_param().abi(), libraryid, &mut result__).from_abi::<super::Foundation::IAsyncOperation<StorageLibrary>>(result__)
        })
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn ChangeTracker(&self) -> ::windows::core::Result<StorageLibraryChangeTracker> {
        let this = &::windows::core::Interface::cast::<IStorageLibrary2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageLibraryChangeTracker>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn AreFolderSuggestionsAvailableAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IStorageLibrary3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn IStorageLibraryStatics<R, F: FnOnce(&IStorageLibraryStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageLibrary, IStorageLibraryStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IStorageLibraryStatics2<R, F: FnOnce(&IStorageLibraryStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageLibrary, IStorageLibraryStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for StorageLibrary {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.StorageLibrary;{1edd7103-0e5e-4d6c-b5e8-9318983d6a03})");
}
unsafe impl ::windows::core::Interface for StorageLibrary {
    type Vtable = IStorageLibrary_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1edd7103_0e5e_4d6c_b5e8_9318983d6a03);
}
impl ::windows::core::RuntimeName for StorageLibrary {
    const NAME: &'static str = "Windows.Storage.StorageLibrary";
}
impl ::core::convert::From<StorageLibrary> for ::windows::core::IUnknown {
    fn from(value: StorageLibrary) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&StorageLibrary> for ::windows::core::IUnknown {
    fn from(value: &StorageLibrary) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageLibrary {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageLibrary {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<StorageLibrary> for ::windows::core::IInspectable {
    fn from(value: StorageLibrary) -> Self {
        value.0
    }
}
impl ::core::convert::From<&StorageLibrary> for ::windows::core::IInspectable {
    fn from(value: &StorageLibrary) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageLibrary {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageLibrary {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `Storage`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct StorageLibraryChange(pub ::windows::core::IInspectable);
impl StorageLibraryChange {
    #[doc = "*Required features: `Storage`*"]
    pub fn ChangeType(&self) -> ::windows::core::Result<StorageLibraryChangeType> {
        let this = self;
        unsafe {
            let mut result__: StorageLibraryChangeType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageLibraryChangeType>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn PreviousPath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn IsOfType(&self, r#type: StorageItemTypes) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn GetStorageItemAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<IStorageItem>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for StorageLibraryChange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.StorageLibraryChange;{00980b23-2be2-4909-aa48-159f5203a51e})");
}
unsafe impl ::windows::core::Interface for StorageLibraryChange {
    type Vtable = IStorageLibraryChange_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00980b23_2be2_4909_aa48_159f5203a51e);
}
impl ::windows::core::RuntimeName for StorageLibraryChange {
    const NAME: &'static str = "Windows.Storage.StorageLibraryChange";
}
impl ::core::convert::From<StorageLibraryChange> for ::windows::core::IUnknown {
    fn from(value: StorageLibraryChange) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&StorageLibraryChange> for ::windows::core::IUnknown {
    fn from(value: &StorageLibraryChange) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageLibraryChange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageLibraryChange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<StorageLibraryChange> for ::windows::core::IInspectable {
    fn from(value: StorageLibraryChange) -> Self {
        value.0
    }
}
impl ::core::convert::From<&StorageLibraryChange> for ::windows::core::IInspectable {
    fn from(value: &StorageLibraryChange) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageLibraryChange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageLibraryChange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for StorageLibraryChange {}
unsafe impl ::core::marker::Sync for StorageLibraryChange {}
#[doc = "*Required features: `Storage`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct StorageLibraryChangeReader(pub ::windows::core::IInspectable);
impl StorageLibraryChangeReader {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Foundation_Collections`*"]
    pub fn ReadBatchAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageLibraryChange>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageLibraryChange>>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn AcceptChangesAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn GetLastChangeId(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<IStorageLibraryChangeReader2>(self)?;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for StorageLibraryChangeReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.StorageLibraryChangeReader;{f205bc83-fca2-41f9-8954-ee2e991eb96f})");
}
unsafe impl ::windows::core::Interface for StorageLibraryChangeReader {
    type Vtable = IStorageLibraryChangeReader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf205bc83_fca2_41f9_8954_ee2e991eb96f);
}
impl ::windows::core::RuntimeName for StorageLibraryChangeReader {
    const NAME: &'static str = "Windows.Storage.StorageLibraryChangeReader";
}
impl ::core::convert::From<StorageLibraryChangeReader> for ::windows::core::IUnknown {
    fn from(value: StorageLibraryChangeReader) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&StorageLibraryChangeReader> for ::windows::core::IUnknown {
    fn from(value: &StorageLibraryChangeReader) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageLibraryChangeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageLibraryChangeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<StorageLibraryChangeReader> for ::windows::core::IInspectable {
    fn from(value: StorageLibraryChangeReader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&StorageLibraryChangeReader> for ::windows::core::IInspectable {
    fn from(value: &StorageLibraryChangeReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageLibraryChangeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageLibraryChangeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for StorageLibraryChangeReader {}
unsafe impl ::core::marker::Sync for StorageLibraryChangeReader {}
#[doc = "*Required features: `Storage`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct StorageLibraryChangeTracker(pub ::windows::core::IInspectable);
impl StorageLibraryChangeTracker {
    #[doc = "*Required features: `Storage`*"]
    pub fn GetChangeReader(&self) -> ::windows::core::Result<StorageLibraryChangeReader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageLibraryChangeReader>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Enable(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn EnableWithOptions<'a, Param0: ::windows::core::IntoParam<'a, StorageLibraryChangeTrackerOptions>>(&self, options: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStorageLibraryChangeTracker2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), options.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Disable(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStorageLibraryChangeTracker2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for StorageLibraryChangeTracker {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.StorageLibraryChangeTracker;{9e157316-6073-44f6-9681-7492d1286c90})");
}
unsafe impl ::windows::core::Interface for StorageLibraryChangeTracker {
    type Vtable = IStorageLibraryChangeTracker_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e157316_6073_44f6_9681_7492d1286c90);
}
impl ::windows::core::RuntimeName for StorageLibraryChangeTracker {
    const NAME: &'static str = "Windows.Storage.StorageLibraryChangeTracker";
}
impl ::core::convert::From<StorageLibraryChangeTracker> for ::windows::core::IUnknown {
    fn from(value: StorageLibraryChangeTracker) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&StorageLibraryChangeTracker> for ::windows::core::IUnknown {
    fn from(value: &StorageLibraryChangeTracker) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageLibraryChangeTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageLibraryChangeTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<StorageLibraryChangeTracker> for ::windows::core::IInspectable {
    fn from(value: StorageLibraryChangeTracker) -> Self {
        value.0
    }
}
impl ::core::convert::From<&StorageLibraryChangeTracker> for ::windows::core::IInspectable {
    fn from(value: &StorageLibraryChangeTracker) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageLibraryChangeTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageLibraryChangeTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for StorageLibraryChangeTracker {}
unsafe impl ::core::marker::Sync for StorageLibraryChangeTracker {}
#[doc = "*Required features: `Storage`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct StorageLibraryChangeTrackerOptions(pub ::windows::core::IInspectable);
impl StorageLibraryChangeTrackerOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageLibraryChangeTrackerOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn TrackChangeDetails(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn SetTrackChangeDetails(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for StorageLibraryChangeTrackerOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.StorageLibraryChangeTrackerOptions;{bb52bcd4-1a6d-59c0-ad2a-823a20532483})");
}
unsafe impl ::windows::core::Interface for StorageLibraryChangeTrackerOptions {
    type Vtable = IStorageLibraryChangeTrackerOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb52bcd4_1a6d_59c0_ad2a_823a20532483);
}
impl ::windows::core::RuntimeName for StorageLibraryChangeTrackerOptions {
    const NAME: &'static str = "Windows.Storage.StorageLibraryChangeTrackerOptions";
}
impl ::core::convert::From<StorageLibraryChangeTrackerOptions> for ::windows::core::IUnknown {
    fn from(value: StorageLibraryChangeTrackerOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&StorageLibraryChangeTrackerOptions> for ::windows::core::IUnknown {
    fn from(value: &StorageLibraryChangeTrackerOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageLibraryChangeTrackerOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageLibraryChangeTrackerOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<StorageLibraryChangeTrackerOptions> for ::windows::core::IInspectable {
    fn from(value: StorageLibraryChangeTrackerOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&StorageLibraryChangeTrackerOptions> for ::windows::core::IInspectable {
    fn from(value: &StorageLibraryChangeTrackerOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageLibraryChangeTrackerOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageLibraryChangeTrackerOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for StorageLibraryChangeTrackerOptions {}
unsafe impl ::core::marker::Sync for StorageLibraryChangeTrackerOptions {}
#[doc = "*Required features: `Storage`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct StorageLibraryChangeType(pub i32);
impl StorageLibraryChangeType {
    pub const Created: StorageLibraryChangeType = StorageLibraryChangeType(0i32);
    pub const Deleted: StorageLibraryChangeType = StorageLibraryChangeType(1i32);
    pub const MovedOrRenamed: StorageLibraryChangeType = StorageLibraryChangeType(2i32);
    pub const ContentsChanged: StorageLibraryChangeType = StorageLibraryChangeType(3i32);
    pub const MovedOutOfLibrary: StorageLibraryChangeType = StorageLibraryChangeType(4i32);
    pub const MovedIntoLibrary: StorageLibraryChangeType = StorageLibraryChangeType(5i32);
    pub const ContentsReplaced: StorageLibraryChangeType = StorageLibraryChangeType(6i32);
    pub const IndexingStatusChanged: StorageLibraryChangeType = StorageLibraryChangeType(7i32);
    pub const EncryptionChanged: StorageLibraryChangeType = StorageLibraryChangeType(8i32);
    pub const ChangeTrackingLost: StorageLibraryChangeType = StorageLibraryChangeType(9i32);
}
impl ::core::convert::From<i32> for StorageLibraryChangeType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for StorageLibraryChangeType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for StorageLibraryChangeType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.StorageLibraryChangeType;i4)");
}
impl ::windows::core::DefaultType for StorageLibraryChangeType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Storage`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct StorageLibraryLastChangeId(pub ::windows::core::IInspectable);
impl StorageLibraryLastChangeId {
    #[doc = "*Required features: `Storage`*"]
    pub fn Unknown() -> ::windows::core::Result<u64> {
        Self::IStorageLibraryLastChangeIdStatics(|this| unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        })
    }
    pub fn IStorageLibraryLastChangeIdStatics<R, F: FnOnce(&IStorageLibraryLastChangeIdStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageLibraryLastChangeId, IStorageLibraryLastChangeIdStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for StorageLibraryLastChangeId {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.StorageLibraryLastChangeId;{5281826a-bbe1-53bc-82ca-81cc7f039329})");
}
unsafe impl ::windows::core::Interface for StorageLibraryLastChangeId {
    type Vtable = IStorageLibraryLastChangeId_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5281826a_bbe1_53bc_82ca_81cc7f039329);
}
impl ::windows::core::RuntimeName for StorageLibraryLastChangeId {
    const NAME: &'static str = "Windows.Storage.StorageLibraryLastChangeId";
}
impl ::core::convert::From<StorageLibraryLastChangeId> for ::windows::core::IUnknown {
    fn from(value: StorageLibraryLastChangeId) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&StorageLibraryLastChangeId> for ::windows::core::IUnknown {
    fn from(value: &StorageLibraryLastChangeId) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageLibraryLastChangeId {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageLibraryLastChangeId {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<StorageLibraryLastChangeId> for ::windows::core::IInspectable {
    fn from(value: StorageLibraryLastChangeId) -> Self {
        value.0
    }
}
impl ::core::convert::From<&StorageLibraryLastChangeId> for ::windows::core::IInspectable {
    fn from(value: &StorageLibraryLastChangeId) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageLibraryLastChangeId {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageLibraryLastChangeId {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for StorageLibraryLastChangeId {}
unsafe impl ::core::marker::Sync for StorageLibraryLastChangeId {}
#[doc = "*Required features: `Storage`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct StorageOpenOptions(pub u32);
impl StorageOpenOptions {
    pub const None: StorageOpenOptions = StorageOpenOptions(0u32);
    pub const AllowOnlyReaders: StorageOpenOptions = StorageOpenOptions(1u32);
    pub const AllowReadersAndWriters: StorageOpenOptions = StorageOpenOptions(2u32);
}
impl ::core::convert::From<u32> for StorageOpenOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for StorageOpenOptions {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for StorageOpenOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.StorageOpenOptions;u4)");
}
impl ::windows::core::DefaultType for StorageOpenOptions {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for StorageOpenOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for StorageOpenOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for StorageOpenOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for StorageOpenOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for StorageOpenOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Storage`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct StorageProvider(pub ::windows::core::IInspectable);
impl StorageProvider {
    #[doc = "*Required features: `Storage`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn IsPropertySupportedForPartialFileAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, propertycanonicalname: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IStorageProvider2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertycanonicalname.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.StorageProvider;{e705eed4-d478-47d6-ba46-1a8ebe114a20})");
}
unsafe impl ::windows::core::Interface for StorageProvider {
    type Vtable = IStorageProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe705eed4_d478_47d6_ba46_1a8ebe114a20);
}
impl ::windows::core::RuntimeName for StorageProvider {
    const NAME: &'static str = "Windows.Storage.StorageProvider";
}
impl ::core::convert::From<StorageProvider> for ::windows::core::IUnknown {
    fn from(value: StorageProvider) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&StorageProvider> for ::windows::core::IUnknown {
    fn from(value: &StorageProvider) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<StorageProvider> for ::windows::core::IInspectable {
    fn from(value: StorageProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&StorageProvider> for ::windows::core::IInspectable {
    fn from(value: &StorageProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `Storage`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct StorageStreamTransaction(pub ::windows::core::IInspectable);
impl StorageStreamTransaction {
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Storage`, `Storage_Streams`*"]
    pub fn Stream(&self) -> ::windows::core::Result<Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Streams::IRandomAccessStream>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn CommitAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for StorageStreamTransaction {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.StorageStreamTransaction;{f67cf363-a53d-4d94-ae2c-67232d93acdd})");
}
unsafe impl ::windows::core::Interface for StorageStreamTransaction {
    type Vtable = IStorageStreamTransaction_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf67cf363_a53d_4d94_ae2c_67232d93acdd);
}
impl ::windows::core::RuntimeName for StorageStreamTransaction {
    const NAME: &'static str = "Windows.Storage.StorageStreamTransaction";
}
impl ::core::convert::From<StorageStreamTransaction> for ::windows::core::IUnknown {
    fn from(value: StorageStreamTransaction) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&StorageStreamTransaction> for ::windows::core::IUnknown {
    fn from(value: &StorageStreamTransaction) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageStreamTransaction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageStreamTransaction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<StorageStreamTransaction> for ::windows::core::IInspectable {
    fn from(value: StorageStreamTransaction) -> Self {
        value.0
    }
}
impl ::core::convert::From<&StorageStreamTransaction> for ::windows::core::IInspectable {
    fn from(value: &StorageStreamTransaction) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageStreamTransaction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageStreamTransaction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
#[cfg(feature = "Storage_Streams")]
#[doc = "*Required features: `Storage`, `Storage_Streams`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct StreamedFileDataRequest(pub ::windows::core::IInspectable);
#[cfg(feature = "Storage_Streams")]
impl StreamedFileDataRequest {
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_Streams`*"]
    pub fn WriteAsync<'a, Param0: ::windows::core::IntoParam<'a, Streams::IBuffer>>(&self, buffer: Param0) -> ::windows::core::Result<super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Storage`, `Foundation`, `Storage_Streams`*"]
    pub fn FlushAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Storage`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn FailAndClose(&self, failuremode: StreamedFileFailureMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStreamedFileDataRequest>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), failuremode).ok() }
    }
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::windows::core::RuntimeType for StreamedFileDataRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.StreamedFileDataRequest;{905a0fe6-bc53-11df-8c49-001e4fc686da})");
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::windows::core::Interface for StreamedFileDataRequest {
    type Vtable = Streams::IOutputStream_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x905a0fe6_bc53_11df_8c49_001e4fc686da);
}
#[cfg(feature = "Storage_Streams")]
impl ::windows::core::RuntimeName for StreamedFileDataRequest {
    const NAME: &'static str = "Windows.Storage.StreamedFileDataRequest";
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<StreamedFileDataRequest> for ::windows::core::IUnknown {
    fn from(value: StreamedFileDataRequest) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<&StreamedFileDataRequest> for ::windows::core::IUnknown {
    fn from(value: &StreamedFileDataRequest) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StreamedFileDataRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StreamedFileDataRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<StreamedFileDataRequest> for ::windows::core::IInspectable {
    fn from(value: StreamedFileDataRequest) -> Self {
        value.0
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<&StreamedFileDataRequest> for ::windows::core::IInspectable {
    fn from(value: &StreamedFileDataRequest) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StreamedFileDataRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StreamedFileDataRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<StreamedFileDataRequest> for Streams::IOutputStream {
    fn from(value: StreamedFileDataRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<&StreamedFileDataRequest> for Streams::IOutputStream {
    fn from(value: &StreamedFileDataRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, Streams::IOutputStream> for StreamedFileDataRequest {
    fn into_param(self) -> ::windows::core::Param<'a, Streams::IOutputStream> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, Streams::IOutputStream> for &StreamedFileDataRequest {
    fn into_param(self) -> ::windows::core::Param<'a, Streams::IOutputStream> {
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
#[cfg(feature = "Storage_Streams")]
#[doc = "*Required features: `Storage`, `Storage_Streams`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct StreamedFileDataRequestedHandler(::windows::core::IUnknown);
#[cfg(feature = "Storage_Streams")]
impl StreamedFileDataRequestedHandler {
    pub fn new<F: FnMut(&::core::option::Option<StreamedFileDataRequest>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = StreamedFileDataRequestedHandler_box::<F> {
            vtable: &StreamedFileDataRequestedHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Storage`, `Storage_Streams`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, StreamedFileDataRequest>>(&self, stream: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), stream.into_param().abi()).ok() }
    }
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::windows::core::RuntimeType for StreamedFileDataRequestedHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({fef6a824-2fe1-4d07-a35b-b77c50b5f4cc})");
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::windows::core::Interface for StreamedFileDataRequestedHandler {
    type Vtable = StreamedFileDataRequestedHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfef6a824_2fe1_4d07_a35b_b77c50b5f4cc);
}
#[cfg(feature = "Storage_Streams")]
#[repr(C)]
#[doc(hidden)]
pub struct StreamedFileDataRequestedHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, stream: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[cfg(feature = "Storage_Streams")]
#[repr(C)]
struct StreamedFileDataRequestedHandler_box<F: FnMut(&::core::option::Option<StreamedFileDataRequest>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const StreamedFileDataRequestedHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
#[cfg(feature = "Storage_Streams")]
impl<F: FnMut(&::core::option::Option<StreamedFileDataRequest>) -> ::windows::core::Result<()> + 'static> StreamedFileDataRequestedHandler_box<F> {
    const VTABLE: StreamedFileDataRequestedHandler_abi = StreamedFileDataRequestedHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<StreamedFileDataRequestedHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(0x8000_4002)
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
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, stream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&stream as *const <StreamedFileDataRequest as ::windows::core::Abi>::Abi as *const <StreamedFileDataRequest as ::windows::core::DefaultType>::DefaultType)).into()
    }
}
#[doc = "*Required features: `Storage`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct StreamedFileFailureMode(pub i32);
impl StreamedFileFailureMode {
    pub const Failed: StreamedFileFailureMode = StreamedFileFailureMode(0i32);
    pub const CurrentlyUnavailable: StreamedFileFailureMode = StreamedFileFailureMode(1i32);
    pub const Incomplete: StreamedFileFailureMode = StreamedFileFailureMode(2i32);
}
impl ::core::convert::From<i32> for StreamedFileFailureMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for StreamedFileFailureMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for StreamedFileFailureMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.StreamedFileFailureMode;i4)");
}
impl ::windows::core::DefaultType for StreamedFileFailureMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `Storage`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SystemAudioProperties(pub ::windows::core::IInspectable);
impl SystemAudioProperties {
    #[doc = "*Required features: `Storage`*"]
    pub fn EncodingBitrate(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SystemAudioProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.SystemAudioProperties;{3f8f38b7-308c-47e1-924d-8645348e5db7})");
}
unsafe impl ::windows::core::Interface for SystemAudioProperties {
    type Vtable = ISystemAudioProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f8f38b7_308c_47e1_924d_8645348e5db7);
}
impl ::windows::core::RuntimeName for SystemAudioProperties {
    const NAME: &'static str = "Windows.Storage.SystemAudioProperties";
}
impl ::core::convert::From<SystemAudioProperties> for ::windows::core::IUnknown {
    fn from(value: SystemAudioProperties) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SystemAudioProperties> for ::windows::core::IUnknown {
    fn from(value: &SystemAudioProperties) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemAudioProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemAudioProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SystemAudioProperties> for ::windows::core::IInspectable {
    fn from(value: SystemAudioProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SystemAudioProperties> for ::windows::core::IInspectable {
    fn from(value: &SystemAudioProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemAudioProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemAudioProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SystemAudioProperties {}
unsafe impl ::core::marker::Sync for SystemAudioProperties {}
#[doc = "*Required features: `Storage`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SystemDataPaths(pub ::windows::core::IInspectable);
impl SystemDataPaths {
    #[doc = "*Required features: `Storage`*"]
    pub fn Fonts(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn ProgramData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Public(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn PublicDesktop(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn PublicDocuments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn PublicDownloads(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn PublicMusic(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn PublicPictures(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn PublicVideos(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn System(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn SystemHost(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn SystemX86(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn SystemX64(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn SystemArm(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn UserProfiles(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Windows(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn GetDefault() -> ::windows::core::Result<SystemDataPaths> {
        Self::ISystemDataPathsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemDataPaths>(result__)
        })
    }
    pub fn ISystemDataPathsStatics<R, F: FnOnce(&ISystemDataPathsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SystemDataPaths, ISystemDataPathsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SystemDataPaths {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.SystemDataPaths;{e32abf70-d8fa-45ec-a942-d2e26fb60ba5})");
}
unsafe impl ::windows::core::Interface for SystemDataPaths {
    type Vtable = ISystemDataPaths_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe32abf70_d8fa_45ec_a942_d2e26fb60ba5);
}
impl ::windows::core::RuntimeName for SystemDataPaths {
    const NAME: &'static str = "Windows.Storage.SystemDataPaths";
}
impl ::core::convert::From<SystemDataPaths> for ::windows::core::IUnknown {
    fn from(value: SystemDataPaths) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SystemDataPaths> for ::windows::core::IUnknown {
    fn from(value: &SystemDataPaths) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemDataPaths {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemDataPaths {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SystemDataPaths> for ::windows::core::IInspectable {
    fn from(value: SystemDataPaths) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SystemDataPaths> for ::windows::core::IInspectable {
    fn from(value: &SystemDataPaths) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemDataPaths {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemDataPaths {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SystemDataPaths {}
unsafe impl ::core::marker::Sync for SystemDataPaths {}
#[doc = "*Required features: `Storage`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SystemGPSProperties(pub ::windows::core::IInspectable);
impl SystemGPSProperties {
    #[doc = "*Required features: `Storage`*"]
    pub fn LatitudeDecimal(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn LongitudeDecimal(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SystemGPSProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.SystemGPSProperties;{c0f46eb4-c174-481a-bc25-921986f6a6f3})");
}
unsafe impl ::windows::core::Interface for SystemGPSProperties {
    type Vtable = ISystemGPSProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0f46eb4_c174_481a_bc25_921986f6a6f3);
}
impl ::windows::core::RuntimeName for SystemGPSProperties {
    const NAME: &'static str = "Windows.Storage.SystemGPSProperties";
}
impl ::core::convert::From<SystemGPSProperties> for ::windows::core::IUnknown {
    fn from(value: SystemGPSProperties) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SystemGPSProperties> for ::windows::core::IUnknown {
    fn from(value: &SystemGPSProperties) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemGPSProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemGPSProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SystemGPSProperties> for ::windows::core::IInspectable {
    fn from(value: SystemGPSProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SystemGPSProperties> for ::windows::core::IInspectable {
    fn from(value: &SystemGPSProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemGPSProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemGPSProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SystemGPSProperties {}
unsafe impl ::core::marker::Sync for SystemGPSProperties {}
#[doc = "*Required features: `Storage`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SystemImageProperties(pub ::windows::core::IInspectable);
impl SystemImageProperties {
    #[doc = "*Required features: `Storage`*"]
    pub fn HorizontalSize(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn VerticalSize(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SystemImageProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.SystemImageProperties;{011b2e30-8b39-4308-bea1-e8aa61e47826})");
}
unsafe impl ::windows::core::Interface for SystemImageProperties {
    type Vtable = ISystemImageProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x011b2e30_8b39_4308_bea1_e8aa61e47826);
}
impl ::windows::core::RuntimeName for SystemImageProperties {
    const NAME: &'static str = "Windows.Storage.SystemImageProperties";
}
impl ::core::convert::From<SystemImageProperties> for ::windows::core::IUnknown {
    fn from(value: SystemImageProperties) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SystemImageProperties> for ::windows::core::IUnknown {
    fn from(value: &SystemImageProperties) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemImageProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemImageProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SystemImageProperties> for ::windows::core::IInspectable {
    fn from(value: SystemImageProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SystemImageProperties> for ::windows::core::IInspectable {
    fn from(value: &SystemImageProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemImageProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemImageProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SystemImageProperties {}
unsafe impl ::core::marker::Sync for SystemImageProperties {}
#[doc = "*Required features: `Storage`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SystemMediaProperties(pub ::windows::core::IInspectable);
impl SystemMediaProperties {
    #[doc = "*Required features: `Storage`*"]
    pub fn Duration(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Producer(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Publisher(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn SubTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Writer(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Year(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SystemMediaProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.SystemMediaProperties;{a42b3316-8415-40dc-8c44-98361d235430})");
}
unsafe impl ::windows::core::Interface for SystemMediaProperties {
    type Vtable = ISystemMediaProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa42b3316_8415_40dc_8c44_98361d235430);
}
impl ::windows::core::RuntimeName for SystemMediaProperties {
    const NAME: &'static str = "Windows.Storage.SystemMediaProperties";
}
impl ::core::convert::From<SystemMediaProperties> for ::windows::core::IUnknown {
    fn from(value: SystemMediaProperties) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SystemMediaProperties> for ::windows::core::IUnknown {
    fn from(value: &SystemMediaProperties) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemMediaProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemMediaProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SystemMediaProperties> for ::windows::core::IInspectable {
    fn from(value: SystemMediaProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SystemMediaProperties> for ::windows::core::IInspectable {
    fn from(value: &SystemMediaProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemMediaProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemMediaProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SystemMediaProperties {}
unsafe impl ::core::marker::Sync for SystemMediaProperties {}
#[doc = "*Required features: `Storage`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SystemMusicProperties(pub ::windows::core::IInspectable);
impl SystemMusicProperties {
    #[doc = "*Required features: `Storage`*"]
    pub fn AlbumArtist(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn AlbumTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Artist(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Composer(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Conductor(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn DisplayArtist(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Genre(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn TrackNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SystemMusicProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.SystemMusicProperties;{b47988d5-67af-4bc3-8d39-5b89022026a1})");
}
unsafe impl ::windows::core::Interface for SystemMusicProperties {
    type Vtable = ISystemMusicProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb47988d5_67af_4bc3_8d39_5b89022026a1);
}
impl ::windows::core::RuntimeName for SystemMusicProperties {
    const NAME: &'static str = "Windows.Storage.SystemMusicProperties";
}
impl ::core::convert::From<SystemMusicProperties> for ::windows::core::IUnknown {
    fn from(value: SystemMusicProperties) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SystemMusicProperties> for ::windows::core::IUnknown {
    fn from(value: &SystemMusicProperties) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemMusicProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemMusicProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SystemMusicProperties> for ::windows::core::IInspectable {
    fn from(value: SystemMusicProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SystemMusicProperties> for ::windows::core::IInspectable {
    fn from(value: &SystemMusicProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemMusicProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemMusicProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SystemMusicProperties {}
unsafe impl ::core::marker::Sync for SystemMusicProperties {}
#[doc = "*Required features: `Storage`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SystemPhotoProperties(pub ::windows::core::IInspectable);
impl SystemPhotoProperties {
    #[doc = "*Required features: `Storage`*"]
    pub fn CameraManufacturer(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn CameraModel(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn DateTaken(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Orientation(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn PeopleNames(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SystemPhotoProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.SystemPhotoProperties;{4734fc3d-ab21-4424-b735-f4353a56c8fc})");
}
unsafe impl ::windows::core::Interface for SystemPhotoProperties {
    type Vtable = ISystemPhotoProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4734fc3d_ab21_4424_b735_f4353a56c8fc);
}
impl ::windows::core::RuntimeName for SystemPhotoProperties {
    const NAME: &'static str = "Windows.Storage.SystemPhotoProperties";
}
impl ::core::convert::From<SystemPhotoProperties> for ::windows::core::IUnknown {
    fn from(value: SystemPhotoProperties) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SystemPhotoProperties> for ::windows::core::IUnknown {
    fn from(value: &SystemPhotoProperties) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemPhotoProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemPhotoProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SystemPhotoProperties> for ::windows::core::IInspectable {
    fn from(value: SystemPhotoProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SystemPhotoProperties> for ::windows::core::IInspectable {
    fn from(value: &SystemPhotoProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemPhotoProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemPhotoProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SystemPhotoProperties {}
unsafe impl ::core::marker::Sync for SystemPhotoProperties {}
#[doc = "*Required features: `Storage`*"]
pub struct SystemProperties {}
impl SystemProperties {
    #[doc = "*Required features: `Storage`*"]
    pub fn Author() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISystemProperties(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Comment() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISystemProperties(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn ItemNameDisplay() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISystemProperties(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Keywords() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISystemProperties(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Rating() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISystemProperties(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Title() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISystemProperties(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Audio() -> ::windows::core::Result<SystemAudioProperties> {
        Self::ISystemProperties(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemAudioProperties>(result__)
        })
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn GPS() -> ::windows::core::Result<SystemGPSProperties> {
        Self::ISystemProperties(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemGPSProperties>(result__)
        })
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Media() -> ::windows::core::Result<SystemMediaProperties> {
        Self::ISystemProperties(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemMediaProperties>(result__)
        })
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Music() -> ::windows::core::Result<SystemMusicProperties> {
        Self::ISystemProperties(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemMusicProperties>(result__)
        })
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Photo() -> ::windows::core::Result<SystemPhotoProperties> {
        Self::ISystemProperties(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemPhotoProperties>(result__)
        })
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Video() -> ::windows::core::Result<SystemVideoProperties> {
        Self::ISystemProperties(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemVideoProperties>(result__)
        })
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Image() -> ::windows::core::Result<SystemImageProperties> {
        Self::ISystemProperties(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemImageProperties>(result__)
        })
    }
    pub fn ISystemProperties<R, F: FnOnce(&ISystemProperties) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SystemProperties, ISystemProperties> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for SystemProperties {
    const NAME: &'static str = "Windows.Storage.SystemProperties";
}
#[doc = "*Required features: `Storage`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SystemVideoProperties(pub ::windows::core::IInspectable);
impl SystemVideoProperties {
    #[doc = "*Required features: `Storage`*"]
    pub fn Director(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn FrameHeight(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn FrameWidth(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Orientation(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn TotalBitrate(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SystemVideoProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.SystemVideoProperties;{2040f715-67f8-4322-9b80-4fa9fefb83e8})");
}
unsafe impl ::windows::core::Interface for SystemVideoProperties {
    type Vtable = ISystemVideoProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2040f715_67f8_4322_9b80_4fa9fefb83e8);
}
impl ::windows::core::RuntimeName for SystemVideoProperties {
    const NAME: &'static str = "Windows.Storage.SystemVideoProperties";
}
impl ::core::convert::From<SystemVideoProperties> for ::windows::core::IUnknown {
    fn from(value: SystemVideoProperties) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SystemVideoProperties> for ::windows::core::IUnknown {
    fn from(value: &SystemVideoProperties) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemVideoProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemVideoProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SystemVideoProperties> for ::windows::core::IInspectable {
    fn from(value: SystemVideoProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SystemVideoProperties> for ::windows::core::IInspectable {
    fn from(value: &SystemVideoProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemVideoProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemVideoProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SystemVideoProperties {}
unsafe impl ::core::marker::Sync for SystemVideoProperties {}
#[doc = "*Required features: `Storage`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UserDataPaths(pub ::windows::core::IInspectable);
impl UserDataPaths {
    #[doc = "*Required features: `Storage`*"]
    pub fn CameraRoll(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Cookies(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Desktop(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Documents(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Downloads(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Favorites(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn History(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn InternetCache(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn LocalAppData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn LocalAppDataLow(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Music(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Pictures(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Profile(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Recent(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn RoamingAppData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn SavedPictures(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Screenshots(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Templates(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn Videos(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Storage`, `System`*"]
    pub fn GetForUser<'a, Param0: ::windows::core::IntoParam<'a, super::System::User>>(user: Param0) -> ::windows::core::Result<UserDataPaths> {
        Self::IUserDataPathsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<UserDataPaths>(result__)
        })
    }
    #[doc = "*Required features: `Storage`*"]
    pub fn GetDefault() -> ::windows::core::Result<UserDataPaths> {
        Self::IUserDataPathsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataPaths>(result__)
        })
    }
    pub fn IUserDataPathsStatics<R, F: FnOnce(&IUserDataPathsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UserDataPaths, IUserDataPathsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataPaths {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.UserDataPaths;{f9c53912-abc4-46ff-8a2b-dc9d7fa6e52f})");
}
unsafe impl ::windows::core::Interface for UserDataPaths {
    type Vtable = IUserDataPaths_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9c53912_abc4_46ff_8a2b_dc9d7fa6e52f);
}
impl ::windows::core::RuntimeName for UserDataPaths {
    const NAME: &'static str = "Windows.Storage.UserDataPaths";
}
impl ::core::convert::From<UserDataPaths> for ::windows::core::IUnknown {
    fn from(value: UserDataPaths) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UserDataPaths> for ::windows::core::IUnknown {
    fn from(value: &UserDataPaths) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataPaths {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDataPaths {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UserDataPaths> for ::windows::core::IInspectable {
    fn from(value: UserDataPaths) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UserDataPaths> for ::windows::core::IInspectable {
    fn from(value: &UserDataPaths) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataPaths {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDataPaths {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for UserDataPaths {}
unsafe impl ::core::marker::Sync for UserDataPaths {}
