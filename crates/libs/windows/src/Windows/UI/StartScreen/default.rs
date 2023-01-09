impl ::core::default::Default for ForegroundText {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ForegroundText {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ForegroundText").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for JumpList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for JumpList {}
impl ::core::fmt::Debug for JumpList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JumpList").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for JumpListItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for JumpListItem {}
impl ::core::fmt::Debug for JumpListItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JumpListItem").field(&self.0).finish()
    }
}
impl ::core::default::Default for JumpListItemKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for JumpListItemKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JumpListItemKind").field(&self.0).finish()
    }
}
impl ::core::default::Default for JumpListSystemGroupKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for JumpListSystemGroupKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JumpListSystemGroupKind").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for SecondaryTile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SecondaryTile {}
impl ::core::fmt::Debug for SecondaryTile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecondaryTile").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for SecondaryTileVisualElements {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SecondaryTileVisualElements {}
impl ::core::fmt::Debug for SecondaryTileVisualElements {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecondaryTileVisualElements").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for StartScreenManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StartScreenManager {}
impl ::core::fmt::Debug for StartScreenManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StartScreenManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for TileMixedRealityModel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TileMixedRealityModel {}
impl ::core::fmt::Debug for TileMixedRealityModel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TileMixedRealityModel").field(&self.0).finish()
    }
}
impl ::core::default::Default for TileMixedRealityModelActivationBehavior {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TileMixedRealityModelActivationBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TileMixedRealityModelActivationBehavior").field(&self.0).finish()
    }
}
impl ::core::default::Default for TileOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TileOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TileOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TileOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TileOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TileOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TileOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TileOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for TileSize {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TileSize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TileSize").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for VisualElementsRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VisualElementsRequest {}
impl ::core::fmt::Debug for VisualElementsRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VisualElementsRequest").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for VisualElementsRequestDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VisualElementsRequestDeferral {}
impl ::core::fmt::Debug for VisualElementsRequestDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VisualElementsRequestDeferral").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for VisualElementsRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VisualElementsRequestedEventArgs {}
impl ::core::fmt::Debug for VisualElementsRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VisualElementsRequestedEventArgs").field(&self.0).finish()
    }
}
