#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDeviceID(pbwindowsaik: ::core::option::Option<&mut [u8]>, pcbresult: *mut u32, pfprotectedbytpm: ::core::option::Option<*mut super::super::Foundation::BOOL>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "tbs.dll""system" fn GetDeviceID ( pbwindowsaik : *mut u8 , cbwindowsaik : u32 , pcbresult : *mut u32 , pfprotectedbytpm : *mut super::super::Foundation:: BOOL ) -> ::windows::core::HRESULT );
    GetDeviceID(::core::mem::transmute(pbwindowsaik.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pbwindowsaik.as_deref().map_or(0, |slice| slice.len() as _), pcbresult, ::core::mem::transmute(pfprotectedbytpm.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDeviceIDString(pszwindowsaik: ::core::option::Option<&mut [u16]>, pcchresult: *mut u32, pfprotectedbytpm: ::core::option::Option<*mut super::super::Foundation::BOOL>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "tbs.dll""system" fn GetDeviceIDString ( pszwindowsaik : ::windows::core::PWSTR , cchwindowsaik : u32 , pcchresult : *mut u32 , pfprotectedbytpm : *mut super::super::Foundation:: BOOL ) -> ::windows::core::HRESULT );
    GetDeviceIDString(::core::mem::transmute(pszwindowsaik.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pszwindowsaik.as_deref().map_or(0, |slice| slice.len() as _), pcchresult, ::core::mem::transmute(pfprotectedbytpm.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
#[inline]
pub unsafe fn Tbsi_Context_Create(pcontextparams: *const TBS_CONTEXT_PARAMS, phcontext: *mut *mut ::core::ffi::c_void) -> u32 {
    ::windows_targets::link ! ( "tbs.dll""system" fn Tbsi_Context_Create ( pcontextparams : *const TBS_CONTEXT_PARAMS , phcontext : *mut *mut ::core::ffi::c_void ) -> u32 );
    Tbsi_Context_Create(pcontextparams, phcontext)
}
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
#[inline]
pub unsafe fn Tbsi_Create_Windows_Key(keyhandle: u32) -> u32 {
    ::windows_targets::link ! ( "tbs.dll""system" fn Tbsi_Create_Windows_Key ( keyhandle : u32 ) -> u32 );
    Tbsi_Create_Windows_Key(keyhandle)
}
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
#[inline]
pub unsafe fn Tbsi_GetDeviceInfo(size: u32, info: *mut ::core::ffi::c_void) -> u32 {
    ::windows_targets::link ! ( "tbs.dll""system" fn Tbsi_GetDeviceInfo ( size : u32 , info : *mut ::core::ffi::c_void ) -> u32 );
    Tbsi_GetDeviceInfo(size, info)
}
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
#[inline]
pub unsafe fn Tbsi_Get_OwnerAuth(hcontext: *const ::core::ffi::c_void, ownerauthtype: u32, poutputbuf: ::core::option::Option<*mut u8>, poutputbuflen: *mut u32) -> u32 {
    ::windows_targets::link ! ( "tbs.dll""system" fn Tbsi_Get_OwnerAuth ( hcontext : *const ::core::ffi::c_void , ownerauthtype : u32 , poutputbuf : *mut u8 , poutputbuflen : *mut u32 ) -> u32 );
    Tbsi_Get_OwnerAuth(hcontext, ownerauthtype, ::core::mem::transmute(poutputbuf.unwrap_or(::std::ptr::null_mut())), poutputbuflen)
}
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
#[inline]
pub unsafe fn Tbsi_Get_TCG_Log(hcontext: *const ::core::ffi::c_void, poutputbuf: ::core::option::Option<*mut u8>, poutputbuflen: *mut u32) -> u32 {
    ::windows_targets::link ! ( "tbs.dll""system" fn Tbsi_Get_TCG_Log ( hcontext : *const ::core::ffi::c_void , poutputbuf : *mut u8 , poutputbuflen : *mut u32 ) -> u32 );
    Tbsi_Get_TCG_Log(hcontext, ::core::mem::transmute(poutputbuf.unwrap_or(::std::ptr::null_mut())), poutputbuflen)
}
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
#[inline]
pub unsafe fn Tbsi_Get_TCG_Log_Ex(logtype: u32, pboutput: ::core::option::Option<*mut u8>, pcboutput: *mut u32) -> u32 {
    ::windows_targets::link ! ( "tbs.dll""system" fn Tbsi_Get_TCG_Log_Ex ( logtype : u32 , pboutput : *mut u8 , pcboutput : *mut u32 ) -> u32 );
    Tbsi_Get_TCG_Log_Ex(logtype, ::core::mem::transmute(pboutput.unwrap_or(::std::ptr::null_mut())), pcboutput)
}
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
#[inline]
pub unsafe fn Tbsi_Physical_Presence_Command(hcontext: *const ::core::ffi::c_void, pabinput: &[u8], paboutput: *mut u8, pcboutput: *mut u32) -> u32 {
    ::windows_targets::link ! ( "tbs.dll""system" fn Tbsi_Physical_Presence_Command ( hcontext : *const ::core::ffi::c_void , pabinput : *const u8 , cbinput : u32 , paboutput : *mut u8 , pcboutput : *mut u32 ) -> u32 );
    Tbsi_Physical_Presence_Command(hcontext, ::core::mem::transmute(pabinput.as_ptr()), pabinput.len() as _, paboutput, pcboutput)
}
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
#[inline]
pub unsafe fn Tbsi_Revoke_Attestation() -> u32 {
    ::windows_targets::link ! ( "tbs.dll""system" fn Tbsi_Revoke_Attestation ( ) -> u32 );
    Tbsi_Revoke_Attestation()
}
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
#[inline]
pub unsafe fn Tbsip_Cancel_Commands(hcontext: *const ::core::ffi::c_void) -> u32 {
    ::windows_targets::link ! ( "tbs.dll""system" fn Tbsip_Cancel_Commands ( hcontext : *const ::core::ffi::c_void ) -> u32 );
    Tbsip_Cancel_Commands(hcontext)
}
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
#[inline]
pub unsafe fn Tbsip_Context_Close(hcontext: *const ::core::ffi::c_void) -> u32 {
    ::windows_targets::link ! ( "tbs.dll""system" fn Tbsip_Context_Close ( hcontext : *const ::core::ffi::c_void ) -> u32 );
    Tbsip_Context_Close(hcontext)
}
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
#[inline]
pub unsafe fn Tbsip_Submit_Command(hcontext: *const ::core::ffi::c_void, locality: TBS_COMMAND_LOCALITY, priority: TBS_COMMAND_PRIORITY, pabcommand: &[u8], pabresult: *mut u8, pcbresult: *mut u32) -> u32 {
    ::windows_targets::link ! ( "tbs.dll""system" fn Tbsip_Submit_Command ( hcontext : *const ::core::ffi::c_void , locality : TBS_COMMAND_LOCALITY , priority : TBS_COMMAND_PRIORITY , pabcommand : *const u8 , cbcommand : u32 , pabresult : *mut u8 , pcbresult : *mut u32 ) -> u32 );
    Tbsip_Submit_Command(hcontext, locality, priority, ::core::mem::transmute(pabcommand.as_ptr()), pabcommand.len() as _, pabresult, pcbresult)
}
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub const TBS_CONTEXT_VERSION_ONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub const TBS_CONTEXT_VERSION_TWO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub const TBS_OWNERAUTH_TYPE_ADMIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub const TBS_OWNERAUTH_TYPE_ENDORSEMENT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub const TBS_OWNERAUTH_TYPE_ENDORSEMENT_20: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub const TBS_OWNERAUTH_TYPE_FULL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub const TBS_OWNERAUTH_TYPE_STORAGE_20: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub const TBS_OWNERAUTH_TYPE_USER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub const TBS_SUCCESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub const TBS_TCGLOG_DRTM_BOOT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub const TBS_TCGLOG_DRTM_CURRENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub const TBS_TCGLOG_DRTM_RESUME: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub const TBS_TCGLOG_SRTM_BOOT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub const TBS_TCGLOG_SRTM_CURRENT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub const TBS_TCGLOG_SRTM_RESUME: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub const TPM_IFTYPE_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub const TPM_IFTYPE_EMULATOR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub const TPM_IFTYPE_HW: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub const TPM_IFTYPE_SPB: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub const TPM_IFTYPE_TRUSTZONE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub const TPM_IFTYPE_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub const TPM_VERSION_12: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub const TPM_VERSION_20: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub const TPM_VERSION_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub const TPM_WNF_INFO_CLEAR_SUCCESSFUL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub const TPM_WNF_INFO_NO_REBOOT_REQUIRED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub const TPM_WNF_INFO_OWNERSHIP_SUCCESSFUL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TBS_COMMAND_LOCALITY(pub u32);
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub const TBS_COMMAND_LOCALITY_ZERO: TBS_COMMAND_LOCALITY = TBS_COMMAND_LOCALITY(0u32);
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub const TBS_COMMAND_LOCALITY_ONE: TBS_COMMAND_LOCALITY = TBS_COMMAND_LOCALITY(1u32);
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub const TBS_COMMAND_LOCALITY_TWO: TBS_COMMAND_LOCALITY = TBS_COMMAND_LOCALITY(2u32);
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub const TBS_COMMAND_LOCALITY_THREE: TBS_COMMAND_LOCALITY = TBS_COMMAND_LOCALITY(3u32);
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub const TBS_COMMAND_LOCALITY_FOUR: TBS_COMMAND_LOCALITY = TBS_COMMAND_LOCALITY(4u32);
impl ::core::marker::Copy for TBS_COMMAND_LOCALITY {}
impl ::core::clone::Clone for TBS_COMMAND_LOCALITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TBS_COMMAND_LOCALITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TBS_COMMAND_LOCALITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TBS_COMMAND_LOCALITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TBS_COMMAND_LOCALITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TBS_COMMAND_PRIORITY(pub u32);
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub const TBS_COMMAND_PRIORITY_LOW: TBS_COMMAND_PRIORITY = TBS_COMMAND_PRIORITY(100u32);
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub const TBS_COMMAND_PRIORITY_NORMAL: TBS_COMMAND_PRIORITY = TBS_COMMAND_PRIORITY(200u32);
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub const TBS_COMMAND_PRIORITY_SYSTEM: TBS_COMMAND_PRIORITY = TBS_COMMAND_PRIORITY(400u32);
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub const TBS_COMMAND_PRIORITY_HIGH: TBS_COMMAND_PRIORITY = TBS_COMMAND_PRIORITY(300u32);
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub const TBS_COMMAND_PRIORITY_MAX: TBS_COMMAND_PRIORITY = TBS_COMMAND_PRIORITY(2147483648u32);
impl ::core::marker::Copy for TBS_COMMAND_PRIORITY {}
impl ::core::clone::Clone for TBS_COMMAND_PRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TBS_COMMAND_PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TBS_COMMAND_PRIORITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TBS_COMMAND_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TBS_COMMAND_PRIORITY").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub struct TBS_CONTEXT_PARAMS {
    pub version: u32,
}
impl ::core::marker::Copy for TBS_CONTEXT_PARAMS {}
impl ::core::clone::Clone for TBS_CONTEXT_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TBS_CONTEXT_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TBS_CONTEXT_PARAMS").field("version", &self.version).finish()
    }
}
impl ::windows::core::TypeKind for TBS_CONTEXT_PARAMS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TBS_CONTEXT_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
    }
}
impl ::core::cmp::Eq for TBS_CONTEXT_PARAMS {}
impl ::core::default::Default for TBS_CONTEXT_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub struct TBS_CONTEXT_PARAMS2 {
    pub version: u32,
    pub Anonymous: TBS_CONTEXT_PARAMS2_0,
}
impl ::core::marker::Copy for TBS_CONTEXT_PARAMS2 {}
impl ::core::clone::Clone for TBS_CONTEXT_PARAMS2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for TBS_CONTEXT_PARAMS2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for TBS_CONTEXT_PARAMS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub union TBS_CONTEXT_PARAMS2_0 {
    pub Anonymous: TBS_CONTEXT_PARAMS2_0_0,
    pub asUINT32: u32,
}
impl ::core::marker::Copy for TBS_CONTEXT_PARAMS2_0 {}
impl ::core::clone::Clone for TBS_CONTEXT_PARAMS2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for TBS_CONTEXT_PARAMS2_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for TBS_CONTEXT_PARAMS2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub struct TBS_CONTEXT_PARAMS2_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for TBS_CONTEXT_PARAMS2_0_0 {}
impl ::core::clone::Clone for TBS_CONTEXT_PARAMS2_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TBS_CONTEXT_PARAMS2_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TBS_CONTEXT_PARAMS2_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows::core::TypeKind for TBS_CONTEXT_PARAMS2_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TBS_CONTEXT_PARAMS2_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for TBS_CONTEXT_PARAMS2_0_0 {}
impl ::core::default::Default for TBS_CONTEXT_PARAMS2_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub struct TPM_DEVICE_INFO {
    pub structVersion: u32,
    pub tpmVersion: u32,
    pub tpmInterfaceType: u32,
    pub tpmImpRevision: u32,
}
impl ::core::marker::Copy for TPM_DEVICE_INFO {}
impl ::core::clone::Clone for TPM_DEVICE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TPM_DEVICE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TPM_DEVICE_INFO").field("structVersion", &self.structVersion).field("tpmVersion", &self.tpmVersion).field("tpmInterfaceType", &self.tpmInterfaceType).field("tpmImpRevision", &self.tpmImpRevision).finish()
    }
}
impl ::windows::core::TypeKind for TPM_DEVICE_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TPM_DEVICE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.structVersion == other.structVersion && self.tpmVersion == other.tpmVersion && self.tpmInterfaceType == other.tpmInterfaceType && self.tpmImpRevision == other.tpmImpRevision
    }
}
impl ::core::cmp::Eq for TPM_DEVICE_INFO {}
impl ::core::default::Default for TPM_DEVICE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub struct TPM_WNF_PROVISIONING {
    pub status: u32,
    pub message: [u8; 28],
}
impl ::core::marker::Copy for TPM_WNF_PROVISIONING {}
impl ::core::clone::Clone for TPM_WNF_PROVISIONING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TPM_WNF_PROVISIONING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TPM_WNF_PROVISIONING").field("status", &self.status).field("message", &self.message).finish()
    }
}
impl ::windows::core::TypeKind for TPM_WNF_PROVISIONING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TPM_WNF_PROVISIONING {
    fn eq(&self, other: &Self) -> bool {
        self.status == other.status && self.message == other.message
    }
}
impl ::core::cmp::Eq for TPM_WNF_PROVISIONING {}
impl ::core::default::Default for TPM_WNF_PROVISIONING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
