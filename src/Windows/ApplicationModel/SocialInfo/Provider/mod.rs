#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct ISocialDashboardItemUpdater(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISocialDashboardItemUpdater {
    type Vtable = ISocialDashboardItemUpdater_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3cde9dc9_4800_46cd_869b_1973ec685bde);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISocialDashboardItemUpdater_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISocialFeedUpdater(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISocialFeedUpdater {
    type Vtable = ISocialFeedUpdater_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7a0c0aa7_ed89_4bd5_a8d9_15f4d9861c10);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISocialFeedUpdater_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::SocialFeedKind) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISocialInfoProviderManagerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISocialInfoProviderManagerStatics {
    type Vtable = ISocialInfoProviderManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1b88e52b_7787_48d6_aa12_d8e8f47ab85a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISocialInfoProviderManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, kind: super::SocialFeedKind, mode: super::SocialFeedUpdateMode, ownerremoteid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ownerremoteid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itemremoteid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, newcount: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contactremoteid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, kind: super::SocialFeedKind) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SocialDashboardItemUpdater(pub ::windows::runtime::IInspectable);
impl SocialDashboardItemUpdater {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`*"]
    pub fn OwnerRemoteId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`*"]
    pub fn Content(&self) -> ::windows::runtime::Result<super::SocialFeedContent> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::SocialFeedContent>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`, `Foundation`*"]
    pub fn SetTimestamp<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`*"]
    pub fn SetThumbnail<'a, Param0: ::windows::runtime::IntoParam<'a, super::SocialItemThumbnail>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`*"]
    pub fn Thumbnail(&self) -> ::windows::runtime::Result<super::SocialItemThumbnail> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::SocialItemThumbnail>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`, `Foundation`*"]
    pub fn CommitAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`, `Foundation`*"]
    pub fn TargetUri(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`, `Foundation`*"]
    pub fn SetTargetUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SocialDashboardItemUpdater {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.SocialInfo.Provider.SocialDashboardItemUpdater;{3cde9dc9-4800-46cd-869b-1973ec685bde})");
}
unsafe impl ::windows::runtime::Interface for SocialDashboardItemUpdater {
    type Vtable = ISocialDashboardItemUpdater_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3cde9dc9_4800_46cd_869b_1973ec685bde);
}
impl ::windows::runtime::RuntimeName for SocialDashboardItemUpdater {
    const NAME: &'static str = "Windows.ApplicationModel.SocialInfo.Provider.SocialDashboardItemUpdater";
}
impl ::core::convert::From<SocialDashboardItemUpdater> for ::windows::runtime::IUnknown {
    fn from(value: SocialDashboardItemUpdater) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SocialDashboardItemUpdater> for ::windows::runtime::IUnknown {
    fn from(value: &SocialDashboardItemUpdater) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SocialDashboardItemUpdater {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SocialDashboardItemUpdater {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SocialDashboardItemUpdater> for ::windows::runtime::IInspectable {
    fn from(value: SocialDashboardItemUpdater) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SocialDashboardItemUpdater> for ::windows::runtime::IInspectable {
    fn from(value: &SocialDashboardItemUpdater) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SocialDashboardItemUpdater {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SocialDashboardItemUpdater {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SocialDashboardItemUpdater {}
unsafe impl ::core::marker::Sync for SocialDashboardItemUpdater {}
#[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SocialFeedUpdater(pub ::windows::runtime::IInspectable);
impl SocialFeedUpdater {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`*"]
    pub fn OwnerRemoteId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<super::SocialFeedKind> {
        let this = self;
        unsafe {
            let mut result__: super::SocialFeedKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::SocialFeedKind>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`, `Foundation_Collections`*"]
    pub fn Items(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVector<super::SocialFeedItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<super::SocialFeedItem>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`, `Foundation`*"]
    pub fn CommitAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SocialFeedUpdater {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.SocialInfo.Provider.SocialFeedUpdater;{7a0c0aa7-ed89-4bd5-a8d9-15f4d9861c10})");
}
unsafe impl ::windows::runtime::Interface for SocialFeedUpdater {
    type Vtable = ISocialFeedUpdater_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7a0c0aa7_ed89_4bd5_a8d9_15f4d9861c10);
}
impl ::windows::runtime::RuntimeName for SocialFeedUpdater {
    const NAME: &'static str = "Windows.ApplicationModel.SocialInfo.Provider.SocialFeedUpdater";
}
impl ::core::convert::From<SocialFeedUpdater> for ::windows::runtime::IUnknown {
    fn from(value: SocialFeedUpdater) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SocialFeedUpdater> for ::windows::runtime::IUnknown {
    fn from(value: &SocialFeedUpdater) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SocialFeedUpdater {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SocialFeedUpdater {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SocialFeedUpdater> for ::windows::runtime::IInspectable {
    fn from(value: SocialFeedUpdater) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SocialFeedUpdater> for ::windows::runtime::IInspectable {
    fn from(value: &SocialFeedUpdater) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SocialFeedUpdater {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SocialFeedUpdater {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SocialFeedUpdater {}
unsafe impl ::core::marker::Sync for SocialFeedUpdater {}
#[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`*"]
pub struct SocialInfoProviderManager {}
impl SocialInfoProviderManager {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`, `Foundation`*"]
    pub fn CreateSocialFeedUpdaterAsync<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(kind: super::SocialFeedKind, mode: super::SocialFeedUpdateMode, ownerremoteid: Param2) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<SocialFeedUpdater>> {
        Self::ISocialInfoProviderManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), kind, mode, ownerremoteid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<SocialFeedUpdater>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`, `Foundation`*"]
    pub fn CreateDashboardItemUpdaterAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(ownerremoteid: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<SocialDashboardItemUpdater>> {
        Self::ISocialInfoProviderManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), ownerremoteid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<SocialDashboardItemUpdater>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`*"]
    pub fn UpdateBadgeCountValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(itemremoteid: Param0, newcount: i32) -> ::windows::runtime::Result<()> {
        Self::ISocialInfoProviderManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), itemremoteid.into_param().abi(), newcount).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`*"]
    pub fn ReportNewContentAvailable<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(contactremoteid: Param0, kind: super::SocialFeedKind) -> ::windows::runtime::Result<()> {
        Self::ISocialInfoProviderManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), contactremoteid.into_param().abi(), kind).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`, `Foundation`*"]
    pub fn ProvisionAsync() -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        Self::ISocialInfoProviderManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`, `Foundation`*"]
    pub fn DeprovisionAsync() -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        Self::ISocialInfoProviderManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    pub fn ISocialInfoProviderManagerStatics<R, F: FnOnce(&ISocialInfoProviderManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SocialInfoProviderManager, ISocialInfoProviderManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for SocialInfoProviderManager {
    const NAME: &'static str = "Windows.ApplicationModel.SocialInfo.Provider.SocialInfoProviderManager";
}
