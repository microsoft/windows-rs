#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDeviceID(pbwindowsaik: *mut u8, cbwindowsaik: u32, pcbresult: *mut u32, pfprotectedbytpm: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDeviceID(pbwindowsaik: *mut u8, cbwindowsaik: u32, pcbresult: *mut u32, pfprotectedbytpm: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        GetDeviceID(::core::mem::transmute(pbwindowsaik), ::core::mem::transmute(cbwindowsaik), ::core::mem::transmute(pcbresult), ::core::mem::transmute(pfprotectedbytpm)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDeviceIDString(pszwindowsaik: &mut [u16], pcchresult: *mut u32, pfprotectedbytpm: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDeviceIDString(pszwindowsaik: ::windows::core::PWSTR, cchwindowsaik: u32, pcchresult: *mut u32, pfprotectedbytpm: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        GetDeviceIDString(::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszwindowsaik)), pszwindowsaik.len() as _, ::core::mem::transmute(pcchresult), ::core::mem::transmute(pfprotectedbytpm)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for TBS_COMMAND_LOCALITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for TBS_COMMAND_LOCALITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TBS_COMMAND_LOCALITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for TBS_COMMAND_PRIORITY {
    type Abi = Self;
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
unsafe impl ::windows::core::Abi for TBS_CONTEXT_PARAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TBS_CONTEXT_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TBS_CONTEXT_PARAMS>()) == 0 }
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
unsafe impl ::windows::core::Abi for TBS_CONTEXT_PARAMS2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TBS_CONTEXT_PARAMS2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TBS_CONTEXT_PARAMS2>()) == 0 }
    }
}
impl ::core::cmp::Eq for TBS_CONTEXT_PARAMS2 {}
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
unsafe impl ::windows::core::Abi for TBS_CONTEXT_PARAMS2_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TBS_CONTEXT_PARAMS2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TBS_CONTEXT_PARAMS2_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for TBS_CONTEXT_PARAMS2_0 {}
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
unsafe impl ::windows::core::Abi for TBS_CONTEXT_PARAMS2_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TBS_CONTEXT_PARAMS2_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TBS_CONTEXT_PARAMS2_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for TBS_CONTEXT_PARAMS2_0_0 {}
impl ::core::default::Default for TBS_CONTEXT_PARAMS2_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
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
unsafe impl ::windows::core::Abi for TPM_DEVICE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TPM_DEVICE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TPM_DEVICE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for TPM_DEVICE_INFO {}
impl ::core::default::Default for TPM_DEVICE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
#[inline]
pub unsafe fn Tbsi_Context_Create(pcontextparams: *const TBS_CONTEXT_PARAMS, phcontext: *mut *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Tbsi_Context_Create(pcontextparams: *const TBS_CONTEXT_PARAMS, phcontext: *mut *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(Tbsi_Context_Create(::core::mem::transmute(pcontextparams), ::core::mem::transmute(phcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
#[inline]
pub unsafe fn Tbsi_Create_Windows_Key(keyhandle: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Tbsi_Create_Windows_Key(keyhandle: u32) -> u32;
        }
        ::core::mem::transmute(Tbsi_Create_Windows_Key(::core::mem::transmute(keyhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
#[inline]
pub unsafe fn Tbsi_GetDeviceInfo(size: u32, info: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Tbsi_GetDeviceInfo(size: u32, info: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(Tbsi_GetDeviceInfo(::core::mem::transmute(size), ::core::mem::transmute(info)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
#[inline]
pub unsafe fn Tbsi_Get_OwnerAuth(hcontext: *const ::core::ffi::c_void, ownerauthtype: u32, poutputbuf: *mut u8, poutputbuflen: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Tbsi_Get_OwnerAuth(hcontext: *const ::core::ffi::c_void, ownerauthtype: u32, poutputbuf: *mut u8, poutputbuflen: *mut u32) -> u32;
        }
        ::core::mem::transmute(Tbsi_Get_OwnerAuth(::core::mem::transmute(hcontext), ::core::mem::transmute(ownerauthtype), ::core::mem::transmute(poutputbuf), ::core::mem::transmute(poutputbuflen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
#[inline]
pub unsafe fn Tbsi_Get_TCG_Log(hcontext: *const ::core::ffi::c_void, poutputbuf: *mut u8, poutputbuflen: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Tbsi_Get_TCG_Log(hcontext: *const ::core::ffi::c_void, poutputbuf: *mut u8, poutputbuflen: *mut u32) -> u32;
        }
        ::core::mem::transmute(Tbsi_Get_TCG_Log(::core::mem::transmute(hcontext), ::core::mem::transmute(poutputbuf), ::core::mem::transmute(poutputbuflen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
#[inline]
pub unsafe fn Tbsi_Get_TCG_Log_Ex(logtype: u32, pboutput: *mut u8, pcboutput: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Tbsi_Get_TCG_Log_Ex(logtype: u32, pboutput: *mut u8, pcboutput: *mut u32) -> u32;
        }
        ::core::mem::transmute(Tbsi_Get_TCG_Log_Ex(::core::mem::transmute(logtype), ::core::mem::transmute(pboutput), ::core::mem::transmute(pcboutput)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
#[inline]
pub unsafe fn Tbsi_Physical_Presence_Command(hcontext: *const ::core::ffi::c_void, pabinput: *const u8, cbinput: u32, paboutput: *mut u8, pcboutput: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Tbsi_Physical_Presence_Command(hcontext: *const ::core::ffi::c_void, pabinput: *const u8, cbinput: u32, paboutput: *mut u8, pcboutput: *mut u32) -> u32;
        }
        ::core::mem::transmute(Tbsi_Physical_Presence_Command(::core::mem::transmute(hcontext), ::core::mem::transmute(pabinput), ::core::mem::transmute(cbinput), ::core::mem::transmute(paboutput), ::core::mem::transmute(pcboutput)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
#[inline]
pub unsafe fn Tbsi_Revoke_Attestation() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Tbsi_Revoke_Attestation() -> u32;
        }
        ::core::mem::transmute(Tbsi_Revoke_Attestation())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
#[inline]
pub unsafe fn Tbsip_Cancel_Commands(hcontext: *const ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Tbsip_Cancel_Commands(hcontext: *const ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(Tbsip_Cancel_Commands(::core::mem::transmute(hcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
#[inline]
pub unsafe fn Tbsip_Context_Close(hcontext: *const ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Tbsip_Context_Close(hcontext: *const ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(Tbsip_Context_Close(::core::mem::transmute(hcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
#[inline]
pub unsafe fn Tbsip_Submit_Command(hcontext: *const ::core::ffi::c_void, locality: TBS_COMMAND_LOCALITY, priority: TBS_COMMAND_PRIORITY, pabcommand: *const u8, cbcommand: u32, pabresult: *mut u8, pcbresult: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Tbsip_Submit_Command(hcontext: *const ::core::ffi::c_void, locality: TBS_COMMAND_LOCALITY, priority: TBS_COMMAND_PRIORITY, pabcommand: *const u8, cbcommand: u32, pabresult: *mut u8, pcbresult: *mut u32) -> u32;
        }
        ::core::mem::transmute(Tbsip_Submit_Command(::core::mem::transmute(hcontext), ::core::mem::transmute(locality), ::core::mem::transmute(priority), ::core::mem::transmute(pabcommand), ::core::mem::transmute(cbcommand), ::core::mem::transmute(pabresult), ::core::mem::transmute(pcbresult)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_TpmBaseServices\"`*"]
pub struct tdTPM_WNF_PROVISIONING {
    pub status: u32,
    pub message: [u8; 28],
}
impl ::core::marker::Copy for tdTPM_WNF_PROVISIONING {}
impl ::core::clone::Clone for tdTPM_WNF_PROVISIONING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for tdTPM_WNF_PROVISIONING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("tdTPM_WNF_PROVISIONING").field("status", &self.status).field("message", &self.message).finish()
    }
}
unsafe impl ::windows::core::Abi for tdTPM_WNF_PROVISIONING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for tdTPM_WNF_PROVISIONING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<tdTPM_WNF_PROVISIONING>()) == 0 }
    }
}
impl ::core::cmp::Eq for tdTPM_WNF_PROVISIONING {}
impl ::core::default::Default for tdTPM_WNF_PROVISIONING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
