#[cfg(feature = "implement_exclusive")]
pub trait ITargetedContentActionImpl: Sized {
    fn InvokeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetedContentAvailabilityChangedEventArgsImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetedContentChangedEventArgsImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
    fn HasPreviousContentExpired(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetedContentCollectionImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportInteraction(&self, interaction: TargetedContentInteraction) -> ::windows::core::Result<()>;
    fn ReportCustomInteraction(&self, custominteractionname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, TargetedContentValue>>;
    fn Collections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentCollection>>;
    fn Items(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentItem>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetedContentContainerImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn Availability(&self) -> ::windows::core::Result<TargetedContentAvailability>;
    fn Content(&self) -> ::windows::core::Result<TargetedContentCollection>;
    fn SelectSingleObject(&self, path: &::windows::core::HSTRING) -> ::windows::core::Result<TargetedContentObject>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetedContentContainerStaticsImpl: Sized {
    fn GetAsync(&self, contentid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<TargetedContentContainer>>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ITargetedContentImageImpl: Sized + IRandomAccessStreamReferenceImpl {
    fn Height(&self) -> ::windows::core::Result<u32>;
    fn Width(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetedContentItemImpl: Sized {
    fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportInteraction(&self, interaction: TargetedContentInteraction) -> ::windows::core::Result<()>;
    fn ReportCustomInteraction(&self, custominteractionname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn State(&self) -> ::windows::core::Result<TargetedContentItemState>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, TargetedContentValue>>;
    fn Collections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentCollection>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetedContentItemStateImpl: Sized {
    fn ShouldDisplay(&self) -> ::windows::core::Result<bool>;
    fn AppInstallationState(&self) -> ::windows::core::Result<TargetedContentAppInstallationState>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetedContentObjectImpl: Sized {
    fn ObjectKind(&self) -> ::windows::core::Result<TargetedContentObjectKind>;
    fn Collection(&self) -> ::windows::core::Result<TargetedContentCollection>;
    fn Item(&self) -> ::windows::core::Result<TargetedContentItem>;
    fn Value(&self) -> ::windows::core::Result<TargetedContentValue>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetedContentStateChangedEventArgsImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetedContentSubscriptionImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetContentContainerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<TargetedContentContainer>>;
    fn ContentChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveContentChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AvailabilityChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentAvailabilityChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAvailabilityChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetedContentSubscriptionOptionsImpl: Sized {
    fn SubscriptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AllowPartialContentAvailability(&self) -> ::windows::core::Result<bool>;
    fn SetAllowPartialContentAvailability(&self, value: bool) -> ::windows::core::Result<()>;
    fn CloudQueryParameters(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn LocalFilters(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn Update(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetedContentSubscriptionStaticsImpl: Sized {
    fn GetAsync(&self, subscriptionid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<TargetedContentSubscription>>;
    fn GetOptions(&self, subscriptionid: &::windows::core::HSTRING) -> ::windows::core::Result<TargetedContentSubscriptionOptions>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetedContentValueImpl: Sized {
    fn ValueKind(&self) -> ::windows::core::Result<TargetedContentValueKind>;
    fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn String(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn Number(&self) -> ::windows::core::Result<f64>;
    fn Boolean(&self) -> ::windows::core::Result<bool>;
    fn File(&self) -> ::windows::core::Result<TargetedContentFile>;
    fn ImageFile(&self) -> ::windows::core::Result<TargetedContentImage>;
    fn Action(&self) -> ::windows::core::Result<TargetedContentAction>;
    fn Strings(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Uris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Uri>>;
    fn Numbers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<f64>>;
    fn Booleans(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<bool>>;
    fn Files(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentFile>>;
    fn ImageFiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentImage>>;
    fn Actions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentAction>>;
}
