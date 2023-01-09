impl ::core::cmp::PartialEq for AsyncActionCompletedHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncActionCompletedHandler {}
impl ::core::fmt::Debug for AsyncActionCompletedHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncActionCompletedHandler").field(&self.0).finish()
    }
}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::cmp::PartialEq for AsyncActionProgressHandler<TProgress> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::cmp::Eq for AsyncActionProgressHandler<TProgress> {}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::fmt::Debug for AsyncActionProgressHandler<TProgress> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncActionProgressHandler").field(&self.0).finish()
    }
}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::cmp::PartialEq for AsyncActionWithProgressCompletedHandler<TProgress> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::cmp::Eq for AsyncActionWithProgressCompletedHandler<TProgress> {}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::fmt::Debug for AsyncActionWithProgressCompletedHandler<TProgress> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncActionWithProgressCompletedHandler").field(&self.0).finish()
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static> ::core::cmp::PartialEq for AsyncOperationCompletedHandler<TResult> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static> ::core::cmp::Eq for AsyncOperationCompletedHandler<TResult> {}
impl<TResult: ::windows::core::RuntimeType + 'static> ::core::fmt::Debug for AsyncOperationCompletedHandler<TResult> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncOperationCompletedHandler").field(&self.0).finish()
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::cmp::PartialEq for AsyncOperationProgressHandler<TResult, TProgress> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::cmp::Eq for AsyncOperationProgressHandler<TResult, TProgress> {}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::fmt::Debug for AsyncOperationProgressHandler<TResult, TProgress> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncOperationProgressHandler").field(&self.0).finish()
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::cmp::PartialEq for AsyncOperationWithProgressCompletedHandler<TResult, TProgress> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::cmp::Eq for AsyncOperationWithProgressCompletedHandler<TResult, TProgress> {}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::fmt::Debug for AsyncOperationWithProgressCompletedHandler<TResult, TProgress> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncOperationWithProgressCompletedHandler").field(&self.0).finish()
    }
}
impl ::core::default::Default for AsyncStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AsyncStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncStatus").field(&self.0).finish()
    }
}
impl ::core::default::Default for DateTime {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DateTime {
    fn eq(&self, other: &Self) -> bool {
        self.UniversalTime == other.UniversalTime
    }
}
impl ::core::cmp::Eq for DateTime {}
impl ::core::fmt::Debug for DateTime {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DateTime").field("UniversalTime", &self.UniversalTime).finish()
    }
}
impl ::core::cmp::PartialEq for Deferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Deferral {}
impl ::core::fmt::Debug for Deferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Deferral").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for DeferralCompletedHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeferralCompletedHandler {}
impl ::core::fmt::Debug for DeferralCompletedHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeferralCompletedHandler").field(&self.0).finish()
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::cmp::PartialEq for EventHandler<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::cmp::Eq for EventHandler<T> {}
impl<T: ::windows::core::RuntimeType + 'static> ::core::fmt::Debug for EventHandler<T> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EventHandler").field(&self.0).finish()
    }
}
impl ::core::default::Default for EventRegistrationToken {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EventRegistrationToken {
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value
    }
}
impl ::core::cmp::Eq for EventRegistrationToken {}
impl ::core::fmt::Debug for EventRegistrationToken {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EventRegistrationToken").field("Value", &self.Value).finish()
    }
}
impl ::core::cmp::PartialEq for IAsyncAction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAsyncAction {}
impl ::core::fmt::Debug for IAsyncAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAsyncAction").field(&self.0).finish()
    }
}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::cmp::PartialEq for IAsyncActionWithProgress<TProgress> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::cmp::Eq for IAsyncActionWithProgress<TProgress> {}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::fmt::Debug for IAsyncActionWithProgress<TProgress> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAsyncActionWithProgress").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAsyncInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAsyncInfo {}
impl ::core::fmt::Debug for IAsyncInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAsyncInfo").field(&self.0).finish()
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static> ::core::cmp::PartialEq for IAsyncOperation<TResult> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static> ::core::cmp::Eq for IAsyncOperation<TResult> {}
impl<TResult: ::windows::core::RuntimeType + 'static> ::core::fmt::Debug for IAsyncOperation<TResult> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAsyncOperation").field(&self.0).finish()
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::cmp::PartialEq for IAsyncOperationWithProgress<TResult, TProgress> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::cmp::Eq for IAsyncOperationWithProgress<TResult, TProgress> {}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::fmt::Debug for IAsyncOperationWithProgress<TResult, TProgress> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAsyncOperationWithProgress").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IClosable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IClosable {}
impl ::core::fmt::Debug for IClosable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IClosable").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IGetActivationFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGetActivationFactory {}
impl ::core::fmt::Debug for IGetActivationFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGetActivationFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMemoryBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMemoryBuffer {}
impl ::core::fmt::Debug for IMemoryBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMemoryBuffer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMemoryBufferReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMemoryBufferReference {}
impl ::core::fmt::Debug for IMemoryBufferReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMemoryBufferReference").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPropertyValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyValue {}
impl ::core::fmt::Debug for IPropertyValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyValue").field(&self.0).finish()
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::cmp::PartialEq for IReference<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::cmp::Eq for IReference<T> {}
impl<T: ::windows::core::RuntimeType + 'static> ::core::fmt::Debug for IReference<T> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReference").field(&self.0).finish()
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::cmp::PartialEq for IReferenceArray<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::cmp::Eq for IReferenceArray<T> {}
impl<T: ::windows::core::RuntimeType + 'static> ::core::fmt::Debug for IReferenceArray<T> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReferenceArray").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IStringable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStringable {}
impl ::core::fmt::Debug for IStringable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStringable").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWwwFormUrlDecoderEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWwwFormUrlDecoderEntry {}
impl ::core::fmt::Debug for IWwwFormUrlDecoderEntry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWwwFormUrlDecoderEntry").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MemoryBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MemoryBuffer {}
impl ::core::fmt::Debug for MemoryBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MemoryBuffer").field(&self.0).finish()
    }
}
impl ::core::default::Default for Point {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y
    }
}
impl ::core::cmp::Eq for Point {}
impl ::core::fmt::Debug for Point {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Point").field("X", &self.X).field("Y", &self.Y).finish()
    }
}
impl ::core::default::Default for PropertyType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PropertyType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PropertyType").field(&self.0).finish()
    }
}
impl ::core::default::Default for Rect {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y && self.Width == other.Width && self.Height == other.Height
    }
}
impl ::core::cmp::Eq for Rect {}
impl ::core::fmt::Debug for Rect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Rect").field("X", &self.X).field("Y", &self.Y).field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
impl ::core::default::Default for Size {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for Size {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height
    }
}
impl ::core::cmp::Eq for Size {}
impl ::core::fmt::Debug for Size {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Size").field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
impl ::core::default::Default for TimeSpan {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TimeSpan {
    fn eq(&self, other: &Self) -> bool {
        self.Duration == other.Duration
    }
}
impl ::core::cmp::Eq for TimeSpan {}
impl ::core::fmt::Debug for TimeSpan {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TimeSpan").field("Duration", &self.Duration).finish()
    }
}
impl<TSender: ::windows::core::RuntimeType + 'static, TResult: ::windows::core::RuntimeType + 'static> ::core::cmp::PartialEq for TypedEventHandler<TSender, TResult> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<TSender: ::windows::core::RuntimeType + 'static, TResult: ::windows::core::RuntimeType + 'static> ::core::cmp::Eq for TypedEventHandler<TSender, TResult> {}
impl<TSender: ::windows::core::RuntimeType + 'static, TResult: ::windows::core::RuntimeType + 'static> ::core::fmt::Debug for TypedEventHandler<TSender, TResult> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TypedEventHandler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for Uri {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Uri {}
impl ::core::fmt::Debug for Uri {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Uri").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for WwwFormUrlDecoder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WwwFormUrlDecoder {}
impl ::core::fmt::Debug for WwwFormUrlDecoder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WwwFormUrlDecoder").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for WwwFormUrlDecoderEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WwwFormUrlDecoderEntry {}
impl ::core::fmt::Debug for WwwFormUrlDecoderEntry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WwwFormUrlDecoderEntry").field(&self.0).finish()
    }
}
