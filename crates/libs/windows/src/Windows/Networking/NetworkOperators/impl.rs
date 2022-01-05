#[cfg(feature = "implement_exclusive")]
pub trait IESimImpl: Sized {
    fn AvailableMemoryInBytes(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn Eid(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FirmwareVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MobileBroadbandModemDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Policy(&self) -> ::windows::core::Result<ESimPolicy>;
    fn State(&self) -> ::windows::core::Result<ESimState>;
    fn GetProfiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ESimProfile>>;
    fn DeleteProfileAsync(&self, profileid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>>;
    fn DownloadProfileMetadataAsync(&self, activationcode: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimDownloadProfileMetadataResult>>;
    fn ResetAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>>;
    fn ProfileChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ESim, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveProfileChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IESim2Impl: Sized {
    fn Discover(&self) -> ::windows::core::Result<ESimDiscoverResult>;
    fn DiscoverWithServerAddressAndMatchingId(&self, serveraddress: &::windows::core::HSTRING, matchingid: &::windows::core::HSTRING) -> ::windows::core::Result<ESimDiscoverResult>;
    fn DiscoverAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimDiscoverResult>>;
    fn DiscoverWithServerAddressAndMatchingIdAsync(&self, serveraddress: &::windows::core::HSTRING, matchingid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimDiscoverResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IESimAddedEventArgsImpl: Sized {
    fn ESim(&self) -> ::windows::core::Result<ESim>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IESimDiscoverEventImpl: Sized {
    fn MatchingId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RspServerAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IESimDiscoverResultImpl: Sized {
    fn Events(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ESimDiscoverEvent>>;
    fn Kind(&self) -> ::windows::core::Result<ESimDiscoverResultKind>;
    fn ProfileMetadata(&self) -> ::windows::core::Result<ESimProfileMetadata>;
    fn Result(&self) -> ::windows::core::Result<ESimOperationResult>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IESimDownloadProfileMetadataResultImpl: Sized {
    fn Result(&self) -> ::windows::core::Result<ESimOperationResult>;
    fn ProfileMetadata(&self) -> ::windows::core::Result<ESimProfileMetadata>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IESimManagerStaticsImpl: Sized {
    fn ServiceInfo(&self) -> ::windows::core::Result<ESimServiceInfo>;
    fn TryCreateESimWatcher(&self) -> ::windows::core::Result<ESimWatcher>;
    fn ServiceInfoChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveServiceInfoChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IESimOperationResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<ESimOperationStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IESimPolicyImpl: Sized {
    fn ShouldEnableManagingUi(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IESimProfileImpl: Sized {
    fn Class(&self) -> ::windows::core::Result<ESimProfileClass>;
    fn Nickname(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Policy(&self) -> ::windows::core::Result<ESimProfilePolicy>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProviderIcon(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn ProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProviderName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn State(&self) -> ::windows::core::Result<ESimProfileState>;
    fn DisableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>>;
    fn EnableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>>;
    fn SetNicknameAsync(&self, newnickname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IESimProfileMetadataImpl: Sized {
    fn IsConfirmationCodeRequired(&self) -> ::windows::core::Result<bool>;
    fn Policy(&self) -> ::windows::core::Result<ESimProfilePolicy>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProviderIcon(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn ProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProviderName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn State(&self) -> ::windows::core::Result<ESimProfileMetadataState>;
    fn DenyInstallAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>>;
    fn ConfirmInstallAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<ESimOperationResult, ESimProfileInstallProgress>>;
    fn ConfirmInstallWithConfirmationCodeAsync(&self, confirmationcode: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<ESimOperationResult, ESimProfileInstallProgress>>;
    fn PostponeInstallAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>>;
    fn StateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ESimProfileMetadata, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IESimProfilePolicyImpl: Sized {
    fn CanDelete(&self) -> ::windows::core::Result<bool>;
    fn CanDisable(&self) -> ::windows::core::Result<bool>;
    fn IsManagedByEnterprise(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IESimRemovedEventArgsImpl: Sized {
    fn ESim(&self) -> ::windows::core::Result<ESim>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IESimServiceInfoImpl: Sized {
    fn AuthenticationPreference(&self) -> ::windows::core::Result<ESimAuthenticationPreference>;
    fn IsESimUiEnabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IESimUpdatedEventArgsImpl: Sized {
    fn ESim(&self) -> ::windows::core::Result<ESim>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IESimWatcherImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<ESimWatcherStatus>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn Added(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ESimWatcher, ESimAddedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAdded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ESimWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Removed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ESimWatcher, ESimRemovedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemoved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Stopped(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ESimWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Updated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ESimWatcher, ESimUpdatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUpdated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFdnAccessManagerStaticsImpl: Sized {
    fn RequestUnlockAsync(&self, contactlistid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHotspotAuthenticationContextImpl: Sized {
    fn WirelessNetworkId(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn NetworkAdapter(&self) -> ::windows::core::Result<super::Connectivity::NetworkAdapter>;
    fn RedirectMessageUrl(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn RedirectMessageXml(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument>;
    fn AuthenticationUrl(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn IssueCredentials(&self, username: &::windows::core::HSTRING, password: &::windows::core::HSTRING, extraparameters: &::windows::core::HSTRING, markasmanualconnectonfailure: bool) -> ::windows::core::Result<()>;
    fn AbortAuthentication(&self, markasmanual: bool) -> ::windows::core::Result<()>;
    fn SkipAuthentication(&self) -> ::windows::core::Result<()>;
    fn TriggerAttentionRequired(&self, packagerelativeapplicationid: &::windows::core::HSTRING, applicationparameters: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHotspotAuthenticationContext2Impl: Sized {
    fn IssueCredentialsAsync(&self, username: &::windows::core::HSTRING, password: &::windows::core::HSTRING, extraparameters: &::windows::core::HSTRING, markasmanualconnectonfailure: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HotspotCredentialsAuthenticationResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHotspotAuthenticationContextStaticsImpl: Sized {
    fn TryGetAuthenticationContext(&self, eventoken: &::windows::core::HSTRING, context: &mut ::core::option::Option<HotspotAuthenticationContext>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHotspotAuthenticationEventDetailsImpl: Sized {
    fn EventToken(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHotspotCredentialsAuthenticationResultImpl: Sized {
    fn HasNetworkErrorOccurred(&self) -> ::windows::core::Result<bool>;
    fn ResponseCode(&self) -> ::windows::core::Result<HotspotAuthenticationResponseCode>;
    fn LogoffUrl(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn AuthenticationReplyXml(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownCSimFilePathsStaticsImpl: Sized {
    fn EFSpn(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn Gid1(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn Gid2(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownRuimFilePathsStaticsImpl: Sized {
    fn EFSpn(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn Gid1(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn Gid2(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownSimFilePathsStaticsImpl: Sized {
    fn EFOns(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn EFSpn(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn Gid1(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn Gid2(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownUSimFilePathsStaticsImpl: Sized {
    fn EFSpn(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn EFOpl(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn EFPnn(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn Gid1(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn Gid2(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandAccountImpl: Sized {
    fn NetworkAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ServiceProviderGuid(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ServiceProviderName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CurrentNetwork(&self) -> ::windows::core::Result<MobileBroadbandNetwork>;
    fn CurrentDeviceInformation(&self) -> ::windows::core::Result<MobileBroadbandDeviceInformation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandAccount2Impl: Sized {
    fn GetConnectionProfiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::Connectivity::ConnectionProfile>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandAccount3Impl: Sized {
    fn AccountExperienceUrl(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandAccountEventArgsImpl: Sized {
    fn NetworkAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandAccountStaticsImpl: Sized {
    fn AvailableNetworkAccountIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn CreateFromNetworkAccountId(&self, networkaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<MobileBroadbandAccount>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandAccountUpdatedEventArgsImpl: Sized {
    fn NetworkAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HasDeviceInformationChanged(&self) -> ::windows::core::Result<bool>;
    fn HasNetworkChanged(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandAccountWatcherImpl: Sized {
    fn AccountAdded(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, MobileBroadbandAccountEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAccountAdded(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AccountUpdated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, MobileBroadbandAccountUpdatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAccountUpdated(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AccountRemoved(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, MobileBroadbandAccountEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAccountRemoved(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Stopped(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Status(&self) -> ::windows::core::Result<MobileBroadbandAccountWatcherStatus>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandAntennaSarImpl: Sized {
    fn AntennaIndex(&self) -> ::windows::core::Result<i32>;
    fn SarBackoffIndex(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandAntennaSarFactoryImpl: Sized {
    fn CreateWithIndex(&self, antennaindex: i32, sarbackoffindex: i32) -> ::windows::core::Result<MobileBroadbandAntennaSar>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandCellCdmaImpl: Sized {
    fn BaseStationId(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn BaseStationPNCode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn BaseStationLatitude(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn BaseStationLongitude(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn BaseStationLastBroadcastGpsTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn NetworkId(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn PilotSignalStrengthInDB(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn SystemId(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandCellGsmImpl: Sized {
    fn BaseStationId(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn CellId(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn ChannelNumber(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn LocationAreaCode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn ProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReceivedSignalStrengthInDBm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn TimingAdvanceInBitPeriods(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandCellLteImpl: Sized {
    fn CellId(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn ChannelNumber(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn PhysicalCellId(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn ProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReferenceSignalReceivedPowerInDBm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn ReferenceSignalReceivedQualityInDBm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn TimingAdvanceInBitPeriods(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn TrackingAreaCode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandCellNRImpl: Sized {
    fn CellId(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i64>>;
    fn ChannelNumber(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn PhysicalCellId(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn ProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReferenceSignalReceivedPowerInDBm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn ReferenceSignalReceivedQualityInDBm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn TimingAdvanceInNanoseconds(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn TrackingAreaCode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SignalToNoiseRatioInDB(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandCellTdscdmaImpl: Sized {
    fn CellId(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn CellParameterId(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn ChannelNumber(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn LocationAreaCode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn PathLossInDB(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn ProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReceivedSignalCodePowerInDBm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn TimingAdvanceInBitPeriods(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandCellUmtsImpl: Sized {
    fn CellId(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn ChannelNumber(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn LocationAreaCode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn PathLossInDB(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn PrimaryScramblingCode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn ProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReceivedSignalCodePowerInDBm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn SignalToNoiseRatioInDB(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandCellsInfoImpl: Sized {
    fn NeighboringCellsCdma(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellCdma>>;
    fn NeighboringCellsGsm(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellGsm>>;
    fn NeighboringCellsLte(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellLte>>;
    fn NeighboringCellsTdscdma(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellTdscdma>>;
    fn NeighboringCellsUmts(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellUmts>>;
    fn ServingCellsCdma(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellCdma>>;
    fn ServingCellsGsm(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellGsm>>;
    fn ServingCellsLte(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellLte>>;
    fn ServingCellsTdscdma(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellTdscdma>>;
    fn ServingCellsUmts(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellUmts>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandCellsInfo2Impl: Sized {
    fn NeighboringCellsNR(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellNR>>;
    fn ServingCellsNR(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellNR>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandCurrentSlotIndexChangedEventArgsImpl: Sized {
    fn CurrentSlotIndex(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandDeviceInformationImpl: Sized {
    fn NetworkDeviceStatus(&self) -> ::windows::core::Result<NetworkDeviceStatus>;
    fn Manufacturer(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Model(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FirmwareInformation(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CellularClass(&self) -> ::windows::core::Result<super::super::Devices::Sms::CellularClass>;
    fn DataClasses(&self) -> ::windows::core::Result<DataClasses>;
    fn CustomDataClass(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MobileEquipmentId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TelephoneNumbers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn SubscriberId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SimIccId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceType(&self) -> ::windows::core::Result<MobileBroadbandDeviceType>;
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CurrentRadioState(&self) -> ::windows::core::Result<MobileBroadbandRadioState>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandDeviceInformation2Impl: Sized {
    fn PinManager(&self) -> ::windows::core::Result<MobileBroadbandPinManager>;
    fn Revision(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SerialNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandDeviceInformation3Impl: Sized {
    fn SimSpn(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SimPnn(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SimGid1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandDeviceInformation4Impl: Sized {
    fn SlotManager(&self) -> ::windows::core::Result<MobileBroadbandSlotManager>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandDeviceServiceImpl: Sized {
    fn DeviceServiceId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SupportedCommands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn OpenDataSession(&self) -> ::windows::core::Result<MobileBroadbandDeviceServiceDataSession>;
    fn OpenCommandSession(&self) -> ::windows::core::Result<MobileBroadbandDeviceServiceCommandSession>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandDeviceServiceCommandResultImpl: Sized {
    fn StatusCode(&self) -> ::windows::core::Result<u32>;
    fn ResponseData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandDeviceServiceCommandSessionImpl: Sized {
    fn SendQueryCommandAsync(&self, commandid: u32, data: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandDeviceServiceCommandResult>>;
    fn SendSetCommandAsync(&self, commandid: u32, data: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandDeviceServiceCommandResult>>;
    fn CloseSession(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandDeviceServiceDataReceivedEventArgsImpl: Sized {
    fn ReceivedData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandDeviceServiceDataSessionImpl: Sized {
    fn WriteDataAsync(&self, value: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn CloseSession(&self) -> ::windows::core::Result<()>;
    fn DataReceived(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MobileBroadbandDeviceServiceDataSession, MobileBroadbandDeviceServiceDataReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDataReceived(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandDeviceServiceInformationImpl: Sized {
    fn DeviceServiceId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn IsDataReadSupported(&self) -> ::windows::core::Result<bool>;
    fn IsDataWriteSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandDeviceServiceTriggerDetailsImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceServiceId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ReceivedData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandModemImpl: Sized {
    fn CurrentAccount(&self) -> ::windows::core::Result<MobileBroadbandAccount>;
    fn DeviceInformation(&self) -> ::windows::core::Result<MobileBroadbandDeviceInformation>;
    fn MaxDeviceServiceCommandSizeInBytes(&self) -> ::windows::core::Result<u32>;
    fn MaxDeviceServiceDataSizeInBytes(&self) -> ::windows::core::Result<u32>;
    fn DeviceServices(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandDeviceServiceInformation>>;
    fn GetDeviceService(&self, deviceserviceid: &::windows::core::GUID) -> ::windows::core::Result<MobileBroadbandDeviceService>;
    fn IsResetSupported(&self) -> ::windows::core::Result<bool>;
    fn ResetAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetCurrentConfigurationAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandModemConfiguration>>;
    fn CurrentNetwork(&self) -> ::windows::core::Result<MobileBroadbandNetwork>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandModem2Impl: Sized {
    fn GetIsPassthroughEnabledAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn SetIsPassthroughEnabledAsync(&self, value: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandModemStatus>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandModem3Impl: Sized {
    fn TryGetPcoAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPco>>;
    fn IsInEmergencyCallMode(&self) -> ::windows::core::Result<bool>;
    fn IsInEmergencyCallModeChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MobileBroadbandModem, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsInEmergencyCallModeChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandModemConfigurationImpl: Sized {
    fn Uicc(&self) -> ::windows::core::Result<MobileBroadbandUicc>;
    fn HomeProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HomeProviderName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandModemConfiguration2Impl: Sized {
    fn SarManager(&self) -> ::windows::core::Result<MobileBroadbandSarManager>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandModemIsolationImpl: Sized {
    fn AddAllowedHost(&self, host: &::core::option::Option<super::HostName>) -> ::windows::core::Result<()>;
    fn AddAllowedHostRange(&self, first: &::core::option::Option<super::HostName>, last: &::core::option::Option<super::HostName>) -> ::windows::core::Result<()>;
    fn ApplyConfigurationAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ClearConfigurationAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandModemIsolationFactoryImpl: Sized {
    fn Create(&self, modemdeviceid: &::windows::core::HSTRING, rulegroupid: &::windows::core::HSTRING) -> ::windows::core::Result<MobileBroadbandModemIsolation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandModemStaticsImpl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromId(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<MobileBroadbandModem>;
    fn GetDefault(&self) -> ::windows::core::Result<MobileBroadbandModem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandNetworkImpl: Sized {
    fn NetworkAdapter(&self) -> ::windows::core::Result<super::Connectivity::NetworkAdapter>;
    fn NetworkRegistrationState(&self) -> ::windows::core::Result<NetworkRegistrationState>;
    fn RegistrationNetworkError(&self) -> ::windows::core::Result<u32>;
    fn PacketAttachNetworkError(&self) -> ::windows::core::Result<u32>;
    fn ActivationNetworkError(&self) -> ::windows::core::Result<u32>;
    fn AccessPointName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RegisteredDataClass(&self) -> ::windows::core::Result<DataClasses>;
    fn RegisteredProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RegisteredProviderName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ShowConnectionUI(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandNetwork2Impl: Sized {
    fn GetVoiceCallSupportAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RegistrationUiccApps(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandUiccApp>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandNetwork3Impl: Sized {
    fn GetCellsInfoAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandCellsInfo>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandNetworkRegistrationStateChangeImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Network(&self) -> ::windows::core::Result<MobileBroadbandNetwork>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandNetworkRegistrationStateChangeTriggerDetailsImpl: Sized {
    fn NetworkRegistrationStateChanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandNetworkRegistrationStateChange>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandPcoImpl: Sized {
    fn Data(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn IsComplete(&self) -> ::windows::core::Result<bool>;
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandPcoDataChangeTriggerDetailsImpl: Sized {
    fn UpdatedData(&self) -> ::windows::core::Result<MobileBroadbandPco>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandPinImpl: Sized {
    fn Type(&self) -> ::windows::core::Result<MobileBroadbandPinType>;
    fn LockState(&self) -> ::windows::core::Result<MobileBroadbandPinLockState>;
    fn Format(&self) -> ::windows::core::Result<MobileBroadbandPinFormat>;
    fn Enabled(&self) -> ::windows::core::Result<bool>;
    fn MaxLength(&self) -> ::windows::core::Result<u32>;
    fn MinLength(&self) -> ::windows::core::Result<u32>;
    fn AttemptsRemaining(&self) -> ::windows::core::Result<u32>;
    fn EnableAsync(&self, currentpin: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>>;
    fn DisableAsync(&self, currentpin: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>>;
    fn EnterAsync(&self, currentpin: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>>;
    fn ChangeAsync(&self, currentpin: &::windows::core::HSTRING, newpin: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>>;
    fn UnblockAsync(&self, pinunblockkey: &::windows::core::HSTRING, newpin: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandPinLockStateChangeImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PinType(&self) -> ::windows::core::Result<MobileBroadbandPinType>;
    fn PinLockState(&self) -> ::windows::core::Result<MobileBroadbandPinLockState>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandPinLockStateChangeTriggerDetailsImpl: Sized {
    fn PinLockStateChanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandPinLockStateChange>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandPinManagerImpl: Sized {
    fn SupportedPins(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandPinType>>;
    fn GetPin(&self, pintype: MobileBroadbandPinType) -> ::windows::core::Result<MobileBroadbandPin>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandPinOperationResultImpl: Sized {
    fn IsSuccessful(&self) -> ::windows::core::Result<bool>;
    fn AttemptsRemaining(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandRadioStateChangeImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RadioState(&self) -> ::windows::core::Result<MobileBroadbandRadioState>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandRadioStateChangeTriggerDetailsImpl: Sized {
    fn RadioStateChanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandRadioStateChange>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandSarManagerImpl: Sized {
    fn IsBackoffEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsWiFiHardwareIntegrated(&self) -> ::windows::core::Result<bool>;
    fn IsSarControlledByHardware(&self) -> ::windows::core::Result<bool>;
    fn Antennas(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandAntennaSar>>;
    fn HysteresisTimerPeriod(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn TransmissionStateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MobileBroadbandSarManager, MobileBroadbandTransmissionStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTransmissionStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnableBackoffAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DisableBackoffAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetConfigurationAsync(&self, antennas: &::core::option::Option<super::super::Foundation::Collections::IIterable<MobileBroadbandAntennaSar>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RevertSarToHardwareControlAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetTransmissionStateChangedHysteresisAsync(&self, timerperiod: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetIsTransmittingAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn StartTransmissionStateMonitoring(&self) -> ::windows::core::Result<()>;
    fn StopTransmissionStateMonitoring(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandSlotInfoImpl: Sized {
    fn Index(&self) -> ::windows::core::Result<i32>;
    fn State(&self) -> ::windows::core::Result<MobileBroadbandSlotState>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandSlotInfoChangedEventArgsImpl: Sized {
    fn SlotInfo(&self) -> ::windows::core::Result<MobileBroadbandSlotInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandSlotManagerImpl: Sized {
    fn SlotInfos(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandSlotInfo>>;
    fn CurrentSlotIndex(&self) -> ::windows::core::Result<i32>;
    fn SetCurrentSlot(&self, slotindex: i32) -> ::windows::core::Result<MobileBroadbandModemStatus>;
    fn SetCurrentSlotAsync(&self, slotindex: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandModemStatus>>;
    fn SlotInfoChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MobileBroadbandSlotManager, MobileBroadbandSlotInfoChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSlotInfoChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CurrentSlotIndexChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MobileBroadbandSlotManager, MobileBroadbandCurrentSlotIndexChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCurrentSlotIndexChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandTransmissionStateChangedEventArgsImpl: Sized {
    fn IsTransmitting(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandUiccImpl: Sized {
    fn SimIccId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetUiccAppsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandUiccAppsResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandUiccAppImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn Kind(&self) -> ::windows::core::Result<UiccAppKind>;
    fn GetRecordDetailsAsync(&self, uiccfilepath: &::core::option::Option<super::super::Foundation::Collections::IIterable<u32>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandUiccAppRecordDetailsResult>>;
    fn ReadRecordAsync(&self, uiccfilepath: &::core::option::Option<super::super::Foundation::Collections::IIterable<u32>>, recordindex: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandUiccAppReadRecordResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandUiccAppReadRecordResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<MobileBroadbandUiccAppOperationStatus>;
    fn Data(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandUiccAppRecordDetailsResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<MobileBroadbandUiccAppOperationStatus>;
    fn Kind(&self) -> ::windows::core::Result<UiccAppRecordKind>;
    fn RecordCount(&self) -> ::windows::core::Result<i32>;
    fn RecordSize(&self) -> ::windows::core::Result<i32>;
    fn ReadAccessCondition(&self) -> ::windows::core::Result<UiccAccessCondition>;
    fn WriteAccessCondition(&self) -> ::windows::core::Result<UiccAccessCondition>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandUiccAppsResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<MobileBroadbandUiccAppOperationStatus>;
    fn UiccApps(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandUiccApp>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkOperatorDataUsageTriggerDetailsImpl: Sized {
    fn NotificationKind(&self) -> ::windows::core::Result<NetworkOperatorDataUsageNotificationKind>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkOperatorNotificationEventDetailsImpl: Sized {
    fn NotificationType(&self) -> ::windows::core::Result<NetworkOperatorEventMessageType>;
    fn NetworkAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EncodingType(&self) -> ::windows::core::Result<u8>;
    fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RuleId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SmsMessage(&self) -> ::windows::core::Result<super::super::Devices::Sms::ISmsMessage>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkOperatorTetheringAccessPointConfigurationImpl: Sized {
    fn Ssid(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSsid(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Passphrase(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPassphrase(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkOperatorTetheringAccessPointConfiguration2Impl: Sized {
    fn IsBandSupported(&self, band: TetheringWiFiBand) -> ::windows::core::Result<bool>;
    fn IsBandSupportedAsync(&self, band: TetheringWiFiBand) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn Band(&self) -> ::windows::core::Result<TetheringWiFiBand>;
    fn SetBand(&self, value: TetheringWiFiBand) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkOperatorTetheringClientImpl: Sized {
    fn MacAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HostNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::HostName>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkOperatorTetheringClientManagerImpl: Sized {
    fn GetTetheringClients(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<NetworkOperatorTetheringClient>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkOperatorTetheringEntitlementCheckImpl: Sized {
    fn AuthorizeTethering(&self, allow: bool, entitlementfailurereason: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkOperatorTetheringManagerImpl: Sized {
    fn MaxClientCount(&self) -> ::windows::core::Result<u32>;
    fn ClientCount(&self) -> ::windows::core::Result<u32>;
    fn TetheringOperationalState(&self) -> ::windows::core::Result<TetheringOperationalState>;
    fn GetCurrentAccessPointConfiguration(&self) -> ::windows::core::Result<NetworkOperatorTetheringAccessPointConfiguration>;
    fn ConfigureAccessPointAsync(&self, configuration: &::core::option::Option<NetworkOperatorTetheringAccessPointConfiguration>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StartTetheringAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<NetworkOperatorTetheringOperationResult>>;
    fn StopTetheringAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<NetworkOperatorTetheringOperationResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkOperatorTetheringManagerStaticsImpl: Sized {
    fn GetTetheringCapability(&self, networkaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<TetheringCapability>;
    fn CreateFromNetworkAccountId(&self, networkaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<NetworkOperatorTetheringManager>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkOperatorTetheringManagerStatics2Impl: Sized {
    fn GetTetheringCapabilityFromConnectionProfile(&self, profile: &::core::option::Option<super::Connectivity::ConnectionProfile>) -> ::windows::core::Result<TetheringCapability>;
    fn CreateFromConnectionProfile(&self, profile: &::core::option::Option<super::Connectivity::ConnectionProfile>) -> ::windows::core::Result<NetworkOperatorTetheringManager>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkOperatorTetheringManagerStatics3Impl: Sized {
    fn CreateFromConnectionProfileWithTargetAdapter(&self, profile: &::core::option::Option<super::Connectivity::ConnectionProfile>, adapter: &::core::option::Option<super::Connectivity::NetworkAdapter>) -> ::windows::core::Result<NetworkOperatorTetheringManager>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkOperatorTetheringManagerStatics4Impl: Sized {
    fn IsNoConnectionsTimeoutEnabled(&self) -> ::windows::core::Result<bool>;
    fn EnableNoConnectionsTimeout(&self) -> ::windows::core::Result<()>;
    fn EnableNoConnectionsTimeoutAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DisableNoConnectionsTimeout(&self) -> ::windows::core::Result<()>;
    fn DisableNoConnectionsTimeoutAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkOperatorTetheringOperationResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<TetheringOperationStatus>;
    fn AdditionalErrorMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProvisionFromXmlDocumentResultsImpl: Sized {
    fn AllElementsProvisioned(&self) -> ::windows::core::Result<bool>;
    fn ProvisionResultsXml(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProvisionedProfileImpl: Sized {
    fn UpdateCost(&self, value: super::Connectivity::NetworkCostType) -> ::windows::core::Result<()>;
    fn UpdateUsage(&self, value: &ProfileUsage) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProvisioningAgentImpl: Sized {
    fn ProvisionFromXmlDocumentAsync(&self, provisioningxmldocument: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProvisionFromXmlDocumentResults>>;
    fn GetProvisionedProfile(&self, mediatype: ProfileMediaType, profilename: &::windows::core::HSTRING) -> ::windows::core::Result<ProvisionedProfile>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProvisioningAgentStaticMethodsImpl: Sized {
    fn CreateFromNetworkAccountId(&self, networkaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<ProvisioningAgent>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITetheringEntitlementCheckTriggerDetailsImpl: Sized {
    fn NetworkAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AllowTethering(&self) -> ::windows::core::Result<()>;
    fn DenyTethering(&self, entitlementfailurereason: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUssdMessageImpl: Sized {
    fn DataCodingScheme(&self) -> ::windows::core::Result<u8>;
    fn SetDataCodingScheme(&self, value: u8) -> ::windows::core::Result<()>;
    fn GetPayload(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn SetPayload(&self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn PayloadAsText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPayloadAsText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUssdMessageFactoryImpl: Sized {
    fn CreateMessage(&self, messagetext: &::windows::core::HSTRING) -> ::windows::core::Result<UssdMessage>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUssdReplyImpl: Sized {
    fn ResultCode(&self) -> ::windows::core::Result<UssdResultCode>;
    fn Message(&self) -> ::windows::core::Result<UssdMessage>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUssdSessionImpl: Sized {
    fn SendMessageAndGetReplyAsync(&self, message: &::core::option::Option<UssdMessage>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UssdReply>>;
    fn Close(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUssdSessionStaticsImpl: Sized {
    fn CreateFromNetworkAccountId(&self, networkaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<UssdSession>;
    fn CreateFromNetworkInterfaceId(&self, networkinterfaceid: &::windows::core::HSTRING) -> ::windows::core::Result<UssdSession>;
}
