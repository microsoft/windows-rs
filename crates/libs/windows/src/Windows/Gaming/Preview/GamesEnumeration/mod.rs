#[doc = "*Required features: `\"Gaming_Preview_GamesEnumeration\"`*"]
#[repr(transparent)]
pub struct IGameListEntry(::windows::core::IUnknown);
impl IGameListEntry {
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    #[cfg(feature = "ApplicationModel")]
    pub fn DisplayInfo(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::AppDisplayInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Category(&self) -> ::windows::core::Result<GameListCategory> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Category)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCategoryAsync(&self, value: GameListCategory) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetCategoryAsync)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IGameListEntry, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IGameListEntry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGameListEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGameListEntry {}
impl ::core::fmt::Debug for IGameListEntry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGameListEntry").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IGameListEntry {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{735924d3-811f-4494-b69c-c641a0c61543}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IGameListEntry {
    type Vtable = IGameListEntry_Vtbl;
}
unsafe impl ::windows::core::Interface for IGameListEntry {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x735924d3_811f_4494_b69c_c641a0c61543);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameListEntry_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel")]
    pub DisplayInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    DisplayInfo: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchAsync: usize,
    pub Category: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameListCategory) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    #[cfg(feature = "Foundation")]
    pub SetCategoryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: GameListCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetCategoryAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameListEntry2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGameListEntry2 {
    type Vtable = IGameListEntry2_Vtbl;
}
unsafe impl ::windows::core::Interface for IGameListEntry2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd84a8f8b_8749_4a25_90d3_f6c5a427886d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameListEntry2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub LaunchableState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameListEntryLaunchableState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage")]
    pub LauncherExecutable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    LauncherExecutable: usize,
    pub LaunchParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub SetLauncherExecutableFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, executablefile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    SetLauncherExecutableFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub SetLauncherExecutableFileWithParamsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, executablefile: *mut ::core::ffi::c_void, launchparams: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    SetLauncherExecutableFileWithParamsAsync: usize,
    pub TitleId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetTitleIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTitleIdAsync: usize,
    pub GameModeConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameListStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGameListStatics {
    type Vtable = IGameListStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IGameListStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ddd0f6f_9c66_4b05_945c_d6ed78491b8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameListStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsyncPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsyncPackageFamilyName: usize,
    #[cfg(feature = "Foundation")]
    pub GameAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GameAdded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGameAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGameAdded: usize,
    #[cfg(feature = "Foundation")]
    pub GameRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GameRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGameRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGameRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub GameUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GameUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGameUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGameUpdated: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameListStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGameListStatics2 {
    type Vtable = IGameListStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IGameListStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x395f2098_ea1a_45aa_9268_a83905686f27);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameListStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub MergeEntriesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, left: *mut ::core::ffi::c_void, right: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MergeEntriesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UnmergeEntryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mergedentry: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UnmergeEntryAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameModeConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGameModeConfiguration {
    type Vtable = IGameModeConfiguration_Vtbl;
}
unsafe impl ::windows::core::Interface for IGameModeConfiguration {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78e591af_b142_4ef0_8830_55bc2be4f5ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameModeConfiguration_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RelatedProcessNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RelatedProcessNames: usize,
    #[cfg(feature = "Foundation")]
    pub PercentGpuTimeAllocatedToGame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PercentGpuTimeAllocatedToGame: usize,
    #[cfg(feature = "Foundation")]
    pub SetPercentGpuTimeAllocatedToGame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPercentGpuTimeAllocatedToGame: usize,
    #[cfg(feature = "Foundation")]
    pub PercentGpuMemoryAllocatedToGame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PercentGpuMemoryAllocatedToGame: usize,
    #[cfg(feature = "Foundation")]
    pub SetPercentGpuMemoryAllocatedToGame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPercentGpuMemoryAllocatedToGame: usize,
    #[cfg(feature = "Foundation")]
    pub PercentGpuMemoryAllocatedToSystemCompositor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PercentGpuMemoryAllocatedToSystemCompositor: usize,
    #[cfg(feature = "Foundation")]
    pub SetPercentGpuMemoryAllocatedToSystemCompositor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPercentGpuMemoryAllocatedToSystemCompositor: usize,
    #[cfg(feature = "Foundation")]
    pub MaxCpuCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxCpuCount: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxCpuCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxCpuCount: usize,
    #[cfg(feature = "Foundation")]
    pub CpuExclusivityMaskLow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CpuExclusivityMaskLow: usize,
    #[cfg(feature = "Foundation")]
    pub SetCpuExclusivityMaskLow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetCpuExclusivityMaskLow: usize,
    #[cfg(feature = "Foundation")]
    pub CpuExclusivityMaskHigh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CpuExclusivityMaskHigh: usize,
    #[cfg(feature = "Foundation")]
    pub SetCpuExclusivityMaskHigh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetCpuExclusivityMaskHigh: usize,
    pub AffinitizeToExclusiveCpus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAffinitizeToExclusiveCpus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SaveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameModeUserConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGameModeUserConfiguration {
    type Vtable = IGameModeUserConfiguration_Vtbl;
}
unsafe impl ::windows::core::Interface for IGameModeUserConfiguration {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72d34af4_756b_470f_a0c2_ba62a90795db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameModeUserConfiguration_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GamingRelatedProcessNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GamingRelatedProcessNames: usize,
    #[cfg(feature = "Foundation")]
    pub SaveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameModeUserConfigurationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGameModeUserConfigurationStatics {
    type Vtable = IGameModeUserConfigurationStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IGameModeUserConfigurationStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e50d97c_66ea_478e_a4a1_f57c0e8d00e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameModeUserConfigurationStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Gaming_Preview_GamesEnumeration\"`*"]
pub struct GameList;
impl GameList {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsync() -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<GameListEntry>>> {
        Self::IGameListStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindAllAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsyncPackageFamilyName(packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<GameListEntry>>> {
        Self::IGameListStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindAllAsyncPackageFamilyName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(packagefamilyname), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GameAdded(handler: &GameListChangedEventHandler) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        Self::IGameListStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GameAdded)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveGameAdded(token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IGameListStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).RemoveGameAdded)(::windows::core::Vtable::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GameRemoved(handler: &GameListRemovedEventHandler) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        Self::IGameListStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GameRemoved)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveGameRemoved(token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IGameListStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).RemoveGameRemoved)(::windows::core::Vtable::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GameUpdated(handler: &GameListChangedEventHandler) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        Self::IGameListStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GameUpdated)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveGameUpdated(token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IGameListStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).RemoveGameUpdated)(::windows::core::Vtable::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MergeEntriesAsync(left: &GameListEntry, right: &GameListEntry) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GameListEntry>> {
        Self::IGameListStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MergeEntriesAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(left), ::core::mem::transmute_copy(right), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UnmergeEntryAsync(mergedentry: &GameListEntry) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<GameListEntry>>> {
        Self::IGameListStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnmergeEntryAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(mergedentry), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGameListStatics<R, F: FnOnce(&IGameListStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<GameList, IGameListStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGameListStatics2<R, F: FnOnce(&IGameListStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<GameList, IGameListStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for GameList {
    const NAME: &'static str = "Windows.Gaming.Preview.GamesEnumeration.GameList";
}
#[doc = "*Required features: `\"Gaming_Preview_GamesEnumeration\"`*"]
#[repr(transparent)]
pub struct GameListEntry(::windows::core::IUnknown);
impl GameListEntry {
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    #[cfg(feature = "ApplicationModel")]
    pub fn DisplayInfo(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::AppDisplayInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Category(&self) -> ::windows::core::Result<GameListCategory> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Category)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCategoryAsync(&self, value: GameListCategory) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetCategoryAsync)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LaunchableState(&self) -> ::windows::core::Result<GameListEntryLaunchableState> {
        let this = &::windows::core::Interface::cast::<IGameListEntry2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchableState)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn LauncherExecutable(&self) -> ::windows::core::Result<super::super::super::Storage::IStorageFile> {
        let this = &::windows::core::Interface::cast::<IGameListEntry2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LauncherExecutable)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LaunchParameters(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IGameListEntry2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchParameters)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn SetLauncherExecutableFileAsync<P0, E0>(&self, executablefile: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Storage::IStorageFile>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IGameListEntry2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetLauncherExecutableFileAsync)(::windows::core::Vtable::as_raw(this), executablefile.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn SetLauncherExecutableFileWithParamsAsync<P0, E0>(&self, executablefile: P0, launchparams: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Storage::IStorageFile>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IGameListEntry2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetLauncherExecutableFileWithParamsAsync)(::windows::core::Vtable::as_raw(this), executablefile.try_into().map_err(|e| e.into())?.abi(), ::core::mem::transmute_copy(launchparams), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TitleId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IGameListEntry2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TitleId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetTitleIdAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IGameListEntry2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetTitleIdAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(id), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GameModeConfiguration(&self) -> ::windows::core::Result<GameModeConfiguration> {
        let this = &::windows::core::Interface::cast::<IGameListEntry2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GameModeConfiguration)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for GameListEntry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameListEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameListEntry {}
impl ::core::fmt::Debug for GameListEntry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameListEntry").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameListEntry {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Preview.GamesEnumeration.GameListEntry;{735924d3-811f-4494-b69c-c641a0c61543})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for GameListEntry {
    type Vtable = IGameListEntry_Vtbl;
}
unsafe impl ::windows::core::Interface for GameListEntry {
    const IID: ::windows::core::GUID = <IGameListEntry as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GameListEntry {
    const NAME: &'static str = "Windows.Gaming.Preview.GamesEnumeration.GameListEntry";
}
::windows::core::interface_hierarchy!(GameListEntry, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<GameListEntry> for IGameListEntry {
    type Error = ::windows::core::Error;
    fn try_from(value: GameListEntry) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&GameListEntry> for IGameListEntry {
    type Error = ::windows::core::Error;
    fn try_from(value: &GameListEntry) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&GameListEntry> for ::windows::core::InParam<IGameListEntry> {
    type Error = ::windows::core::Error;
    fn try_from(value: &GameListEntry) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for GameListEntry {}
unsafe impl ::core::marker::Sync for GameListEntry {}
#[doc = "*Required features: `\"Gaming_Preview_GamesEnumeration\"`*"]
#[repr(transparent)]
pub struct GameModeConfiguration(::windows::core::IUnknown);
impl GameModeConfiguration {
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RelatedProcessNames(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RelatedProcessNames)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PercentGpuTimeAllocatedToGame(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PercentGpuTimeAllocatedToGame)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPercentGpuTimeAllocatedToGame<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::IReference<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPercentGpuTimeAllocatedToGame)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PercentGpuMemoryAllocatedToGame(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PercentGpuMemoryAllocatedToGame)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPercentGpuMemoryAllocatedToGame<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::IReference<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPercentGpuMemoryAllocatedToGame)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PercentGpuMemoryAllocatedToSystemCompositor(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PercentGpuMemoryAllocatedToSystemCompositor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPercentGpuMemoryAllocatedToSystemCompositor<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::IReference<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPercentGpuMemoryAllocatedToSystemCompositor)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxCpuCount(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxCpuCount)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxCpuCount<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::IReference<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxCpuCount)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CpuExclusivityMaskLow(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CpuExclusivityMaskLow)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCpuExclusivityMaskLow<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::IReference<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCpuExclusivityMaskLow)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CpuExclusivityMaskHigh(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CpuExclusivityMaskHigh)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCpuExclusivityMaskHigh<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::IReference<i32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCpuExclusivityMaskHigh)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn AffinitizeToExclusiveCpus(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AffinitizeToExclusiveCpus)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAffinitizeToExclusiveCpus(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAffinitizeToExclusiveCpus)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SaveAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for GameModeConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameModeConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameModeConfiguration {}
impl ::core::fmt::Debug for GameModeConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameModeConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameModeConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Preview.GamesEnumeration.GameModeConfiguration;{78e591af-b142-4ef0-8830-55bc2be4f5ea})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for GameModeConfiguration {
    type Vtable = IGameModeConfiguration_Vtbl;
}
unsafe impl ::windows::core::Interface for GameModeConfiguration {
    const IID: ::windows::core::GUID = <IGameModeConfiguration as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GameModeConfiguration {
    const NAME: &'static str = "Windows.Gaming.Preview.GamesEnumeration.GameModeConfiguration";
}
::windows::core::interface_hierarchy!(GameModeConfiguration, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GameModeConfiguration {}
unsafe impl ::core::marker::Sync for GameModeConfiguration {}
#[doc = "*Required features: `\"Gaming_Preview_GamesEnumeration\"`*"]
#[repr(transparent)]
pub struct GameModeUserConfiguration(::windows::core::IUnknown);
impl GameModeUserConfiguration {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GamingRelatedProcessNames(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GamingRelatedProcessNames)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SaveAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<GameModeUserConfiguration> {
        Self::IGameModeUserConfigurationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefault)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGameModeUserConfigurationStatics<R, F: FnOnce(&IGameModeUserConfigurationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<GameModeUserConfiguration, IGameModeUserConfigurationStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for GameModeUserConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameModeUserConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameModeUserConfiguration {}
impl ::core::fmt::Debug for GameModeUserConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameModeUserConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameModeUserConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Preview.GamesEnumeration.GameModeUserConfiguration;{72d34af4-756b-470f-a0c2-ba62a90795db})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for GameModeUserConfiguration {
    type Vtable = IGameModeUserConfiguration_Vtbl;
}
unsafe impl ::windows::core::Interface for GameModeUserConfiguration {
    const IID: ::windows::core::GUID = <IGameModeUserConfiguration as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GameModeUserConfiguration {
    const NAME: &'static str = "Windows.Gaming.Preview.GamesEnumeration.GameModeUserConfiguration";
}
::windows::core::interface_hierarchy!(GameModeUserConfiguration, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GameModeUserConfiguration {}
unsafe impl ::core::marker::Sync for GameModeUserConfiguration {}
#[doc = "*Required features: `\"Gaming_Preview_GamesEnumeration\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GameListCategory(pub i32);
impl GameListCategory {
    pub const Candidate: Self = Self(0i32);
    pub const ConfirmedBySystem: Self = Self(1i32);
    pub const ConfirmedByUser: Self = Self(2i32);
}
impl ::core::marker::Copy for GameListCategory {}
impl ::core::clone::Clone for GameListCategory {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GameListCategory {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GameListCategory {
    type Abi = Self;
}
impl ::core::fmt::Debug for GameListCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameListCategory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameListCategory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Preview.GamesEnumeration.GameListCategory;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Gaming_Preview_GamesEnumeration\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GameListEntryLaunchableState(pub i32);
impl GameListEntryLaunchableState {
    pub const NotLaunchable: Self = Self(0i32);
    pub const ByLastRunningFullPath: Self = Self(1i32);
    pub const ByUserProvidedPath: Self = Self(2i32);
    pub const ByTile: Self = Self(3i32);
}
impl ::core::marker::Copy for GameListEntryLaunchableState {}
impl ::core::clone::Clone for GameListEntryLaunchableState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GameListEntryLaunchableState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GameListEntryLaunchableState {
    type Abi = Self;
}
impl ::core::fmt::Debug for GameListEntryLaunchableState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameListEntryLaunchableState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameListEntryLaunchableState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Preview.GamesEnumeration.GameListEntryLaunchableState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Gaming_Preview_GamesEnumeration\"`*"]
#[repr(transparent)]
pub struct GameListChangedEventHandler(pub ::windows::core::IUnknown);
impl GameListChangedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<GameListEntry>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = GameListChangedEventHandlerBox::<F> { vtable: &GameListChangedEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self, game: &GameListEntry) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Invoke)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(game)).ok() }
    }
}
#[repr(C)]
struct GameListChangedEventHandlerBox<F: FnMut(&::core::option::Option<GameListEntry>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const GameListChangedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<GameListEntry>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> GameListChangedEventHandlerBox<F> {
    const VTABLE: GameListChangedEventHandler_Vtbl = GameListChangedEventHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<GameListChangedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, game: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&game)).into()
    }
}
impl ::core::clone::Clone for GameListChangedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameListChangedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameListChangedEventHandler {}
impl ::core::fmt::Debug for GameListChangedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameListChangedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for GameListChangedEventHandler {
    type Vtable = GameListChangedEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for GameListChangedEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25f6a421_d8f5_4d91_b40e_53d5e86fde64);
}
unsafe impl ::windows::core::RuntimeType for GameListChangedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{25f6a421-d8f5-4d91-b40e-53d5e86fde64}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct GameListChangedEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, game: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Gaming_Preview_GamesEnumeration\"`*"]
#[repr(transparent)]
pub struct GameListRemovedEventHandler(pub ::windows::core::IUnknown);
impl GameListRemovedEventHandler {
    pub fn new<F: FnMut(&::windows::core::HSTRING) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = GameListRemovedEventHandlerBox::<F> { vtable: &GameListRemovedEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self, identifier: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Invoke)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(identifier)).ok() }
    }
}
#[repr(C)]
struct GameListRemovedEventHandlerBox<F: FnMut(&::windows::core::HSTRING) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const GameListRemovedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::windows::core::HSTRING) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> GameListRemovedEventHandlerBox<F> {
    const VTABLE: GameListRemovedEventHandler_Vtbl = GameListRemovedEventHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<GameListRemovedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, identifier: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&identifier)).into()
    }
}
impl ::core::clone::Clone for GameListRemovedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GameListRemovedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GameListRemovedEventHandler {}
impl ::core::fmt::Debug for GameListRemovedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameListRemovedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for GameListRemovedEventHandler {
    type Vtable = GameListRemovedEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for GameListRemovedEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10c5648f_6c8f_4712_9b38_474bc22e76d8);
}
unsafe impl ::windows::core::RuntimeType for GameListRemovedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{10c5648f-6c8f-4712-9b38-474bc22e76d8}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct GameListRemovedEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identifier: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
