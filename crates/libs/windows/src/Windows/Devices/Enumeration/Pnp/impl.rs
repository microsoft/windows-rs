#[cfg(feature = "implement_exclusive")]
pub trait IPnpObjectImpl: Sized {
    fn Type(&self) -> ::windows::core::Result<PnpObjectType>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
    fn Update(&self, updateinfo: &::core::option::Option<PnpObjectUpdate>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPnpObjectStaticsImpl: Sized {
    fn CreateFromIdAsync(&self, r#type: PnpObjectType, id: &::windows::core::HSTRING, requestedproperties: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<PnpObject>>;
    fn FindAllAsync(&self, r#type: PnpObjectType, requestedproperties: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<PnpObjectCollection>>;
    fn FindAllAsyncAqsFilter(&self, r#type: PnpObjectType, requestedproperties: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, aqsfilter: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<PnpObjectCollection>>;
    fn CreateWatcher(&self, r#type: PnpObjectType, requestedproperties: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<PnpObjectWatcher>;
    fn CreateWatcherAqsFilter(&self, r#type: PnpObjectType, requestedproperties: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, aqsfilter: &::windows::core::HSTRING) -> ::windows::core::Result<PnpObjectWatcher>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPnpObjectUpdateImpl: Sized {
    fn Type(&self) -> ::windows::core::Result<PnpObjectType>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPnpObjectWatcherImpl: Sized {
    fn Added(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, PnpObject>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAdded(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Updated(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, PnpObjectUpdate>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveUpdated(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Removed(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, PnpObjectUpdate>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemoved(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Stopped(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<PnpObjectWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Status(&self) -> ::windows::core::Result<super::DeviceWatcherStatus>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
}
