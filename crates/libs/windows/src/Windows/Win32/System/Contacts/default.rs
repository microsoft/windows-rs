impl ::core::default::Default for CONTACT_AGGREGATION_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CONTACT_AGGREGATION_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.dwCount == other.dwCount && self.lpb == other.lpb
    }
}
impl ::core::cmp::Eq for CONTACT_AGGREGATION_BLOB {}
impl ::core::fmt::Debug for CONTACT_AGGREGATION_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONTACT_AGGREGATION_BLOB").field("dwCount", &self.dwCount).field("lpb", &self.lpb).finish()
    }
}
impl ::core::default::Default for CONTACT_AGGREGATION_COLLECTION_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CONTACT_AGGREGATION_COLLECTION_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONTACT_AGGREGATION_COLLECTION_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IContact {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContact {}
impl ::core::fmt::Debug for IContact {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContact").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IContactAggregationAggregate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactAggregationAggregate {}
impl ::core::fmt::Debug for IContactAggregationAggregate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactAggregationAggregate").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IContactAggregationAggregateCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactAggregationAggregateCollection {}
impl ::core::fmt::Debug for IContactAggregationAggregateCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactAggregationAggregateCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IContactAggregationContact {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactAggregationContact {}
impl ::core::fmt::Debug for IContactAggregationContact {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactAggregationContact").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IContactAggregationContactCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactAggregationContactCollection {}
impl ::core::fmt::Debug for IContactAggregationContactCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactAggregationContactCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IContactAggregationGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactAggregationGroup {}
impl ::core::fmt::Debug for IContactAggregationGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactAggregationGroup").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IContactAggregationGroupCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactAggregationGroupCollection {}
impl ::core::fmt::Debug for IContactAggregationGroupCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactAggregationGroupCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IContactAggregationLink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactAggregationLink {}
impl ::core::fmt::Debug for IContactAggregationLink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactAggregationLink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IContactAggregationLinkCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactAggregationLinkCollection {}
impl ::core::fmt::Debug for IContactAggregationLinkCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactAggregationLinkCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IContactAggregationManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactAggregationManager {}
impl ::core::fmt::Debug for IContactAggregationManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactAggregationManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IContactAggregationServerPerson {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactAggregationServerPerson {}
impl ::core::fmt::Debug for IContactAggregationServerPerson {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactAggregationServerPerson").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IContactAggregationServerPersonCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactAggregationServerPersonCollection {}
impl ::core::fmt::Debug for IContactAggregationServerPersonCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactAggregationServerPersonCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IContactCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactCollection {}
impl ::core::fmt::Debug for IContactCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IContactManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactManager {}
impl ::core::fmt::Debug for IContactManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IContactProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactProperties {}
impl ::core::fmt::Debug for IContactProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactProperties").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IContactPropertyCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactPropertyCollection {}
impl ::core::fmt::Debug for IContactPropertyCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactPropertyCollection").field(&self.0).finish()
    }
}
