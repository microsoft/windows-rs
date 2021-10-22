#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub const ATA_FLAGS_48BIT_COMMAND: u32 = 8u32;
pub const ATA_FLAGS_DATA_IN: u32 = 2u32;
pub const ATA_FLAGS_DATA_OUT: u32 = 4u32;
pub const ATA_FLAGS_DRDY_REQUIRED: u32 = 1u32;
pub const ATA_FLAGS_NO_MULTIPLE: u32 = 32u32;
pub const ATA_FLAGS_USE_DMA: u32 = 16u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ATA_PASS_THROUGH_DIRECT {
    pub Length: u16,
    pub AtaFlags: u16,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub ReservedAsUchar: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub ReservedAsUlong: u32,
    pub DataBuffer: *mut ::std::ffi::c_void,
    pub PreviousTaskFile: [u8; 8],
    pub CurrentTaskFile: [u8; 8],
}
impl ATA_PASS_THROUGH_DIRECT {}
impl ::std::default::Default for ATA_PASS_THROUGH_DIRECT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ATA_PASS_THROUGH_DIRECT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ATA_PASS_THROUGH_DIRECT")
            .field("Length", &self.Length)
            .field("AtaFlags", &self.AtaFlags)
            .field("PathId", &self.PathId)
            .field("TargetId", &self.TargetId)
            .field("Lun", &self.Lun)
            .field("ReservedAsUchar", &self.ReservedAsUchar)
            .field("DataTransferLength", &self.DataTransferLength)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("ReservedAsUlong", &self.ReservedAsUlong)
            .field("DataBuffer", &self.DataBuffer)
            .field("PreviousTaskFile", &self.PreviousTaskFile)
            .field("CurrentTaskFile", &self.CurrentTaskFile)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ATA_PASS_THROUGH_DIRECT {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length
            && self.AtaFlags == other.AtaFlags
            && self.PathId == other.PathId
            && self.TargetId == other.TargetId
            && self.Lun == other.Lun
            && self.ReservedAsUchar == other.ReservedAsUchar
            && self.DataTransferLength == other.DataTransferLength
            && self.TimeOutValue == other.TimeOutValue
            && self.ReservedAsUlong == other.ReservedAsUlong
            && self.DataBuffer == other.DataBuffer
            && self.PreviousTaskFile == other.PreviousTaskFile
            && self.CurrentTaskFile == other.CurrentTaskFile
    }
}
impl ::std::cmp::Eq for ATA_PASS_THROUGH_DIRECT {}
unsafe impl ::windows::runtime::Abi for ATA_PASS_THROUGH_DIRECT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ATA_PASS_THROUGH_DIRECT32 {
    pub Length: u16,
    pub AtaFlags: u16,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub ReservedAsUchar: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub ReservedAsUlong: u32,
    pub DataBuffer: *mut ::std::ffi::c_void,
    pub PreviousTaskFile: [u8; 8],
    pub CurrentTaskFile: [u8; 8],
}
impl ATA_PASS_THROUGH_DIRECT32 {}
impl ::std::default::Default for ATA_PASS_THROUGH_DIRECT32 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ATA_PASS_THROUGH_DIRECT32 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ATA_PASS_THROUGH_DIRECT32")
            .field("Length", &self.Length)
            .field("AtaFlags", &self.AtaFlags)
            .field("PathId", &self.PathId)
            .field("TargetId", &self.TargetId)
            .field("Lun", &self.Lun)
            .field("ReservedAsUchar", &self.ReservedAsUchar)
            .field("DataTransferLength", &self.DataTransferLength)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("ReservedAsUlong", &self.ReservedAsUlong)
            .field("DataBuffer", &self.DataBuffer)
            .field("PreviousTaskFile", &self.PreviousTaskFile)
            .field("CurrentTaskFile", &self.CurrentTaskFile)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ATA_PASS_THROUGH_DIRECT32 {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length
            && self.AtaFlags == other.AtaFlags
            && self.PathId == other.PathId
            && self.TargetId == other.TargetId
            && self.Lun == other.Lun
            && self.ReservedAsUchar == other.ReservedAsUchar
            && self.DataTransferLength == other.DataTransferLength
            && self.TimeOutValue == other.TimeOutValue
            && self.ReservedAsUlong == other.ReservedAsUlong
            && self.DataBuffer == other.DataBuffer
            && self.PreviousTaskFile == other.PreviousTaskFile
            && self.CurrentTaskFile == other.CurrentTaskFile
    }
}
impl ::std::cmp::Eq for ATA_PASS_THROUGH_DIRECT32 {}
unsafe impl ::windows::runtime::Abi for ATA_PASS_THROUGH_DIRECT32 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ATA_PASS_THROUGH_EX {
    pub Length: u16,
    pub AtaFlags: u16,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub ReservedAsUchar: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub ReservedAsUlong: u32,
    pub DataBufferOffset: usize,
    pub PreviousTaskFile: [u8; 8],
    pub CurrentTaskFile: [u8; 8],
}
impl ATA_PASS_THROUGH_EX {}
impl ::std::default::Default for ATA_PASS_THROUGH_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ATA_PASS_THROUGH_EX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ATA_PASS_THROUGH_EX")
            .field("Length", &self.Length)
            .field("AtaFlags", &self.AtaFlags)
            .field("PathId", &self.PathId)
            .field("TargetId", &self.TargetId)
            .field("Lun", &self.Lun)
            .field("ReservedAsUchar", &self.ReservedAsUchar)
            .field("DataTransferLength", &self.DataTransferLength)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("ReservedAsUlong", &self.ReservedAsUlong)
            .field("DataBufferOffset", &self.DataBufferOffset)
            .field("PreviousTaskFile", &self.PreviousTaskFile)
            .field("CurrentTaskFile", &self.CurrentTaskFile)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ATA_PASS_THROUGH_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length
            && self.AtaFlags == other.AtaFlags
            && self.PathId == other.PathId
            && self.TargetId == other.TargetId
            && self.Lun == other.Lun
            && self.ReservedAsUchar == other.ReservedAsUchar
            && self.DataTransferLength == other.DataTransferLength
            && self.TimeOutValue == other.TimeOutValue
            && self.ReservedAsUlong == other.ReservedAsUlong
            && self.DataBufferOffset == other.DataBufferOffset
            && self.PreviousTaskFile == other.PreviousTaskFile
            && self.CurrentTaskFile == other.CurrentTaskFile
    }
}
impl ::std::cmp::Eq for ATA_PASS_THROUGH_EX {}
unsafe impl ::windows::runtime::Abi for ATA_PASS_THROUGH_EX {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ATA_PASS_THROUGH_EX32 {
    pub Length: u16,
    pub AtaFlags: u16,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub ReservedAsUchar: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub ReservedAsUlong: u32,
    pub DataBufferOffset: u32,
    pub PreviousTaskFile: [u8; 8],
    pub CurrentTaskFile: [u8; 8],
}
impl ATA_PASS_THROUGH_EX32 {}
impl ::std::default::Default for ATA_PASS_THROUGH_EX32 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ATA_PASS_THROUGH_EX32 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ATA_PASS_THROUGH_EX32")
            .field("Length", &self.Length)
            .field("AtaFlags", &self.AtaFlags)
            .field("PathId", &self.PathId)
            .field("TargetId", &self.TargetId)
            .field("Lun", &self.Lun)
            .field("ReservedAsUchar", &self.ReservedAsUchar)
            .field("DataTransferLength", &self.DataTransferLength)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("ReservedAsUlong", &self.ReservedAsUlong)
            .field("DataBufferOffset", &self.DataBufferOffset)
            .field("PreviousTaskFile", &self.PreviousTaskFile)
            .field("CurrentTaskFile", &self.CurrentTaskFile)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ATA_PASS_THROUGH_EX32 {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length
            && self.AtaFlags == other.AtaFlags
            && self.PathId == other.PathId
            && self.TargetId == other.TargetId
            && self.Lun == other.Lun
            && self.ReservedAsUchar == other.ReservedAsUchar
            && self.DataTransferLength == other.DataTransferLength
            && self.TimeOutValue == other.TimeOutValue
            && self.ReservedAsUlong == other.ReservedAsUlong
            && self.DataBufferOffset == other.DataBufferOffset
            && self.PreviousTaskFile == other.PreviousTaskFile
            && self.CurrentTaskFile == other.CurrentTaskFile
    }
}
impl ::std::cmp::Eq for ATA_PASS_THROUGH_EX32 {}
unsafe impl ::windows::runtime::Abi for ATA_PASS_THROUGH_EX32 {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AddISNSServerA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    address: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn AddISNSServerA(address: super::super::Foundation::PSTR) -> u32;
        }
        ::std::mem::transmute(AddISNSServerA(address.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AddISNSServerW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    address: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn AddISNSServerW(address: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(AddISNSServerW(address.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn AddIScsiConnectionA<
    'a,
    Param7: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID,
    reserved: *mut ::std::ffi::c_void,
    initiatorportnumber: u32,
    targetportal: *mut ISCSI_TARGET_PORTALA,
    securityflags: u64,
    loginoptions: *mut ISCSI_LOGIN_OPTIONS,
    keysize: u32,
    key: Param7,
    connectionid: *mut ISCSI_UNIQUE_SESSION_ID,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn AddIScsiConnectionA(
                uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID,
                reserved: *mut ::std::ffi::c_void,
                initiatorportnumber: u32,
                targetportal: *mut ISCSI_TARGET_PORTALA,
                securityflags: u64,
                loginoptions: *mut ISCSI_LOGIN_OPTIONS,
                keysize: u32,
                key: super::super::Foundation::PSTR,
                connectionid: *mut ISCSI_UNIQUE_SESSION_ID,
            ) -> u32;
        }
        ::std::mem::transmute(AddIScsiConnectionA(
            ::std::mem::transmute(uniquesessionid),
            ::std::mem::transmute(reserved),
            ::std::mem::transmute(initiatorportnumber),
            ::std::mem::transmute(targetportal),
            ::std::mem::transmute(securityflags),
            ::std::mem::transmute(loginoptions),
            ::std::mem::transmute(keysize),
            key.into_param().abi(),
            ::std::mem::transmute(connectionid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AddIScsiConnectionW<
    'a,
    Param7: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID,
    reserved: *mut ::std::ffi::c_void,
    initiatorportnumber: u32,
    targetportal: *mut ISCSI_TARGET_PORTALW,
    securityflags: u64,
    loginoptions: *mut ISCSI_LOGIN_OPTIONS,
    keysize: u32,
    key: Param7,
    connectionid: *mut ISCSI_UNIQUE_SESSION_ID,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn AddIScsiConnectionW(
                uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID,
                reserved: *mut ::std::ffi::c_void,
                initiatorportnumber: u32,
                targetportal: *mut ISCSI_TARGET_PORTALW,
                securityflags: u64,
                loginoptions: *mut ISCSI_LOGIN_OPTIONS,
                keysize: u32,
                key: super::super::Foundation::PSTR,
                connectionid: *mut ISCSI_UNIQUE_SESSION_ID,
            ) -> u32;
        }
        ::std::mem::transmute(AddIScsiConnectionW(
            ::std::mem::transmute(uniquesessionid),
            ::std::mem::transmute(reserved),
            ::std::mem::transmute(initiatorportnumber),
            ::std::mem::transmute(targetportal),
            ::std::mem::transmute(securityflags),
            ::std::mem::transmute(loginoptions),
            ::std::mem::transmute(keysize),
            key.into_param().abi(),
            ::std::mem::transmute(connectionid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn AddIScsiSendTargetPortalA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    initiatorinstance: Param0,
    initiatorportnumber: u32,
    loginoptions: *mut ISCSI_LOGIN_OPTIONS,
    securityflags: u64,
    portal: *mut ISCSI_TARGET_PORTALA,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn AddIScsiSendTargetPortalA(
                initiatorinstance: super::super::Foundation::PSTR,
                initiatorportnumber: u32,
                loginoptions: *mut ISCSI_LOGIN_OPTIONS,
                securityflags: u64,
                portal: *mut ISCSI_TARGET_PORTALA,
            ) -> u32;
        }
        ::std::mem::transmute(AddIScsiSendTargetPortalA(
            initiatorinstance.into_param().abi(),
            ::std::mem::transmute(initiatorportnumber),
            ::std::mem::transmute(loginoptions),
            ::std::mem::transmute(securityflags),
            ::std::mem::transmute(portal),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AddIScsiSendTargetPortalW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    initiatorinstance: Param0,
    initiatorportnumber: u32,
    loginoptions: *mut ISCSI_LOGIN_OPTIONS,
    securityflags: u64,
    portal: *mut ISCSI_TARGET_PORTALW,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn AddIScsiSendTargetPortalW(
                initiatorinstance: super::super::Foundation::PWSTR,
                initiatorportnumber: u32,
                loginoptions: *mut ISCSI_LOGIN_OPTIONS,
                securityflags: u64,
                portal: *mut ISCSI_TARGET_PORTALW,
            ) -> u32;
        }
        ::std::mem::transmute(AddIScsiSendTargetPortalW(
            initiatorinstance.into_param().abi(),
            ::std::mem::transmute(initiatorportnumber),
            ::std::mem::transmute(loginoptions),
            ::std::mem::transmute(securityflags),
            ::std::mem::transmute(portal),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn AddIScsiStaticTargetA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>,
>(
    targetname: Param0,
    targetalias: Param1,
    targetflags: u32,
    persist: Param3,
    mappings: *mut ISCSI_TARGET_MAPPINGA,
    loginoptions: *mut ISCSI_LOGIN_OPTIONS,
    portalgroup: *mut ISCSI_TARGET_PORTAL_GROUPA,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn AddIScsiStaticTargetA(
                targetname: super::super::Foundation::PSTR,
                targetalias: super::super::Foundation::PSTR,
                targetflags: u32,
                persist: super::super::Foundation::BOOLEAN,
                mappings: *mut ISCSI_TARGET_MAPPINGA,
                loginoptions: *mut ISCSI_LOGIN_OPTIONS,
                portalgroup: *mut ISCSI_TARGET_PORTAL_GROUPA,
            ) -> u32;
        }
        ::std::mem::transmute(AddIScsiStaticTargetA(
            targetname.into_param().abi(),
            targetalias.into_param().abi(),
            ::std::mem::transmute(targetflags),
            persist.into_param().abi(),
            ::std::mem::transmute(mappings),
            ::std::mem::transmute(loginoptions),
            ::std::mem::transmute(portalgroup),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AddIScsiStaticTargetW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>,
>(
    targetname: Param0,
    targetalias: Param1,
    targetflags: u32,
    persist: Param3,
    mappings: *mut ISCSI_TARGET_MAPPINGW,
    loginoptions: *mut ISCSI_LOGIN_OPTIONS,
    portalgroup: *mut ISCSI_TARGET_PORTAL_GROUPW,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn AddIScsiStaticTargetW(
                targetname: super::super::Foundation::PWSTR,
                targetalias: super::super::Foundation::PWSTR,
                targetflags: u32,
                persist: super::super::Foundation::BOOLEAN,
                mappings: *mut ISCSI_TARGET_MAPPINGW,
                loginoptions: *mut ISCSI_LOGIN_OPTIONS,
                portalgroup: *mut ISCSI_TARGET_PORTAL_GROUPW,
            ) -> u32;
        }
        ::std::mem::transmute(AddIScsiStaticTargetW(
            targetname.into_param().abi(),
            targetalias.into_param().abi(),
            ::std::mem::transmute(targetflags),
            persist.into_param().abi(),
            ::std::mem::transmute(mappings),
            ::std::mem::transmute(loginoptions),
            ::std::mem::transmute(portalgroup),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AddPersistentIScsiDeviceA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    devicepath: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn AddPersistentIScsiDeviceA(devicepath: super::super::Foundation::PSTR) -> u32;
        }
        ::std::mem::transmute(AddPersistentIScsiDeviceA(devicepath.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AddPersistentIScsiDeviceW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    devicepath: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn AddPersistentIScsiDeviceW(devicepath: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(AddPersistentIScsiDeviceW(devicepath.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AddRadiusServerA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    address: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn AddRadiusServerA(address: super::super::Foundation::PSTR) -> u32;
        }
        ::std::mem::transmute(AddRadiusServerA(address.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AddRadiusServerW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    address: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn AddRadiusServerW(address: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(AddRadiusServerW(address.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn ClearPersistentIScsiDevices() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn ClearPersistentIScsiDevices() -> u32;
        }
        ::std::mem::transmute(ClearPersistentIScsiDevices())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DSM_NOTIFICATION_REQUEST_BLOCK {
    pub Size: u32,
    pub Version: u32,
    pub NotifyFlags: u32,
    pub DataSetProfile: u32,
    pub Reserved: [u32; 3],
    pub DataSetRangesCount: u32,
    pub DataSetRanges: [MP_DEVICE_DATA_SET_RANGE; 1],
}
impl DSM_NOTIFICATION_REQUEST_BLOCK {}
impl ::std::default::Default for DSM_NOTIFICATION_REQUEST_BLOCK {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DSM_NOTIFICATION_REQUEST_BLOCK {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DSM_NOTIFICATION_REQUEST_BLOCK")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("NotifyFlags", &self.NotifyFlags)
            .field("DataSetProfile", &self.DataSetProfile)
            .field("Reserved", &self.Reserved)
            .field("DataSetRangesCount", &self.DataSetRangesCount)
            .field("DataSetRanges", &self.DataSetRanges)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DSM_NOTIFICATION_REQUEST_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.NotifyFlags == other.NotifyFlags
            && self.DataSetProfile == other.DataSetProfile
            && self.Reserved == other.Reserved
            && self.DataSetRangesCount == other.DataSetRangesCount
            && self.DataSetRanges == other.DataSetRanges
    }
}
impl ::std::cmp::Eq for DSM_NOTIFICATION_REQUEST_BLOCK {}
unsafe impl ::windows::runtime::Abi for DSM_NOTIFICATION_REQUEST_BLOCK {
    type Abi = Self;
    type DefaultType = Self;
}
pub type DUMP_DEVICE_POWERON_ROUTINE =
    unsafe extern "system" fn(context: *const ::std::ffi::c_void) -> i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DUMP_DRIVER {
    pub DumpDriverList: *mut ::std::ffi::c_void,
    pub DriverName: [u16; 15],
    pub BaseName: [u16; 15],
}
impl DUMP_DRIVER {}
impl ::std::default::Default for DUMP_DRIVER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DUMP_DRIVER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DUMP_DRIVER")
            .field("DumpDriverList", &self.DumpDriverList)
            .field("DriverName", &self.DriverName)
            .field("BaseName", &self.BaseName)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DUMP_DRIVER {
    fn eq(&self, other: &Self) -> bool {
        self.DumpDriverList == other.DumpDriverList
            && self.DriverName == other.DriverName
            && self.BaseName == other.BaseName
    }
}
impl ::std::cmp::Eq for DUMP_DRIVER {}
unsafe impl ::windows::runtime::Abi for DUMP_DRIVER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DUMP_DRIVER_EX {
    pub DumpDriverList: *mut ::std::ffi::c_void,
    pub DriverName: [u16; 15],
    pub BaseName: [u16; 15],
    pub DriverFullPath: NTSCSI_UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl DUMP_DRIVER_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DUMP_DRIVER_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DUMP_DRIVER_EX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DUMP_DRIVER_EX")
            .field("DumpDriverList", &self.DumpDriverList)
            .field("DriverName", &self.DriverName)
            .field("BaseName", &self.BaseName)
            .field("DriverFullPath", &self.DriverFullPath)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DUMP_DRIVER_EX {
    fn eq(&self, other: &Self) -> bool {
        self.DumpDriverList == other.DumpDriverList
            && self.DriverName == other.DriverName
            && self.BaseName == other.BaseName
            && self.DriverFullPath == other.DriverFullPath
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DUMP_DRIVER_EX {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DUMP_DRIVER_EX {
    type Abi = Self;
    type DefaultType = Self;
}
pub const DUMP_DRIVER_NAME_LENGTH: u32 = 15u32;
pub const DUMP_EX_FLAG_DRIVER_FULL_PATH_SUPPORT: u32 = 8u32;
pub const DUMP_EX_FLAG_RESUME_SUPPORT: u32 = 4u32;
pub const DUMP_EX_FLAG_SUPPORT_64BITMEMORY: u32 = 1u32;
pub const DUMP_EX_FLAG_SUPPORT_DD_TELEMETRY: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DUMP_POINTERS {
    pub AdapterObject: *mut _ADAPTER_OBJECT,
    pub MappedRegisterBase: *mut ::std::ffi::c_void,
    pub DumpData: *mut ::std::ffi::c_void,
    pub CommonBufferVa: *mut ::std::ffi::c_void,
    pub CommonBufferPa: i64,
    pub CommonBufferSize: u32,
    pub AllocateCommonBuffers: super::super::Foundation::BOOLEAN,
    pub UseDiskDump: super::super::Foundation::BOOLEAN,
    pub Spare1: [u8; 2],
    pub DeviceObject: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl DUMP_POINTERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DUMP_POINTERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DUMP_POINTERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DUMP_POINTERS")
            .field("AdapterObject", &self.AdapterObject)
            .field("MappedRegisterBase", &self.MappedRegisterBase)
            .field("DumpData", &self.DumpData)
            .field("CommonBufferVa", &self.CommonBufferVa)
            .field("CommonBufferPa", &self.CommonBufferPa)
            .field("CommonBufferSize", &self.CommonBufferSize)
            .field("AllocateCommonBuffers", &self.AllocateCommonBuffers)
            .field("UseDiskDump", &self.UseDiskDump)
            .field("Spare1", &self.Spare1)
            .field("DeviceObject", &self.DeviceObject)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DUMP_POINTERS {
    fn eq(&self, other: &Self) -> bool {
        self.AdapterObject == other.AdapterObject
            && self.MappedRegisterBase == other.MappedRegisterBase
            && self.DumpData == other.DumpData
            && self.CommonBufferVa == other.CommonBufferVa
            && self.CommonBufferPa == other.CommonBufferPa
            && self.CommonBufferSize == other.CommonBufferSize
            && self.AllocateCommonBuffers == other.AllocateCommonBuffers
            && self.UseDiskDump == other.UseDiskDump
            && self.Spare1 == other.Spare1
            && self.DeviceObject == other.DeviceObject
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DUMP_POINTERS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DUMP_POINTERS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DUMP_POINTERS_EX {
    pub Header: DUMP_POINTERS_VERSION,
    pub DumpData: *mut ::std::ffi::c_void,
    pub CommonBufferVa: *mut ::std::ffi::c_void,
    pub CommonBufferSize: u32,
    pub AllocateCommonBuffers: super::super::Foundation::BOOLEAN,
    pub DeviceObject: *mut ::std::ffi::c_void,
    pub DriverList: *mut ::std::ffi::c_void,
    pub dwPortFlags: u32,
    pub MaxDeviceDumpSectionSize: u32,
    pub MaxDeviceDumpLevel: u32,
    pub MaxTransferSize: u32,
    pub AdapterObject: *mut ::std::ffi::c_void,
    pub MappedRegisterBase: *mut ::std::ffi::c_void,
    pub DeviceReady: *mut super::super::Foundation::BOOLEAN,
    pub DumpDevicePowerOn: ::std::option::Option<PDUMP_DEVICE_POWERON_ROUTINE>,
    pub DumpDevicePowerOnContext: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl DUMP_POINTERS_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DUMP_POINTERS_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DUMP_POINTERS_EX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DUMP_POINTERS_EX")
            .field("Header", &self.Header)
            .field("DumpData", &self.DumpData)
            .field("CommonBufferVa", &self.CommonBufferVa)
            .field("CommonBufferSize", &self.CommonBufferSize)
            .field("AllocateCommonBuffers", &self.AllocateCommonBuffers)
            .field("DeviceObject", &self.DeviceObject)
            .field("DriverList", &self.DriverList)
            .field("dwPortFlags", &self.dwPortFlags)
            .field("MaxDeviceDumpSectionSize", &self.MaxDeviceDumpSectionSize)
            .field("MaxDeviceDumpLevel", &self.MaxDeviceDumpLevel)
            .field("MaxTransferSize", &self.MaxTransferSize)
            .field("AdapterObject", &self.AdapterObject)
            .field("MappedRegisterBase", &self.MappedRegisterBase)
            .field("DeviceReady", &self.DeviceReady)
            .field("DumpDevicePowerOnContext", &self.DumpDevicePowerOnContext)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DUMP_POINTERS_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
            && self.DumpData == other.DumpData
            && self.CommonBufferVa == other.CommonBufferVa
            && self.CommonBufferSize == other.CommonBufferSize
            && self.AllocateCommonBuffers == other.AllocateCommonBuffers
            && self.DeviceObject == other.DeviceObject
            && self.DriverList == other.DriverList
            && self.dwPortFlags == other.dwPortFlags
            && self.MaxDeviceDumpSectionSize == other.MaxDeviceDumpSectionSize
            && self.MaxDeviceDumpLevel == other.MaxDeviceDumpLevel
            && self.MaxTransferSize == other.MaxTransferSize
            && self.AdapterObject == other.AdapterObject
            && self.MappedRegisterBase == other.MappedRegisterBase
            && self.DeviceReady == other.DeviceReady
            && self.DumpDevicePowerOn.map(|f| f as usize)
                == other.DumpDevicePowerOn.map(|f| f as usize)
            && self.DumpDevicePowerOnContext == other.DumpDevicePowerOnContext
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DUMP_POINTERS_EX {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DUMP_POINTERS_EX {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DUMP_POINTERS_VERSION {
    pub Version: u32,
    pub Size: u32,
}
impl DUMP_POINTERS_VERSION {}
impl ::std::default::Default for DUMP_POINTERS_VERSION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DUMP_POINTERS_VERSION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DUMP_POINTERS_VERSION")
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DUMP_POINTERS_VERSION {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size
    }
}
impl ::std::cmp::Eq for DUMP_POINTERS_VERSION {}
unsafe impl ::windows::runtime::Abi for DUMP_POINTERS_VERSION {
    type Abi = Self;
    type DefaultType = Self;
}
pub const DUMP_POINTERS_VERSION_1: u32 = 1u32;
pub const DUMP_POINTERS_VERSION_2: u32 = 2u32;
pub const DUMP_POINTERS_VERSION_3: u32 = 3u32;
pub const DUMP_POINTERS_VERSION_4: u32 = 4u32;
pub const FILE_DEVICE_SCSI: u32 = 27u32;
pub const FIRMWARE_FUNCTION_ACTIVATE: u32 = 3u32;
pub const FIRMWARE_FUNCTION_DOWNLOAD: u32 = 2u32;
pub const FIRMWARE_FUNCTION_GET_INFO: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FIRMWARE_REQUEST_BLOCK {
    pub Version: u32,
    pub Size: u32,
    pub Function: u32,
    pub Flags: u32,
    pub DataBufferOffset: u32,
    pub DataBufferLength: u32,
}
impl FIRMWARE_REQUEST_BLOCK {}
impl ::std::default::Default for FIRMWARE_REQUEST_BLOCK {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FIRMWARE_REQUEST_BLOCK {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FIRMWARE_REQUEST_BLOCK")
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .field("Function", &self.Function)
            .field("Flags", &self.Flags)
            .field("DataBufferOffset", &self.DataBufferOffset)
            .field("DataBufferLength", &self.DataBufferLength)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FIRMWARE_REQUEST_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Size == other.Size
            && self.Function == other.Function
            && self.Flags == other.Flags
            && self.DataBufferOffset == other.DataBufferOffset
            && self.DataBufferLength == other.DataBufferLength
    }
}
impl ::std::cmp::Eq for FIRMWARE_REQUEST_BLOCK {}
unsafe impl ::windows::runtime::Abi for FIRMWARE_REQUEST_BLOCK {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FIRMWARE_REQUEST_BLOCK_STRUCTURE_VERSION: u32 = 1u32;
pub const FIRMWARE_REQUEST_FLAG_CONTROLLER: u32 = 1u32;
pub const FIRMWARE_REQUEST_FLAG_FIRST_SEGMENT: u32 = 4u32;
pub const FIRMWARE_REQUEST_FLAG_LAST_SEGMENT: u32 = 2u32;
pub const FIRMWARE_REQUEST_FLAG_SWITCH_TO_EXISTING_FIRMWARE: u32 = 2147483648u32;
pub const FIRMWARE_STATUS_COMMAND_ABORT: u32 = 133u32;
pub const FIRMWARE_STATUS_CONTROLLER_ERROR: u32 = 16u32;
pub const FIRMWARE_STATUS_DEVICE_ERROR: u32 = 64u32;
pub const FIRMWARE_STATUS_END_OF_MEDIA: u32 = 134u32;
pub const FIRMWARE_STATUS_ERROR: u32 = 1u32;
pub const FIRMWARE_STATUS_ID_NOT_FOUND: u32 = 131u32;
pub const FIRMWARE_STATUS_ILLEGAL_LENGTH: u32 = 135u32;
pub const FIRMWARE_STATUS_ILLEGAL_REQUEST: u32 = 2u32;
pub const FIRMWARE_STATUS_INPUT_BUFFER_TOO_BIG: u32 = 4u32;
pub const FIRMWARE_STATUS_INTERFACE_CRC_ERROR: u32 = 128u32;
pub const FIRMWARE_STATUS_INVALID_IMAGE: u32 = 7u32;
pub const FIRMWARE_STATUS_INVALID_PARAMETER: u32 = 3u32;
pub const FIRMWARE_STATUS_INVALID_SLOT: u32 = 6u32;
pub const FIRMWARE_STATUS_MEDIA_CHANGE: u32 = 130u32;
pub const FIRMWARE_STATUS_MEDIA_CHANGE_REQUEST: u32 = 132u32;
pub const FIRMWARE_STATUS_OUTPUT_BUFFER_TOO_SMALL: u32 = 5u32;
pub const FIRMWARE_STATUS_POWER_CYCLE_REQUIRED: u32 = 32u32;
pub const FIRMWARE_STATUS_SUCCESS: u32 = 0u32;
pub const FIRMWARE_STATUS_UNCORRECTABLE_DATA_ERROR: u32 = 129u32;
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn GetDevicesForIScsiSessionA(
    uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID,
    devicecount: *mut u32,
    devices: *mut ISCSI_DEVICE_ON_SESSIONA,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn GetDevicesForIScsiSessionA(
                uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID,
                devicecount: *mut u32,
                devices: *mut ISCSI_DEVICE_ON_SESSIONA,
            ) -> u32;
        }
        ::std::mem::transmute(GetDevicesForIScsiSessionA(
            ::std::mem::transmute(uniquesessionid),
            ::std::mem::transmute(devicecount),
            ::std::mem::transmute(devices),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn GetDevicesForIScsiSessionW(
    uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID,
    devicecount: *mut u32,
    devices: *mut ISCSI_DEVICE_ON_SESSIONW,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn GetDevicesForIScsiSessionW(
                uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID,
                devicecount: *mut u32,
                devices: *mut ISCSI_DEVICE_ON_SESSIONW,
            ) -> u32;
        }
        ::std::mem::transmute(GetDevicesForIScsiSessionW(
            ::std::mem::transmute(uniquesessionid),
            ::std::mem::transmute(devicecount),
            ::std::mem::transmute(devices),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetIScsiIKEInfoA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    initiatorname: Param0,
    initiatorportnumber: u32,
    reserved: *mut u32,
    authinfo: *mut IKE_AUTHENTICATION_INFORMATION,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn GetIScsiIKEInfoA(
                initiatorname: super::super::Foundation::PSTR,
                initiatorportnumber: u32,
                reserved: *mut u32,
                authinfo: *mut IKE_AUTHENTICATION_INFORMATION,
            ) -> u32;
        }
        ::std::mem::transmute(GetIScsiIKEInfoA(
            initiatorname.into_param().abi(),
            ::std::mem::transmute(initiatorportnumber),
            ::std::mem::transmute(reserved),
            ::std::mem::transmute(authinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetIScsiIKEInfoW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    initiatorname: Param0,
    initiatorportnumber: u32,
    reserved: *mut u32,
    authinfo: *mut IKE_AUTHENTICATION_INFORMATION,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn GetIScsiIKEInfoW(
                initiatorname: super::super::Foundation::PWSTR,
                initiatorportnumber: u32,
                reserved: *mut u32,
                authinfo: *mut IKE_AUTHENTICATION_INFORMATION,
            ) -> u32;
        }
        ::std::mem::transmute(GetIScsiIKEInfoW(
            initiatorname.into_param().abi(),
            ::std::mem::transmute(initiatorportnumber),
            ::std::mem::transmute(reserved),
            ::std::mem::transmute(authinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetIScsiInitiatorNodeNameA(initiatornodename: super::super::Foundation::PSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn GetIScsiInitiatorNodeNameA(initiatornodename: super::super::Foundation::PSTR)
                -> u32;
        }
        ::std::mem::transmute(GetIScsiInitiatorNodeNameA(::std::mem::transmute(
            initiatornodename,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetIScsiInitiatorNodeNameW(
    initiatornodename: super::super::Foundation::PWSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn GetIScsiInitiatorNodeNameW(
                initiatornodename: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(GetIScsiInitiatorNodeNameW(::std::mem::transmute(
            initiatornodename,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetIScsiSessionListA(
    buffersize: *mut u32,
    sessioncount: *mut u32,
    sessioninfo: *mut ISCSI_SESSION_INFOA,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn GetIScsiSessionListA(
                buffersize: *mut u32,
                sessioncount: *mut u32,
                sessioninfo: *mut ISCSI_SESSION_INFOA,
            ) -> u32;
        }
        ::std::mem::transmute(GetIScsiSessionListA(
            ::std::mem::transmute(buffersize),
            ::std::mem::transmute(sessioncount),
            ::std::mem::transmute(sessioninfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetIScsiSessionListEx(
    buffersize: *mut u32,
    sessioncountptr: *mut u32,
    sessioninfo: *mut ISCSI_SESSION_INFO_EX,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn GetIScsiSessionListEx(
                buffersize: *mut u32,
                sessioncountptr: *mut u32,
                sessioninfo: *mut ISCSI_SESSION_INFO_EX,
            ) -> u32;
        }
        ::std::mem::transmute(GetIScsiSessionListEx(
            ::std::mem::transmute(buffersize),
            ::std::mem::transmute(sessioncountptr),
            ::std::mem::transmute(sessioninfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetIScsiSessionListW(
    buffersize: *mut u32,
    sessioncount: *mut u32,
    sessioninfo: *mut ISCSI_SESSION_INFOW,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn GetIScsiSessionListW(
                buffersize: *mut u32,
                sessioncount: *mut u32,
                sessioninfo: *mut ISCSI_SESSION_INFOW,
            ) -> u32;
        }
        ::std::mem::transmute(GetIScsiSessionListW(
            ::std::mem::transmute(buffersize),
            ::std::mem::transmute(sessioncount),
            ::std::mem::transmute(sessioninfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetIScsiTargetInformationA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    targetname: Param0,
    discoverymechanism: Param1,
    infoclass: TARGET_INFORMATION_CLASS,
    buffersize: *mut u32,
    buffer: *mut ::std::ffi::c_void,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn GetIScsiTargetInformationA(
                targetname: super::super::Foundation::PSTR,
                discoverymechanism: super::super::Foundation::PSTR,
                infoclass: TARGET_INFORMATION_CLASS,
                buffersize: *mut u32,
                buffer: *mut ::std::ffi::c_void,
            ) -> u32;
        }
        ::std::mem::transmute(GetIScsiTargetInformationA(
            targetname.into_param().abi(),
            discoverymechanism.into_param().abi(),
            ::std::mem::transmute(infoclass),
            ::std::mem::transmute(buffersize),
            ::std::mem::transmute(buffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetIScsiTargetInformationW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    targetname: Param0,
    discoverymechanism: Param1,
    infoclass: TARGET_INFORMATION_CLASS,
    buffersize: *mut u32,
    buffer: *mut ::std::ffi::c_void,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn GetIScsiTargetInformationW(
                targetname: super::super::Foundation::PWSTR,
                discoverymechanism: super::super::Foundation::PWSTR,
                infoclass: TARGET_INFORMATION_CLASS,
                buffersize: *mut u32,
                buffer: *mut ::std::ffi::c_void,
            ) -> u32;
        }
        ::std::mem::transmute(GetIScsiTargetInformationW(
            targetname.into_param().abi(),
            discoverymechanism.into_param().abi(),
            ::std::mem::transmute(infoclass),
            ::std::mem::transmute(buffersize),
            ::std::mem::transmute(buffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn GetIScsiVersionInformation(versioninfo: *mut ISCSI_VERSION_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn GetIScsiVersionInformation(versioninfo: *mut ISCSI_VERSION_INFO) -> u32;
        }
        ::std::mem::transmute(GetIScsiVersionInformation(::std::mem::transmute(
            versioninfo,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct HYBRID_DEMOTE_BY_SIZE {
    pub Version: u32,
    pub Size: u32,
    pub SourcePriority: u8,
    pub TargetPriority: u8,
    pub Reserved0: u16,
    pub Reserved1: u32,
    pub LbaCount: u64,
}
impl HYBRID_DEMOTE_BY_SIZE {}
impl ::std::default::Default for HYBRID_DEMOTE_BY_SIZE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HYBRID_DEMOTE_BY_SIZE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HYBRID_DEMOTE_BY_SIZE")
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .field("SourcePriority", &self.SourcePriority)
            .field("TargetPriority", &self.TargetPriority)
            .field("Reserved0", &self.Reserved0)
            .field("Reserved1", &self.Reserved1)
            .field("LbaCount", &self.LbaCount)
            .finish()
    }
}
impl ::std::cmp::PartialEq for HYBRID_DEMOTE_BY_SIZE {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Size == other.Size
            && self.SourcePriority == other.SourcePriority
            && self.TargetPriority == other.TargetPriority
            && self.Reserved0 == other.Reserved0
            && self.Reserved1 == other.Reserved1
            && self.LbaCount == other.LbaCount
    }
}
impl ::std::cmp::Eq for HYBRID_DEMOTE_BY_SIZE {}
unsafe impl ::windows::runtime::Abi for HYBRID_DEMOTE_BY_SIZE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct HYBRID_DIRTY_THRESHOLDS {
    pub Version: u32,
    pub Size: u32,
    pub DirtyLowThreshold: u32,
    pub DirtyHighThreshold: u32,
}
impl HYBRID_DIRTY_THRESHOLDS {}
impl ::std::default::Default for HYBRID_DIRTY_THRESHOLDS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HYBRID_DIRTY_THRESHOLDS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HYBRID_DIRTY_THRESHOLDS")
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .field("DirtyLowThreshold", &self.DirtyLowThreshold)
            .field("DirtyHighThreshold", &self.DirtyHighThreshold)
            .finish()
    }
}
impl ::std::cmp::PartialEq for HYBRID_DIRTY_THRESHOLDS {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Size == other.Size
            && self.DirtyLowThreshold == other.DirtyLowThreshold
            && self.DirtyHighThreshold == other.DirtyHighThreshold
    }
}
impl ::std::cmp::Eq for HYBRID_DIRTY_THRESHOLDS {}
unsafe impl ::windows::runtime::Abi for HYBRID_DIRTY_THRESHOLDS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const HYBRID_FUNCTION_DEMOTE_BY_SIZE: u32 = 19u32;
pub const HYBRID_FUNCTION_DISABLE_CACHING_MEDIUM: u32 = 16u32;
pub const HYBRID_FUNCTION_ENABLE_CACHING_MEDIUM: u32 = 17u32;
pub const HYBRID_FUNCTION_GET_INFO: u32 = 1u32;
pub const HYBRID_FUNCTION_SET_DIRTY_THRESHOLD: u32 = 18u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HYBRID_INFORMATION {
    pub Version: u32,
    pub Size: u32,
    pub HybridSupported: super::super::Foundation::BOOLEAN,
    pub Status: NVCACHE_STATUS,
    pub CacheTypeEffective: NVCACHE_TYPE,
    pub CacheTypeDefault: NVCACHE_TYPE,
    pub FractionBase: u32,
    pub CacheSize: u64,
    pub Attributes: HYBRID_INFORMATION_0,
    pub Priorities: HYBRID_INFORMATION_1,
}
#[cfg(feature = "Win32_Foundation")]
impl HYBRID_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HYBRID_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HYBRID_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HYBRID_INFORMATION")
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .field("HybridSupported", &self.HybridSupported)
            .field("Status", &self.Status)
            .field("CacheTypeEffective", &self.CacheTypeEffective)
            .field("CacheTypeDefault", &self.CacheTypeDefault)
            .field("FractionBase", &self.FractionBase)
            .field("CacheSize", &self.CacheSize)
            .field("Attributes", &self.Attributes)
            .field("Priorities", &self.Priorities)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HYBRID_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Size == other.Size
            && self.HybridSupported == other.HybridSupported
            && self.Status == other.Status
            && self.CacheTypeEffective == other.CacheTypeEffective
            && self.CacheTypeDefault == other.CacheTypeDefault
            && self.FractionBase == other.FractionBase
            && self.CacheSize == other.CacheSize
            && self.Attributes == other.Attributes
            && self.Priorities == other.Priorities
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HYBRID_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HYBRID_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct HYBRID_INFORMATION_0 {
    pub _bitfield: u32,
}
impl HYBRID_INFORMATION_0 {}
impl ::std::default::Default for HYBRID_INFORMATION_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HYBRID_INFORMATION_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Attributes_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for HYBRID_INFORMATION_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for HYBRID_INFORMATION_0 {}
unsafe impl ::windows::runtime::Abi for HYBRID_INFORMATION_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HYBRID_INFORMATION_1 {
    pub PriorityLevelCount: u8,
    pub MaxPriorityBehavior: super::super::Foundation::BOOLEAN,
    pub OptimalWriteGranularity: u8,
    pub Reserved: u8,
    pub DirtyThresholdLow: u32,
    pub DirtyThresholdHigh: u32,
    pub SupportedCommands: HYBRID_INFORMATION_1_0,
    pub Priority: [NVCACHE_PRIORITY_LEVEL_DESCRIPTOR; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl HYBRID_INFORMATION_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HYBRID_INFORMATION_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HYBRID_INFORMATION_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Priorities_e__Struct")
            .field("PriorityLevelCount", &self.PriorityLevelCount)
            .field("MaxPriorityBehavior", &self.MaxPriorityBehavior)
            .field("OptimalWriteGranularity", &self.OptimalWriteGranularity)
            .field("Reserved", &self.Reserved)
            .field("DirtyThresholdLow", &self.DirtyThresholdLow)
            .field("DirtyThresholdHigh", &self.DirtyThresholdHigh)
            .field("SupportedCommands", &self.SupportedCommands)
            .field("Priority", &self.Priority)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HYBRID_INFORMATION_1 {
    fn eq(&self, other: &Self) -> bool {
        self.PriorityLevelCount == other.PriorityLevelCount
            && self.MaxPriorityBehavior == other.MaxPriorityBehavior
            && self.OptimalWriteGranularity == other.OptimalWriteGranularity
            && self.Reserved == other.Reserved
            && self.DirtyThresholdLow == other.DirtyThresholdLow
            && self.DirtyThresholdHigh == other.DirtyThresholdHigh
            && self.SupportedCommands == other.SupportedCommands
            && self.Priority == other.Priority
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HYBRID_INFORMATION_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HYBRID_INFORMATION_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct HYBRID_INFORMATION_1_0 {
    pub _bitfield: u32,
    pub MaxEvictCommands: u32,
    pub MaxLbaRangeCountForEvict: u32,
    pub MaxLbaRangeCountForChangeLba: u32,
}
impl HYBRID_INFORMATION_1_0 {}
impl ::std::default::Default for HYBRID_INFORMATION_1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HYBRID_INFORMATION_1_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_SupportedCommands_e__Struct")
            .field("_bitfield", &self._bitfield)
            .field("MaxEvictCommands", &self.MaxEvictCommands)
            .field("MaxLbaRangeCountForEvict", &self.MaxLbaRangeCountForEvict)
            .field(
                "MaxLbaRangeCountForChangeLba",
                &self.MaxLbaRangeCountForChangeLba,
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for HYBRID_INFORMATION_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
            && self.MaxEvictCommands == other.MaxEvictCommands
            && self.MaxLbaRangeCountForEvict == other.MaxLbaRangeCountForEvict
            && self.MaxLbaRangeCountForChangeLba == other.MaxLbaRangeCountForChangeLba
    }
}
impl ::std::cmp::Eq for HYBRID_INFORMATION_1_0 {}
unsafe impl ::windows::runtime::Abi for HYBRID_INFORMATION_1_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct HYBRID_REQUEST_BLOCK {
    pub Version: u32,
    pub Size: u32,
    pub Function: u32,
    pub Flags: u32,
    pub DataBufferOffset: u32,
    pub DataBufferLength: u32,
}
impl HYBRID_REQUEST_BLOCK {}
impl ::std::default::Default for HYBRID_REQUEST_BLOCK {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HYBRID_REQUEST_BLOCK {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HYBRID_REQUEST_BLOCK")
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .field("Function", &self.Function)
            .field("Flags", &self.Flags)
            .field("DataBufferOffset", &self.DataBufferOffset)
            .field("DataBufferLength", &self.DataBufferLength)
            .finish()
    }
}
impl ::std::cmp::PartialEq for HYBRID_REQUEST_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Size == other.Size
            && self.Function == other.Function
            && self.Flags == other.Flags
            && self.DataBufferOffset == other.DataBufferOffset
            && self.DataBufferLength == other.DataBufferLength
    }
}
impl ::std::cmp::Eq for HYBRID_REQUEST_BLOCK {}
unsafe impl ::windows::runtime::Abi for HYBRID_REQUEST_BLOCK {
    type Abi = Self;
    type DefaultType = Self;
}
pub const HYBRID_REQUEST_BLOCK_STRUCTURE_VERSION: u32 = 1u32;
pub const HYBRID_REQUEST_INFO_STRUCTURE_VERSION: u32 = 1u32;
pub const HYBRID_STATUS_ENABLE_REFCOUNT_HOLD: u32 = 16u32;
pub const HYBRID_STATUS_ILLEGAL_REQUEST: u32 = 1u32;
pub const HYBRID_STATUS_INVALID_PARAMETER: u32 = 2u32;
pub const HYBRID_STATUS_OUTPUT_BUFFER_TOO_SMALL: u32 = 3u32;
pub const HYBRID_STATUS_SUCCESS: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IDE_IO_CONTROL {
    pub HeaderLength: u32,
    pub Signature: [u8; 8],
    pub Timeout: u32,
    pub ControlCode: u32,
    pub ReturnStatus: u32,
    pub DataLength: u32,
}
impl IDE_IO_CONTROL {}
impl ::std::default::Default for IDE_IO_CONTROL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IDE_IO_CONTROL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IDE_IO_CONTROL")
            .field("HeaderLength", &self.HeaderLength)
            .field("Signature", &self.Signature)
            .field("Timeout", &self.Timeout)
            .field("ControlCode", &self.ControlCode)
            .field("ReturnStatus", &self.ReturnStatus)
            .field("DataLength", &self.DataLength)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IDE_IO_CONTROL {
    fn eq(&self, other: &Self) -> bool {
        self.HeaderLength == other.HeaderLength
            && self.Signature == other.Signature
            && self.Timeout == other.Timeout
            && self.ControlCode == other.ControlCode
            && self.ReturnStatus == other.ReturnStatus
            && self.DataLength == other.DataLength
    }
}
impl ::std::cmp::Eq for IDE_IO_CONTROL {}
unsafe impl ::windows::runtime::Abi for IDE_IO_CONTROL {
    type Abi = Self;
    type DefaultType = Self;
}
pub const ID_FQDN: u32 = 2u32;
pub const ID_IPV4_ADDR: u32 = 1u32;
pub const ID_IPV6_ADDR: u32 = 5u32;
pub const ID_USER_FQDN: u32 = 3u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IKE_AUTHENTICATION_INFORMATION {
    pub AuthMethod: IKE_AUTHENTICATION_METHOD,
    pub Anonymous: IKE_AUTHENTICATION_INFORMATION_0,
}
impl IKE_AUTHENTICATION_INFORMATION {}
impl ::std::default::Default for IKE_AUTHENTICATION_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IKE_AUTHENTICATION_INFORMATION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IKE_AUTHENTICATION_INFORMATION {}
unsafe impl ::windows::runtime::Abi for IKE_AUTHENTICATION_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IKE_AUTHENTICATION_INFORMATION_0 {
    pub PsKey: IKE_AUTHENTICATION_PRESHARED_KEY,
}
impl IKE_AUTHENTICATION_INFORMATION_0 {}
impl ::std::default::Default for IKE_AUTHENTICATION_INFORMATION_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IKE_AUTHENTICATION_INFORMATION_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IKE_AUTHENTICATION_INFORMATION_0 {}
unsafe impl ::windows::runtime::Abi for IKE_AUTHENTICATION_INFORMATION_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IKE_AUTHENTICATION_METHOD(pub i32);
pub const IKE_AUTHENTICATION_PRESHARED_KEY_METHOD: IKE_AUTHENTICATION_METHOD =
    IKE_AUTHENTICATION_METHOD(1i32);
impl ::std::convert::From<i32> for IKE_AUTHENTICATION_METHOD {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IKE_AUTHENTICATION_METHOD {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IKE_AUTHENTICATION_PRESHARED_KEY {
    pub SecurityFlags: u64,
    pub IdType: u8,
    pub IdLengthInBytes: u32,
    pub Id: *mut u8,
    pub KeyLengthInBytes: u32,
    pub Key: *mut u8,
}
impl IKE_AUTHENTICATION_PRESHARED_KEY {}
impl ::std::default::Default for IKE_AUTHENTICATION_PRESHARED_KEY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IKE_AUTHENTICATION_PRESHARED_KEY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKE_AUTHENTICATION_PRESHARED_KEY")
            .field("SecurityFlags", &self.SecurityFlags)
            .field("IdType", &self.IdType)
            .field("IdLengthInBytes", &self.IdLengthInBytes)
            .field("Id", &self.Id)
            .field("KeyLengthInBytes", &self.KeyLengthInBytes)
            .field("Key", &self.Key)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IKE_AUTHENTICATION_PRESHARED_KEY {
    fn eq(&self, other: &Self) -> bool {
        self.SecurityFlags == other.SecurityFlags
            && self.IdType == other.IdType
            && self.IdLengthInBytes == other.IdLengthInBytes
            && self.Id == other.Id
            && self.KeyLengthInBytes == other.KeyLengthInBytes
            && self.Key == other.Key
    }
}
impl ::std::cmp::Eq for IKE_AUTHENTICATION_PRESHARED_KEY {}
unsafe impl ::windows::runtime::Abi for IKE_AUTHENTICATION_PRESHARED_KEY {
    type Abi = Self;
    type DefaultType = Self;
}
pub const IOCTL_ATA_MINIPORT: u32 = 315444u32;
pub const IOCTL_ATA_PASS_THROUGH: u32 = 315436u32;
pub const IOCTL_ATA_PASS_THROUGH_DIRECT: u32 = 315440u32;
pub const IOCTL_IDE_PASS_THROUGH: u32 = 315432u32;
pub const IOCTL_MINIPORT_PROCESS_SERVICE_IRP: u32 = 315448u32;
pub const IOCTL_MPIO_PASS_THROUGH_PATH: u32 = 315452u32;
pub const IOCTL_MPIO_PASS_THROUGH_PATH_DIRECT: u32 = 315456u32;
pub const IOCTL_MPIO_PASS_THROUGH_PATH_DIRECT_EX: u32 = 315472u32;
pub const IOCTL_MPIO_PASS_THROUGH_PATH_EX: u32 = 315468u32;
pub const IOCTL_SCSI_BASE: u32 = 4u32;
pub const IOCTL_SCSI_FREE_DUMP_POINTERS: u32 = 266276u32;
pub const IOCTL_SCSI_GET_ADDRESS: u32 = 266264u32;
pub const IOCTL_SCSI_GET_CAPABILITIES: u32 = 266256u32;
pub const IOCTL_SCSI_GET_DUMP_POINTERS: u32 = 266272u32;
pub const IOCTL_SCSI_GET_INQUIRY_DATA: u32 = 266252u32;
pub const IOCTL_SCSI_MINIPORT: u32 = 315400u32;
pub const IOCTL_SCSI_PASS_THROUGH: u32 = 315396u32;
pub const IOCTL_SCSI_PASS_THROUGH_DIRECT: u32 = 315412u32;
pub const IOCTL_SCSI_PASS_THROUGH_DIRECT_EX: u32 = 315464u32;
pub const IOCTL_SCSI_PASS_THROUGH_EX: u32 = 315460u32;
pub const IOCTL_SCSI_RESCAN_BUS: u32 = 266268u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IO_SCSI_CAPABILITIES {
    pub Length: u32,
    pub MaximumTransferLength: u32,
    pub MaximumPhysicalPages: u32,
    pub SupportedAsynchronousEvents: u32,
    pub AlignmentMask: u32,
    pub TaggedQueuing: super::super::Foundation::BOOLEAN,
    pub AdapterScansDown: super::super::Foundation::BOOLEAN,
    pub AdapterUsesPio: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl IO_SCSI_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IO_SCSI_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for IO_SCSI_CAPABILITIES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IO_SCSI_CAPABILITIES")
            .field("Length", &self.Length)
            .field("MaximumTransferLength", &self.MaximumTransferLength)
            .field("MaximumPhysicalPages", &self.MaximumPhysicalPages)
            .field(
                "SupportedAsynchronousEvents",
                &self.SupportedAsynchronousEvents,
            )
            .field("AlignmentMask", &self.AlignmentMask)
            .field("TaggedQueuing", &self.TaggedQueuing)
            .field("AdapterScansDown", &self.AdapterScansDown)
            .field("AdapterUsesPio", &self.AdapterUsesPio)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IO_SCSI_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length
            && self.MaximumTransferLength == other.MaximumTransferLength
            && self.MaximumPhysicalPages == other.MaximumPhysicalPages
            && self.SupportedAsynchronousEvents == other.SupportedAsynchronousEvents
            && self.AlignmentMask == other.AlignmentMask
            && self.TaggedQueuing == other.TaggedQueuing
            && self.AdapterScansDown == other.AdapterScansDown
            && self.AdapterUsesPio == other.AdapterUsesPio
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IO_SCSI_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IO_SCSI_CAPABILITIES {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ISCSI_AUTH_TYPES(pub i32);
pub const ISCSI_NO_AUTH_TYPE: ISCSI_AUTH_TYPES = ISCSI_AUTH_TYPES(0i32);
pub const ISCSI_CHAP_AUTH_TYPE: ISCSI_AUTH_TYPES = ISCSI_AUTH_TYPES(1i32);
pub const ISCSI_MUTUAL_CHAP_AUTH_TYPE: ISCSI_AUTH_TYPES = ISCSI_AUTH_TYPES(2i32);
impl ::std::convert::From<i32> for ISCSI_AUTH_TYPES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ISCSI_AUTH_TYPES {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct ISCSI_CONNECTION_INFOA {
    pub ConnectionId: ISCSI_UNIQUE_SESSION_ID,
    pub InitiatorAddress: super::super::Foundation::PSTR,
    pub TargetAddress: super::super::Foundation::PSTR,
    pub InitiatorSocket: u16,
    pub TargetSocket: u16,
    pub CID: [u8; 2],
}
#[cfg(feature = "Win32_Foundation")]
impl ISCSI_CONNECTION_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ISCSI_CONNECTION_INFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for ISCSI_CONNECTION_INFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ISCSI_CONNECTION_INFOA")
            .field("ConnectionId", &self.ConnectionId)
            .field("InitiatorAddress", &self.InitiatorAddress)
            .field("TargetAddress", &self.TargetAddress)
            .field("InitiatorSocket", &self.InitiatorSocket)
            .field("TargetSocket", &self.TargetSocket)
            .field("CID", &self.CID)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ISCSI_CONNECTION_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.ConnectionId == other.ConnectionId
            && self.InitiatorAddress == other.InitiatorAddress
            && self.TargetAddress == other.TargetAddress
            && self.InitiatorSocket == other.InitiatorSocket
            && self.TargetSocket == other.TargetSocket
            && self.CID == other.CID
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ISCSI_CONNECTION_INFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ISCSI_CONNECTION_INFOA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct ISCSI_CONNECTION_INFOW {
    pub ConnectionId: ISCSI_UNIQUE_SESSION_ID,
    pub InitiatorAddress: super::super::Foundation::PWSTR,
    pub TargetAddress: super::super::Foundation::PWSTR,
    pub InitiatorSocket: u16,
    pub TargetSocket: u16,
    pub CID: [u8; 2],
}
#[cfg(feature = "Win32_Foundation")]
impl ISCSI_CONNECTION_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ISCSI_CONNECTION_INFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for ISCSI_CONNECTION_INFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ISCSI_CONNECTION_INFOW")
            .field("ConnectionId", &self.ConnectionId)
            .field("InitiatorAddress", &self.InitiatorAddress)
            .field("TargetAddress", &self.TargetAddress)
            .field("InitiatorSocket", &self.InitiatorSocket)
            .field("TargetSocket", &self.TargetSocket)
            .field("CID", &self.CID)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ISCSI_CONNECTION_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.ConnectionId == other.ConnectionId
            && self.InitiatorAddress == other.InitiatorAddress
            && self.TargetAddress == other.TargetAddress
            && self.InitiatorSocket == other.InitiatorSocket
            && self.TargetSocket == other.TargetSocket
            && self.CID == other.CID
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ISCSI_CONNECTION_INFOW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ISCSI_CONNECTION_INFOW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ISCSI_CONNECTION_INFO_EX {
    pub ConnectionId: ISCSI_UNIQUE_SESSION_ID,
    pub State: u8,
    pub Protocol: u8,
    pub HeaderDigest: u8,
    pub DataDigest: u8,
    pub MaxRecvDataSegmentLength: u32,
    pub AuthType: ISCSI_AUTH_TYPES,
    pub EstimatedThroughput: u64,
    pub MaxDatagramSize: u32,
}
impl ISCSI_CONNECTION_INFO_EX {}
impl ::std::default::Default for ISCSI_CONNECTION_INFO_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ISCSI_CONNECTION_INFO_EX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ISCSI_CONNECTION_INFO_EX")
            .field("ConnectionId", &self.ConnectionId)
            .field("State", &self.State)
            .field("Protocol", &self.Protocol)
            .field("HeaderDigest", &self.HeaderDigest)
            .field("DataDigest", &self.DataDigest)
            .field("MaxRecvDataSegmentLength", &self.MaxRecvDataSegmentLength)
            .field("AuthType", &self.AuthType)
            .field("EstimatedThroughput", &self.EstimatedThroughput)
            .field("MaxDatagramSize", &self.MaxDatagramSize)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ISCSI_CONNECTION_INFO_EX {
    fn eq(&self, other: &Self) -> bool {
        self.ConnectionId == other.ConnectionId
            && self.State == other.State
            && self.Protocol == other.Protocol
            && self.HeaderDigest == other.HeaderDigest
            && self.DataDigest == other.DataDigest
            && self.MaxRecvDataSegmentLength == other.MaxRecvDataSegmentLength
            && self.AuthType == other.AuthType
            && self.EstimatedThroughput == other.EstimatedThroughput
            && self.MaxDatagramSize == other.MaxDatagramSize
    }
}
impl ::std::cmp::Eq for ISCSI_CONNECTION_INFO_EX {}
unsafe impl ::windows::runtime::Abi for ISCSI_CONNECTION_INFO_EX {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct ISCSI_DEVICE_ON_SESSIONA {
    pub InitiatorName: [super::super::System::SystemServices::CHAR; 256],
    pub TargetName: [super::super::System::SystemServices::CHAR; 224],
    pub ScsiAddress: SCSI_ADDRESS,
    pub DeviceInterfaceType: ::windows::runtime::GUID,
    pub DeviceInterfaceName: [super::super::System::SystemServices::CHAR; 260],
    pub LegacyName: [super::super::System::SystemServices::CHAR; 260],
    pub StorageDeviceNumber: super::super::System::SystemServices::STORAGE_DEVICE_NUMBER,
    pub DeviceInstance: u32,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ISCSI_DEVICE_ON_SESSIONA {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for ISCSI_DEVICE_ON_SESSIONA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::fmt::Debug for ISCSI_DEVICE_ON_SESSIONA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ISCSI_DEVICE_ON_SESSIONA")
            .field("InitiatorName", &self.InitiatorName)
            .field("TargetName", &self.TargetName)
            .field("ScsiAddress", &self.ScsiAddress)
            .field("DeviceInterfaceType", &self.DeviceInterfaceType)
            .field("DeviceInterfaceName", &self.DeviceInterfaceName)
            .field("LegacyName", &self.LegacyName)
            .field("StorageDeviceNumber", &self.StorageDeviceNumber)
            .field("DeviceInstance", &self.DeviceInstance)
            .finish()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for ISCSI_DEVICE_ON_SESSIONA {
    fn eq(&self, other: &Self) -> bool {
        self.InitiatorName == other.InitiatorName
            && self.TargetName == other.TargetName
            && self.ScsiAddress == other.ScsiAddress
            && self.DeviceInterfaceType == other.DeviceInterfaceType
            && self.DeviceInterfaceName == other.DeviceInterfaceName
            && self.LegacyName == other.LegacyName
            && self.StorageDeviceNumber == other.StorageDeviceNumber
            && self.DeviceInstance == other.DeviceInstance
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for ISCSI_DEVICE_ON_SESSIONA {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for ISCSI_DEVICE_ON_SESSIONA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct ISCSI_DEVICE_ON_SESSIONW {
    pub InitiatorName: [u16; 256],
    pub TargetName: [u16; 224],
    pub ScsiAddress: SCSI_ADDRESS,
    pub DeviceInterfaceType: ::windows::runtime::GUID,
    pub DeviceInterfaceName: [u16; 260],
    pub LegacyName: [u16; 260],
    pub StorageDeviceNumber: super::super::System::SystemServices::STORAGE_DEVICE_NUMBER,
    pub DeviceInstance: u32,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ISCSI_DEVICE_ON_SESSIONW {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for ISCSI_DEVICE_ON_SESSIONW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::fmt::Debug for ISCSI_DEVICE_ON_SESSIONW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ISCSI_DEVICE_ON_SESSIONW")
            .field("InitiatorName", &self.InitiatorName)
            .field("TargetName", &self.TargetName)
            .field("ScsiAddress", &self.ScsiAddress)
            .field("DeviceInterfaceType", &self.DeviceInterfaceType)
            .field("DeviceInterfaceName", &self.DeviceInterfaceName)
            .field("LegacyName", &self.LegacyName)
            .field("StorageDeviceNumber", &self.StorageDeviceNumber)
            .field("DeviceInstance", &self.DeviceInstance)
            .finish()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for ISCSI_DEVICE_ON_SESSIONW {
    fn eq(&self, other: &Self) -> bool {
        self.InitiatorName == other.InitiatorName
            && self.TargetName == other.TargetName
            && self.ScsiAddress == other.ScsiAddress
            && self.DeviceInterfaceType == other.DeviceInterfaceType
            && self.DeviceInterfaceName == other.DeviceInterfaceName
            && self.LegacyName == other.LegacyName
            && self.StorageDeviceNumber == other.StorageDeviceNumber
            && self.DeviceInstance == other.DeviceInstance
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for ISCSI_DEVICE_ON_SESSIONW {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for ISCSI_DEVICE_ON_SESSIONW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ISCSI_DIGEST_TYPES(pub i32);
pub const ISCSI_DIGEST_TYPE_NONE: ISCSI_DIGEST_TYPES = ISCSI_DIGEST_TYPES(0i32);
pub const ISCSI_DIGEST_TYPE_CRC32C: ISCSI_DIGEST_TYPES = ISCSI_DIGEST_TYPES(1i32);
impl ::std::convert::From<i32> for ISCSI_DIGEST_TYPES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ISCSI_DIGEST_TYPES {
    type Abi = Self;
    type DefaultType = Self;
}
pub const ISCSI_LOGIN_FLAG_ALLOW_PORTAL_HOPPING: u32 = 8u32;
pub const ISCSI_LOGIN_FLAG_MULTIPATH_ENABLED: u32 = 2u32;
pub const ISCSI_LOGIN_FLAG_REQUIRE_IPSEC: u32 = 1u32;
pub const ISCSI_LOGIN_FLAG_RESERVED1: u32 = 4u32;
pub const ISCSI_LOGIN_FLAG_USE_RADIUS_RESPONSE: u32 = 16u32;
pub const ISCSI_LOGIN_FLAG_USE_RADIUS_VERIFICATION: u32 = 32u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ISCSI_LOGIN_OPTIONS {
    pub Version: u32,
    pub InformationSpecified: u32,
    pub LoginFlags: u32,
    pub AuthType: ISCSI_AUTH_TYPES,
    pub HeaderDigest: ISCSI_DIGEST_TYPES,
    pub DataDigest: ISCSI_DIGEST_TYPES,
    pub MaximumConnections: u32,
    pub DefaultTime2Wait: u32,
    pub DefaultTime2Retain: u32,
    pub UsernameLength: u32,
    pub PasswordLength: u32,
    pub Username: *mut u8,
    pub Password: *mut u8,
}
impl ISCSI_LOGIN_OPTIONS {}
impl ::std::default::Default for ISCSI_LOGIN_OPTIONS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ISCSI_LOGIN_OPTIONS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ISCSI_LOGIN_OPTIONS")
            .field("Version", &self.Version)
            .field("InformationSpecified", &self.InformationSpecified)
            .field("LoginFlags", &self.LoginFlags)
            .field("AuthType", &self.AuthType)
            .field("HeaderDigest", &self.HeaderDigest)
            .field("DataDigest", &self.DataDigest)
            .field("MaximumConnections", &self.MaximumConnections)
            .field("DefaultTime2Wait", &self.DefaultTime2Wait)
            .field("DefaultTime2Retain", &self.DefaultTime2Retain)
            .field("UsernameLength", &self.UsernameLength)
            .field("PasswordLength", &self.PasswordLength)
            .field("Username", &self.Username)
            .field("Password", &self.Password)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ISCSI_LOGIN_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.InformationSpecified == other.InformationSpecified
            && self.LoginFlags == other.LoginFlags
            && self.AuthType == other.AuthType
            && self.HeaderDigest == other.HeaderDigest
            && self.DataDigest == other.DataDigest
            && self.MaximumConnections == other.MaximumConnections
            && self.DefaultTime2Wait == other.DefaultTime2Wait
            && self.DefaultTime2Retain == other.DefaultTime2Retain
            && self.UsernameLength == other.UsernameLength
            && self.PasswordLength == other.PasswordLength
            && self.Username == other.Username
            && self.Password == other.Password
    }
}
impl ::std::cmp::Eq for ISCSI_LOGIN_OPTIONS {}
unsafe impl ::windows::runtime::Abi for ISCSI_LOGIN_OPTIONS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const ISCSI_LOGIN_OPTIONS_AUTH_TYPE: u32 = 128u32;
pub const ISCSI_LOGIN_OPTIONS_DATA_DIGEST: u32 = 2u32;
pub const ISCSI_LOGIN_OPTIONS_DEFAULT_TIME_2_RETAIN: u32 = 16u32;
pub const ISCSI_LOGIN_OPTIONS_DEFAULT_TIME_2_WAIT: u32 = 8u32;
pub const ISCSI_LOGIN_OPTIONS_HEADER_DIGEST: u32 = 1u32;
pub const ISCSI_LOGIN_OPTIONS_MAXIMUM_CONNECTIONS: u32 = 4u32;
pub const ISCSI_LOGIN_OPTIONS_PASSWORD: u32 = 64u32;
pub const ISCSI_LOGIN_OPTIONS_USERNAME: u32 = 32u32;
pub const ISCSI_LOGIN_OPTIONS_VERSION: u32 = 0u32;
pub const ISCSI_SECURITY_FLAG_AGGRESSIVE_MODE_ENABLED: u32 = 8u32;
pub const ISCSI_SECURITY_FLAG_IKE_IPSEC_ENABLED: u32 = 2u32;
pub const ISCSI_SECURITY_FLAG_MAIN_MODE_ENABLED: u32 = 4u32;
pub const ISCSI_SECURITY_FLAG_PFS_ENABLED: u32 = 16u32;
pub const ISCSI_SECURITY_FLAG_TRANSPORT_MODE_PREFERRED: u32 = 32u32;
pub const ISCSI_SECURITY_FLAG_TUNNEL_MODE_PREFERRED: u32 = 64u32;
pub const ISCSI_SECURITY_FLAG_VALID: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct ISCSI_SESSION_INFOA {
    pub SessionId: ISCSI_UNIQUE_SESSION_ID,
    pub InitiatorName: super::super::Foundation::PSTR,
    pub TargetNodeName: super::super::Foundation::PSTR,
    pub TargetName: super::super::Foundation::PSTR,
    pub ISID: [u8; 6],
    pub TSID: [u8; 2],
    pub ConnectionCount: u32,
    pub Connections: *mut ISCSI_CONNECTION_INFOA,
}
#[cfg(feature = "Win32_Foundation")]
impl ISCSI_SESSION_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ISCSI_SESSION_INFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for ISCSI_SESSION_INFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ISCSI_SESSION_INFOA")
            .field("SessionId", &self.SessionId)
            .field("InitiatorName", &self.InitiatorName)
            .field("TargetNodeName", &self.TargetNodeName)
            .field("TargetName", &self.TargetName)
            .field("ISID", &self.ISID)
            .field("TSID", &self.TSID)
            .field("ConnectionCount", &self.ConnectionCount)
            .field("Connections", &self.Connections)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ISCSI_SESSION_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.SessionId == other.SessionId
            && self.InitiatorName == other.InitiatorName
            && self.TargetNodeName == other.TargetNodeName
            && self.TargetName == other.TargetName
            && self.ISID == other.ISID
            && self.TSID == other.TSID
            && self.ConnectionCount == other.ConnectionCount
            && self.Connections == other.Connections
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ISCSI_SESSION_INFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ISCSI_SESSION_INFOA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct ISCSI_SESSION_INFOW {
    pub SessionId: ISCSI_UNIQUE_SESSION_ID,
    pub InitiatorName: super::super::Foundation::PWSTR,
    pub TargetNodeName: super::super::Foundation::PWSTR,
    pub TargetName: super::super::Foundation::PWSTR,
    pub ISID: [u8; 6],
    pub TSID: [u8; 2],
    pub ConnectionCount: u32,
    pub Connections: *mut ISCSI_CONNECTION_INFOW,
}
#[cfg(feature = "Win32_Foundation")]
impl ISCSI_SESSION_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ISCSI_SESSION_INFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for ISCSI_SESSION_INFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ISCSI_SESSION_INFOW")
            .field("SessionId", &self.SessionId)
            .field("InitiatorName", &self.InitiatorName)
            .field("TargetNodeName", &self.TargetNodeName)
            .field("TargetName", &self.TargetName)
            .field("ISID", &self.ISID)
            .field("TSID", &self.TSID)
            .field("ConnectionCount", &self.ConnectionCount)
            .field("Connections", &self.Connections)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ISCSI_SESSION_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.SessionId == other.SessionId
            && self.InitiatorName == other.InitiatorName
            && self.TargetNodeName == other.TargetNodeName
            && self.TargetName == other.TargetName
            && self.ISID == other.ISID
            && self.TSID == other.TSID
            && self.ConnectionCount == other.ConnectionCount
            && self.Connections == other.Connections
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ISCSI_SESSION_INFOW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ISCSI_SESSION_INFOW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct ISCSI_SESSION_INFO_EX {
    pub SessionId: ISCSI_UNIQUE_SESSION_ID,
    pub InitialR2t: super::super::Foundation::BOOLEAN,
    pub ImmediateData: super::super::Foundation::BOOLEAN,
    pub Type: u8,
    pub DataSequenceInOrder: super::super::Foundation::BOOLEAN,
    pub DataPduInOrder: super::super::Foundation::BOOLEAN,
    pub ErrorRecoveryLevel: u8,
    pub MaxOutstandingR2t: u32,
    pub FirstBurstLength: u32,
    pub MaxBurstLength: u32,
    pub MaximumConnections: u32,
    pub ConnectionCount: u32,
    pub Connections: *mut ISCSI_CONNECTION_INFO_EX,
}
#[cfg(feature = "Win32_Foundation")]
impl ISCSI_SESSION_INFO_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ISCSI_SESSION_INFO_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for ISCSI_SESSION_INFO_EX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ISCSI_SESSION_INFO_EX")
            .field("SessionId", &self.SessionId)
            .field("InitialR2t", &self.InitialR2t)
            .field("ImmediateData", &self.ImmediateData)
            .field("Type", &self.Type)
            .field("DataSequenceInOrder", &self.DataSequenceInOrder)
            .field("DataPduInOrder", &self.DataPduInOrder)
            .field("ErrorRecoveryLevel", &self.ErrorRecoveryLevel)
            .field("MaxOutstandingR2t", &self.MaxOutstandingR2t)
            .field("FirstBurstLength", &self.FirstBurstLength)
            .field("MaxBurstLength", &self.MaxBurstLength)
            .field("MaximumConnections", &self.MaximumConnections)
            .field("ConnectionCount", &self.ConnectionCount)
            .field("Connections", &self.Connections)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ISCSI_SESSION_INFO_EX {
    fn eq(&self, other: &Self) -> bool {
        self.SessionId == other.SessionId
            && self.InitialR2t == other.InitialR2t
            && self.ImmediateData == other.ImmediateData
            && self.Type == other.Type
            && self.DataSequenceInOrder == other.DataSequenceInOrder
            && self.DataPduInOrder == other.DataPduInOrder
            && self.ErrorRecoveryLevel == other.ErrorRecoveryLevel
            && self.MaxOutstandingR2t == other.MaxOutstandingR2t
            && self.FirstBurstLength == other.FirstBurstLength
            && self.MaxBurstLength == other.MaxBurstLength
            && self.MaximumConnections == other.MaximumConnections
            && self.ConnectionCount == other.ConnectionCount
            && self.Connections == other.Connections
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ISCSI_SESSION_INFO_EX {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ISCSI_SESSION_INFO_EX {
    type Abi = Self;
    type DefaultType = Self;
}
pub const ISCSI_TARGET_FLAG_HIDE_STATIC_TARGET: u32 = 2u32;
pub const ISCSI_TARGET_FLAG_MERGE_TARGET_INFORMATION: u32 = 4u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct ISCSI_TARGET_MAPPINGA {
    pub InitiatorName: [super::super::System::SystemServices::CHAR; 256],
    pub TargetName: [super::super::System::SystemServices::CHAR; 224],
    pub OSDeviceName: [super::super::System::SystemServices::CHAR; 260],
    pub SessionId: ISCSI_UNIQUE_SESSION_ID,
    pub OSBusNumber: u32,
    pub OSTargetNumber: u32,
    pub LUNCount: u32,
    pub LUNList: *mut SCSI_LUN_LIST,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ISCSI_TARGET_MAPPINGA {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for ISCSI_TARGET_MAPPINGA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::fmt::Debug for ISCSI_TARGET_MAPPINGA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ISCSI_TARGET_MAPPINGA")
            .field("InitiatorName", &self.InitiatorName)
            .field("TargetName", &self.TargetName)
            .field("OSDeviceName", &self.OSDeviceName)
            .field("SessionId", &self.SessionId)
            .field("OSBusNumber", &self.OSBusNumber)
            .field("OSTargetNumber", &self.OSTargetNumber)
            .field("LUNCount", &self.LUNCount)
            .field("LUNList", &self.LUNList)
            .finish()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for ISCSI_TARGET_MAPPINGA {
    fn eq(&self, other: &Self) -> bool {
        self.InitiatorName == other.InitiatorName
            && self.TargetName == other.TargetName
            && self.OSDeviceName == other.OSDeviceName
            && self.SessionId == other.SessionId
            && self.OSBusNumber == other.OSBusNumber
            && self.OSTargetNumber == other.OSTargetNumber
            && self.LUNCount == other.LUNCount
            && self.LUNList == other.LUNList
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for ISCSI_TARGET_MAPPINGA {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for ISCSI_TARGET_MAPPINGA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ISCSI_TARGET_MAPPINGW {
    pub InitiatorName: [u16; 256],
    pub TargetName: [u16; 224],
    pub OSDeviceName: [u16; 260],
    pub SessionId: ISCSI_UNIQUE_SESSION_ID,
    pub OSBusNumber: u32,
    pub OSTargetNumber: u32,
    pub LUNCount: u32,
    pub LUNList: *mut SCSI_LUN_LIST,
}
impl ISCSI_TARGET_MAPPINGW {}
impl ::std::default::Default for ISCSI_TARGET_MAPPINGW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ISCSI_TARGET_MAPPINGW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ISCSI_TARGET_MAPPINGW")
            .field("InitiatorName", &self.InitiatorName)
            .field("TargetName", &self.TargetName)
            .field("OSDeviceName", &self.OSDeviceName)
            .field("SessionId", &self.SessionId)
            .field("OSBusNumber", &self.OSBusNumber)
            .field("OSTargetNumber", &self.OSTargetNumber)
            .field("LUNCount", &self.LUNCount)
            .field("LUNList", &self.LUNList)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ISCSI_TARGET_MAPPINGW {
    fn eq(&self, other: &Self) -> bool {
        self.InitiatorName == other.InitiatorName
            && self.TargetName == other.TargetName
            && self.OSDeviceName == other.OSDeviceName
            && self.SessionId == other.SessionId
            && self.OSBusNumber == other.OSBusNumber
            && self.OSTargetNumber == other.OSTargetNumber
            && self.LUNCount == other.LUNCount
            && self.LUNList == other.LUNList
    }
}
impl ::std::cmp::Eq for ISCSI_TARGET_MAPPINGW {}
unsafe impl ::windows::runtime::Abi for ISCSI_TARGET_MAPPINGW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct ISCSI_TARGET_PORTALA {
    pub SymbolicName: [super::super::System::SystemServices::CHAR; 256],
    pub Address: [super::super::System::SystemServices::CHAR; 256],
    pub Socket: u16,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ISCSI_TARGET_PORTALA {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for ISCSI_TARGET_PORTALA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::fmt::Debug for ISCSI_TARGET_PORTALA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ISCSI_TARGET_PORTALA")
            .field("SymbolicName", &self.SymbolicName)
            .field("Address", &self.Address)
            .field("Socket", &self.Socket)
            .finish()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for ISCSI_TARGET_PORTALA {
    fn eq(&self, other: &Self) -> bool {
        self.SymbolicName == other.SymbolicName
            && self.Address == other.Address
            && self.Socket == other.Socket
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for ISCSI_TARGET_PORTALA {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for ISCSI_TARGET_PORTALA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ISCSI_TARGET_PORTALW {
    pub SymbolicName: [u16; 256],
    pub Address: [u16; 256],
    pub Socket: u16,
}
impl ISCSI_TARGET_PORTALW {}
impl ::std::default::Default for ISCSI_TARGET_PORTALW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ISCSI_TARGET_PORTALW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ISCSI_TARGET_PORTALW")
            .field("SymbolicName", &self.SymbolicName)
            .field("Address", &self.Address)
            .field("Socket", &self.Socket)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ISCSI_TARGET_PORTALW {
    fn eq(&self, other: &Self) -> bool {
        self.SymbolicName == other.SymbolicName
            && self.Address == other.Address
            && self.Socket == other.Socket
    }
}
impl ::std::cmp::Eq for ISCSI_TARGET_PORTALW {}
unsafe impl ::windows::runtime::Abi for ISCSI_TARGET_PORTALW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct ISCSI_TARGET_PORTAL_GROUPA {
    pub Count: u32,
    pub Portals: [ISCSI_TARGET_PORTALA; 1],
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ISCSI_TARGET_PORTAL_GROUPA {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for ISCSI_TARGET_PORTAL_GROUPA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::fmt::Debug for ISCSI_TARGET_PORTAL_GROUPA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ISCSI_TARGET_PORTAL_GROUPA")
            .field("Count", &self.Count)
            .field("Portals", &self.Portals)
            .finish()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for ISCSI_TARGET_PORTAL_GROUPA {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Portals == other.Portals
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for ISCSI_TARGET_PORTAL_GROUPA {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for ISCSI_TARGET_PORTAL_GROUPA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ISCSI_TARGET_PORTAL_GROUPW {
    pub Count: u32,
    pub Portals: [ISCSI_TARGET_PORTALW; 1],
}
impl ISCSI_TARGET_PORTAL_GROUPW {}
impl ::std::default::Default for ISCSI_TARGET_PORTAL_GROUPW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ISCSI_TARGET_PORTAL_GROUPW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ISCSI_TARGET_PORTAL_GROUPW")
            .field("Count", &self.Count)
            .field("Portals", &self.Portals)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ISCSI_TARGET_PORTAL_GROUPW {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Portals == other.Portals
    }
}
impl ::std::cmp::Eq for ISCSI_TARGET_PORTAL_GROUPW {}
unsafe impl ::windows::runtime::Abi for ISCSI_TARGET_PORTAL_GROUPW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct ISCSI_TARGET_PORTAL_INFOA {
    pub InitiatorName: [super::super::System::SystemServices::CHAR; 256],
    pub InitiatorPortNumber: u32,
    pub SymbolicName: [super::super::System::SystemServices::CHAR; 256],
    pub Address: [super::super::System::SystemServices::CHAR; 256],
    pub Socket: u16,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ISCSI_TARGET_PORTAL_INFOA {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for ISCSI_TARGET_PORTAL_INFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::fmt::Debug for ISCSI_TARGET_PORTAL_INFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ISCSI_TARGET_PORTAL_INFOA")
            .field("InitiatorName", &self.InitiatorName)
            .field("InitiatorPortNumber", &self.InitiatorPortNumber)
            .field("SymbolicName", &self.SymbolicName)
            .field("Address", &self.Address)
            .field("Socket", &self.Socket)
            .finish()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for ISCSI_TARGET_PORTAL_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.InitiatorName == other.InitiatorName
            && self.InitiatorPortNumber == other.InitiatorPortNumber
            && self.SymbolicName == other.SymbolicName
            && self.Address == other.Address
            && self.Socket == other.Socket
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for ISCSI_TARGET_PORTAL_INFOA {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for ISCSI_TARGET_PORTAL_INFOA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ISCSI_TARGET_PORTAL_INFOW {
    pub InitiatorName: [u16; 256],
    pub InitiatorPortNumber: u32,
    pub SymbolicName: [u16; 256],
    pub Address: [u16; 256],
    pub Socket: u16,
}
impl ISCSI_TARGET_PORTAL_INFOW {}
impl ::std::default::Default for ISCSI_TARGET_PORTAL_INFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ISCSI_TARGET_PORTAL_INFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ISCSI_TARGET_PORTAL_INFOW")
            .field("InitiatorName", &self.InitiatorName)
            .field("InitiatorPortNumber", &self.InitiatorPortNumber)
            .field("SymbolicName", &self.SymbolicName)
            .field("Address", &self.Address)
            .field("Socket", &self.Socket)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ISCSI_TARGET_PORTAL_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.InitiatorName == other.InitiatorName
            && self.InitiatorPortNumber == other.InitiatorPortNumber
            && self.SymbolicName == other.SymbolicName
            && self.Address == other.Address
            && self.Socket == other.Socket
    }
}
impl ::std::cmp::Eq for ISCSI_TARGET_PORTAL_INFOW {}
unsafe impl ::windows::runtime::Abi for ISCSI_TARGET_PORTAL_INFOW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct ISCSI_TARGET_PORTAL_INFO_EXA {
    pub InitiatorName: [super::super::System::SystemServices::CHAR; 256],
    pub InitiatorPortNumber: u32,
    pub SymbolicName: [super::super::System::SystemServices::CHAR; 256],
    pub Address: [super::super::System::SystemServices::CHAR; 256],
    pub Socket: u16,
    pub SecurityFlags: u64,
    pub LoginOptions: ISCSI_LOGIN_OPTIONS,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ISCSI_TARGET_PORTAL_INFO_EXA {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for ISCSI_TARGET_PORTAL_INFO_EXA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::fmt::Debug for ISCSI_TARGET_PORTAL_INFO_EXA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ISCSI_TARGET_PORTAL_INFO_EXA")
            .field("InitiatorName", &self.InitiatorName)
            .field("InitiatorPortNumber", &self.InitiatorPortNumber)
            .field("SymbolicName", &self.SymbolicName)
            .field("Address", &self.Address)
            .field("Socket", &self.Socket)
            .field("SecurityFlags", &self.SecurityFlags)
            .field("LoginOptions", &self.LoginOptions)
            .finish()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for ISCSI_TARGET_PORTAL_INFO_EXA {
    fn eq(&self, other: &Self) -> bool {
        self.InitiatorName == other.InitiatorName
            && self.InitiatorPortNumber == other.InitiatorPortNumber
            && self.SymbolicName == other.SymbolicName
            && self.Address == other.Address
            && self.Socket == other.Socket
            && self.SecurityFlags == other.SecurityFlags
            && self.LoginOptions == other.LoginOptions
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for ISCSI_TARGET_PORTAL_INFO_EXA {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for ISCSI_TARGET_PORTAL_INFO_EXA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ISCSI_TARGET_PORTAL_INFO_EXW {
    pub InitiatorName: [u16; 256],
    pub InitiatorPortNumber: u32,
    pub SymbolicName: [u16; 256],
    pub Address: [u16; 256],
    pub Socket: u16,
    pub SecurityFlags: u64,
    pub LoginOptions: ISCSI_LOGIN_OPTIONS,
}
impl ISCSI_TARGET_PORTAL_INFO_EXW {}
impl ::std::default::Default for ISCSI_TARGET_PORTAL_INFO_EXW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ISCSI_TARGET_PORTAL_INFO_EXW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ISCSI_TARGET_PORTAL_INFO_EXW")
            .field("InitiatorName", &self.InitiatorName)
            .field("InitiatorPortNumber", &self.InitiatorPortNumber)
            .field("SymbolicName", &self.SymbolicName)
            .field("Address", &self.Address)
            .field("Socket", &self.Socket)
            .field("SecurityFlags", &self.SecurityFlags)
            .field("LoginOptions", &self.LoginOptions)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ISCSI_TARGET_PORTAL_INFO_EXW {
    fn eq(&self, other: &Self) -> bool {
        self.InitiatorName == other.InitiatorName
            && self.InitiatorPortNumber == other.InitiatorPortNumber
            && self.SymbolicName == other.SymbolicName
            && self.Address == other.Address
            && self.Socket == other.Socket
            && self.SecurityFlags == other.SecurityFlags
            && self.LoginOptions == other.LoginOptions
    }
}
impl ::std::cmp::Eq for ISCSI_TARGET_PORTAL_INFO_EXW {}
unsafe impl ::windows::runtime::Abi for ISCSI_TARGET_PORTAL_INFO_EXW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ISCSI_UNIQUE_SESSION_ID {
    pub AdapterUnique: u64,
    pub AdapterSpecific: u64,
}
impl ISCSI_UNIQUE_SESSION_ID {}
impl ::std::default::Default for ISCSI_UNIQUE_SESSION_ID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ISCSI_UNIQUE_SESSION_ID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ISCSI_UNIQUE_SESSION_ID")
            .field("AdapterUnique", &self.AdapterUnique)
            .field("AdapterSpecific", &self.AdapterSpecific)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ISCSI_UNIQUE_SESSION_ID {
    fn eq(&self, other: &Self) -> bool {
        self.AdapterUnique == other.AdapterUnique && self.AdapterSpecific == other.AdapterSpecific
    }
}
impl ::std::cmp::Eq for ISCSI_UNIQUE_SESSION_ID {}
unsafe impl ::windows::runtime::Abi for ISCSI_UNIQUE_SESSION_ID {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ISCSI_VERSION_INFO {
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub BuildNumber: u32,
}
impl ISCSI_VERSION_INFO {}
impl ::std::default::Default for ISCSI_VERSION_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ISCSI_VERSION_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ISCSI_VERSION_INFO")
            .field("MajorVersion", &self.MajorVersion)
            .field("MinorVersion", &self.MinorVersion)
            .field("BuildNumber", &self.BuildNumber)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ISCSI_VERSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion
            && self.MinorVersion == other.MinorVersion
            && self.BuildNumber == other.BuildNumber
    }
}
impl ::std::cmp::Eq for ISCSI_VERSION_INFO {}
unsafe impl ::windows::runtime::Abi for ISCSI_VERSION_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn LoginIScsiTargetA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param9: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param10: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>,
>(
    targetname: Param0,
    isinformationalsession: Param1,
    initiatorinstance: Param2,
    initiatorportnumber: u32,
    targetportal: *mut ISCSI_TARGET_PORTALA,
    securityflags: u64,
    mappings: *mut ISCSI_TARGET_MAPPINGA,
    loginoptions: *mut ISCSI_LOGIN_OPTIONS,
    keysize: u32,
    key: Param9,
    ispersistent: Param10,
    uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID,
    uniqueconnectionid: *mut ISCSI_UNIQUE_SESSION_ID,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn LoginIScsiTargetA(
                targetname: super::super::Foundation::PSTR,
                isinformationalsession: super::super::Foundation::BOOLEAN,
                initiatorinstance: super::super::Foundation::PSTR,
                initiatorportnumber: u32,
                targetportal: *mut ISCSI_TARGET_PORTALA,
                securityflags: u64,
                mappings: *mut ISCSI_TARGET_MAPPINGA,
                loginoptions: *mut ISCSI_LOGIN_OPTIONS,
                keysize: u32,
                key: super::super::Foundation::PSTR,
                ispersistent: super::super::Foundation::BOOLEAN,
                uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID,
                uniqueconnectionid: *mut ISCSI_UNIQUE_SESSION_ID,
            ) -> u32;
        }
        ::std::mem::transmute(LoginIScsiTargetA(
            targetname.into_param().abi(),
            isinformationalsession.into_param().abi(),
            initiatorinstance.into_param().abi(),
            ::std::mem::transmute(initiatorportnumber),
            ::std::mem::transmute(targetportal),
            ::std::mem::transmute(securityflags),
            ::std::mem::transmute(mappings),
            ::std::mem::transmute(loginoptions),
            ::std::mem::transmute(keysize),
            key.into_param().abi(),
            ispersistent.into_param().abi(),
            ::std::mem::transmute(uniquesessionid),
            ::std::mem::transmute(uniqueconnectionid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn LoginIScsiTargetW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param9: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param10: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>,
>(
    targetname: Param0,
    isinformationalsession: Param1,
    initiatorinstance: Param2,
    initiatorportnumber: u32,
    targetportal: *mut ISCSI_TARGET_PORTALW,
    securityflags: u64,
    mappings: *mut ISCSI_TARGET_MAPPINGW,
    loginoptions: *mut ISCSI_LOGIN_OPTIONS,
    keysize: u32,
    key: Param9,
    ispersistent: Param10,
    uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID,
    uniqueconnectionid: *mut ISCSI_UNIQUE_SESSION_ID,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn LoginIScsiTargetW(
                targetname: super::super::Foundation::PWSTR,
                isinformationalsession: super::super::Foundation::BOOLEAN,
                initiatorinstance: super::super::Foundation::PWSTR,
                initiatorportnumber: u32,
                targetportal: *mut ISCSI_TARGET_PORTALW,
                securityflags: u64,
                mappings: *mut ISCSI_TARGET_MAPPINGW,
                loginoptions: *mut ISCSI_LOGIN_OPTIONS,
                keysize: u32,
                key: super::super::Foundation::PSTR,
                ispersistent: super::super::Foundation::BOOLEAN,
                uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID,
                uniqueconnectionid: *mut ISCSI_UNIQUE_SESSION_ID,
            ) -> u32;
        }
        ::std::mem::transmute(LoginIScsiTargetW(
            targetname.into_param().abi(),
            isinformationalsession.into_param().abi(),
            initiatorinstance.into_param().abi(),
            ::std::mem::transmute(initiatorportnumber),
            ::std::mem::transmute(targetportal),
            ::std::mem::transmute(securityflags),
            ::std::mem::transmute(mappings),
            ::std::mem::transmute(loginoptions),
            ::std::mem::transmute(keysize),
            key.into_param().abi(),
            ispersistent.into_param().abi(),
            ::std::mem::transmute(uniquesessionid),
            ::std::mem::transmute(uniqueconnectionid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn LogoutIScsiTarget(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn LogoutIScsiTarget(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID) -> u32;
        }
        ::std::mem::transmute(LogoutIScsiTarget(::std::mem::transmute(uniquesessionid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const MAX_ISCSI_ALIAS_LEN: u32 = 255u32;
pub const MAX_ISCSI_DISCOVERY_DOMAIN_LEN: u32 = 256u32;
pub const MAX_ISCSI_HBANAME_LEN: u32 = 256u32;
pub const MAX_ISCSI_NAME_LEN: u32 = 223u32;
pub const MAX_ISCSI_PORTAL_ADDRESS_LEN: u32 = 256u32;
pub const MAX_ISCSI_PORTAL_ALIAS_LEN: u32 = 256u32;
pub const MAX_ISCSI_PORTAL_NAME_LEN: u32 = 256u32;
pub const MAX_ISCSI_TEXT_ADDRESS_LEN: u32 = 256u32;
pub const MAX_RADIUS_ADDRESS_LEN: u32 = 41u32;
pub const MINIPORT_DSM_NOTIFICATION_VERSION: u32 = 1u32;
pub const MINIPORT_DSM_NOTIFICATION_VERSION_1: u32 = 1u32;
pub const MINIPORT_DSM_NOTIFY_FLAG_BEGIN: u32 = 1u32;
pub const MINIPORT_DSM_NOTIFY_FLAG_END: u32 = 2u32;
pub const MINIPORT_DSM_PROFILE_CRASHDUMP_FILE: u32 = 3u32;
pub const MINIPORT_DSM_PROFILE_HIBERNATION_FILE: u32 = 2u32;
pub const MINIPORT_DSM_PROFILE_PAGE_FILE: u32 = 1u32;
pub const MINIPORT_DSM_PROFILE_UNKNOWN: u32 = 0u32;
pub const MPIO_IOCTL_FLAG_INVOLVE_DSM: u32 = 4u32;
pub const MPIO_IOCTL_FLAG_USE_PATHID: u32 = 1u32;
pub const MPIO_IOCTL_FLAG_USE_SCSIADDRESS: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MPIO_PASS_THROUGH_PATH {
    pub PassThrough: SCSI_PASS_THROUGH,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
impl MPIO_PASS_THROUGH_PATH {}
impl ::std::default::Default for MPIO_PASS_THROUGH_PATH {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MPIO_PASS_THROUGH_PATH {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MPIO_PASS_THROUGH_PATH")
            .field("PassThrough", &self.PassThrough)
            .field("Version", &self.Version)
            .field("Length", &self.Length)
            .field("Flags", &self.Flags)
            .field("PortNumber", &self.PortNumber)
            .field("MpioPathId", &self.MpioPathId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MPIO_PASS_THROUGH_PATH {
    fn eq(&self, other: &Self) -> bool {
        self.PassThrough == other.PassThrough
            && self.Version == other.Version
            && self.Length == other.Length
            && self.Flags == other.Flags
            && self.PortNumber == other.PortNumber
            && self.MpioPathId == other.MpioPathId
    }
}
impl ::std::cmp::Eq for MPIO_PASS_THROUGH_PATH {}
unsafe impl ::windows::runtime::Abi for MPIO_PASS_THROUGH_PATH {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MPIO_PASS_THROUGH_PATH32 {
    pub PassThrough: SCSI_PASS_THROUGH32,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
impl MPIO_PASS_THROUGH_PATH32 {}
impl ::std::default::Default for MPIO_PASS_THROUGH_PATH32 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MPIO_PASS_THROUGH_PATH32 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MPIO_PASS_THROUGH_PATH32")
            .field("PassThrough", &self.PassThrough)
            .field("Version", &self.Version)
            .field("Length", &self.Length)
            .field("Flags", &self.Flags)
            .field("PortNumber", &self.PortNumber)
            .field("MpioPathId", &self.MpioPathId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MPIO_PASS_THROUGH_PATH32 {
    fn eq(&self, other: &Self) -> bool {
        self.PassThrough == other.PassThrough
            && self.Version == other.Version
            && self.Length == other.Length
            && self.Flags == other.Flags
            && self.PortNumber == other.PortNumber
            && self.MpioPathId == other.MpioPathId
    }
}
impl ::std::cmp::Eq for MPIO_PASS_THROUGH_PATH32 {}
unsafe impl ::windows::runtime::Abi for MPIO_PASS_THROUGH_PATH32 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MPIO_PASS_THROUGH_PATH32_EX {
    pub PassThroughOffset: u32,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
impl MPIO_PASS_THROUGH_PATH32_EX {}
impl ::std::default::Default for MPIO_PASS_THROUGH_PATH32_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MPIO_PASS_THROUGH_PATH32_EX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MPIO_PASS_THROUGH_PATH32_EX")
            .field("PassThroughOffset", &self.PassThroughOffset)
            .field("Version", &self.Version)
            .field("Length", &self.Length)
            .field("Flags", &self.Flags)
            .field("PortNumber", &self.PortNumber)
            .field("MpioPathId", &self.MpioPathId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MPIO_PASS_THROUGH_PATH32_EX {
    fn eq(&self, other: &Self) -> bool {
        self.PassThroughOffset == other.PassThroughOffset
            && self.Version == other.Version
            && self.Length == other.Length
            && self.Flags == other.Flags
            && self.PortNumber == other.PortNumber
            && self.MpioPathId == other.MpioPathId
    }
}
impl ::std::cmp::Eq for MPIO_PASS_THROUGH_PATH32_EX {}
unsafe impl ::windows::runtime::Abi for MPIO_PASS_THROUGH_PATH32_EX {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MPIO_PASS_THROUGH_PATH_DIRECT {
    pub PassThrough: SCSI_PASS_THROUGH_DIRECT,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
impl MPIO_PASS_THROUGH_PATH_DIRECT {}
impl ::std::default::Default for MPIO_PASS_THROUGH_PATH_DIRECT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MPIO_PASS_THROUGH_PATH_DIRECT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MPIO_PASS_THROUGH_PATH_DIRECT")
            .field("PassThrough", &self.PassThrough)
            .field("Version", &self.Version)
            .field("Length", &self.Length)
            .field("Flags", &self.Flags)
            .field("PortNumber", &self.PortNumber)
            .field("MpioPathId", &self.MpioPathId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MPIO_PASS_THROUGH_PATH_DIRECT {
    fn eq(&self, other: &Self) -> bool {
        self.PassThrough == other.PassThrough
            && self.Version == other.Version
            && self.Length == other.Length
            && self.Flags == other.Flags
            && self.PortNumber == other.PortNumber
            && self.MpioPathId == other.MpioPathId
    }
}
impl ::std::cmp::Eq for MPIO_PASS_THROUGH_PATH_DIRECT {}
unsafe impl ::windows::runtime::Abi for MPIO_PASS_THROUGH_PATH_DIRECT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MPIO_PASS_THROUGH_PATH_DIRECT32 {
    pub PassThrough: SCSI_PASS_THROUGH_DIRECT32,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
impl MPIO_PASS_THROUGH_PATH_DIRECT32 {}
impl ::std::default::Default for MPIO_PASS_THROUGH_PATH_DIRECT32 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MPIO_PASS_THROUGH_PATH_DIRECT32 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MPIO_PASS_THROUGH_PATH_DIRECT32")
            .field("PassThrough", &self.PassThrough)
            .field("Version", &self.Version)
            .field("Length", &self.Length)
            .field("Flags", &self.Flags)
            .field("PortNumber", &self.PortNumber)
            .field("MpioPathId", &self.MpioPathId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MPIO_PASS_THROUGH_PATH_DIRECT32 {
    fn eq(&self, other: &Self) -> bool {
        self.PassThrough == other.PassThrough
            && self.Version == other.Version
            && self.Length == other.Length
            && self.Flags == other.Flags
            && self.PortNumber == other.PortNumber
            && self.MpioPathId == other.MpioPathId
    }
}
impl ::std::cmp::Eq for MPIO_PASS_THROUGH_PATH_DIRECT32 {}
unsafe impl ::windows::runtime::Abi for MPIO_PASS_THROUGH_PATH_DIRECT32 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MPIO_PASS_THROUGH_PATH_DIRECT32_EX {
    pub PassThroughOffset: u32,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
impl MPIO_PASS_THROUGH_PATH_DIRECT32_EX {}
impl ::std::default::Default for MPIO_PASS_THROUGH_PATH_DIRECT32_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MPIO_PASS_THROUGH_PATH_DIRECT32_EX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MPIO_PASS_THROUGH_PATH_DIRECT32_EX")
            .field("PassThroughOffset", &self.PassThroughOffset)
            .field("Version", &self.Version)
            .field("Length", &self.Length)
            .field("Flags", &self.Flags)
            .field("PortNumber", &self.PortNumber)
            .field("MpioPathId", &self.MpioPathId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MPIO_PASS_THROUGH_PATH_DIRECT32_EX {
    fn eq(&self, other: &Self) -> bool {
        self.PassThroughOffset == other.PassThroughOffset
            && self.Version == other.Version
            && self.Length == other.Length
            && self.Flags == other.Flags
            && self.PortNumber == other.PortNumber
            && self.MpioPathId == other.MpioPathId
    }
}
impl ::std::cmp::Eq for MPIO_PASS_THROUGH_PATH_DIRECT32_EX {}
unsafe impl ::windows::runtime::Abi for MPIO_PASS_THROUGH_PATH_DIRECT32_EX {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MPIO_PASS_THROUGH_PATH_DIRECT_EX {
    pub PassThroughOffset: u32,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
impl MPIO_PASS_THROUGH_PATH_DIRECT_EX {}
impl ::std::default::Default for MPIO_PASS_THROUGH_PATH_DIRECT_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MPIO_PASS_THROUGH_PATH_DIRECT_EX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MPIO_PASS_THROUGH_PATH_DIRECT_EX")
            .field("PassThroughOffset", &self.PassThroughOffset)
            .field("Version", &self.Version)
            .field("Length", &self.Length)
            .field("Flags", &self.Flags)
            .field("PortNumber", &self.PortNumber)
            .field("MpioPathId", &self.MpioPathId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MPIO_PASS_THROUGH_PATH_DIRECT_EX {
    fn eq(&self, other: &Self) -> bool {
        self.PassThroughOffset == other.PassThroughOffset
            && self.Version == other.Version
            && self.Length == other.Length
            && self.Flags == other.Flags
            && self.PortNumber == other.PortNumber
            && self.MpioPathId == other.MpioPathId
    }
}
impl ::std::cmp::Eq for MPIO_PASS_THROUGH_PATH_DIRECT_EX {}
unsafe impl ::windows::runtime::Abi for MPIO_PASS_THROUGH_PATH_DIRECT_EX {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MPIO_PASS_THROUGH_PATH_EX {
    pub PassThroughOffset: u32,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
impl MPIO_PASS_THROUGH_PATH_EX {}
impl ::std::default::Default for MPIO_PASS_THROUGH_PATH_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MPIO_PASS_THROUGH_PATH_EX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MPIO_PASS_THROUGH_PATH_EX")
            .field("PassThroughOffset", &self.PassThroughOffset)
            .field("Version", &self.Version)
            .field("Length", &self.Length)
            .field("Flags", &self.Flags)
            .field("PortNumber", &self.PortNumber)
            .field("MpioPathId", &self.MpioPathId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MPIO_PASS_THROUGH_PATH_EX {
    fn eq(&self, other: &Self) -> bool {
        self.PassThroughOffset == other.PassThroughOffset
            && self.Version == other.Version
            && self.Length == other.Length
            && self.Flags == other.Flags
            && self.PortNumber == other.PortNumber
            && self.MpioPathId == other.MpioPathId
    }
}
impl ::std::cmp::Eq for MPIO_PASS_THROUGH_PATH_EX {}
unsafe impl ::windows::runtime::Abi for MPIO_PASS_THROUGH_PATH_EX {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MP_DEVICE_DATA_SET_RANGE {
    pub StartingOffset: i64,
    pub LengthInBytes: u64,
}
impl MP_DEVICE_DATA_SET_RANGE {}
impl ::std::default::Default for MP_DEVICE_DATA_SET_RANGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MP_DEVICE_DATA_SET_RANGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MP_DEVICE_DATA_SET_RANGE")
            .field("StartingOffset", &self.StartingOffset)
            .field("LengthInBytes", &self.LengthInBytes)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MP_DEVICE_DATA_SET_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.StartingOffset == other.StartingOffset && self.LengthInBytes == other.LengthInBytes
    }
}
impl ::std::cmp::Eq for MP_DEVICE_DATA_SET_RANGE {}
unsafe impl ::windows::runtime::Abi for MP_DEVICE_DATA_SET_RANGE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct MP_STORAGE_DIAGNOSTIC_LEVEL(pub i32);
pub const MpStorageDiagnosticLevelDefault: MP_STORAGE_DIAGNOSTIC_LEVEL =
    MP_STORAGE_DIAGNOSTIC_LEVEL(0i32);
pub const MpStorageDiagnosticLevelMax: MP_STORAGE_DIAGNOSTIC_LEVEL =
    MP_STORAGE_DIAGNOSTIC_LEVEL(1i32);
impl ::std::convert::From<i32> for MP_STORAGE_DIAGNOSTIC_LEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MP_STORAGE_DIAGNOSTIC_LEVEL {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct MP_STORAGE_DIAGNOSTIC_TARGET_TYPE(pub i32);
pub const MpStorageDiagnosticTargetTypeUndefined: MP_STORAGE_DIAGNOSTIC_TARGET_TYPE =
    MP_STORAGE_DIAGNOSTIC_TARGET_TYPE(0i32);
pub const MpStorageDiagnosticTargetTypeMiniport: MP_STORAGE_DIAGNOSTIC_TARGET_TYPE =
    MP_STORAGE_DIAGNOSTIC_TARGET_TYPE(2i32);
pub const MpStorageDiagnosticTargetTypeHbaFirmware: MP_STORAGE_DIAGNOSTIC_TARGET_TYPE =
    MP_STORAGE_DIAGNOSTIC_TARGET_TYPE(3i32);
pub const MpStorageDiagnosticTargetTypeMax: MP_STORAGE_DIAGNOSTIC_TARGET_TYPE =
    MP_STORAGE_DIAGNOSTIC_TARGET_TYPE(4i32);
impl ::std::convert::From<i32> for MP_STORAGE_DIAGNOSTIC_TARGET_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MP_STORAGE_DIAGNOSTIC_TARGET_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const NRB_FUNCTION_ADD_LBAS_PINNED_SET: u32 = 16u32;
pub const NRB_FUNCTION_FLUSH_NVCACHE: u32 = 20u32;
pub const NRB_FUNCTION_NVCACHE_INFO: u32 = 236u32;
pub const NRB_FUNCTION_NVCACHE_POWER_MODE_RETURN: u32 = 1u32;
pub const NRB_FUNCTION_NVCACHE_POWER_MODE_SET: u32 = 0u32;
pub const NRB_FUNCTION_NVSEPARATED_FLUSH: u32 = 193u32;
pub const NRB_FUNCTION_NVSEPARATED_INFO: u32 = 192u32;
pub const NRB_FUNCTION_NVSEPARATED_WB_DISABLE: u32 = 194u32;
pub const NRB_FUNCTION_NVSEPARATED_WB_REVERT_DEFAULT: u32 = 195u32;
pub const NRB_FUNCTION_PASS_HINT_PAYLOAD: u32 = 224u32;
pub const NRB_FUNCTION_QUERY_ASCENDER_STATUS: u32 = 208u32;
pub const NRB_FUNCTION_QUERY_CACHE_MISS: u32 = 19u32;
pub const NRB_FUNCTION_QUERY_HYBRID_DISK_STATUS: u32 = 209u32;
pub const NRB_FUNCTION_QUERY_PINNED_SET: u32 = 18u32;
pub const NRB_FUNCTION_REMOVE_LBAS_PINNED_SET: u32 = 17u32;
pub const NRB_FUNCTION_SPINDLE_STATUS: u32 = 229u32;
pub const NRB_ILLEGAL_REQUEST: u32 = 1u32;
pub const NRB_INPUT_DATA_OVERRUN: u32 = 3u32;
pub const NRB_INPUT_DATA_UNDERRUN: u32 = 4u32;
pub const NRB_INVALID_PARAMETER: u32 = 2u32;
pub const NRB_OUTPUT_DATA_OVERRUN: u32 = 5u32;
pub const NRB_OUTPUT_DATA_UNDERRUN: u32 = 6u32;
pub const NRB_SUCCESS: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NTSCSI_UNICODE_STRING {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl NTSCSI_UNICODE_STRING {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NTSCSI_UNICODE_STRING {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NTSCSI_UNICODE_STRING {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NTSCSI_UNICODE_STRING")
            .field("Length", &self.Length)
            .field("MaximumLength", &self.MaximumLength)
            .field("Buffer", &self.Buffer)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NTSCSI_UNICODE_STRING {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length
            && self.MaximumLength == other.MaximumLength
            && self.Buffer == other.Buffer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NTSCSI_UNICODE_STRING {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NTSCSI_UNICODE_STRING {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct NVCACHE_HINT_PAYLOAD {
    pub Command: u8,
    pub Feature7_0: u8,
    pub Feature15_8: u8,
    pub Count15_8: u8,
    pub LBA7_0: u8,
    pub LBA15_8: u8,
    pub LBA23_16: u8,
    pub LBA31_24: u8,
    pub LBA39_32: u8,
    pub LBA47_40: u8,
    pub Auxiliary7_0: u8,
    pub Auxiliary23_16: u8,
    pub Reserved: [u8; 4],
}
impl NVCACHE_HINT_PAYLOAD {}
impl ::std::default::Default for NVCACHE_HINT_PAYLOAD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NVCACHE_HINT_PAYLOAD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NVCACHE_HINT_PAYLOAD")
            .field("Command", &self.Command)
            .field("Feature7_0", &self.Feature7_0)
            .field("Feature15_8", &self.Feature15_8)
            .field("Count15_8", &self.Count15_8)
            .field("LBA7_0", &self.LBA7_0)
            .field("LBA15_8", &self.LBA15_8)
            .field("LBA23_16", &self.LBA23_16)
            .field("LBA31_24", &self.LBA31_24)
            .field("LBA39_32", &self.LBA39_32)
            .field("LBA47_40", &self.LBA47_40)
            .field("Auxiliary7_0", &self.Auxiliary7_0)
            .field("Auxiliary23_16", &self.Auxiliary23_16)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NVCACHE_HINT_PAYLOAD {
    fn eq(&self, other: &Self) -> bool {
        self.Command == other.Command
            && self.Feature7_0 == other.Feature7_0
            && self.Feature15_8 == other.Feature15_8
            && self.Count15_8 == other.Count15_8
            && self.LBA7_0 == other.LBA7_0
            && self.LBA15_8 == other.LBA15_8
            && self.LBA23_16 == other.LBA23_16
            && self.LBA31_24 == other.LBA31_24
            && self.LBA39_32 == other.LBA39_32
            && self.LBA47_40 == other.LBA47_40
            && self.Auxiliary7_0 == other.Auxiliary7_0
            && self.Auxiliary23_16 == other.Auxiliary23_16
            && self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for NVCACHE_HINT_PAYLOAD {}
unsafe impl ::windows::runtime::Abi for NVCACHE_HINT_PAYLOAD {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct NVCACHE_PRIORITY_LEVEL_DESCRIPTOR {
    pub PriorityLevel: u8,
    pub Reserved0: [u8; 3],
    pub ConsumedNVMSizeFraction: u32,
    pub ConsumedMappingResourcesFraction: u32,
    pub ConsumedNVMSizeForDirtyDataFraction: u32,
    pub ConsumedMappingResourcesForDirtyDataFraction: u32,
    pub Reserved1: u32,
}
impl NVCACHE_PRIORITY_LEVEL_DESCRIPTOR {}
impl ::std::default::Default for NVCACHE_PRIORITY_LEVEL_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NVCACHE_PRIORITY_LEVEL_DESCRIPTOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NVCACHE_PRIORITY_LEVEL_DESCRIPTOR")
            .field("PriorityLevel", &self.PriorityLevel)
            .field("Reserved0", &self.Reserved0)
            .field("ConsumedNVMSizeFraction", &self.ConsumedNVMSizeFraction)
            .field(
                "ConsumedMappingResourcesFraction",
                &self.ConsumedMappingResourcesFraction,
            )
            .field(
                "ConsumedNVMSizeForDirtyDataFraction",
                &self.ConsumedNVMSizeForDirtyDataFraction,
            )
            .field(
                "ConsumedMappingResourcesForDirtyDataFraction",
                &self.ConsumedMappingResourcesForDirtyDataFraction,
            )
            .field("Reserved1", &self.Reserved1)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NVCACHE_PRIORITY_LEVEL_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.PriorityLevel == other.PriorityLevel
            && self.Reserved0 == other.Reserved0
            && self.ConsumedNVMSizeFraction == other.ConsumedNVMSizeFraction
            && self.ConsumedMappingResourcesFraction == other.ConsumedMappingResourcesFraction
            && self.ConsumedNVMSizeForDirtyDataFraction == other.ConsumedNVMSizeForDirtyDataFraction
            && self.ConsumedMappingResourcesForDirtyDataFraction
                == other.ConsumedMappingResourcesForDirtyDataFraction
            && self.Reserved1 == other.Reserved1
    }
}
impl ::std::cmp::Eq for NVCACHE_PRIORITY_LEVEL_DESCRIPTOR {}
unsafe impl ::windows::runtime::Abi for NVCACHE_PRIORITY_LEVEL_DESCRIPTOR {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct NVCACHE_REQUEST_BLOCK {
    pub NRBSize: u32,
    pub Function: u16,
    pub NRBFlags: u32,
    pub NRBStatus: u32,
    pub Count: u32,
    pub LBA: u64,
    pub DataBufSize: u32,
    pub NVCacheStatus: u32,
    pub NVCacheSubStatus: u32,
}
impl NVCACHE_REQUEST_BLOCK {}
impl ::std::default::Default for NVCACHE_REQUEST_BLOCK {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NVCACHE_REQUEST_BLOCK {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NVCACHE_REQUEST_BLOCK")
            .field("NRBSize", &self.NRBSize)
            .field("Function", &self.Function)
            .field("NRBFlags", &self.NRBFlags)
            .field("NRBStatus", &self.NRBStatus)
            .field("Count", &self.Count)
            .field("LBA", &self.LBA)
            .field("DataBufSize", &self.DataBufSize)
            .field("NVCacheStatus", &self.NVCacheStatus)
            .field("NVCacheSubStatus", &self.NVCacheSubStatus)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NVCACHE_REQUEST_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        self.NRBSize == other.NRBSize
            && self.Function == other.Function
            && self.NRBFlags == other.NRBFlags
            && self.NRBStatus == other.NRBStatus
            && self.Count == other.Count
            && self.LBA == other.LBA
            && self.DataBufSize == other.DataBufSize
            && self.NVCacheStatus == other.NVCacheStatus
            && self.NVCacheSubStatus == other.NVCacheSubStatus
    }
}
impl ::std::cmp::Eq for NVCACHE_REQUEST_BLOCK {}
unsafe impl ::windows::runtime::Abi for NVCACHE_REQUEST_BLOCK {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct NVCACHE_STATUS(pub i32);
pub const NvCacheStatusUnknown: NVCACHE_STATUS = NVCACHE_STATUS(0i32);
pub const NvCacheStatusDisabling: NVCACHE_STATUS = NVCACHE_STATUS(1i32);
pub const NvCacheStatusDisabled: NVCACHE_STATUS = NVCACHE_STATUS(2i32);
pub const NvCacheStatusEnabled: NVCACHE_STATUS = NVCACHE_STATUS(3i32);
impl ::std::convert::From<i32> for NVCACHE_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NVCACHE_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct NVCACHE_TYPE(pub i32);
pub const NvCacheTypeUnknown: NVCACHE_TYPE = NVCACHE_TYPE(0i32);
pub const NvCacheTypeNone: NVCACHE_TYPE = NVCACHE_TYPE(1i32);
pub const NvCacheTypeWriteBack: NVCACHE_TYPE = NVCACHE_TYPE(2i32);
pub const NvCacheTypeWriteThrough: NVCACHE_TYPE = NVCACHE_TYPE(3i32);
impl ::std::convert::From<i32> for NVCACHE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NVCACHE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct NV_FEATURE_PARAMETER {
    pub NVPowerModeEnabled: u16,
    pub NVParameterReserv1: u16,
    pub NVCmdEnabled: u16,
    pub NVParameterReserv2: u16,
    pub NVPowerModeVer: u16,
    pub NVCmdVer: u16,
    pub NVSize: u32,
    pub NVReadSpeed: u16,
    pub NVWrtSpeed: u16,
    pub DeviceSpinUpTime: u32,
}
impl NV_FEATURE_PARAMETER {}
impl ::std::default::Default for NV_FEATURE_PARAMETER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NV_FEATURE_PARAMETER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NV_FEATURE_PARAMETER")
            .field("NVPowerModeEnabled", &self.NVPowerModeEnabled)
            .field("NVParameterReserv1", &self.NVParameterReserv1)
            .field("NVCmdEnabled", &self.NVCmdEnabled)
            .field("NVParameterReserv2", &self.NVParameterReserv2)
            .field("NVPowerModeVer", &self.NVPowerModeVer)
            .field("NVCmdVer", &self.NVCmdVer)
            .field("NVSize", &self.NVSize)
            .field("NVReadSpeed", &self.NVReadSpeed)
            .field("NVWrtSpeed", &self.NVWrtSpeed)
            .field("DeviceSpinUpTime", &self.DeviceSpinUpTime)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NV_FEATURE_PARAMETER {
    fn eq(&self, other: &Self) -> bool {
        self.NVPowerModeEnabled == other.NVPowerModeEnabled
            && self.NVParameterReserv1 == other.NVParameterReserv1
            && self.NVCmdEnabled == other.NVCmdEnabled
            && self.NVParameterReserv2 == other.NVParameterReserv2
            && self.NVPowerModeVer == other.NVPowerModeVer
            && self.NVCmdVer == other.NVCmdVer
            && self.NVSize == other.NVSize
            && self.NVReadSpeed == other.NVReadSpeed
            && self.NVWrtSpeed == other.NVWrtSpeed
            && self.DeviceSpinUpTime == other.DeviceSpinUpTime
    }
}
impl ::std::cmp::Eq for NV_FEATURE_PARAMETER {}
unsafe impl ::windows::runtime::Abi for NV_FEATURE_PARAMETER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct NV_SEP_CACHE_PARAMETER {
    pub Version: u32,
    pub Size: u32,
    pub Flags: NV_SEP_CACHE_PARAMETER_0,
    pub WriteCacheType: u8,
    pub WriteCacheTypeEffective: u8,
    pub ParameterReserve1: [u8; 3],
}
impl NV_SEP_CACHE_PARAMETER {}
impl ::std::default::Default for NV_SEP_CACHE_PARAMETER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for NV_SEP_CACHE_PARAMETER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for NV_SEP_CACHE_PARAMETER {}
unsafe impl ::windows::runtime::Abi for NV_SEP_CACHE_PARAMETER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union NV_SEP_CACHE_PARAMETER_0 {
    pub CacheFlags: NV_SEP_CACHE_PARAMETER_0_0,
    pub CacheFlagsSet: u8,
}
impl NV_SEP_CACHE_PARAMETER_0 {}
impl ::std::default::Default for NV_SEP_CACHE_PARAMETER_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for NV_SEP_CACHE_PARAMETER_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for NV_SEP_CACHE_PARAMETER_0 {}
unsafe impl ::windows::runtime::Abi for NV_SEP_CACHE_PARAMETER_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct NV_SEP_CACHE_PARAMETER_0_0 {
    pub _bitfield: u8,
}
impl NV_SEP_CACHE_PARAMETER_0_0 {}
impl ::std::default::Default for NV_SEP_CACHE_PARAMETER_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NV_SEP_CACHE_PARAMETER_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_CacheFlags_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NV_SEP_CACHE_PARAMETER_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for NV_SEP_CACHE_PARAMETER_0_0 {}
unsafe impl ::windows::runtime::Abi for NV_SEP_CACHE_PARAMETER_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const NV_SEP_CACHE_PARAMETER_VERSION: u32 = 1u32;
pub const NV_SEP_CACHE_PARAMETER_VERSION_1: u32 = 1u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct NV_SEP_WRITE_CACHE_TYPE(pub i32);
pub const NVSEPWriteCacheTypeUnknown: NV_SEP_WRITE_CACHE_TYPE = NV_SEP_WRITE_CACHE_TYPE(0i32);
pub const NVSEPWriteCacheTypeNone: NV_SEP_WRITE_CACHE_TYPE = NV_SEP_WRITE_CACHE_TYPE(1i32);
pub const NVSEPWriteCacheTypeWriteBack: NV_SEP_WRITE_CACHE_TYPE = NV_SEP_WRITE_CACHE_TYPE(2i32);
pub const NVSEPWriteCacheTypeWriteThrough: NV_SEP_WRITE_CACHE_TYPE = NV_SEP_WRITE_CACHE_TYPE(3i32);
impl ::std::convert::From<i32> for NV_SEP_WRITE_CACHE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NV_SEP_WRITE_CACHE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub type PDUMP_DEVICE_POWERON_ROUTINE = unsafe extern "system" fn() -> i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub struct PERSISTENT_ISCSI_LOGIN_INFOA {
    pub TargetName: [super::super::System::SystemServices::CHAR; 224],
    pub IsInformationalSession: super::super::Foundation::BOOLEAN,
    pub InitiatorInstance: [super::super::System::SystemServices::CHAR; 256],
    pub InitiatorPortNumber: u32,
    pub TargetPortal: ISCSI_TARGET_PORTALA,
    pub SecurityFlags: u64,
    pub Mappings: *mut ISCSI_TARGET_MAPPINGA,
    pub LoginOptions: ISCSI_LOGIN_OPTIONS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl PERSISTENT_ISCSI_LOGIN_INFOA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for PERSISTENT_ISCSI_LOGIN_INFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::fmt::Debug for PERSISTENT_ISCSI_LOGIN_INFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PERSISTENT_ISCSI_LOGIN_INFOA")
            .field("TargetName", &self.TargetName)
            .field("IsInformationalSession", &self.IsInformationalSession)
            .field("InitiatorInstance", &self.InitiatorInstance)
            .field("InitiatorPortNumber", &self.InitiatorPortNumber)
            .field("TargetPortal", &self.TargetPortal)
            .field("SecurityFlags", &self.SecurityFlags)
            .field("Mappings", &self.Mappings)
            .field("LoginOptions", &self.LoginOptions)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for PERSISTENT_ISCSI_LOGIN_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.TargetName == other.TargetName
            && self.IsInformationalSession == other.IsInformationalSession
            && self.InitiatorInstance == other.InitiatorInstance
            && self.InitiatorPortNumber == other.InitiatorPortNumber
            && self.TargetPortal == other.TargetPortal
            && self.SecurityFlags == other.SecurityFlags
            && self.Mappings == other.Mappings
            && self.LoginOptions == other.LoginOptions
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for PERSISTENT_ISCSI_LOGIN_INFOA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for PERSISTENT_ISCSI_LOGIN_INFOA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PERSISTENT_ISCSI_LOGIN_INFOW {
    pub TargetName: [u16; 224],
    pub IsInformationalSession: super::super::Foundation::BOOLEAN,
    pub InitiatorInstance: [u16; 256],
    pub InitiatorPortNumber: u32,
    pub TargetPortal: ISCSI_TARGET_PORTALW,
    pub SecurityFlags: u64,
    pub Mappings: *mut ISCSI_TARGET_MAPPINGW,
    pub LoginOptions: ISCSI_LOGIN_OPTIONS,
}
#[cfg(feature = "Win32_Foundation")]
impl PERSISTENT_ISCSI_LOGIN_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PERSISTENT_ISCSI_LOGIN_INFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PERSISTENT_ISCSI_LOGIN_INFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PERSISTENT_ISCSI_LOGIN_INFOW")
            .field("TargetName", &self.TargetName)
            .field("IsInformationalSession", &self.IsInformationalSession)
            .field("InitiatorInstance", &self.InitiatorInstance)
            .field("InitiatorPortNumber", &self.InitiatorPortNumber)
            .field("TargetPortal", &self.TargetPortal)
            .field("SecurityFlags", &self.SecurityFlags)
            .field("Mappings", &self.Mappings)
            .field("LoginOptions", &self.LoginOptions)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PERSISTENT_ISCSI_LOGIN_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.TargetName == other.TargetName
            && self.IsInformationalSession == other.IsInformationalSession
            && self.InitiatorInstance == other.InitiatorInstance
            && self.InitiatorPortNumber == other.InitiatorPortNumber
            && self.TargetPortal == other.TargetPortal
            && self.SecurityFlags == other.SecurityFlags
            && self.Mappings == other.Mappings
            && self.LoginOptions == other.LoginOptions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PERSISTENT_ISCSI_LOGIN_INFOW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PERSISTENT_ISCSI_LOGIN_INFOW {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn RefreshISNSServerA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    address: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn RefreshISNSServerA(address: super::super::Foundation::PSTR) -> u32;
        }
        ::std::mem::transmute(RefreshISNSServerA(address.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn RefreshISNSServerW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    address: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn RefreshISNSServerW(address: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(RefreshISNSServerW(address.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn RefreshIScsiSendTargetPortalA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    initiatorinstance: Param0,
    initiatorportnumber: u32,
    portal: *mut ISCSI_TARGET_PORTALA,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn RefreshIScsiSendTargetPortalA(
                initiatorinstance: super::super::Foundation::PSTR,
                initiatorportnumber: u32,
                portal: *mut ISCSI_TARGET_PORTALA,
            ) -> u32;
        }
        ::std::mem::transmute(RefreshIScsiSendTargetPortalA(
            initiatorinstance.into_param().abi(),
            ::std::mem::transmute(initiatorportnumber),
            ::std::mem::transmute(portal),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn RefreshIScsiSendTargetPortalW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    initiatorinstance: Param0,
    initiatorportnumber: u32,
    portal: *mut ISCSI_TARGET_PORTALW,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn RefreshIScsiSendTargetPortalW(
                initiatorinstance: super::super::Foundation::PWSTR,
                initiatorportnumber: u32,
                portal: *mut ISCSI_TARGET_PORTALW,
            ) -> u32;
        }
        ::std::mem::transmute(RefreshIScsiSendTargetPortalW(
            initiatorinstance.into_param().abi(),
            ::std::mem::transmute(initiatorportnumber),
            ::std::mem::transmute(portal),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn RemoveISNSServerA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    address: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn RemoveISNSServerA(address: super::super::Foundation::PSTR) -> u32;
        }
        ::std::mem::transmute(RemoveISNSServerA(address.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn RemoveISNSServerW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    address: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn RemoveISNSServerW(address: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(RemoveISNSServerW(address.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn RemoveIScsiConnection(
    uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID,
    connectionid: *mut ISCSI_UNIQUE_SESSION_ID,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn RemoveIScsiConnection(
                uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID,
                connectionid: *mut ISCSI_UNIQUE_SESSION_ID,
            ) -> u32;
        }
        ::std::mem::transmute(RemoveIScsiConnection(
            ::std::mem::transmute(uniquesessionid),
            ::std::mem::transmute(connectionid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn RemoveIScsiPersistentTargetA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    initiatorinstance: Param0,
    initiatorportnumber: u32,
    targetname: Param2,
    portal: *mut ISCSI_TARGET_PORTALA,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn RemoveIScsiPersistentTargetA(
                initiatorinstance: super::super::Foundation::PSTR,
                initiatorportnumber: u32,
                targetname: super::super::Foundation::PSTR,
                portal: *mut ISCSI_TARGET_PORTALA,
            ) -> u32;
        }
        ::std::mem::transmute(RemoveIScsiPersistentTargetA(
            initiatorinstance.into_param().abi(),
            ::std::mem::transmute(initiatorportnumber),
            targetname.into_param().abi(),
            ::std::mem::transmute(portal),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn RemoveIScsiPersistentTargetW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    initiatorinstance: Param0,
    initiatorportnumber: u32,
    targetname: Param2,
    portal: *mut ISCSI_TARGET_PORTALW,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn RemoveIScsiPersistentTargetW(
                initiatorinstance: super::super::Foundation::PWSTR,
                initiatorportnumber: u32,
                targetname: super::super::Foundation::PWSTR,
                portal: *mut ISCSI_TARGET_PORTALW,
            ) -> u32;
        }
        ::std::mem::transmute(RemoveIScsiPersistentTargetW(
            initiatorinstance.into_param().abi(),
            ::std::mem::transmute(initiatorportnumber),
            targetname.into_param().abi(),
            ::std::mem::transmute(portal),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn RemoveIScsiSendTargetPortalA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    initiatorinstance: Param0,
    initiatorportnumber: u32,
    portal: *mut ISCSI_TARGET_PORTALA,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn RemoveIScsiSendTargetPortalA(
                initiatorinstance: super::super::Foundation::PSTR,
                initiatorportnumber: u32,
                portal: *mut ISCSI_TARGET_PORTALA,
            ) -> u32;
        }
        ::std::mem::transmute(RemoveIScsiSendTargetPortalA(
            initiatorinstance.into_param().abi(),
            ::std::mem::transmute(initiatorportnumber),
            ::std::mem::transmute(portal),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn RemoveIScsiSendTargetPortalW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    initiatorinstance: Param0,
    initiatorportnumber: u32,
    portal: *mut ISCSI_TARGET_PORTALW,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn RemoveIScsiSendTargetPortalW(
                initiatorinstance: super::super::Foundation::PWSTR,
                initiatorportnumber: u32,
                portal: *mut ISCSI_TARGET_PORTALW,
            ) -> u32;
        }
        ::std::mem::transmute(RemoveIScsiSendTargetPortalW(
            initiatorinstance.into_param().abi(),
            ::std::mem::transmute(initiatorportnumber),
            ::std::mem::transmute(portal),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn RemoveIScsiStaticTargetA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    targetname: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn RemoveIScsiStaticTargetA(targetname: super::super::Foundation::PSTR) -> u32;
        }
        ::std::mem::transmute(RemoveIScsiStaticTargetA(targetname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn RemoveIScsiStaticTargetW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    targetname: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn RemoveIScsiStaticTargetW(targetname: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(RemoveIScsiStaticTargetW(targetname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn RemovePersistentIScsiDeviceA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    devicepath: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn RemovePersistentIScsiDeviceA(devicepath: super::super::Foundation::PSTR) -> u32;
        }
        ::std::mem::transmute(RemovePersistentIScsiDeviceA(devicepath.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn RemovePersistentIScsiDeviceW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    devicepath: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn RemovePersistentIScsiDeviceW(devicepath: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(RemovePersistentIScsiDeviceW(devicepath.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn RemoveRadiusServerA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    address: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn RemoveRadiusServerA(address: super::super::Foundation::PSTR) -> u32;
        }
        ::std::mem::transmute(RemoveRadiusServerA(address.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn RemoveRadiusServerW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    address: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn RemoveRadiusServerW(address: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(RemoveRadiusServerW(address.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn ReportActiveIScsiTargetMappingsA(
    buffersize: *mut u32,
    mappingcount: *mut u32,
    mappings: *mut ISCSI_TARGET_MAPPINGA,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn ReportActiveIScsiTargetMappingsA(
                buffersize: *mut u32,
                mappingcount: *mut u32,
                mappings: *mut ISCSI_TARGET_MAPPINGA,
            ) -> u32;
        }
        ::std::mem::transmute(ReportActiveIScsiTargetMappingsA(
            ::std::mem::transmute(buffersize),
            ::std::mem::transmute(mappingcount),
            ::std::mem::transmute(mappings),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn ReportActiveIScsiTargetMappingsW(
    buffersize: *mut u32,
    mappingcount: *mut u32,
    mappings: *mut ISCSI_TARGET_MAPPINGW,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn ReportActiveIScsiTargetMappingsW(
                buffersize: *mut u32,
                mappingcount: *mut u32,
                mappings: *mut ISCSI_TARGET_MAPPINGW,
            ) -> u32;
        }
        ::std::mem::transmute(ReportActiveIScsiTargetMappingsW(
            ::std::mem::transmute(buffersize),
            ::std::mem::transmute(mappingcount),
            ::std::mem::transmute(mappings),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn ReportISNSServerListA(
    buffersizeinchar: *mut u32,
    buffer: super::super::Foundation::PSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn ReportISNSServerListA(
                buffersizeinchar: *mut u32,
                buffer: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(ReportISNSServerListA(
            ::std::mem::transmute(buffersizeinchar),
            ::std::mem::transmute(buffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn ReportISNSServerListW(
    buffersizeinchar: *mut u32,
    buffer: super::super::Foundation::PWSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn ReportISNSServerListW(
                buffersizeinchar: *mut u32,
                buffer: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(ReportISNSServerListW(
            ::std::mem::transmute(buffersizeinchar),
            ::std::mem::transmute(buffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn ReportIScsiInitiatorListA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    buffersize: *mut u32,
    buffer: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn ReportIScsiInitiatorListA(
                buffersize: *mut u32,
                buffer: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(ReportIScsiInitiatorListA(
            ::std::mem::transmute(buffersize),
            buffer.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn ReportIScsiInitiatorListW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    buffersize: *mut u32,
    buffer: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn ReportIScsiInitiatorListW(
                buffersize: *mut u32,
                buffer: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(ReportIScsiInitiatorListW(
            ::std::mem::transmute(buffersize),
            buffer.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn ReportIScsiPersistentLoginsA(
    count: *mut u32,
    persistentlogininfo: *mut PERSISTENT_ISCSI_LOGIN_INFOA,
    buffersizeinbytes: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn ReportIScsiPersistentLoginsA(
                count: *mut u32,
                persistentlogininfo: *mut PERSISTENT_ISCSI_LOGIN_INFOA,
                buffersizeinbytes: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(ReportIScsiPersistentLoginsA(
            ::std::mem::transmute(count),
            ::std::mem::transmute(persistentlogininfo),
            ::std::mem::transmute(buffersizeinbytes),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn ReportIScsiPersistentLoginsW(
    count: *mut u32,
    persistentlogininfo: *mut PERSISTENT_ISCSI_LOGIN_INFOW,
    buffersizeinbytes: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn ReportIScsiPersistentLoginsW(
                count: *mut u32,
                persistentlogininfo: *mut PERSISTENT_ISCSI_LOGIN_INFOW,
                buffersizeinbytes: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(ReportIScsiPersistentLoginsW(
            ::std::mem::transmute(count),
            ::std::mem::transmute(persistentlogininfo),
            ::std::mem::transmute(buffersizeinbytes),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn ReportIScsiSendTargetPortalsA(
    portalcount: *mut u32,
    portalinfo: *mut ISCSI_TARGET_PORTAL_INFOA,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn ReportIScsiSendTargetPortalsA(
                portalcount: *mut u32,
                portalinfo: *mut ISCSI_TARGET_PORTAL_INFOA,
            ) -> u32;
        }
        ::std::mem::transmute(ReportIScsiSendTargetPortalsA(
            ::std::mem::transmute(portalcount),
            ::std::mem::transmute(portalinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn ReportIScsiSendTargetPortalsExA(
    portalcount: *mut u32,
    portalinfosize: *mut u32,
    portalinfo: *mut ISCSI_TARGET_PORTAL_INFO_EXA,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn ReportIScsiSendTargetPortalsExA(
                portalcount: *mut u32,
                portalinfosize: *mut u32,
                portalinfo: *mut ISCSI_TARGET_PORTAL_INFO_EXA,
            ) -> u32;
        }
        ::std::mem::transmute(ReportIScsiSendTargetPortalsExA(
            ::std::mem::transmute(portalcount),
            ::std::mem::transmute(portalinfosize),
            ::std::mem::transmute(portalinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn ReportIScsiSendTargetPortalsExW(
    portalcount: *mut u32,
    portalinfosize: *mut u32,
    portalinfo: *mut ISCSI_TARGET_PORTAL_INFO_EXW,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn ReportIScsiSendTargetPortalsExW(
                portalcount: *mut u32,
                portalinfosize: *mut u32,
                portalinfo: *mut ISCSI_TARGET_PORTAL_INFO_EXW,
            ) -> u32;
        }
        ::std::mem::transmute(ReportIScsiSendTargetPortalsExW(
            ::std::mem::transmute(portalcount),
            ::std::mem::transmute(portalinfosize),
            ::std::mem::transmute(portalinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn ReportIScsiSendTargetPortalsW(
    portalcount: *mut u32,
    portalinfo: *mut ISCSI_TARGET_PORTAL_INFOW,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn ReportIScsiSendTargetPortalsW(
                portalcount: *mut u32,
                portalinfo: *mut ISCSI_TARGET_PORTAL_INFOW,
            ) -> u32;
        }
        ::std::mem::transmute(ReportIScsiSendTargetPortalsW(
            ::std::mem::transmute(portalcount),
            ::std::mem::transmute(portalinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn ReportIScsiTargetPortalsA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    initiatorname: Param0,
    targetname: Param1,
    targetportaltag: *mut u16,
    elementcount: *mut u32,
    portals: *mut ISCSI_TARGET_PORTALA,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn ReportIScsiTargetPortalsA(
                initiatorname: super::super::Foundation::PSTR,
                targetname: super::super::Foundation::PSTR,
                targetportaltag: *mut u16,
                elementcount: *mut u32,
                portals: *mut ISCSI_TARGET_PORTALA,
            ) -> u32;
        }
        ::std::mem::transmute(ReportIScsiTargetPortalsA(
            initiatorname.into_param().abi(),
            targetname.into_param().abi(),
            ::std::mem::transmute(targetportaltag),
            ::std::mem::transmute(elementcount),
            ::std::mem::transmute(portals),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn ReportIScsiTargetPortalsW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    initiatorname: Param0,
    targetname: Param1,
    targetportaltag: *mut u16,
    elementcount: *mut u32,
    portals: *mut ISCSI_TARGET_PORTALW,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn ReportIScsiTargetPortalsW(
                initiatorname: super::super::Foundation::PWSTR,
                targetname: super::super::Foundation::PWSTR,
                targetportaltag: *mut u16,
                elementcount: *mut u32,
                portals: *mut ISCSI_TARGET_PORTALW,
            ) -> u32;
        }
        ::std::mem::transmute(ReportIScsiTargetPortalsW(
            initiatorname.into_param().abi(),
            targetname.into_param().abi(),
            ::std::mem::transmute(targetportaltag),
            ::std::mem::transmute(elementcount),
            ::std::mem::transmute(portals),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn ReportIScsiTargetsA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    forceupdate: Param0,
    buffersize: *mut u32,
    buffer: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn ReportIScsiTargetsA(
                forceupdate: super::super::Foundation::BOOLEAN,
                buffersize: *mut u32,
                buffer: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(ReportIScsiTargetsA(
            forceupdate.into_param().abi(),
            ::std::mem::transmute(buffersize),
            buffer.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn ReportIScsiTargetsW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    forceupdate: Param0,
    buffersize: *mut u32,
    buffer: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn ReportIScsiTargetsW(
                forceupdate: super::super::Foundation::BOOLEAN,
                buffersize: *mut u32,
                buffer: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(ReportIScsiTargetsW(
            forceupdate.into_param().abi(),
            ::std::mem::transmute(buffersize),
            buffer.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn ReportPersistentIScsiDevicesA(
    buffersizeinchar: *mut u32,
    buffer: super::super::Foundation::PSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn ReportPersistentIScsiDevicesA(
                buffersizeinchar: *mut u32,
                buffer: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(ReportPersistentIScsiDevicesA(
            ::std::mem::transmute(buffersizeinchar),
            ::std::mem::transmute(buffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn ReportPersistentIScsiDevicesW(
    buffersizeinchar: *mut u32,
    buffer: super::super::Foundation::PWSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn ReportPersistentIScsiDevicesW(
                buffersizeinchar: *mut u32,
                buffer: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(ReportPersistentIScsiDevicesW(
            ::std::mem::transmute(buffersizeinchar),
            ::std::mem::transmute(buffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn ReportRadiusServerListA(
    buffersizeinchar: *mut u32,
    buffer: super::super::Foundation::PSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn ReportRadiusServerListA(
                buffersizeinchar: *mut u32,
                buffer: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(ReportRadiusServerListA(
            ::std::mem::transmute(buffersizeinchar),
            ::std::mem::transmute(buffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn ReportRadiusServerListW(
    buffersizeinchar: *mut u32,
    buffer: super::super::Foundation::PWSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn ReportRadiusServerListW(
                buffersizeinchar: *mut u32,
                buffer: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(ReportRadiusServerListW(
            ::std::mem::transmute(buffersizeinchar),
            ::std::mem::transmute(buffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SCSI_ADAPTER_BUS_INFO {
    pub NumberOfBuses: u8,
    pub BusData: [SCSI_BUS_DATA; 1],
}
impl SCSI_ADAPTER_BUS_INFO {}
impl ::std::default::Default for SCSI_ADAPTER_BUS_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SCSI_ADAPTER_BUS_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SCSI_ADAPTER_BUS_INFO")
            .field("NumberOfBuses", &self.NumberOfBuses)
            .field("BusData", &self.BusData)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SCSI_ADAPTER_BUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfBuses == other.NumberOfBuses && self.BusData == other.BusData
    }
}
impl ::std::cmp::Eq for SCSI_ADAPTER_BUS_INFO {}
unsafe impl ::windows::runtime::Abi for SCSI_ADAPTER_BUS_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SCSI_ADDRESS {
    pub Length: u32,
    pub PortNumber: u8,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
}
impl SCSI_ADDRESS {}
impl ::std::default::Default for SCSI_ADDRESS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SCSI_ADDRESS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SCSI_ADDRESS")
            .field("Length", &self.Length)
            .field("PortNumber", &self.PortNumber)
            .field("PathId", &self.PathId)
            .field("TargetId", &self.TargetId)
            .field("Lun", &self.Lun)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SCSI_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length
            && self.PortNumber == other.PortNumber
            && self.PathId == other.PathId
            && self.TargetId == other.TargetId
            && self.Lun == other.Lun
    }
}
impl ::std::cmp::Eq for SCSI_ADDRESS {}
unsafe impl ::windows::runtime::Abi for SCSI_ADDRESS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SCSI_BUS_DATA {
    pub NumberOfLogicalUnits: u8,
    pub InitiatorBusId: u8,
    pub InquiryDataOffset: u32,
}
impl SCSI_BUS_DATA {}
impl ::std::default::Default for SCSI_BUS_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SCSI_BUS_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SCSI_BUS_DATA")
            .field("NumberOfLogicalUnits", &self.NumberOfLogicalUnits)
            .field("InitiatorBusId", &self.InitiatorBusId)
            .field("InquiryDataOffset", &self.InquiryDataOffset)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SCSI_BUS_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfLogicalUnits == other.NumberOfLogicalUnits
            && self.InitiatorBusId == other.InitiatorBusId
            && self.InquiryDataOffset == other.InquiryDataOffset
    }
}
impl ::std::cmp::Eq for SCSI_BUS_DATA {}
unsafe impl ::windows::runtime::Abi for SCSI_BUS_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SCSI_INQUIRY_DATA {
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub DeviceClaimed: super::super::Foundation::BOOLEAN,
    pub InquiryDataLength: u32,
    pub NextInquiryDataOffset: u32,
    pub InquiryData: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl SCSI_INQUIRY_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SCSI_INQUIRY_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SCSI_INQUIRY_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SCSI_INQUIRY_DATA")
            .field("PathId", &self.PathId)
            .field("TargetId", &self.TargetId)
            .field("Lun", &self.Lun)
            .field("DeviceClaimed", &self.DeviceClaimed)
            .field("InquiryDataLength", &self.InquiryDataLength)
            .field("NextInquiryDataOffset", &self.NextInquiryDataOffset)
            .field("InquiryData", &self.InquiryData)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SCSI_INQUIRY_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.PathId == other.PathId
            && self.TargetId == other.TargetId
            && self.Lun == other.Lun
            && self.DeviceClaimed == other.DeviceClaimed
            && self.InquiryDataLength == other.InquiryDataLength
            && self.NextInquiryDataOffset == other.NextInquiryDataOffset
            && self.InquiryData == other.InquiryData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SCSI_INQUIRY_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SCSI_INQUIRY_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SCSI_IOCTL_DATA_BIDIRECTIONAL: u32 = 3u32;
pub const SCSI_IOCTL_DATA_IN: u32 = 1u32;
pub const SCSI_IOCTL_DATA_OUT: u32 = 0u32;
pub const SCSI_IOCTL_DATA_UNSPECIFIED: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SCSI_LUN_LIST {
    pub OSLUN: u32,
    pub TargetLUN: u64,
}
impl SCSI_LUN_LIST {}
impl ::std::default::Default for SCSI_LUN_LIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SCSI_LUN_LIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SCSI_LUN_LIST")
            .field("OSLUN", &self.OSLUN)
            .field("TargetLUN", &self.TargetLUN)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SCSI_LUN_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.OSLUN == other.OSLUN && self.TargetLUN == other.TargetLUN
    }
}
impl ::std::cmp::Eq for SCSI_LUN_LIST {}
unsafe impl ::windows::runtime::Abi for SCSI_LUN_LIST {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SCSI_PASS_THROUGH {
    pub Length: u16,
    pub ScsiStatus: u8,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub CdbLength: u8,
    pub SenseInfoLength: u8,
    pub DataIn: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub DataBufferOffset: usize,
    pub SenseInfoOffset: u32,
    pub Cdb: [u8; 16],
}
impl SCSI_PASS_THROUGH {}
impl ::std::default::Default for SCSI_PASS_THROUGH {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SCSI_PASS_THROUGH {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SCSI_PASS_THROUGH")
            .field("Length", &self.Length)
            .field("ScsiStatus", &self.ScsiStatus)
            .field("PathId", &self.PathId)
            .field("TargetId", &self.TargetId)
            .field("Lun", &self.Lun)
            .field("CdbLength", &self.CdbLength)
            .field("SenseInfoLength", &self.SenseInfoLength)
            .field("DataIn", &self.DataIn)
            .field("DataTransferLength", &self.DataTransferLength)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("DataBufferOffset", &self.DataBufferOffset)
            .field("SenseInfoOffset", &self.SenseInfoOffset)
            .field("Cdb", &self.Cdb)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SCSI_PASS_THROUGH {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length
            && self.ScsiStatus == other.ScsiStatus
            && self.PathId == other.PathId
            && self.TargetId == other.TargetId
            && self.Lun == other.Lun
            && self.CdbLength == other.CdbLength
            && self.SenseInfoLength == other.SenseInfoLength
            && self.DataIn == other.DataIn
            && self.DataTransferLength == other.DataTransferLength
            && self.TimeOutValue == other.TimeOutValue
            && self.DataBufferOffset == other.DataBufferOffset
            && self.SenseInfoOffset == other.SenseInfoOffset
            && self.Cdb == other.Cdb
    }
}
impl ::std::cmp::Eq for SCSI_PASS_THROUGH {}
unsafe impl ::windows::runtime::Abi for SCSI_PASS_THROUGH {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SCSI_PASS_THROUGH32 {
    pub Length: u16,
    pub ScsiStatus: u8,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub CdbLength: u8,
    pub SenseInfoLength: u8,
    pub DataIn: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub DataBufferOffset: u32,
    pub SenseInfoOffset: u32,
    pub Cdb: [u8; 16],
}
impl SCSI_PASS_THROUGH32 {}
impl ::std::default::Default for SCSI_PASS_THROUGH32 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SCSI_PASS_THROUGH32 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SCSI_PASS_THROUGH32")
            .field("Length", &self.Length)
            .field("ScsiStatus", &self.ScsiStatus)
            .field("PathId", &self.PathId)
            .field("TargetId", &self.TargetId)
            .field("Lun", &self.Lun)
            .field("CdbLength", &self.CdbLength)
            .field("SenseInfoLength", &self.SenseInfoLength)
            .field("DataIn", &self.DataIn)
            .field("DataTransferLength", &self.DataTransferLength)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("DataBufferOffset", &self.DataBufferOffset)
            .field("SenseInfoOffset", &self.SenseInfoOffset)
            .field("Cdb", &self.Cdb)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SCSI_PASS_THROUGH32 {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length
            && self.ScsiStatus == other.ScsiStatus
            && self.PathId == other.PathId
            && self.TargetId == other.TargetId
            && self.Lun == other.Lun
            && self.CdbLength == other.CdbLength
            && self.SenseInfoLength == other.SenseInfoLength
            && self.DataIn == other.DataIn
            && self.DataTransferLength == other.DataTransferLength
            && self.TimeOutValue == other.TimeOutValue
            && self.DataBufferOffset == other.DataBufferOffset
            && self.SenseInfoOffset == other.SenseInfoOffset
            && self.Cdb == other.Cdb
    }
}
impl ::std::cmp::Eq for SCSI_PASS_THROUGH32 {}
unsafe impl ::windows::runtime::Abi for SCSI_PASS_THROUGH32 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SCSI_PASS_THROUGH32_EX {
    pub Version: u32,
    pub Length: u32,
    pub CdbLength: u32,
    pub StorAddressLength: u32,
    pub ScsiStatus: u8,
    pub SenseInfoLength: u8,
    pub DataDirection: u8,
    pub Reserved: u8,
    pub TimeOutValue: u32,
    pub StorAddressOffset: u32,
    pub SenseInfoOffset: u32,
    pub DataOutTransferLength: u32,
    pub DataInTransferLength: u32,
    pub DataOutBufferOffset: u32,
    pub DataInBufferOffset: u32,
    pub Cdb: [u8; 1],
}
impl SCSI_PASS_THROUGH32_EX {}
impl ::std::default::Default for SCSI_PASS_THROUGH32_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SCSI_PASS_THROUGH32_EX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SCSI_PASS_THROUGH32_EX")
            .field("Version", &self.Version)
            .field("Length", &self.Length)
            .field("CdbLength", &self.CdbLength)
            .field("StorAddressLength", &self.StorAddressLength)
            .field("ScsiStatus", &self.ScsiStatus)
            .field("SenseInfoLength", &self.SenseInfoLength)
            .field("DataDirection", &self.DataDirection)
            .field("Reserved", &self.Reserved)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("StorAddressOffset", &self.StorAddressOffset)
            .field("SenseInfoOffset", &self.SenseInfoOffset)
            .field("DataOutTransferLength", &self.DataOutTransferLength)
            .field("DataInTransferLength", &self.DataInTransferLength)
            .field("DataOutBufferOffset", &self.DataOutBufferOffset)
            .field("DataInBufferOffset", &self.DataInBufferOffset)
            .field("Cdb", &self.Cdb)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SCSI_PASS_THROUGH32_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Length == other.Length
            && self.CdbLength == other.CdbLength
            && self.StorAddressLength == other.StorAddressLength
            && self.ScsiStatus == other.ScsiStatus
            && self.SenseInfoLength == other.SenseInfoLength
            && self.DataDirection == other.DataDirection
            && self.Reserved == other.Reserved
            && self.TimeOutValue == other.TimeOutValue
            && self.StorAddressOffset == other.StorAddressOffset
            && self.SenseInfoOffset == other.SenseInfoOffset
            && self.DataOutTransferLength == other.DataOutTransferLength
            && self.DataInTransferLength == other.DataInTransferLength
            && self.DataOutBufferOffset == other.DataOutBufferOffset
            && self.DataInBufferOffset == other.DataInBufferOffset
            && self.Cdb == other.Cdb
    }
}
impl ::std::cmp::Eq for SCSI_PASS_THROUGH32_EX {}
unsafe impl ::windows::runtime::Abi for SCSI_PASS_THROUGH32_EX {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SCSI_PASS_THROUGH_DIRECT {
    pub Length: u16,
    pub ScsiStatus: u8,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub CdbLength: u8,
    pub SenseInfoLength: u8,
    pub DataIn: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub DataBuffer: *mut ::std::ffi::c_void,
    pub SenseInfoOffset: u32,
    pub Cdb: [u8; 16],
}
impl SCSI_PASS_THROUGH_DIRECT {}
impl ::std::default::Default for SCSI_PASS_THROUGH_DIRECT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SCSI_PASS_THROUGH_DIRECT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SCSI_PASS_THROUGH_DIRECT")
            .field("Length", &self.Length)
            .field("ScsiStatus", &self.ScsiStatus)
            .field("PathId", &self.PathId)
            .field("TargetId", &self.TargetId)
            .field("Lun", &self.Lun)
            .field("CdbLength", &self.CdbLength)
            .field("SenseInfoLength", &self.SenseInfoLength)
            .field("DataIn", &self.DataIn)
            .field("DataTransferLength", &self.DataTransferLength)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("DataBuffer", &self.DataBuffer)
            .field("SenseInfoOffset", &self.SenseInfoOffset)
            .field("Cdb", &self.Cdb)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SCSI_PASS_THROUGH_DIRECT {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length
            && self.ScsiStatus == other.ScsiStatus
            && self.PathId == other.PathId
            && self.TargetId == other.TargetId
            && self.Lun == other.Lun
            && self.CdbLength == other.CdbLength
            && self.SenseInfoLength == other.SenseInfoLength
            && self.DataIn == other.DataIn
            && self.DataTransferLength == other.DataTransferLength
            && self.TimeOutValue == other.TimeOutValue
            && self.DataBuffer == other.DataBuffer
            && self.SenseInfoOffset == other.SenseInfoOffset
            && self.Cdb == other.Cdb
    }
}
impl ::std::cmp::Eq for SCSI_PASS_THROUGH_DIRECT {}
unsafe impl ::windows::runtime::Abi for SCSI_PASS_THROUGH_DIRECT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SCSI_PASS_THROUGH_DIRECT32 {
    pub Length: u16,
    pub ScsiStatus: u8,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub CdbLength: u8,
    pub SenseInfoLength: u8,
    pub DataIn: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub DataBuffer: *mut ::std::ffi::c_void,
    pub SenseInfoOffset: u32,
    pub Cdb: [u8; 16],
}
impl SCSI_PASS_THROUGH_DIRECT32 {}
impl ::std::default::Default for SCSI_PASS_THROUGH_DIRECT32 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SCSI_PASS_THROUGH_DIRECT32 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SCSI_PASS_THROUGH_DIRECT32")
            .field("Length", &self.Length)
            .field("ScsiStatus", &self.ScsiStatus)
            .field("PathId", &self.PathId)
            .field("TargetId", &self.TargetId)
            .field("Lun", &self.Lun)
            .field("CdbLength", &self.CdbLength)
            .field("SenseInfoLength", &self.SenseInfoLength)
            .field("DataIn", &self.DataIn)
            .field("DataTransferLength", &self.DataTransferLength)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("DataBuffer", &self.DataBuffer)
            .field("SenseInfoOffset", &self.SenseInfoOffset)
            .field("Cdb", &self.Cdb)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SCSI_PASS_THROUGH_DIRECT32 {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length
            && self.ScsiStatus == other.ScsiStatus
            && self.PathId == other.PathId
            && self.TargetId == other.TargetId
            && self.Lun == other.Lun
            && self.CdbLength == other.CdbLength
            && self.SenseInfoLength == other.SenseInfoLength
            && self.DataIn == other.DataIn
            && self.DataTransferLength == other.DataTransferLength
            && self.TimeOutValue == other.TimeOutValue
            && self.DataBuffer == other.DataBuffer
            && self.SenseInfoOffset == other.SenseInfoOffset
            && self.Cdb == other.Cdb
    }
}
impl ::std::cmp::Eq for SCSI_PASS_THROUGH_DIRECT32 {}
unsafe impl ::windows::runtime::Abi for SCSI_PASS_THROUGH_DIRECT32 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SCSI_PASS_THROUGH_DIRECT32_EX {
    pub Version: u32,
    pub Length: u32,
    pub CdbLength: u32,
    pub StorAddressLength: u32,
    pub ScsiStatus: u8,
    pub SenseInfoLength: u8,
    pub DataDirection: u8,
    pub Reserved: u8,
    pub TimeOutValue: u32,
    pub StorAddressOffset: u32,
    pub SenseInfoOffset: u32,
    pub DataOutTransferLength: u32,
    pub DataInTransferLength: u32,
    pub DataOutBuffer: *mut ::std::ffi::c_void,
    pub DataInBuffer: *mut ::std::ffi::c_void,
    pub Cdb: [u8; 1],
}
impl SCSI_PASS_THROUGH_DIRECT32_EX {}
impl ::std::default::Default for SCSI_PASS_THROUGH_DIRECT32_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SCSI_PASS_THROUGH_DIRECT32_EX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SCSI_PASS_THROUGH_DIRECT32_EX")
            .field("Version", &self.Version)
            .field("Length", &self.Length)
            .field("CdbLength", &self.CdbLength)
            .field("StorAddressLength", &self.StorAddressLength)
            .field("ScsiStatus", &self.ScsiStatus)
            .field("SenseInfoLength", &self.SenseInfoLength)
            .field("DataDirection", &self.DataDirection)
            .field("Reserved", &self.Reserved)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("StorAddressOffset", &self.StorAddressOffset)
            .field("SenseInfoOffset", &self.SenseInfoOffset)
            .field("DataOutTransferLength", &self.DataOutTransferLength)
            .field("DataInTransferLength", &self.DataInTransferLength)
            .field("DataOutBuffer", &self.DataOutBuffer)
            .field("DataInBuffer", &self.DataInBuffer)
            .field("Cdb", &self.Cdb)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SCSI_PASS_THROUGH_DIRECT32_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Length == other.Length
            && self.CdbLength == other.CdbLength
            && self.StorAddressLength == other.StorAddressLength
            && self.ScsiStatus == other.ScsiStatus
            && self.SenseInfoLength == other.SenseInfoLength
            && self.DataDirection == other.DataDirection
            && self.Reserved == other.Reserved
            && self.TimeOutValue == other.TimeOutValue
            && self.StorAddressOffset == other.StorAddressOffset
            && self.SenseInfoOffset == other.SenseInfoOffset
            && self.DataOutTransferLength == other.DataOutTransferLength
            && self.DataInTransferLength == other.DataInTransferLength
            && self.DataOutBuffer == other.DataOutBuffer
            && self.DataInBuffer == other.DataInBuffer
            && self.Cdb == other.Cdb
    }
}
impl ::std::cmp::Eq for SCSI_PASS_THROUGH_DIRECT32_EX {}
unsafe impl ::windows::runtime::Abi for SCSI_PASS_THROUGH_DIRECT32_EX {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SCSI_PASS_THROUGH_DIRECT_EX {
    pub Version: u32,
    pub Length: u32,
    pub CdbLength: u32,
    pub StorAddressLength: u32,
    pub ScsiStatus: u8,
    pub SenseInfoLength: u8,
    pub DataDirection: u8,
    pub Reserved: u8,
    pub TimeOutValue: u32,
    pub StorAddressOffset: u32,
    pub SenseInfoOffset: u32,
    pub DataOutTransferLength: u32,
    pub DataInTransferLength: u32,
    pub DataOutBuffer: *mut ::std::ffi::c_void,
    pub DataInBuffer: *mut ::std::ffi::c_void,
    pub Cdb: [u8; 1],
}
impl SCSI_PASS_THROUGH_DIRECT_EX {}
impl ::std::default::Default for SCSI_PASS_THROUGH_DIRECT_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SCSI_PASS_THROUGH_DIRECT_EX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SCSI_PASS_THROUGH_DIRECT_EX")
            .field("Version", &self.Version)
            .field("Length", &self.Length)
            .field("CdbLength", &self.CdbLength)
            .field("StorAddressLength", &self.StorAddressLength)
            .field("ScsiStatus", &self.ScsiStatus)
            .field("SenseInfoLength", &self.SenseInfoLength)
            .field("DataDirection", &self.DataDirection)
            .field("Reserved", &self.Reserved)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("StorAddressOffset", &self.StorAddressOffset)
            .field("SenseInfoOffset", &self.SenseInfoOffset)
            .field("DataOutTransferLength", &self.DataOutTransferLength)
            .field("DataInTransferLength", &self.DataInTransferLength)
            .field("DataOutBuffer", &self.DataOutBuffer)
            .field("DataInBuffer", &self.DataInBuffer)
            .field("Cdb", &self.Cdb)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SCSI_PASS_THROUGH_DIRECT_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Length == other.Length
            && self.CdbLength == other.CdbLength
            && self.StorAddressLength == other.StorAddressLength
            && self.ScsiStatus == other.ScsiStatus
            && self.SenseInfoLength == other.SenseInfoLength
            && self.DataDirection == other.DataDirection
            && self.Reserved == other.Reserved
            && self.TimeOutValue == other.TimeOutValue
            && self.StorAddressOffset == other.StorAddressOffset
            && self.SenseInfoOffset == other.SenseInfoOffset
            && self.DataOutTransferLength == other.DataOutTransferLength
            && self.DataInTransferLength == other.DataInTransferLength
            && self.DataOutBuffer == other.DataOutBuffer
            && self.DataInBuffer == other.DataInBuffer
            && self.Cdb == other.Cdb
    }
}
impl ::std::cmp::Eq for SCSI_PASS_THROUGH_DIRECT_EX {}
unsafe impl ::windows::runtime::Abi for SCSI_PASS_THROUGH_DIRECT_EX {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SCSI_PASS_THROUGH_EX {
    pub Version: u32,
    pub Length: u32,
    pub CdbLength: u32,
    pub StorAddressLength: u32,
    pub ScsiStatus: u8,
    pub SenseInfoLength: u8,
    pub DataDirection: u8,
    pub Reserved: u8,
    pub TimeOutValue: u32,
    pub StorAddressOffset: u32,
    pub SenseInfoOffset: u32,
    pub DataOutTransferLength: u32,
    pub DataInTransferLength: u32,
    pub DataOutBufferOffset: usize,
    pub DataInBufferOffset: usize,
    pub Cdb: [u8; 1],
}
impl SCSI_PASS_THROUGH_EX {}
impl ::std::default::Default for SCSI_PASS_THROUGH_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SCSI_PASS_THROUGH_EX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SCSI_PASS_THROUGH_EX")
            .field("Version", &self.Version)
            .field("Length", &self.Length)
            .field("CdbLength", &self.CdbLength)
            .field("StorAddressLength", &self.StorAddressLength)
            .field("ScsiStatus", &self.ScsiStatus)
            .field("SenseInfoLength", &self.SenseInfoLength)
            .field("DataDirection", &self.DataDirection)
            .field("Reserved", &self.Reserved)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("StorAddressOffset", &self.StorAddressOffset)
            .field("SenseInfoOffset", &self.SenseInfoOffset)
            .field("DataOutTransferLength", &self.DataOutTransferLength)
            .field("DataInTransferLength", &self.DataInTransferLength)
            .field("DataOutBufferOffset", &self.DataOutBufferOffset)
            .field("DataInBufferOffset", &self.DataInBufferOffset)
            .field("Cdb", &self.Cdb)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SCSI_PASS_THROUGH_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Length == other.Length
            && self.CdbLength == other.CdbLength
            && self.StorAddressLength == other.StorAddressLength
            && self.ScsiStatus == other.ScsiStatus
            && self.SenseInfoLength == other.SenseInfoLength
            && self.DataDirection == other.DataDirection
            && self.Reserved == other.Reserved
            && self.TimeOutValue == other.TimeOutValue
            && self.StorAddressOffset == other.StorAddressOffset
            && self.SenseInfoOffset == other.SenseInfoOffset
            && self.DataOutTransferLength == other.DataOutTransferLength
            && self.DataInTransferLength == other.DataInTransferLength
            && self.DataOutBufferOffset == other.DataOutBufferOffset
            && self.DataInBufferOffset == other.DataInBufferOffset
            && self.Cdb == other.Cdb
    }
}
impl ::std::cmp::Eq for SCSI_PASS_THROUGH_EX {}
unsafe impl ::windows::runtime::Abi for SCSI_PASS_THROUGH_EX {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SRB_IO_CONTROL {
    pub HeaderLength: u32,
    pub Signature: [u8; 8],
    pub Timeout: u32,
    pub ControlCode: u32,
    pub ReturnCode: u32,
    pub Length: u32,
}
impl SRB_IO_CONTROL {}
impl ::std::default::Default for SRB_IO_CONTROL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SRB_IO_CONTROL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SRB_IO_CONTROL")
            .field("HeaderLength", &self.HeaderLength)
            .field("Signature", &self.Signature)
            .field("Timeout", &self.Timeout)
            .field("ControlCode", &self.ControlCode)
            .field("ReturnCode", &self.ReturnCode)
            .field("Length", &self.Length)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SRB_IO_CONTROL {
    fn eq(&self, other: &Self) -> bool {
        self.HeaderLength == other.HeaderLength
            && self.Signature == other.Signature
            && self.Timeout == other.Timeout
            && self.ControlCode == other.ControlCode
            && self.ReturnCode == other.ReturnCode
            && self.Length == other.Length
    }
}
impl ::std::cmp::Eq for SRB_IO_CONTROL {}
unsafe impl ::windows::runtime::Abi for SRB_IO_CONTROL {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct STORAGE_DIAGNOSTIC_MP_REQUEST {
    pub Version: u32,
    pub Size: u32,
    pub TargetType: MP_STORAGE_DIAGNOSTIC_TARGET_TYPE,
    pub Level: MP_STORAGE_DIAGNOSTIC_LEVEL,
    pub ProviderId: ::windows::runtime::GUID,
    pub BufferSize: u32,
    pub Reserved: u32,
    pub DataBuffer: [u8; 1],
}
impl STORAGE_DIAGNOSTIC_MP_REQUEST {}
impl ::std::default::Default for STORAGE_DIAGNOSTIC_MP_REQUEST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for STORAGE_DIAGNOSTIC_MP_REQUEST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STORAGE_DIAGNOSTIC_MP_REQUEST")
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .field("TargetType", &self.TargetType)
            .field("Level", &self.Level)
            .field("ProviderId", &self.ProviderId)
            .field("BufferSize", &self.BufferSize)
            .field("Reserved", &self.Reserved)
            .field("DataBuffer", &self.DataBuffer)
            .finish()
    }
}
impl ::std::cmp::PartialEq for STORAGE_DIAGNOSTIC_MP_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Size == other.Size
            && self.TargetType == other.TargetType
            && self.Level == other.Level
            && self.ProviderId == other.ProviderId
            && self.BufferSize == other.BufferSize
            && self.Reserved == other.Reserved
            && self.DataBuffer == other.DataBuffer
    }
}
impl ::std::cmp::Eq for STORAGE_DIAGNOSTIC_MP_REQUEST {}
unsafe impl ::windows::runtime::Abi for STORAGE_DIAGNOSTIC_MP_REQUEST {
    type Abi = Self;
    type DefaultType = Self;
}
pub const STORAGE_DIAGNOSTIC_STATUS_BUFFER_TOO_SMALL: u32 = 1u32;
pub const STORAGE_DIAGNOSTIC_STATUS_INVALID_PARAMETER: u32 = 3u32;
pub const STORAGE_DIAGNOSTIC_STATUS_INVALID_SIGNATURE: u32 = 4u32;
pub const STORAGE_DIAGNOSTIC_STATUS_INVALID_TARGET_TYPE: u32 = 5u32;
pub const STORAGE_DIAGNOSTIC_STATUS_MORE_DATA: u32 = 6u32;
pub const STORAGE_DIAGNOSTIC_STATUS_SUCCESS: u32 = 0u32;
pub const STORAGE_DIAGNOSTIC_STATUS_UNSUPPORTED_VERSION: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct STORAGE_ENDURANCE_DATA_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub EnduranceInfo: STORAGE_ENDURANCE_INFO,
}
impl STORAGE_ENDURANCE_DATA_DESCRIPTOR {}
impl ::std::default::Default for STORAGE_ENDURANCE_DATA_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for STORAGE_ENDURANCE_DATA_DESCRIPTOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STORAGE_ENDURANCE_DATA_DESCRIPTOR")
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .field("EnduranceInfo", &self.EnduranceInfo)
            .finish()
    }
}
impl ::std::cmp::PartialEq for STORAGE_ENDURANCE_DATA_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Size == other.Size
            && self.EnduranceInfo == other.EnduranceInfo
    }
}
impl ::std::cmp::Eq for STORAGE_ENDURANCE_DATA_DESCRIPTOR {}
unsafe impl ::windows::runtime::Abi for STORAGE_ENDURANCE_DATA_DESCRIPTOR {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct STORAGE_ENDURANCE_INFO {
    pub ValidFields: u32,
    pub GroupId: u32,
    pub Flags: STORAGE_ENDURANCE_INFO_0,
    pub LifePercentage: u32,
    pub BytesReadCount: [u8; 16],
    pub ByteWriteCount: [u8; 16],
}
impl STORAGE_ENDURANCE_INFO {}
impl ::std::default::Default for STORAGE_ENDURANCE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for STORAGE_ENDURANCE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STORAGE_ENDURANCE_INFO")
            .field("ValidFields", &self.ValidFields)
            .field("GroupId", &self.GroupId)
            .field("Flags", &self.Flags)
            .field("LifePercentage", &self.LifePercentage)
            .field("BytesReadCount", &self.BytesReadCount)
            .field("ByteWriteCount", &self.ByteWriteCount)
            .finish()
    }
}
impl ::std::cmp::PartialEq for STORAGE_ENDURANCE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ValidFields == other.ValidFields
            && self.GroupId == other.GroupId
            && self.Flags == other.Flags
            && self.LifePercentage == other.LifePercentage
            && self.BytesReadCount == other.BytesReadCount
            && self.ByteWriteCount == other.ByteWriteCount
    }
}
impl ::std::cmp::Eq for STORAGE_ENDURANCE_INFO {}
unsafe impl ::windows::runtime::Abi for STORAGE_ENDURANCE_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct STORAGE_ENDURANCE_INFO_0 {
    pub _bitfield: u32,
}
impl STORAGE_ENDURANCE_INFO_0 {}
impl ::std::default::Default for STORAGE_ENDURANCE_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for STORAGE_ENDURANCE_INFO_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Flags_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for STORAGE_ENDURANCE_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for STORAGE_ENDURANCE_INFO_0 {}
unsafe impl ::windows::runtime::Abi for STORAGE_ENDURANCE_INFO_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct STORAGE_FIRMWARE_ACTIVATE {
    pub Version: u32,
    pub Size: u32,
    pub SlotToActivate: u8,
    pub Reserved0: [u8; 3],
}
impl STORAGE_FIRMWARE_ACTIVATE {}
impl ::std::default::Default for STORAGE_FIRMWARE_ACTIVATE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for STORAGE_FIRMWARE_ACTIVATE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STORAGE_FIRMWARE_ACTIVATE")
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .field("SlotToActivate", &self.SlotToActivate)
            .field("Reserved0", &self.Reserved0)
            .finish()
    }
}
impl ::std::cmp::PartialEq for STORAGE_FIRMWARE_ACTIVATE {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Size == other.Size
            && self.SlotToActivate == other.SlotToActivate
            && self.Reserved0 == other.Reserved0
    }
}
impl ::std::cmp::Eq for STORAGE_FIRMWARE_ACTIVATE {}
unsafe impl ::windows::runtime::Abi for STORAGE_FIRMWARE_ACTIVATE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const STORAGE_FIRMWARE_ACTIVATE_STRUCTURE_VERSION: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct STORAGE_FIRMWARE_DOWNLOAD {
    pub Version: u32,
    pub Size: u32,
    pub Offset: u64,
    pub BufferSize: u64,
    pub ImageBuffer: [u8; 1],
}
impl STORAGE_FIRMWARE_DOWNLOAD {}
impl ::std::default::Default for STORAGE_FIRMWARE_DOWNLOAD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for STORAGE_FIRMWARE_DOWNLOAD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STORAGE_FIRMWARE_DOWNLOAD")
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .field("Offset", &self.Offset)
            .field("BufferSize", &self.BufferSize)
            .field("ImageBuffer", &self.ImageBuffer)
            .finish()
    }
}
impl ::std::cmp::PartialEq for STORAGE_FIRMWARE_DOWNLOAD {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Size == other.Size
            && self.Offset == other.Offset
            && self.BufferSize == other.BufferSize
            && self.ImageBuffer == other.ImageBuffer
    }
}
impl ::std::cmp::Eq for STORAGE_FIRMWARE_DOWNLOAD {}
unsafe impl ::windows::runtime::Abi for STORAGE_FIRMWARE_DOWNLOAD {
    type Abi = Self;
    type DefaultType = Self;
}
pub const STORAGE_FIRMWARE_DOWNLOAD_STRUCTURE_VERSION: u32 = 1u32;
pub const STORAGE_FIRMWARE_DOWNLOAD_STRUCTURE_VERSION_V2: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct STORAGE_FIRMWARE_DOWNLOAD_V2 {
    pub Version: u32,
    pub Size: u32,
    pub Offset: u64,
    pub BufferSize: u64,
    pub Slot: u8,
    pub Reserved: [u8; 3],
    pub ImageSize: u32,
    pub ImageBuffer: [u8; 1],
}
impl STORAGE_FIRMWARE_DOWNLOAD_V2 {}
impl ::std::default::Default for STORAGE_FIRMWARE_DOWNLOAD_V2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for STORAGE_FIRMWARE_DOWNLOAD_V2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STORAGE_FIRMWARE_DOWNLOAD_V2")
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .field("Offset", &self.Offset)
            .field("BufferSize", &self.BufferSize)
            .field("Slot", &self.Slot)
            .field("Reserved", &self.Reserved)
            .field("ImageSize", &self.ImageSize)
            .field("ImageBuffer", &self.ImageBuffer)
            .finish()
    }
}
impl ::std::cmp::PartialEq for STORAGE_FIRMWARE_DOWNLOAD_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Size == other.Size
            && self.Offset == other.Offset
            && self.BufferSize == other.BufferSize
            && self.Slot == other.Slot
            && self.Reserved == other.Reserved
            && self.ImageSize == other.ImageSize
            && self.ImageBuffer == other.ImageBuffer
    }
}
impl ::std::cmp::Eq for STORAGE_FIRMWARE_DOWNLOAD_V2 {}
unsafe impl ::windows::runtime::Abi for STORAGE_FIRMWARE_DOWNLOAD_V2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_FIRMWARE_INFO {
    pub Version: u32,
    pub Size: u32,
    pub UpgradeSupport: super::super::Foundation::BOOLEAN,
    pub SlotCount: u8,
    pub ActiveSlot: u8,
    pub PendingActivateSlot: u8,
    pub Reserved: u32,
    pub Slot: [STORAGE_FIRMWARE_SLOT_INFO; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl STORAGE_FIRMWARE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for STORAGE_FIRMWARE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for STORAGE_FIRMWARE_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for STORAGE_FIRMWARE_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for STORAGE_FIRMWARE_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const STORAGE_FIRMWARE_INFO_INVALID_SLOT: u32 = 255u32;
pub const STORAGE_FIRMWARE_INFO_STRUCTURE_VERSION: u32 = 1u32;
pub const STORAGE_FIRMWARE_INFO_STRUCTURE_VERSION_V2: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_FIRMWARE_INFO_V2 {
    pub Version: u32,
    pub Size: u32,
    pub UpgradeSupport: super::super::Foundation::BOOLEAN,
    pub SlotCount: u8,
    pub ActiveSlot: u8,
    pub PendingActivateSlot: u8,
    pub FirmwareShared: super::super::Foundation::BOOLEAN,
    pub Reserved: [u8; 3],
    pub ImagePayloadAlignment: u32,
    pub ImagePayloadMaxSize: u32,
    pub Slot: [STORAGE_FIRMWARE_SLOT_INFO_V2; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl STORAGE_FIRMWARE_INFO_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for STORAGE_FIRMWARE_INFO_V2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for STORAGE_FIRMWARE_INFO_V2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STORAGE_FIRMWARE_INFO_V2")
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .field("UpgradeSupport", &self.UpgradeSupport)
            .field("SlotCount", &self.SlotCount)
            .field("ActiveSlot", &self.ActiveSlot)
            .field("PendingActivateSlot", &self.PendingActivateSlot)
            .field("FirmwareShared", &self.FirmwareShared)
            .field("Reserved", &self.Reserved)
            .field("ImagePayloadAlignment", &self.ImagePayloadAlignment)
            .field("ImagePayloadMaxSize", &self.ImagePayloadMaxSize)
            .field("Slot", &self.Slot)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for STORAGE_FIRMWARE_INFO_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Size == other.Size
            && self.UpgradeSupport == other.UpgradeSupport
            && self.SlotCount == other.SlotCount
            && self.ActiveSlot == other.ActiveSlot
            && self.PendingActivateSlot == other.PendingActivateSlot
            && self.FirmwareShared == other.FirmwareShared
            && self.Reserved == other.Reserved
            && self.ImagePayloadAlignment == other.ImagePayloadAlignment
            && self.ImagePayloadMaxSize == other.ImagePayloadMaxSize
            && self.Slot == other.Slot
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for STORAGE_FIRMWARE_INFO_V2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for STORAGE_FIRMWARE_INFO_V2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_FIRMWARE_SLOT_INFO {
    pub SlotNumber: u8,
    pub ReadOnly: super::super::Foundation::BOOLEAN,
    pub Reserved: [u8; 6],
    pub Revision: STORAGE_FIRMWARE_SLOT_INFO_0,
}
#[cfg(feature = "Win32_Foundation")]
impl STORAGE_FIRMWARE_SLOT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for STORAGE_FIRMWARE_SLOT_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for STORAGE_FIRMWARE_SLOT_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for STORAGE_FIRMWARE_SLOT_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for STORAGE_FIRMWARE_SLOT_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union STORAGE_FIRMWARE_SLOT_INFO_0 {
    pub Info: [u8; 8],
    pub AsUlonglong: u64,
}
impl STORAGE_FIRMWARE_SLOT_INFO_0 {}
impl ::std::default::Default for STORAGE_FIRMWARE_SLOT_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for STORAGE_FIRMWARE_SLOT_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for STORAGE_FIRMWARE_SLOT_INFO_0 {}
unsafe impl ::windows::runtime::Abi for STORAGE_FIRMWARE_SLOT_INFO_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_FIRMWARE_SLOT_INFO_V2 {
    pub SlotNumber: u8,
    pub ReadOnly: super::super::Foundation::BOOLEAN,
    pub Reserved: [u8; 6],
    pub Revision: [u8; 16],
}
#[cfg(feature = "Win32_Foundation")]
impl STORAGE_FIRMWARE_SLOT_INFO_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for STORAGE_FIRMWARE_SLOT_INFO_V2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for STORAGE_FIRMWARE_SLOT_INFO_V2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STORAGE_FIRMWARE_SLOT_INFO_V2")
            .field("SlotNumber", &self.SlotNumber)
            .field("ReadOnly", &self.ReadOnly)
            .field("Reserved", &self.Reserved)
            .field("Revision", &self.Revision)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for STORAGE_FIRMWARE_SLOT_INFO_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.SlotNumber == other.SlotNumber
            && self.ReadOnly == other.ReadOnly
            && self.Reserved == other.Reserved
            && self.Revision == other.Revision
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for STORAGE_FIRMWARE_SLOT_INFO_V2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for STORAGE_FIRMWARE_SLOT_INFO_V2 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const STORAGE_FIRMWARE_SLOT_INFO_V2_REVISION_LENGTH: u32 = 16u32;
pub const ScsiRawInterfaceGuid: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1408590601,
    46783,
    4560,
    [148, 242, 0, 160, 201, 30, 251, 139],
);
pub unsafe fn SendScsiInquiry(
    uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID,
    lun: u64,
    evpdcmddt: u8,
    pagecode: u8,
    scsistatus: *mut u8,
    responsesize: *mut u32,
    responsebuffer: *mut u8,
    sensesize: *mut u32,
    sensebuffer: *mut u8,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn SendScsiInquiry(
                uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID,
                lun: u64,
                evpdcmddt: u8,
                pagecode: u8,
                scsistatus: *mut u8,
                responsesize: *mut u32,
                responsebuffer: *mut u8,
                sensesize: *mut u32,
                sensebuffer: *mut u8,
            ) -> u32;
        }
        ::std::mem::transmute(SendScsiInquiry(
            ::std::mem::transmute(uniquesessionid),
            ::std::mem::transmute(lun),
            ::std::mem::transmute(evpdcmddt),
            ::std::mem::transmute(pagecode),
            ::std::mem::transmute(scsistatus),
            ::std::mem::transmute(responsesize),
            ::std::mem::transmute(responsebuffer),
            ::std::mem::transmute(sensesize),
            ::std::mem::transmute(sensebuffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SendScsiReadCapacity(
    uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID,
    lun: u64,
    scsistatus: *mut u8,
    responsesize: *mut u32,
    responsebuffer: *mut u8,
    sensesize: *mut u32,
    sensebuffer: *mut u8,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn SendScsiReadCapacity(
                uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID,
                lun: u64,
                scsistatus: *mut u8,
                responsesize: *mut u32,
                responsebuffer: *mut u8,
                sensesize: *mut u32,
                sensebuffer: *mut u8,
            ) -> u32;
        }
        ::std::mem::transmute(SendScsiReadCapacity(
            ::std::mem::transmute(uniquesessionid),
            ::std::mem::transmute(lun),
            ::std::mem::transmute(scsistatus),
            ::std::mem::transmute(responsesize),
            ::std::mem::transmute(responsebuffer),
            ::std::mem::transmute(sensesize),
            ::std::mem::transmute(sensebuffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SendScsiReportLuns(
    uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID,
    scsistatus: *mut u8,
    responsesize: *mut u32,
    responsebuffer: *mut u8,
    sensesize: *mut u32,
    sensebuffer: *mut u8,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn SendScsiReportLuns(
                uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID,
                scsistatus: *mut u8,
                responsesize: *mut u32,
                responsebuffer: *mut u8,
                sensesize: *mut u32,
                sensebuffer: *mut u8,
            ) -> u32;
        }
        ::std::mem::transmute(SendScsiReportLuns(
            ::std::mem::transmute(uniquesessionid),
            ::std::mem::transmute(scsistatus),
            ::std::mem::transmute(responsesize),
            ::std::mem::transmute(responsebuffer),
            ::std::mem::transmute(sensesize),
            ::std::mem::transmute(sensebuffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetIScsiGroupPresharedKey<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>,
>(
    keylength: u32,
    key: *mut u8,
    persist: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn SetIScsiGroupPresharedKey(
                keylength: u32,
                key: *mut u8,
                persist: super::super::Foundation::BOOLEAN,
            ) -> u32;
        }
        ::std::mem::transmute(SetIScsiGroupPresharedKey(
            ::std::mem::transmute(keylength),
            ::std::mem::transmute(key),
            persist.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetIScsiIKEInfoA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>,
>(
    initiatorname: Param0,
    initiatorportnumber: u32,
    authinfo: *mut IKE_AUTHENTICATION_INFORMATION,
    persist: Param3,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn SetIScsiIKEInfoA(
                initiatorname: super::super::Foundation::PSTR,
                initiatorportnumber: u32,
                authinfo: *mut IKE_AUTHENTICATION_INFORMATION,
                persist: super::super::Foundation::BOOLEAN,
            ) -> u32;
        }
        ::std::mem::transmute(SetIScsiIKEInfoA(
            initiatorname.into_param().abi(),
            ::std::mem::transmute(initiatorportnumber),
            ::std::mem::transmute(authinfo),
            persist.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetIScsiIKEInfoW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>,
>(
    initiatorname: Param0,
    initiatorportnumber: u32,
    authinfo: *mut IKE_AUTHENTICATION_INFORMATION,
    persist: Param3,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn SetIScsiIKEInfoW(
                initiatorname: super::super::Foundation::PWSTR,
                initiatorportnumber: u32,
                authinfo: *mut IKE_AUTHENTICATION_INFORMATION,
                persist: super::super::Foundation::BOOLEAN,
            ) -> u32;
        }
        ::std::mem::transmute(SetIScsiIKEInfoW(
            initiatorname.into_param().abi(),
            ::std::mem::transmute(initiatorportnumber),
            ::std::mem::transmute(authinfo),
            persist.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SetIScsiInitiatorCHAPSharedSecret(
    sharedsecretlength: u32,
    sharedsecret: *mut u8,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn SetIScsiInitiatorCHAPSharedSecret(
                sharedsecretlength: u32,
                sharedsecret: *mut u8,
            ) -> u32;
        }
        ::std::mem::transmute(SetIScsiInitiatorCHAPSharedSecret(
            ::std::mem::transmute(sharedsecretlength),
            ::std::mem::transmute(sharedsecret),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetIScsiInitiatorNodeNameA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    initiatornodename: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn SetIScsiInitiatorNodeNameA(initiatornodename: super::super::Foundation::PSTR)
                -> u32;
        }
        ::std::mem::transmute(SetIScsiInitiatorNodeNameA(
            initiatornodename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetIScsiInitiatorNodeNameW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    initiatornodename: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn SetIScsiInitiatorNodeNameW(
                initiatornodename: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(SetIScsiInitiatorNodeNameW(
            initiatornodename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SetIScsiInitiatorRADIUSSharedSecret(
    sharedsecretlength: u32,
    sharedsecret: *mut u8,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn SetIScsiInitiatorRADIUSSharedSecret(
                sharedsecretlength: u32,
                sharedsecret: *mut u8,
            ) -> u32;
        }
        ::std::mem::transmute(SetIScsiInitiatorRADIUSSharedSecret(
            ::std::mem::transmute(sharedsecretlength),
            ::std::mem::transmute(sharedsecret),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetIScsiTunnelModeOuterAddressA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>,
>(
    initiatorname: Param0,
    initiatorportnumber: u32,
    destinationaddress: Param2,
    outermodeaddress: Param3,
    persist: Param4,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn SetIScsiTunnelModeOuterAddressA(
                initiatorname: super::super::Foundation::PSTR,
                initiatorportnumber: u32,
                destinationaddress: super::super::Foundation::PSTR,
                outermodeaddress: super::super::Foundation::PSTR,
                persist: super::super::Foundation::BOOLEAN,
            ) -> u32;
        }
        ::std::mem::transmute(SetIScsiTunnelModeOuterAddressA(
            initiatorname.into_param().abi(),
            ::std::mem::transmute(initiatorportnumber),
            destinationaddress.into_param().abi(),
            outermodeaddress.into_param().abi(),
            persist.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetIScsiTunnelModeOuterAddressW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>,
>(
    initiatorname: Param0,
    initiatorportnumber: u32,
    destinationaddress: Param2,
    outermodeaddress: Param3,
    persist: Param4,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn SetIScsiTunnelModeOuterAddressW(
                initiatorname: super::super::Foundation::PWSTR,
                initiatorportnumber: u32,
                destinationaddress: super::super::Foundation::PWSTR,
                outermodeaddress: super::super::Foundation::PWSTR,
                persist: super::super::Foundation::BOOLEAN,
            ) -> u32;
        }
        ::std::mem::transmute(SetIScsiTunnelModeOuterAddressW(
            initiatorname.into_param().abi(),
            ::std::mem::transmute(initiatorportnumber),
            destinationaddress.into_param().abi(),
            outermodeaddress.into_param().abi(),
            persist.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SetupPersistentIScsiDevices() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn SetupPersistentIScsiDevices() -> u32;
        }
        ::std::mem::transmute(SetupPersistentIScsiDevices())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SetupPersistentIScsiVolumes() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "iscsidsc")]
        extern "system" {
            fn SetupPersistentIScsiVolumes() -> u32;
        }
        ::std::mem::transmute(SetupPersistentIScsiVolumes())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct TARGETPROTOCOLTYPE(pub i32);
pub const ISCSI_TCP_PROTOCOL_TYPE: TARGETPROTOCOLTYPE = TARGETPROTOCOLTYPE(0i32);
impl ::std::convert::From<i32> for TARGETPROTOCOLTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TARGETPROTOCOLTYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct TARGET_INFORMATION_CLASS(pub i32);
pub const ProtocolType: TARGET_INFORMATION_CLASS = TARGET_INFORMATION_CLASS(0i32);
pub const TargetAlias: TARGET_INFORMATION_CLASS = TARGET_INFORMATION_CLASS(1i32);
pub const DiscoveryMechanisms: TARGET_INFORMATION_CLASS = TARGET_INFORMATION_CLASS(2i32);
pub const PortalGroups: TARGET_INFORMATION_CLASS = TARGET_INFORMATION_CLASS(3i32);
pub const PersistentTargetMappings: TARGET_INFORMATION_CLASS = TARGET_INFORMATION_CLASS(4i32);
pub const InitiatorName: TARGET_INFORMATION_CLASS = TARGET_INFORMATION_CLASS(5i32);
pub const TargetFlags: TARGET_INFORMATION_CLASS = TARGET_INFORMATION_CLASS(6i32);
pub const LoginOptions: TARGET_INFORMATION_CLASS = TARGET_INFORMATION_CLASS(7i32);
impl ::std::convert::From<i32> for TARGET_INFORMATION_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TARGET_INFORMATION_CLASS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WmiScsiAddressGuid: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1408590607,
    46783,
    4560,
    [148, 242, 0, 160, 201, 30, 251, 139],
);
#[repr(C)]
#[derive(
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
)]
pub struct _ADAPTER_OBJECT(pub u8);
