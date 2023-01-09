impl ::core::cmp::PartialEq for Appointment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Appointment {}
impl ::core::fmt::Debug for Appointment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Appointment").field(&self.0).finish()
    }
}
impl ::core::default::Default for AppointmentBusyStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AppointmentBusyStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentBusyStatus").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for AppointmentCalendar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentCalendar {}
impl ::core::fmt::Debug for AppointmentCalendar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentCalendar").field(&self.0).finish()
    }
}
impl ::core::default::Default for AppointmentCalendarOtherAppReadAccess {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AppointmentCalendarOtherAppReadAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentCalendarOtherAppReadAccess").field(&self.0).finish()
    }
}
impl ::core::default::Default for AppointmentCalendarOtherAppWriteAccess {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AppointmentCalendarOtherAppWriteAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentCalendarOtherAppWriteAccess").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for AppointmentCalendarSyncManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentCalendarSyncManager {}
impl ::core::fmt::Debug for AppointmentCalendarSyncManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentCalendarSyncManager").field(&self.0).finish()
    }
}
impl ::core::default::Default for AppointmentCalendarSyncStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AppointmentCalendarSyncStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentCalendarSyncStatus").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for AppointmentConflictResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentConflictResult {}
impl ::core::fmt::Debug for AppointmentConflictResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentConflictResult").field(&self.0).finish()
    }
}
impl ::core::default::Default for AppointmentConflictType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AppointmentConflictType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentConflictType").field(&self.0).finish()
    }
}
impl ::core::default::Default for AppointmentDaysOfWeek {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AppointmentDaysOfWeek {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentDaysOfWeek").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for AppointmentDaysOfWeek {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AppointmentDaysOfWeek {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AppointmentDaysOfWeek {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AppointmentDaysOfWeek {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AppointmentDaysOfWeek {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for AppointmentDetailsKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AppointmentDetailsKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentDetailsKind").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for AppointmentException {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentException {}
impl ::core::fmt::Debug for AppointmentException {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentException").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for AppointmentInvitee {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentInvitee {}
impl ::core::fmt::Debug for AppointmentInvitee {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentInvitee").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for AppointmentManagerForUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentManagerForUser {}
impl ::core::fmt::Debug for AppointmentManagerForUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentManagerForUser").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for AppointmentOrganizer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentOrganizer {}
impl ::core::fmt::Debug for AppointmentOrganizer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentOrganizer").field(&self.0).finish()
    }
}
impl ::core::default::Default for AppointmentParticipantResponse {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AppointmentParticipantResponse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentParticipantResponse").field(&self.0).finish()
    }
}
impl ::core::default::Default for AppointmentParticipantRole {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AppointmentParticipantRole {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentParticipantRole").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for AppointmentRecurrence {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentRecurrence {}
impl ::core::fmt::Debug for AppointmentRecurrence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentRecurrence").field(&self.0).finish()
    }
}
impl ::core::default::Default for AppointmentRecurrenceUnit {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AppointmentRecurrenceUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentRecurrenceUnit").field(&self.0).finish()
    }
}
impl ::core::default::Default for AppointmentSensitivity {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AppointmentSensitivity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentSensitivity").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for AppointmentStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentStore {}
impl ::core::fmt::Debug for AppointmentStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentStore").field(&self.0).finish()
    }
}
impl ::core::default::Default for AppointmentStoreAccessType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AppointmentStoreAccessType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentStoreAccessType").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for AppointmentStoreChange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentStoreChange {}
impl ::core::fmt::Debug for AppointmentStoreChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentStoreChange").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for AppointmentStoreChangeReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentStoreChangeReader {}
impl ::core::fmt::Debug for AppointmentStoreChangeReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentStoreChangeReader").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for AppointmentStoreChangeTracker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentStoreChangeTracker {}
impl ::core::fmt::Debug for AppointmentStoreChangeTracker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentStoreChangeTracker").field(&self.0).finish()
    }
}
impl ::core::default::Default for AppointmentStoreChangeType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AppointmentStoreChangeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentStoreChangeType").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for AppointmentStoreChangedDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentStoreChangedDeferral {}
impl ::core::fmt::Debug for AppointmentStoreChangedDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentStoreChangedDeferral").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for AppointmentStoreChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentStoreChangedEventArgs {}
impl ::core::fmt::Debug for AppointmentStoreChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentStoreChangedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for AppointmentStoreNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentStoreNotificationTriggerDetails {}
impl ::core::fmt::Debug for AppointmentStoreNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentStoreNotificationTriggerDetails").field(&self.0).finish()
    }
}
impl ::core::default::Default for AppointmentSummaryCardView {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AppointmentSummaryCardView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentSummaryCardView").field(&self.0).finish()
    }
}
impl ::core::default::Default for AppointmentWeekOfMonth {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AppointmentWeekOfMonth {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentWeekOfMonth").field(&self.0).finish()
    }
}
impl ::core::default::Default for FindAppointmentCalendarsOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FindAppointmentCalendarsOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FindAppointmentCalendarsOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for FindAppointmentCalendarsOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FindAppointmentCalendarsOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FindAppointmentCalendarsOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FindAppointmentCalendarsOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FindAppointmentCalendarsOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::cmp::PartialEq for FindAppointmentsOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FindAppointmentsOptions {}
impl ::core::fmt::Debug for FindAppointmentsOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FindAppointmentsOptions").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAppointmentParticipant {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppointmentParticipant {}
impl ::core::fmt::Debug for IAppointmentParticipant {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppointmentParticipant").field(&self.0).finish()
    }
}
impl ::core::default::Default for RecurrenceType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RecurrenceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RecurrenceType").field(&self.0).finish()
    }
}
