impl ::core::default::Default for CausalityRelation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CausalityRelation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CausalityRelation").field(&self.0).finish()
    }
}
impl ::core::default::Default for CausalitySource {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CausalitySource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CausalitySource").field(&self.0).finish()
    }
}
impl ::core::default::Default for CausalitySynchronousWork {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CausalitySynchronousWork {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CausalitySynchronousWork").field(&self.0).finish()
    }
}
impl ::core::default::Default for CausalityTraceLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CausalityTraceLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CausalityTraceLevel").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ErrorDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ErrorDetails {}
impl ::core::fmt::Debug for ErrorDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ErrorDetails").field(&self.0).finish()
    }
}
impl ::core::default::Default for ErrorOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ErrorOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ErrorOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ErrorOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ErrorOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ErrorOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ErrorOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ErrorOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::cmp::PartialEq for FileLoggingSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileLoggingSession {}
impl ::core::fmt::Debug for FileLoggingSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileLoggingSession").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IErrorReportingSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IErrorReportingSettings {}
impl ::core::fmt::Debug for IErrorReportingSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IErrorReportingSettings").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFileLoggingSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileLoggingSession {}
impl ::core::fmt::Debug for IFileLoggingSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileLoggingSession").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ILoggingChannel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILoggingChannel {}
impl ::core::fmt::Debug for ILoggingChannel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILoggingChannel").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ILoggingSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILoggingSession {}
impl ::core::fmt::Debug for ILoggingSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILoggingSession").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ILoggingTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILoggingTarget {}
impl ::core::fmt::Debug for ILoggingTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILoggingTarget").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for LogFileGeneratedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LogFileGeneratedEventArgs {}
impl ::core::fmt::Debug for LogFileGeneratedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LogFileGeneratedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for LoggingActivity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LoggingActivity {}
impl ::core::fmt::Debug for LoggingActivity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LoggingActivity").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for LoggingChannel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LoggingChannel {}
impl ::core::fmt::Debug for LoggingChannel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LoggingChannel").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for LoggingChannelOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LoggingChannelOptions {}
impl ::core::fmt::Debug for LoggingChannelOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LoggingChannelOptions").field(&self.0).finish()
    }
}
impl ::core::default::Default for LoggingFieldFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LoggingFieldFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LoggingFieldFormat").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for LoggingFields {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LoggingFields {}
impl ::core::fmt::Debug for LoggingFields {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LoggingFields").field(&self.0).finish()
    }
}
impl ::core::default::Default for LoggingLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LoggingLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LoggingLevel").field(&self.0).finish()
    }
}
impl ::core::default::Default for LoggingOpcode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LoggingOpcode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LoggingOpcode").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for LoggingOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LoggingOptions {}
impl ::core::fmt::Debug for LoggingOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LoggingOptions").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for LoggingSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LoggingSession {}
impl ::core::fmt::Debug for LoggingSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LoggingSession").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for RuntimeBrokerErrorSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RuntimeBrokerErrorSettings {}
impl ::core::fmt::Debug for RuntimeBrokerErrorSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RuntimeBrokerErrorSettings").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for TracingStatusChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TracingStatusChangedEventArgs {}
impl ::core::fmt::Debug for TracingStatusChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TracingStatusChangedEventArgs").field(&self.0).finish()
    }
}
