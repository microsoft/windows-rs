impl ::core::default::Default for QStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for QStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QStatus").field(&self.0).finish()
    }
}
impl ::core::default::Default for alljoyn_about_announceflag {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for alljoyn_about_announceflag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_about_announceflag").field(&self.0).finish()
    }
}
impl ::core::default::Default for alljoyn_aboutdatalistener_callbacks {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for alljoyn_aboutlistener_callback {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for alljoyn_applicationstate {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for alljoyn_applicationstate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_applicationstate").field(&self.0).finish()
    }
}
impl ::core::default::Default for alljoyn_applicationstatelistener_callbacks {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for alljoyn_authlistener_callbacks {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for alljoyn_authlistenerasync_callbacks {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for alljoyn_buslistener_callbacks {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for alljoyn_busobject_callbacks {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for alljoyn_busobject_methodentry {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for alljoyn_certificateid {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for alljoyn_certificateid {
    fn eq(&self, other: &Self) -> bool {
        self.serial == other.serial && self.serialLen == other.serialLen && self.issuerPublicKey == other.issuerPublicKey && self.issuerAki == other.issuerAki && self.issuerAkiLen == other.issuerAkiLen
    }
}
impl ::core::cmp::Eq for alljoyn_certificateid {}
impl ::core::fmt::Debug for alljoyn_certificateid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("alljoyn_certificateid").field("serial", &self.serial).field("serialLen", &self.serialLen).field("issuerPublicKey", &self.issuerPublicKey).field("issuerAki", &self.issuerAki).field("issuerAkiLen", &self.issuerAkiLen).finish()
    }
}
impl ::core::default::Default for alljoyn_certificateidarray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for alljoyn_certificateidarray {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.ids == other.ids
    }
}
impl ::core::cmp::Eq for alljoyn_certificateidarray {}
impl ::core::fmt::Debug for alljoyn_certificateidarray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("alljoyn_certificateidarray").field("count", &self.count).field("ids", &self.ids).finish()
    }
}
impl ::core::default::Default for alljoyn_claimcapability_masks {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for alljoyn_claimcapability_masks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_claimcapability_masks").field(&self.0).finish()
    }
}
impl ::core::default::Default for alljoyn_claimcapabilityadditionalinfo_masks {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for alljoyn_claimcapabilityadditionalinfo_masks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_claimcapabilityadditionalinfo_masks").field(&self.0).finish()
    }
}
impl ::core::default::Default for alljoyn_interfacedescription_member {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for alljoyn_interfacedescription_member {
    fn eq(&self, other: &Self) -> bool {
        self.iface == other.iface && self.memberType == other.memberType && self.name == other.name && self.signature == other.signature && self.returnSignature == other.returnSignature && self.argNames == other.argNames && self.internal_member == other.internal_member
    }
}
impl ::core::cmp::Eq for alljoyn_interfacedescription_member {}
impl ::core::fmt::Debug for alljoyn_interfacedescription_member {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("alljoyn_interfacedescription_member").field("iface", &self.iface).field("memberType", &self.memberType).field("name", &self.name).field("signature", &self.signature).field("returnSignature", &self.returnSignature).field("argNames", &self.argNames).field("internal_member", &self.internal_member).finish()
    }
}
impl ::core::default::Default for alljoyn_interfacedescription_property {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for alljoyn_interfacedescription_property {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.signature == other.signature && self.access == other.access && self.internal_property == other.internal_property
    }
}
impl ::core::cmp::Eq for alljoyn_interfacedescription_property {}
impl ::core::fmt::Debug for alljoyn_interfacedescription_property {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("alljoyn_interfacedescription_property").field("name", &self.name).field("signature", &self.signature).field("access", &self.access).field("internal_property", &self.internal_property).finish()
    }
}
impl ::core::default::Default for alljoyn_interfacedescription_securitypolicy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for alljoyn_interfacedescription_securitypolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_interfacedescription_securitypolicy").field(&self.0).finish()
    }
}
impl ::core::default::Default for alljoyn_keystorelistener_callbacks {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for alljoyn_keystorelistener_with_synchronization_callbacks {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for alljoyn_manifestarray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for alljoyn_manifestarray {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.xmls == other.xmls
    }
}
impl ::core::cmp::Eq for alljoyn_manifestarray {}
impl ::core::fmt::Debug for alljoyn_manifestarray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("alljoyn_manifestarray").field("count", &self.count).field("xmls", &self.xmls).finish()
    }
}
impl ::core::default::Default for alljoyn_messagetype {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for alljoyn_messagetype {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_messagetype").field(&self.0).finish()
    }
}
impl ::core::default::Default for alljoyn_observerlistener_callback {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for alljoyn_permissionconfigurationlistener_callbacks {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for alljoyn_pinglistener_callback {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for alljoyn_sessionlistener_callbacks {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for alljoyn_sessionlostreason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for alljoyn_sessionlostreason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_sessionlostreason").field(&self.0).finish()
    }
}
impl ::core::default::Default for alljoyn_sessionportlistener_callbacks {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for alljoyn_typeid {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for alljoyn_typeid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("alljoyn_typeid").field(&self.0).finish()
    }
}
