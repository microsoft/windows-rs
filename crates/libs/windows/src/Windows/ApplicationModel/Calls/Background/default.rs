impl ::core::default::Default for PhoneCallBlockedReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PhoneCallBlockedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallBlockedReason").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PhoneCallBlockedTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneCallBlockedTriggerDetails {}
impl ::core::fmt::Debug for PhoneCallBlockedTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallBlockedTriggerDetails").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PhoneCallOriginDataRequestTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PhoneCallOriginDataRequestTriggerDetails {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PhoneCallOriginDataRequestTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallOriginDataRequestTriggerDetails").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for PhoneIncomingCallDismissedReason {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PhoneIncomingCallDismissedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneIncomingCallDismissedReason").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PhoneIncomingCallDismissedTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PhoneIncomingCallDismissedTriggerDetails {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PhoneIncomingCallDismissedTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneIncomingCallDismissedTriggerDetails").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PhoneIncomingCallNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneIncomingCallNotificationTriggerDetails {}
impl ::core::fmt::Debug for PhoneIncomingCallNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneIncomingCallNotificationTriggerDetails").field(&self.0).finish()
    }
}
impl ::core::default::Default for PhoneLineChangeKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PhoneLineChangeKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLineChangeKind").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PhoneLineChangedTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneLineChangedTriggerDetails {}
impl ::core::fmt::Debug for PhoneLineChangedTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLineChangedTriggerDetails").field(&self.0).finish()
    }
}
impl ::core::default::Default for PhoneLineProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PhoneLineProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneLineProperties").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PhoneLineProperties {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PhoneLineProperties {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PhoneLineProperties {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PhoneLineProperties {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PhoneLineProperties {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::cmp::PartialEq for PhoneNewVoicemailMessageTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneNewVoicemailMessageTriggerDetails {}
impl ::core::fmt::Debug for PhoneNewVoicemailMessageTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneNewVoicemailMessageTriggerDetails").field(&self.0).finish()
    }
}
impl ::core::default::Default for PhoneTriggerType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PhoneTriggerType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneTriggerType").field(&self.0).finish()
    }
}
