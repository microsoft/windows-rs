#[cfg(feature = "implement_exclusive")]
pub trait ISpatialAnchorImpl: Sized {
    fn CoordinateSystem(&self) -> ::windows::core::Result<SpatialCoordinateSystem>;
    fn RawCoordinateSystem(&self) -> ::windows::core::Result<SpatialCoordinateSystem>;
    fn RawCoordinateSystemAdjusted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SpatialAnchor, SpatialAnchorRawCoordinateSystemAdjustedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRawCoordinateSystemAdjusted(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialAnchor2Impl: Sized {
    fn RemovedByUser(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialAnchorExportSufficiencyImpl: Sized {
    fn IsMinimallySufficient(&self) -> ::windows::core::Result<bool>;
    fn SufficiencyLevel(&self) -> ::windows::core::Result<f64>;
    fn RecommendedSufficiencyLevel(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialAnchorExporterImpl: Sized {
    fn GetAnchorExportSufficiencyAsync(&self, anchor: &::core::option::Option<SpatialAnchor>, purpose: SpatialAnchorExportPurpose) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpatialAnchorExportSufficiency>>;
    fn TryExportAnchorAsync(&self, anchor: &::core::option::Option<SpatialAnchor>, purpose: SpatialAnchorExportPurpose, stream: &::core::option::Option<super::super::Storage::Streams::IOutputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialAnchorExporterStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<SpatialAnchorExporter>;
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpatialPerceptionAccessStatus>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialAnchorManagerStaticsImpl: Sized {
    fn RequestStoreAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpatialAnchorStore>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialAnchorRawCoordinateSystemAdjustedEventArgsImpl: Sized {
    fn OldRawCoordinateSystemToNewRawCoordinateSystemTransform(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Matrix4x4>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialAnchorStaticsImpl: Sized {
    fn TryCreateRelativeTo(&self, coordinatesystem: &::core::option::Option<SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialAnchor>;
    fn TryCreateWithPositionRelativeTo(&self, coordinatesystem: &::core::option::Option<SpatialCoordinateSystem>, position: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<SpatialAnchor>;
    fn TryCreateWithPositionAndOrientationRelativeTo(&self, coordinatesystem: &::core::option::Option<SpatialCoordinateSystem>, position: &super::super::Foundation::Numerics::Vector3, orientation: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<SpatialAnchor>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialAnchorStoreImpl: Sized {
    fn GetAllSavedAnchors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, SpatialAnchor>>;
    fn TrySave(&self, id: &::windows::core::HSTRING, anchor: &::core::option::Option<SpatialAnchor>) -> ::windows::core::Result<bool>;
    fn Remove(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISpatialAnchorTransferManagerStaticsImpl: Sized {
    fn TryImportAnchorsAsync(&self, stream: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, SpatialAnchor>>>;
    fn TryExportAnchorsAsync(&self, anchors: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, SpatialAnchor>>>, stream: &::core::option::Option<super::super::Storage::Streams::IOutputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpatialPerceptionAccessStatus>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialBoundingVolumeImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialBoundingVolumeStaticsImpl: Sized {
    fn FromBox(&self, coordinatesystem: &::core::option::Option<SpatialCoordinateSystem>, r#box: &SpatialBoundingBox) -> ::windows::core::Result<SpatialBoundingVolume>;
    fn FromOrientedBox(&self, coordinatesystem: &::core::option::Option<SpatialCoordinateSystem>, r#box: &SpatialBoundingOrientedBox) -> ::windows::core::Result<SpatialBoundingVolume>;
    fn FromSphere(&self, coordinatesystem: &::core::option::Option<SpatialCoordinateSystem>, sphere: &SpatialBoundingSphere) -> ::windows::core::Result<SpatialBoundingVolume>;
    fn FromFrustum(&self, coordinatesystem: &::core::option::Option<SpatialCoordinateSystem>, frustum: &SpatialBoundingFrustum) -> ::windows::core::Result<SpatialBoundingVolume>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialCoordinateSystemImpl: Sized {
    fn TryGetTransformTo(&self, target: &::core::option::Option<SpatialCoordinateSystem>) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::Numerics::Matrix4x4>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialEntityImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Anchor(&self) -> ::windows::core::Result<SpatialAnchor>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialEntityAddedEventArgsImpl: Sized {
    fn Entity(&self) -> ::windows::core::Result<SpatialEntity>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialEntityFactoryImpl: Sized {
    fn CreateWithSpatialAnchor(&self, spatialanchor: &::core::option::Option<SpatialAnchor>) -> ::windows::core::Result<SpatialEntity>;
    fn CreateWithSpatialAnchorAndProperties(&self, spatialanchor: &::core::option::Option<SpatialAnchor>, propertyset: &::core::option::Option<super::super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<SpatialEntity>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialEntityRemovedEventArgsImpl: Sized {
    fn Entity(&self) -> ::windows::core::Result<SpatialEntity>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialEntityStoreImpl: Sized {
    fn SaveAsync(&self, entity: &::core::option::Option<SpatialEntity>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RemoveAsync(&self, entity: &::core::option::Option<SpatialEntity>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn CreateEntityWatcher(&self) -> ::windows::core::Result<SpatialEntityWatcher>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialEntityStoreStaticsImpl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
    fn TryGetForRemoteSystemSession(&self, session: &::core::option::Option<super::super::System::RemoteSystems::RemoteSystemSession>) -> ::windows::core::Result<SpatialEntityStore>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialEntityUpdatedEventArgsImpl: Sized {
    fn Entity(&self) -> ::windows::core::Result<SpatialEntity>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialEntityWatcherImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<SpatialEntityWatcherStatus>;
    fn Added(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityAddedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAdded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Updated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityUpdatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUpdated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Removed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityRemovedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemoved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SpatialEntityWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialLocationImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn Orientation(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Quaternion>;
    fn AbsoluteLinearVelocity(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn AbsoluteLinearAcceleration(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn AbsoluteAngularVelocity(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Quaternion>;
    fn AbsoluteAngularAcceleration(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Quaternion>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialLocation2Impl: Sized {
    fn AbsoluteAngularVelocityAxisAngle(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn AbsoluteAngularAccelerationAxisAngle(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialLocatorImpl: Sized {
    fn Locatability(&self) -> ::windows::core::Result<SpatialLocatability>;
    fn LocatabilityChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SpatialLocator, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLocatabilityChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PositionalTrackingDeactivating(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SpatialLocator, SpatialLocatorPositionalTrackingDeactivatingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePositionalTrackingDeactivating(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TryLocateAtTimestamp(&self, timestamp: &::core::option::Option<super::PerceptionTimestamp>, coordinatesystem: &::core::option::Option<SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialLocation>;
    fn CreateAttachedFrameOfReferenceAtCurrentHeading(&self) -> ::windows::core::Result<SpatialLocatorAttachedFrameOfReference>;
    fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition(&self, relativeposition: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<SpatialLocatorAttachedFrameOfReference>;
    fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation(&self, relativeposition: &super::super::Foundation::Numerics::Vector3, relativeorientation: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<SpatialLocatorAttachedFrameOfReference>;
    fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading(&self, relativeposition: &super::super::Foundation::Numerics::Vector3, relativeorientation: &super::super::Foundation::Numerics::Quaternion, relativeheadinginradians: f64) -> ::windows::core::Result<SpatialLocatorAttachedFrameOfReference>;
    fn CreateStationaryFrameOfReferenceAtCurrentLocation(&self) -> ::windows::core::Result<SpatialStationaryFrameOfReference>;
    fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition(&self, relativeposition: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<SpatialStationaryFrameOfReference>;
    fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation(&self, relativeposition: &super::super::Foundation::Numerics::Vector3, relativeorientation: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<SpatialStationaryFrameOfReference>;
    fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading(&self, relativeposition: &super::super::Foundation::Numerics::Vector3, relativeorientation: &super::super::Foundation::Numerics::Quaternion, relativeheadinginradians: f64) -> ::windows::core::Result<SpatialStationaryFrameOfReference>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialLocatorAttachedFrameOfReferenceImpl: Sized {
    fn RelativePosition(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetRelativePosition(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn RelativeOrientation(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Quaternion>;
    fn SetRelativeOrientation(&self, value: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()>;
    fn AdjustHeading(&self, headingoffsetinradians: f64) -> ::windows::core::Result<()>;
    fn GetStationaryCoordinateSystemAtTimestamp(&self, timestamp: &::core::option::Option<super::PerceptionTimestamp>) -> ::windows::core::Result<SpatialCoordinateSystem>;
    fn TryGetRelativeHeadingAtTimestamp(&self, timestamp: &::core::option::Option<super::PerceptionTimestamp>) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialLocatorPositionalTrackingDeactivatingEventArgsImpl: Sized {
    fn Canceled(&self) -> ::windows::core::Result<bool>;
    fn SetCanceled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialLocatorStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<SpatialLocator>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialStageFrameOfReferenceImpl: Sized {
    fn CoordinateSystem(&self) -> ::windows::core::Result<SpatialCoordinateSystem>;
    fn MovementRange(&self) -> ::windows::core::Result<SpatialMovementRange>;
    fn LookDirectionRange(&self) -> ::windows::core::Result<SpatialLookDirectionRange>;
    fn GetCoordinateSystemAtCurrentLocation(&self, locator: &::core::option::Option<SpatialLocator>) -> ::windows::core::Result<SpatialCoordinateSystem>;
    fn TryGetMovementBounds(&self, coordinatesystem: &::core::option::Option<SpatialCoordinateSystem>) -> ::windows::core::Result<::windows::core::Array<super::super::Foundation::Numerics::Vector3>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialStageFrameOfReferenceStaticsImpl: Sized {
    fn Current(&self) -> ::windows::core::Result<SpatialStageFrameOfReference>;
    fn CurrentChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCurrentChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RequestNewStageAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpatialStageFrameOfReference>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialStationaryFrameOfReferenceImpl: Sized {
    fn CoordinateSystem(&self) -> ::windows::core::Result<SpatialCoordinateSystem>;
}
