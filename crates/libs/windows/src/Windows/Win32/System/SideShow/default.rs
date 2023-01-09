impl ::core::default::Default for APPLICATION_EVENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CONTENT_MISSING_EVENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DEVICE_USER_CHANGE_EVENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for EVENT_DATA_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ISideShowBulkCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISideShowBulkCapabilities {}
impl ::core::fmt::Debug for ISideShowBulkCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISideShowBulkCapabilities").field(&self.0).finish()
    }
}
impl ISideShowBulkCapabilities {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetCapability(&self, in_keycapability: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, inout_pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetCapability)(::windows::core::Vtable::as_raw(self), in_keycapability, inout_pvalue).ok()
    }
}
impl ::core::cmp::PartialEq for ISideShowCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISideShowCapabilities {}
impl ::core::fmt::Debug for ISideShowCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISideShowCapabilities").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISideShowCapabilitiesCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISideShowCapabilitiesCollection {}
impl ::core::fmt::Debug for ISideShowCapabilitiesCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISideShowCapabilitiesCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISideShowContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISideShowContent {}
impl ::core::fmt::Debug for ISideShowContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISideShowContent").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISideShowContentManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISideShowContentManager {}
impl ::core::fmt::Debug for ISideShowContentManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISideShowContentManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISideShowEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISideShowEvents {}
impl ::core::fmt::Debug for ISideShowEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISideShowEvents").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISideShowKeyCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISideShowKeyCollection {}
impl ::core::fmt::Debug for ISideShowKeyCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISideShowKeyCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISideShowNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISideShowNotification {}
impl ::core::fmt::Debug for ISideShowNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISideShowNotification").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISideShowNotificationManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISideShowNotificationManager {}
impl ::core::fmt::Debug for ISideShowNotificationManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISideShowNotificationManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISideShowPropVariantCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISideShowPropVariantCollection {}
impl ::core::fmt::Debug for ISideShowPropVariantCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISideShowPropVariantCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISideShowSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISideShowSession {}
impl ::core::fmt::Debug for ISideShowSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISideShowSession").field(&self.0).finish()
    }
}
impl ::core::default::Default for NEW_EVENT_DATA_AVAILABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SCF_BUTTON_IDS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCF_BUTTON_IDS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCF_BUTTON_IDS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCF_CONTEXTMENU_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCF_CONTEXTMENU_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.PreviousPage == other.PreviousPage && self.TargetPage == other.TargetPage && self.PreviousItemId == other.PreviousItemId && self.MenuPage == other.MenuPage && self.MenuItemId == other.MenuItemId
    }
}
impl ::core::cmp::Eq for SCF_CONTEXTMENU_EVENT {}
impl ::core::fmt::Debug for SCF_CONTEXTMENU_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCF_CONTEXTMENU_EVENT").field("PreviousPage", &self.PreviousPage).field("TargetPage", &self.TargetPage).field("PreviousItemId", &self.PreviousItemId).field("MenuPage", &self.MenuPage).field("MenuItemId", &self.MenuItemId).finish()
    }
}
impl ::core::default::Default for SCF_EVENT_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCF_EVENT_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.PreviousPage == other.PreviousPage && self.TargetPage == other.TargetPage
    }
}
impl ::core::cmp::Eq for SCF_EVENT_HEADER {}
impl ::core::fmt::Debug for SCF_EVENT_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCF_EVENT_HEADER").field("PreviousPage", &self.PreviousPage).field("TargetPage", &self.TargetPage).finish()
    }
}
impl ::core::default::Default for SCF_EVENT_IDS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCF_EVENT_IDS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCF_EVENT_IDS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCF_MENUACTION_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCF_MENUACTION_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.PreviousPage == other.PreviousPage && self.TargetPage == other.TargetPage && self.Button == other.Button && self.ItemId == other.ItemId
    }
}
impl ::core::cmp::Eq for SCF_MENUACTION_EVENT {}
impl ::core::fmt::Debug for SCF_MENUACTION_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCF_MENUACTION_EVENT").field("PreviousPage", &self.PreviousPage).field("TargetPage", &self.TargetPage).field("Button", &self.Button).field("ItemId", &self.ItemId).finish()
    }
}
impl ::core::default::Default for SCF_NAVIGATION_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCF_NAVIGATION_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.PreviousPage == other.PreviousPage && self.TargetPage == other.TargetPage && self.Button == other.Button
    }
}
impl ::core::cmp::Eq for SCF_NAVIGATION_EVENT {}
impl ::core::fmt::Debug for SCF_NAVIGATION_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCF_NAVIGATION_EVENT").field("PreviousPage", &self.PreviousPage).field("TargetPage", &self.TargetPage).field("Button", &self.Button).finish()
    }
}
impl ::core::default::Default for SIDESHOW_COLOR_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SIDESHOW_COLOR_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SIDESHOW_COLOR_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SIDESHOW_SCREEN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SIDESHOW_SCREEN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SIDESHOW_SCREEN_TYPE").field(&self.0).finish()
    }
}
