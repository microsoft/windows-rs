#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLX_CLIENT_CREDENTIALS_INFO_V1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLX_CLIENT_CREDENTIALS_INFO_V1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType && self.pszUserName == other.pszUserName && self.pszDomain == other.pszDomain && self.pszPassword == other.pszPassword && self.fPromptForPassword == other.fPromptForPassword
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLX_CLIENT_CREDENTIALS_INFO_V1_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WLX_CLIENT_CREDENTIALS_INFO_V1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLX_CLIENT_CREDENTIALS_INFO_V1_0").field("dwType", &self.dwType).field("pszUserName", &self.pszUserName).field("pszDomain", &self.pszDomain).field("pszPassword", &self.pszPassword).field("fPromptForPassword", &self.fPromptForPassword).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLX_CLIENT_CREDENTIALS_INFO_V2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLX_CLIENT_CREDENTIALS_INFO_V2_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType && self.pszUserName == other.pszUserName && self.pszDomain == other.pszDomain && self.pszPassword == other.pszPassword && self.fPromptForPassword == other.fPromptForPassword && self.fDisconnectOnLogonFailure == other.fDisconnectOnLogonFailure
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLX_CLIENT_CREDENTIALS_INFO_V2_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WLX_CLIENT_CREDENTIALS_INFO_V2_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLX_CLIENT_CREDENTIALS_INFO_V2_0").field("dwType", &self.dwType).field("pszUserName", &self.pszUserName).field("pszDomain", &self.pszDomain).field("pszPassword", &self.pszPassword).field("fPromptForPassword", &self.fPromptForPassword).field("fDisconnectOnLogonFailure", &self.fDisconnectOnLogonFailure).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLX_CONSOLESWITCH_CREDENTIALS_INFO_V1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLX_CONSOLESWITCH_CREDENTIALS_INFO_V1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType
            && self.UserToken == other.UserToken
            && self.LogonId == other.LogonId
            && self.Quotas == other.Quotas
            && self.UserName == other.UserName
            && self.Domain == other.Domain
            && self.LogonTime == other.LogonTime
            && self.SmartCardLogon == other.SmartCardLogon
            && self.ProfileLength == other.ProfileLength
            && self.MessageType == other.MessageType
            && self.LogonCount == other.LogonCount
            && self.BadPasswordCount == other.BadPasswordCount
            && self.ProfileLogonTime == other.ProfileLogonTime
            && self.LogoffTime == other.LogoffTime
            && self.KickOffTime == other.KickOffTime
            && self.PasswordLastSet == other.PasswordLastSet
            && self.PasswordCanChange == other.PasswordCanChange
            && self.PasswordMustChange == other.PasswordMustChange
            && self.LogonScript == other.LogonScript
            && self.HomeDirectory == other.HomeDirectory
            && self.FullName == other.FullName
            && self.ProfilePath == other.ProfilePath
            && self.HomeDirectoryDrive == other.HomeDirectoryDrive
            && self.LogonServer == other.LogonServer
            && self.UserFlags == other.UserFlags
            && self.PrivateDataLen == other.PrivateDataLen
            && self.PrivateData == other.PrivateData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLX_CONSOLESWITCH_CREDENTIALS_INFO_V1_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WLX_CONSOLESWITCH_CREDENTIALS_INFO_V1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLX_CONSOLESWITCH_CREDENTIALS_INFO_V1_0")
            .field("dwType", &self.dwType)
            .field("UserToken", &self.UserToken)
            .field("LogonId", &self.LogonId)
            .field("Quotas", &self.Quotas)
            .field("UserName", &self.UserName)
            .field("Domain", &self.Domain)
            .field("LogonTime", &self.LogonTime)
            .field("SmartCardLogon", &self.SmartCardLogon)
            .field("ProfileLength", &self.ProfileLength)
            .field("MessageType", &self.MessageType)
            .field("LogonCount", &self.LogonCount)
            .field("BadPasswordCount", &self.BadPasswordCount)
            .field("ProfileLogonTime", &self.ProfileLogonTime)
            .field("LogoffTime", &self.LogoffTime)
            .field("KickOffTime", &self.KickOffTime)
            .field("PasswordLastSet", &self.PasswordLastSet)
            .field("PasswordCanChange", &self.PasswordCanChange)
            .field("PasswordMustChange", &self.PasswordMustChange)
            .field("LogonScript", &self.LogonScript)
            .field("HomeDirectory", &self.HomeDirectory)
            .field("FullName", &self.FullName)
            .field("ProfilePath", &self.ProfilePath)
            .field("HomeDirectoryDrive", &self.HomeDirectoryDrive)
            .field("LogonServer", &self.LogonServer)
            .field("UserFlags", &self.UserFlags)
            .field("PrivateDataLen", &self.PrivateDataLen)
            .field("PrivateData", &self.PrivateData)
            .finish()
    }
}
#[cfg(feature = "Win32_System_StationsAndDesktops")]
impl ::core::default::Default for WLX_DESKTOP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_StationsAndDesktops")]
impl ::core::cmp::PartialEq for WLX_DESKTOP {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Flags == other.Flags && self.hDesktop == other.hDesktop && self.pszDesktopName == other.pszDesktopName
    }
}
#[cfg(feature = "Win32_System_StationsAndDesktops")]
impl ::core::cmp::Eq for WLX_DESKTOP {}
#[cfg(feature = "Win32_System_StationsAndDesktops")]
impl ::core::fmt::Debug for WLX_DESKTOP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLX_DESKTOP").field("Size", &self.Size).field("Flags", &self.Flags).field("hDesktop", &self.hDesktop).field("pszDesktopName", &self.pszDesktopName).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for WLX_DISPATCH_VERSION_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for WLX_DISPATCH_VERSION_1_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for WLX_DISPATCH_VERSION_1_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for WLX_DISPATCH_VERSION_1_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for WLX_DISPATCH_VERSION_1_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WLX_MPR_NOTIFY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WLX_MPR_NOTIFY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszUserName == other.pszUserName && self.pszDomain == other.pszDomain && self.pszPassword == other.pszPassword && self.pszOldPassword == other.pszOldPassword
    }
}
impl ::core::cmp::Eq for WLX_MPR_NOTIFY_INFO {}
impl ::core::fmt::Debug for WLX_MPR_NOTIFY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLX_MPR_NOTIFY_INFO").field("pszUserName", &self.pszUserName).field("pszDomain", &self.pszDomain).field("pszPassword", &self.pszPassword).field("pszOldPassword", &self.pszOldPassword).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_StationsAndDesktops"))]
impl ::core::default::Default for WLX_NOTIFICATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WLX_PROFILE_V1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WLX_PROFILE_V1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType && self.pszProfile == other.pszProfile
    }
}
impl ::core::cmp::Eq for WLX_PROFILE_V1_0 {}
impl ::core::fmt::Debug for WLX_PROFILE_V1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLX_PROFILE_V1_0").field("dwType", &self.dwType).field("pszProfile", &self.pszProfile).finish()
    }
}
impl ::core::default::Default for WLX_PROFILE_V2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WLX_PROFILE_V2_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType && self.pszProfile == other.pszProfile && self.pszPolicy == other.pszPolicy && self.pszNetworkDefaultUserProfile == other.pszNetworkDefaultUserProfile && self.pszServerName == other.pszServerName && self.pszEnvironment == other.pszEnvironment
    }
}
impl ::core::cmp::Eq for WLX_PROFILE_V2_0 {}
impl ::core::fmt::Debug for WLX_PROFILE_V2_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLX_PROFILE_V2_0").field("dwType", &self.dwType).field("pszProfile", &self.pszProfile).field("pszPolicy", &self.pszPolicy).field("pszNetworkDefaultUserProfile", &self.pszNetworkDefaultUserProfile).field("pszServerName", &self.pszServerName).field("pszEnvironment", &self.pszEnvironment).finish()
    }
}
impl ::core::default::Default for WLX_SC_NOTIFICATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WLX_SC_NOTIFICATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszCard == other.pszCard && self.pszReader == other.pszReader && self.pszContainer == other.pszContainer && self.pszCryptoProvider == other.pszCryptoProvider
    }
}
impl ::core::cmp::Eq for WLX_SC_NOTIFICATION_INFO {}
impl ::core::fmt::Debug for WLX_SC_NOTIFICATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLX_SC_NOTIFICATION_INFO").field("pszCard", &self.pszCard).field("pszReader", &self.pszReader).field("pszContainer", &self.pszContainer).field("pszCryptoProvider", &self.pszCryptoProvider).finish()
    }
}
impl ::core::default::Default for WLX_SHUTDOWN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WLX_SHUTDOWN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLX_SHUTDOWN_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WLX_TERMINAL_SERVICES_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WLX_TERMINAL_SERVICES_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.ProfilePath == other.ProfilePath && self.HomeDir == other.HomeDir && self.HomeDirDrive == other.HomeDirDrive
    }
}
impl ::core::cmp::Eq for WLX_TERMINAL_SERVICES_DATA {}
impl ::core::fmt::Debug for WLX_TERMINAL_SERVICES_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLX_TERMINAL_SERVICES_DATA").field("ProfilePath", &self.ProfilePath).field("HomeDir", &self.HomeDir).field("HomeDirDrive", &self.HomeDirDrive).finish()
    }
}
