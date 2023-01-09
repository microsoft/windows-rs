impl ::core::cmp::PartialEq for NamedResource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NamedResource {}
impl ::core::fmt::Debug for NamedResource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NamedResource").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ResourceCandidate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ResourceCandidate {}
impl ::core::fmt::Debug for ResourceCandidate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceCandidate").field(&self.0).finish()
    }
}
impl ::core::default::Default for ResourceCandidateKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ResourceCandidateKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceCandidateKind").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for ResourceCandidateVectorView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for ResourceCandidateVectorView {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for ResourceCandidateVectorView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceCandidateVectorView").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ResourceContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ResourceContext {}
impl ::core::fmt::Debug for ResourceContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceContext").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for ResourceContextLanguagesVectorView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for ResourceContextLanguagesVectorView {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for ResourceContextLanguagesVectorView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceContextLanguagesVectorView").field(&self.0).finish()
    }
}
impl ::core::default::Default for ResourceLayoutInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ResourceLayoutInfo {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion && self.ResourceSubtreeCount == other.ResourceSubtreeCount && self.NamedResourceCount == other.NamedResourceCount && self.Checksum == other.Checksum
    }
}
impl ::core::cmp::Eq for ResourceLayoutInfo {}
impl ::core::fmt::Debug for ResourceLayoutInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ResourceLayoutInfo").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).field("ResourceSubtreeCount", &self.ResourceSubtreeCount).field("NamedResourceCount", &self.NamedResourceCount).field("Checksum", &self.Checksum).finish()
    }
}
impl ::core::cmp::PartialEq for ResourceManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ResourceManager {}
impl ::core::fmt::Debug for ResourceManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ResourceMap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ResourceMap {}
impl ::core::fmt::Debug for ResourceMap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceMap").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for ResourceMapIterator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for ResourceMapIterator {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for ResourceMapIterator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceMapIterator").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for ResourceMapMapView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for ResourceMapMapView {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for ResourceMapMapView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceMapMapView").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for ResourceMapMapViewIterator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for ResourceMapMapViewIterator {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for ResourceMapMapViewIterator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceMapMapViewIterator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ResourceQualifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ResourceQualifier {}
impl ::core::fmt::Debug for ResourceQualifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceQualifier").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for ResourceQualifierMapView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for ResourceQualifierMapView {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for ResourceQualifierMapView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceQualifierMapView").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for ResourceQualifierObservableMap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for ResourceQualifierObservableMap {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for ResourceQualifierObservableMap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceQualifierObservableMap").field(&self.0).finish()
    }
}
impl ::core::default::Default for ResourceQualifierPersistence {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ResourceQualifierPersistence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceQualifierPersistence").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for ResourceQualifierVectorView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for ResourceQualifierVectorView {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for ResourceQualifierVectorView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceQualifierVectorView").field(&self.0).finish()
    }
}
