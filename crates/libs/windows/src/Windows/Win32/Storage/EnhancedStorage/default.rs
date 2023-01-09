impl ::core::default::Default for ACT_AUTHORIZATION_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ACT_AUTHORIZATION_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.ulState == other.ulState
    }
}
impl ::core::cmp::Eq for ACT_AUTHORIZATION_STATE {}
impl ::core::fmt::Debug for ACT_AUTHORIZATION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACT_AUTHORIZATION_STATE").field("ulState", &self.ulState).finish()
    }
}
impl ::core::default::Default for ACT_AUTHORIZATION_STATE_VALUE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ACT_AUTHORIZATION_STATE_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACT_AUTHORIZATION_STATE_VALUE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENHANCED_STORAGE_PASSWORD_SILO_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ENHANCED_STORAGE_PASSWORD_SILO_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.CurrentAdminFailures == other.CurrentAdminFailures
            && self.CurrentUserFailures == other.CurrentUserFailures
            && self.TotalUserAuthenticationCount == other.TotalUserAuthenticationCount
            && self.TotalAdminAuthenticationCount == other.TotalAdminAuthenticationCount
            && self.FipsCompliant == other.FipsCompliant
            && self.SecurityIDAvailable == other.SecurityIDAvailable
            && self.InitializeInProgress == other.InitializeInProgress
            && self.ITMSArmed == other.ITMSArmed
            && self.ITMSArmable == other.ITMSArmable
            && self.UserCreated == other.UserCreated
            && self.ResetOnPORDefault == other.ResetOnPORDefault
            && self.ResetOnPORCurrent == other.ResetOnPORCurrent
            && self.MaxAdminFailures == other.MaxAdminFailures
            && self.MaxUserFailures == other.MaxUserFailures
            && self.TimeToCompleteInitialization == other.TimeToCompleteInitialization
            && self.TimeRemainingToCompleteInitialization == other.TimeRemainingToCompleteInitialization
            && self.MinTimeToAuthenticate == other.MinTimeToAuthenticate
            && self.MaxAdminPasswordSize == other.MaxAdminPasswordSize
            && self.MinAdminPasswordSize == other.MinAdminPasswordSize
            && self.MaxAdminHintSize == other.MaxAdminHintSize
            && self.MaxUserPasswordSize == other.MaxUserPasswordSize
            && self.MinUserPasswordSize == other.MinUserPasswordSize
            && self.MaxUserHintSize == other.MaxUserHintSize
            && self.MaxUserNameSize == other.MaxUserNameSize
            && self.MaxSiloNameSize == other.MaxSiloNameSize
            && self.MaxChallengeSize == other.MaxChallengeSize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ENHANCED_STORAGE_PASSWORD_SILO_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ENHANCED_STORAGE_PASSWORD_SILO_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENHANCED_STORAGE_PASSWORD_SILO_INFORMATION")
            .field("CurrentAdminFailures", &self.CurrentAdminFailures)
            .field("CurrentUserFailures", &self.CurrentUserFailures)
            .field("TotalUserAuthenticationCount", &self.TotalUserAuthenticationCount)
            .field("TotalAdminAuthenticationCount", &self.TotalAdminAuthenticationCount)
            .field("FipsCompliant", &self.FipsCompliant)
            .field("SecurityIDAvailable", &self.SecurityIDAvailable)
            .field("InitializeInProgress", &self.InitializeInProgress)
            .field("ITMSArmed", &self.ITMSArmed)
            .field("ITMSArmable", &self.ITMSArmable)
            .field("UserCreated", &self.UserCreated)
            .field("ResetOnPORDefault", &self.ResetOnPORDefault)
            .field("ResetOnPORCurrent", &self.ResetOnPORCurrent)
            .field("MaxAdminFailures", &self.MaxAdminFailures)
            .field("MaxUserFailures", &self.MaxUserFailures)
            .field("TimeToCompleteInitialization", &self.TimeToCompleteInitialization)
            .field("TimeRemainingToCompleteInitialization", &self.TimeRemainingToCompleteInitialization)
            .field("MinTimeToAuthenticate", &self.MinTimeToAuthenticate)
            .field("MaxAdminPasswordSize", &self.MaxAdminPasswordSize)
            .field("MinAdminPasswordSize", &self.MinAdminPasswordSize)
            .field("MaxAdminHintSize", &self.MaxAdminHintSize)
            .field("MaxUserPasswordSize", &self.MaxUserPasswordSize)
            .field("MinUserPasswordSize", &self.MinUserPasswordSize)
            .field("MaxUserHintSize", &self.MaxUserHintSize)
            .field("MaxUserNameSize", &self.MaxUserNameSize)
            .field("MaxSiloNameSize", &self.MaxSiloNameSize)
            .field("MaxChallengeSize", &self.MaxChallengeSize)
            .finish()
    }
}
impl ::core::cmp::PartialEq for IEnhancedStorageACT {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnhancedStorageACT {}
impl ::core::fmt::Debug for IEnhancedStorageACT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnhancedStorageACT").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnhancedStorageACT2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnhancedStorageACT2 {}
impl ::core::fmt::Debug for IEnhancedStorageACT2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnhancedStorageACT2").field(&self.0).finish()
    }
}
impl IEnhancedStorageACT2 {
    pub unsafe fn Authorize(&self, hwndparent: u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Authorize)(::windows::core::Vtable::as_raw(self), hwndparent, dwflags).ok()
    }
    pub unsafe fn Unauthorize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Unauthorize)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetAuthorizationState(&self) -> ::windows::core::Result<ACT_AUTHORIZATION_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetAuthorizationState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMatchingVolume(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMatchingVolume)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUniqueIdentity(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUniqueIdentity)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSilos(&self, pppienhancedstoragesilos: *mut *mut ::core::option::Option<IEnhancedStorageSilo>, pcenhancedstoragesilos: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetSilos)(::windows::core::Vtable::as_raw(self), pppienhancedstoragesilos, pcenhancedstoragesilos).ok()
    }
}
impl ::core::cmp::PartialEq for IEnhancedStorageACT3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnhancedStorageACT3 {}
impl ::core::fmt::Debug for IEnhancedStorageACT3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnhancedStorageACT3").field(&self.0).finish()
    }
}
impl IEnhancedStorageACT3 {
    pub unsafe fn Authorize(&self, hwndparent: u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Authorize)(::windows::core::Vtable::as_raw(self), hwndparent, dwflags).ok()
    }
    pub unsafe fn Unauthorize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Unauthorize)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetAuthorizationState(&self) -> ::windows::core::Result<ACT_AUTHORIZATION_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetAuthorizationState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMatchingVolume(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetMatchingVolume)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUniqueIdentity(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetUniqueIdentity)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSilos(&self, pppienhancedstoragesilos: *mut *mut ::core::option::Option<IEnhancedStorageSilo>, pcenhancedstoragesilos: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetSilos)(::windows::core::Vtable::as_raw(self), pppienhancedstoragesilos, pcenhancedstoragesilos).ok()
    }
    pub unsafe fn GetDeviceName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDeviceName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsDeviceRemovable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsDeviceRemovable)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IEnhancedStorageSilo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnhancedStorageSilo {}
impl ::core::fmt::Debug for IEnhancedStorageSilo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnhancedStorageSilo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnhancedStorageSiloAction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnhancedStorageSiloAction {}
impl ::core::fmt::Debug for IEnhancedStorageSiloAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnhancedStorageSiloAction").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumEnhancedStorageACT {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumEnhancedStorageACT {}
impl ::core::fmt::Debug for IEnumEnhancedStorageACT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumEnhancedStorageACT").field(&self.0).finish()
    }
}
impl ::core::default::Default for SILO_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SILO_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ulSTID == other.ulSTID && self.SpecificationMajor == other.SpecificationMajor && self.SpecificationMinor == other.SpecificationMinor && self.ImplementationMajor == other.ImplementationMajor && self.ImplementationMinor == other.ImplementationMinor && self.r#type == other.r#type && self.capabilities == other.capabilities
    }
}
impl ::core::cmp::Eq for SILO_INFO {}
impl ::core::fmt::Debug for SILO_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SILO_INFO").field("ulSTID", &self.ulSTID).field("SpecificationMajor", &self.SpecificationMajor).field("SpecificationMinor", &self.SpecificationMinor).field("ImplementationMajor", &self.ImplementationMajor).field("ImplementationMinor", &self.ImplementationMinor).field("type", &self.r#type).field("capabilities", &self.capabilities).finish()
    }
}
