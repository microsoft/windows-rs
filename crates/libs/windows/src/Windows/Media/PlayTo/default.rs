impl ::core::cmp::PartialEq for CurrentTimeChangeRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CurrentTimeChangeRequestedEventArgs {}
impl ::core::fmt::Debug for CurrentTimeChangeRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CurrentTimeChangeRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MuteChangeRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MuteChangeRequestedEventArgs {}
impl ::core::fmt::Debug for MuteChangeRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MuteChangeRequestedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PlayToConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PlayToConnection {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToConnection").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for PlayToConnectionError {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToConnectionError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToConnectionError").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PlayToConnectionErrorEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PlayToConnectionErrorEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToConnectionErrorEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToConnectionErrorEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for PlayToConnectionState {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToConnectionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToConnectionState").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PlayToConnectionStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PlayToConnectionStateChangedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToConnectionStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToConnectionStateChangedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PlayToConnectionTransferredEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PlayToConnectionTransferredEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToConnectionTransferredEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToConnectionTransferredEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PlayToManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PlayToManager {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PlayToReceiver {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlayToReceiver {}
impl ::core::fmt::Debug for PlayToReceiver {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToReceiver").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PlayToSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PlayToSource {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToSource").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PlayToSourceDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PlayToSourceDeferral {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToSourceDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToSourceDeferral").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PlayToSourceRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PlayToSourceRequest {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToSourceRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToSourceRequest").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PlayToSourceRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PlayToSourceRequestedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToSourceRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToSourceRequestedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PlayToSourceSelectedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PlayToSourceSelectedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToSourceSelectedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToSourceSelectedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PlaybackRateChangeRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlaybackRateChangeRequestedEventArgs {}
impl ::core::fmt::Debug for PlaybackRateChangeRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaybackRateChangeRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for SourceChangeRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SourceChangeRequestedEventArgs {}
impl ::core::fmt::Debug for SourceChangeRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SourceChangeRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for VolumeChangeRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VolumeChangeRequestedEventArgs {}
impl ::core::fmt::Debug for VolumeChangeRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VolumeChangeRequestedEventArgs").field(&self.0).finish()
    }
}
