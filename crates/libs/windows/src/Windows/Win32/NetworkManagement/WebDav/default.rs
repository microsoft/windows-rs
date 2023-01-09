impl ::core::default::Default for AUTHNEXTSTEP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AUTHNEXTSTEP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHNEXTSTEP").field(&self.0).finish()
    }
}
impl ::core::default::Default for DAV_CALLBACK_AUTH_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DAV_CALLBACK_AUTH_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.pBuffer == other.pBuffer && self.ulSize == other.ulSize && self.ulType == other.ulType
    }
}
impl ::core::cmp::Eq for DAV_CALLBACK_AUTH_BLOB {}
impl ::core::fmt::Debug for DAV_CALLBACK_AUTH_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DAV_CALLBACK_AUTH_BLOB").field("pBuffer", &self.pBuffer).field("ulSize", &self.ulSize).field("ulType", &self.ulType).finish()
    }
}
impl ::core::default::Default for DAV_CALLBACK_AUTH_UNP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DAV_CALLBACK_AUTH_UNP {
    fn eq(&self, other: &Self) -> bool {
        self.pszUserName == other.pszUserName && self.ulUserNameLength == other.ulUserNameLength && self.pszPassword == other.pszPassword && self.ulPasswordLength == other.ulPasswordLength
    }
}
impl ::core::cmp::Eq for DAV_CALLBACK_AUTH_UNP {}
impl ::core::fmt::Debug for DAV_CALLBACK_AUTH_UNP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DAV_CALLBACK_AUTH_UNP").field("pszUserName", &self.pszUserName).field("ulUserNameLength", &self.ulUserNameLength).field("pszPassword", &self.pszPassword).field("ulPasswordLength", &self.ulPasswordLength).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DAV_CALLBACK_CRED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DAV_CALLBACK_CRED {
    fn eq(&self, other: &Self) -> bool {
        self.AuthBlob == other.AuthBlob && self.UNPBlob == other.UNPBlob && self.bAuthBlobValid == other.bAuthBlobValid && self.bSave == other.bSave
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DAV_CALLBACK_CRED {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DAV_CALLBACK_CRED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DAV_CALLBACK_CRED").field("AuthBlob", &self.AuthBlob).field("UNPBlob", &self.UNPBlob).field("bAuthBlobValid", &self.bAuthBlobValid).field("bSave", &self.bSave).finish()
    }
}
