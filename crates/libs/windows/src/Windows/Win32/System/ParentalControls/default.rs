impl ::core::cmp::PartialEq for IWPCGamesSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWPCGamesSettings {}
impl ::core::fmt::Debug for IWPCGamesSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWPCGamesSettings").field(&self.0).finish()
    }
}
impl IWPCGamesSettings {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsLoggingRequired(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsLoggingRequired)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastSettingsChangeTime(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLastSettingsChangeTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetRestrictions(&self) -> ::windows::core::Result<WPCFLAG_RESTRICTION> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetRestrictions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IWPCProviderConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWPCProviderConfig {}
impl ::core::fmt::Debug for IWPCProviderConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWPCProviderConfig").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWPCProviderState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWPCProviderState {}
impl ::core::fmt::Debug for IWPCProviderState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWPCProviderState").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWPCProviderSupport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWPCProviderSupport {}
impl ::core::fmt::Debug for IWPCProviderSupport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWPCProviderSupport").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWPCSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWPCSettings {}
impl ::core::fmt::Debug for IWPCSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWPCSettings").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWPCWebSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWPCWebSettings {}
impl ::core::fmt::Debug for IWPCWebSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWPCWebSettings").field(&self.0).finish()
    }
}
impl IWPCWebSettings {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsLoggingRequired(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsLoggingRequired)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastSettingsChangeTime(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLastSettingsChangeTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetRestrictions(&self) -> ::windows::core::Result<WPCFLAG_RESTRICTION> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetRestrictions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IWindowsParentalControls {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWindowsParentalControls {}
impl ::core::fmt::Debug for IWindowsParentalControls {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWindowsParentalControls").field(&self.0).finish()
    }
}
impl IWindowsParentalControls {
    pub unsafe fn GetVisibility(&self) -> ::windows::core::Result<WPCFLAG_VISIBILITY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetVisibility)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUserSettings<P0>(&self, pcszsid: P0) -> ::windows::core::Result<IWPCSettings>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUserSettings)(::windows::core::Vtable::as_raw(self), pcszsid.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetWebSettings<P0>(&self, pcszsid: P0) -> ::windows::core::Result<IWPCWebSettings>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetWebSettings)(::windows::core::Vtable::as_raw(self), pcszsid.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetWebFilterInfo(&self, pguidid: *mut ::windows::core::GUID, ppszname: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetWebFilterInfo)(::windows::core::Vtable::as_raw(self), pguidid, ::core::mem::transmute(ppszname.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
impl ::core::cmp::PartialEq for IWindowsParentalControlsCore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWindowsParentalControlsCore {}
impl ::core::fmt::Debug for IWindowsParentalControlsCore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWindowsParentalControlsCore").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPCFLAG_IM_FEATURE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPCFLAG_IM_FEATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPCFLAG_IM_FEATURE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPCFLAG_IM_LEAVE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPCFLAG_IM_LEAVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPCFLAG_IM_LEAVE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPCFLAG_ISBLOCKED {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPCFLAG_ISBLOCKED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPCFLAG_ISBLOCKED").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPCFLAG_LOGOFF_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPCFLAG_LOGOFF_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPCFLAG_LOGOFF_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPCFLAG_OVERRIDE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPCFLAG_OVERRIDE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPCFLAG_OVERRIDE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPCFLAG_RESTRICTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPCFLAG_RESTRICTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPCFLAG_RESTRICTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPCFLAG_VISIBILITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPCFLAG_VISIBILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPCFLAG_VISIBILITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPCFLAG_WEB_SETTING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPCFLAG_WEB_SETTING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPCFLAG_WEB_SETTING").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPC_ARGS_APPLICATIONEVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPC_ARGS_APPLICATIONEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_APPLICATIONEVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPC_ARGS_APPOVERRIDEEVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPC_ARGS_APPOVERRIDEEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_APPOVERRIDEEVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPC_ARGS_COMPUTERUSAGEEVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPC_ARGS_COMPUTERUSAGEEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_COMPUTERUSAGEEVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPC_ARGS_CONTENTUSAGEEVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPC_ARGS_CONTENTUSAGEEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_CONTENTUSAGEEVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPC_ARGS_CONVERSATIONINITEVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPC_ARGS_CONVERSATIONINITEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_CONVERSATIONINITEVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPC_ARGS_CONVERSATIONJOINEVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPC_ARGS_CONVERSATIONJOINEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_CONVERSATIONJOINEVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPC_ARGS_CONVERSATIONLEAVEEVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPC_ARGS_CONVERSATIONLEAVEEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_CONVERSATIONLEAVEEVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPC_ARGS_CUSTOMEVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPC_ARGS_CUSTOMEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_CUSTOMEVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPC_ARGS_EMAILCONTACTEVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPC_ARGS_EMAILCONTACTEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_EMAILCONTACTEVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPC_ARGS_EMAILRECEIEVEDEVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPC_ARGS_EMAILRECEIEVEDEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_EMAILRECEIEVEDEVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPC_ARGS_EMAILSENTEVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPC_ARGS_EMAILSENTEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_EMAILSENTEVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPC_ARGS_FILEDOWNLOADEVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPC_ARGS_FILEDOWNLOADEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_FILEDOWNLOADEVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPC_ARGS_GAMESTARTEVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPC_ARGS_GAMESTARTEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_GAMESTARTEVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPC_ARGS_IMCONTACTEVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPC_ARGS_IMCONTACTEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_IMCONTACTEVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPC_ARGS_IMFEATUREEVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPC_ARGS_IMFEATUREEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_IMFEATUREEVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPC_ARGS_MEDIADOWNLOADEVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPC_ARGS_MEDIADOWNLOADEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_MEDIADOWNLOADEVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPC_ARGS_MEDIAPLAYBACKEVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPC_ARGS_MEDIAPLAYBACKEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_MEDIAPLAYBACKEVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPC_ARGS_SAFERAPPBLOCKED {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPC_ARGS_SAFERAPPBLOCKED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_SAFERAPPBLOCKED").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPC_ARGS_SETTINGSCHANGEEVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPC_ARGS_SETTINGSCHANGEEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_SETTINGSCHANGEEVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPC_ARGS_URLVISITEVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPC_ARGS_URLVISITEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_URLVISITEVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPC_ARGS_WEBOVERRIDEEVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPC_ARGS_WEBOVERRIDEEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_WEBOVERRIDEEVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPC_ARGS_WEBSITEVISITEVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPC_ARGS_WEBSITEVISITEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_ARGS_WEBSITEVISITEVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPC_MEDIA_EXPLICIT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPC_MEDIA_EXPLICIT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_MEDIA_EXPLICIT").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPC_MEDIA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPC_MEDIA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_MEDIA_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPC_SETTINGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPC_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPC_SETTINGS").field(&self.0).finish()
    }
}
