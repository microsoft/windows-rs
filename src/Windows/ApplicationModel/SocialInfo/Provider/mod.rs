#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISocialDashboardItemUpdater(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISocialDashboardItemUpdater {
    type Vtable = ISocialDashboardItemUpdater_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1021222345, 18432, 18125, [134, 155, 25, 115, 236, 104, 91, 222]);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISocialFeedUpdater(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISocialFeedUpdater {
    type Vtable = ISocialFeedUpdater_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2047609511, 60809, 19413, [168, 217, 21, 244, 217, 134, 28, 16]);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::SocialFeedKind) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISocialInfoProviderManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISocialInfoProviderManagerStatics {
    type Vtable = ISocialInfoProviderManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(461956395, 30599, 18646, [170, 18, 216, 232, 244, 122, 184, 90]);
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
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, kind: super::SocialFeedKind, mode: super::SocialFeedUpdateMode, ownerremoteid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ownerremoteid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itemremoteid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, newcount: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contactremoteid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, kind: super::SocialFeedKind) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct SocialDashboardItemUpdater(::windows::runtime::IInspectable);
impl SocialDashboardItemUpdater {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`*"]
    pub fn OwnerRemoteId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`*"]
    pub fn Content(&self) -> ::windows::runtime::Result<super::SocialFeedContent> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::SocialFeedContent>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`, `Foundation`*"]
    pub fn SetTimestamp<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`*"]
    pub fn SetThumbnail<'a, Param0: ::windows::runtime::IntoParam<'a, super::SocialItemThumbnail>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`*"]
    pub fn Thumbnail(&self) -> ::windows::runtime::Result<super::SocialItemThumbnail> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::SocialItemThumbnail>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`, `Foundation`*"]
    pub fn CommitAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`, `Foundation`*"]
    pub fn TargetUri(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`, `Foundation`*"]
    pub fn SetTargetUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SocialDashboardItemUpdater {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.SocialInfo.Provider.SocialDashboardItemUpdater;{3cde9dc9-4800-46cd-869b-1973ec685bde})");
}
unsafe impl ::windows::runtime::Interface for SocialDashboardItemUpdater {
    type Vtable = ISocialDashboardItemUpdater_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1021222345, 18432, 18125, [134, 155, 25, 115, 236, 104, 91, 222]);
}
impl ::windows::runtime::RuntimeName for SocialDashboardItemUpdater {
    const NAME: &'static str = "Windows.ApplicationModel.SocialInfo.Provider.SocialDashboardItemUpdater";
}
unsafe impl ::std::marker::Send for SocialDashboardItemUpdater {}
unsafe impl ::std::marker::Sync for SocialDashboardItemUpdater {}
#[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct SocialFeedUpdater(::windows::runtime::IInspectable);
impl SocialFeedUpdater {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`*"]
    pub fn OwnerRemoteId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<super::SocialFeedKind> {
        let this = self;
        unsafe {
            let mut result__: super::SocialFeedKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::SocialFeedKind>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`, `Foundation_Collections`*"]
    pub fn Items(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVector<super::SocialFeedItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<super::SocialFeedItem>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`, `Foundation`*"]
    pub fn CommitAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SocialFeedUpdater {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.SocialInfo.Provider.SocialFeedUpdater;{7a0c0aa7-ed89-4bd5-a8d9-15f4d9861c10})");
}
unsafe impl ::windows::runtime::Interface for SocialFeedUpdater {
    type Vtable = ISocialFeedUpdater_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2047609511, 60809, 19413, [168, 217, 21, 244, 217, 134, 28, 16]);
}
impl ::windows::runtime::RuntimeName for SocialFeedUpdater {
    const NAME: &'static str = "Windows.ApplicationModel.SocialInfo.Provider.SocialFeedUpdater";
}
unsafe impl ::std::marker::Send for SocialFeedUpdater {}
unsafe impl ::std::marker::Sync for SocialFeedUpdater {}
#[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`*"]
pub struct SocialInfoProviderManager {}
impl SocialInfoProviderManager {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`, `Foundation`*"]
    pub fn CreateSocialFeedUpdaterAsync<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(kind: super::SocialFeedKind, mode: super::SocialFeedUpdateMode, ownerremoteid: Param2) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<SocialFeedUpdater>> {
        Self::ISocialInfoProviderManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), kind, mode, ownerremoteid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<SocialFeedUpdater>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`, `Foundation`*"]
    pub fn CreateDashboardItemUpdaterAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(ownerremoteid: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<SocialDashboardItemUpdater>> {
        Self::ISocialInfoProviderManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), ownerremoteid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<SocialDashboardItemUpdater>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`*"]
    pub fn UpdateBadgeCountValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(itemremoteid: Param0, newcount: i32) -> ::windows::runtime::Result<()> {
        Self::ISocialInfoProviderManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), itemremoteid.into_param().abi(), newcount).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`*"]
    pub fn ReportNewContentAvailable<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(contactremoteid: Param0, kind: super::SocialFeedKind) -> ::windows::runtime::Result<()> {
        Self::ISocialInfoProviderManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), contactremoteid.into_param().abi(), kind).ok() })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`, `Foundation`*"]
    pub fn ProvisionAsync() -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        Self::ISocialInfoProviderManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_SocialInfo_Provider`, `Foundation`*"]
    pub fn DeprovisionAsync() -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        Self::ISocialInfoProviderManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
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
