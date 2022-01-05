#[cfg(feature = "implement_exclusive")]
pub trait IDeviceAccessChangedEventArgsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<DeviceAccessStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceAccessChangedEventArgs2Impl: Sized + IDeviceAccessChangedEventArgsImpl {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceAccessInformationImpl: Sized {
    fn AccessChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DeviceAccessInformation, DeviceAccessChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAccessChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CurrentStatus(&self) -> ::windows::core::Result<DeviceAccessStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceAccessInformationStaticsImpl: Sized {
    fn CreateFromId(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<DeviceAccessInformation>;
    fn CreateFromDeviceClassId(&self, deviceclassid: &::windows::core::GUID) -> ::windows::core::Result<DeviceAccessInformation>;
    fn CreateFromDeviceClass(&self, deviceclass: DeviceClass) -> ::windows::core::Result<DeviceAccessInformation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceConnectionChangeTriggerDetailsImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceDisconnectButtonClickedEventArgsImpl: Sized {
    fn Device(&self) -> ::windows::core::Result<DeviceInformation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceInformationImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsDefault(&self) -> ::windows::core::Result<bool>;
    fn EnclosureLocation(&self) -> ::windows::core::Result<EnclosureLocation>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
    fn Update(&self, updateinfo: &::core::option::Option<DeviceInformationUpdate>) -> ::windows::core::Result<()>;
    fn GetThumbnailAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceThumbnail>>;
    fn GetGlyphThumbnailAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceThumbnail>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceInformation2Impl: Sized {
    fn Kind(&self) -> ::windows::core::Result<DeviceInformationKind>;
    fn Pairing(&self) -> ::windows::core::Result<DeviceInformationPairing>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceInformationCustomPairingImpl: Sized {
    fn PairAsync(&self, pairingkindssupported: DevicePairingKinds) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>>;
    fn PairWithProtectionLevelAsync(&self, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>>;
    fn PairWithProtectionLevelAndSettingsAsync(&self, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel, devicepairingsettings: &::core::option::Option<IDevicePairingSettings>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>>;
    fn PairingRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DeviceInformationCustomPairing, DevicePairingRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePairingRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceInformationPairingImpl: Sized {
    fn IsPaired(&self) -> ::windows::core::Result<bool>;
    fn CanPair(&self) -> ::windows::core::Result<bool>;
    fn PairAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>>;
    fn PairWithProtectionLevelAsync(&self, minprotectionlevel: DevicePairingProtectionLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceInformationPairing2Impl: Sized {
    fn ProtectionLevel(&self) -> ::windows::core::Result<DevicePairingProtectionLevel>;
    fn Custom(&self) -> ::windows::core::Result<DeviceInformationCustomPairing>;
    fn PairWithProtectionLevelAndSettingsAsync(&self, minprotectionlevel: DevicePairingProtectionLevel, devicepairingsettings: &::core::option::Option<IDevicePairingSettings>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>>;
    fn UnpairAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceUnpairingResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceInformationPairingStaticsImpl: Sized {
    fn TryRegisterForAllInboundPairingRequests(&self, pairingkindssupported: DevicePairingKinds) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceInformationPairingStatics2Impl: Sized {
    fn TryRegisterForAllInboundPairingRequestsWithProtectionLevel(&self, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceInformationStaticsImpl: Sized {
    fn CreateFromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformation>>;
    fn CreateFromIdAsyncAdditionalProperties(&self, deviceid: &::windows::core::HSTRING, additionalproperties: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformation>>;
    fn FindAllAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>>;
    fn FindAllAsyncDeviceClass(&self, deviceclass: DeviceClass) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>>;
    fn FindAllAsyncAqsFilter(&self, aqsfilter: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>>;
    fn FindAllAsyncAqsFilterAndAdditionalProperties(&self, aqsfilter: &::windows::core::HSTRING, additionalproperties: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>>;
    fn CreateWatcher(&self) -> ::windows::core::Result<DeviceWatcher>;
    fn CreateWatcherDeviceClass(&self, deviceclass: DeviceClass) -> ::windows::core::Result<DeviceWatcher>;
    fn CreateWatcherAqsFilter(&self, aqsfilter: &::windows::core::HSTRING) -> ::windows::core::Result<DeviceWatcher>;
    fn CreateWatcherAqsFilterAndAdditionalProperties(&self, aqsfilter: &::windows::core::HSTRING, additionalproperties: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<DeviceWatcher>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceInformationStatics2Impl: Sized {
    fn GetAqsFilterFromDeviceClass(&self, deviceclass: DeviceClass) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CreateFromIdAsyncWithKindAndAdditionalProperties(&self, deviceid: &::windows::core::HSTRING, additionalproperties: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, kind: DeviceInformationKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformation>>;
    fn FindAllAsyncWithKindAqsFilterAndAdditionalProperties(&self, aqsfilter: &::windows::core::HSTRING, additionalproperties: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, kind: DeviceInformationKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>>;
    fn CreateWatcherWithKindAqsFilterAndAdditionalProperties(&self, aqsfilter: &::windows::core::HSTRING, additionalproperties: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, kind: DeviceInformationKind) -> ::windows::core::Result<DeviceWatcher>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceInformationUpdateImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceInformationUpdate2Impl: Sized {
    fn Kind(&self) -> ::windows::core::Result<DeviceInformationKind>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDevicePairingRequestedEventArgsImpl: Sized {
    fn DeviceInformation(&self) -> ::windows::core::Result<DeviceInformation>;
    fn PairingKind(&self) -> ::windows::core::Result<DevicePairingKinds>;
    fn Pin(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Accept(&self) -> ::windows::core::Result<()>;
    fn AcceptWithPin(&self, pin: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDevicePairingRequestedEventArgs2Impl: Sized {
    fn AcceptWithPasswordCredential(&self, passwordcredential: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDevicePairingResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<DevicePairingResultStatus>;
    fn ProtectionLevelUsed(&self) -> ::windows::core::Result<DevicePairingProtectionLevel>;
}
pub trait IDevicePairingSettingsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IDevicePickerImpl: Sized {
    fn Filter(&self) -> ::windows::core::Result<DevicePickerFilter>;
    fn Appearance(&self) -> ::windows::core::Result<DevicePickerAppearance>;
    fn RequestedProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn DeviceSelected(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DevicePicker, DeviceSelectedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDeviceSelected(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DisconnectButtonClicked(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DevicePicker, DeviceDisconnectButtonClickedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDisconnectButtonClicked(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DevicePickerDismissed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DevicePicker, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDevicePickerDismissed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Show(&self, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn ShowWithPlacement(&self, selection: &super::super::Foundation::Rect, placement: super::super::UI::Popups::Placement) -> ::windows::core::Result<()>;
    fn PickSingleDeviceAsync(&self, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformation>>;
    fn PickSingleDeviceAsyncWithPlacement(&self, selection: &super::super::Foundation::Rect, placement: super::super::UI::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformation>>;
    fn Hide(&self) -> ::windows::core::Result<()>;
    fn SetDisplayStatus(&self, device: &::core::option::Option<DeviceInformation>, status: &::windows::core::HSTRING, options: DevicePickerDisplayStatusOptions) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDevicePickerAppearanceImpl: Sized {
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ForegroundColor(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetForegroundColor(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn BackgroundColor(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetBackgroundColor(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn AccentColor(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetAccentColor(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn SelectedForegroundColor(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetSelectedForegroundColor(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn SelectedBackgroundColor(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetSelectedBackgroundColor(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn SelectedAccentColor(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetSelectedAccentColor(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDevicePickerFilterImpl: Sized {
    fn SupportedDeviceClasses(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<DeviceClass>>;
    fn SupportedDeviceSelectors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceSelectedEventArgsImpl: Sized {
    fn SelectedDevice(&self) -> ::windows::core::Result<DeviceInformation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceUnpairingResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<DeviceUnpairingResultStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceWatcherImpl: Sized {
    fn Added(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DeviceWatcher, DeviceInformation>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAdded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Updated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DeviceWatcher, DeviceInformationUpdate>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUpdated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Removed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DeviceWatcher, DeviceInformationUpdate>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemoved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DeviceWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Stopped(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DeviceWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Status(&self) -> ::windows::core::Result<DeviceWatcherStatus>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceWatcher2Impl: Sized {
    fn GetBackgroundTrigger(&self, requestedeventkinds: &::core::option::Option<super::super::Foundation::Collections::IIterable<DeviceWatcherEventKind>>) -> ::windows::core::Result<super::super::ApplicationModel::Background::DeviceWatcherTrigger>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceWatcherEventImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<DeviceWatcherEventKind>;
    fn DeviceInformation(&self) -> ::windows::core::Result<DeviceInformation>;
    fn DeviceInformationUpdate(&self) -> ::windows::core::Result<DeviceInformationUpdate>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceWatcherTriggerDetailsImpl: Sized {
    fn DeviceWatcherEvents(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<DeviceWatcherEvent>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEnclosureLocationImpl: Sized {
    fn InDock(&self) -> ::windows::core::Result<bool>;
    fn InLid(&self) -> ::windows::core::Result<bool>;
    fn Panel(&self) -> ::windows::core::Result<Panel>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEnclosureLocation2Impl: Sized + IEnclosureLocationImpl {
    fn RotationAngleInDegreesClockwise(&self) -> ::windows::core::Result<u32>;
}
