impl ::core::default::Default for HCN_NOTIFICATIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HCN_NOTIFICATIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HCN_NOTIFICATIONS").field(&self.0).finish()
    }
}
impl ::core::default::Default for HCN_PORT_ACCESS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HCN_PORT_ACCESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HCN_PORT_ACCESS").field(&self.0).finish()
    }
}
impl ::core::default::Default for HCN_PORT_PROTOCOL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HCN_PORT_PROTOCOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HCN_PORT_PROTOCOL").field(&self.0).finish()
    }
}
impl ::core::default::Default for HCN_PORT_RANGE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HCN_PORT_RANGE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.OwningPartitionId == other.OwningPartitionId && self.TargetPartitionId == other.TargetPartitionId && self.Protocol == other.Protocol && self.Priority == other.Priority && self.ReservationType == other.ReservationType && self.SharingFlags == other.SharingFlags && self.DeliveryMode == other.DeliveryMode && self.StartingPort == other.StartingPort && self.EndingPort == other.EndingPort
    }
}
impl ::core::cmp::Eq for HCN_PORT_RANGE_ENTRY {}
impl ::core::fmt::Debug for HCN_PORT_RANGE_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HCN_PORT_RANGE_ENTRY").field("OwningPartitionId", &self.OwningPartitionId).field("TargetPartitionId", &self.TargetPartitionId).field("Protocol", &self.Protocol).field("Priority", &self.Priority).field("ReservationType", &self.ReservationType).field("SharingFlags", &self.SharingFlags).field("DeliveryMode", &self.DeliveryMode).field("StartingPort", &self.StartingPort).field("EndingPort", &self.EndingPort).finish()
    }
}
impl ::core::default::Default for HCN_PORT_RANGE_RESERVATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HCN_PORT_RANGE_RESERVATION {
    fn eq(&self, other: &Self) -> bool {
        self.startingPort == other.startingPort && self.endingPort == other.endingPort
    }
}
impl ::core::cmp::Eq for HCN_PORT_RANGE_RESERVATION {}
impl ::core::fmt::Debug for HCN_PORT_RANGE_RESERVATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HCN_PORT_RANGE_RESERVATION").field("startingPort", &self.startingPort).field("endingPort", &self.endingPort).finish()
    }
}
