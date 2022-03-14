#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISocialDashboardItemUpdater(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISocialDashboardItemUpdater {
    type Vtable = ISocialDashboardItemUpdater_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3cde9dc9_4800_46cd_869b_1973ec685bde);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISocialDashboardItemUpdater_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub OwnerRemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OwnerRemoteId: usize,
    #[cfg(feature = "deprecated")]
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Content: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Timestamp: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetTimestamp: usize,
    #[cfg(feature = "deprecated")]
    pub SetThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetThumbnail: usize,
    #[cfg(feature = "deprecated")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Thumbnail: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CommitAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    CommitAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub TargetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    TargetUri: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetTargetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetTargetUri: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISocialFeedUpdater(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISocialFeedUpdater {
    type Vtable = ISocialFeedUpdater_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a0c0aa7_ed89_4bd5_a8d9_15f4d9861c10);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISocialFeedUpdater_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub OwnerRemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OwnerRemoteId: usize,
    #[cfg(feature = "deprecated")]
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::SocialFeedKind) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Kind: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub Items: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    Items: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CommitAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    CommitAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISocialInfoProviderManagerStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISocialInfoProviderManagerStatics {
    type Vtable = ISocialInfoProviderManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b88e52b_7787_48d6_aa12_d8e8f47ab85a);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISocialInfoProviderManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CreateSocialFeedUpdaterAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: super::SocialFeedKind, mode: super::SocialFeedUpdateMode, ownerremoteid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    CreateSocialFeedUpdaterAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CreateDashboardItemUpdaterAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ownerremoteid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    CreateDashboardItemUpdaterAsync: usize,
    #[cfg(feature = "deprecated")]
    pub UpdateBadgeCountValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemremoteid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, newcount: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    UpdateBadgeCountValue: usize,
    #[cfg(feature = "deprecated")]
    pub ReportNewContentAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contactremoteid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, kind: super::SocialFeedKind) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ReportNewContentAvailable: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ProvisionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ProvisionAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub DeprovisionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    DeprovisionAsync: usize,
}
#[doc = "*Required features: `\"ApplicationModel_SocialInfo_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SocialDashboardItemUpdater(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SocialDashboardItemUpdater {
    #[doc = "*Required features: `\"ApplicationModel_SocialInfo_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn OwnerRemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OwnerRemoteId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_SocialInfo_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Content(&self) -> ::windows::core::Result<super::SocialFeedContent> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Content)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::SocialFeedContent>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_SocialInfo_Provider\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Timestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_SocialInfo_Provider\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetTimestamp<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTimestamp)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_SocialInfo_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetThumbnail<'a, Param0: ::windows::core::IntoParam<'a, super::SocialItemThumbnail>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetThumbnail)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_SocialInfo_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Thumbnail(&self) -> ::windows::core::Result<super::SocialItemThumbnail> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Thumbnail)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::SocialItemThumbnail>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_SocialInfo_Provider\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn CommitAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CommitAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_SocialInfo_Provider\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn TargetUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TargetUri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_SocialInfo_Provider\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetTargetUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTargetUri)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SocialDashboardItemUpdater {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SocialDashboardItemUpdater {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SocialDashboardItemUpdater {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SocialDashboardItemUpdater {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocialDashboardItemUpdater").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SocialDashboardItemUpdater {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.SocialInfo.Provider.SocialDashboardItemUpdater;{3cde9dc9-4800-46cd-869b-1973ec685bde})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SocialDashboardItemUpdater {
    type Vtable = ISocialDashboardItemUpdater_Vtbl;
    const IID: ::windows::core::GUID = <ISocialDashboardItemUpdater as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SocialDashboardItemUpdater {
    const NAME: &'static str = "Windows.ApplicationModel.SocialInfo.Provider.SocialDashboardItemUpdater";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SocialDashboardItemUpdater> for ::windows::core::IUnknown {
    fn from(value: SocialDashboardItemUpdater) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SocialDashboardItemUpdater> for ::windows::core::IUnknown {
    fn from(value: &SocialDashboardItemUpdater) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SocialDashboardItemUpdater {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SocialDashboardItemUpdater {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SocialDashboardItemUpdater> for ::windows::core::IInspectable {
    fn from(value: SocialDashboardItemUpdater) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SocialDashboardItemUpdater> for ::windows::core::IInspectable {
    fn from(value: &SocialDashboardItemUpdater) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SocialDashboardItemUpdater {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SocialDashboardItemUpdater {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for SocialDashboardItemUpdater {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for SocialDashboardItemUpdater {}
#[doc = "*Required features: `\"ApplicationModel_SocialInfo_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SocialFeedUpdater(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SocialFeedUpdater {
    #[doc = "*Required features: `\"ApplicationModel_SocialInfo_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn OwnerRemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OwnerRemoteId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_SocialInfo_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Kind(&self) -> ::windows::core::Result<super::SocialFeedKind> {
        let this = self;
        unsafe {
            let mut result__: super::SocialFeedKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::SocialFeedKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_SocialInfo_Provider\"`, `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::SocialFeedItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Items)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<super::SocialFeedItem>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_SocialInfo_Provider\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn CommitAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CommitAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SocialFeedUpdater {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SocialFeedUpdater {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SocialFeedUpdater {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SocialFeedUpdater {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocialFeedUpdater").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for SocialFeedUpdater {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.SocialInfo.Provider.SocialFeedUpdater;{7a0c0aa7-ed89-4bd5-a8d9-15f4d9861c10})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SocialFeedUpdater {
    type Vtable = ISocialFeedUpdater_Vtbl;
    const IID: ::windows::core::GUID = <ISocialFeedUpdater as ::windows::core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SocialFeedUpdater {
    const NAME: &'static str = "Windows.ApplicationModel.SocialInfo.Provider.SocialFeedUpdater";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SocialFeedUpdater> for ::windows::core::IUnknown {
    fn from(value: SocialFeedUpdater) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SocialFeedUpdater> for ::windows::core::IUnknown {
    fn from(value: &SocialFeedUpdater) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SocialFeedUpdater {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SocialFeedUpdater {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<SocialFeedUpdater> for ::windows::core::IInspectable {
    fn from(value: SocialFeedUpdater) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&SocialFeedUpdater> for ::windows::core::IInspectable {
    fn from(value: &SocialFeedUpdater) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SocialFeedUpdater {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SocialFeedUpdater {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for SocialFeedUpdater {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for SocialFeedUpdater {}
#[doc = "*Required features: `\"ApplicationModel_SocialInfo_Provider\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
pub struct SocialInfoProviderManager {}
#[cfg(feature = "deprecated")]
impl SocialInfoProviderManager {
    #[doc = "*Required features: `\"ApplicationModel_SocialInfo_Provider\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn CreateSocialFeedUpdaterAsync<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(kind: super::SocialFeedKind, mode: super::SocialFeedUpdateMode, ownerremoteid: Param2) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<SocialFeedUpdater>> {
        Self::ISocialInfoProviderManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateSocialFeedUpdaterAsync)(::core::mem::transmute_copy(this), kind, mode, ownerremoteid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<SocialFeedUpdater>>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_SocialInfo_Provider\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn CreateDashboardItemUpdaterAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(ownerremoteid: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<SocialDashboardItemUpdater>> {
        Self::ISocialInfoProviderManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateDashboardItemUpdaterAsync)(::core::mem::transmute_copy(this), ownerremoteid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<SocialDashboardItemUpdater>>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_SocialInfo_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn UpdateBadgeCountValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(itemremoteid: Param0, newcount: i32) -> ::windows::core::Result<()> {
        Self::ISocialInfoProviderManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).UpdateBadgeCountValue)(::core::mem::transmute_copy(this), itemremoteid.into_param().abi(), newcount).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel_SocialInfo_Provider\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ReportNewContentAvailable<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(contactremoteid: Param0, kind: super::SocialFeedKind) -> ::windows::core::Result<()> {
        Self::ISocialInfoProviderManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).ReportNewContentAvailable)(::core::mem::transmute_copy(this), contactremoteid.into_param().abi(), kind).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel_SocialInfo_Provider\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn ProvisionAsync() -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        Self::ISocialInfoProviderManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProvisionAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_SocialInfo_Provider\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn DeprovisionAsync() -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::ISocialInfoProviderManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeprovisionAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn ISocialInfoProviderManagerStatics<R, F: FnOnce(&ISocialInfoProviderManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SocialInfoProviderManager, ISocialInfoProviderManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SocialInfoProviderManager {
    const NAME: &'static str = "Windows.ApplicationModel.SocialInfo.Provider.SocialInfoProviderManager";
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
