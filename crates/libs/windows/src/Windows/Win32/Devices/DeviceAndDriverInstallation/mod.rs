#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ALLOC_LOG_CONF: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const BASIC_LOG_CONF: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const BOOT_LOG_CONF: u32 = 3u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct BUSNUMBER_DES {
    pub BUSD_Count: u32,
    pub BUSD_Type: u32,
    pub BUSD_Flags: u32,
    pub BUSD_Alloc_Base: u32,
    pub BUSD_Alloc_End: u32,
}
impl ::core::marker::Copy for BUSNUMBER_DES {}
impl ::core::clone::Clone for BUSNUMBER_DES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BUSNUMBER_DES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BUSNUMBER_DES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BUSNUMBER_DES>()) == 0 }
    }
}
impl ::core::cmp::Eq for BUSNUMBER_DES {}
impl ::core::default::Default for BUSNUMBER_DES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct BUSNUMBER_RANGE {
    pub BUSR_Min: u32,
    pub BUSR_Max: u32,
    pub BUSR_nBusNumbers: u32,
    pub BUSR_Flags: u32,
}
impl ::core::marker::Copy for BUSNUMBER_RANGE {}
impl ::core::clone::Clone for BUSNUMBER_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BUSNUMBER_RANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BUSNUMBER_RANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BUSNUMBER_RANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for BUSNUMBER_RANGE {}
impl ::core::default::Default for BUSNUMBER_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct BUSNUMBER_RESOURCE {
    pub BusNumber_Header: BUSNUMBER_DES,
    pub BusNumber_Data: [BUSNUMBER_RANGE; 1],
}
impl ::core::marker::Copy for BUSNUMBER_RESOURCE {}
impl ::core::clone::Clone for BUSNUMBER_RESOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BUSNUMBER_RESOURCE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BUSNUMBER_RESOURCE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BUSNUMBER_RESOURCE>()) == 0 }
    }
}
impl ::core::cmp::Eq for BUSNUMBER_RESOURCE {}
impl ::core::default::Default for BUSNUMBER_RESOURCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct CABINET_INFO_A {
    pub CabinetPath: ::windows::core::PCSTR,
    pub CabinetFile: ::windows::core::PCSTR,
    pub DiskName: ::windows::core::PCSTR,
    pub SetId: u16,
    pub CabinetNumber: u16,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for CABINET_INFO_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for CABINET_INFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for CABINET_INFO_A {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for CABINET_INFO_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CABINET_INFO_A>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for CABINET_INFO_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for CABINET_INFO_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct CABINET_INFO_A {
    pub CabinetPath: ::windows::core::PCSTR,
    pub CabinetFile: ::windows::core::PCSTR,
    pub DiskName: ::windows::core::PCSTR,
    pub SetId: u16,
    pub CabinetNumber: u16,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for CABINET_INFO_A {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for CABINET_INFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for CABINET_INFO_A {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for CABINET_INFO_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CABINET_INFO_A>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for CABINET_INFO_A {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for CABINET_INFO_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct CABINET_INFO_W {
    pub CabinetPath: ::windows::core::PCWSTR,
    pub CabinetFile: ::windows::core::PCWSTR,
    pub DiskName: ::windows::core::PCWSTR,
    pub SetId: u16,
    pub CabinetNumber: u16,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for CABINET_INFO_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for CABINET_INFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for CABINET_INFO_W {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for CABINET_INFO_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CABINET_INFO_W>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for CABINET_INFO_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for CABINET_INFO_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct CABINET_INFO_W {
    pub CabinetPath: ::windows::core::PCWSTR,
    pub CabinetFile: ::windows::core::PCWSTR,
    pub DiskName: ::windows::core::PCWSTR,
    pub SetId: u16,
    pub CabinetNumber: u16,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for CABINET_INFO_W {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for CABINET_INFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for CABINET_INFO_W {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for CABINET_INFO_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CABINET_INFO_W>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for CABINET_INFO_W {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for CABINET_INFO_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CMP_WaitNoPendingInstallEvents(dwtimeout: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMP_WaitNoPendingInstallEvents(dwtimeout: u32) -> u32;
        }
        ::core::mem::transmute(CMP_WaitNoPendingInstallEvents(::core::mem::transmute(dwtimeout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_ADD_ID_BITS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_ADD_ID_COMPATIBLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_ADD_ID_HARDWARE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_ADD_RANGE_ADDIFCONFLICT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_ADD_RANGE_BITS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_ADD_RANGE_DONOTADDIFCONFLICT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Data_HtmlHelp\"`*"]
#[cfg(feature = "Win32_Data_HtmlHelp")]
#[inline]
pub unsafe fn CM_Add_Empty_Log_Conf(plclogconf: *mut usize, dndevinst: u32, priority: super::super::Data::HtmlHelp::PRIORITY, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Add_Empty_Log_Conf(plclogconf: *mut usize, dndevinst: u32, priority: super::super::Data::HtmlHelp::PRIORITY, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Add_Empty_Log_Conf(::core::mem::transmute(plclogconf), ::core::mem::transmute(dndevinst), ::core::mem::transmute(priority), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Data_HtmlHelp\"`*"]
#[cfg(feature = "Win32_Data_HtmlHelp")]
#[inline]
pub unsafe fn CM_Add_Empty_Log_Conf_Ex(plclogconf: *mut usize, dndevinst: u32, priority: super::super::Data::HtmlHelp::PRIORITY, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Add_Empty_Log_Conf_Ex(plclogconf: *mut usize, dndevinst: u32, priority: super::super::Data::HtmlHelp::PRIORITY, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Add_Empty_Log_Conf_Ex(::core::mem::transmute(plclogconf), ::core::mem::transmute(dndevinst), ::core::mem::transmute(priority), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Add_IDA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(dndevinst: u32, pszid: Param1, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Add_IDA(dndevinst: u32, pszid: ::windows::core::PCSTR, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Add_IDA(::core::mem::transmute(dndevinst), pszid.into_param().abi(), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Add_IDW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(dndevinst: u32, pszid: Param1, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Add_IDW(dndevinst: u32, pszid: ::windows::core::PCWSTR, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Add_IDW(::core::mem::transmute(dndevinst), pszid.into_param().abi(), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Add_ID_ExA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(dndevinst: u32, pszid: Param1, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Add_ID_ExA(dndevinst: u32, pszid: ::windows::core::PCSTR, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Add_ID_ExA(::core::mem::transmute(dndevinst), pszid.into_param().abi(), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Add_ID_ExW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(dndevinst: u32, pszid: Param1, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Add_ID_ExW(dndevinst: u32, pszid: ::windows::core::PCWSTR, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Add_ID_ExW(::core::mem::transmute(dndevinst), pszid.into_param().abi(), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Add_Range(ullstartvalue: u64, ullendvalue: u64, rlh: usize, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Add_Range(ullstartvalue: u64, ullendvalue: u64, rlh: usize, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Add_Range(::core::mem::transmute(ullstartvalue), ::core::mem::transmute(ullendvalue), ::core::mem::transmute(rlh), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Add_Res_Des(prdresdes: *mut usize, lclogconf: usize, resourceid: u32, resourcedata: *const ::core::ffi::c_void, resourcelen: u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Add_Res_Des(prdresdes: *mut usize, lclogconf: usize, resourceid: u32, resourcedata: *const ::core::ffi::c_void, resourcelen: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Add_Res_Des(::core::mem::transmute(prdresdes), ::core::mem::transmute(lclogconf), ::core::mem::transmute(resourceid), ::core::mem::transmute(resourcedata), ::core::mem::transmute(resourcelen), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Add_Res_Des_Ex(prdresdes: *mut usize, lclogconf: usize, resourceid: u32, resourcedata: *const ::core::ffi::c_void, resourcelen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Add_Res_Des_Ex(prdresdes: *mut usize, lclogconf: usize, resourceid: u32, resourcedata: *const ::core::ffi::c_void, resourcelen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Add_Res_Des_Ex(::core::mem::transmute(prdresdes), ::core::mem::transmute(lclogconf), ::core::mem::transmute(resourceid), ::core::mem::transmute(resourcedata), ::core::mem::transmute(resourcelen), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CDFLAGS_DRIVER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CDFLAGS_RESERVED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CDFLAGS_ROOT_OWNED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CDMASK_DESCRIPTION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CDMASK_DEVINST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CDMASK_FLAGS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CDMASK_RESDES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CDMASK_VALID: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CLASS_PROPERTY_BITS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CLASS_PROPERTY_INSTALLER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CLASS_PROPERTY_INTERFACE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CREATE_DEVINST_BITS: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CREATE_DEVINST_DO_NOT_INSTALL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CREATE_DEVINST_GENERATE_ID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CREATE_DEVINST_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CREATE_DEVINST_NO_WAIT_INSTALL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CREATE_DEVINST_PHANTOM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CREATE_DEVNODE_BITS: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CREATE_DEVNODE_DO_NOT_INSTALL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CREATE_DEVNODE_GENERATE_ID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CREATE_DEVNODE_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CREATE_DEVNODE_NO_WAIT_INSTALL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CREATE_DEVNODE_PHANTOM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CRP_CHARACTERISTICS: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CRP_DEVTYPE: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CRP_EXCLUSIVE: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CRP_LOWERFILTERS: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CRP_MAX: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CRP_MIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CRP_SECURITY: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CRP_SECURITY_SDS: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CRP_UPPERFILTERS: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CUSTOMDEVPROP_BITS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CUSTOMDEVPROP_MERGE_MULTISZ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Connect_MachineA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(uncservername: Param0, phmachine: *mut isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Connect_MachineA(uncservername: ::windows::core::PCSTR, phmachine: *mut isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Connect_MachineA(uncservername.into_param().abi(), ::core::mem::transmute(phmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Connect_MachineW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(uncservername: Param0, phmachine: *mut isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Connect_MachineW(uncservername: ::windows::core::PCWSTR, phmachine: *mut isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Connect_MachineW(uncservername.into_param().abi(), ::core::mem::transmute(phmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Create_DevNodeA(pdndevinst: *mut u32, pdeviceid: *const i8, dnparent: u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Create_DevNodeA(pdndevinst: *mut u32, pdeviceid: *const i8, dnparent: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Create_DevNodeA(::core::mem::transmute(pdndevinst), ::core::mem::transmute(pdeviceid), ::core::mem::transmute(dnparent), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Create_DevNodeW(pdndevinst: *mut u32, pdeviceid: *const u16, dnparent: u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Create_DevNodeW(pdndevinst: *mut u32, pdeviceid: *const u16, dnparent: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Create_DevNodeW(::core::mem::transmute(pdndevinst), ::core::mem::transmute(pdeviceid), ::core::mem::transmute(dnparent), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Create_DevNode_ExA(pdndevinst: *mut u32, pdeviceid: *const i8, dnparent: u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Create_DevNode_ExA(pdndevinst: *mut u32, pdeviceid: *const i8, dnparent: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Create_DevNode_ExA(::core::mem::transmute(pdndevinst), ::core::mem::transmute(pdeviceid), ::core::mem::transmute(dnparent), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Create_DevNode_ExW(pdndevinst: *mut u32, pdeviceid: *const u16, dnparent: u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Create_DevNode_ExW(pdndevinst: *mut u32, pdeviceid: *const u16, dnparent: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Create_DevNode_ExW(::core::mem::transmute(pdndevinst), ::core::mem::transmute(pdeviceid), ::core::mem::transmute(dnparent), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Create_Range_List(prlh: *mut usize, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Create_Range_List(prlh: *mut usize, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Create_Range_List(::core::mem::transmute(prlh), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DELETE_CLASS_BITS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DELETE_CLASS_INTERFACE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DELETE_CLASS_ONLY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DELETE_CLASS_SUBKEYS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DETECT_BITS: u32 = 2147483655u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DETECT_CRASHED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DETECT_HWPROF_FIRST_BOOT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DETECT_NEW_PROFILE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DETECT_RUN: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVCAP_DOCKDEVICE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVCAP_EJECTSUPPORTED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVCAP_HARDWAREDISABLED: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVCAP_LOCKSUPPORTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVCAP_NONDYNAMIC: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVCAP_RAWDEVICEOK: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVCAP_REMOVABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVCAP_SECUREDEVICE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVCAP_SILENTINSTALL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVCAP_SURPRISEREMOVALOK: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVCAP_UNIQUEID: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_EDGE_BOTTOM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_EDGE_LEFT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_EDGE_RIGHT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_EDGE_TOP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_EDGE_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_JOINT_TYPE_HINGE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_JOINT_TYPE_PIVOT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_JOINT_TYPE_PLANAR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_JOINT_TYPE_SWIVEL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_JOINT_TYPE_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_ORIENTATION_HORIZONTAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_ORIENTATION_VERTICAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_SHAPE_OVAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_SHAPE_RECTANGLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_SHAPE_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_SIDE_BACK: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_SIDE_BOTTOM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_SIDE_FRONT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_SIDE_LEFT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_SIDE_RIGHT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_SIDE_TOP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_SIDE_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DISABLE_ABSOLUTE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DISABLE_BITS: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DISABLE_HARDWARE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DISABLE_PERSIST: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DISABLE_POLITE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DISABLE_UI_NOT_OK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_ADDRESS: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_BASE_CONTAINERID: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_BUSNUMBER: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_BUSTYPEGUID: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_CAPABILITIES: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_CHARACTERISTICS: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_CLASS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_CLASSGUID: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_COMPATIBLEIDS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_CONFIGFLAGS: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_DEVICEDESC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_DEVICE_POWER_DATA: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_DEVTYPE: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_DRIVER: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_ENUMERATOR_NAME: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_EXCLUSIVE: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_FRIENDLYNAME: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_HARDWAREID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_INSTALL_STATE: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_LEGACYBUSTYPE: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_LOCATION_INFORMATION: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_LOCATION_PATHS: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_LOWERFILTERS: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_MAX: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_MFG: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_MIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_PHYSICAL_DEVICE_OBJECT_NAME: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_REMOVAL_POLICY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_REMOVAL_POLICY_HW_DEFAULT: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_REMOVAL_POLICY_OVERRIDE: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_SECURITY: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_SECURITY_SDS: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_SERVICE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_UI_NUMBER: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_UI_NUMBER_DESC_FORMAT: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_UNUSED0: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_UNUSED1: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_UNUSED2: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_UPPERFILTERS: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Delete_Class_Key(classguid: *const ::windows::core::GUID, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Delete_Class_Key(classguid: *const ::windows::core::GUID, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Delete_Class_Key(::core::mem::transmute(classguid), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Delete_Class_Key_Ex(classguid: *const ::windows::core::GUID, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Delete_Class_Key_Ex(classguid: *const ::windows::core::GUID, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Delete_Class_Key_Ex(::core::mem::transmute(classguid), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Delete_DevNode_Key(dndevnode: u32, ulhardwareprofile: u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Delete_DevNode_Key(dndevnode: u32, ulhardwareprofile: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Delete_DevNode_Key(::core::mem::transmute(dndevnode), ::core::mem::transmute(ulhardwareprofile), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Delete_DevNode_Key_Ex(dndevnode: u32, ulhardwareprofile: u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Delete_DevNode_Key_Ex(dndevnode: u32, ulhardwareprofile: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Delete_DevNode_Key_Ex(::core::mem::transmute(dndevnode), ::core::mem::transmute(ulhardwareprofile), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Delete_Device_Interface_KeyA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(pszdeviceinterface: Param0, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Delete_Device_Interface_KeyA(pszdeviceinterface: ::windows::core::PCSTR, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Delete_Device_Interface_KeyA(pszdeviceinterface.into_param().abi(), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Delete_Device_Interface_KeyW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pszdeviceinterface: Param0, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Delete_Device_Interface_KeyW(pszdeviceinterface: ::windows::core::PCWSTR, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Delete_Device_Interface_KeyW(pszdeviceinterface.into_param().abi(), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Delete_Device_Interface_Key_ExA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(pszdeviceinterface: Param0, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Delete_Device_Interface_Key_ExA(pszdeviceinterface: ::windows::core::PCSTR, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Delete_Device_Interface_Key_ExA(pszdeviceinterface.into_param().abi(), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Delete_Device_Interface_Key_ExW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pszdeviceinterface: Param0, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Delete_Device_Interface_Key_ExW(pszdeviceinterface: ::windows::core::PCWSTR, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Delete_Device_Interface_Key_ExW(pszdeviceinterface.into_param().abi(), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Delete_Range(ullstartvalue: u64, ullendvalue: u64, rlh: usize, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Delete_Range(ullstartvalue: u64, ullendvalue: u64, rlh: usize, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Delete_Range(::core::mem::transmute(ullstartvalue), ::core::mem::transmute(ullendvalue), ::core::mem::transmute(rlh), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CM_Detect_Resource_Conflict(dndevinst: u32, resourceid: u32, resourcedata: *const ::core::ffi::c_void, resourcelen: u32, pbconflictdetected: *mut super::super::Foundation::BOOL, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Detect_Resource_Conflict(dndevinst: u32, resourceid: u32, resourcedata: *const ::core::ffi::c_void, resourcelen: u32, pbconflictdetected: *mut super::super::Foundation::BOOL, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Detect_Resource_Conflict(::core::mem::transmute(dndevinst), ::core::mem::transmute(resourceid), ::core::mem::transmute(resourcedata), ::core::mem::transmute(resourcelen), ::core::mem::transmute(pbconflictdetected), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CM_Detect_Resource_Conflict_Ex(dndevinst: u32, resourceid: u32, resourcedata: *const ::core::ffi::c_void, resourcelen: u32, pbconflictdetected: *mut super::super::Foundation::BOOL, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Detect_Resource_Conflict_Ex(dndevinst: u32, resourceid: u32, resourcedata: *const ::core::ffi::c_void, resourcelen: u32, pbconflictdetected: *mut super::super::Foundation::BOOL, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Detect_Resource_Conflict_Ex(::core::mem::transmute(dndevinst), ::core::mem::transmute(resourceid), ::core::mem::transmute(resourcedata), ::core::mem::transmute(resourcelen), ::core::mem::transmute(pbconflictdetected), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Disable_DevNode(dndevinst: u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Disable_DevNode(dndevinst: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Disable_DevNode(::core::mem::transmute(dndevinst), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Disable_DevNode_Ex(dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Disable_DevNode_Ex(dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Disable_DevNode_Ex(::core::mem::transmute(dndevinst), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Disconnect_Machine(hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Disconnect_Machine(hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Disconnect_Machine(::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Dup_Range_List(rlhold: usize, rlhnew: usize, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Dup_Range_List(rlhold: usize, rlhnew: usize, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Dup_Range_List(::core::mem::transmute(rlhold), ::core::mem::transmute(rlhnew), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_ENUMERATE_CLASSES_BITS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_ENUMERATE_CLASSES_INSTALLER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_ENUMERATE_CLASSES_INTERFACE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Enable_DevNode(dndevinst: u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Enable_DevNode(dndevinst: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Enable_DevNode(::core::mem::transmute(dndevinst), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Enable_DevNode_Ex(dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Enable_DevNode_Ex(dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Enable_DevNode_Ex(::core::mem::transmute(dndevinst), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Enumerate_Classes(ulclassindex: u32, classguid: *mut ::windows::core::GUID, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Enumerate_Classes(ulclassindex: u32, classguid: *mut ::windows::core::GUID, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Enumerate_Classes(::core::mem::transmute(ulclassindex), ::core::mem::transmute(classguid), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Enumerate_Classes_Ex(ulclassindex: u32, classguid: *mut ::windows::core::GUID, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Enumerate_Classes_Ex(ulclassindex: u32, classguid: *mut ::windows::core::GUID, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Enumerate_Classes_Ex(::core::mem::transmute(ulclassindex), ::core::mem::transmute(classguid), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Enumerate_EnumeratorsA(ulenumindex: u32, buffer: ::windows::core::PSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Enumerate_EnumeratorsA(ulenumindex: u32, buffer: ::windows::core::PSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Enumerate_EnumeratorsA(::core::mem::transmute(ulenumindex), ::core::mem::transmute(buffer), ::core::mem::transmute(pullength), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Enumerate_EnumeratorsW(ulenumindex: u32, buffer: ::windows::core::PWSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Enumerate_EnumeratorsW(ulenumindex: u32, buffer: ::windows::core::PWSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Enumerate_EnumeratorsW(::core::mem::transmute(ulenumindex), ::core::mem::transmute(buffer), ::core::mem::transmute(pullength), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Enumerate_Enumerators_ExA(ulenumindex: u32, buffer: ::windows::core::PSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Enumerate_Enumerators_ExA(ulenumindex: u32, buffer: ::windows::core::PSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Enumerate_Enumerators_ExA(::core::mem::transmute(ulenumindex), ::core::mem::transmute(buffer), ::core::mem::transmute(pullength), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Enumerate_Enumerators_ExW(ulenumindex: u32, buffer: ::windows::core::PWSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Enumerate_Enumerators_ExW(ulenumindex: u32, buffer: ::windows::core::PWSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Enumerate_Enumerators_ExW(::core::mem::transmute(ulenumindex), ::core::mem::transmute(buffer), ::core::mem::transmute(pullength), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Find_Range(pullstart: *mut u64, ullstart: u64, ullength: u32, ullalignment: u64, ullend: u64, rlh: usize, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Find_Range(pullstart: *mut u64, ullstart: u64, ullength: u32, ullalignment: u64, ullend: u64, rlh: usize, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Find_Range(::core::mem::transmute(pullstart), ::core::mem::transmute(ullstart), ::core::mem::transmute(ullength), ::core::mem::transmute(ullalignment), ::core::mem::transmute(ullend), ::core::mem::transmute(rlh), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_First_Range(rlh: usize, pullstart: *mut u64, pullend: *mut u64, preelement: *mut usize, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_First_Range(rlh: usize, pullstart: *mut u64, pullend: *mut u64, preelement: *mut usize, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_First_Range(::core::mem::transmute(rlh), ::core::mem::transmute(pullstart), ::core::mem::transmute(pullend), ::core::mem::transmute(preelement), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Free_Log_Conf(lclogconftobefreed: usize, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Free_Log_Conf(lclogconftobefreed: usize, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Free_Log_Conf(::core::mem::transmute(lclogconftobefreed), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Free_Log_Conf_Ex(lclogconftobefreed: usize, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Free_Log_Conf_Ex(lclogconftobefreed: usize, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Free_Log_Conf_Ex(::core::mem::transmute(lclogconftobefreed), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Free_Log_Conf_Handle(lclogconf: usize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Free_Log_Conf_Handle(lclogconf: usize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Free_Log_Conf_Handle(::core::mem::transmute(lclogconf)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Free_Range_List(rlh: usize, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Free_Range_List(rlh: usize, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Free_Range_List(::core::mem::transmute(rlh), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Free_Res_Des(prdresdes: *mut usize, rdresdes: usize, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Free_Res_Des(prdresdes: *mut usize, rdresdes: usize, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Free_Res_Des(::core::mem::transmute(prdresdes), ::core::mem::transmute(rdresdes), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Free_Res_Des_Ex(prdresdes: *mut usize, rdresdes: usize, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Free_Res_Des_Ex(prdresdes: *mut usize, rdresdes: usize, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Free_Res_Des_Ex(::core::mem::transmute(prdresdes), ::core::mem::transmute(rdresdes), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Free_Res_Des_Handle(rdresdes: usize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Free_Res_Des_Handle(rdresdes: usize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Free_Res_Des_Handle(::core::mem::transmute(rdresdes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Free_Resource_Conflict_Handle(clconflictlist: usize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Free_Resource_Conflict_Handle(clconflictlist: usize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Free_Resource_Conflict_Handle(::core::mem::transmute(clconflictlist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GETIDLIST_DONOTGENERATE: u32 = 268435520u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GETIDLIST_FILTER_BITS: u32 = 268435583u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GETIDLIST_FILTER_BUSRELATIONS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GETIDLIST_FILTER_CLASS: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GETIDLIST_FILTER_EJECTRELATIONS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GETIDLIST_FILTER_ENUMERATOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GETIDLIST_FILTER_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GETIDLIST_FILTER_POWERRELATIONS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GETIDLIST_FILTER_PRESENT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GETIDLIST_FILTER_REMOVALRELATIONS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GETIDLIST_FILTER_SERVICE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GETIDLIST_FILTER_TRANSPORTRELATIONS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GET_DEVICE_INTERFACE_LIST_ALL_DEVICES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GET_DEVICE_INTERFACE_LIST_BITS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GET_DEVICE_INTERFACE_LIST_PRESENT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GLOBAL_STATE_CAN_DO_UI: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GLOBAL_STATE_DETECTION_PENDING: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GLOBAL_STATE_ON_BIG_STACK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GLOBAL_STATE_REBOOT_REQUIRED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GLOBAL_STATE_SERVICES_AVAILABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GLOBAL_STATE_SHUTTING_DOWN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Child(pdndevinst: *mut u32, dndevinst: u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Child(pdndevinst: *mut u32, dndevinst: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Child(::core::mem::transmute(pdndevinst), ::core::mem::transmute(dndevinst), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Child_Ex(pdndevinst: *mut u32, dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Child_Ex(pdndevinst: *mut u32, dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Child_Ex(::core::mem::transmute(pdndevinst), ::core::mem::transmute(dndevinst), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Class_Key_NameA(classguid: *const ::windows::core::GUID, pszkeyname: ::windows::core::PSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Class_Key_NameA(classguid: *const ::windows::core::GUID, pszkeyname: ::windows::core::PSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Class_Key_NameA(::core::mem::transmute(classguid), ::core::mem::transmute(pszkeyname), ::core::mem::transmute(pullength), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Class_Key_NameW(classguid: *const ::windows::core::GUID, pszkeyname: ::windows::core::PWSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Class_Key_NameW(classguid: *const ::windows::core::GUID, pszkeyname: ::windows::core::PWSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Class_Key_NameW(::core::mem::transmute(classguid), ::core::mem::transmute(pszkeyname), ::core::mem::transmute(pullength), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Class_Key_Name_ExA(classguid: *const ::windows::core::GUID, pszkeyname: ::windows::core::PSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Class_Key_Name_ExA(classguid: *const ::windows::core::GUID, pszkeyname: ::windows::core::PSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Class_Key_Name_ExA(::core::mem::transmute(classguid), ::core::mem::transmute(pszkeyname), ::core::mem::transmute(pullength), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Class_Key_Name_ExW(classguid: *const ::windows::core::GUID, pszkeyname: ::windows::core::PWSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Class_Key_Name_ExW(classguid: *const ::windows::core::GUID, pszkeyname: ::windows::core::PWSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Class_Key_Name_ExW(::core::mem::transmute(classguid), ::core::mem::transmute(pszkeyname), ::core::mem::transmute(pullength), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Class_NameA(classguid: *const ::windows::core::GUID, buffer: ::windows::core::PSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Class_NameA(classguid: *const ::windows::core::GUID, buffer: ::windows::core::PSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Class_NameA(::core::mem::transmute(classguid), ::core::mem::transmute(buffer), ::core::mem::transmute(pullength), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Class_NameW(classguid: *const ::windows::core::GUID, buffer: ::windows::core::PWSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Class_NameW(classguid: *const ::windows::core::GUID, buffer: ::windows::core::PWSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Class_NameW(::core::mem::transmute(classguid), ::core::mem::transmute(buffer), ::core::mem::transmute(pullength), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Class_Name_ExA(classguid: *const ::windows::core::GUID, buffer: ::windows::core::PSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Class_Name_ExA(classguid: *const ::windows::core::GUID, buffer: ::windows::core::PSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Class_Name_ExA(::core::mem::transmute(classguid), ::core::mem::transmute(buffer), ::core::mem::transmute(pullength), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Class_Name_ExW(classguid: *const ::windows::core::GUID, buffer: ::windows::core::PWSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Class_Name_ExW(classguid: *const ::windows::core::GUID, buffer: ::windows::core::PWSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Class_Name_ExW(::core::mem::transmute(classguid), ::core::mem::transmute(buffer), ::core::mem::transmute(pullength), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn CM_Get_Class_PropertyW(classguid: *const ::windows::core::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: *mut u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Class_PropertyW(classguid: *const ::windows::core::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: *mut u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Class_PropertyW(::core::mem::transmute(classguid), ::core::mem::transmute(propertykey), ::core::mem::transmute(propertytype), ::core::mem::transmute(propertybuffer), ::core::mem::transmute(propertybuffersize), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn CM_Get_Class_Property_ExW(classguid: *const ::windows::core::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Class_Property_ExW(classguid: *const ::windows::core::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Class_Property_ExW(::core::mem::transmute(classguid), ::core::mem::transmute(propertykey), ::core::mem::transmute(propertytype), ::core::mem::transmute(propertybuffer), ::core::mem::transmute(propertybuffersize), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn CM_Get_Class_Property_Keys(classguid: *const ::windows::core::GUID, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: *mut u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Class_Property_Keys(classguid: *const ::windows::core::GUID, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: *mut u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Class_Property_Keys(::core::mem::transmute(classguid), ::core::mem::transmute(propertykeyarray), ::core::mem::transmute(propertykeycount), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn CM_Get_Class_Property_Keys_Ex(classguid: *const ::windows::core::GUID, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Class_Property_Keys_Ex(classguid: *const ::windows::core::GUID, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Class_Property_Keys_Ex(::core::mem::transmute(classguid), ::core::mem::transmute(propertykeyarray), ::core::mem::transmute(propertykeycount), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Class_Registry_PropertyA(classguid: *const ::windows::core::GUID, ulproperty: u32, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Class_Registry_PropertyA(classguid: *const ::windows::core::GUID, ulproperty: u32, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Class_Registry_PropertyA(::core::mem::transmute(classguid), ::core::mem::transmute(ulproperty), ::core::mem::transmute(pulregdatatype), ::core::mem::transmute(buffer), ::core::mem::transmute(pullength), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Class_Registry_PropertyW(classguid: *const ::windows::core::GUID, ulproperty: u32, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Class_Registry_PropertyW(classguid: *const ::windows::core::GUID, ulproperty: u32, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Class_Registry_PropertyW(::core::mem::transmute(classguid), ::core::mem::transmute(ulproperty), ::core::mem::transmute(pulregdatatype), ::core::mem::transmute(buffer), ::core::mem::transmute(pullength), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Depth(puldepth: *mut u32, dndevinst: u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Depth(puldepth: *mut u32, dndevinst: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Depth(::core::mem::transmute(puldepth), ::core::mem::transmute(dndevinst), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Depth_Ex(puldepth: *mut u32, dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Depth_Ex(puldepth: *mut u32, dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Depth_Ex(::core::mem::transmute(puldepth), ::core::mem::transmute(dndevinst), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_DevNode_Custom_PropertyA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(dndevinst: u32, pszcustompropertyname: Param1, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_DevNode_Custom_PropertyA(dndevinst: u32, pszcustompropertyname: ::windows::core::PCSTR, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_DevNode_Custom_PropertyA(::core::mem::transmute(dndevinst), pszcustompropertyname.into_param().abi(), ::core::mem::transmute(pulregdatatype), ::core::mem::transmute(buffer), ::core::mem::transmute(pullength), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_DevNode_Custom_PropertyW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(dndevinst: u32, pszcustompropertyname: Param1, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_DevNode_Custom_PropertyW(dndevinst: u32, pszcustompropertyname: ::windows::core::PCWSTR, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_DevNode_Custom_PropertyW(::core::mem::transmute(dndevinst), pszcustompropertyname.into_param().abi(), ::core::mem::transmute(pulregdatatype), ::core::mem::transmute(buffer), ::core::mem::transmute(pullength), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_DevNode_Custom_Property_ExA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(dndevinst: u32, pszcustompropertyname: Param1, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_DevNode_Custom_Property_ExA(dndevinst: u32, pszcustompropertyname: ::windows::core::PCSTR, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_DevNode_Custom_Property_ExA(::core::mem::transmute(dndevinst), pszcustompropertyname.into_param().abi(), ::core::mem::transmute(pulregdatatype), ::core::mem::transmute(buffer), ::core::mem::transmute(pullength), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_DevNode_Custom_Property_ExW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(dndevinst: u32, pszcustompropertyname: Param1, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_DevNode_Custom_Property_ExW(dndevinst: u32, pszcustompropertyname: ::windows::core::PCWSTR, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_DevNode_Custom_Property_ExW(::core::mem::transmute(dndevinst), pszcustompropertyname.into_param().abi(), ::core::mem::transmute(pulregdatatype), ::core::mem::transmute(buffer), ::core::mem::transmute(pullength), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn CM_Get_DevNode_PropertyW(dndevinst: u32, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: *mut u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_DevNode_PropertyW(dndevinst: u32, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: *mut u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_DevNode_PropertyW(::core::mem::transmute(dndevinst), ::core::mem::transmute(propertykey), ::core::mem::transmute(propertytype), ::core::mem::transmute(propertybuffer), ::core::mem::transmute(propertybuffersize), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn CM_Get_DevNode_Property_ExW(dndevinst: u32, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_DevNode_Property_ExW(dndevinst: u32, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_DevNode_Property_ExW(::core::mem::transmute(dndevinst), ::core::mem::transmute(propertykey), ::core::mem::transmute(propertytype), ::core::mem::transmute(propertybuffer), ::core::mem::transmute(propertybuffersize), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn CM_Get_DevNode_Property_Keys(dndevinst: u32, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: *mut u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_DevNode_Property_Keys(dndevinst: u32, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: *mut u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_DevNode_Property_Keys(::core::mem::transmute(dndevinst), ::core::mem::transmute(propertykeyarray), ::core::mem::transmute(propertykeycount), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn CM_Get_DevNode_Property_Keys_Ex(dndevinst: u32, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_DevNode_Property_Keys_Ex(dndevinst: u32, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_DevNode_Property_Keys_Ex(::core::mem::transmute(dndevinst), ::core::mem::transmute(propertykeyarray), ::core::mem::transmute(propertykeycount), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_DevNode_Registry_PropertyA(dndevinst: u32, ulproperty: u32, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_DevNode_Registry_PropertyA(dndevinst: u32, ulproperty: u32, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_DevNode_Registry_PropertyA(::core::mem::transmute(dndevinst), ::core::mem::transmute(ulproperty), ::core::mem::transmute(pulregdatatype), ::core::mem::transmute(buffer), ::core::mem::transmute(pullength), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_DevNode_Registry_PropertyW(dndevinst: u32, ulproperty: u32, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_DevNode_Registry_PropertyW(dndevinst: u32, ulproperty: u32, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_DevNode_Registry_PropertyW(::core::mem::transmute(dndevinst), ::core::mem::transmute(ulproperty), ::core::mem::transmute(pulregdatatype), ::core::mem::transmute(buffer), ::core::mem::transmute(pullength), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_DevNode_Registry_Property_ExA(dndevinst: u32, ulproperty: u32, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_DevNode_Registry_Property_ExA(dndevinst: u32, ulproperty: u32, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_DevNode_Registry_Property_ExA(::core::mem::transmute(dndevinst), ::core::mem::transmute(ulproperty), ::core::mem::transmute(pulregdatatype), ::core::mem::transmute(buffer), ::core::mem::transmute(pullength), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_DevNode_Registry_Property_ExW(dndevinst: u32, ulproperty: u32, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_DevNode_Registry_Property_ExW(dndevinst: u32, ulproperty: u32, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_DevNode_Registry_Property_ExW(::core::mem::transmute(dndevinst), ::core::mem::transmute(ulproperty), ::core::mem::transmute(pulregdatatype), ::core::mem::transmute(buffer), ::core::mem::transmute(pullength), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_DevNode_Status(pulstatus: *mut u32, pulproblemnumber: *mut u32, dndevinst: u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_DevNode_Status(pulstatus: *mut u32, pulproblemnumber: *mut u32, dndevinst: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_DevNode_Status(::core::mem::transmute(pulstatus), ::core::mem::transmute(pulproblemnumber), ::core::mem::transmute(dndevinst), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_DevNode_Status_Ex(pulstatus: *mut u32, pulproblemnumber: *mut u32, dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_DevNode_Status_Ex(pulstatus: *mut u32, pulproblemnumber: *mut u32, dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_DevNode_Status_Ex(::core::mem::transmute(pulstatus), ::core::mem::transmute(pulproblemnumber), ::core::mem::transmute(dndevinst), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Device_IDA(dndevinst: u32, buffer: &mut [u8], ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Device_IDA(dndevinst: u32, buffer: ::windows::core::PSTR, bufferlen: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Device_IDA(::core::mem::transmute(dndevinst), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(buffer)), buffer.len() as _, ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Device_IDW(dndevinst: u32, buffer: &mut [u16], ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Device_IDW(dndevinst: u32, buffer: ::windows::core::PWSTR, bufferlen: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Device_IDW(::core::mem::transmute(dndevinst), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(buffer)), buffer.len() as _, ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Device_ID_ExA(dndevinst: u32, buffer: &mut [u8], ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Device_ID_ExA(dndevinst: u32, buffer: ::windows::core::PSTR, bufferlen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Device_ID_ExA(::core::mem::transmute(dndevinst), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(buffer)), buffer.len() as _, ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Device_ID_ExW(dndevinst: u32, buffer: &mut [u16], ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Device_ID_ExW(dndevinst: u32, buffer: ::windows::core::PWSTR, bufferlen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Device_ID_ExW(::core::mem::transmute(dndevinst), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(buffer)), buffer.len() as _, ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Device_ID_ListA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(pszfilter: Param0, buffer: &mut [u8], ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Device_ID_ListA(pszfilter: ::windows::core::PCSTR, buffer: ::windows::core::PSTR, bufferlen: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Device_ID_ListA(pszfilter.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(buffer)), buffer.len() as _, ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Device_ID_ListW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pszfilter: Param0, buffer: &mut [u16], ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Device_ID_ListW(pszfilter: ::windows::core::PCWSTR, buffer: ::windows::core::PWSTR, bufferlen: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Device_ID_ListW(pszfilter.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(buffer)), buffer.len() as _, ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Device_ID_List_ExA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(pszfilter: Param0, buffer: &mut [u8], ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Device_ID_List_ExA(pszfilter: ::windows::core::PCSTR, buffer: ::windows::core::PSTR, bufferlen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Device_ID_List_ExA(pszfilter.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(buffer)), buffer.len() as _, ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Device_ID_List_ExW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pszfilter: Param0, buffer: &mut [u16], ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Device_ID_List_ExW(pszfilter: ::windows::core::PCWSTR, buffer: ::windows::core::PWSTR, bufferlen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Device_ID_List_ExW(pszfilter.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(buffer)), buffer.len() as _, ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Device_ID_List_SizeA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(pullen: *mut u32, pszfilter: Param1, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Device_ID_List_SizeA(pullen: *mut u32, pszfilter: ::windows::core::PCSTR, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Device_ID_List_SizeA(::core::mem::transmute(pullen), pszfilter.into_param().abi(), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Device_ID_List_SizeW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pullen: *mut u32, pszfilter: Param1, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Device_ID_List_SizeW(pullen: *mut u32, pszfilter: ::windows::core::PCWSTR, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Device_ID_List_SizeW(::core::mem::transmute(pullen), pszfilter.into_param().abi(), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Device_ID_List_Size_ExA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(pullen: *mut u32, pszfilter: Param1, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Device_ID_List_Size_ExA(pullen: *mut u32, pszfilter: ::windows::core::PCSTR, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Device_ID_List_Size_ExA(::core::mem::transmute(pullen), pszfilter.into_param().abi(), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Device_ID_List_Size_ExW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pullen: *mut u32, pszfilter: Param1, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Device_ID_List_Size_ExW(pullen: *mut u32, pszfilter: ::windows::core::PCWSTR, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Device_ID_List_Size_ExW(::core::mem::transmute(pullen), pszfilter.into_param().abi(), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Device_ID_Size(pullen: *mut u32, dndevinst: u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Device_ID_Size(pullen: *mut u32, dndevinst: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Device_ID_Size(::core::mem::transmute(pullen), ::core::mem::transmute(dndevinst), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Device_ID_Size_Ex(pullen: *mut u32, dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Device_ID_Size_Ex(pullen: *mut u32, dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Device_ID_Size_Ex(::core::mem::transmute(pullen), ::core::mem::transmute(dndevinst), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Device_Interface_AliasA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(pszdeviceinterface: Param0, aliasinterfaceguid: *const ::windows::core::GUID, pszaliasdeviceinterface: ::windows::core::PSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Device_Interface_AliasA(pszdeviceinterface: ::windows::core::PCSTR, aliasinterfaceguid: *const ::windows::core::GUID, pszaliasdeviceinterface: ::windows::core::PSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Device_Interface_AliasA(pszdeviceinterface.into_param().abi(), ::core::mem::transmute(aliasinterfaceguid), ::core::mem::transmute(pszaliasdeviceinterface), ::core::mem::transmute(pullength), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Device_Interface_AliasW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pszdeviceinterface: Param0, aliasinterfaceguid: *const ::windows::core::GUID, pszaliasdeviceinterface: ::windows::core::PWSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Device_Interface_AliasW(pszdeviceinterface: ::windows::core::PCWSTR, aliasinterfaceguid: *const ::windows::core::GUID, pszaliasdeviceinterface: ::windows::core::PWSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Device_Interface_AliasW(pszdeviceinterface.into_param().abi(), ::core::mem::transmute(aliasinterfaceguid), ::core::mem::transmute(pszaliasdeviceinterface), ::core::mem::transmute(pullength), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Device_Interface_Alias_ExA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(pszdeviceinterface: Param0, aliasinterfaceguid: *const ::windows::core::GUID, pszaliasdeviceinterface: ::windows::core::PSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Device_Interface_Alias_ExA(pszdeviceinterface: ::windows::core::PCSTR, aliasinterfaceguid: *const ::windows::core::GUID, pszaliasdeviceinterface: ::windows::core::PSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Device_Interface_Alias_ExA(pszdeviceinterface.into_param().abi(), ::core::mem::transmute(aliasinterfaceguid), ::core::mem::transmute(pszaliasdeviceinterface), ::core::mem::transmute(pullength), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Device_Interface_Alias_ExW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pszdeviceinterface: Param0, aliasinterfaceguid: *const ::windows::core::GUID, pszaliasdeviceinterface: ::windows::core::PWSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Device_Interface_Alias_ExW(pszdeviceinterface: ::windows::core::PCWSTR, aliasinterfaceguid: *const ::windows::core::GUID, pszaliasdeviceinterface: ::windows::core::PWSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Device_Interface_Alias_ExW(pszdeviceinterface.into_param().abi(), ::core::mem::transmute(aliasinterfaceguid), ::core::mem::transmute(pszaliasdeviceinterface), ::core::mem::transmute(pullength), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Device_Interface_ListA(interfaceclassguid: *const ::windows::core::GUID, pdeviceid: *const i8, buffer: &mut [u8], ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Device_Interface_ListA(interfaceclassguid: *const ::windows::core::GUID, pdeviceid: *const i8, buffer: ::windows::core::PSTR, bufferlen: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Device_Interface_ListA(::core::mem::transmute(interfaceclassguid), ::core::mem::transmute(pdeviceid), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(buffer)), buffer.len() as _, ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Device_Interface_ListW(interfaceclassguid: *const ::windows::core::GUID, pdeviceid: *const u16, buffer: &mut [u16], ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Device_Interface_ListW(interfaceclassguid: *const ::windows::core::GUID, pdeviceid: *const u16, buffer: ::windows::core::PWSTR, bufferlen: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Device_Interface_ListW(::core::mem::transmute(interfaceclassguid), ::core::mem::transmute(pdeviceid), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(buffer)), buffer.len() as _, ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Device_Interface_List_ExA(interfaceclassguid: *const ::windows::core::GUID, pdeviceid: *const i8, buffer: &mut [u8], ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Device_Interface_List_ExA(interfaceclassguid: *const ::windows::core::GUID, pdeviceid: *const i8, buffer: ::windows::core::PSTR, bufferlen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Device_Interface_List_ExA(::core::mem::transmute(interfaceclassguid), ::core::mem::transmute(pdeviceid), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(buffer)), buffer.len() as _, ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Device_Interface_List_ExW(interfaceclassguid: *const ::windows::core::GUID, pdeviceid: *const u16, buffer: &mut [u16], ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Device_Interface_List_ExW(interfaceclassguid: *const ::windows::core::GUID, pdeviceid: *const u16, buffer: ::windows::core::PWSTR, bufferlen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Device_Interface_List_ExW(::core::mem::transmute(interfaceclassguid), ::core::mem::transmute(pdeviceid), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(buffer)), buffer.len() as _, ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Device_Interface_List_SizeA(pullen: *mut u32, interfaceclassguid: *const ::windows::core::GUID, pdeviceid: *const i8, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Device_Interface_List_SizeA(pullen: *mut u32, interfaceclassguid: *const ::windows::core::GUID, pdeviceid: *const i8, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Device_Interface_List_SizeA(::core::mem::transmute(pullen), ::core::mem::transmute(interfaceclassguid), ::core::mem::transmute(pdeviceid), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Device_Interface_List_SizeW(pullen: *mut u32, interfaceclassguid: *const ::windows::core::GUID, pdeviceid: *const u16, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Device_Interface_List_SizeW(pullen: *mut u32, interfaceclassguid: *const ::windows::core::GUID, pdeviceid: *const u16, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Device_Interface_List_SizeW(::core::mem::transmute(pullen), ::core::mem::transmute(interfaceclassguid), ::core::mem::transmute(pdeviceid), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Device_Interface_List_Size_ExA(pullen: *mut u32, interfaceclassguid: *const ::windows::core::GUID, pdeviceid: *const i8, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Device_Interface_List_Size_ExA(pullen: *mut u32, interfaceclassguid: *const ::windows::core::GUID, pdeviceid: *const i8, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Device_Interface_List_Size_ExA(::core::mem::transmute(pullen), ::core::mem::transmute(interfaceclassguid), ::core::mem::transmute(pdeviceid), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Device_Interface_List_Size_ExW(pullen: *mut u32, interfaceclassguid: *const ::windows::core::GUID, pdeviceid: *const u16, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Device_Interface_List_Size_ExW(pullen: *mut u32, interfaceclassguid: *const ::windows::core::GUID, pdeviceid: *const u16, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Device_Interface_List_Size_ExW(::core::mem::transmute(pullen), ::core::mem::transmute(interfaceclassguid), ::core::mem::transmute(pdeviceid), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn CM_Get_Device_Interface_PropertyW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pszdeviceinterface: Param0, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: *mut u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Device_Interface_PropertyW(pszdeviceinterface: ::windows::core::PCWSTR, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: *mut u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Device_Interface_PropertyW(pszdeviceinterface.into_param().abi(), ::core::mem::transmute(propertykey), ::core::mem::transmute(propertytype), ::core::mem::transmute(propertybuffer), ::core::mem::transmute(propertybuffersize), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn CM_Get_Device_Interface_Property_ExW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pszdeviceinterface: Param0, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Device_Interface_Property_ExW(pszdeviceinterface: ::windows::core::PCWSTR, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Device_Interface_Property_ExW(pszdeviceinterface.into_param().abi(), ::core::mem::transmute(propertykey), ::core::mem::transmute(propertytype), ::core::mem::transmute(propertybuffer), ::core::mem::transmute(propertybuffersize), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn CM_Get_Device_Interface_Property_KeysW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pszdeviceinterface: Param0, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: *mut u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Device_Interface_Property_KeysW(pszdeviceinterface: ::windows::core::PCWSTR, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: *mut u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Device_Interface_Property_KeysW(pszdeviceinterface.into_param().abi(), ::core::mem::transmute(propertykeyarray), ::core::mem::transmute(propertykeycount), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn CM_Get_Device_Interface_Property_Keys_ExW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pszdeviceinterface: Param0, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Device_Interface_Property_Keys_ExW(pszdeviceinterface: ::windows::core::PCWSTR, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Device_Interface_Property_Keys_ExW(pszdeviceinterface.into_param().abi(), ::core::mem::transmute(propertykeyarray), ::core::mem::transmute(propertykeycount), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_First_Log_Conf(plclogconf: *mut usize, dndevinst: u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_First_Log_Conf(plclogconf: *mut usize, dndevinst: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_First_Log_Conf(::core::mem::transmute(plclogconf), ::core::mem::transmute(dndevinst), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_First_Log_Conf_Ex(plclogconf: *mut usize, dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_First_Log_Conf_Ex(plclogconf: *mut usize, dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_First_Log_Conf_Ex(::core::mem::transmute(plclogconf), ::core::mem::transmute(dndevinst), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Global_State(pulstate: *mut u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Global_State(pulstate: *mut u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Global_State(::core::mem::transmute(pulstate), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Global_State_Ex(pulstate: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Global_State_Ex(pulstate: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Global_State_Ex(::core::mem::transmute(pulstate), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_HW_Prof_FlagsA(pdeviceid: *const i8, ulhardwareprofile: u32, pulvalue: *mut u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_HW_Prof_FlagsA(pdeviceid: *const i8, ulhardwareprofile: u32, pulvalue: *mut u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_HW_Prof_FlagsA(::core::mem::transmute(pdeviceid), ::core::mem::transmute(ulhardwareprofile), ::core::mem::transmute(pulvalue), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_HW_Prof_FlagsW(pdeviceid: *const u16, ulhardwareprofile: u32, pulvalue: *mut u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_HW_Prof_FlagsW(pdeviceid: *const u16, ulhardwareprofile: u32, pulvalue: *mut u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_HW_Prof_FlagsW(::core::mem::transmute(pdeviceid), ::core::mem::transmute(ulhardwareprofile), ::core::mem::transmute(pulvalue), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_HW_Prof_Flags_ExA(pdeviceid: *const i8, ulhardwareprofile: u32, pulvalue: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_HW_Prof_Flags_ExA(pdeviceid: *const i8, ulhardwareprofile: u32, pulvalue: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_HW_Prof_Flags_ExA(::core::mem::transmute(pdeviceid), ::core::mem::transmute(ulhardwareprofile), ::core::mem::transmute(pulvalue), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_HW_Prof_Flags_ExW(pdeviceid: *const u16, ulhardwareprofile: u32, pulvalue: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_HW_Prof_Flags_ExW(pdeviceid: *const u16, ulhardwareprofile: u32, pulvalue: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_HW_Prof_Flags_ExW(::core::mem::transmute(pdeviceid), ::core::mem::transmute(ulhardwareprofile), ::core::mem::transmute(pulvalue), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CM_Get_Hardware_Profile_InfoA(ulindex: u32, phwprofileinfo: *mut HWProfileInfo_sA, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Hardware_Profile_InfoA(ulindex: u32, phwprofileinfo: *mut HWProfileInfo_sA, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Hardware_Profile_InfoA(::core::mem::transmute(ulindex), ::core::mem::transmute(phwprofileinfo), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Hardware_Profile_InfoW(ulindex: u32, phwprofileinfo: *mut HWProfileInfo_sW, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Hardware_Profile_InfoW(ulindex: u32, phwprofileinfo: *mut HWProfileInfo_sW, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Hardware_Profile_InfoW(::core::mem::transmute(ulindex), ::core::mem::transmute(phwprofileinfo), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CM_Get_Hardware_Profile_Info_ExA(ulindex: u32, phwprofileinfo: *mut HWProfileInfo_sA, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Hardware_Profile_Info_ExA(ulindex: u32, phwprofileinfo: *mut HWProfileInfo_sA, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Hardware_Profile_Info_ExA(::core::mem::transmute(ulindex), ::core::mem::transmute(phwprofileinfo), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Hardware_Profile_Info_ExW(ulindex: u32, phwprofileinfo: *mut HWProfileInfo_sW, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Hardware_Profile_Info_ExW(ulindex: u32, phwprofileinfo: *mut HWProfileInfo_sW, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Hardware_Profile_Info_ExW(::core::mem::transmute(ulindex), ::core::mem::transmute(phwprofileinfo), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Log_Conf_Priority(lclogconf: usize, ppriority: *mut u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Log_Conf_Priority(lclogconf: usize, ppriority: *mut u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Log_Conf_Priority(::core::mem::transmute(lclogconf), ::core::mem::transmute(ppriority), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Log_Conf_Priority_Ex(lclogconf: usize, ppriority: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Log_Conf_Priority_Ex(lclogconf: usize, ppriority: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Log_Conf_Priority_Ex(::core::mem::transmute(lclogconf), ::core::mem::transmute(ppriority), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Next_Log_Conf(plclogconf: *mut usize, lclogconf: usize, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Next_Log_Conf(plclogconf: *mut usize, lclogconf: usize, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Next_Log_Conf(::core::mem::transmute(plclogconf), ::core::mem::transmute(lclogconf), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Next_Log_Conf_Ex(plclogconf: *mut usize, lclogconf: usize, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Next_Log_Conf_Ex(plclogconf: *mut usize, lclogconf: usize, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Next_Log_Conf_Ex(::core::mem::transmute(plclogconf), ::core::mem::transmute(lclogconf), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Next_Res_Des(prdresdes: *mut usize, rdresdes: usize, forresource: u32, presourceid: *mut u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Next_Res_Des(prdresdes: *mut usize, rdresdes: usize, forresource: u32, presourceid: *mut u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Next_Res_Des(::core::mem::transmute(prdresdes), ::core::mem::transmute(rdresdes), ::core::mem::transmute(forresource), ::core::mem::transmute(presourceid), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Next_Res_Des_Ex(prdresdes: *mut usize, rdresdes: usize, forresource: u32, presourceid: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Next_Res_Des_Ex(prdresdes: *mut usize, rdresdes: usize, forresource: u32, presourceid: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Next_Res_Des_Ex(::core::mem::transmute(prdresdes), ::core::mem::transmute(rdresdes), ::core::mem::transmute(forresource), ::core::mem::transmute(presourceid), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Parent(pdndevinst: *mut u32, dndevinst: u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Parent(pdndevinst: *mut u32, dndevinst: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Parent(::core::mem::transmute(pdndevinst), ::core::mem::transmute(dndevinst), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Parent_Ex(pdndevinst: *mut u32, dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Parent_Ex(pdndevinst: *mut u32, dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Parent_Ex(::core::mem::transmute(pdndevinst), ::core::mem::transmute(dndevinst), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Res_Des_Data(rdresdes: usize, buffer: *mut ::core::ffi::c_void, bufferlen: u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Res_Des_Data(rdresdes: usize, buffer: *mut ::core::ffi::c_void, bufferlen: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Res_Des_Data(::core::mem::transmute(rdresdes), ::core::mem::transmute(buffer), ::core::mem::transmute(bufferlen), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Res_Des_Data_Ex(rdresdes: usize, buffer: *mut ::core::ffi::c_void, bufferlen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Res_Des_Data_Ex(rdresdes: usize, buffer: *mut ::core::ffi::c_void, bufferlen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Res_Des_Data_Ex(::core::mem::transmute(rdresdes), ::core::mem::transmute(buffer), ::core::mem::transmute(bufferlen), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Res_Des_Data_Size(pulsize: *mut u32, rdresdes: usize, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Res_Des_Data_Size(pulsize: *mut u32, rdresdes: usize, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Res_Des_Data_Size(::core::mem::transmute(pulsize), ::core::mem::transmute(rdresdes), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Res_Des_Data_Size_Ex(pulsize: *mut u32, rdresdes: usize, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Res_Des_Data_Size_Ex(pulsize: *mut u32, rdresdes: usize, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Res_Des_Data_Size_Ex(::core::mem::transmute(pulsize), ::core::mem::transmute(rdresdes), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Resource_Conflict_Count(clconflictlist: usize, pulcount: *mut u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Resource_Conflict_Count(clconflictlist: usize, pulcount: *mut u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Resource_Conflict_Count(::core::mem::transmute(clconflictlist), ::core::mem::transmute(pulcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CM_Get_Resource_Conflict_DetailsA(clconflictlist: usize, ulindex: u32, pconflictdetails: *mut CONFLICT_DETAILS_A) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Resource_Conflict_DetailsA(clconflictlist: usize, ulindex: u32, pconflictdetails: *mut CONFLICT_DETAILS_A) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Resource_Conflict_DetailsA(::core::mem::transmute(clconflictlist), ::core::mem::transmute(ulindex), ::core::mem::transmute(pconflictdetails)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Resource_Conflict_DetailsW(clconflictlist: usize, ulindex: u32, pconflictdetails: *mut CONFLICT_DETAILS_W) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Resource_Conflict_DetailsW(clconflictlist: usize, ulindex: u32, pconflictdetails: *mut CONFLICT_DETAILS_W) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Resource_Conflict_DetailsW(::core::mem::transmute(clconflictlist), ::core::mem::transmute(ulindex), ::core::mem::transmute(pconflictdetails)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Sibling(pdndevinst: *mut u32, dndevinst: u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Sibling(pdndevinst: *mut u32, dndevinst: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Sibling(::core::mem::transmute(pdndevinst), ::core::mem::transmute(dndevinst), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Sibling_Ex(pdndevinst: *mut u32, dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Sibling_Ex(pdndevinst: *mut u32, dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Get_Sibling_Ex(::core::mem::transmute(pdndevinst), ::core::mem::transmute(dndevinst), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Version() -> u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Version() -> u16;
        }
        ::core::mem::transmute(CM_Get_Version())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Get_Version_Ex(hmachine: isize) -> u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Get_Version_Ex(hmachine: isize) -> u16;
        }
        ::core::mem::transmute(CM_Get_Version_Ex(::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_HWPI_DOCKED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_HWPI_NOT_DOCKABLE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_HWPI_UNDOCKED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_INSTALL_STATE_FAILED_INSTALL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_INSTALL_STATE_FINISH_INSTALL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_INSTALL_STATE_INSTALLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_INSTALL_STATE_NEEDS_REINSTALL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Intersect_Range_List(rlhold1: usize, rlhold2: usize, rlhnew: usize, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Intersect_Range_List(rlhold1: usize, rlhold2: usize, rlhnew: usize, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Intersect_Range_List(::core::mem::transmute(rlhold1), ::core::mem::transmute(rlhold2), ::core::mem::transmute(rlhnew), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Invert_Range_List(rlhold: usize, rlhnew: usize, ullmaxvalue: u64, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Invert_Range_List(rlhold: usize, rlhnew: usize, ullmaxvalue: u64, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Invert_Range_List(::core::mem::transmute(rlhold), ::core::mem::transmute(rlhnew), ::core::mem::transmute(ullmaxvalue), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CM_Is_Dock_Station_Present(pbpresent: *mut super::super::Foundation::BOOL) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Is_Dock_Station_Present(pbpresent: *mut super::super::Foundation::BOOL) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Is_Dock_Station_Present(::core::mem::transmute(pbpresent)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CM_Is_Dock_Station_Present_Ex(pbpresent: *mut super::super::Foundation::BOOL, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Is_Dock_Station_Present_Ex(pbpresent: *mut super::super::Foundation::BOOL, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Is_Dock_Station_Present_Ex(::core::mem::transmute(pbpresent), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CM_Is_Version_Available(wversion: u16) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Is_Version_Available(wversion: u16) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CM_Is_Version_Available(::core::mem::transmute(wversion)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CM_Is_Version_Available_Ex(wversion: u16, hmachine: isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Is_Version_Available_Ex(wversion: u16, hmachine: isize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CM_Is_Version_Available_Ex(::core::mem::transmute(wversion), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_LOCATE_DEVINST_BITS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_LOCATE_DEVINST_CANCELREMOVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_LOCATE_DEVINST_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_LOCATE_DEVINST_NOVALIDATION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_LOCATE_DEVINST_PHANTOM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_LOCATE_DEVNODE_BITS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_LOCATE_DEVNODE_CANCELREMOVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_LOCATE_DEVNODE_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_LOCATE_DEVNODE_NOVALIDATION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_LOCATE_DEVNODE_PHANTOM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Locate_DevNodeA(pdndevinst: *mut u32, pdeviceid: *const i8, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Locate_DevNodeA(pdndevinst: *mut u32, pdeviceid: *const i8, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Locate_DevNodeA(::core::mem::transmute(pdndevinst), ::core::mem::transmute(pdeviceid), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Locate_DevNodeW(pdndevinst: *mut u32, pdeviceid: *const u16, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Locate_DevNodeW(pdndevinst: *mut u32, pdeviceid: *const u16, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Locate_DevNodeW(::core::mem::transmute(pdndevinst), ::core::mem::transmute(pdeviceid), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Locate_DevNode_ExA(pdndevinst: *mut u32, pdeviceid: *const i8, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Locate_DevNode_ExA(pdndevinst: *mut u32, pdeviceid: *const i8, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Locate_DevNode_ExA(::core::mem::transmute(pdndevinst), ::core::mem::transmute(pdeviceid), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Locate_DevNode_ExW(pdndevinst: *mut u32, pdeviceid: *const u16, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Locate_DevNode_ExW(pdndevinst: *mut u32, pdeviceid: *const u16, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Locate_DevNode_ExW(::core::mem::transmute(pdndevinst), ::core::mem::transmute(pdeviceid), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_MapCrToWin32Err(cmreturncode: CONFIGRET, defaulterr: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_MapCrToWin32Err(cmreturncode: CONFIGRET, defaulterr: u32) -> u32;
        }
        ::core::mem::transmute(CM_MapCrToWin32Err(::core::mem::transmute(cmreturncode), ::core::mem::transmute(defaulterr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Merge_Range_List(rlhold1: usize, rlhold2: usize, rlhnew: usize, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Merge_Range_List(rlhold1: usize, rlhold2: usize, rlhnew: usize, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Merge_Range_List(::core::mem::transmute(rlhold1), ::core::mem::transmute(rlhold2), ::core::mem::transmute(rlhnew), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Modify_Res_Des(prdresdes: *mut usize, rdresdes: usize, resourceid: u32, resourcedata: *const ::core::ffi::c_void, resourcelen: u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Modify_Res_Des(prdresdes: *mut usize, rdresdes: usize, resourceid: u32, resourcedata: *const ::core::ffi::c_void, resourcelen: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Modify_Res_Des(::core::mem::transmute(prdresdes), ::core::mem::transmute(rdresdes), ::core::mem::transmute(resourceid), ::core::mem::transmute(resourcedata), ::core::mem::transmute(resourcelen), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Modify_Res_Des_Ex(prdresdes: *mut usize, rdresdes: usize, resourceid: u32, resourcedata: *const ::core::ffi::c_void, resourcelen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Modify_Res_Des_Ex(prdresdes: *mut usize, rdresdes: usize, resourceid: u32, resourcedata: *const ::core::ffi::c_void, resourcelen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Modify_Res_Des_Ex(::core::mem::transmute(prdresdes), ::core::mem::transmute(rdresdes), ::core::mem::transmute(resourceid), ::core::mem::transmute(resourcedata), ::core::mem::transmute(resourcelen), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Move_DevNode(dnfromdevinst: u32, dntodevinst: u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Move_DevNode(dnfromdevinst: u32, dntodevinst: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Move_DevNode(::core::mem::transmute(dnfromdevinst), ::core::mem::transmute(dntodevinst), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Move_DevNode_Ex(dnfromdevinst: u32, dntodevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Move_DevNode_Ex(dnfromdevinst: u32, dntodevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Move_DevNode_Ex(::core::mem::transmute(dnfromdevinst), ::core::mem::transmute(dntodevinst), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NAME_ATTRIBUTE_NAME_RETRIEVED_FROM_DEVICE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NAME_ATTRIBUTE_USER_ASSIGNED_NAME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CM_NOTIFY_ACTION(pub i32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NOTIFY_ACTION_DEVICEINTERFACEARRIVAL: CM_NOTIFY_ACTION = CM_NOTIFY_ACTION(0i32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NOTIFY_ACTION_DEVICEINTERFACEREMOVAL: CM_NOTIFY_ACTION = CM_NOTIFY_ACTION(1i32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NOTIFY_ACTION_DEVICEQUERYREMOVE: CM_NOTIFY_ACTION = CM_NOTIFY_ACTION(2i32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NOTIFY_ACTION_DEVICEQUERYREMOVEFAILED: CM_NOTIFY_ACTION = CM_NOTIFY_ACTION(3i32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NOTIFY_ACTION_DEVICEREMOVEPENDING: CM_NOTIFY_ACTION = CM_NOTIFY_ACTION(4i32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NOTIFY_ACTION_DEVICEREMOVECOMPLETE: CM_NOTIFY_ACTION = CM_NOTIFY_ACTION(5i32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NOTIFY_ACTION_DEVICECUSTOMEVENT: CM_NOTIFY_ACTION = CM_NOTIFY_ACTION(6i32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NOTIFY_ACTION_DEVICEINSTANCEENUMERATED: CM_NOTIFY_ACTION = CM_NOTIFY_ACTION(7i32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NOTIFY_ACTION_DEVICEINSTANCESTARTED: CM_NOTIFY_ACTION = CM_NOTIFY_ACTION(8i32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NOTIFY_ACTION_DEVICEINSTANCEREMOVED: CM_NOTIFY_ACTION = CM_NOTIFY_ACTION(9i32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NOTIFY_ACTION_MAX: CM_NOTIFY_ACTION = CM_NOTIFY_ACTION(10i32);
impl ::core::marker::Copy for CM_NOTIFY_ACTION {}
impl ::core::clone::Clone for CM_NOTIFY_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CM_NOTIFY_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CM_NOTIFY_ACTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for CM_NOTIFY_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CM_NOTIFY_ACTION").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct CM_NOTIFY_EVENT_DATA {
    pub FilterType: CM_NOTIFY_FILTER_TYPE,
    pub Reserved: u32,
    pub u: CM_NOTIFY_EVENT_DATA_0,
}
impl ::core::marker::Copy for CM_NOTIFY_EVENT_DATA {}
impl ::core::clone::Clone for CM_NOTIFY_EVENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CM_NOTIFY_EVENT_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CM_NOTIFY_EVENT_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CM_NOTIFY_EVENT_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for CM_NOTIFY_EVENT_DATA {}
impl ::core::default::Default for CM_NOTIFY_EVENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub union CM_NOTIFY_EVENT_DATA_0 {
    pub DeviceInterface: CM_NOTIFY_EVENT_DATA_0_2,
    pub DeviceHandle: CM_NOTIFY_EVENT_DATA_0_0,
    pub DeviceInstance: CM_NOTIFY_EVENT_DATA_0_1,
}
impl ::core::marker::Copy for CM_NOTIFY_EVENT_DATA_0 {}
impl ::core::clone::Clone for CM_NOTIFY_EVENT_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CM_NOTIFY_EVENT_DATA_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CM_NOTIFY_EVENT_DATA_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CM_NOTIFY_EVENT_DATA_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for CM_NOTIFY_EVENT_DATA_0 {}
impl ::core::default::Default for CM_NOTIFY_EVENT_DATA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct CM_NOTIFY_EVENT_DATA_0_0 {
    pub EventGuid: ::windows::core::GUID,
    pub NameOffset: i32,
    pub DataSize: u32,
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for CM_NOTIFY_EVENT_DATA_0_0 {}
impl ::core::clone::Clone for CM_NOTIFY_EVENT_DATA_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CM_NOTIFY_EVENT_DATA_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CM_NOTIFY_EVENT_DATA_0_0").field("EventGuid", &self.EventGuid).field("NameOffset", &self.NameOffset).field("DataSize", &self.DataSize).field("Data", &self.Data).finish()
    }
}
unsafe impl ::windows::core::Abi for CM_NOTIFY_EVENT_DATA_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CM_NOTIFY_EVENT_DATA_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CM_NOTIFY_EVENT_DATA_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for CM_NOTIFY_EVENT_DATA_0_0 {}
impl ::core::default::Default for CM_NOTIFY_EVENT_DATA_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct CM_NOTIFY_EVENT_DATA_0_1 {
    pub InstanceId: [u16; 1],
}
impl ::core::marker::Copy for CM_NOTIFY_EVENT_DATA_0_1 {}
impl ::core::clone::Clone for CM_NOTIFY_EVENT_DATA_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CM_NOTIFY_EVENT_DATA_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CM_NOTIFY_EVENT_DATA_0_1").field("InstanceId", &self.InstanceId).finish()
    }
}
unsafe impl ::windows::core::Abi for CM_NOTIFY_EVENT_DATA_0_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CM_NOTIFY_EVENT_DATA_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CM_NOTIFY_EVENT_DATA_0_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for CM_NOTIFY_EVENT_DATA_0_1 {}
impl ::core::default::Default for CM_NOTIFY_EVENT_DATA_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct CM_NOTIFY_EVENT_DATA_0_2 {
    pub ClassGuid: ::windows::core::GUID,
    pub SymbolicLink: [u16; 1],
}
impl ::core::marker::Copy for CM_NOTIFY_EVENT_DATA_0_2 {}
impl ::core::clone::Clone for CM_NOTIFY_EVENT_DATA_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CM_NOTIFY_EVENT_DATA_0_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CM_NOTIFY_EVENT_DATA_0_2").field("ClassGuid", &self.ClassGuid).field("SymbolicLink", &self.SymbolicLink).finish()
    }
}
unsafe impl ::windows::core::Abi for CM_NOTIFY_EVENT_DATA_0_2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CM_NOTIFY_EVENT_DATA_0_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CM_NOTIFY_EVENT_DATA_0_2>()) == 0 }
    }
}
impl ::core::cmp::Eq for CM_NOTIFY_EVENT_DATA_0_2 {}
impl ::core::default::Default for CM_NOTIFY_EVENT_DATA_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CM_NOTIFY_FILTER {
    pub cbSize: u32,
    pub Flags: u32,
    pub FilterType: CM_NOTIFY_FILTER_TYPE,
    pub Reserved: u32,
    pub u: CM_NOTIFY_FILTER_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CM_NOTIFY_FILTER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CM_NOTIFY_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CM_NOTIFY_FILTER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CM_NOTIFY_FILTER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CM_NOTIFY_FILTER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CM_NOTIFY_FILTER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CM_NOTIFY_FILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union CM_NOTIFY_FILTER_0 {
    pub DeviceInterface: CM_NOTIFY_FILTER_0_2,
    pub DeviceHandle: CM_NOTIFY_FILTER_0_0,
    pub DeviceInstance: CM_NOTIFY_FILTER_0_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CM_NOTIFY_FILTER_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CM_NOTIFY_FILTER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CM_NOTIFY_FILTER_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CM_NOTIFY_FILTER_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CM_NOTIFY_FILTER_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CM_NOTIFY_FILTER_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CM_NOTIFY_FILTER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CM_NOTIFY_FILTER_0_0 {
    pub hTarget: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CM_NOTIFY_FILTER_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CM_NOTIFY_FILTER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CM_NOTIFY_FILTER_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CM_NOTIFY_FILTER_0_0").field("hTarget", &self.hTarget).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CM_NOTIFY_FILTER_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CM_NOTIFY_FILTER_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CM_NOTIFY_FILTER_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CM_NOTIFY_FILTER_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CM_NOTIFY_FILTER_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CM_NOTIFY_FILTER_0_1 {
    pub InstanceId: [u16; 200],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CM_NOTIFY_FILTER_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CM_NOTIFY_FILTER_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CM_NOTIFY_FILTER_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CM_NOTIFY_FILTER_0_1").field("InstanceId", &self.InstanceId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CM_NOTIFY_FILTER_0_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CM_NOTIFY_FILTER_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CM_NOTIFY_FILTER_0_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CM_NOTIFY_FILTER_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CM_NOTIFY_FILTER_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CM_NOTIFY_FILTER_0_2 {
    pub ClassGuid: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CM_NOTIFY_FILTER_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CM_NOTIFY_FILTER_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CM_NOTIFY_FILTER_0_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CM_NOTIFY_FILTER_0_2").field("ClassGuid", &self.ClassGuid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CM_NOTIFY_FILTER_0_2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CM_NOTIFY_FILTER_0_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CM_NOTIFY_FILTER_0_2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CM_NOTIFY_FILTER_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CM_NOTIFY_FILTER_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NOTIFY_FILTER_FLAG_ALL_DEVICE_INSTANCES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NOTIFY_FILTER_FLAG_ALL_INTERFACE_CLASSES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CM_NOTIFY_FILTER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NOTIFY_FILTER_TYPE_DEVICEINTERFACE: CM_NOTIFY_FILTER_TYPE = CM_NOTIFY_FILTER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NOTIFY_FILTER_TYPE_DEVICEHANDLE: CM_NOTIFY_FILTER_TYPE = CM_NOTIFY_FILTER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NOTIFY_FILTER_TYPE_DEVICEINSTANCE: CM_NOTIFY_FILTER_TYPE = CM_NOTIFY_FILTER_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NOTIFY_FILTER_TYPE_MAX: CM_NOTIFY_FILTER_TYPE = CM_NOTIFY_FILTER_TYPE(3i32);
impl ::core::marker::Copy for CM_NOTIFY_FILTER_TYPE {}
impl ::core::clone::Clone for CM_NOTIFY_FILTER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CM_NOTIFY_FILTER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CM_NOTIFY_FILTER_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for CM_NOTIFY_FILTER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CM_NOTIFY_FILTER_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Next_Range(preelement: *mut usize, pullstart: *mut u64, pullend: *mut u64, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Next_Range(preelement: *mut usize, pullstart: *mut u64, pullend: *mut u64, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Next_Range(::core::mem::transmute(preelement), ::core::mem::transmute(pullstart), ::core::mem::transmute(pullend), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_OPEN_CLASS_KEY_BITS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_OPEN_CLASS_KEY_INSTALLER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_OPEN_CLASS_KEY_INTERFACE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn CM_Open_Class_KeyA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(classguid: *const ::windows::core::GUID, pszclassname: Param1, samdesired: u32, disposition: u32, phkclass: *mut super::super::System::Registry::HKEY, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Open_Class_KeyA(classguid: *const ::windows::core::GUID, pszclassname: ::windows::core::PCSTR, samdesired: u32, disposition: u32, phkclass: *mut super::super::System::Registry::HKEY, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Open_Class_KeyA(::core::mem::transmute(classguid), pszclassname.into_param().abi(), ::core::mem::transmute(samdesired), ::core::mem::transmute(disposition), ::core::mem::transmute(phkclass), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn CM_Open_Class_KeyW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(classguid: *const ::windows::core::GUID, pszclassname: Param1, samdesired: u32, disposition: u32, phkclass: *mut super::super::System::Registry::HKEY, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Open_Class_KeyW(classguid: *const ::windows::core::GUID, pszclassname: ::windows::core::PCWSTR, samdesired: u32, disposition: u32, phkclass: *mut super::super::System::Registry::HKEY, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Open_Class_KeyW(::core::mem::transmute(classguid), pszclassname.into_param().abi(), ::core::mem::transmute(samdesired), ::core::mem::transmute(disposition), ::core::mem::transmute(phkclass), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn CM_Open_Class_Key_ExA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(classguid: *const ::windows::core::GUID, pszclassname: Param1, samdesired: u32, disposition: u32, phkclass: *mut super::super::System::Registry::HKEY, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Open_Class_Key_ExA(classguid: *const ::windows::core::GUID, pszclassname: ::windows::core::PCSTR, samdesired: u32, disposition: u32, phkclass: *mut super::super::System::Registry::HKEY, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Open_Class_Key_ExA(::core::mem::transmute(classguid), pszclassname.into_param().abi(), ::core::mem::transmute(samdesired), ::core::mem::transmute(disposition), ::core::mem::transmute(phkclass), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn CM_Open_Class_Key_ExW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(classguid: *const ::windows::core::GUID, pszclassname: Param1, samdesired: u32, disposition: u32, phkclass: *mut super::super::System::Registry::HKEY, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Open_Class_Key_ExW(classguid: *const ::windows::core::GUID, pszclassname: ::windows::core::PCWSTR, samdesired: u32, disposition: u32, phkclass: *mut super::super::System::Registry::HKEY, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Open_Class_Key_ExW(::core::mem::transmute(classguid), pszclassname.into_param().abi(), ::core::mem::transmute(samdesired), ::core::mem::transmute(disposition), ::core::mem::transmute(phkclass), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn CM_Open_DevNode_Key(dndevnode: u32, samdesired: u32, ulhardwareprofile: u32, disposition: u32, phkdevice: *mut super::super::System::Registry::HKEY, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Open_DevNode_Key(dndevnode: u32, samdesired: u32, ulhardwareprofile: u32, disposition: u32, phkdevice: *mut super::super::System::Registry::HKEY, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Open_DevNode_Key(::core::mem::transmute(dndevnode), ::core::mem::transmute(samdesired), ::core::mem::transmute(ulhardwareprofile), ::core::mem::transmute(disposition), ::core::mem::transmute(phkdevice), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn CM_Open_DevNode_Key_Ex(dndevnode: u32, samdesired: u32, ulhardwareprofile: u32, disposition: u32, phkdevice: *mut super::super::System::Registry::HKEY, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Open_DevNode_Key_Ex(dndevnode: u32, samdesired: u32, ulhardwareprofile: u32, disposition: u32, phkdevice: *mut super::super::System::Registry::HKEY, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Open_DevNode_Key_Ex(::core::mem::transmute(dndevnode), ::core::mem::transmute(samdesired), ::core::mem::transmute(ulhardwareprofile), ::core::mem::transmute(disposition), ::core::mem::transmute(phkdevice), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn CM_Open_Device_Interface_KeyA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(pszdeviceinterface: Param0, samdesired: u32, disposition: u32, phkdeviceinterface: *mut super::super::System::Registry::HKEY, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Open_Device_Interface_KeyA(pszdeviceinterface: ::windows::core::PCSTR, samdesired: u32, disposition: u32, phkdeviceinterface: *mut super::super::System::Registry::HKEY, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Open_Device_Interface_KeyA(pszdeviceinterface.into_param().abi(), ::core::mem::transmute(samdesired), ::core::mem::transmute(disposition), ::core::mem::transmute(phkdeviceinterface), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn CM_Open_Device_Interface_KeyW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pszdeviceinterface: Param0, samdesired: u32, disposition: u32, phkdeviceinterface: *mut super::super::System::Registry::HKEY, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Open_Device_Interface_KeyW(pszdeviceinterface: ::windows::core::PCWSTR, samdesired: u32, disposition: u32, phkdeviceinterface: *mut super::super::System::Registry::HKEY, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Open_Device_Interface_KeyW(pszdeviceinterface.into_param().abi(), ::core::mem::transmute(samdesired), ::core::mem::transmute(disposition), ::core::mem::transmute(phkdeviceinterface), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn CM_Open_Device_Interface_Key_ExA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(pszdeviceinterface: Param0, samdesired: u32, disposition: u32, phkdeviceinterface: *mut super::super::System::Registry::HKEY, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Open_Device_Interface_Key_ExA(pszdeviceinterface: ::windows::core::PCSTR, samdesired: u32, disposition: u32, phkdeviceinterface: *mut super::super::System::Registry::HKEY, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Open_Device_Interface_Key_ExA(pszdeviceinterface.into_param().abi(), ::core::mem::transmute(samdesired), ::core::mem::transmute(disposition), ::core::mem::transmute(phkdeviceinterface), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn CM_Open_Device_Interface_Key_ExW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pszdeviceinterface: Param0, samdesired: u32, disposition: u32, phkdeviceinterface: *mut super::super::System::Registry::HKEY, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Open_Device_Interface_Key_ExW(pszdeviceinterface: ::windows::core::PCWSTR, samdesired: u32, disposition: u32, phkdeviceinterface: *mut super::super::System::Registry::HKEY, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Open_Device_Interface_Key_ExW(pszdeviceinterface.into_param().abi(), ::core::mem::transmute(samdesired), ::core::mem::transmute(disposition), ::core::mem::transmute(phkdeviceinterface), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_BIOS_TABLE: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_BOOT_CONFIG_CONFLICT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_CANT_SHARE_IRQ: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_CONSOLE_LOCKED: u32 = 55u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_DEVICE_NOT_THERE: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_DEVICE_RESET: u32 = 54u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_DEVLOADER_FAILED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_DEVLOADER_NOT_FOUND: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_DEVLOADER_NOT_READY: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_DISABLED: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_DISABLED_SERVICE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_DRIVER_BLOCKED: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_DRIVER_FAILED_LOAD: u32 = 39u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_DRIVER_FAILED_PRIOR_UNLOAD: u32 = 38u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_DRIVER_SERVICE_KEY_INVALID: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_DUPLICATE_DEVICE: u32 = 42u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_ENTRY_IS_WRONG_TYPE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_FAILED_ADD: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_FAILED_DRIVER_ENTRY: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_FAILED_FILTER: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_FAILED_INSTALL: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_FAILED_POST_START: u32 = 43u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_FAILED_START: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_GUEST_ASSIGNMENT_FAILED: u32 = 57u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_HALTED: u32 = 44u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_HARDWARE_DISABLED: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_HELD_FOR_EJECT: u32 = 47u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_INVALID_DATA: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_IRQ_TRANSLATION_FAILED: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_LACKED_ARBITRATOR: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_LEGACY_SERVICE_NO_DEVICES: u32 = 41u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_LIAR: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_MOVED: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_NEED_CLASS_CONFIG: u32 = 56u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_NEED_RESTART: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_NORMAL_CONFLICT: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_NOT_CONFIGURED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_NOT_VERIFIED: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_NO_SOFTCONFIG: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_NO_VALID_LOG_CONF: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_OUT_OF_MEMORY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_PARTIAL_LOG_CONF: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_PHANTOM: u32 = 45u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_REENUMERATION: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_REGISTRY: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_REGISTRY_TOO_LARGE: u32 = 49u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_REINSTALL: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_SETPROPERTIES_FAILED: u32 = 50u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_SYSTEM_SHUTDOWN: u32 = 46u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_TOO_EARLY: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_TRANSLATION_FAILED: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_UNKNOWN_RESOURCE: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_UNSIGNED_DRIVER: u32 = 52u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_USED_BY_DEBUGGER: u32 = 53u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_VXDLDR: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_WAITING_ON_DEPENDENCY: u32 = 51u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_WILL_BE_REMOVED: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_QUERY_ARBITRATOR_BITS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_QUERY_ARBITRATOR_RAW: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_QUERY_ARBITRATOR_TRANSLATED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_QUERY_REMOVE_UI_NOT_OK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_QUERY_REMOVE_UI_OK: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Query_And_Remove_SubTreeA(dnancestor: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: &mut [u8], ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Query_And_Remove_SubTreeA(dnancestor: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: ::windows::core::PSTR, ulnamelength: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Query_And_Remove_SubTreeA(::core::mem::transmute(dnancestor), ::core::mem::transmute(pvetotype), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszvetoname)), pszvetoname.len() as _, ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Query_And_Remove_SubTreeW(dnancestor: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: &mut [u16], ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Query_And_Remove_SubTreeW(dnancestor: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: ::windows::core::PWSTR, ulnamelength: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Query_And_Remove_SubTreeW(::core::mem::transmute(dnancestor), ::core::mem::transmute(pvetotype), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszvetoname)), pszvetoname.len() as _, ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Query_And_Remove_SubTree_ExA(dnancestor: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: &mut [u8], ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Query_And_Remove_SubTree_ExA(dnancestor: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: ::windows::core::PSTR, ulnamelength: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Query_And_Remove_SubTree_ExA(::core::mem::transmute(dnancestor), ::core::mem::transmute(pvetotype), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszvetoname)), pszvetoname.len() as _, ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Query_And_Remove_SubTree_ExW(dnancestor: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: &mut [u16], ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Query_And_Remove_SubTree_ExW(dnancestor: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: ::windows::core::PWSTR, ulnamelength: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Query_And_Remove_SubTree_ExW(::core::mem::transmute(dnancestor), ::core::mem::transmute(pvetotype), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszvetoname)), pszvetoname.len() as _, ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Query_Arbitrator_Free_Data(pdata: *mut ::core::ffi::c_void, datalen: u32, dndevinst: u32, resourceid: u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Query_Arbitrator_Free_Data(pdata: *mut ::core::ffi::c_void, datalen: u32, dndevinst: u32, resourceid: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Query_Arbitrator_Free_Data(::core::mem::transmute(pdata), ::core::mem::transmute(datalen), ::core::mem::transmute(dndevinst), ::core::mem::transmute(resourceid), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Query_Arbitrator_Free_Data_Ex(pdata: *mut ::core::ffi::c_void, datalen: u32, dndevinst: u32, resourceid: u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Query_Arbitrator_Free_Data_Ex(pdata: *mut ::core::ffi::c_void, datalen: u32, dndevinst: u32, resourceid: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Query_Arbitrator_Free_Data_Ex(::core::mem::transmute(pdata), ::core::mem::transmute(datalen), ::core::mem::transmute(dndevinst), ::core::mem::transmute(resourceid), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Query_Arbitrator_Free_Size(pulsize: *mut u32, dndevinst: u32, resourceid: u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Query_Arbitrator_Free_Size(pulsize: *mut u32, dndevinst: u32, resourceid: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Query_Arbitrator_Free_Size(::core::mem::transmute(pulsize), ::core::mem::transmute(dndevinst), ::core::mem::transmute(resourceid), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Query_Arbitrator_Free_Size_Ex(pulsize: *mut u32, dndevinst: u32, resourceid: u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Query_Arbitrator_Free_Size_Ex(pulsize: *mut u32, dndevinst: u32, resourceid: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Query_Arbitrator_Free_Size_Ex(::core::mem::transmute(pulsize), ::core::mem::transmute(dndevinst), ::core::mem::transmute(resourceid), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Query_Remove_SubTree(dnancestor: u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Query_Remove_SubTree(dnancestor: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Query_Remove_SubTree(::core::mem::transmute(dnancestor), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Query_Remove_SubTree_Ex(dnancestor: u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Query_Remove_SubTree_Ex(dnancestor: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Query_Remove_SubTree_Ex(::core::mem::transmute(dnancestor), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Query_Resource_Conflict_List(pclconflictlist: *mut usize, dndevinst: u32, resourceid: u32, resourcedata: *const ::core::ffi::c_void, resourcelen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Query_Resource_Conflict_List(pclconflictlist: *mut usize, dndevinst: u32, resourceid: u32, resourcedata: *const ::core::ffi::c_void, resourcelen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Query_Resource_Conflict_List(::core::mem::transmute(pclconflictlist), ::core::mem::transmute(dndevinst), ::core::mem::transmute(resourceid), ::core::mem::transmute(resourcedata), ::core::mem::transmute(resourcelen), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REENUMERATE_ASYNCHRONOUS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REENUMERATE_BITS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REENUMERATE_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REENUMERATE_RETRY_INSTALLATION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REENUMERATE_SYNCHRONOUS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REGISTER_DEVICE_DRIVER_BITS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REGISTER_DEVICE_DRIVER_DISABLEABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REGISTER_DEVICE_DRIVER_REMOVABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REGISTER_DEVICE_DRIVER_STATIC: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REGISTRY_BITS: u32 = 769u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REGISTRY_CONFIG: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REGISTRY_HARDWARE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REGISTRY_SOFTWARE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REGISTRY_USER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REMOVAL_POLICY_EXPECT_NO_REMOVAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REMOVAL_POLICY_EXPECT_ORDERLY_REMOVAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REMOVAL_POLICY_EXPECT_SURPRISE_REMOVAL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REMOVE_BITS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REMOVE_DISABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REMOVE_NO_RESTART: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REMOVE_UI_NOT_OK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REMOVE_UI_OK: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_RESDES_WIDTH_32: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_RESDES_WIDTH_64: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_RESDES_WIDTH_BITS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_RESDES_WIDTH_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Reenumerate_DevNode(dndevinst: u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Reenumerate_DevNode(dndevinst: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Reenumerate_DevNode(::core::mem::transmute(dndevinst), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Reenumerate_DevNode_Ex(dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Reenumerate_DevNode_Ex(dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Reenumerate_DevNode_Ex(::core::mem::transmute(dndevinst), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Register_Device_Driver(dndevinst: u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Register_Device_Driver(dndevinst: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Register_Device_Driver(::core::mem::transmute(dndevinst), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Register_Device_Driver_Ex(dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Register_Device_Driver_Ex(dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Register_Device_Driver_Ex(::core::mem::transmute(dndevinst), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Register_Device_InterfaceA<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(dndevinst: u32, interfaceclassguid: *const ::windows::core::GUID, pszreference: Param2, pszdeviceinterface: ::windows::core::PSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Register_Device_InterfaceA(dndevinst: u32, interfaceclassguid: *const ::windows::core::GUID, pszreference: ::windows::core::PCSTR, pszdeviceinterface: ::windows::core::PSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Register_Device_InterfaceA(::core::mem::transmute(dndevinst), ::core::mem::transmute(interfaceclassguid), pszreference.into_param().abi(), ::core::mem::transmute(pszdeviceinterface), ::core::mem::transmute(pullength), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Register_Device_InterfaceW<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(dndevinst: u32, interfaceclassguid: *const ::windows::core::GUID, pszreference: Param2, pszdeviceinterface: ::windows::core::PWSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Register_Device_InterfaceW(dndevinst: u32, interfaceclassguid: *const ::windows::core::GUID, pszreference: ::windows::core::PCWSTR, pszdeviceinterface: ::windows::core::PWSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Register_Device_InterfaceW(::core::mem::transmute(dndevinst), ::core::mem::transmute(interfaceclassguid), pszreference.into_param().abi(), ::core::mem::transmute(pszdeviceinterface), ::core::mem::transmute(pullength), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Register_Device_Interface_ExA<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(dndevinst: u32, interfaceclassguid: *const ::windows::core::GUID, pszreference: Param2, pszdeviceinterface: ::windows::core::PSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Register_Device_Interface_ExA(dndevinst: u32, interfaceclassguid: *const ::windows::core::GUID, pszreference: ::windows::core::PCSTR, pszdeviceinterface: ::windows::core::PSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Register_Device_Interface_ExA(::core::mem::transmute(dndevinst), ::core::mem::transmute(interfaceclassguid), pszreference.into_param().abi(), ::core::mem::transmute(pszdeviceinterface), ::core::mem::transmute(pullength), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Register_Device_Interface_ExW<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(dndevinst: u32, interfaceclassguid: *const ::windows::core::GUID, pszreference: Param2, pszdeviceinterface: ::windows::core::PWSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Register_Device_Interface_ExW(dndevinst: u32, interfaceclassguid: *const ::windows::core::GUID, pszreference: ::windows::core::PCWSTR, pszdeviceinterface: ::windows::core::PWSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Register_Device_Interface_ExW(::core::mem::transmute(dndevinst), ::core::mem::transmute(interfaceclassguid), pszreference.into_param().abi(), ::core::mem::transmute(pszdeviceinterface), ::core::mem::transmute(pullength), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CM_Register_Notification(pfilter: *const CM_NOTIFY_FILTER, pcontext: *const ::core::ffi::c_void, pcallback: PCM_NOTIFY_CALLBACK, pnotifycontext: *mut isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Register_Notification(pfilter: *const CM_NOTIFY_FILTER, pcontext: *const ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, pnotifycontext: *mut isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Register_Notification(::core::mem::transmute(pfilter), ::core::mem::transmute(pcontext), ::core::mem::transmute(pcallback), ::core::mem::transmute(pnotifycontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Remove_SubTree(dnancestor: u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Remove_SubTree(dnancestor: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Remove_SubTree(::core::mem::transmute(dnancestor), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Remove_SubTree_Ex(dnancestor: u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Remove_SubTree_Ex(dnancestor: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Remove_SubTree_Ex(::core::mem::transmute(dnancestor), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Request_Device_EjectA(dndevinst: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: &mut [u8], ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Request_Device_EjectA(dndevinst: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: ::windows::core::PSTR, ulnamelength: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Request_Device_EjectA(::core::mem::transmute(dndevinst), ::core::mem::transmute(pvetotype), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszvetoname)), pszvetoname.len() as _, ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Request_Device_EjectW(dndevinst: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: &mut [u16], ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Request_Device_EjectW(dndevinst: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: ::windows::core::PWSTR, ulnamelength: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Request_Device_EjectW(::core::mem::transmute(dndevinst), ::core::mem::transmute(pvetotype), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszvetoname)), pszvetoname.len() as _, ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Request_Device_Eject_ExA(dndevinst: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: &mut [u8], ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Request_Device_Eject_ExA(dndevinst: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: ::windows::core::PSTR, ulnamelength: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Request_Device_Eject_ExA(::core::mem::transmute(dndevinst), ::core::mem::transmute(pvetotype), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszvetoname)), pszvetoname.len() as _, ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Request_Device_Eject_ExW(dndevinst: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: &mut [u16], ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Request_Device_Eject_ExW(dndevinst: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: ::windows::core::PWSTR, ulnamelength: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Request_Device_Eject_ExW(::core::mem::transmute(dndevinst), ::core::mem::transmute(pvetotype), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszvetoname)), pszvetoname.len() as _, ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Request_Eject_PC() -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Request_Eject_PC() -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Request_Eject_PC())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Request_Eject_PC_Ex(hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Request_Eject_PC_Ex(hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Request_Eject_PC_Ex(::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Run_Detection(ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Run_Detection(ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Run_Detection(::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Run_Detection_Ex(ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Run_Detection_Ex(ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Run_Detection_Ex(::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SETUP_BITS: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SETUP_DEVINST_CONFIG: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SETUP_DEVINST_CONFIG_CLASS: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SETUP_DEVINST_CONFIG_EXTENSIONS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SETUP_DEVINST_CONFIG_RESET: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SETUP_DEVINST_READY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SETUP_DEVINST_RESET: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SETUP_DEVNODE_CONFIG: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SETUP_DEVNODE_CONFIG_CLASS: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SETUP_DEVNODE_CONFIG_EXTENSIONS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SETUP_DEVNODE_CONFIG_RESET: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SETUP_DEVNODE_READY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SETUP_DEVNODE_RESET: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SETUP_DOWNLOAD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SETUP_PROP_CHANGE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SETUP_WRITE_LOG_CONFS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SET_DEVINST_PROBLEM_BITS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SET_DEVINST_PROBLEM_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SET_DEVINST_PROBLEM_OVERRIDE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SET_DEVNODE_PROBLEM_BITS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SET_DEVNODE_PROBLEM_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SET_DEVNODE_PROBLEM_OVERRIDE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SET_HW_PROF_FLAGS_BITS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SET_HW_PROF_FLAGS_UI_NOT_OK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn CM_Set_Class_PropertyW(classguid: *const ::windows::core::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Set_Class_PropertyW(classguid: *const ::windows::core::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Set_Class_PropertyW(::core::mem::transmute(classguid), ::core::mem::transmute(propertykey), ::core::mem::transmute(propertytype), ::core::mem::transmute(propertybuffer), ::core::mem::transmute(propertybuffersize), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn CM_Set_Class_Property_ExW(classguid: *const ::windows::core::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Set_Class_Property_ExW(classguid: *const ::windows::core::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Set_Class_Property_ExW(::core::mem::transmute(classguid), ::core::mem::transmute(propertykey), ::core::mem::transmute(propertytype), ::core::mem::transmute(propertybuffer), ::core::mem::transmute(propertybuffersize), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Set_Class_Registry_PropertyA(classguid: *const ::windows::core::GUID, ulproperty: u32, buffer: *const ::core::ffi::c_void, ullength: u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Set_Class_Registry_PropertyA(classguid: *const ::windows::core::GUID, ulproperty: u32, buffer: *const ::core::ffi::c_void, ullength: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Set_Class_Registry_PropertyA(::core::mem::transmute(classguid), ::core::mem::transmute(ulproperty), ::core::mem::transmute(buffer), ::core::mem::transmute(ullength), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Set_Class_Registry_PropertyW(classguid: *const ::windows::core::GUID, ulproperty: u32, buffer: *const ::core::ffi::c_void, ullength: u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Set_Class_Registry_PropertyW(classguid: *const ::windows::core::GUID, ulproperty: u32, buffer: *const ::core::ffi::c_void, ullength: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Set_Class_Registry_PropertyW(::core::mem::transmute(classguid), ::core::mem::transmute(ulproperty), ::core::mem::transmute(buffer), ::core::mem::transmute(ullength), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Set_DevNode_Problem(dndevinst: u32, ulproblem: u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Set_DevNode_Problem(dndevinst: u32, ulproblem: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Set_DevNode_Problem(::core::mem::transmute(dndevinst), ::core::mem::transmute(ulproblem), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Set_DevNode_Problem_Ex(dndevinst: u32, ulproblem: u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Set_DevNode_Problem_Ex(dndevinst: u32, ulproblem: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Set_DevNode_Problem_Ex(::core::mem::transmute(dndevinst), ::core::mem::transmute(ulproblem), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn CM_Set_DevNode_PropertyW(dndevinst: u32, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Set_DevNode_PropertyW(dndevinst: u32, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Set_DevNode_PropertyW(::core::mem::transmute(dndevinst), ::core::mem::transmute(propertykey), ::core::mem::transmute(propertytype), ::core::mem::transmute(propertybuffer), ::core::mem::transmute(propertybuffersize), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn CM_Set_DevNode_Property_ExW(dndevinst: u32, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Set_DevNode_Property_ExW(dndevinst: u32, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Set_DevNode_Property_ExW(::core::mem::transmute(dndevinst), ::core::mem::transmute(propertykey), ::core::mem::transmute(propertytype), ::core::mem::transmute(propertybuffer), ::core::mem::transmute(propertybuffersize), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Set_DevNode_Registry_PropertyA(dndevinst: u32, ulproperty: u32, buffer: *const ::core::ffi::c_void, ullength: u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Set_DevNode_Registry_PropertyA(dndevinst: u32, ulproperty: u32, buffer: *const ::core::ffi::c_void, ullength: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Set_DevNode_Registry_PropertyA(::core::mem::transmute(dndevinst), ::core::mem::transmute(ulproperty), ::core::mem::transmute(buffer), ::core::mem::transmute(ullength), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Set_DevNode_Registry_PropertyW(dndevinst: u32, ulproperty: u32, buffer: *const ::core::ffi::c_void, ullength: u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Set_DevNode_Registry_PropertyW(dndevinst: u32, ulproperty: u32, buffer: *const ::core::ffi::c_void, ullength: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Set_DevNode_Registry_PropertyW(::core::mem::transmute(dndevinst), ::core::mem::transmute(ulproperty), ::core::mem::transmute(buffer), ::core::mem::transmute(ullength), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Set_DevNode_Registry_Property_ExA(dndevinst: u32, ulproperty: u32, buffer: *const ::core::ffi::c_void, ullength: u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Set_DevNode_Registry_Property_ExA(dndevinst: u32, ulproperty: u32, buffer: *const ::core::ffi::c_void, ullength: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Set_DevNode_Registry_Property_ExA(::core::mem::transmute(dndevinst), ::core::mem::transmute(ulproperty), ::core::mem::transmute(buffer), ::core::mem::transmute(ullength), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Set_DevNode_Registry_Property_ExW(dndevinst: u32, ulproperty: u32, buffer: *const ::core::ffi::c_void, ullength: u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Set_DevNode_Registry_Property_ExW(dndevinst: u32, ulproperty: u32, buffer: *const ::core::ffi::c_void, ullength: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Set_DevNode_Registry_Property_ExW(::core::mem::transmute(dndevinst), ::core::mem::transmute(ulproperty), ::core::mem::transmute(buffer), ::core::mem::transmute(ullength), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn CM_Set_Device_Interface_PropertyW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pszdeviceinterface: Param0, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Set_Device_Interface_PropertyW(pszdeviceinterface: ::windows::core::PCWSTR, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Set_Device_Interface_PropertyW(pszdeviceinterface.into_param().abi(), ::core::mem::transmute(propertykey), ::core::mem::transmute(propertytype), ::core::mem::transmute(propertybuffer), ::core::mem::transmute(propertybuffersize), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn CM_Set_Device_Interface_Property_ExW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pszdeviceinterface: Param0, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Set_Device_Interface_Property_ExW(pszdeviceinterface: ::windows::core::PCWSTR, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Set_Device_Interface_Property_ExW(pszdeviceinterface.into_param().abi(), ::core::mem::transmute(propertykey), ::core::mem::transmute(propertytype), ::core::mem::transmute(propertybuffer), ::core::mem::transmute(propertybuffersize), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Set_HW_Prof(ulhardwareprofile: u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Set_HW_Prof(ulhardwareprofile: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Set_HW_Prof(::core::mem::transmute(ulhardwareprofile), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Set_HW_Prof_Ex(ulhardwareprofile: u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Set_HW_Prof_Ex(ulhardwareprofile: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Set_HW_Prof_Ex(::core::mem::transmute(ulhardwareprofile), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Set_HW_Prof_FlagsA(pdeviceid: *const i8, ulconfig: u32, ulvalue: u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Set_HW_Prof_FlagsA(pdeviceid: *const i8, ulconfig: u32, ulvalue: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Set_HW_Prof_FlagsA(::core::mem::transmute(pdeviceid), ::core::mem::transmute(ulconfig), ::core::mem::transmute(ulvalue), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Set_HW_Prof_FlagsW(pdeviceid: *const u16, ulconfig: u32, ulvalue: u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Set_HW_Prof_FlagsW(pdeviceid: *const u16, ulconfig: u32, ulvalue: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Set_HW_Prof_FlagsW(::core::mem::transmute(pdeviceid), ::core::mem::transmute(ulconfig), ::core::mem::transmute(ulvalue), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Set_HW_Prof_Flags_ExA(pdeviceid: *const i8, ulconfig: u32, ulvalue: u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Set_HW_Prof_Flags_ExA(pdeviceid: *const i8, ulconfig: u32, ulvalue: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Set_HW_Prof_Flags_ExA(::core::mem::transmute(pdeviceid), ::core::mem::transmute(ulconfig), ::core::mem::transmute(ulvalue), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Set_HW_Prof_Flags_ExW(pdeviceid: *const u16, ulconfig: u32, ulvalue: u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Set_HW_Prof_Flags_ExW(pdeviceid: *const u16, ulconfig: u32, ulvalue: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Set_HW_Prof_Flags_ExW(::core::mem::transmute(pdeviceid), ::core::mem::transmute(ulconfig), ::core::mem::transmute(ulvalue), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Setup_DevNode(dndevinst: u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Setup_DevNode(dndevinst: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Setup_DevNode(::core::mem::transmute(dndevinst), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Setup_DevNode_Ex(dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Setup_DevNode_Ex(dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Setup_DevNode_Ex(::core::mem::transmute(dndevinst), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Test_Range_Available(ullstartvalue: u64, ullendvalue: u64, rlh: usize, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Test_Range_Available(ullstartvalue: u64, ullendvalue: u64, rlh: usize, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Test_Range_Available(::core::mem::transmute(ullstartvalue), ::core::mem::transmute(ullendvalue), ::core::mem::transmute(rlh), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Uninstall_DevNode(dndevinst: u32, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Uninstall_DevNode(dndevinst: u32, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Uninstall_DevNode(::core::mem::transmute(dndevinst), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Uninstall_DevNode_Ex(dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Uninstall_DevNode_Ex(dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Uninstall_DevNode_Ex(::core::mem::transmute(dndevinst), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Unregister_Device_InterfaceA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(pszdeviceinterface: Param0, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Unregister_Device_InterfaceA(pszdeviceinterface: ::windows::core::PCSTR, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Unregister_Device_InterfaceA(pszdeviceinterface.into_param().abi(), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Unregister_Device_InterfaceW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pszdeviceinterface: Param0, ulflags: u32) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Unregister_Device_InterfaceW(pszdeviceinterface: ::windows::core::PCWSTR, ulflags: u32) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Unregister_Device_InterfaceW(pszdeviceinterface.into_param().abi(), ::core::mem::transmute(ulflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Unregister_Device_Interface_ExA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(pszdeviceinterface: Param0, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Unregister_Device_Interface_ExA(pszdeviceinterface: ::windows::core::PCSTR, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Unregister_Device_Interface_ExA(pszdeviceinterface.into_param().abi(), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Unregister_Device_Interface_ExW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pszdeviceinterface: Param0, ulflags: u32, hmachine: isize) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Unregister_Device_Interface_ExW(pszdeviceinterface: ::windows::core::PCWSTR, ulflags: u32, hmachine: isize) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Unregister_Device_Interface_ExW(pszdeviceinterface.into_param().abi(), ::core::mem::transmute(ulflags), ::core::mem::transmute(hmachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn CM_Unregister_Notification<'a, Param0: ::windows::core::IntoParam<'a, HCMNOTIFICATION>>(notifycontext: Param0) -> CONFIGRET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CM_Unregister_Notification(notifycontext: HCMNOTIFICATION) -> CONFIGRET;
        }
        ::core::mem::transmute(CM_Unregister_Notification(notifycontext.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct COINSTALLER_CONTEXT_DATA {
    pub PostProcessing: super::super::Foundation::BOOL,
    pub InstallResult: u32,
    pub PrivateData: *mut ::core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COINSTALLER_CONTEXT_DATA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COINSTALLER_CONTEXT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for COINSTALLER_CONTEXT_DATA {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for COINSTALLER_CONTEXT_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COINSTALLER_CONTEXT_DATA>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for COINSTALLER_CONTEXT_DATA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COINSTALLER_CONTEXT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct COINSTALLER_CONTEXT_DATA {
    pub PostProcessing: super::super::Foundation::BOOL,
    pub InstallResult: u32,
    pub PrivateData: *mut ::core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COINSTALLER_CONTEXT_DATA {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COINSTALLER_CONTEXT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for COINSTALLER_CONTEXT_DATA {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for COINSTALLER_CONTEXT_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COINSTALLER_CONTEXT_DATA>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for COINSTALLER_CONTEXT_DATA {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COINSTALLER_CONTEXT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CONFIGMG_VERSION: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CONFIGRET(pub u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_SUCCESS: CONFIGRET = CONFIGRET(0u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_DEFAULT: CONFIGRET = CONFIGRET(1u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_OUT_OF_MEMORY: CONFIGRET = CONFIGRET(2u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_POINTER: CONFIGRET = CONFIGRET(3u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_FLAG: CONFIGRET = CONFIGRET(4u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_DEVNODE: CONFIGRET = CONFIGRET(5u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_DEVINST: CONFIGRET = CONFIGRET(5u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_RES_DES: CONFIGRET = CONFIGRET(6u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_LOG_CONF: CONFIGRET = CONFIGRET(7u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_ARBITRATOR: CONFIGRET = CONFIGRET(8u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_NODELIST: CONFIGRET = CONFIGRET(9u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_DEVNODE_HAS_REQS: CONFIGRET = CONFIGRET(10u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_DEVINST_HAS_REQS: CONFIGRET = CONFIGRET(10u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_RESOURCEID: CONFIGRET = CONFIGRET(11u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_DLVXD_NOT_FOUND: CONFIGRET = CONFIGRET(12u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_NO_SUCH_DEVNODE: CONFIGRET = CONFIGRET(13u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_NO_SUCH_DEVINST: CONFIGRET = CONFIGRET(13u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_NO_MORE_LOG_CONF: CONFIGRET = CONFIGRET(14u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_NO_MORE_RES_DES: CONFIGRET = CONFIGRET(15u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_ALREADY_SUCH_DEVNODE: CONFIGRET = CONFIGRET(16u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_ALREADY_SUCH_DEVINST: CONFIGRET = CONFIGRET(16u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_RANGE_LIST: CONFIGRET = CONFIGRET(17u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_RANGE: CONFIGRET = CONFIGRET(18u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_FAILURE: CONFIGRET = CONFIGRET(19u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_NO_SUCH_LOGICAL_DEV: CONFIGRET = CONFIGRET(20u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_CREATE_BLOCKED: CONFIGRET = CONFIGRET(21u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_NOT_SYSTEM_VM: CONFIGRET = CONFIGRET(22u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_REMOVE_VETOED: CONFIGRET = CONFIGRET(23u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_APM_VETOED: CONFIGRET = CONFIGRET(24u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_LOAD_TYPE: CONFIGRET = CONFIGRET(25u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_BUFFER_SMALL: CONFIGRET = CONFIGRET(26u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_NO_ARBITRATOR: CONFIGRET = CONFIGRET(27u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_NO_REGISTRY_HANDLE: CONFIGRET = CONFIGRET(28u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_REGISTRY_ERROR: CONFIGRET = CONFIGRET(29u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_DEVICE_ID: CONFIGRET = CONFIGRET(30u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_DATA: CONFIGRET = CONFIGRET(31u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_API: CONFIGRET = CONFIGRET(32u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_DEVLOADER_NOT_READY: CONFIGRET = CONFIGRET(33u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_NEED_RESTART: CONFIGRET = CONFIGRET(34u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_NO_MORE_HW_PROFILES: CONFIGRET = CONFIGRET(35u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_DEVICE_NOT_THERE: CONFIGRET = CONFIGRET(36u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_NO_SUCH_VALUE: CONFIGRET = CONFIGRET(37u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_WRONG_TYPE: CONFIGRET = CONFIGRET(38u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_PRIORITY: CONFIGRET = CONFIGRET(39u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_NOT_DISABLEABLE: CONFIGRET = CONFIGRET(40u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_FREE_RESOURCES: CONFIGRET = CONFIGRET(41u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_QUERY_VETOED: CONFIGRET = CONFIGRET(42u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_CANT_SHARE_IRQ: CONFIGRET = CONFIGRET(43u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_NO_DEPENDENT: CONFIGRET = CONFIGRET(44u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_SAME_RESOURCES: CONFIGRET = CONFIGRET(45u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_NO_SUCH_REGISTRY_KEY: CONFIGRET = CONFIGRET(46u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_MACHINENAME: CONFIGRET = CONFIGRET(47u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_REMOTE_COMM_FAILURE: CONFIGRET = CONFIGRET(48u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_MACHINE_UNAVAILABLE: CONFIGRET = CONFIGRET(49u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_NO_CM_SERVICES: CONFIGRET = CONFIGRET(50u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_ACCESS_DENIED: CONFIGRET = CONFIGRET(51u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_CALL_NOT_IMPLEMENTED: CONFIGRET = CONFIGRET(52u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_PROPERTY: CONFIGRET = CONFIGRET(53u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_DEVICE_INTERFACE_ACTIVE: CONFIGRET = CONFIGRET(54u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_NO_SUCH_DEVICE_INTERFACE: CONFIGRET = CONFIGRET(55u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_REFERENCE_STRING: CONFIGRET = CONFIGRET(56u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_CONFLICT_LIST: CONFIGRET = CONFIGRET(57u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_INDEX: CONFIGRET = CONFIGRET(58u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_STRUCTURE_SIZE: CONFIGRET = CONFIGRET(59u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NUM_CR_RESULTS: CONFIGRET = CONFIGRET(60u32);
impl ::core::marker::Copy for CONFIGRET {}
impl ::core::clone::Clone for CONFIGRET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CONFIGRET {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CONFIGRET {
    type Abi = Self;
}
impl ::core::fmt::Debug for CONFIGRET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONFIGRET").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CONFLICT_DETAILS_A {
    pub CD_ulSize: u32,
    pub CD_ulMask: u32,
    pub CD_dnDevInst: u32,
    pub CD_rdResDes: usize,
    pub CD_ulFlags: u32,
    pub CD_szDescription: [super::super::Foundation::CHAR; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CONFLICT_DETAILS_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CONFLICT_DETAILS_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CONFLICT_DETAILS_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONFLICT_DETAILS_A").field("CD_ulSize", &self.CD_ulSize).field("CD_ulMask", &self.CD_ulMask).field("CD_dnDevInst", &self.CD_dnDevInst).field("CD_rdResDes", &self.CD_rdResDes).field("CD_ulFlags", &self.CD_ulFlags).field("CD_szDescription", &self.CD_szDescription).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CONFLICT_DETAILS_A {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CONFLICT_DETAILS_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONFLICT_DETAILS_A>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CONFLICT_DETAILS_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CONFLICT_DETAILS_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct CONFLICT_DETAILS_W {
    pub CD_ulSize: u32,
    pub CD_ulMask: u32,
    pub CD_dnDevInst: u32,
    pub CD_rdResDes: usize,
    pub CD_ulFlags: u32,
    pub CD_szDescription: [u16; 260],
}
impl ::core::marker::Copy for CONFLICT_DETAILS_W {}
impl ::core::clone::Clone for CONFLICT_DETAILS_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CONFLICT_DETAILS_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONFLICT_DETAILS_W").field("CD_ulSize", &self.CD_ulSize).field("CD_ulMask", &self.CD_ulMask).field("CD_dnDevInst", &self.CD_dnDevInst).field("CD_rdResDes", &self.CD_rdResDes).field("CD_ulFlags", &self.CD_ulFlags).field("CD_szDescription", &self.CD_szDescription).finish()
    }
}
unsafe impl ::windows::core::Abi for CONFLICT_DETAILS_W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CONFLICT_DETAILS_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONFLICT_DETAILS_W>()) == 0 }
    }
}
impl ::core::cmp::Eq for CONFLICT_DETAILS_W {}
impl ::core::default::Default for CONFLICT_DETAILS_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const COPYFLG_FORCE_FILE_IN_USE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const COPYFLG_IN_USE_TRY_RENAME: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const COPYFLG_NODECOMP: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const COPYFLG_NOPRUNE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const COPYFLG_NOSKIP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const COPYFLG_NOVERSIONCHECK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const COPYFLG_NO_OVERWRITE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const COPYFLG_NO_VERSION_DIALOG: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const COPYFLG_OVERWRITE_OLDER_ONLY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const COPYFLG_PROTECTED_WINDOWS_DRIVER_FILE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const COPYFLG_REPLACEONLY: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const COPYFLG_REPLACE_BOOT_FILE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const COPYFLG_WARN_IF_SKIP: u32 = 1u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct CS_DES {
    pub CSD_SignatureLength: u32,
    pub CSD_LegacyDataOffset: u32,
    pub CSD_LegacyDataSize: u32,
    pub CSD_Flags: u32,
    pub CSD_ClassGuid: ::windows::core::GUID,
    pub CSD_Signature: [u8; 1],
}
impl ::core::marker::Copy for CS_DES {}
impl ::core::clone::Clone for CS_DES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CS_DES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CS_DES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CS_DES>()) == 0 }
    }
}
impl ::core::cmp::Eq for CS_DES {}
impl ::core::default::Default for CS_DES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct CS_RESOURCE {
    pub CS_Header: CS_DES,
}
impl ::core::marker::Copy for CS_RESOURCE {}
impl ::core::clone::Clone for CS_RESOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CS_RESOURCE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CS_RESOURCE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CS_RESOURCE>()) == 0 }
    }
}
impl ::core::cmp::Eq for CS_RESOURCE {}
impl ::core::default::Default for CS_RESOURCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct Connection_Des_s {
    pub COND_Type: u32,
    pub COND_Flags: u32,
    pub COND_Class: u8,
    pub COND_ClassType: u8,
    pub COND_Reserved1: u8,
    pub COND_Reserved2: u8,
    pub COND_Id: i64,
}
impl ::core::marker::Copy for Connection_Des_s {}
impl ::core::clone::Clone for Connection_Des_s {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for Connection_Des_s {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for Connection_Des_s {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<Connection_Des_s>()) == 0 }
    }
}
impl ::core::cmp::Eq for Connection_Des_s {}
impl ::core::default::Default for Connection_Des_s {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct Connection_Resource_s {
    pub Connection_Header: Connection_Des_s,
}
impl ::core::marker::Copy for Connection_Resource_s {}
impl ::core::clone::Clone for Connection_Resource_s {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for Connection_Resource_s {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for Connection_Resource_s {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<Connection_Resource_s>()) == 0 }
    }
}
impl ::core::cmp::Eq for Connection_Resource_s {}
impl ::core::default::Default for Connection_Resource_s {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DELFLG_IN_USE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DELFLG_IN_USE1: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIBCI_NODISPLAYCLASS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIBCI_NOINSTALLCLASS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DICD_GENERATE_ID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DICD_INHERIT_CLASSDRVS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DICLASSPROP_INSTALLER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DICLASSPROP_INTERFACE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DICS_DISABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DICS_ENABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DICS_FLAG_CONFIGGENERAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DICS_FLAG_CONFIGSPECIFIC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DICS_FLAG_GLOBAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DICS_PROPCHANGE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DICS_START: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DICS_STOP: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DICUSTOMDEVPROP_MERGE_MULTISZ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_ADDPROPERTYPAGE_ADVANCED: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_ADDPROPERTYPAGE_BASIC: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_ADDREMOTEPROPERTYPAGE_ADVANCED: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_ALLOW_INSTALL: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_ASSIGNRESOURCES: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_CALCDISKSPACE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_DESTROYPRIVATEDATA: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_DESTROYWIZARDDATA: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_DETECT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_DETECTCANCEL: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_DETECTVERIFY: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_ENABLECLASS: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_FINISHINSTALL_ACTION: u32 = 42u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_FIRSTTIMESETUP: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_FOUNDDEVICE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_INSTALLCLASSDRIVERS: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_INSTALLDEVICE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_INSTALLDEVICEFILES: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_INSTALLINTERFACES: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_INSTALLWIZARD: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_MOVEDEVICE: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_NEWDEVICEWIZARD_FINISHINSTALL: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_NEWDEVICEWIZARD_POSTANALYZE: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_NEWDEVICEWIZARD_PREANALYZE: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_NEWDEVICEWIZARD_PRESELECT: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_NEWDEVICEWIZARD_SELECT: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_POWERMESSAGEWAKE: u32 = 39u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_PROPERTIES: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_PROPERTYCHANGE: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_REGISTERDEVICE: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_REGISTER_COINSTALLERS: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_REMOVE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_RESERVED1: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_RESERVED2: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_SELECTBESTCOMPATDRV: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_SELECTCLASSDRIVERS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_SELECTDEVICE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_TROUBLESHOOTER: u32 = 38u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_UNREMOVE: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_UNUSED1: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_UPDATEDRIVER_UI: u32 = 41u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_VALIDATECLASSDRIVERS: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_VALIDATEDRIVER: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIGCDP_FLAG_ADVANCED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIGCDP_FLAG_BASIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIGCDP_FLAG_REMOTE_ADVANCED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIGCDP_FLAG_REMOTE_BASIC: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIGCF_ALLCLASSES: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIGCF_DEFAULT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIGCF_DEVICEINTERFACE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIGCF_INTERFACEDEVICE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIGCF_PRESENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIGCF_PROFILE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIIDFLAG_BITS: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIIDFLAG_INSTALLCOPYINFDRIVERS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIIDFLAG_INSTALLNULLDRIVER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIIDFLAG_NOFINISHINSTALLUI: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIIDFLAG_SHOWSEARCHUI: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIIRFLAG_FORCE_INF: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIIRFLAG_HOTPATCH: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIIRFLAG_HW_USING_THE_INF: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIIRFLAG_INF_ALREADY_COPIED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIIRFLAG_INSTALL_AS_SET: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIIRFLAG_NOBACKUP: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIIRFLAG_PRE_CONFIGURE_INF: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIOCR_INSTALLER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIOCR_INTERFACE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIODI_NO_ADD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIOD_CANCEL_REMOVE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIOD_INHERIT_CLASSDRVS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIREG_BOTH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIREG_DEV: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIREG_DRV: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_ABSOLUTE: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_ABSOLUTE_16BIT: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_APPS: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_BOOT: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_COLOR: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_COMMON_APPDATA: u32 = 16419u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_COMMON_DESKTOPDIRECTORY: u32 = 16409u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_COMMON_DOCUMENTS: u32 = 16430u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_COMMON_FAVORITES: u32 = 16415u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_COMMON_PROGRAMS: u32 = 16407u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_COMMON_STARTMENU: u32 = 16406u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_COMMON_STARTUP: u32 = 16408u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_COMMON_TEMPLATES: u32 = 16429u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_DEFAULT: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_DRIVERS: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_DRIVER_STORE: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_FONTS: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_HELP: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_INF: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_IOSUBSYS: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_LOADER: u32 = 54u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_NULL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_PRINTPROCESSOR: u32 = 55u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_PROGRAM_FILES: u32 = 16422u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_PROGRAM_FILES_COMMON: u32 = 16427u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_PROGRAM_FILES_COMMONX86: u32 = 16428u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_PROGRAM_FILES_X86: u32 = 16426u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_SHARED: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_SPOOL: u32 = 51u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_SPOOLDRIVERS: u32 = 52u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_SRCPATH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_SYSTEM: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_SYSTEM16: u32 = 50u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_SYSTEM_X86: u32 = 16425u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_USER: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_USERPROFILE: u32 = 53u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_VIEWERS: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_WINDOWS: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIURFLAG_NO_REMOVE_INF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIURFLAG_RESERVED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_AUTOASSIGNRES: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_CLASSINSTALLPARAMS: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_COMPAT_FROM_CLASS: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_DIDCLASS: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_DIDCOMPAT: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_DISABLED: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_DONOTCALLCONFIGMG: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_DRIVERPAGE_ADDED: i32 = 67108864i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_ENUMSINGLEINF: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_ALLOWEXCLUDEDDRVS: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_ALTPLATFORM_DRVSEARCH: i32 = 268435456i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_ALWAYSWRITEIDS: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_APPENDDRIVERLIST: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_BACKUPONREPLACE: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_CI_FAILED: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_DEVICECHANGE: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_DIDCOMPATINFO: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_DIDINFOLIST: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_DRIVERLIST_FROM_URL: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_EXCLUDE_OLD_INET_DRIVERS: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_FILTERCLASSES: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_FILTERSIMILARDRIVERS: i32 = 33554432i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_FINISHINSTALL_ACTION: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_INET_DRIVER: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_INSTALLEDDRIVER: i32 = 67108864i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_IN_SYSTEM_SETUP: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_NOUIONQUERYREMOVE: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_NO_CLASSLIST_NODE_MERGE: i32 = 134217728i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_NO_DRVREG_MODIFY: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_POWERPAGE_ADDED: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_PREINSTALLBACKUP: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_PROPCHANGE_PENDING: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_RECURSIVESEARCH: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_RESERVED1: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_RESERVED2: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_RESERVED3: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_RESERVED4: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_RESTART_DEVICE_ONLY: i32 = 536870912i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_SEARCH_PUBLISHED_INFS: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_SETFAILEDINSTALL: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_USECLASSFORCOMPAT: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FORCECOPY: i32 = 33554432i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_GENERALPAGE_ADDED: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_INF_IS_SORTED: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_INSTALLDISABLED: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_MULTMFGS: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_NEEDREBOOT: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_NEEDRESTART: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_NOBROWSE: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_NODI_DEFAULTACTION: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_NOFILECOPY: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_NOSELECTICONS: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_NOVCP: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_NOWRITE_IDS: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_OVERRIDE_INFFLAGS: i32 = 268435456i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_PROPERTIES_CHANGE: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_PROPS_NOCHANGEUSAGE: i32 = 536870912i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_QUIETINSTALL: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_REMOVEDEVICE_CONFIGSPECIFIC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_REMOVEDEVICE_GLOBAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_RESOURCEPAGE_ADDED: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_SHOWALL: i32 = 7i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_SHOWCLASS: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_SHOWCOMPAT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_SHOWOEM: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_UNREMOVEDEVICE_CONFIGSPECIFIC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_USECI_SELECTSTRINGS: i32 = 134217728i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct DMA_DES {
    pub DD_Count: u32,
    pub DD_Type: u32,
    pub DD_Flags: u32,
    pub DD_Alloc_Chan: u32,
}
impl ::core::marker::Copy for DMA_DES {}
impl ::core::clone::Clone for DMA_DES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DMA_DES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DMA_DES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DMA_DES>()) == 0 }
    }
}
impl ::core::cmp::Eq for DMA_DES {}
impl ::core::default::Default for DMA_DES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct DMA_RANGE {
    pub DR_Min: u32,
    pub DR_Max: u32,
    pub DR_Flags: u32,
}
impl ::core::marker::Copy for DMA_RANGE {}
impl ::core::clone::Clone for DMA_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DMA_RANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DMA_RANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DMA_RANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for DMA_RANGE {}
impl ::core::default::Default for DMA_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct DMA_RESOURCE {
    pub DMA_Header: DMA_DES,
    pub DMA_Data: [DMA_RANGE; 1],
}
impl ::core::marker::Copy for DMA_RESOURCE {}
impl ::core::clone::Clone for DMA_RESOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DMA_RESOURCE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DMA_RESOURCE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DMA_RESOURCE>()) == 0 }
    }
}
impl ::core::cmp::Eq for DMA_RESOURCE {}
impl ::core::default::Default for DMA_RESOURCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DMI_BKCOLOR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DMI_MASK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DMI_USERECT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_ALWAYSEXCLUDEFROMLIST: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_AUTHENTICODE_SIGNED: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_BAD_DRIVER: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_BASIC_DRIVER: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_CLASS_DRIVER: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_COMPATIBLE_DRIVER: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_DUPDESC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_DUPDRIVERVER: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_DUPPROVIDER: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_EXCLUDEFROMLIST: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_INBOX_DRIVER: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_INET_DRIVER: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_INF_IS_SIGNED: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_INSTALLEDDRIVER: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_LEGACYINF: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_NODRIVER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_OEM_F6_INF: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_OLDDRIVER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_OLD_INET_DRIVER: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_REQUESTADDITIONALSOFTWARE: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_UNUSED1: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_UNUSED2: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_UNUSED_22: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_UNUSED_23: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_UNUSED_24: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_UNUSED_25: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_UNUSED_26: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_UNUSED_27: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_UNUSED_28: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_UNUSED_29: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_UNUSED_30: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_UNUSED_31: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_APM_DRIVER: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_APM_ENUMERATOR: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_ARM_WAKEUP: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_BAD_PARTIAL: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_BOOT_LOG_PROB: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_CHILD_WITH_INVALID_ID: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_DEVICE_DISCONNECTED: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_DISABLEABLE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_DRIVER_BLOCKED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_DRIVER_LOADED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_ENUM_LOADED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_FILTERED: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_HARDWARE_ENUM: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_HAS_MARK: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_HAS_PROBLEM: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_LEGACY_DRIVER: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_LIAR: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_MANUAL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_MF_CHILD: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_MF_PARENT: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_MOVED: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_NEEDS_LOCKING: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_NEED_RESTART: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_NEED_TO_ENUM: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_NOT_FIRST_TIME: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_NOT_FIRST_TIMEE: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_NO_SHOW_IN_DM: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_NT_DRIVER: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_NT_ENUMERATOR: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_PRIVATE_PROBLEM: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_QUERY_REMOVE_ACTIVE: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_QUERY_REMOVE_PENDING: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_REBAL_CANDIDATE: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_REMOVABLE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_ROOT_ENUMERATED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_SILENT_INSTALL: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_STARTED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_STOP_FREE_RES: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_WILL_BE_REMOVED: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DPROMPT_BUFFERTOOSMALL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DPROMPT_CANCEL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DPROMPT_OUTOFMEMORY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DPROMPT_SKIPFILE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DPROMPT_SUCCESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DRIVER_COMPATID_RANK: u32 = 16383u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DRIVER_HARDWAREID_MASK: u32 = 2147487743u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DRIVER_HARDWAREID_RANK: u32 = 4095u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DRIVER_UNTRUSTED_COMPATID_RANK: u32 = 49151u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DRIVER_UNTRUSTED_HARDWAREID_RANK: u32 = 36863u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DRIVER_UNTRUSTED_RANK: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DRIVER_W9X_SUSPECT_COMPATID_RANK: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DRIVER_W9X_SUSPECT_HARDWAREID_RANK: u32 = 53247u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DRIVER_W9X_SUSPECT_RANK: u32 = 3221225472u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DWORD_MAX: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DYNAWIZ_FLAG_ANALYZE_HANDLECONFLICT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DYNAWIZ_FLAG_INSTALLDET_NEXT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DYNAWIZ_FLAG_INSTALLDET_PREV: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DYNAWIZ_FLAG_PAGESADDED: u32 = 1u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct DevPrivate_Des_s {
    pub PD_Count: u32,
    pub PD_Type: u32,
    pub PD_Data1: u32,
    pub PD_Data2: u32,
    pub PD_Data3: u32,
    pub PD_Flags: u32,
}
impl ::core::marker::Copy for DevPrivate_Des_s {}
impl ::core::clone::Clone for DevPrivate_Des_s {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DevPrivate_Des_s {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DevPrivate_Des_s {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DevPrivate_Des_s>()) == 0 }
    }
}
impl ::core::cmp::Eq for DevPrivate_Des_s {}
impl ::core::default::Default for DevPrivate_Des_s {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct DevPrivate_Range_s {
    pub PR_Data1: u32,
    pub PR_Data2: u32,
    pub PR_Data3: u32,
}
impl ::core::marker::Copy for DevPrivate_Range_s {}
impl ::core::clone::Clone for DevPrivate_Range_s {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DevPrivate_Range_s {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DevPrivate_Range_s {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DevPrivate_Range_s>()) == 0 }
    }
}
impl ::core::cmp::Eq for DevPrivate_Range_s {}
impl ::core::default::Default for DevPrivate_Range_s {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct DevPrivate_Resource_s {
    pub PRV_Header: DevPrivate_Des_s,
    pub PRV_Data: [DevPrivate_Range_s; 1],
}
impl ::core::marker::Copy for DevPrivate_Resource_s {}
impl ::core::clone::Clone for DevPrivate_Resource_s {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DevPrivate_Resource_s {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DevPrivate_Resource_s {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DevPrivate_Resource_s>()) == 0 }
    }
}
impl ::core::cmp::Eq for DevPrivate_Resource_s {}
impl ::core::default::Default for DevPrivate_Resource_s {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DiInstallDevice<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwndparent: Param0, deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *const SP_DRVINFO_DATA_V2_A, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DiInstallDevice(hwndparent: super::super::Foundation::HWND, deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *const SP_DRVINFO_DATA_V2_A, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DiInstallDevice(hwndparent.into_param().abi(), ::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(driverinfodata), ::core::mem::transmute(flags), ::core::mem::transmute(needreboot)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DiInstallDriverA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hwndparent: Param0, infpath: Param1, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DiInstallDriverA(hwndparent: super::super::Foundation::HWND, infpath: ::windows::core::PCSTR, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DiInstallDriverA(hwndparent.into_param().abi(), infpath.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(needreboot)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DiInstallDriverW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hwndparent: Param0, infpath: Param1, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DiInstallDriverW(hwndparent: super::super::Foundation::HWND, infpath: ::windows::core::PCWSTR, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DiInstallDriverW(hwndparent.into_param().abi(), infpath.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(needreboot)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DiRollbackDriver<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, hwndparent: Param2, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DiRollbackDriver(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, hwndparent: super::super::Foundation::HWND, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DiRollbackDriver(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), hwndparent.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(needreboot)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DiShowUpdateDevice<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwndparent: Param0, deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DiShowUpdateDevice(hwndparent: super::super::Foundation::HWND, deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DiShowUpdateDevice(hwndparent.into_param().abi(), ::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(flags), ::core::mem::transmute(needreboot)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DiShowUpdateDriver<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hwndparent: Param0, filepath: Param1, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DiShowUpdateDriver(hwndparent: super::super::Foundation::HWND, filepath: ::windows::core::PCWSTR, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DiShowUpdateDriver(hwndparent.into_param().abi(), filepath.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(needreboot)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DiUninstallDevice<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwndparent: Param0, deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DiUninstallDevice(hwndparent: super::super::Foundation::HWND, deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DiUninstallDevice(hwndparent.into_param().abi(), ::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(flags), ::core::mem::transmute(needreboot)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DiUninstallDriverA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hwndparent: Param0, infpath: Param1, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DiUninstallDriverA(hwndparent: super::super::Foundation::HWND, infpath: ::windows::core::PCSTR, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DiUninstallDriverA(hwndparent.into_param().abi(), infpath.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(needreboot)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DiUninstallDriverW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hwndparent: Param0, infpath: Param1, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DiUninstallDriverW(hwndparent: super::super::Foundation::HWND, infpath: ::windows::core::PCWSTR, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DiUninstallDriverW(hwndparent.into_param().abi(), infpath.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(needreboot)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ENABLECLASS_FAILURE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ENABLECLASS_QUERY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ENABLECLASS_SUCCESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FILEOP_ABORT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FILEOP_BACKUP: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FILEOP_DOIT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FILEOP_NEWPATH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FILEOP_RENAME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FILEOP_RETRY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FILEOP_SKIP: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct FILEPATHS_A {
    pub Target: ::windows::core::PCSTR,
    pub Source: ::windows::core::PCSTR,
    pub Win32Error: u32,
    pub Flags: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for FILEPATHS_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for FILEPATHS_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for FILEPATHS_A {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for FILEPATHS_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILEPATHS_A>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for FILEPATHS_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for FILEPATHS_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct FILEPATHS_A {
    pub Target: ::windows::core::PCSTR,
    pub Source: ::windows::core::PCSTR,
    pub Win32Error: u32,
    pub Flags: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for FILEPATHS_A {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for FILEPATHS_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for FILEPATHS_A {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for FILEPATHS_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILEPATHS_A>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for FILEPATHS_A {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for FILEPATHS_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct FILEPATHS_SIGNERINFO_A {
    pub Target: ::windows::core::PCSTR,
    pub Source: ::windows::core::PCSTR,
    pub Win32Error: u32,
    pub Flags: u32,
    pub DigitalSigner: ::windows::core::PCSTR,
    pub Version: ::windows::core::PCSTR,
    pub CatalogFile: ::windows::core::PCSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for FILEPATHS_SIGNERINFO_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for FILEPATHS_SIGNERINFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for FILEPATHS_SIGNERINFO_A {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for FILEPATHS_SIGNERINFO_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILEPATHS_SIGNERINFO_A>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for FILEPATHS_SIGNERINFO_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for FILEPATHS_SIGNERINFO_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct FILEPATHS_SIGNERINFO_A {
    pub Target: ::windows::core::PCSTR,
    pub Source: ::windows::core::PCSTR,
    pub Win32Error: u32,
    pub Flags: u32,
    pub DigitalSigner: ::windows::core::PCSTR,
    pub Version: ::windows::core::PCSTR,
    pub CatalogFile: ::windows::core::PCSTR,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for FILEPATHS_SIGNERINFO_A {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for FILEPATHS_SIGNERINFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for FILEPATHS_SIGNERINFO_A {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for FILEPATHS_SIGNERINFO_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILEPATHS_SIGNERINFO_A>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for FILEPATHS_SIGNERINFO_A {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for FILEPATHS_SIGNERINFO_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct FILEPATHS_SIGNERINFO_W {
    pub Target: ::windows::core::PCWSTR,
    pub Source: ::windows::core::PCWSTR,
    pub Win32Error: u32,
    pub Flags: u32,
    pub DigitalSigner: ::windows::core::PCWSTR,
    pub Version: ::windows::core::PCWSTR,
    pub CatalogFile: ::windows::core::PCWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for FILEPATHS_SIGNERINFO_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for FILEPATHS_SIGNERINFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for FILEPATHS_SIGNERINFO_W {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for FILEPATHS_SIGNERINFO_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILEPATHS_SIGNERINFO_W>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for FILEPATHS_SIGNERINFO_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for FILEPATHS_SIGNERINFO_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct FILEPATHS_SIGNERINFO_W {
    pub Target: ::windows::core::PCWSTR,
    pub Source: ::windows::core::PCWSTR,
    pub Win32Error: u32,
    pub Flags: u32,
    pub DigitalSigner: ::windows::core::PCWSTR,
    pub Version: ::windows::core::PCWSTR,
    pub CatalogFile: ::windows::core::PCWSTR,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for FILEPATHS_SIGNERINFO_W {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for FILEPATHS_SIGNERINFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for FILEPATHS_SIGNERINFO_W {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for FILEPATHS_SIGNERINFO_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILEPATHS_SIGNERINFO_W>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for FILEPATHS_SIGNERINFO_W {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for FILEPATHS_SIGNERINFO_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct FILEPATHS_W {
    pub Target: ::windows::core::PCWSTR,
    pub Source: ::windows::core::PCWSTR,
    pub Win32Error: u32,
    pub Flags: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for FILEPATHS_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for FILEPATHS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for FILEPATHS_W {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for FILEPATHS_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILEPATHS_W>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for FILEPATHS_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for FILEPATHS_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct FILEPATHS_W {
    pub Target: ::windows::core::PCWSTR,
    pub Source: ::windows::core::PCWSTR,
    pub Win32Error: u32,
    pub Flags: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for FILEPATHS_W {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for FILEPATHS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for FILEPATHS_W {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for FILEPATHS_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILEPATHS_W>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for FILEPATHS_W {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for FILEPATHS_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FILE_COMPRESSION_MSZIP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FILE_COMPRESSION_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FILE_COMPRESSION_NTCAB: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FILE_COMPRESSION_WINLZA: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_IN_CABINET_INFO_A {
    pub NameInCabinet: ::windows::core::PCSTR,
    pub FileSize: u32,
    pub Win32Error: u32,
    pub DosDate: u16,
    pub DosTime: u16,
    pub DosAttribs: u16,
    pub FullTargetName: [super::super::Foundation::CHAR; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_IN_CABINET_INFO_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_IN_CABINET_INFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FILE_IN_CABINET_INFO_A {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FILE_IN_CABINET_INFO_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_IN_CABINET_INFO_A>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FILE_IN_CABINET_INFO_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FILE_IN_CABINET_INFO_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_IN_CABINET_INFO_A {
    pub NameInCabinet: ::windows::core::PCSTR,
    pub FileSize: u32,
    pub Win32Error: u32,
    pub DosDate: u16,
    pub DosTime: u16,
    pub DosAttribs: u16,
    pub FullTargetName: [super::super::Foundation::CHAR; 260],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_IN_CABINET_INFO_A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_IN_CABINET_INFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FILE_IN_CABINET_INFO_A {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FILE_IN_CABINET_INFO_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_IN_CABINET_INFO_A>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FILE_IN_CABINET_INFO_A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FILE_IN_CABINET_INFO_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct FILE_IN_CABINET_INFO_W {
    pub NameInCabinet: ::windows::core::PCWSTR,
    pub FileSize: u32,
    pub Win32Error: u32,
    pub DosDate: u16,
    pub DosTime: u16,
    pub DosAttribs: u16,
    pub FullTargetName: [u16; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for FILE_IN_CABINET_INFO_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for FILE_IN_CABINET_INFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for FILE_IN_CABINET_INFO_W {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for FILE_IN_CABINET_INFO_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_IN_CABINET_INFO_W>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for FILE_IN_CABINET_INFO_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for FILE_IN_CABINET_INFO_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct FILE_IN_CABINET_INFO_W {
    pub NameInCabinet: ::windows::core::PCWSTR,
    pub FileSize: u32,
    pub Win32Error: u32,
    pub DosDate: u16,
    pub DosTime: u16,
    pub DosAttribs: u16,
    pub FullTargetName: [u16; 260],
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for FILE_IN_CABINET_INFO_W {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for FILE_IN_CABINET_INFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for FILE_IN_CABINET_INFO_W {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for FILE_IN_CABINET_INFO_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_IN_CABINET_INFO_W>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for FILE_IN_CABINET_INFO_W {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for FILE_IN_CABINET_INFO_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FILTERED_LOG_CONF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_ADDPROPERTY_AND: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_ADDPROPERTY_APPEND: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_ADDPROPERTY_NOCLOBBER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_ADDPROPERTY_OR: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_ADDPROPERTY_OVERWRITEONLY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_ADDREG_32BITKEY: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_ADDREG_64BITKEY: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_ADDREG_APPEND: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_ADDREG_BINVALUETYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_ADDREG_DELREG_BIT: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_ADDREG_DELVAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_ADDREG_KEYONLY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_ADDREG_KEYONLY_COMMON: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_ADDREG_NOCLOBBER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_ADDREG_OVERWRITEONLY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_ADDREG_TYPE_EXPAND_SZ: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_ADDREG_TYPE_MULTI_SZ: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_ADDREG_TYPE_SZ: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_BITREG_32BITKEY: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_BITREG_64BITKEY: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_BITREG_CLEARBITS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_BITREG_SETBITS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_DELPROPERTY_MULTI_SZ_DELSTRING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_DELREG_32BITKEY: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_DELREG_64BITKEY: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_DELREG_KEYONLY_COMMON: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_DELREG_OPERATION_MASK: u32 = 254u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_DELREG_TYPE_EXPAND_SZ: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_DELREG_TYPE_MULTI_SZ: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_DELREG_TYPE_SZ: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_DELREG_VALUE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_INI2REG_32BITKEY: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_INI2REG_64BITKEY: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_PROFITEM_CSIDL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_PROFITEM_CURRENTUSER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_PROFITEM_DELETE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_PROFITEM_GROUP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_REGSVR_DLLINSTALL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_REGSVR_DLLREGISTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FORCED_LOG_CONF: u32 = 4u32;
pub const GUID_ACPI_CMOS_INTERFACE_STANDARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a8d0384_6505_40ca_bc39_56c15f8c5fed);
pub const GUID_ACPI_INTERFACE_STANDARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb091a08a_ba97_11d0_bd14_00aa00b7b32a);
pub const GUID_ACPI_INTERFACE_STANDARD2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8695f63_1831_4870_a8cf_9c2f03f9dcb5);
pub const GUID_ACPI_PORT_RANGES_INTERFACE_STANDARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf14f609b_cbbd_4957_a674_bc00213f1c97);
pub const GUID_ACPI_REGS_INTERFACE_STANDARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06141966_7245_6369_462e_4e656c736f6e);
pub const GUID_AGP_TARGET_BUS_INTERFACE_STANDARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb15cfce8_06d1_4d37_9d4c_bedde0c2a6ff);
pub const GUID_ARBITER_INTERFACE_STANDARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe644f185_8c0e_11d0_becf_08002be2092f);
pub const GUID_BUS_INTERFACE_STANDARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x496b8280_6f25_11d0_beaf_08002be2092f);
pub const GUID_BUS_RESOURCE_UPDATE_INTERFACE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27d0102d_bfb2_4164_81dd_dbb82f968b48);
pub const GUID_BUS_TYPE_1394: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf74e73eb_9ac5_45eb_be4d_772cc71ddfb3);
pub const GUID_BUS_TYPE_ACPI: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7b46895_001a_4942_891f_a7d46610a843);
pub const GUID_BUS_TYPE_AVC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc06ff265_ae09_48f0_812c_16753d7cba83);
pub const GUID_BUS_TYPE_DOT4PRT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x441ee001_4342_11d5_a184_00c04f60524d);
pub const GUID_BUS_TYPE_EISA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xddc35509_f3fc_11d0_a537_0000f8753ed1);
pub const GUID_BUS_TYPE_HID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeeaf37d0_1963_47c4_aa48_72476db7cf49);
pub const GUID_BUS_TYPE_INTERNAL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1530ea73_086b_11d1_a09f_00c04fc340b1);
pub const GUID_BUS_TYPE_IRDA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ae17dc1_c944_44d6_881f_4c2e61053bc1);
pub const GUID_BUS_TYPE_ISAPNP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe676f854_d87d_11d0_92b2_00a0c9055fc5);
pub const GUID_BUS_TYPE_LPTENUM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4ca1000_2ddc_11d5_a17a_00c04f60524d);
pub const GUID_BUS_TYPE_MCA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c75997a_dc33_11d0_92b2_00a0c9055fc5);
pub const GUID_BUS_TYPE_PCI: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8ebdfb0_b510_11d0_80e5_00a0c92542e3);
pub const GUID_BUS_TYPE_PCMCIA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09343630_af9f_11d0_92e9_0000f81e1b30);
pub const GUID_BUS_TYPE_SCM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x375a5912_804c_45aa_bdc2_fdd25a1d9512);
pub const GUID_BUS_TYPE_SD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe700cc04_4036_4e89_9579_89ebf45f00cd);
pub const GUID_BUS_TYPE_SERENUM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77114a87_8944_11d1_bd90_00a0c906be2d);
pub const GUID_BUS_TYPE_SW_DEVICE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06d10322_7de0_4cef_8e25_197d0e7442e2);
pub const GUID_BUS_TYPE_USB: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d7debbc_c85d_11d1_9eb4_006008c3a19a);
pub const GUID_BUS_TYPE_USBPRINT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x441ee000_4342_11d5_a184_00c04f60524d);
pub const GUID_D3COLD_AUX_POWER_AND_TIMING_INTERFACE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0044d8aa_f664_4588_9ffc_2afeaf5950b9);
pub const GUID_D3COLD_SUPPORT_INTERFACE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb38290e5_3cd0_4f9d_9937_f5fe2b44d47a);
pub const GUID_DEVCLASS_1394: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6bdd1fc1_810f_11d0_bec7_08002be2092f);
pub const GUID_DEVCLASS_1394DEBUG: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66f250d6_7801_4a64_b139_eea80a450b24);
pub const GUID_DEVCLASS_61883: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ebefbc0_3200_11d2_b4c2_00a0c9697d07);
pub const GUID_DEVCLASS_ADAPTER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d36e964_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_APMSUPPORT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd45b1c18_c8fa_11d1_9f77_0000f805f530);
pub const GUID_DEVCLASS_AVC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc06ff265_ae09_48f0_812c_16753d7cba83);
pub const GUID_DEVCLASS_BATTERY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72631e54_78a4_11d0_bcf7_00aa00b7b32a);
pub const GUID_DEVCLASS_BIOMETRIC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53d29ef7_377c_4d14_864b_eb3a85769359);
pub const GUID_DEVCLASS_BLUETOOTH: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0cbf06c_cd8b_4647_bb8a_263b43f0f974);
pub const GUID_DEVCLASS_CAMERA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca3e7ab9_b4c3_4ae6_8251_579ef933890f);
pub const GUID_DEVCLASS_CDROM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d36e965_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_COMPUTEACCELERATOR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf01a9d53_3ff6_48d2_9f97_c8a7004be10c);
pub const GUID_DEVCLASS_COMPUTER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d36e966_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_DECODER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6bdd1fc2_810f_11d0_bec7_08002be2092f);
pub const GUID_DEVCLASS_DISKDRIVE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d36e967_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_DISPLAY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d36e968_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_DOT4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x48721b56_6795_11d2_b1a8_0080c72e74a2);
pub const GUID_DEVCLASS_DOT4PRINT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49ce6ac8_6f86_11d2_b1e5_0080c72e74a2);
pub const GUID_DEVCLASS_EHSTORAGESILO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9da2b80f_f89f_4a49_a5c2_511b085b9e8a);
pub const GUID_DEVCLASS_ENUM1394: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc459df55_db08_11d1_b009_00a0c9081ff6);
pub const GUID_DEVCLASS_EXTENSION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2f84ce7_8efa_411c_aa69_97454ca4cb57);
pub const GUID_DEVCLASS_FDC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d36e969_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_FIRMWARE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2e7dd72_6468_4e36_b6f1_6488f42c1b52);
pub const GUID_DEVCLASS_FLOPPYDISK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d36e980_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_FSFILTER_ACTIVITYMONITOR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb86dff51_a31e_4bac_b3cf_e8cfe75c9fc2);
pub const GUID_DEVCLASS_FSFILTER_ANTIVIRUS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1d1a169_c54f_4379_81db_bee7d88d7454);
pub const GUID_DEVCLASS_FSFILTER_BOTTOM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x37765ea0_5958_4fc9_b04b_2fdfef97e59e);
pub const GUID_DEVCLASS_FSFILTER_CFSMETADATASERVER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcdcf0939_b75b_4630_bf76_80f7ba655884);
pub const GUID_DEVCLASS_FSFILTER_COMPRESSION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3586baf_b5aa_49b5_8d6c_0569284c639f);
pub const GUID_DEVCLASS_FSFILTER_CONTENTSCREENER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e3f0674_c83c_4558_bb26_9820e1eba5c5);
pub const GUID_DEVCLASS_FSFILTER_CONTINUOUSBACKUP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71aa14f8_6fad_4622_ad77_92bb9d7e6947);
pub const GUID_DEVCLASS_FSFILTER_COPYPROTECTION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89786ff1_9c12_402f_9c9e_17753c7f4375);
pub const GUID_DEVCLASS_FSFILTER_ENCRYPTION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0a701c0_a511_42ff_aa6c_06dc0395576f);
pub const GUID_DEVCLASS_FSFILTER_HSM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd546500a_2aeb_45f6_9482_f4b1799c3177);
pub const GUID_DEVCLASS_FSFILTER_INFRASTRUCTURE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe55fa6f9_128c_4d04_abab_630c74b1453a);
pub const GUID_DEVCLASS_FSFILTER_OPENFILEBACKUP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8ecafa6_66d1_41a5_899b_66585d7216b7);
pub const GUID_DEVCLASS_FSFILTER_PHYSICALQUOTAMANAGEMENT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a0a8e78_bba6_4fc4_a709_1e33cd09d67e);
pub const GUID_DEVCLASS_FSFILTER_QUOTAMANAGEMENT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8503c911_a6c7_4919_8f79_5028f5866b0c);
pub const GUID_DEVCLASS_FSFILTER_REPLICATION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x48d3ebc4_4cf8_48ff_b869_9c68ad42eb9f);
pub const GUID_DEVCLASS_FSFILTER_SECURITYENHANCER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd02bc3da_0c8e_4945_9bd5_f1883c226c8c);
pub const GUID_DEVCLASS_FSFILTER_SYSTEM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d1b9aaa_01e2_46af_849f_272b3f324c46);
pub const GUID_DEVCLASS_FSFILTER_SYSTEMRECOVERY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2db15374_706e_4131_a0c7_d7c78eb0289a);
pub const GUID_DEVCLASS_FSFILTER_TOP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb369baf4_5568_4e82_a87e_a93eb16bca87);
pub const GUID_DEVCLASS_FSFILTER_UNDELETE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe8f1572_c67a_48c0_bbac_0b5c6d66cafb);
pub const GUID_DEVCLASS_FSFILTER_VIRTUALIZATION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf75a86c0_10d8_4c3a_b233_ed60e4cdfaac);
pub const GUID_DEVCLASS_GPS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6bdd1fc3_810f_11d0_bec7_08002be2092f);
pub const GUID_DEVCLASS_HDC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d36e96a_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_HIDCLASS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x745a17a0_74d3_11d0_b6fe_00a0c90f57da);
pub const GUID_DEVCLASS_HOLOGRAPHIC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd612553d_06b1_49ca_8938_e39ef80eb16f);
pub const GUID_DEVCLASS_IMAGE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6bdd1fc6_810f_11d0_bec7_08002be2092f);
pub const GUID_DEVCLASS_INFINIBAND: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30ef7132_d858_4a0c_ac24_b9028a5cca3f);
pub const GUID_DEVCLASS_INFRARED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6bdd1fc5_810f_11d0_bec7_08002be2092f);
pub const GUID_DEVCLASS_KEYBOARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d36e96b_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_LEGACYDRIVER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ecc055d_047f_11d1_a537_0000f8753ed1);
pub const GUID_DEVCLASS_MEDIA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d36e96c_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_MEDIUM_CHANGER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce5939ae_ebde_11d0_b181_0000f8753ec4);
pub const GUID_DEVCLASS_MEMORY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5099944a_f6b9_4057_a056_8c550228544c);
pub const GUID_DEVCLASS_MODEM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d36e96d_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_MONITOR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d36e96e_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_MOUSE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d36e96f_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_MTD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d36e970_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_MULTIFUNCTION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d36e971_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_MULTIPORTSERIAL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50906cb8_ba12_11d1_bf5d_0000f805f530);
pub const GUID_DEVCLASS_NET: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d36e972_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_NETCLIENT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d36e973_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_NETDRIVER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87ef9ad1_8f70_49ee_b215_ab1fcadcbe3c);
pub const GUID_DEVCLASS_NETSERVICE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d36e974_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_NETTRANS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d36e975_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_NETUIO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78912bc1_cb8e_4b28_a329_f322ebadbe0f);
pub const GUID_DEVCLASS_NODRIVER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d36e976_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_PCMCIA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d36e977_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_PNPPRINTERS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4658ee7e_f050_11d1_b6bd_00c04fa372a7);
pub const GUID_DEVCLASS_PORTS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d36e978_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_PRINTER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d36e979_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_PRINTERUPGRADE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d36e97a_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_PRINTQUEUE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ed2bbf9_11f0_4084_b21f_ad83a8e6dcdc);
pub const GUID_DEVCLASS_PROCESSOR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50127dc3_0f36_415e_a6cc_4cb3be910b65);
pub const GUID_DEVCLASS_SBP2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd48179be_ec20_11d1_b6b8_00c04fa372a7);
pub const GUID_DEVCLASS_SCMDISK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53966cb1_4d46_4166_bf23_c522403cd495);
pub const GUID_DEVCLASS_SCMVOLUME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53ccb149_e543_4c84_b6e0_bce4f6b7e806);
pub const GUID_DEVCLASS_SCSIADAPTER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d36e97b_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_SECURITYACCELERATOR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x268c95a1_edfe_11d3_95c3_0010dc4050a5);
pub const GUID_DEVCLASS_SENSOR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5175d334_c371_4806_b3ba_71fd53c9258d);
pub const GUID_DEVCLASS_SIDESHOW: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x997b5d8d_c442_4f2e_baf3_9c8e671e9e21);
pub const GUID_DEVCLASS_SMARTCARDREADER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50dd5230_ba8a_11d1_bf5d_0000f805f530);
pub const GUID_DEVCLASS_SMRDISK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53487c23_680f_4585_acc3_1f10d6777e82);
pub const GUID_DEVCLASS_SMRVOLUME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53b3cf03_8f5a_4788_91b6_d19ed9fcccbf);
pub const GUID_DEVCLASS_SOFTWARECOMPONENT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c4c3332_344d_483c_8739_259e934c9cc8);
pub const GUID_DEVCLASS_SOUND: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d36e97c_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_SYSTEM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d36e97d_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_TAPEDRIVE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d807884_7d21_11cf_801c_08002be10318);
pub const GUID_DEVCLASS_UCM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6f1aa1c_7f3b_4473_b2e8_c97d8ac71d53);
pub const GUID_DEVCLASS_UNKNOWN: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d36e97e_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVCLASS_USB: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36fc9e60_c465_11cf_8056_444553540000);
pub const GUID_DEVCLASS_VOLUME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71a27cdd_812a_11d0_bec7_08002be2092f);
pub const GUID_DEVCLASS_VOLUMESNAPSHOT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x533c5b84_ec70_11d2_9505_00c04f79deaf);
pub const GUID_DEVCLASS_WCEUSBS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25dbce51_6c8f_4a72_8a6d_b54c2b4fc835);
pub const GUID_DEVCLASS_WPD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeec5ad98_8080_425f_922a_dabf3de3f69a);
pub const GUID_DEVICE_INTERFACE_ARRIVAL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb3a4004_46f0_11d0_b08f_00609713053f);
pub const GUID_DEVICE_INTERFACE_REMOVAL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb3a4005_46f0_11d0_b08f_00609713053f);
pub const GUID_DEVICE_RESET_INTERFACE_STANDARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x649fdf26_3bc0_4813_ad24_7e0c1eda3fa3);
pub const GUID_DMA_CACHE_COHERENCY_INTERFACE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb520f7fa_8a5a_4e40_a3f6_6be1e162d935);
pub const GUID_HWPROFILE_CHANGE_CANCELLED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb3a4002_46f0_11d0_b08f_00609713053f);
pub const GUID_HWPROFILE_CHANGE_COMPLETE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb3a4003_46f0_11d0_b08f_00609713053f);
pub const GUID_HWPROFILE_QUERY_CHANGE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb3a4001_46f0_11d0_b08f_00609713053f);
pub const GUID_INT_ROUTE_INTERFACE_STANDARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70941bf4_0073_11d1_a09e_00c04fc340b1);
pub const GUID_IOMMU_BUS_INTERFACE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1efee0b2_d278_4ae4_bddc_1b34dd648043);
pub const GUID_KERNEL_SOFT_RESTART_CANCEL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31d737e7_8c0b_468a_956e_9f433ec358fb);
pub const GUID_KERNEL_SOFT_RESTART_FINALIZE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20e91abd_350a_4d4f_8577_99c81507473a);
pub const GUID_KERNEL_SOFT_RESTART_PREPARE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde373def_a85c_4f76_8cbf_f96bea8bd10f);
pub const GUID_LEGACY_DEVICE_DETECTION_STANDARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50feb0de_596a_11d2_a5b8_0000f81a4619);
pub const GUID_MF_ENUMERATION_INTERFACE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaeb895f0_5586_11d1_8d84_00a0c906b244);
pub const GUID_MSIX_TABLE_CONFIG_INTERFACE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a6a460b_194f_455d_b34b_b84c5b05712b);
pub const GUID_NPEM_CONTROL_INTERFACE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d95573d_b774_488a_b120_4f284a9eff51);
pub const GUID_PARTITION_UNIT_INTERFACE_STANDARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52363f5b_d891_429b_8195_aec5fef6853c);
pub const GUID_PCC_INTERFACE_INTERNAL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7cce62ce_c189_4814_a6a7_12112089e938);
pub const GUID_PCC_INTERFACE_STANDARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ee8ba63_0f59_4a24_8a45_35808bdd1249);
pub const GUID_PCI_ATS_INTERFACE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x010a7fe8_96f5_4943_bedf_95e651b93412);
pub const GUID_PCI_BUS_INTERFACE_STANDARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x496b8281_6f25_11d0_beaf_08002be2092f);
pub const GUID_PCI_BUS_INTERFACE_STANDARD2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde94e966_fdff_4c9c_9998_6747b150e74c);
pub const GUID_PCI_DEVICE_PRESENT_INTERFACE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1b82c26_bf49_45ef_b216_71cbd7889b57);
pub const GUID_PCI_EXPRESS_LINK_QUIESCENT_INTERFACE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x146cd41c_dae3_4437_8aff_2af3f038099b);
pub const GUID_PCI_EXPRESS_ROOT_PORT_INTERFACE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83a7734a_84c7_4161_9a98_6000ed0c4a33);
pub const GUID_PCI_FPGA_CONTROL_INTERFACE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2df3f7a8_b9b3_4063_9215_b5d14a0b266e);
pub const GUID_PCI_PTM_CONTROL_INTERFACE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x348a5ebb_ba24_44b7_9916_285687735117);
pub const GUID_PCI_SECURITY_INTERFACE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e7f1451_199e_4acc_ba2d_762b4edf4674);
pub const GUID_PCI_VIRTUALIZATION_INTERFACE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64897b47_3a4a_4d75_bc74_89dd6c078293);
pub const GUID_PCMCIA_BUS_INTERFACE_STANDARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76173af0_c504_11d1_947f_00c04fb960ee);
pub const GUID_PNP_CUSTOM_NOTIFICATION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaca73f8e_8d23_11d1_ac7d_0000f87571d0);
pub const GUID_PNP_EXTENDED_ADDRESS_INTERFACE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8e992ec_a797_4dc4_8846_84d041707446);
pub const GUID_PNP_LOCATION_INTERFACE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70211b0e_0afb_47db_afc1_410bf842497a);
pub const GUID_PNP_POWER_NOTIFICATION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2cf0660_eb7a_11d1_bd7f_0000f87571d0);
pub const GUID_PNP_POWER_SETTING_CHANGE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29c69b3e_c79a_43bf_bbde_a932fa1bea7e);
pub const GUID_POWER_DEVICE_ENABLE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x827c0a6f_feb0_11d0_bd26_00aa00b7b32a);
pub const GUID_POWER_DEVICE_TIMEOUTS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa45da735_feb0_11d0_bd26_00aa00b7b32a);
pub const GUID_POWER_DEVICE_WAKE_ENABLE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9546a82_feb0_11d0_bd26_00aa00b7b32a);
pub const GUID_PROCESSOR_PCC_INTERFACE_STANDARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x37b17e9a_c21c_4296_972d_11c4b32b28f0);
pub const GUID_QUERY_CRASHDUMP_FUNCTIONS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9cc6b8ff_32e2_4834_b1de_b32ef8880a4b);
pub const GUID_RECOVERY_NVMED_PREPARE_SHUTDOWN: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b9770ea_bde7_400b_a9b9_4f684f54cc2a);
pub const GUID_RECOVERY_PCI_PREPARE_SHUTDOWN: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90d889de_8704_44cf_8115_ed8528d2b2da);
pub const GUID_REENUMERATE_SELF_INTERFACE_STANDARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2aeb0243_6a6e_486b_82fc_d815f6b97006);
pub const GUID_SCM_BUS_INTERFACE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25944783_ce79_4232_815e_4a30014e8eb4);
pub const GUID_SCM_BUS_LD_INTERFACE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b89307d_d76b_4f48_b186_54041ae92e8d);
pub const GUID_SCM_BUS_NVD_INTERFACE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8de064ff_b630_42e4_88ea_6f24c8641175);
pub const GUID_SCM_PHYSICAL_NVDIMM_INTERFACE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0079c21b_917e_405e_a9ce_0732b5bbcebd);
pub const GUID_SDEV_IDENTIFIER_INTERFACE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49d67af8_916c_4ee8_9df1_889f17d21e91);
pub const GUID_SECURE_DRIVER_INTERFACE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x370f67e1_4ff5_4a94_9a35_06c5d9cc30e2);
pub const GUID_TARGET_DEVICE_QUERY_REMOVE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb3a4006_46f0_11d0_b08f_00609713053f);
pub const GUID_TARGET_DEVICE_REMOVE_CANCELLED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb3a4007_46f0_11d0_b08f_00609713053f);
pub const GUID_TARGET_DEVICE_REMOVE_COMPLETE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb3a4008_46f0_11d0_b08f_00609713053f);
pub const GUID_TARGET_DEVICE_TRANSPORT_RELATIONS_CHANGED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcf528f6_a82f_47b1_ad3a_8050594cad28);
pub const GUID_THERMAL_COOLING_INTERFACE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecbe47a8_c498_4bb9_bd70_e867e0940d22);
pub const GUID_TRANSLATOR_INTERFACE_STANDARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c154a92_aacf_11d0_8d2a_00a0c906b244);
pub const GUID_WUDF_DEVICE_HOST_PROBLEM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc43d25bd_9346_40ee_a2d2_d70c15f8b75b);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HCMNOTIFICATION(pub isize);
impl HCMNOTIFICATION {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
    pub fn ok(self) -> ::windows::core::Result<Self> {
        if !self.is_invalid() {
            Ok(self)
        } else {
            Err(::windows::core::Error::from_win32())
        }
    }
}
impl ::core::default::Default for HCMNOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HCMNOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HCMNOTIFICATION {}
impl ::core::fmt::Debug for HCMNOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HCMNOTIFICATION").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for HCMNOTIFICATION {
    type Abi = Self;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HWProfileInfo_sA {
    pub HWPI_ulHWProfile: u32,
    pub HWPI_szFriendlyName: [super::super::Foundation::CHAR; 80],
    pub HWPI_dwFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HWProfileInfo_sA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HWProfileInfo_sA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HWProfileInfo_sA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HWProfileInfo_sA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HWProfileInfo_sA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HWProfileInfo_sA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HWProfileInfo_sA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct HWProfileInfo_sW {
    pub HWPI_ulHWProfile: u32,
    pub HWPI_szFriendlyName: [u16; 80],
    pub HWPI_dwFlags: u32,
}
impl ::core::marker::Copy for HWProfileInfo_sW {}
impl ::core::clone::Clone for HWProfileInfo_sW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HWProfileInfo_sW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HWProfileInfo_sW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HWProfileInfo_sW>()) == 0 }
    }
}
impl ::core::cmp::Eq for HWProfileInfo_sW {}
impl ::core::default::Default for HWProfileInfo_sW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDD_DYNAWIZ_ANALYZEDEV_PAGE: u32 = 10010u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDD_DYNAWIZ_ANALYZE_NEXTPAGE: u32 = 10004u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDD_DYNAWIZ_ANALYZE_PREVPAGE: u32 = 10003u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDD_DYNAWIZ_FIRSTPAGE: u32 = 10000u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDD_DYNAWIZ_INSTALLDETECTEDDEVS_PAGE: u32 = 10011u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDD_DYNAWIZ_INSTALLDETECTED_NEXTPAGE: u32 = 10007u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDD_DYNAWIZ_INSTALLDETECTED_NODEVS: u32 = 10008u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDD_DYNAWIZ_INSTALLDETECTED_PREVPAGE: u32 = 10006u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDD_DYNAWIZ_SELECTCLASS_PAGE: u32 = 10012u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDD_DYNAWIZ_SELECTDEV_PAGE: u32 = 10009u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDD_DYNAWIZ_SELECT_NEXTPAGE: u32 = 10002u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDD_DYNAWIZ_SELECT_PREVPAGE: u32 = 10001u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDF_CHECKFIRST: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDF_NOBEEP: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDF_NOBROWSE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDF_NOCOMPRESSED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDF_NODETAILS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDF_NOFOREGROUND: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDF_NOREMOVABLEMEDIAPROMPT: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDF_NOSKIP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDF_OEMDISK: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDF_USEDISKNAMEASPROMPT: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDF_WARNIFSKIP: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDI_CLASSICON_OVERLAYFIRST: u32 = 500u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDI_CLASSICON_OVERLAYLAST: u32 = 502u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDI_CONFLICT: u32 = 161u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDI_DISABLED_OVL: u32 = 501u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDI_FORCED_OVL: u32 = 502u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDI_PROBLEM_OVL: u32 = 500u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDI_RESOURCE: u32 = 159u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDI_RESOURCEFIRST: u32 = 159u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDI_RESOURCELAST: u32 = 161u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDI_RESOURCEOVERLAYFIRST: u32 = 161u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDI_RESOURCEOVERLAYLAST: u32 = 161u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct INFCONTEXT {
    pub Inf: *mut ::core::ffi::c_void,
    pub CurrentInf: *mut ::core::ffi::c_void,
    pub Section: u32,
    pub Line: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for INFCONTEXT {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for INFCONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for INFCONTEXT {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for INFCONTEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INFCONTEXT>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for INFCONTEXT {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for INFCONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct INFCONTEXT {
    pub Inf: *mut ::core::ffi::c_void,
    pub CurrentInf: *mut ::core::ffi::c_void,
    pub Section: u32,
    pub Line: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for INFCONTEXT {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for INFCONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for INFCONTEXT {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for INFCONTEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INFCONTEXT>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for INFCONTEXT {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for INFCONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFINFO_DEFAULT_SEARCH: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFINFO_INF_NAME_IS_ABSOLUTE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFINFO_INF_PATH_LIST_SEARCH: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFINFO_INF_SPEC_IS_HINF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFINFO_REVERSE_DEFAULT_SEARCH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_BUS_ALL: &'static str = "BUS_ALL";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_BUS_EISA: &'static str = "BUS_EISA";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_BUS_ISA: &'static str = "BUS_ISA";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_BUS_MCA: &'static str = "BUS_MCA";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_CFGPRI_DESIRED: &'static str = "DESIRED";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_CFGPRI_DISABLED: &'static str = "DISABLED";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_CFGPRI_FORCECONFIG: &'static str = "FORCECONFIG";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_CFGPRI_HARDRECONFIG: &'static str = "HARDRECONFIG";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_CFGPRI_HARDWIRED: &'static str = "HARDWIRED";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_CFGPRI_NORMAL: &'static str = "NORMAL";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_CFGPRI_POWEROFF: &'static str = "POWEROFF";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_CFGPRI_REBOOT: &'static str = "REBOOT";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_CFGPRI_RESTART: &'static str = "RESTART";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_CFGPRI_SUBOPTIMAL: &'static str = "SUBOPTIMAL";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_CFGTYPE_BASIC: &'static str = "BASIC";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_CFGTYPE_FORCED: &'static str = "FORCED";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_CFGTYPE_OVERRIDE: &'static str = "OVERRIDE";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_CLASS_SAFEEXCL: &'static str = "SAFE_EXCL";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_CONTROLFLAGS_SECTION: &'static str = "ControlFlags";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_DRIVERSELECT_FUNCTIONS: &'static str = "DriverSelectFunctions";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_DRIVERSELECT_SECTION: &'static str = "DriverSelect";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_DRIVERVERSION_SECTION: &'static str = "DriverVer";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_ACTION: &'static str = "Action";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_ALWAYSEXCLUDEFROMSELECT: &'static str = "AlwaysExcludeFromSelect";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_BUFFER_SIZE: &'static str = "BufferSize";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_CATALOGFILE: &'static str = "CatalogFile";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_CHANNEL_ACCESS: &'static str = "Access";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_CHANNEL_ENABLED: &'static str = "Enabled";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_CHANNEL_ISOLATION: &'static str = "Isolation";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_CHANNEL_VALUE: &'static str = "Value";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_CLASS: &'static str = "Class";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_CLASSGUID: &'static str = "ClassGUID";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_CLOCK_TYPE: &'static str = "ClockType";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_CONFIGPRIORITY: &'static str = "ConfigPriority";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_COPYFILESONLY: &'static str = "CopyFilesOnly";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_DATA_ITEM: &'static str = "DataItem";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_DELAYEDAUTOSTART: &'static str = "DelayedAutoStart";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_DEPENDENCIES: &'static str = "Dependencies";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_DESCRIPTION: &'static str = "Description";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_DETECTLIST: &'static str = "DetectList";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_DETPARAMS: &'static str = "Params";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_DISABLE_REALTIME_PERSISTENCE: &'static str = "DisableRealtimePersistence";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_DISPLAYNAME: &'static str = "DisplayName";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_DMA: &'static str = "DMA";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_DMACONFIG: &'static str = "DMAConfig";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_DRIVERSET: &'static str = "DriverSet";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_ENABLED: &'static str = "Enabled";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_ENABLE_FLAGS: &'static str = "EnableFlags";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_ENABLE_LEVEL: &'static str = "EnableLevel";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_ENABLE_PROPERTY: &'static str = "EnableProperty";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_ERRORCONTROL: &'static str = "ErrorControl";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_EXCLUDEFROMSELECT: &'static str = "ExcludeFromSelect";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_EXCLUDERES: &'static str = "ExcludeRes";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_EXTENSIONID: &'static str = "ExtensionId";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_FILE_MAX: &'static str = "FileMax";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_FILE_NAME: &'static str = "FileName";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_FLUSH_TIMER: &'static str = "FlushTimer";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_FROMINET: &'static str = "FromINet";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_HARDWARE_CLASS: &'static str = "Class";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_HARDWARE_CLASSGUID: &'static str = "ClassGUID";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_INTERACTIVEINSTALL: &'static str = "InteractiveInstall";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_IO: &'static str = "IO";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_IOCONFIG: &'static str = "IOConfig";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_IRQ: &'static str = "IRQ";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_IRQCONFIG: &'static str = "IRQConfig";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_LOADORDERGROUP: &'static str = "LoadOrderGroup";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_LOGGING_AUTOBACKUP: &'static str = "LoggingAutoBackup";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_LOGGING_MAXSIZE: &'static str = "LoggingMaxSize";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_LOGGING_RETENTION: &'static str = "LoggingRetention";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_LOG_FILE_MODE: &'static str = "LogFileMode";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_MATCH_ALL_KEYWORD: &'static str = "MatchAllKeyword";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_MATCH_ANY_KEYWORD: &'static str = "MatchAnyKeyword";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_MAXIMUM_BUFFERS: &'static str = "MaximumBuffers";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_MAX_FILE_SIZE: &'static str = "MaxFileSize";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_MEM: &'static str = "Mem";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_MEMCONFIG: &'static str = "MemConfig";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_MEMLARGECONFIG: &'static str = "MemLargeConfig";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_MESSAGE_FILE: &'static str = "MessageFile";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_MFCARDCONFIG: &'static str = "MfCardConfig";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_MINIMUM_BUFFERS: &'static str = "MinimumBuffers";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_NAME: &'static str = "Name";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_NOSETUPINF: &'static str = "NoSetupInf";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_PARAMETER_FILE: &'static str = "ParameterFile";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_PATH: &'static str = "Path";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_PCCARDCONFIG: &'static str = "PcCardConfig";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_PNPLOCKDOWN: &'static str = "PnpLockDown";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_PROVIDER: &'static str = "Provider";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_PROVIDER_NAME: &'static str = "ProviderName";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_REQUESTADDITIONALSOFTWARE: &'static str = "RequestAdditionalSoftware";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_REQUIREDPRIVILEGES: &'static str = "RequiredPrivileges";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_RESOURCE_FILE: &'static str = "ResourceFile";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_SECURITY: &'static str = "Security";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_SERVICEBINARY: &'static str = "ServiceBinary";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_SERVICESIDTYPE: &'static str = "ServiceSidType";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_SERVICETYPE: &'static str = "ServiceType";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_SIGNATURE: &'static str = "Signature";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_SKIPLIST: &'static str = "SkipList";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_START: &'static str = "Start";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_STARTNAME: &'static str = "StartName";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_STARTTYPE: &'static str = "StartType";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_SUB_TYPE: &'static str = "SubType";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_TRIGGER_TYPE: &'static str = "TriggerType";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_PLATFORM_NT: &'static str = "NT";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_PLATFORM_NTALPHA: &'static str = "NTAlpha";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_PLATFORM_NTAMD64: &'static str = "NTAMD64";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_PLATFORM_NTARM: &'static str = "NTARM";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_PLATFORM_NTARM64: &'static str = "NTARM64";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_PLATFORM_NTAXP64: &'static str = "NTAXP64";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_PLATFORM_NTIA64: &'static str = "NTIA64";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_PLATFORM_NTMIPS: &'static str = "NTMIPS";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_PLATFORM_NTPPC: &'static str = "NTPPC";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_PLATFORM_NTX86: &'static str = "NTx86";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_PLATFORM_WIN: &'static str = "Win";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_REBOOT: &'static str = "Reboot";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_RESTART: &'static str = "Restart";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_RISK_BIOSROMRD: &'static str = "RISK_BIOSROMRD";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_RISK_DELICATE: &'static str = "RISK_DELICATE";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_RISK_IORD: &'static str = "RISK_IORD";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_RISK_IOWR: &'static str = "RISK_IOWR";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_RISK_LOW: &'static str = "RISK_LOW";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_RISK_MEMRD: &'static str = "RISK_MEMRD";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_RISK_MEMWR: &'static str = "RISK_MEMWR";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_RISK_NONE: &'static str = "RISK_NONE";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_RISK_QUERYDRV: &'static str = "RISK_QUERYDRV";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_RISK_SWINT: &'static str = "RISK_SWINT";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_RISK_UNRELIABLE: &'static str = "RISK_UNRELIABLE";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_RISK_VERYHIGH: &'static str = "RISK_VERYHIGH";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_RISK_VERYLOW: &'static str = "RISK_VERYLOW";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_AUTOEXECBAT: &'static str = "AutoexecBatDrivers";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_AVOIDCFGSYSDEV: &'static str = "Det.AvoidCfgSysDev";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_AVOIDENVDEV: &'static str = "Det.AvoidEnvDev";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_AVOIDINIDEV: &'static str = "Det.AvoidIniDev";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_BADACPIBIOS: &'static str = "BadACPIBios";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_BADDISKBIOS: &'static str = "BadDiskBios";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_BADDSBIOS: &'static str = "BadDSBios";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_BADPMCALLBIOS: &'static str = "BadProtectedModeCallBios";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_BADPNPBIOS: &'static str = "BadPnpBios";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_BADRMCALLBIOS: &'static str = "BadRealModeCallBios";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_BADROUTINGTABLEBIOS: &'static str = "BadPCIIRQRoutingTableBios";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_CFGSYS: &'static str = "ConfigSysDrivers";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_CLASS_INSTALL: &'static str = "ClassInstall";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_CLASS_INSTALL_32: &'static str = "ClassInstall32";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_DEFAULT_INSTALL: &'static str = "DefaultInstall";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_DEFAULT_UNINSTALL: &'static str = "DefaultUninstall";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_DETCLASSINFO: &'static str = "Det.ClassInfo";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_DETMODULES: &'static str = "Det.Modules";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_DETOPTIONS: &'static str = "Det.Options";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_DEVINFS: &'static str = "Det.DevINFs";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_DISPLAY_CLEANUP: &'static str = "DisplayCleanup";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_EXTENSIONCONTRACTS: &'static str = "ExtensionContracts";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_FORCEHWVERIFY: &'static str = "Det.ForceHWVerify";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_GOODACPIBIOS: &'static str = "GoodACPIBios";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_HPOMNIBOOK: &'static str = "Det.HPOmnibook";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_INTERFACE_INSTALL_32: &'static str = "InterfaceInstall32";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_MACHINEIDBIOS: &'static str = "MachineIDBios";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_MANUALDEV: &'static str = "Det.ManualDev";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_MFG: &'static str = "Manufacturer";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_REGCFGSYSDEV: &'static str = "Det.RegCfgSysDev";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_REGENVDEV: &'static str = "Det.RegEnvDev";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_REGINIDEV: &'static str = "Det.RegIniDev";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_SYSINI: &'static str = "SystemIniDrivers";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_SYSINIDRV: &'static str = "SystemIniDriversLine";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_TARGETCOMPUTERS: &'static str = "TargetComputers";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_VERSION: &'static str = "Version";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_WININIRUN: &'static str = "WinIniRunLine";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SOFTWAREVERSION_SECTION: &'static str = "SoftwareVersion";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_STRKEY_DRVDESC: &'static str = "DriverDesc";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SUBKEY_COINSTALLERS: &'static str = "CoInstallers";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SUBKEY_CTL: &'static str = "CTL";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SUBKEY_DET: &'static str = "Det";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SUBKEY_EVENTS: &'static str = "Events";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SUBKEY_FACTDEF: &'static str = "FactDef";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SUBKEY_FILTERS: &'static str = "Filters";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SUBKEY_HW: &'static str = "Hw";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SUBKEY_INTERFACES: &'static str = "Interfaces";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SUBKEY_LOGCONFIG: &'static str = "LogConfig";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SUBKEY_LOGCONFIGOVERRIDE: &'static str = "LogConfigOverride";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SUBKEY_NORESOURCEDUPS: &'static str = "NoResDup";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SUBKEY_POSSIBLEDUPS: &'static str = "PosDup";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SUBKEY_SERVICES: &'static str = "Services";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SUBKEY_SOFTWARE: &'static str = "Software";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SUBKEY_WMI: &'static str = "WMI";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INF_STYLE_CACHE_DISABLE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INF_STYLE_CACHE_ENABLE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INF_STYLE_CACHE_IGNORE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INSTALLFLAG_BITS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INSTALLFLAG_FORCE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INSTALLFLAG_NONINTERACTIVE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INSTALLFLAG_READONLY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IOA_Local: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IO_ALIAS_10_BIT_DECODE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IO_ALIAS_12_BIT_DECODE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IO_ALIAS_16_BIT_DECODE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IO_ALIAS_POSITIVE_DECODE: u32 = 255u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct IO_DES {
    pub IOD_Count: u32,
    pub IOD_Type: u32,
    pub IOD_Alloc_Base: u64,
    pub IOD_Alloc_End: u64,
    pub IOD_DesFlags: u32,
}
impl ::core::marker::Copy for IO_DES {}
impl ::core::clone::Clone for IO_DES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IO_DES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IO_DES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IO_DES>()) == 0 }
    }
}
impl ::core::cmp::Eq for IO_DES {}
impl ::core::default::Default for IO_DES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct IO_RANGE {
    pub IOR_Align: u64,
    pub IOR_nPorts: u32,
    pub IOR_Min: u64,
    pub IOR_Max: u64,
    pub IOR_RangeFlags: u32,
    pub IOR_Alias: u64,
}
impl ::core::marker::Copy for IO_RANGE {}
impl ::core::clone::Clone for IO_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IO_RANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IO_RANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IO_RANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for IO_RANGE {}
impl ::core::default::Default for IO_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct IO_RESOURCE {
    pub IO_Header: IO_DES,
    pub IO_Data: [IO_RANGE; 1],
}
impl ::core::marker::Copy for IO_RESOURCE {}
impl ::core::clone::Clone for IO_RESOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IO_RESOURCE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IO_RESOURCE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IO_RESOURCE>()) == 0 }
    }
}
impl ::core::cmp::Eq for IO_RESOURCE {}
impl ::core::default::Default for IO_RESOURCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct IRQ_DES_32 {
    pub IRQD_Count: u32,
    pub IRQD_Type: u32,
    pub IRQD_Flags: u32,
    pub IRQD_Alloc_Num: u32,
    pub IRQD_Affinity: u32,
}
impl ::core::marker::Copy for IRQ_DES_32 {}
impl ::core::clone::Clone for IRQ_DES_32 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IRQ_DES_32 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IRQ_DES_32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IRQ_DES_32>()) == 0 }
    }
}
impl ::core::cmp::Eq for IRQ_DES_32 {}
impl ::core::default::Default for IRQ_DES_32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct IRQ_DES_64 {
    pub IRQD_Count: u32,
    pub IRQD_Type: u32,
    pub IRQD_Flags: u32,
    pub IRQD_Alloc_Num: u32,
    pub IRQD_Affinity: u64,
}
impl ::core::marker::Copy for IRQ_DES_64 {}
impl ::core::clone::Clone for IRQ_DES_64 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IRQ_DES_64 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IRQ_DES_64 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IRQ_DES_64>()) == 0 }
    }
}
impl ::core::cmp::Eq for IRQ_DES_64 {}
impl ::core::default::Default for IRQ_DES_64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct IRQ_RANGE {
    pub IRQR_Min: u32,
    pub IRQR_Max: u32,
    pub IRQR_Flags: u32,
}
impl ::core::marker::Copy for IRQ_RANGE {}
impl ::core::clone::Clone for IRQ_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IRQ_RANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IRQ_RANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IRQ_RANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for IRQ_RANGE {}
impl ::core::default::Default for IRQ_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct IRQ_RESOURCE_32 {
    pub IRQ_Header: IRQ_DES_32,
    pub IRQ_Data: [IRQ_RANGE; 1],
}
impl ::core::marker::Copy for IRQ_RESOURCE_32 {}
impl ::core::clone::Clone for IRQ_RESOURCE_32 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IRQ_RESOURCE_32 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IRQ_RESOURCE_32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IRQ_RESOURCE_32>()) == 0 }
    }
}
impl ::core::cmp::Eq for IRQ_RESOURCE_32 {}
impl ::core::default::Default for IRQ_RESOURCE_32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct IRQ_RESOURCE_64 {
    pub IRQ_Header: IRQ_DES_64,
    pub IRQ_Data: [IRQ_RANGE; 1],
}
impl ::core::marker::Copy for IRQ_RESOURCE_64 {}
impl ::core::clone::Clone for IRQ_RESOURCE_64 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IRQ_RESOURCE_64 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IRQ_RESOURCE_64 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IRQ_RESOURCE_64>()) == 0 }
    }
}
impl ::core::cmp::Eq for IRQ_RESOURCE_64 {}
impl ::core::default::Default for IRQ_RESOURCE_64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InstallHinfSectionA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(window: Param0, modulehandle: Param1, commandline: Param2, showcommand: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InstallHinfSectionA(window: super::super::Foundation::HWND, modulehandle: super::super::Foundation::HINSTANCE, commandline: ::windows::core::PCSTR, showcommand: i32);
        }
        InstallHinfSectionA(window.into_param().abi(), modulehandle.into_param().abi(), commandline.into_param().abi(), ::core::mem::transmute(showcommand))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InstallHinfSectionW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(window: Param0, modulehandle: Param1, commandline: Param2, showcommand: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InstallHinfSectionW(window: super::super::Foundation::HWND, modulehandle: super::super::Foundation::HINSTANCE, commandline: ::windows::core::PCWSTR, showcommand: i32);
        }
        InstallHinfSectionW(window.into_param().abi(), modulehandle.into_param().abi(), commandline.into_param().abi(), ::core::mem::transmute(showcommand))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LCPRI_BOOTCONFIG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LCPRI_DESIRED: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LCPRI_DISABLED: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LCPRI_FORCECONFIG: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LCPRI_HARDRECONFIG: u32 = 49152u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LCPRI_HARDWIRED: u32 = 57344u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LCPRI_IMPOSSIBLE: u32 = 61440u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LCPRI_LASTBESTCONFIG: u32 = 16383u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LCPRI_LASTSOFTCONFIG: u32 = 32767u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LCPRI_NORMAL: u32 = 12288u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LCPRI_POWEROFF: u32 = 40960u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LCPRI_REBOOT: u32 = 36864u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LCPRI_RESTART: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LCPRI_SUBOPTIMAL: u32 = 20480u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LINE_LEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LOG_CONF_BITS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LogSevError: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LogSevFatalError: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LogSevInformation: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LogSevMaximum: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LogSevWarning: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_CLASS_NAME_LEN: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_CONFIG_VALUE: u32 = 9999u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_DEVICE_ID_LEN: u32 = 200u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_DEVNODE_ID_LEN: u32 = 200u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_DMA_CHANNELS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_GUID_STRING_LEN: u32 = 39u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_IDD_DYNAWIZ_RESOURCE_ID: u32 = 11000u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_INFSTR_STRKEY_LEN: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_INF_FLAG: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_INF_SECTION_NAME_LENGTH: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_INF_STRING_LENGTH: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_INSTALLWIZARD_DYNAPAGES: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_INSTANCE_VALUE: u32 = 9999u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_INSTRUCTION_LEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_IO_PORTS: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_IRQS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_KEY_LEN: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_LABEL_LEN: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_LCPRI: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_MEM_REGISTERS: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_PRIORITYSTR_LEN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_PROFILE_LEN: u32 = 80u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_SERVICE_NAME_LEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_SUBTITLE_LEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_TITLE_LEN: u32 = 60u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct MEM_DES {
    pub MD_Count: u32,
    pub MD_Type: u32,
    pub MD_Alloc_Base: u64,
    pub MD_Alloc_End: u64,
    pub MD_Flags: u32,
    pub MD_Reserved: u32,
}
impl ::core::marker::Copy for MEM_DES {}
impl ::core::clone::Clone for MEM_DES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MEM_DES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MEM_DES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MEM_DES>()) == 0 }
    }
}
impl ::core::cmp::Eq for MEM_DES {}
impl ::core::default::Default for MEM_DES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct MEM_RANGE {
    pub MR_Align: u64,
    pub MR_nBytes: u32,
    pub MR_Min: u64,
    pub MR_Max: u64,
    pub MR_Flags: u32,
    pub MR_Reserved: u32,
}
impl ::core::marker::Copy for MEM_RANGE {}
impl ::core::clone::Clone for MEM_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MEM_RANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MEM_RANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MEM_RANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for MEM_RANGE {}
impl ::core::default::Default for MEM_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct MEM_RESOURCE {
    pub MEM_Header: MEM_DES,
    pub MEM_Data: [MEM_RANGE; 1],
}
impl ::core::marker::Copy for MEM_RESOURCE {}
impl ::core::clone::Clone for MEM_RESOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MEM_RESOURCE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MEM_RESOURCE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MEM_RESOURCE>()) == 0 }
    }
}
impl ::core::cmp::Eq for MEM_RESOURCE {}
impl ::core::default::Default for MEM_RESOURCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct MFCARD_DES {
    pub PMF_Count: u32,
    pub PMF_Type: u32,
    pub PMF_Flags: u32,
    pub PMF_ConfigOptions: u8,
    pub PMF_IoResourceIndex: u8,
    pub PMF_Reserved: [u8; 2],
    pub PMF_ConfigRegisterBase: u32,
}
impl ::core::marker::Copy for MFCARD_DES {}
impl ::core::clone::Clone for MFCARD_DES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MFCARD_DES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MFCARD_DES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MFCARD_DES>()) == 0 }
    }
}
impl ::core::cmp::Eq for MFCARD_DES {}
impl ::core::default::Default for MFCARD_DES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct MFCARD_RESOURCE {
    pub MfCard_Header: MFCARD_DES,
}
impl ::core::marker::Copy for MFCARD_RESOURCE {}
impl ::core::clone::Clone for MFCARD_RESOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MFCARD_RESOURCE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MFCARD_RESOURCE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MFCARD_RESOURCE>()) == 0 }
    }
}
impl ::core::cmp::Eq for MFCARD_RESOURCE {}
impl ::core::default::Default for MFCARD_RESOURCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MIN_IDD_DYNAWIZ_RESOURCE_ID: u32 = 10000u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct Mem_Large_Des_s {
    pub MLD_Count: u32,
    pub MLD_Type: u32,
    pub MLD_Alloc_Base: u64,
    pub MLD_Alloc_End: u64,
    pub MLD_Flags: u32,
    pub MLD_Reserved: u32,
}
impl ::core::marker::Copy for Mem_Large_Des_s {}
impl ::core::clone::Clone for Mem_Large_Des_s {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for Mem_Large_Des_s {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for Mem_Large_Des_s {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<Mem_Large_Des_s>()) == 0 }
    }
}
impl ::core::cmp::Eq for Mem_Large_Des_s {}
impl ::core::default::Default for Mem_Large_Des_s {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct Mem_Large_Range_s {
    pub MLR_Align: u64,
    pub MLR_nBytes: u64,
    pub MLR_Min: u64,
    pub MLR_Max: u64,
    pub MLR_Flags: u32,
    pub MLR_Reserved: u32,
}
impl ::core::marker::Copy for Mem_Large_Range_s {}
impl ::core::clone::Clone for Mem_Large_Range_s {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for Mem_Large_Range_s {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for Mem_Large_Range_s {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<Mem_Large_Range_s>()) == 0 }
    }
}
impl ::core::cmp::Eq for Mem_Large_Range_s {}
impl ::core::default::Default for Mem_Large_Range_s {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct Mem_Large_Resource_s {
    pub MEM_LARGE_Header: Mem_Large_Des_s,
    pub MEM_LARGE_Data: [Mem_Large_Range_s; 1],
}
impl ::core::marker::Copy for Mem_Large_Resource_s {}
impl ::core::clone::Clone for Mem_Large_Resource_s {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for Mem_Large_Resource_s {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for Mem_Large_Resource_s {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<Mem_Large_Resource_s>()) == 0 }
    }
}
impl ::core::cmp::Eq for Mem_Large_Resource_s {}
impl ::core::default::Default for Mem_Large_Resource_s {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NDW_INSTALLFLAG_CI_PICKED_OEM: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NDW_INSTALLFLAG_DIDFACTDEFS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NDW_INSTALLFLAG_EXPRESSINTRO: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NDW_INSTALLFLAG_HARDWAREALLREADYIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NDW_INSTALLFLAG_INSTALLSPECIFIC: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NDW_INSTALLFLAG_KNOWNCLASS: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NDW_INSTALLFLAG_NEEDREBOOT: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NDW_INSTALLFLAG_NEEDRESTART: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NDW_INSTALLFLAG_NEEDSHUTDOWN: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NDW_INSTALLFLAG_NODETECTEDDEVS: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NDW_INSTALLFLAG_PCMCIADEVICE: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NDW_INSTALLFLAG_PCMCIAMODE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NDW_INSTALLFLAG_SKIPCLASSLIST: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NDW_INSTALLFLAG_SKIPISDEVINSTALLED: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NDW_INSTALLFLAG_USERCANCEL: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NUM_CM_PROB: u32 = 58u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NUM_CM_PROB_V1: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NUM_CM_PROB_V2: u32 = 50u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NUM_CM_PROB_V3: u32 = 51u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NUM_CM_PROB_V4: u32 = 52u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NUM_CM_PROB_V5: u32 = 53u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NUM_CM_PROB_V6: u32 = 54u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NUM_CM_PROB_V7: u32 = 55u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NUM_CM_PROB_V8: u32 = 57u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NUM_CM_PROB_V9: u32 = 58u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NUM_LOG_CONF: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct OEM_SOURCE_MEDIA_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPOST_NONE: OEM_SOURCE_MEDIA_TYPE = OEM_SOURCE_MEDIA_TYPE(0u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPOST_PATH: OEM_SOURCE_MEDIA_TYPE = OEM_SOURCE_MEDIA_TYPE(1u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPOST_URL: OEM_SOURCE_MEDIA_TYPE = OEM_SOURCE_MEDIA_TYPE(2u32);
impl ::core::marker::Copy for OEM_SOURCE_MEDIA_TYPE {}
impl ::core::clone::Clone for OEM_SOURCE_MEDIA_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OEM_SOURCE_MEDIA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for OEM_SOURCE_MEDIA_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for OEM_SOURCE_MEDIA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OEM_SOURCE_MEDIA_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const OVERRIDE_LOG_CONF: u32 = 5u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct PCCARD_DES {
    pub PCD_Count: u32,
    pub PCD_Type: u32,
    pub PCD_Flags: u32,
    pub PCD_ConfigIndex: u8,
    pub PCD_Reserved: [u8; 3],
    pub PCD_MemoryCardBase1: u32,
    pub PCD_MemoryCardBase2: u32,
    pub PCD_MemoryCardBase: [u32; 2],
    pub PCD_MemoryFlags: [u16; 2],
    pub PCD_IoFlags: [u8; 2],
}
impl ::core::marker::Copy for PCCARD_DES {}
impl ::core::clone::Clone for PCCARD_DES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PCCARD_DES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PCCARD_DES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PCCARD_DES>()) == 0 }
    }
}
impl ::core::cmp::Eq for PCCARD_DES {}
impl ::core::default::Default for PCCARD_DES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct PCCARD_RESOURCE {
    pub PcCard_Header: PCCARD_DES,
}
impl ::core::marker::Copy for PCCARD_RESOURCE {}
impl ::core::clone::Clone for PCCARD_RESOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PCCARD_RESOURCE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PCCARD_RESOURCE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PCCARD_RESOURCE>()) == 0 }
    }
}
impl ::core::cmp::Eq for PCCARD_RESOURCE {}
impl ::core::default::Default for PCCARD_RESOURCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PCD_MAX_IO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PCD_MAX_MEMORY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub type PCM_NOTIFY_CALLBACK = ::core::option::Option<unsafe extern "system" fn(hnotify: HCMNOTIFICATION, context: *const ::core::ffi::c_void, action: CM_NOTIFY_ACTION, eventdata: *const CM_NOTIFY_EVENT_DATA, eventdatasize: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDETECT_PROGRESS_NOTIFY = ::core::option::Option<unsafe extern "system" fn(progressnotifyparam: *const ::core::ffi::c_void, detectcomplete: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PNP_VETO_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PNP_VetoTypeUnknown: PNP_VETO_TYPE = PNP_VETO_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PNP_VetoLegacyDevice: PNP_VETO_TYPE = PNP_VETO_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PNP_VetoPendingClose: PNP_VETO_TYPE = PNP_VETO_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PNP_VetoWindowsApp: PNP_VETO_TYPE = PNP_VETO_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PNP_VetoWindowsService: PNP_VETO_TYPE = PNP_VETO_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PNP_VetoOutstandingOpen: PNP_VETO_TYPE = PNP_VETO_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PNP_VetoDevice: PNP_VETO_TYPE = PNP_VETO_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PNP_VetoDriver: PNP_VETO_TYPE = PNP_VETO_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PNP_VetoIllegalDeviceRequest: PNP_VETO_TYPE = PNP_VETO_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PNP_VetoInsufficientPower: PNP_VETO_TYPE = PNP_VETO_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PNP_VetoNonDisableable: PNP_VETO_TYPE = PNP_VETO_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PNP_VetoLegacyDriver: PNP_VETO_TYPE = PNP_VETO_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PNP_VetoInsufficientRights: PNP_VETO_TYPE = PNP_VETO_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PNP_VetoAlreadyRemoved: PNP_VETO_TYPE = PNP_VETO_TYPE(13i32);
impl ::core::marker::Copy for PNP_VETO_TYPE {}
impl ::core::clone::Clone for PNP_VETO_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PNP_VETO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PNP_VETO_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PNP_VETO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PNP_VETO_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PRIORITY_BIT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PRIORITY_EQUAL_FIRST: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PRIORITY_EQUAL_LAST: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub type PSP_DETSIG_CMPPROC = ::core::option::Option<unsafe extern "system" fn(deviceinfoset: *const ::core::ffi::c_void, newdevicedata: *const SP_DEVINFO_DATA, existingdevicedata: *const SP_DEVINFO_DATA, comparecontext: *const ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub type PSP_FILE_CALLBACK_A = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, notification: u32, param1: usize, param2: usize) -> u32>;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub type PSP_FILE_CALLBACK_W = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, notification: u32, param1: usize, param2: usize) -> u32>;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ROLLBACK_BITS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ROLLBACK_FLAG_NO_UI: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const RegDisposition_Bits: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const RegDisposition_OpenAlways: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const RegDisposition_OpenExisting: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ResType_All: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ResType_BusNumber: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ResType_ClassSpecific: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ResType_Connection: u32 = 32772u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ResType_DMA: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ResType_DevicePrivate: u32 = 32769u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ResType_DoNotUse: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ResType_IO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ResType_IRQ: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ResType_Ignored_Bit: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ResType_MAX: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ResType_Mem: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ResType_MemLarge: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ResType_MfCardConfig: u32 = 32771u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ResType_None: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ResType_PcCardConfig: u32 = 32770u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ResType_Reserved: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SCWMI_CLOBBER_SECURITY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SETDIRID_NOT_FULL_PATH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SETUP_DI_BUILD_DRIVER_DRIVER_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDIT_CLASSDRIVER: SETUP_DI_BUILD_DRIVER_DRIVER_TYPE = SETUP_DI_BUILD_DRIVER_DRIVER_TYPE(1u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDIT_COMPATDRIVER: SETUP_DI_BUILD_DRIVER_DRIVER_TYPE = SETUP_DI_BUILD_DRIVER_DRIVER_TYPE(2u32);
impl ::core::marker::Copy for SETUP_DI_BUILD_DRIVER_DRIVER_TYPE {}
impl ::core::clone::Clone for SETUP_DI_BUILD_DRIVER_DRIVER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SETUP_DI_BUILD_DRIVER_DRIVER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SETUP_DI_BUILD_DRIVER_DRIVER_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SETUP_DI_BUILD_DRIVER_DRIVER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SETUP_DI_BUILD_DRIVER_DRIVER_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SETUP_FILE_OPERATION(pub u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FILEOP_DELETE: SETUP_FILE_OPERATION = SETUP_FILE_OPERATION(2u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FILEOP_COPY: SETUP_FILE_OPERATION = SETUP_FILE_OPERATION(0u32);
impl ::core::marker::Copy for SETUP_FILE_OPERATION {}
impl ::core::clone::Clone for SETUP_FILE_OPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SETUP_FILE_OPERATION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SETUP_FILE_OPERATION {
    type Abi = Self;
}
impl ::core::fmt::Debug for SETUP_FILE_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SETUP_FILE_OPERATION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SIGNERSCORE_AUTHENTICODE: u32 = 251658240u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SIGNERSCORE_INBOX: u32 = 218103811u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SIGNERSCORE_LOGO_PREMIUM: u32 = 218103809u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SIGNERSCORE_LOGO_STANDARD: u32 = 218103810u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SIGNERSCORE_MASK: u32 = 4278190080u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SIGNERSCORE_SIGNED_MASK: u32 = 4026531840u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SIGNERSCORE_UNCLASSIFIED: u32 = 218103812u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SIGNERSCORE_UNKNOWN: u32 = 4278190080u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SIGNERSCORE_UNSIGNED: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SIGNERSCORE_W9X_SUSPECT: u32 = 3221225472u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SIGNERSCORE_WHQL: u32 = 218103813u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SOURCE_MEDIA_A {
    pub Reserved: ::windows::core::PCSTR,
    pub Tagfile: ::windows::core::PCSTR,
    pub Description: ::windows::core::PCSTR,
    pub SourcePath: ::windows::core::PCSTR,
    pub SourceFile: ::windows::core::PCSTR,
    pub Flags: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SOURCE_MEDIA_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SOURCE_MEDIA_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for SOURCE_MEDIA_A {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SOURCE_MEDIA_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SOURCE_MEDIA_A>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SOURCE_MEDIA_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SOURCE_MEDIA_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SOURCE_MEDIA_A {
    pub Reserved: ::windows::core::PCSTR,
    pub Tagfile: ::windows::core::PCSTR,
    pub Description: ::windows::core::PCSTR,
    pub SourcePath: ::windows::core::PCSTR,
    pub SourceFile: ::windows::core::PCSTR,
    pub Flags: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SOURCE_MEDIA_A {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SOURCE_MEDIA_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for SOURCE_MEDIA_A {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for SOURCE_MEDIA_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SOURCE_MEDIA_A>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for SOURCE_MEDIA_A {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for SOURCE_MEDIA_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SOURCE_MEDIA_W {
    pub Reserved: ::windows::core::PCWSTR,
    pub Tagfile: ::windows::core::PCWSTR,
    pub Description: ::windows::core::PCWSTR,
    pub SourcePath: ::windows::core::PCWSTR,
    pub SourceFile: ::windows::core::PCWSTR,
    pub Flags: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SOURCE_MEDIA_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SOURCE_MEDIA_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for SOURCE_MEDIA_W {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SOURCE_MEDIA_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SOURCE_MEDIA_W>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SOURCE_MEDIA_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SOURCE_MEDIA_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SOURCE_MEDIA_W {
    pub Reserved: ::windows::core::PCWSTR,
    pub Tagfile: ::windows::core::PCWSTR,
    pub Description: ::windows::core::PCWSTR,
    pub SourcePath: ::windows::core::PCWSTR,
    pub SourceFile: ::windows::core::PCWSTR,
    pub Flags: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SOURCE_MEDIA_W {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SOURCE_MEDIA_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for SOURCE_MEDIA_W {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for SOURCE_MEDIA_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SOURCE_MEDIA_W>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for SOURCE_MEDIA_W {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for SOURCE_MEDIA_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPCRP_CHARACTERISTICS: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPCRP_DEVTYPE: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPCRP_EXCLUSIVE: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPCRP_LOWERFILTERS: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPCRP_MAXIMUM_PROPERTY: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPCRP_SECURITY: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPCRP_SECURITY_SDS: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPCRP_UPPERFILTERS: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDIT_NODRIVER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_ADDRESS: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_BASE_CONTAINERID: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_BUSNUMBER: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_BUSTYPEGUID: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_CAPABILITIES: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_CHARACTERISTICS: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_CLASS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_CLASSGUID: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_COMPATIBLEIDS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_CONFIGFLAGS: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_DEVICEDESC: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_DEVICE_POWER_DATA: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_DEVTYPE: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_DRIVER: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_ENUMERATOR_NAME: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_EXCLUSIVE: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_FRIENDLYNAME: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_HARDWAREID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_INSTALL_STATE: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_LEGACYBUSTYPE: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_LOCATION_INFORMATION: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_LOCATION_PATHS: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_LOWERFILTERS: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_MAXIMUM_PROPERTY: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_MFG: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_PHYSICAL_DEVICE_OBJECT_NAME: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_REMOVAL_POLICY: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_REMOVAL_POLICY_HW_DEFAULT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_REMOVAL_POLICY_OVERRIDE: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_SECURITY: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_SECURITY_SDS: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_SERVICE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_UI_NUMBER: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_UI_NUMBER_DESC_FORMAT: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_UNUSED0: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_UNUSED1: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_UNUSED2: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_UPPERFILTERS: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDSL_DISALLOW_NEGATIVE_ADJUST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDSL_IGNORE_DISK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILELOG_FORCENEW: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILELOG_OEMFILE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILELOG_QUERYONLY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILELOG_SYSTEMLOG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_BACKUPERROR: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_CABINETINFO: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_COPYERROR: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_DELETEERROR: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_ENDBACKUP: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_ENDCOPY: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_ENDDELETE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_ENDQUEUE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_ENDREGISTRATION: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_ENDRENAME: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_ENDSUBQUEUE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_FILEEXTRACTED: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_FILEINCABINET: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_FILEOPDELAYED: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_LANGMISMATCH: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_NEEDMEDIA: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_NEEDNEWCABINET: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_QUEUESCAN: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_QUEUESCAN_EX: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_QUEUESCAN_SIGNERINFO: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_RENAMEERROR: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_STARTBACKUP: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_STARTCOPY: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_STARTDELETE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_STARTQUEUE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_STARTREGISTRATION: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_STARTRENAME: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_STARTSUBQUEUE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_TARGETEXISTS: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_TARGETNEWER: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILEQ_FILE_IN_USE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILEQ_REBOOT_IN_PROGRESS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILEQ_REBOOT_RECOMMENDED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPID_ACTIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPID_DEFAULT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPID_REMOVED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINST_ALL: u32 = 2047u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINST_BITREG: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINST_COPYINF: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINST_DEVICEINSTALL: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINST_FILES: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINST_INI2REG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINST_INIFILES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINST_LOGCONFIG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINST_LOGCONFIGS_ARE_OVERRIDES: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINST_LOGCONFIG_IS_FORCED: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINST_PROFILEITEMS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINST_PROPERTIES: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINST_REGISTERCALLBACKAWARE: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINST_REGISTRY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINST_REGSVR: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINST_SINGLESECTION: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINST_UNREGSVR: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINT_ACTIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINT_DEFAULT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINT_REMOVED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPOST_MAX: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPPSR_ENUM_ADV_DEVICE_PROPERTIES: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPPSR_ENUM_BASIC_DEVICE_PROPERTIES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPPSR_SELECT_DEVICE_RESOURCES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPQ_DELAYED_COPY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPQ_FLAG_ABORT_IF_UNSIGNED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPQ_FLAG_BACKUP_AWARE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPQ_FLAG_DO_SHUFFLEMOVE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPQ_FLAG_FILES_MODIFIED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPQ_FLAG_VALID: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPQ_SCAN_ACTIVATE_DRP: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPQ_SCAN_FILE_COMPARISON: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPQ_SCAN_FILE_PRESENCE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPQ_SCAN_FILE_PRESENCE_WITHOUT_SOURCE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPQ_SCAN_FILE_VALIDITY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPQ_SCAN_INFORM_USER: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPQ_SCAN_PRUNE_COPY_QUEUE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPQ_SCAN_PRUNE_DELREN: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPQ_SCAN_USE_CALLBACK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPQ_SCAN_USE_CALLBACKEX: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPQ_SCAN_USE_CALLBACK_SIGNERINFO: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPRDI_FIND_DUPS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPREG_DLLINSTALL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPREG_GETPROCADDR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPREG_LOADLIBRARY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPREG_REGSVR: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPREG_SUCCESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPREG_TIMEOUT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPREG_UNKNOWN: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPSVCINST_ASSOCSERVICE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPSVCINST_CLOBBER_SECURITY: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPSVCINST_DELETEEVENTLOGENTRY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPSVCINST_NOCLOBBER_DELAYEDAUTOSTART: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPSVCINST_NOCLOBBER_DEPENDENCIES: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPSVCINST_NOCLOBBER_DESCRIPTION: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPSVCINST_NOCLOBBER_DISPLAYNAME: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPSVCINST_NOCLOBBER_ERRORCONTROL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPSVCINST_NOCLOBBER_LOADORDERGROUP: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPSVCINST_NOCLOBBER_REQUIREDPRIVILEGES: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPSVCINST_NOCLOBBER_SERVICESIDTYPE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPSVCINST_NOCLOBBER_STARTTYPE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPSVCINST_NOCLOBBER_TRIGGERS: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPSVCINST_STARTSERVICE: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPSVCINST_STOPSERVICE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPSVCINST_TAGTOFRONT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPSVCINST_UNIQUE_NAME: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPWPT_SELECTDEVICE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPWP_USE_DEVINFO_DATA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_ALTPLATFORM_FLAGS_SUITE_MASK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_ALTPLATFORM_FLAGS_VERSION_RANGE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub struct SP_ALTPLATFORM_INFO_V1 {
    pub cbSize: u32,
    pub Platform: super::super::System::Diagnostics::Debug::VER_PLATFORM,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ProcessorArchitecture: u16,
    pub Reserved: u16,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::marker::Copy for SP_ALTPLATFORM_INFO_V1 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::clone::Clone for SP_ALTPLATFORM_INFO_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
unsafe impl ::windows::core::Abi for SP_ALTPLATFORM_INFO_V1 {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::cmp::PartialEq for SP_ALTPLATFORM_INFO_V1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_ALTPLATFORM_INFO_V1>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::cmp::Eq for SP_ALTPLATFORM_INFO_V1 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::default::Default for SP_ALTPLATFORM_INFO_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub struct SP_ALTPLATFORM_INFO_V1 {
    pub cbSize: u32,
    pub Platform: super::super::System::Diagnostics::Debug::VER_PLATFORM,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ProcessorArchitecture: u16,
    pub Reserved: u16,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::marker::Copy for SP_ALTPLATFORM_INFO_V1 {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::clone::Clone for SP_ALTPLATFORM_INFO_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
unsafe impl ::windows::core::Abi for SP_ALTPLATFORM_INFO_V1 {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::cmp::PartialEq for SP_ALTPLATFORM_INFO_V1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_ALTPLATFORM_INFO_V1>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::cmp::Eq for SP_ALTPLATFORM_INFO_V1 {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::default::Default for SP_ALTPLATFORM_INFO_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub struct SP_ALTPLATFORM_INFO_V2 {
    pub cbSize: u32,
    pub Platform: super::super::System::Diagnostics::Debug::VER_PLATFORM,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ProcessorArchitecture: u16,
    pub Anonymous: SP_ALTPLATFORM_INFO_V2_0,
    pub FirstValidatedMajorVersion: u32,
    pub FirstValidatedMinorVersion: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::marker::Copy for SP_ALTPLATFORM_INFO_V2 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::clone::Clone for SP_ALTPLATFORM_INFO_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
unsafe impl ::windows::core::Abi for SP_ALTPLATFORM_INFO_V2 {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::cmp::PartialEq for SP_ALTPLATFORM_INFO_V2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_ALTPLATFORM_INFO_V2>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::cmp::Eq for SP_ALTPLATFORM_INFO_V2 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::default::Default for SP_ALTPLATFORM_INFO_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub union SP_ALTPLATFORM_INFO_V2_0 {
    pub Reserved: u16,
    pub Flags: u16,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::marker::Copy for SP_ALTPLATFORM_INFO_V2_0 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::clone::Clone for SP_ALTPLATFORM_INFO_V2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
unsafe impl ::windows::core::Abi for SP_ALTPLATFORM_INFO_V2_0 {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::cmp::PartialEq for SP_ALTPLATFORM_INFO_V2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_ALTPLATFORM_INFO_V2_0>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::cmp::Eq for SP_ALTPLATFORM_INFO_V2_0 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::default::Default for SP_ALTPLATFORM_INFO_V2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub struct SP_ALTPLATFORM_INFO_V2 {
    pub cbSize: u32,
    pub Platform: super::super::System::Diagnostics::Debug::VER_PLATFORM,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ProcessorArchitecture: u16,
    pub Anonymous: SP_ALTPLATFORM_INFO_V2_0,
    pub FirstValidatedMajorVersion: u32,
    pub FirstValidatedMinorVersion: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::marker::Copy for SP_ALTPLATFORM_INFO_V2 {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::clone::Clone for SP_ALTPLATFORM_INFO_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
unsafe impl ::windows::core::Abi for SP_ALTPLATFORM_INFO_V2 {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::cmp::PartialEq for SP_ALTPLATFORM_INFO_V2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_ALTPLATFORM_INFO_V2>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::cmp::Eq for SP_ALTPLATFORM_INFO_V2 {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::default::Default for SP_ALTPLATFORM_INFO_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub union SP_ALTPLATFORM_INFO_V2_0 {
    pub Reserved: u16,
    pub Flags: u16,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::marker::Copy for SP_ALTPLATFORM_INFO_V2_0 {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::clone::Clone for SP_ALTPLATFORM_INFO_V2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
unsafe impl ::windows::core::Abi for SP_ALTPLATFORM_INFO_V2_0 {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::cmp::PartialEq for SP_ALTPLATFORM_INFO_V2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_ALTPLATFORM_INFO_V2_0>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::cmp::Eq for SP_ALTPLATFORM_INFO_V2_0 {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::default::Default for SP_ALTPLATFORM_INFO_V2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_ALTPLATFORM_INFO_V3 {
    pub cbSize: u32,
    pub Platform: u32,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ProcessorArchitecture: u16,
    pub Anonymous: SP_ALTPLATFORM_INFO_V3_0,
    pub FirstValidatedMajorVersion: u32,
    pub FirstValidatedMinorVersion: u32,
    pub ProductType: u8,
    pub SuiteMask: u16,
    pub BuildNumber: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_ALTPLATFORM_INFO_V3 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_ALTPLATFORM_INFO_V3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for SP_ALTPLATFORM_INFO_V3 {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SP_ALTPLATFORM_INFO_V3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_ALTPLATFORM_INFO_V3>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SP_ALTPLATFORM_INFO_V3 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SP_ALTPLATFORM_INFO_V3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub union SP_ALTPLATFORM_INFO_V3_0 {
    pub Reserved: u16,
    pub Flags: u16,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_ALTPLATFORM_INFO_V3_0 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_ALTPLATFORM_INFO_V3_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for SP_ALTPLATFORM_INFO_V3_0 {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SP_ALTPLATFORM_INFO_V3_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_ALTPLATFORM_INFO_V3_0>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SP_ALTPLATFORM_INFO_V3_0 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SP_ALTPLATFORM_INFO_V3_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_ALTPLATFORM_INFO_V3 {
    pub cbSize: u32,
    pub Platform: u32,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ProcessorArchitecture: u16,
    pub Anonymous: SP_ALTPLATFORM_INFO_V3_0,
    pub FirstValidatedMajorVersion: u32,
    pub FirstValidatedMinorVersion: u32,
    pub ProductType: u8,
    pub SuiteMask: u16,
    pub BuildNumber: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_ALTPLATFORM_INFO_V3 {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_ALTPLATFORM_INFO_V3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for SP_ALTPLATFORM_INFO_V3 {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for SP_ALTPLATFORM_INFO_V3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_ALTPLATFORM_INFO_V3>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for SP_ALTPLATFORM_INFO_V3 {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for SP_ALTPLATFORM_INFO_V3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub union SP_ALTPLATFORM_INFO_V3_0 {
    pub Reserved: u16,
    pub Flags: u16,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_ALTPLATFORM_INFO_V3_0 {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_ALTPLATFORM_INFO_V3_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for SP_ALTPLATFORM_INFO_V3_0 {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for SP_ALTPLATFORM_INFO_V3_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_ALTPLATFORM_INFO_V3_0>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for SP_ALTPLATFORM_INFO_V3_0 {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for SP_ALTPLATFORM_INFO_V3_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_BACKUP_BACKUPPASS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_BACKUP_BOOTFILE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_BACKUP_DEMANDPASS: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_BACKUP_QUEUE_PARAMS_V1_A {
    pub cbSize: u32,
    pub FullInfPath: [super::super::Foundation::CHAR; 260],
    pub FilenameOffset: i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_BACKUP_QUEUE_PARAMS_V1_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_BACKUP_QUEUE_PARAMS_V1_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SP_BACKUP_QUEUE_PARAMS_V1_A {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SP_BACKUP_QUEUE_PARAMS_V1_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_BACKUP_QUEUE_PARAMS_V1_A>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SP_BACKUP_QUEUE_PARAMS_V1_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SP_BACKUP_QUEUE_PARAMS_V1_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_BACKUP_QUEUE_PARAMS_V1_A {
    pub cbSize: u32,
    pub FullInfPath: [super::super::Foundation::CHAR; 260],
    pub FilenameOffset: i32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_BACKUP_QUEUE_PARAMS_V1_A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_BACKUP_QUEUE_PARAMS_V1_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SP_BACKUP_QUEUE_PARAMS_V1_A {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SP_BACKUP_QUEUE_PARAMS_V1_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_BACKUP_QUEUE_PARAMS_V1_A>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SP_BACKUP_QUEUE_PARAMS_V1_A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SP_BACKUP_QUEUE_PARAMS_V1_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_BACKUP_QUEUE_PARAMS_V1_W {
    pub cbSize: u32,
    pub FullInfPath: [u16; 260],
    pub FilenameOffset: i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_BACKUP_QUEUE_PARAMS_V1_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_BACKUP_QUEUE_PARAMS_V1_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for SP_BACKUP_QUEUE_PARAMS_V1_W {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SP_BACKUP_QUEUE_PARAMS_V1_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_BACKUP_QUEUE_PARAMS_V1_W>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SP_BACKUP_QUEUE_PARAMS_V1_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SP_BACKUP_QUEUE_PARAMS_V1_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_BACKUP_QUEUE_PARAMS_V1_W {
    pub cbSize: u32,
    pub FullInfPath: [u16; 260],
    pub FilenameOffset: i32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_BACKUP_QUEUE_PARAMS_V1_W {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_BACKUP_QUEUE_PARAMS_V1_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for SP_BACKUP_QUEUE_PARAMS_V1_W {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for SP_BACKUP_QUEUE_PARAMS_V1_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_BACKUP_QUEUE_PARAMS_V1_W>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for SP_BACKUP_QUEUE_PARAMS_V1_W {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for SP_BACKUP_QUEUE_PARAMS_V1_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_BACKUP_QUEUE_PARAMS_V2_A {
    pub cbSize: u32,
    pub FullInfPath: [super::super::Foundation::CHAR; 260],
    pub FilenameOffset: i32,
    pub ReinstallInstance: [super::super::Foundation::CHAR; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_BACKUP_QUEUE_PARAMS_V2_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_BACKUP_QUEUE_PARAMS_V2_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SP_BACKUP_QUEUE_PARAMS_V2_A {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SP_BACKUP_QUEUE_PARAMS_V2_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_BACKUP_QUEUE_PARAMS_V2_A>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SP_BACKUP_QUEUE_PARAMS_V2_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SP_BACKUP_QUEUE_PARAMS_V2_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_BACKUP_QUEUE_PARAMS_V2_A {
    pub cbSize: u32,
    pub FullInfPath: [super::super::Foundation::CHAR; 260],
    pub FilenameOffset: i32,
    pub ReinstallInstance: [super::super::Foundation::CHAR; 260],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_BACKUP_QUEUE_PARAMS_V2_A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_BACKUP_QUEUE_PARAMS_V2_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SP_BACKUP_QUEUE_PARAMS_V2_A {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SP_BACKUP_QUEUE_PARAMS_V2_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_BACKUP_QUEUE_PARAMS_V2_A>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SP_BACKUP_QUEUE_PARAMS_V2_A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SP_BACKUP_QUEUE_PARAMS_V2_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_BACKUP_QUEUE_PARAMS_V2_W {
    pub cbSize: u32,
    pub FullInfPath: [u16; 260],
    pub FilenameOffset: i32,
    pub ReinstallInstance: [u16; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_BACKUP_QUEUE_PARAMS_V2_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_BACKUP_QUEUE_PARAMS_V2_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for SP_BACKUP_QUEUE_PARAMS_V2_W {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SP_BACKUP_QUEUE_PARAMS_V2_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_BACKUP_QUEUE_PARAMS_V2_W>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SP_BACKUP_QUEUE_PARAMS_V2_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SP_BACKUP_QUEUE_PARAMS_V2_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_BACKUP_QUEUE_PARAMS_V2_W {
    pub cbSize: u32,
    pub FullInfPath: [u16; 260],
    pub FilenameOffset: i32,
    pub ReinstallInstance: [u16; 260],
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_BACKUP_QUEUE_PARAMS_V2_W {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_BACKUP_QUEUE_PARAMS_V2_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for SP_BACKUP_QUEUE_PARAMS_V2_W {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for SP_BACKUP_QUEUE_PARAMS_V2_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_BACKUP_QUEUE_PARAMS_V2_W>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for SP_BACKUP_QUEUE_PARAMS_V2_W {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for SP_BACKUP_QUEUE_PARAMS_V2_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_BACKUP_SPECIAL: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_UI_Controls\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_Controls")]
pub struct SP_CLASSIMAGELIST_DATA {
    pub cbSize: u32,
    pub ImageList: super::super::UI::Controls::HIMAGELIST,
    pub Reserved: usize,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_Controls")]
impl ::core::marker::Copy for SP_CLASSIMAGELIST_DATA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_Controls")]
impl ::core::clone::Clone for SP_CLASSIMAGELIST_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_Controls")]
unsafe impl ::windows::core::Abi for SP_CLASSIMAGELIST_DATA {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_Controls")]
impl ::core::cmp::PartialEq for SP_CLASSIMAGELIST_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_CLASSIMAGELIST_DATA>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_Controls")]
impl ::core::cmp::Eq for SP_CLASSIMAGELIST_DATA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_Controls")]
impl ::core::default::Default for SP_CLASSIMAGELIST_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_UI_Controls\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_Controls")]
pub struct SP_CLASSIMAGELIST_DATA {
    pub cbSize: u32,
    pub ImageList: super::super::UI::Controls::HIMAGELIST,
    pub Reserved: usize,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_Controls")]
impl ::core::marker::Copy for SP_CLASSIMAGELIST_DATA {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_Controls")]
impl ::core::clone::Clone for SP_CLASSIMAGELIST_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_Controls")]
unsafe impl ::windows::core::Abi for SP_CLASSIMAGELIST_DATA {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_Controls")]
impl ::core::cmp::PartialEq for SP_CLASSIMAGELIST_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_CLASSIMAGELIST_DATA>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_Controls")]
impl ::core::cmp::Eq for SP_CLASSIMAGELIST_DATA {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_Controls")]
impl ::core::default::Default for SP_CLASSIMAGELIST_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_CLASSINSTALL_HEADER {
    pub cbSize: u32,
    pub InstallFunction: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_CLASSINSTALL_HEADER {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_CLASSINSTALL_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for SP_CLASSINSTALL_HEADER {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SP_CLASSINSTALL_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_CLASSINSTALL_HEADER>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SP_CLASSINSTALL_HEADER {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SP_CLASSINSTALL_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_CLASSINSTALL_HEADER {
    pub cbSize: u32,
    pub InstallFunction: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_CLASSINSTALL_HEADER {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_CLASSINSTALL_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for SP_CLASSINSTALL_HEADER {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for SP_CLASSINSTALL_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_CLASSINSTALL_HEADER>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for SP_CLASSINSTALL_HEADER {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for SP_CLASSINSTALL_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SP_COPY_STYLE(pub u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_DELETESOURCE: SP_COPY_STYLE = SP_COPY_STYLE(1u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_REPLACEONLY: SP_COPY_STYLE = SP_COPY_STYLE(2u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_NEWER_OR_SAME: SP_COPY_STYLE = SP_COPY_STYLE(4u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_NEWER_ONLY: SP_COPY_STYLE = SP_COPY_STYLE(65536u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_NOOVERWRITE: SP_COPY_STYLE = SP_COPY_STYLE(8u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_NODECOMP: SP_COPY_STYLE = SP_COPY_STYLE(16u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_LANGUAGEAWARE: SP_COPY_STYLE = SP_COPY_STYLE(32u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_SOURCE_ABSOLUTE: SP_COPY_STYLE = SP_COPY_STYLE(64u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_SOURCEPATH_ABSOLUTE: SP_COPY_STYLE = SP_COPY_STYLE(128u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_FORCE_IN_USE: SP_COPY_STYLE = SP_COPY_STYLE(512u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_IN_USE_NEEDS_REBOOT: SP_COPY_STYLE = SP_COPY_STYLE(256u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_NOSKIP: SP_COPY_STYLE = SP_COPY_STYLE(1024u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_FORCE_NOOVERWRITE: SP_COPY_STYLE = SP_COPY_STYLE(4096u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_FORCE_NEWER: SP_COPY_STYLE = SP_COPY_STYLE(8192u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_WARNIFSKIP: SP_COPY_STYLE = SP_COPY_STYLE(16384u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_NOBROWSE: SP_COPY_STYLE = SP_COPY_STYLE(32768u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_NEWER: SP_COPY_STYLE = SP_COPY_STYLE(4u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_RESERVED: SP_COPY_STYLE = SP_COPY_STYLE(131072u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_OEMINF_CATALOG_ONLY: SP_COPY_STYLE = SP_COPY_STYLE(262144u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_REPLACE_BOOT_FILE: SP_COPY_STYLE = SP_COPY_STYLE(524288u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_NOPRUNE: SP_COPY_STYLE = SP_COPY_STYLE(1048576u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_OEM_F6_INF: SP_COPY_STYLE = SP_COPY_STYLE(2097152u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_ALREADYDECOMP: SP_COPY_STYLE = SP_COPY_STYLE(4194304u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_WINDOWS_SIGNED: SP_COPY_STYLE = SP_COPY_STYLE(16777216u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_PNPLOCKED: SP_COPY_STYLE = SP_COPY_STYLE(33554432u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_IN_USE_TRY_RENAME: SP_COPY_STYLE = SP_COPY_STYLE(67108864u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_INBOX_INF: SP_COPY_STYLE = SP_COPY_STYLE(134217728u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_HARDLINK: SP_COPY_STYLE = SP_COPY_STYLE(268435456u32);
impl ::core::marker::Copy for SP_COPY_STYLE {}
impl ::core::clone::Clone for SP_COPY_STYLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SP_COPY_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SP_COPY_STYLE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SP_COPY_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SP_COPY_STYLE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SP_COPY_STYLE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SP_COPY_STYLE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SP_COPY_STYLE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SP_COPY_STYLE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SP_COPY_STYLE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DETECTDEVICE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub DetectProgressNotify: PDETECT_PROGRESS_NOTIFY,
    pub ProgressNotifyParam: *mut ::core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DETECTDEVICE_PARAMS {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DETECTDEVICE_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SP_DETECTDEVICE_PARAMS {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SP_DETECTDEVICE_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_DETECTDEVICE_PARAMS>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SP_DETECTDEVICE_PARAMS {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SP_DETECTDEVICE_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DETECTDEVICE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub DetectProgressNotify: PDETECT_PROGRESS_NOTIFY,
    pub ProgressNotifyParam: *mut ::core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DETECTDEVICE_PARAMS {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DETECTDEVICE_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SP_DETECTDEVICE_PARAMS {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SP_DETECTDEVICE_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_DETECTDEVICE_PARAMS>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SP_DETECTDEVICE_PARAMS {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SP_DETECTDEVICE_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_DEVICE_INTERFACE_DATA {
    pub cbSize: u32,
    pub InterfaceClassGuid: ::windows::core::GUID,
    pub Flags: u32,
    pub Reserved: usize,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_DEVICE_INTERFACE_DATA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_DEVICE_INTERFACE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for SP_DEVICE_INTERFACE_DATA {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SP_DEVICE_INTERFACE_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_DEVICE_INTERFACE_DATA>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SP_DEVICE_INTERFACE_DATA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SP_DEVICE_INTERFACE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_DEVICE_INTERFACE_DATA {
    pub cbSize: u32,
    pub InterfaceClassGuid: ::windows::core::GUID,
    pub Flags: u32,
    pub Reserved: usize,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_DEVICE_INTERFACE_DATA {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_DEVICE_INTERFACE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for SP_DEVICE_INTERFACE_DATA {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for SP_DEVICE_INTERFACE_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_DEVICE_INTERFACE_DATA>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for SP_DEVICE_INTERFACE_DATA {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for SP_DEVICE_INTERFACE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DEVICE_INTERFACE_DETAIL_DATA_A {
    pub cbSize: u32,
    pub DevicePath: [super::super::Foundation::CHAR; 1],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DEVICE_INTERFACE_DETAIL_DATA_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DEVICE_INTERFACE_DETAIL_DATA_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SP_DEVICE_INTERFACE_DETAIL_DATA_A {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SP_DEVICE_INTERFACE_DETAIL_DATA_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_DEVICE_INTERFACE_DETAIL_DATA_A>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SP_DEVICE_INTERFACE_DETAIL_DATA_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SP_DEVICE_INTERFACE_DETAIL_DATA_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DEVICE_INTERFACE_DETAIL_DATA_A {
    pub cbSize: u32,
    pub DevicePath: [super::super::Foundation::CHAR; 1],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DEVICE_INTERFACE_DETAIL_DATA_A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DEVICE_INTERFACE_DETAIL_DATA_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SP_DEVICE_INTERFACE_DETAIL_DATA_A {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SP_DEVICE_INTERFACE_DETAIL_DATA_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_DEVICE_INTERFACE_DETAIL_DATA_A>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SP_DEVICE_INTERFACE_DETAIL_DATA_A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SP_DEVICE_INTERFACE_DETAIL_DATA_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_DEVICE_INTERFACE_DETAIL_DATA_W {
    pub cbSize: u32,
    pub DevicePath: [u16; 1],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_DEVICE_INTERFACE_DETAIL_DATA_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_DEVICE_INTERFACE_DETAIL_DATA_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for SP_DEVICE_INTERFACE_DETAIL_DATA_W {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SP_DEVICE_INTERFACE_DETAIL_DATA_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_DEVICE_INTERFACE_DETAIL_DATA_W>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SP_DEVICE_INTERFACE_DETAIL_DATA_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SP_DEVICE_INTERFACE_DETAIL_DATA_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_DEVICE_INTERFACE_DETAIL_DATA_W {
    pub cbSize: u32,
    pub DevicePath: [u16; 1],
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_DEVICE_INTERFACE_DETAIL_DATA_W {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_DEVICE_INTERFACE_DETAIL_DATA_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for SP_DEVICE_INTERFACE_DETAIL_DATA_W {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for SP_DEVICE_INTERFACE_DETAIL_DATA_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_DEVICE_INTERFACE_DETAIL_DATA_W>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for SP_DEVICE_INTERFACE_DETAIL_DATA_W {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for SP_DEVICE_INTERFACE_DETAIL_DATA_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_DEVINFO_DATA {
    pub cbSize: u32,
    pub ClassGuid: ::windows::core::GUID,
    pub DevInst: u32,
    pub Reserved: usize,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_DEVINFO_DATA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_DEVINFO_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for SP_DEVINFO_DATA {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SP_DEVINFO_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_DEVINFO_DATA>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SP_DEVINFO_DATA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SP_DEVINFO_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_DEVINFO_DATA {
    pub cbSize: u32,
    pub ClassGuid: ::windows::core::GUID,
    pub DevInst: u32,
    pub Reserved: usize,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_DEVINFO_DATA {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_DEVINFO_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for SP_DEVINFO_DATA {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for SP_DEVINFO_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_DEVINFO_DATA>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for SP_DEVINFO_DATA {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for SP_DEVINFO_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DEVINFO_LIST_DETAIL_DATA_A {
    pub cbSize: u32,
    pub ClassGuid: ::windows::core::GUID,
    pub RemoteMachineHandle: super::super::Foundation::HANDLE,
    pub RemoteMachineName: [super::super::Foundation::CHAR; 263],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DEVINFO_LIST_DETAIL_DATA_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DEVINFO_LIST_DETAIL_DATA_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SP_DEVINFO_LIST_DETAIL_DATA_A {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SP_DEVINFO_LIST_DETAIL_DATA_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_DEVINFO_LIST_DETAIL_DATA_A>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SP_DEVINFO_LIST_DETAIL_DATA_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SP_DEVINFO_LIST_DETAIL_DATA_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DEVINFO_LIST_DETAIL_DATA_A {
    pub cbSize: u32,
    pub ClassGuid: ::windows::core::GUID,
    pub RemoteMachineHandle: super::super::Foundation::HANDLE,
    pub RemoteMachineName: [super::super::Foundation::CHAR; 263],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DEVINFO_LIST_DETAIL_DATA_A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DEVINFO_LIST_DETAIL_DATA_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SP_DEVINFO_LIST_DETAIL_DATA_A {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SP_DEVINFO_LIST_DETAIL_DATA_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_DEVINFO_LIST_DETAIL_DATA_A>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SP_DEVINFO_LIST_DETAIL_DATA_A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SP_DEVINFO_LIST_DETAIL_DATA_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DEVINFO_LIST_DETAIL_DATA_W {
    pub cbSize: u32,
    pub ClassGuid: ::windows::core::GUID,
    pub RemoteMachineHandle: super::super::Foundation::HANDLE,
    pub RemoteMachineName: [u16; 263],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DEVINFO_LIST_DETAIL_DATA_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DEVINFO_LIST_DETAIL_DATA_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SP_DEVINFO_LIST_DETAIL_DATA_W {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SP_DEVINFO_LIST_DETAIL_DATA_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_DEVINFO_LIST_DETAIL_DATA_W>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SP_DEVINFO_LIST_DETAIL_DATA_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SP_DEVINFO_LIST_DETAIL_DATA_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DEVINFO_LIST_DETAIL_DATA_W {
    pub cbSize: u32,
    pub ClassGuid: ::windows::core::GUID,
    pub RemoteMachineHandle: super::super::Foundation::HANDLE,
    pub RemoteMachineName: [u16; 263],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DEVINFO_LIST_DETAIL_DATA_W {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DEVINFO_LIST_DETAIL_DATA_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SP_DEVINFO_LIST_DETAIL_DATA_W {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SP_DEVINFO_LIST_DETAIL_DATA_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_DEVINFO_LIST_DETAIL_DATA_W>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SP_DEVINFO_LIST_DETAIL_DATA_W {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SP_DEVINFO_LIST_DETAIL_DATA_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DEVINSTALL_PARAMS_A {
    pub cbSize: u32,
    pub Flags: u32,
    pub FlagsEx: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub InstallMsgHandler: PSP_FILE_CALLBACK_A,
    pub InstallMsgHandlerContext: *mut ::core::ffi::c_void,
    pub FileQueue: *mut ::core::ffi::c_void,
    pub ClassInstallReserved: usize,
    pub Reserved: u32,
    pub DriverPath: [super::super::Foundation::CHAR; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DEVINSTALL_PARAMS_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DEVINSTALL_PARAMS_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SP_DEVINSTALL_PARAMS_A {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SP_DEVINSTALL_PARAMS_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_DEVINSTALL_PARAMS_A>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SP_DEVINSTALL_PARAMS_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SP_DEVINSTALL_PARAMS_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DEVINSTALL_PARAMS_A {
    pub cbSize: u32,
    pub Flags: u32,
    pub FlagsEx: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub InstallMsgHandler: PSP_FILE_CALLBACK_A,
    pub InstallMsgHandlerContext: *mut ::core::ffi::c_void,
    pub FileQueue: *mut ::core::ffi::c_void,
    pub ClassInstallReserved: usize,
    pub Reserved: u32,
    pub DriverPath: [super::super::Foundation::CHAR; 260],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DEVINSTALL_PARAMS_A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DEVINSTALL_PARAMS_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SP_DEVINSTALL_PARAMS_A {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SP_DEVINSTALL_PARAMS_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_DEVINSTALL_PARAMS_A>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SP_DEVINSTALL_PARAMS_A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SP_DEVINSTALL_PARAMS_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DEVINSTALL_PARAMS_W {
    pub cbSize: u32,
    pub Flags: u32,
    pub FlagsEx: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub InstallMsgHandler: PSP_FILE_CALLBACK_A,
    pub InstallMsgHandlerContext: *mut ::core::ffi::c_void,
    pub FileQueue: *mut ::core::ffi::c_void,
    pub ClassInstallReserved: usize,
    pub Reserved: u32,
    pub DriverPath: [u16; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DEVINSTALL_PARAMS_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DEVINSTALL_PARAMS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SP_DEVINSTALL_PARAMS_W {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SP_DEVINSTALL_PARAMS_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_DEVINSTALL_PARAMS_W>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SP_DEVINSTALL_PARAMS_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SP_DEVINSTALL_PARAMS_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DEVINSTALL_PARAMS_W {
    pub cbSize: u32,
    pub Flags: u32,
    pub FlagsEx: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub InstallMsgHandler: PSP_FILE_CALLBACK_A,
    pub InstallMsgHandlerContext: *mut ::core::ffi::c_void,
    pub FileQueue: *mut ::core::ffi::c_void,
    pub ClassInstallReserved: usize,
    pub Reserved: u32,
    pub DriverPath: [u16; 260],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DEVINSTALL_PARAMS_W {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DEVINSTALL_PARAMS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SP_DEVINSTALL_PARAMS_W {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SP_DEVINSTALL_PARAMS_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_DEVINSTALL_PARAMS_W>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SP_DEVINSTALL_PARAMS_W {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SP_DEVINSTALL_PARAMS_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DRVINFO_DATA_V1_A {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [super::super::Foundation::CHAR; 256],
    pub MfgName: [super::super::Foundation::CHAR; 256],
    pub ProviderName: [super::super::Foundation::CHAR; 256],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DRVINFO_DATA_V1_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DRVINFO_DATA_V1_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SP_DRVINFO_DATA_V1_A {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SP_DRVINFO_DATA_V1_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_DRVINFO_DATA_V1_A>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SP_DRVINFO_DATA_V1_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SP_DRVINFO_DATA_V1_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DRVINFO_DATA_V1_A {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [super::super::Foundation::CHAR; 256],
    pub MfgName: [super::super::Foundation::CHAR; 256],
    pub ProviderName: [super::super::Foundation::CHAR; 256],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DRVINFO_DATA_V1_A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DRVINFO_DATA_V1_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SP_DRVINFO_DATA_V1_A {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SP_DRVINFO_DATA_V1_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_DRVINFO_DATA_V1_A>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SP_DRVINFO_DATA_V1_A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SP_DRVINFO_DATA_V1_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_DRVINFO_DATA_V1_W {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [u16; 256],
    pub MfgName: [u16; 256],
    pub ProviderName: [u16; 256],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_DRVINFO_DATA_V1_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_DRVINFO_DATA_V1_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for SP_DRVINFO_DATA_V1_W {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SP_DRVINFO_DATA_V1_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_DRVINFO_DATA_V1_W>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SP_DRVINFO_DATA_V1_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SP_DRVINFO_DATA_V1_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_DRVINFO_DATA_V1_W {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [u16; 256],
    pub MfgName: [u16; 256],
    pub ProviderName: [u16; 256],
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_DRVINFO_DATA_V1_W {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_DRVINFO_DATA_V1_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for SP_DRVINFO_DATA_V1_W {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for SP_DRVINFO_DATA_V1_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_DRVINFO_DATA_V1_W>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for SP_DRVINFO_DATA_V1_W {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for SP_DRVINFO_DATA_V1_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DRVINFO_DATA_V2_A {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [super::super::Foundation::CHAR; 256],
    pub MfgName: [super::super::Foundation::CHAR; 256],
    pub ProviderName: [super::super::Foundation::CHAR; 256],
    pub DriverDate: super::super::Foundation::FILETIME,
    pub DriverVersion: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DRVINFO_DATA_V2_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DRVINFO_DATA_V2_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SP_DRVINFO_DATA_V2_A {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SP_DRVINFO_DATA_V2_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_DRVINFO_DATA_V2_A>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SP_DRVINFO_DATA_V2_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SP_DRVINFO_DATA_V2_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DRVINFO_DATA_V2_A {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [super::super::Foundation::CHAR; 256],
    pub MfgName: [super::super::Foundation::CHAR; 256],
    pub ProviderName: [super::super::Foundation::CHAR; 256],
    pub DriverDate: super::super::Foundation::FILETIME,
    pub DriverVersion: u64,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DRVINFO_DATA_V2_A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DRVINFO_DATA_V2_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SP_DRVINFO_DATA_V2_A {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SP_DRVINFO_DATA_V2_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_DRVINFO_DATA_V2_A>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SP_DRVINFO_DATA_V2_A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SP_DRVINFO_DATA_V2_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DRVINFO_DATA_V2_W {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [u16; 256],
    pub MfgName: [u16; 256],
    pub ProviderName: [u16; 256],
    pub DriverDate: super::super::Foundation::FILETIME,
    pub DriverVersion: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DRVINFO_DATA_V2_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DRVINFO_DATA_V2_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SP_DRVINFO_DATA_V2_W {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SP_DRVINFO_DATA_V2_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_DRVINFO_DATA_V2_W>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SP_DRVINFO_DATA_V2_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SP_DRVINFO_DATA_V2_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DRVINFO_DATA_V2_W {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [u16; 256],
    pub MfgName: [u16; 256],
    pub ProviderName: [u16; 256],
    pub DriverDate: super::super::Foundation::FILETIME,
    pub DriverVersion: u64,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DRVINFO_DATA_V2_W {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DRVINFO_DATA_V2_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SP_DRVINFO_DATA_V2_W {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SP_DRVINFO_DATA_V2_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_DRVINFO_DATA_V2_W>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SP_DRVINFO_DATA_V2_W {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SP_DRVINFO_DATA_V2_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DRVINFO_DETAIL_DATA_A {
    pub cbSize: u32,
    pub InfDate: super::super::Foundation::FILETIME,
    pub CompatIDsOffset: u32,
    pub CompatIDsLength: u32,
    pub Reserved: usize,
    pub SectionName: [super::super::Foundation::CHAR; 256],
    pub InfFileName: [super::super::Foundation::CHAR; 260],
    pub DrvDescription: [super::super::Foundation::CHAR; 256],
    pub HardwareID: [super::super::Foundation::CHAR; 1],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DRVINFO_DETAIL_DATA_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DRVINFO_DETAIL_DATA_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SP_DRVINFO_DETAIL_DATA_A {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SP_DRVINFO_DETAIL_DATA_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_DRVINFO_DETAIL_DATA_A>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SP_DRVINFO_DETAIL_DATA_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SP_DRVINFO_DETAIL_DATA_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DRVINFO_DETAIL_DATA_A {
    pub cbSize: u32,
    pub InfDate: super::super::Foundation::FILETIME,
    pub CompatIDsOffset: u32,
    pub CompatIDsLength: u32,
    pub Reserved: usize,
    pub SectionName: [super::super::Foundation::CHAR; 256],
    pub InfFileName: [super::super::Foundation::CHAR; 260],
    pub DrvDescription: [super::super::Foundation::CHAR; 256],
    pub HardwareID: [super::super::Foundation::CHAR; 1],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DRVINFO_DETAIL_DATA_A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DRVINFO_DETAIL_DATA_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SP_DRVINFO_DETAIL_DATA_A {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SP_DRVINFO_DETAIL_DATA_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_DRVINFO_DETAIL_DATA_A>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SP_DRVINFO_DETAIL_DATA_A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SP_DRVINFO_DETAIL_DATA_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DRVINFO_DETAIL_DATA_W {
    pub cbSize: u32,
    pub InfDate: super::super::Foundation::FILETIME,
    pub CompatIDsOffset: u32,
    pub CompatIDsLength: u32,
    pub Reserved: usize,
    pub SectionName: [u16; 256],
    pub InfFileName: [u16; 260],
    pub DrvDescription: [u16; 256],
    pub HardwareID: [u16; 1],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DRVINFO_DETAIL_DATA_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DRVINFO_DETAIL_DATA_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SP_DRVINFO_DETAIL_DATA_W {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SP_DRVINFO_DETAIL_DATA_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_DRVINFO_DETAIL_DATA_W>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SP_DRVINFO_DETAIL_DATA_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SP_DRVINFO_DETAIL_DATA_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DRVINFO_DETAIL_DATA_W {
    pub cbSize: u32,
    pub InfDate: super::super::Foundation::FILETIME,
    pub CompatIDsOffset: u32,
    pub CompatIDsLength: u32,
    pub Reserved: usize,
    pub SectionName: [u16; 256],
    pub InfFileName: [u16; 260],
    pub DrvDescription: [u16; 256],
    pub HardwareID: [u16; 1],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DRVINFO_DETAIL_DATA_W {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DRVINFO_DETAIL_DATA_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SP_DRVINFO_DETAIL_DATA_W {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SP_DRVINFO_DETAIL_DATA_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_DRVINFO_DETAIL_DATA_W>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SP_DRVINFO_DETAIL_DATA_W {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SP_DRVINFO_DETAIL_DATA_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_DRVINSTALL_PARAMS {
    pub cbSize: u32,
    pub Rank: u32,
    pub Flags: u32,
    pub PrivateData: usize,
    pub Reserved: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_DRVINSTALL_PARAMS {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_DRVINSTALL_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for SP_DRVINSTALL_PARAMS {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SP_DRVINSTALL_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_DRVINSTALL_PARAMS>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SP_DRVINSTALL_PARAMS {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SP_DRVINSTALL_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_DRVINSTALL_PARAMS {
    pub cbSize: u32,
    pub Rank: u32,
    pub Flags: u32,
    pub PrivateData: usize,
    pub Reserved: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_DRVINSTALL_PARAMS {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_DRVINSTALL_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for SP_DRVINSTALL_PARAMS {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for SP_DRVINSTALL_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_DRVINSTALL_PARAMS>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for SP_DRVINSTALL_PARAMS {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for SP_DRVINSTALL_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_ENABLECLASS_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub ClassGuid: ::windows::core::GUID,
    pub EnableMessage: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_ENABLECLASS_PARAMS {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_ENABLECLASS_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for SP_ENABLECLASS_PARAMS {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SP_ENABLECLASS_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_ENABLECLASS_PARAMS>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SP_ENABLECLASS_PARAMS {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SP_ENABLECLASS_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_ENABLECLASS_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub ClassGuid: ::windows::core::GUID,
    pub EnableMessage: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_ENABLECLASS_PARAMS {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_ENABLECLASS_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for SP_ENABLECLASS_PARAMS {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for SP_ENABLECLASS_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_ENABLECLASS_PARAMS>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for SP_ENABLECLASS_PARAMS {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for SP_ENABLECLASS_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_FILE_COPY_PARAMS_A {
    pub cbSize: u32,
    pub QueueHandle: *mut ::core::ffi::c_void,
    pub SourceRootPath: ::windows::core::PCSTR,
    pub SourcePath: ::windows::core::PCSTR,
    pub SourceFilename: ::windows::core::PCSTR,
    pub SourceDescription: ::windows::core::PCSTR,
    pub SourceTagfile: ::windows::core::PCSTR,
    pub TargetDirectory: ::windows::core::PCSTR,
    pub TargetFilename: ::windows::core::PCSTR,
    pub CopyStyle: u32,
    pub LayoutInf: *mut ::core::ffi::c_void,
    pub SecurityDescriptor: ::windows::core::PCSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_FILE_COPY_PARAMS_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_FILE_COPY_PARAMS_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for SP_FILE_COPY_PARAMS_A {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SP_FILE_COPY_PARAMS_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_FILE_COPY_PARAMS_A>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SP_FILE_COPY_PARAMS_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SP_FILE_COPY_PARAMS_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_FILE_COPY_PARAMS_A {
    pub cbSize: u32,
    pub QueueHandle: *mut ::core::ffi::c_void,
    pub SourceRootPath: ::windows::core::PCSTR,
    pub SourcePath: ::windows::core::PCSTR,
    pub SourceFilename: ::windows::core::PCSTR,
    pub SourceDescription: ::windows::core::PCSTR,
    pub SourceTagfile: ::windows::core::PCSTR,
    pub TargetDirectory: ::windows::core::PCSTR,
    pub TargetFilename: ::windows::core::PCSTR,
    pub CopyStyle: u32,
    pub LayoutInf: *mut ::core::ffi::c_void,
    pub SecurityDescriptor: ::windows::core::PCSTR,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_FILE_COPY_PARAMS_A {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_FILE_COPY_PARAMS_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for SP_FILE_COPY_PARAMS_A {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for SP_FILE_COPY_PARAMS_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_FILE_COPY_PARAMS_A>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for SP_FILE_COPY_PARAMS_A {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for SP_FILE_COPY_PARAMS_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_FILE_COPY_PARAMS_W {
    pub cbSize: u32,
    pub QueueHandle: *mut ::core::ffi::c_void,
    pub SourceRootPath: ::windows::core::PCWSTR,
    pub SourcePath: ::windows::core::PCWSTR,
    pub SourceFilename: ::windows::core::PCWSTR,
    pub SourceDescription: ::windows::core::PCWSTR,
    pub SourceTagfile: ::windows::core::PCWSTR,
    pub TargetDirectory: ::windows::core::PCWSTR,
    pub TargetFilename: ::windows::core::PCWSTR,
    pub CopyStyle: u32,
    pub LayoutInf: *mut ::core::ffi::c_void,
    pub SecurityDescriptor: ::windows::core::PCWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_FILE_COPY_PARAMS_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_FILE_COPY_PARAMS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for SP_FILE_COPY_PARAMS_W {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SP_FILE_COPY_PARAMS_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_FILE_COPY_PARAMS_W>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SP_FILE_COPY_PARAMS_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SP_FILE_COPY_PARAMS_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_FILE_COPY_PARAMS_W {
    pub cbSize: u32,
    pub QueueHandle: *mut ::core::ffi::c_void,
    pub SourceRootPath: ::windows::core::PCWSTR,
    pub SourcePath: ::windows::core::PCWSTR,
    pub SourceFilename: ::windows::core::PCWSTR,
    pub SourceDescription: ::windows::core::PCWSTR,
    pub SourceTagfile: ::windows::core::PCWSTR,
    pub TargetDirectory: ::windows::core::PCWSTR,
    pub TargetFilename: ::windows::core::PCWSTR,
    pub CopyStyle: u32,
    pub LayoutInf: *mut ::core::ffi::c_void,
    pub SecurityDescriptor: ::windows::core::PCWSTR,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_FILE_COPY_PARAMS_W {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_FILE_COPY_PARAMS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for SP_FILE_COPY_PARAMS_W {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for SP_FILE_COPY_PARAMS_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_FILE_COPY_PARAMS_W>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for SP_FILE_COPY_PARAMS_W {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for SP_FILE_COPY_PARAMS_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_FLAG_CABINETCONTINUATION: u32 = 2048u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_INF_INFORMATION {
    pub InfStyle: SP_INF_STYLE,
    pub InfCount: u32,
    pub VersionData: [u8; 1],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_INF_INFORMATION {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_INF_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for SP_INF_INFORMATION {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SP_INF_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_INF_INFORMATION>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SP_INF_INFORMATION {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SP_INF_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_INF_INFORMATION {
    pub InfStyle: SP_INF_STYLE,
    pub InfCount: u32,
    pub VersionData: [u8; 1],
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_INF_INFORMATION {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_INF_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for SP_INF_INFORMATION {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for SP_INF_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_INF_INFORMATION>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for SP_INF_INFORMATION {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for SP_INF_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_INF_SIGNER_INFO_V1_A {
    pub cbSize: u32,
    pub CatalogFile: [super::super::Foundation::CHAR; 260],
    pub DigitalSigner: [super::super::Foundation::CHAR; 260],
    pub DigitalSignerVersion: [super::super::Foundation::CHAR; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_INF_SIGNER_INFO_V1_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_INF_SIGNER_INFO_V1_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SP_INF_SIGNER_INFO_V1_A {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SP_INF_SIGNER_INFO_V1_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_INF_SIGNER_INFO_V1_A>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SP_INF_SIGNER_INFO_V1_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SP_INF_SIGNER_INFO_V1_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_INF_SIGNER_INFO_V1_A {
    pub cbSize: u32,
    pub CatalogFile: [super::super::Foundation::CHAR; 260],
    pub DigitalSigner: [super::super::Foundation::CHAR; 260],
    pub DigitalSignerVersion: [super::super::Foundation::CHAR; 260],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_INF_SIGNER_INFO_V1_A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_INF_SIGNER_INFO_V1_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SP_INF_SIGNER_INFO_V1_A {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SP_INF_SIGNER_INFO_V1_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_INF_SIGNER_INFO_V1_A>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SP_INF_SIGNER_INFO_V1_A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SP_INF_SIGNER_INFO_V1_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_INF_SIGNER_INFO_V1_W {
    pub cbSize: u32,
    pub CatalogFile: [u16; 260],
    pub DigitalSigner: [u16; 260],
    pub DigitalSignerVersion: [u16; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_INF_SIGNER_INFO_V1_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_INF_SIGNER_INFO_V1_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for SP_INF_SIGNER_INFO_V1_W {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SP_INF_SIGNER_INFO_V1_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_INF_SIGNER_INFO_V1_W>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SP_INF_SIGNER_INFO_V1_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SP_INF_SIGNER_INFO_V1_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_INF_SIGNER_INFO_V1_W {
    pub cbSize: u32,
    pub CatalogFile: [u16; 260],
    pub DigitalSigner: [u16; 260],
    pub DigitalSignerVersion: [u16; 260],
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_INF_SIGNER_INFO_V1_W {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_INF_SIGNER_INFO_V1_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for SP_INF_SIGNER_INFO_V1_W {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for SP_INF_SIGNER_INFO_V1_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_INF_SIGNER_INFO_V1_W>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for SP_INF_SIGNER_INFO_V1_W {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for SP_INF_SIGNER_INFO_V1_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_INF_SIGNER_INFO_V2_A {
    pub cbSize: u32,
    pub CatalogFile: [super::super::Foundation::CHAR; 260],
    pub DigitalSigner: [super::super::Foundation::CHAR; 260],
    pub DigitalSignerVersion: [super::super::Foundation::CHAR; 260],
    pub SignerScore: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_INF_SIGNER_INFO_V2_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_INF_SIGNER_INFO_V2_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SP_INF_SIGNER_INFO_V2_A {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SP_INF_SIGNER_INFO_V2_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_INF_SIGNER_INFO_V2_A>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SP_INF_SIGNER_INFO_V2_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SP_INF_SIGNER_INFO_V2_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_INF_SIGNER_INFO_V2_A {
    pub cbSize: u32,
    pub CatalogFile: [super::super::Foundation::CHAR; 260],
    pub DigitalSigner: [super::super::Foundation::CHAR; 260],
    pub DigitalSignerVersion: [super::super::Foundation::CHAR; 260],
    pub SignerScore: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_INF_SIGNER_INFO_V2_A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_INF_SIGNER_INFO_V2_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SP_INF_SIGNER_INFO_V2_A {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SP_INF_SIGNER_INFO_V2_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_INF_SIGNER_INFO_V2_A>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SP_INF_SIGNER_INFO_V2_A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SP_INF_SIGNER_INFO_V2_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_INF_SIGNER_INFO_V2_W {
    pub cbSize: u32,
    pub CatalogFile: [u16; 260],
    pub DigitalSigner: [u16; 260],
    pub DigitalSignerVersion: [u16; 260],
    pub SignerScore: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_INF_SIGNER_INFO_V2_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_INF_SIGNER_INFO_V2_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for SP_INF_SIGNER_INFO_V2_W {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SP_INF_SIGNER_INFO_V2_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_INF_SIGNER_INFO_V2_W>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SP_INF_SIGNER_INFO_V2_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SP_INF_SIGNER_INFO_V2_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_INF_SIGNER_INFO_V2_W {
    pub cbSize: u32,
    pub CatalogFile: [u16; 260],
    pub DigitalSigner: [u16; 260],
    pub DigitalSignerVersion: [u16; 260],
    pub SignerScore: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_INF_SIGNER_INFO_V2_W {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_INF_SIGNER_INFO_V2_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for SP_INF_SIGNER_INFO_V2_W {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for SP_INF_SIGNER_INFO_V2_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_INF_SIGNER_INFO_V2_W>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for SP_INF_SIGNER_INFO_V2_W {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for SP_INF_SIGNER_INFO_V2_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SP_INF_STYLE(pub u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INF_STYLE_NONE: SP_INF_STYLE = SP_INF_STYLE(0u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INF_STYLE_OLDNT: SP_INF_STYLE = SP_INF_STYLE(1u32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INF_STYLE_WIN4: SP_INF_STYLE = SP_INF_STYLE(2u32);
impl ::core::marker::Copy for SP_INF_STYLE {}
impl ::core::clone::Clone for SP_INF_STYLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SP_INF_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SP_INF_STYLE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SP_INF_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SP_INF_STYLE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub struct SP_INSTALLWIZARD_DATA {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Flags: u32,
    pub DynamicPages: [super::super::UI::Controls::HPROPSHEETPAGE; 20],
    pub NumDynamicPages: u32,
    pub DynamicPageFlags: u32,
    pub PrivateFlags: u32,
    pub PrivateData: super::super::Foundation::LPARAM,
    pub hwndWizardDlg: super::super::Foundation::HWND,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::marker::Copy for SP_INSTALLWIZARD_DATA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::clone::Clone for SP_INSTALLWIZARD_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
unsafe impl ::windows::core::Abi for SP_INSTALLWIZARD_DATA {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::cmp::PartialEq for SP_INSTALLWIZARD_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_INSTALLWIZARD_DATA>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::cmp::Eq for SP_INSTALLWIZARD_DATA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::default::Default for SP_INSTALLWIZARD_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub struct SP_INSTALLWIZARD_DATA {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Flags: u32,
    pub DynamicPages: [super::super::UI::Controls::HPROPSHEETPAGE; 20],
    pub NumDynamicPages: u32,
    pub DynamicPageFlags: u32,
    pub PrivateFlags: u32,
    pub PrivateData: super::super::Foundation::LPARAM,
    pub hwndWizardDlg: super::super::Foundation::HWND,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::marker::Copy for SP_INSTALLWIZARD_DATA {}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::clone::Clone for SP_INSTALLWIZARD_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
unsafe impl ::windows::core::Abi for SP_INSTALLWIZARD_DATA {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::cmp::PartialEq for SP_INSTALLWIZARD_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_INSTALLWIZARD_DATA>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::cmp::Eq for SP_INSTALLWIZARD_DATA {}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::default::Default for SP_INSTALLWIZARD_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_MAX_MACHINENAME_LENGTH: u32 = 263u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub struct SP_NEWDEVICEWIZARD_DATA {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Flags: u32,
    pub DynamicPages: [super::super::UI::Controls::HPROPSHEETPAGE; 20],
    pub NumDynamicPages: u32,
    pub hwndWizardDlg: super::super::Foundation::HWND,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::marker::Copy for SP_NEWDEVICEWIZARD_DATA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::clone::Clone for SP_NEWDEVICEWIZARD_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
unsafe impl ::windows::core::Abi for SP_NEWDEVICEWIZARD_DATA {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::cmp::PartialEq for SP_NEWDEVICEWIZARD_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_NEWDEVICEWIZARD_DATA>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::cmp::Eq for SP_NEWDEVICEWIZARD_DATA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::default::Default for SP_NEWDEVICEWIZARD_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub struct SP_NEWDEVICEWIZARD_DATA {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Flags: u32,
    pub DynamicPages: [super::super::UI::Controls::HPROPSHEETPAGE; 20],
    pub NumDynamicPages: u32,
    pub hwndWizardDlg: super::super::Foundation::HWND,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::marker::Copy for SP_NEWDEVICEWIZARD_DATA {}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::clone::Clone for SP_NEWDEVICEWIZARD_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
unsafe impl ::windows::core::Abi for SP_NEWDEVICEWIZARD_DATA {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::cmp::PartialEq for SP_NEWDEVICEWIZARD_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_NEWDEVICEWIZARD_DATA>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::cmp::Eq for SP_NEWDEVICEWIZARD_DATA {}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::default::Default for SP_NEWDEVICEWIZARD_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_ORIGINAL_FILE_INFO_A {
    pub cbSize: u32,
    pub OriginalInfName: [super::super::Foundation::CHAR; 260],
    pub OriginalCatalogName: [super::super::Foundation::CHAR; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_ORIGINAL_FILE_INFO_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_ORIGINAL_FILE_INFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SP_ORIGINAL_FILE_INFO_A {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SP_ORIGINAL_FILE_INFO_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_ORIGINAL_FILE_INFO_A>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SP_ORIGINAL_FILE_INFO_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SP_ORIGINAL_FILE_INFO_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_ORIGINAL_FILE_INFO_A {
    pub cbSize: u32,
    pub OriginalInfName: [super::super::Foundation::CHAR; 260],
    pub OriginalCatalogName: [super::super::Foundation::CHAR; 260],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_ORIGINAL_FILE_INFO_A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_ORIGINAL_FILE_INFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SP_ORIGINAL_FILE_INFO_A {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SP_ORIGINAL_FILE_INFO_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_ORIGINAL_FILE_INFO_A>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SP_ORIGINAL_FILE_INFO_A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SP_ORIGINAL_FILE_INFO_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_ORIGINAL_FILE_INFO_W {
    pub cbSize: u32,
    pub OriginalInfName: [u16; 260],
    pub OriginalCatalogName: [u16; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_ORIGINAL_FILE_INFO_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_ORIGINAL_FILE_INFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for SP_ORIGINAL_FILE_INFO_W {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SP_ORIGINAL_FILE_INFO_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_ORIGINAL_FILE_INFO_W>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SP_ORIGINAL_FILE_INFO_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SP_ORIGINAL_FILE_INFO_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_ORIGINAL_FILE_INFO_W {
    pub cbSize: u32,
    pub OriginalInfName: [u16; 260],
    pub OriginalCatalogName: [u16; 260],
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_ORIGINAL_FILE_INFO_W {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_ORIGINAL_FILE_INFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for SP_ORIGINAL_FILE_INFO_W {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for SP_ORIGINAL_FILE_INFO_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_ORIGINAL_FILE_INFO_W>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for SP_ORIGINAL_FILE_INFO_W {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for SP_ORIGINAL_FILE_INFO_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_POWERMESSAGEWAKE_PARAMS_A {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub PowerMessageWake: [super::super::Foundation::CHAR; 512],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_POWERMESSAGEWAKE_PARAMS_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_POWERMESSAGEWAKE_PARAMS_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SP_POWERMESSAGEWAKE_PARAMS_A {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SP_POWERMESSAGEWAKE_PARAMS_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_POWERMESSAGEWAKE_PARAMS_A>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SP_POWERMESSAGEWAKE_PARAMS_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SP_POWERMESSAGEWAKE_PARAMS_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_POWERMESSAGEWAKE_PARAMS_W {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub PowerMessageWake: [u16; 512],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_POWERMESSAGEWAKE_PARAMS_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_POWERMESSAGEWAKE_PARAMS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for SP_POWERMESSAGEWAKE_PARAMS_W {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SP_POWERMESSAGEWAKE_PARAMS_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_POWERMESSAGEWAKE_PARAMS_W>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SP_POWERMESSAGEWAKE_PARAMS_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SP_POWERMESSAGEWAKE_PARAMS_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_POWERMESSAGEWAKE_PARAMS_W {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub PowerMessageWake: [u16; 512],
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_POWERMESSAGEWAKE_PARAMS_W {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_POWERMESSAGEWAKE_PARAMS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for SP_POWERMESSAGEWAKE_PARAMS_W {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for SP_POWERMESSAGEWAKE_PARAMS_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_POWERMESSAGEWAKE_PARAMS_W>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for SP_POWERMESSAGEWAKE_PARAMS_W {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for SP_POWERMESSAGEWAKE_PARAMS_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_PROPCHANGE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub StateChange: u32,
    pub Scope: u32,
    pub HwProfile: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_PROPCHANGE_PARAMS {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_PROPCHANGE_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for SP_PROPCHANGE_PARAMS {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SP_PROPCHANGE_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_PROPCHANGE_PARAMS>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SP_PROPCHANGE_PARAMS {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SP_PROPCHANGE_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_PROPCHANGE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub StateChange: u32,
    pub Scope: u32,
    pub HwProfile: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_PROPCHANGE_PARAMS {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_PROPCHANGE_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for SP_PROPCHANGE_PARAMS {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for SP_PROPCHANGE_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_PROPCHANGE_PARAMS>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for SP_PROPCHANGE_PARAMS {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for SP_PROPCHANGE_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_PROPSHEETPAGE_REQUEST {
    pub cbSize: u32,
    pub PageRequested: u32,
    pub DeviceInfoSet: *mut ::core::ffi::c_void,
    pub DeviceInfoData: *mut SP_DEVINFO_DATA,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_PROPSHEETPAGE_REQUEST {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_PROPSHEETPAGE_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for SP_PROPSHEETPAGE_REQUEST {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SP_PROPSHEETPAGE_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_PROPSHEETPAGE_REQUEST>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SP_PROPSHEETPAGE_REQUEST {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SP_PROPSHEETPAGE_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_PROPSHEETPAGE_REQUEST {
    pub cbSize: u32,
    pub PageRequested: u32,
    pub DeviceInfoSet: *mut ::core::ffi::c_void,
    pub DeviceInfoData: *mut SP_DEVINFO_DATA,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_PROPSHEETPAGE_REQUEST {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_PROPSHEETPAGE_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for SP_PROPSHEETPAGE_REQUEST {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for SP_PROPSHEETPAGE_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_PROPSHEETPAGE_REQUEST>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for SP_PROPSHEETPAGE_REQUEST {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for SP_PROPSHEETPAGE_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_REGISTER_CONTROL_STATUSA {
    pub cbSize: u32,
    pub FileName: ::windows::core::PCSTR,
    pub Win32Error: u32,
    pub FailureCode: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_REGISTER_CONTROL_STATUSA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_REGISTER_CONTROL_STATUSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for SP_REGISTER_CONTROL_STATUSA {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SP_REGISTER_CONTROL_STATUSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_REGISTER_CONTROL_STATUSA>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SP_REGISTER_CONTROL_STATUSA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SP_REGISTER_CONTROL_STATUSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_REGISTER_CONTROL_STATUSA {
    pub cbSize: u32,
    pub FileName: ::windows::core::PCSTR,
    pub Win32Error: u32,
    pub FailureCode: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_REGISTER_CONTROL_STATUSA {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_REGISTER_CONTROL_STATUSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for SP_REGISTER_CONTROL_STATUSA {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for SP_REGISTER_CONTROL_STATUSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_REGISTER_CONTROL_STATUSA>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for SP_REGISTER_CONTROL_STATUSA {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for SP_REGISTER_CONTROL_STATUSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_REGISTER_CONTROL_STATUSW {
    pub cbSize: u32,
    pub FileName: ::windows::core::PCWSTR,
    pub Win32Error: u32,
    pub FailureCode: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_REGISTER_CONTROL_STATUSW {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_REGISTER_CONTROL_STATUSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for SP_REGISTER_CONTROL_STATUSW {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SP_REGISTER_CONTROL_STATUSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_REGISTER_CONTROL_STATUSW>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SP_REGISTER_CONTROL_STATUSW {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SP_REGISTER_CONTROL_STATUSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_REGISTER_CONTROL_STATUSW {
    pub cbSize: u32,
    pub FileName: ::windows::core::PCWSTR,
    pub Win32Error: u32,
    pub FailureCode: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_REGISTER_CONTROL_STATUSW {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_REGISTER_CONTROL_STATUSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for SP_REGISTER_CONTROL_STATUSW {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for SP_REGISTER_CONTROL_STATUSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_REGISTER_CONTROL_STATUSW>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for SP_REGISTER_CONTROL_STATUSW {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for SP_REGISTER_CONTROL_STATUSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_REMOVEDEVICE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Scope: u32,
    pub HwProfile: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_REMOVEDEVICE_PARAMS {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_REMOVEDEVICE_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for SP_REMOVEDEVICE_PARAMS {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SP_REMOVEDEVICE_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_REMOVEDEVICE_PARAMS>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SP_REMOVEDEVICE_PARAMS {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SP_REMOVEDEVICE_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_REMOVEDEVICE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Scope: u32,
    pub HwProfile: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_REMOVEDEVICE_PARAMS {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_REMOVEDEVICE_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for SP_REMOVEDEVICE_PARAMS {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for SP_REMOVEDEVICE_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_REMOVEDEVICE_PARAMS>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for SP_REMOVEDEVICE_PARAMS {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for SP_REMOVEDEVICE_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_SELECTDEVICE_PARAMS_A {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Title: [super::super::Foundation::CHAR; 60],
    pub Instructions: [super::super::Foundation::CHAR; 256],
    pub ListLabel: [super::super::Foundation::CHAR; 30],
    pub SubTitle: [super::super::Foundation::CHAR; 256],
    pub Reserved: [u8; 2],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_SELECTDEVICE_PARAMS_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_SELECTDEVICE_PARAMS_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SP_SELECTDEVICE_PARAMS_A {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SP_SELECTDEVICE_PARAMS_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_SELECTDEVICE_PARAMS_A>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SP_SELECTDEVICE_PARAMS_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SP_SELECTDEVICE_PARAMS_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_SELECTDEVICE_PARAMS_W {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Title: [u16; 60],
    pub Instructions: [u16; 256],
    pub ListLabel: [u16; 30],
    pub SubTitle: [u16; 256],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_SELECTDEVICE_PARAMS_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_SELECTDEVICE_PARAMS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for SP_SELECTDEVICE_PARAMS_W {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SP_SELECTDEVICE_PARAMS_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_SELECTDEVICE_PARAMS_W>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SP_SELECTDEVICE_PARAMS_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SP_SELECTDEVICE_PARAMS_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_SELECTDEVICE_PARAMS_W {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Title: [u16; 60],
    pub Instructions: [u16; 256],
    pub ListLabel: [u16; 30],
    pub SubTitle: [u16; 256],
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_SELECTDEVICE_PARAMS_W {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_SELECTDEVICE_PARAMS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for SP_SELECTDEVICE_PARAMS_W {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for SP_SELECTDEVICE_PARAMS_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_SELECTDEVICE_PARAMS_W>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for SP_SELECTDEVICE_PARAMS_W {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for SP_SELECTDEVICE_PARAMS_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_TROUBLESHOOTER_PARAMS_A {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub ChmFile: [super::super::Foundation::CHAR; 260],
    pub HtmlTroubleShooter: [super::super::Foundation::CHAR; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_TROUBLESHOOTER_PARAMS_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_TROUBLESHOOTER_PARAMS_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SP_TROUBLESHOOTER_PARAMS_A {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SP_TROUBLESHOOTER_PARAMS_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_TROUBLESHOOTER_PARAMS_A>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SP_TROUBLESHOOTER_PARAMS_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SP_TROUBLESHOOTER_PARAMS_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_TROUBLESHOOTER_PARAMS_W {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub ChmFile: [u16; 260],
    pub HtmlTroubleShooter: [u16; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_TROUBLESHOOTER_PARAMS_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_TROUBLESHOOTER_PARAMS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for SP_TROUBLESHOOTER_PARAMS_W {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SP_TROUBLESHOOTER_PARAMS_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_TROUBLESHOOTER_PARAMS_W>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SP_TROUBLESHOOTER_PARAMS_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SP_TROUBLESHOOTER_PARAMS_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_TROUBLESHOOTER_PARAMS_W {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub ChmFile: [u16; 260],
    pub HtmlTroubleShooter: [u16; 260],
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_TROUBLESHOOTER_PARAMS_W {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_TROUBLESHOOTER_PARAMS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for SP_TROUBLESHOOTER_PARAMS_W {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for SP_TROUBLESHOOTER_PARAMS_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_TROUBLESHOOTER_PARAMS_W>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for SP_TROUBLESHOOTER_PARAMS_W {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for SP_TROUBLESHOOTER_PARAMS_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_UNREMOVEDEVICE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Scope: u32,
    pub HwProfile: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_UNREMOVEDEVICE_PARAMS {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_UNREMOVEDEVICE_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for SP_UNREMOVEDEVICE_PARAMS {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SP_UNREMOVEDEVICE_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_UNREMOVEDEVICE_PARAMS>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SP_UNREMOVEDEVICE_PARAMS {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SP_UNREMOVEDEVICE_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_UNREMOVEDEVICE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Scope: u32,
    pub HwProfile: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_UNREMOVEDEVICE_PARAMS {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_UNREMOVEDEVICE_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for SP_UNREMOVEDEVICE_PARAMS {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for SP_UNREMOVEDEVICE_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SP_UNREMOVEDEVICE_PARAMS>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for SP_UNREMOVEDEVICE_PARAMS {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for SP_UNREMOVEDEVICE_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SRCINFO_DESCRIPTION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SRCINFO_FLAGS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SRCINFO_PATH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SRCINFO_TAGFILE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SRCINFO_TAGFILE2: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SRCLIST_APPEND: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SRCLIST_NOBROWSE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SRCLIST_NOSTRIPPLATFORM: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SRCLIST_SUBDIRS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SRCLIST_SYSIFADMIN: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SRCLIST_SYSTEM: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SRCLIST_TEMPORARY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SRCLIST_USER: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SRC_FLAGS_CABFILE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SUOI_FORCEDELETE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SUOI_INTERNAL1: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_ADDAUTOLOGGER: &'static str = "AddAutoLogger";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_ADDAUTOLOGGERPROVIDER: &'static str = "AddAutoLoggerProvider";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_ADDCHANNEL: &'static str = "AddChannel";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_ADDEVENTPROVIDER: &'static str = "AddEventProvider";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_ADDFILTER: &'static str = "AddFilter";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_ADDIME: &'static str = "AddIme";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_ADDINTERFACE: &'static str = "AddInterface";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_ADDPOWERSETTING: &'static str = "AddPowerSetting";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_ADDPROP: &'static str = "AddProperty";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_ADDREG: &'static str = "AddReg";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_ADDREGNOCLOBBER: &'static str = "AddRegNoClobber";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_ADDSERVICE: &'static str = "AddService";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_ADDTRIGGER: &'static str = "AddTrigger";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_BITREG: &'static str = "BitReg";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_CLEANONLY: &'static str = "CleanOnly";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_COPYFILES: &'static str = "CopyFiles";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_COPYINF: &'static str = "CopyINF";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_DEFAULTOPTION: &'static str = "DefaultOption";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_DEFDESTDIR: &'static str = "DefaultDestDir";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_DELFILES: &'static str = "DelFiles";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_DELIME: &'static str = "DelIme";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_DELPROP: &'static str = "DelProperty";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_DELREG: &'static str = "DelReg";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_DELSERVICE: &'static str = "DelService";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_DESTDIRS: &'static str = "DestinationDirs";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_EXCLUDEID: &'static str = "ExcludeId";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_FEATURESCORE: &'static str = "FeatureScore";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_FILTERLEVEL: &'static str = "FilterLevel";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_FILTERPOSITION: &'static str = "FilterPosition";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_HARDWARE: &'static str = "Hardware";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_IMPORTCHANNEL: &'static str = "ImportChannel";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_INI2REG: &'static str = "Ini2Reg";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_LAYOUT_FILE: &'static str = "LayoutFile";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_LDIDOEM: &'static str = "LdidOEM";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_LFN_SECTION: &'static str = "VarLDID.LFN";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_LISTOPTIONS: &'static str = "ListOptions";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_LOGCONFIG: &'static str = "LogConfig";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_MODULES: &'static str = "Modules";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_OPTIONDESC: &'static str = "OptionDesc";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_PHASE1: &'static str = "Phase1";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_PROFILEITEMS: &'static str = "ProfileItems";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_REGSVR: &'static str = "RegisterDlls";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_RENFILES: &'static str = "RenFiles";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_SFN_SECTION: &'static str = "VarLDID.SFN";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_SRCDISKFILES: &'static str = "SourceDisksFiles";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_SRCDISKNAMES: &'static str = "SourceDisksNames";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_STRINGS: &'static str = "Strings";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_UNREGSVR: &'static str = "UnregisterDlls";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_UPDATEAUTOLOGGER: &'static str = "UpdateAutoLogger";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_UPDATEINIFIELDS: &'static str = "UpdateIniFields";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_UPDATEINIS: &'static str = "UpdateInis";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_UPGRADEONLY: &'static str = "UpgradeOnly";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupAddInstallSectionToDiskSpaceListA<'a, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, layoutinfhandle: *const ::core::ffi::c_void, sectionname: Param3, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupAddInstallSectionToDiskSpaceListA(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, layoutinfhandle: *const ::core::ffi::c_void, sectionname: ::windows::core::PCSTR, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupAddInstallSectionToDiskSpaceListA(::core::mem::transmute(diskspace), ::core::mem::transmute(infhandle), ::core::mem::transmute(layoutinfhandle), sectionname.into_param().abi(), ::core::mem::transmute(reserved1), ::core::mem::transmute(reserved2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupAddInstallSectionToDiskSpaceListW<'a, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, layoutinfhandle: *const ::core::ffi::c_void, sectionname: Param3, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupAddInstallSectionToDiskSpaceListW(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, layoutinfhandle: *const ::core::ffi::c_void, sectionname: ::windows::core::PCWSTR, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupAddInstallSectionToDiskSpaceListW(::core::mem::transmute(diskspace), ::core::mem::transmute(infhandle), ::core::mem::transmute(layoutinfhandle), sectionname.into_param().abi(), ::core::mem::transmute(reserved1), ::core::mem::transmute(reserved2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupAddSectionToDiskSpaceListA<'a, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, sectionname: Param3, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupAddSectionToDiskSpaceListA(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, sectionname: ::windows::core::PCSTR, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupAddSectionToDiskSpaceListA(::core::mem::transmute(diskspace), ::core::mem::transmute(infhandle), ::core::mem::transmute(listinfhandle), sectionname.into_param().abi(), ::core::mem::transmute(operation), ::core::mem::transmute(reserved1), ::core::mem::transmute(reserved2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupAddSectionToDiskSpaceListW<'a, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, sectionname: Param3, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupAddSectionToDiskSpaceListW(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, sectionname: ::windows::core::PCWSTR, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupAddSectionToDiskSpaceListW(::core::mem::transmute(diskspace), ::core::mem::transmute(infhandle), ::core::mem::transmute(listinfhandle), sectionname.into_param().abi(), ::core::mem::transmute(operation), ::core::mem::transmute(reserved1), ::core::mem::transmute(reserved2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupAddToDiskSpaceListA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(diskspace: *const ::core::ffi::c_void, targetfilespec: Param1, filesize: i64, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupAddToDiskSpaceListA(diskspace: *const ::core::ffi::c_void, targetfilespec: ::windows::core::PCSTR, filesize: i64, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupAddToDiskSpaceListA(::core::mem::transmute(diskspace), targetfilespec.into_param().abi(), ::core::mem::transmute(filesize), ::core::mem::transmute(operation), ::core::mem::transmute(reserved1), ::core::mem::transmute(reserved2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupAddToDiskSpaceListW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(diskspace: *const ::core::ffi::c_void, targetfilespec: Param1, filesize: i64, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupAddToDiskSpaceListW(diskspace: *const ::core::ffi::c_void, targetfilespec: ::windows::core::PCWSTR, filesize: i64, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupAddToDiskSpaceListW(::core::mem::transmute(diskspace), targetfilespec.into_param().abi(), ::core::mem::transmute(filesize), ::core::mem::transmute(operation), ::core::mem::transmute(reserved1), ::core::mem::transmute(reserved2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupAddToSourceListA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(flags: u32, source: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupAddToSourceListA(flags: u32, source: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupAddToSourceListA(::core::mem::transmute(flags), source.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupAddToSourceListW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(flags: u32, source: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupAddToSourceListW(flags: u32, source: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupAddToSourceListW(::core::mem::transmute(flags), source.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupAdjustDiskSpaceListA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(diskspace: *const ::core::ffi::c_void, driveroot: Param1, amount: i64, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupAdjustDiskSpaceListA(diskspace: *const ::core::ffi::c_void, driveroot: ::windows::core::PCSTR, amount: i64, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupAdjustDiskSpaceListA(::core::mem::transmute(diskspace), driveroot.into_param().abi(), ::core::mem::transmute(amount), ::core::mem::transmute(reserved1), ::core::mem::transmute(reserved2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupAdjustDiskSpaceListW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(diskspace: *const ::core::ffi::c_void, driveroot: Param1, amount: i64, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupAdjustDiskSpaceListW(diskspace: *const ::core::ffi::c_void, driveroot: ::windows::core::PCWSTR, amount: i64, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupAdjustDiskSpaceListW(::core::mem::transmute(diskspace), driveroot.into_param().abi(), ::core::mem::transmute(amount), ::core::mem::transmute(reserved1), ::core::mem::transmute(reserved2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupBackupErrorA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hwndparent: Param0, dialogtitle: Param1, sourcefile: Param2, targetfile: Param3, win32errorcode: u32, style: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupBackupErrorA(hwndparent: super::super::Foundation::HWND, dialogtitle: ::windows::core::PCSTR, sourcefile: ::windows::core::PCSTR, targetfile: ::windows::core::PCSTR, win32errorcode: u32, style: u32) -> u32;
        }
        ::core::mem::transmute(SetupBackupErrorA(hwndparent.into_param().abi(), dialogtitle.into_param().abi(), sourcefile.into_param().abi(), targetfile.into_param().abi(), ::core::mem::transmute(win32errorcode), ::core::mem::transmute(style)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupBackupErrorW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hwndparent: Param0, dialogtitle: Param1, sourcefile: Param2, targetfile: Param3, win32errorcode: u32, style: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupBackupErrorW(hwndparent: super::super::Foundation::HWND, dialogtitle: ::windows::core::PCWSTR, sourcefile: ::windows::core::PCWSTR, targetfile: ::windows::core::PCWSTR, win32errorcode: u32, style: u32) -> u32;
        }
        ::core::mem::transmute(SetupBackupErrorW(hwndparent.into_param().abi(), dialogtitle.into_param().abi(), sourcefile.into_param().abi(), targetfile.into_param().abi(), ::core::mem::transmute(win32errorcode), ::core::mem::transmute(style)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupCancelTemporarySourceList() -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupCancelTemporarySourceList() -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupCancelTemporarySourceList())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupCloseFileQueue(queuehandle: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupCloseFileQueue(queuehandle: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupCloseFileQueue(::core::mem::transmute(queuehandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn SetupCloseInfFile(infhandle: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupCloseInfFile(infhandle: *const ::core::ffi::c_void);
        }
        SetupCloseInfFile(::core::mem::transmute(infhandle))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn SetupCloseLog() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupCloseLog();
        }
        SetupCloseLog()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupCommitFileQueueA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(owner: Param0, queuehandle: *const ::core::ffi::c_void, msghandler: PSP_FILE_CALLBACK_A, context: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupCommitFileQueueA(owner: super::super::Foundation::HWND, queuehandle: *const ::core::ffi::c_void, msghandler: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupCommitFileQueueA(owner.into_param().abi(), ::core::mem::transmute(queuehandle), ::core::mem::transmute(msghandler), ::core::mem::transmute(context)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupCommitFileQueueW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(owner: Param0, queuehandle: *const ::core::ffi::c_void, msghandler: PSP_FILE_CALLBACK_W, context: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupCommitFileQueueW(owner: super::super::Foundation::HWND, queuehandle: *const ::core::ffi::c_void, msghandler: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupCommitFileQueueW(owner.into_param().abi(), ::core::mem::transmute(queuehandle), ::core::mem::transmute(msghandler), ::core::mem::transmute(context)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupConfigureWmiFromInfSectionA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(infhandle: *const ::core::ffi::c_void, sectionname: Param1, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupConfigureWmiFromInfSectionA(infhandle: *const ::core::ffi::c_void, sectionname: ::windows::core::PCSTR, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupConfigureWmiFromInfSectionA(::core::mem::transmute(infhandle), sectionname.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupConfigureWmiFromInfSectionW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(infhandle: *const ::core::ffi::c_void, sectionname: Param1, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupConfigureWmiFromInfSectionW(infhandle: *const ::core::ffi::c_void, sectionname: ::windows::core::PCWSTR, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupConfigureWmiFromInfSectionW(::core::mem::transmute(infhandle), sectionname.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupCopyErrorA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hwndparent: Param0, dialogtitle: Param1, diskname: Param2, pathtosource: Param3, sourcefile: Param4, targetpathfile: Param5, win32errorcode: u32, style: u32, pathbuffer: &mut [u8], pathrequiredsize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupCopyErrorA(hwndparent: super::super::Foundation::HWND, dialogtitle: ::windows::core::PCSTR, diskname: ::windows::core::PCSTR, pathtosource: ::windows::core::PCSTR, sourcefile: ::windows::core::PCSTR, targetpathfile: ::windows::core::PCSTR, win32errorcode: u32, style: u32, pathbuffer: ::windows::core::PSTR, pathbuffersize: u32, pathrequiredsize: *mut u32) -> u32;
        }
        ::core::mem::transmute(SetupCopyErrorA(hwndparent.into_param().abi(), dialogtitle.into_param().abi(), diskname.into_param().abi(), pathtosource.into_param().abi(), sourcefile.into_param().abi(), targetpathfile.into_param().abi(), ::core::mem::transmute(win32errorcode), ::core::mem::transmute(style), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pathbuffer)), pathbuffer.len() as _, ::core::mem::transmute(pathrequiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupCopyErrorW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hwndparent: Param0, dialogtitle: Param1, diskname: Param2, pathtosource: Param3, sourcefile: Param4, targetpathfile: Param5, win32errorcode: u32, style: u32, pathbuffer: &mut [u16], pathrequiredsize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupCopyErrorW(hwndparent: super::super::Foundation::HWND, dialogtitle: ::windows::core::PCWSTR, diskname: ::windows::core::PCWSTR, pathtosource: ::windows::core::PCWSTR, sourcefile: ::windows::core::PCWSTR, targetpathfile: ::windows::core::PCWSTR, win32errorcode: u32, style: u32, pathbuffer: ::windows::core::PWSTR, pathbuffersize: u32, pathrequiredsize: *mut u32) -> u32;
        }
        ::core::mem::transmute(SetupCopyErrorW(hwndparent.into_param().abi(), dialogtitle.into_param().abi(), diskname.into_param().abi(), pathtosource.into_param().abi(), sourcefile.into_param().abi(), targetpathfile.into_param().abi(), ::core::mem::transmute(win32errorcode), ::core::mem::transmute(style), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pathbuffer)), pathbuffer.len() as _, ::core::mem::transmute(pathrequiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupCopyOEMInfA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(sourceinffilename: Param0, oemsourcemedialocation: Param1, oemsourcemediatype: OEM_SOURCE_MEDIA_TYPE, copystyle: u32, destinationinffilename: &mut [u8], requiredsize: *mut u32, destinationinffilenamecomponent: *mut ::windows::core::PSTR) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupCopyOEMInfA(sourceinffilename: ::windows::core::PCSTR, oemsourcemedialocation: ::windows::core::PCSTR, oemsourcemediatype: OEM_SOURCE_MEDIA_TYPE, copystyle: u32, destinationinffilename: ::windows::core::PSTR, destinationinffilenamesize: u32, requiredsize: *mut u32, destinationinffilenamecomponent: *mut ::windows::core::PSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupCopyOEMInfA(sourceinffilename.into_param().abi(), oemsourcemedialocation.into_param().abi(), ::core::mem::transmute(oemsourcemediatype), ::core::mem::transmute(copystyle), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(destinationinffilename)), destinationinffilename.len() as _, ::core::mem::transmute(requiredsize), ::core::mem::transmute(destinationinffilenamecomponent)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupCopyOEMInfW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(sourceinffilename: Param0, oemsourcemedialocation: Param1, oemsourcemediatype: OEM_SOURCE_MEDIA_TYPE, copystyle: u32, destinationinffilename: &mut [u16], requiredsize: *mut u32, destinationinffilenamecomponent: *mut ::windows::core::PWSTR) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupCopyOEMInfW(sourceinffilename: ::windows::core::PCWSTR, oemsourcemedialocation: ::windows::core::PCWSTR, oemsourcemediatype: OEM_SOURCE_MEDIA_TYPE, copystyle: u32, destinationinffilename: ::windows::core::PWSTR, destinationinffilenamesize: u32, requiredsize: *mut u32, destinationinffilenamecomponent: *mut ::windows::core::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupCopyOEMInfW(sourceinffilename.into_param().abi(), oemsourcemedialocation.into_param().abi(), ::core::mem::transmute(oemsourcemediatype), ::core::mem::transmute(copystyle), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(destinationinffilename)), destinationinffilename.len() as _, ::core::mem::transmute(requiredsize), ::core::mem::transmute(destinationinffilenamecomponent)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn SetupCreateDiskSpaceListA(reserved1: *mut ::core::ffi::c_void, reserved2: u32, flags: u32) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupCreateDiskSpaceListA(reserved1: *mut ::core::ffi::c_void, reserved2: u32, flags: u32) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(SetupCreateDiskSpaceListA(::core::mem::transmute(reserved1), ::core::mem::transmute(reserved2), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn SetupCreateDiskSpaceListW(reserved1: *mut ::core::ffi::c_void, reserved2: u32, flags: u32) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupCreateDiskSpaceListW(reserved1: *mut ::core::ffi::c_void, reserved2: u32, flags: u32) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(SetupCreateDiskSpaceListW(::core::mem::transmute(reserved1), ::core::mem::transmute(reserved2), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn SetupDecompressOrCopyFileA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(sourcefilename: Param0, targetfilename: Param1, compressiontype: *const u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDecompressOrCopyFileA(sourcefilename: ::windows::core::PCSTR, targetfilename: ::windows::core::PCSTR, compressiontype: *const u32) -> u32;
        }
        ::core::mem::transmute(SetupDecompressOrCopyFileA(sourcefilename.into_param().abi(), targetfilename.into_param().abi(), ::core::mem::transmute(compressiontype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn SetupDecompressOrCopyFileW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(sourcefilename: Param0, targetfilename: Param1, compressiontype: *const u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDecompressOrCopyFileW(sourcefilename: ::windows::core::PCWSTR, targetfilename: ::windows::core::PCWSTR, compressiontype: *const u32) -> u32;
        }
        ::core::mem::transmute(SetupDecompressOrCopyFileW(sourcefilename.into_param().abi(), targetfilename.into_param().abi(), ::core::mem::transmute(compressiontype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn SetupDefaultQueueCallbackA(context: *const ::core::ffi::c_void, notification: u32, param1: usize, param2: usize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDefaultQueueCallbackA(context: *const ::core::ffi::c_void, notification: u32, param1: usize, param2: usize) -> u32;
        }
        ::core::mem::transmute(SetupDefaultQueueCallbackA(::core::mem::transmute(context), ::core::mem::transmute(notification), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn SetupDefaultQueueCallbackW(context: *const ::core::ffi::c_void, notification: u32, param1: usize, param2: usize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDefaultQueueCallbackW(context: *const ::core::ffi::c_void, notification: u32, param1: usize, param2: usize) -> u32;
        }
        ::core::mem::transmute(SetupDefaultQueueCallbackW(::core::mem::transmute(context), ::core::mem::transmute(notification), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDeleteErrorA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hwndparent: Param0, dialogtitle: Param1, file: Param2, win32errorcode: u32, style: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDeleteErrorA(hwndparent: super::super::Foundation::HWND, dialogtitle: ::windows::core::PCSTR, file: ::windows::core::PCSTR, win32errorcode: u32, style: u32) -> u32;
        }
        ::core::mem::transmute(SetupDeleteErrorA(hwndparent.into_param().abi(), dialogtitle.into_param().abi(), file.into_param().abi(), ::core::mem::transmute(win32errorcode), ::core::mem::transmute(style)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDeleteErrorW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hwndparent: Param0, dialogtitle: Param1, file: Param2, win32errorcode: u32, style: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDeleteErrorW(hwndparent: super::super::Foundation::HWND, dialogtitle: ::windows::core::PCWSTR, file: ::windows::core::PCWSTR, win32errorcode: u32, style: u32) -> u32;
        }
        ::core::mem::transmute(SetupDeleteErrorW(hwndparent.into_param().abi(), dialogtitle.into_param().abi(), file.into_param().abi(), ::core::mem::transmute(win32errorcode), ::core::mem::transmute(style)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDestroyDiskSpaceList(diskspace: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDestroyDiskSpaceList(diskspace: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDestroyDiskSpaceList(::core::mem::transmute(diskspace)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiAskForOEMDisk(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiAskForOEMDisk(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiAskForOEMDisk(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiBuildClassInfoList(flags: u32, classguidlist: &mut [::windows::core::GUID], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiBuildClassInfoList(flags: u32, classguidlist: *mut ::windows::core::GUID, classguidlistsize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiBuildClassInfoList(::core::mem::transmute(flags), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(classguidlist)), classguidlist.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiBuildClassInfoListExA<'a, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(flags: u32, classguidlist: &mut [::windows::core::GUID], requiredsize: *mut u32, machinename: Param4, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiBuildClassInfoListExA(flags: u32, classguidlist: *mut ::windows::core::GUID, classguidlistsize: u32, requiredsize: *mut u32, machinename: ::windows::core::PCSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiBuildClassInfoListExA(::core::mem::transmute(flags), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(classguidlist)), classguidlist.len() as _, ::core::mem::transmute(requiredsize), machinename.into_param().abi(), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiBuildClassInfoListExW<'a, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(flags: u32, classguidlist: &mut [::windows::core::GUID], requiredsize: *mut u32, machinename: Param4, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiBuildClassInfoListExW(flags: u32, classguidlist: *mut ::windows::core::GUID, classguidlistsize: u32, requiredsize: *mut u32, machinename: ::windows::core::PCWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiBuildClassInfoListExW(::core::mem::transmute(flags), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(classguidlist)), classguidlist.len() as _, ::core::mem::transmute(requiredsize), machinename.into_param().abi(), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiBuildDriverInfoList(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA, drivertype: SETUP_DI_BUILD_DRIVER_DRIVER_TYPE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiBuildDriverInfoList(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA, drivertype: SETUP_DI_BUILD_DRIVER_DRIVER_TYPE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiBuildDriverInfoList(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(drivertype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiCallClassInstaller(installfunction: u32, deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiCallClassInstaller(installfunction: u32, deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiCallClassInstaller(::core::mem::transmute(installfunction), ::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiCancelDriverInfoSearch(deviceinfoset: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiCancelDriverInfoSearch(deviceinfoset: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiCancelDriverInfoSearch(::core::mem::transmute(deviceinfoset)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiChangeState(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiChangeState(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiChangeState(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiClassGuidsFromNameA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(classname: Param0, classguidlist: &mut [::windows::core::GUID], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiClassGuidsFromNameA(classname: ::windows::core::PCSTR, classguidlist: *mut ::windows::core::GUID, classguidlistsize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiClassGuidsFromNameA(classname.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(classguidlist)), classguidlist.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiClassGuidsFromNameExA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(classname: Param0, classguidlist: &mut [::windows::core::GUID], requiredsize: *mut u32, machinename: Param4, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiClassGuidsFromNameExA(classname: ::windows::core::PCSTR, classguidlist: *mut ::windows::core::GUID, classguidlistsize: u32, requiredsize: *mut u32, machinename: ::windows::core::PCSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiClassGuidsFromNameExA(classname.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(classguidlist)), classguidlist.len() as _, ::core::mem::transmute(requiredsize), machinename.into_param().abi(), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiClassGuidsFromNameExW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(classname: Param0, classguidlist: &mut [::windows::core::GUID], requiredsize: *mut u32, machinename: Param4, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiClassGuidsFromNameExW(classname: ::windows::core::PCWSTR, classguidlist: *mut ::windows::core::GUID, classguidlistsize: u32, requiredsize: *mut u32, machinename: ::windows::core::PCWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiClassGuidsFromNameExW(classname.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(classguidlist)), classguidlist.len() as _, ::core::mem::transmute(requiredsize), machinename.into_param().abi(), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiClassGuidsFromNameW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(classname: Param0, classguidlist: &mut [::windows::core::GUID], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiClassGuidsFromNameW(classname: ::windows::core::PCWSTR, classguidlist: *mut ::windows::core::GUID, classguidlistsize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiClassGuidsFromNameW(classname.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(classguidlist)), classguidlist.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiClassNameFromGuidA(classguid: *const ::windows::core::GUID, classname: &mut [u8], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiClassNameFromGuidA(classguid: *const ::windows::core::GUID, classname: ::windows::core::PSTR, classnamesize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiClassNameFromGuidA(::core::mem::transmute(classguid), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(classname)), classname.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiClassNameFromGuidExA<'a, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(classguid: *const ::windows::core::GUID, classname: &mut [u8], requiredsize: *mut u32, machinename: Param4, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiClassNameFromGuidExA(classguid: *const ::windows::core::GUID, classname: ::windows::core::PSTR, classnamesize: u32, requiredsize: *mut u32, machinename: ::windows::core::PCSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiClassNameFromGuidExA(::core::mem::transmute(classguid), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(classname)), classname.len() as _, ::core::mem::transmute(requiredsize), machinename.into_param().abi(), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiClassNameFromGuidExW<'a, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(classguid: *const ::windows::core::GUID, classname: &mut [u16], requiredsize: *mut u32, machinename: Param4, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiClassNameFromGuidExW(classguid: *const ::windows::core::GUID, classname: ::windows::core::PWSTR, classnamesize: u32, requiredsize: *mut u32, machinename: ::windows::core::PCWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiClassNameFromGuidExW(::core::mem::transmute(classguid), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(classname)), classname.len() as _, ::core::mem::transmute(requiredsize), machinename.into_param().abi(), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiClassNameFromGuidW(classguid: *const ::windows::core::GUID, classname: &mut [u16], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiClassNameFromGuidW(classguid: *const ::windows::core::GUID, classname: ::windows::core::PWSTR, classnamesize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiClassNameFromGuidW(::core::mem::transmute(classguid), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(classname)), classname.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn SetupDiCreateDevRegKeyA<'a, Param6: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, scope: u32, hwprofile: u32, keytype: u32, infhandle: *const ::core::ffi::c_void, infsectionname: Param6) -> super::super::System::Registry::HKEY {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiCreateDevRegKeyA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, scope: u32, hwprofile: u32, keytype: u32, infhandle: *const ::core::ffi::c_void, infsectionname: ::windows::core::PCSTR) -> super::super::System::Registry::HKEY;
        }
        ::core::mem::transmute(SetupDiCreateDevRegKeyA(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(scope), ::core::mem::transmute(hwprofile), ::core::mem::transmute(keytype), ::core::mem::transmute(infhandle), infsectionname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn SetupDiCreateDevRegKeyW<'a, Param6: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, scope: u32, hwprofile: u32, keytype: u32, infhandle: *const ::core::ffi::c_void, infsectionname: Param6) -> super::super::System::Registry::HKEY {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiCreateDevRegKeyW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, scope: u32, hwprofile: u32, keytype: u32, infhandle: *const ::core::ffi::c_void, infsectionname: ::windows::core::PCWSTR) -> super::super::System::Registry::HKEY;
        }
        ::core::mem::transmute(SetupDiCreateDevRegKeyW(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(scope), ::core::mem::transmute(hwprofile), ::core::mem::transmute(keytype), ::core::mem::transmute(infhandle), infsectionname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiCreateDeviceInfoA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(deviceinfoset: *const ::core::ffi::c_void, devicename: Param1, classguid: *const ::windows::core::GUID, devicedescription: Param3, hwndparent: Param4, creationflags: u32, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiCreateDeviceInfoA(deviceinfoset: *const ::core::ffi::c_void, devicename: ::windows::core::PCSTR, classguid: *const ::windows::core::GUID, devicedescription: ::windows::core::PCSTR, hwndparent: super::super::Foundation::HWND, creationflags: u32, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiCreateDeviceInfoA(::core::mem::transmute(deviceinfoset), devicename.into_param().abi(), ::core::mem::transmute(classguid), devicedescription.into_param().abi(), hwndparent.into_param().abi(), ::core::mem::transmute(creationflags), ::core::mem::transmute(deviceinfodata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiCreateDeviceInfoList<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(classguid: *const ::windows::core::GUID, hwndparent: Param1) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiCreateDeviceInfoList(classguid: *const ::windows::core::GUID, hwndparent: super::super::Foundation::HWND) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(SetupDiCreateDeviceInfoList(::core::mem::transmute(classguid), hwndparent.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiCreateDeviceInfoListExA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(classguid: *const ::windows::core::GUID, hwndparent: Param1, machinename: Param2, reserved: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiCreateDeviceInfoListExA(classguid: *const ::windows::core::GUID, hwndparent: super::super::Foundation::HWND, machinename: ::windows::core::PCSTR, reserved: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(SetupDiCreateDeviceInfoListExA(::core::mem::transmute(classguid), hwndparent.into_param().abi(), machinename.into_param().abi(), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiCreateDeviceInfoListExW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(classguid: *const ::windows::core::GUID, hwndparent: Param1, machinename: Param2, reserved: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiCreateDeviceInfoListExW(classguid: *const ::windows::core::GUID, hwndparent: super::super::Foundation::HWND, machinename: ::windows::core::PCWSTR, reserved: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(SetupDiCreateDeviceInfoListExW(::core::mem::transmute(classguid), hwndparent.into_param().abi(), machinename.into_param().abi(), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiCreateDeviceInfoW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(deviceinfoset: *const ::core::ffi::c_void, devicename: Param1, classguid: *const ::windows::core::GUID, devicedescription: Param3, hwndparent: Param4, creationflags: u32, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiCreateDeviceInfoW(deviceinfoset: *const ::core::ffi::c_void, devicename: ::windows::core::PCWSTR, classguid: *const ::windows::core::GUID, devicedescription: ::windows::core::PCWSTR, hwndparent: super::super::Foundation::HWND, creationflags: u32, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiCreateDeviceInfoW(::core::mem::transmute(deviceinfoset), devicename.into_param().abi(), ::core::mem::transmute(classguid), devicedescription.into_param().abi(), hwndparent.into_param().abi(), ::core::mem::transmute(creationflags), ::core::mem::transmute(deviceinfodata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiCreateDeviceInterfaceA<'a, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, interfaceclassguid: *const ::windows::core::GUID, referencestring: Param3, creationflags: u32, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiCreateDeviceInterfaceA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, interfaceclassguid: *const ::windows::core::GUID, referencestring: ::windows::core::PCSTR, creationflags: u32, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiCreateDeviceInterfaceA(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(interfaceclassguid), referencestring.into_param().abi(), ::core::mem::transmute(creationflags), ::core::mem::transmute(deviceinterfacedata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn SetupDiCreateDeviceInterfaceRegKeyA<'a, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, reserved: u32, samdesired: u32, infhandle: *const ::core::ffi::c_void, infsectionname: Param5) -> super::super::System::Registry::HKEY {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiCreateDeviceInterfaceRegKeyA(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, reserved: u32, samdesired: u32, infhandle: *const ::core::ffi::c_void, infsectionname: ::windows::core::PCSTR) -> super::super::System::Registry::HKEY;
        }
        ::core::mem::transmute(SetupDiCreateDeviceInterfaceRegKeyA(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinterfacedata), ::core::mem::transmute(reserved), ::core::mem::transmute(samdesired), ::core::mem::transmute(infhandle), infsectionname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn SetupDiCreateDeviceInterfaceRegKeyW<'a, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, reserved: u32, samdesired: u32, infhandle: *const ::core::ffi::c_void, infsectionname: Param5) -> super::super::System::Registry::HKEY {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiCreateDeviceInterfaceRegKeyW(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, reserved: u32, samdesired: u32, infhandle: *const ::core::ffi::c_void, infsectionname: ::windows::core::PCWSTR) -> super::super::System::Registry::HKEY;
        }
        ::core::mem::transmute(SetupDiCreateDeviceInterfaceRegKeyW(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinterfacedata), ::core::mem::transmute(reserved), ::core::mem::transmute(samdesired), ::core::mem::transmute(infhandle), infsectionname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiCreateDeviceInterfaceW<'a, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, interfaceclassguid: *const ::windows::core::GUID, referencestring: Param3, creationflags: u32, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiCreateDeviceInterfaceW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, interfaceclassguid: *const ::windows::core::GUID, referencestring: ::windows::core::PCWSTR, creationflags: u32, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiCreateDeviceInterfaceW(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(interfaceclassguid), referencestring.into_param().abi(), ::core::mem::transmute(creationflags), ::core::mem::transmute(deviceinterfacedata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiDeleteDevRegKey(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, scope: u32, hwprofile: u32, keytype: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiDeleteDevRegKey(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, scope: u32, hwprofile: u32, keytype: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiDeleteDevRegKey(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(scope), ::core::mem::transmute(hwprofile), ::core::mem::transmute(keytype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiDeleteDeviceInfo(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiDeleteDeviceInfo(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiDeleteDeviceInfo(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiDeleteDeviceInterfaceData(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiDeleteDeviceInterfaceData(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiDeleteDeviceInterfaceData(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinterfacedata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiDeleteDeviceInterfaceRegKey(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, reserved: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiDeleteDeviceInterfaceRegKey(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, reserved: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiDeleteDeviceInterfaceRegKey(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinterfacedata), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
#[inline]
pub unsafe fn SetupDiDestroyClassImageList(classimagelistdata: *const SP_CLASSIMAGELIST_DATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiDestroyClassImageList(classimagelistdata: *const SP_CLASSIMAGELIST_DATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiDestroyClassImageList(::core::mem::transmute(classimagelistdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiDestroyDeviceInfoList(deviceinfoset: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiDestroyDeviceInfoList(deviceinfoset: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiDestroyDeviceInfoList(::core::mem::transmute(deviceinfoset)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiDestroyDriverInfoList(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, drivertype: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiDestroyDriverInfoList(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, drivertype: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiDestroyDriverInfoList(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(drivertype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn SetupDiDrawMiniIcon<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Gdi::HDC>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::RECT>>(hdc: Param0, rc: Param1, miniiconindex: i32, flags: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiDrawMiniIcon(hdc: super::super::Graphics::Gdi::HDC, rc: super::super::Foundation::RECT, miniiconindex: i32, flags: u32) -> i32;
        }
        ::core::mem::transmute(SetupDiDrawMiniIcon(hdc.into_param().abi(), rc.into_param().abi(), ::core::mem::transmute(miniiconindex), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiEnumDeviceInfo(deviceinfoset: *const ::core::ffi::c_void, memberindex: u32, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiEnumDeviceInfo(deviceinfoset: *const ::core::ffi::c_void, memberindex: u32, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiEnumDeviceInfo(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(memberindex), ::core::mem::transmute(deviceinfodata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiEnumDeviceInterfaces(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, interfaceclassguid: *const ::windows::core::GUID, memberindex: u32, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiEnumDeviceInterfaces(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, interfaceclassguid: *const ::windows::core::GUID, memberindex: u32, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiEnumDeviceInterfaces(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(interfaceclassguid), ::core::mem::transmute(memberindex), ::core::mem::transmute(deviceinterfacedata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiEnumDriverInfoA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, drivertype: u32, memberindex: u32, driverinfodata: *mut SP_DRVINFO_DATA_V2_A) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiEnumDriverInfoA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, drivertype: u32, memberindex: u32, driverinfodata: *mut SP_DRVINFO_DATA_V2_A) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiEnumDriverInfoA(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(drivertype), ::core::mem::transmute(memberindex), ::core::mem::transmute(driverinfodata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiEnumDriverInfoW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, drivertype: u32, memberindex: u32, driverinfodata: *mut SP_DRVINFO_DATA_V2_W) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiEnumDriverInfoW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, drivertype: u32, memberindex: u32, driverinfodata: *mut SP_DRVINFO_DATA_V2_W) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiEnumDriverInfoW(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(drivertype), ::core::mem::transmute(memberindex), ::core::mem::transmute(driverinfodata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
#[inline]
pub unsafe fn SetupDiGetActualModelsSectionA(context: *const INFCONTEXT, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, infsectionwithext: &mut [u8], requiredsize: *mut u32, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetActualModelsSectionA(context: *const INFCONTEXT, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, infsectionwithext: ::windows::core::PSTR, infsectionwithextsize: u32, requiredsize: *mut u32, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetActualModelsSectionA(::core::mem::transmute(context), ::core::mem::transmute(alternateplatforminfo), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(infsectionwithext)), infsectionwithext.len() as _, ::core::mem::transmute(requiredsize), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
#[inline]
pub unsafe fn SetupDiGetActualModelsSectionW(context: *const INFCONTEXT, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, infsectionwithext: &mut [u16], requiredsize: *mut u32, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetActualModelsSectionW(context: *const INFCONTEXT, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, infsectionwithext: ::windows::core::PWSTR, infsectionwithextsize: u32, requiredsize: *mut u32, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetActualModelsSectionW(::core::mem::transmute(context), ::core::mem::transmute(alternateplatforminfo), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(infsectionwithext)), infsectionwithext.len() as _, ::core::mem::transmute(requiredsize), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetActualSectionToInstallA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(infhandle: *const ::core::ffi::c_void, infsectionname: Param1, infsectionwithext: &mut [u8], requiredsize: *mut u32, extension: *mut ::windows::core::PSTR) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetActualSectionToInstallA(infhandle: *const ::core::ffi::c_void, infsectionname: ::windows::core::PCSTR, infsectionwithext: ::windows::core::PSTR, infsectionwithextsize: u32, requiredsize: *mut u32, extension: *mut ::windows::core::PSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetActualSectionToInstallA(::core::mem::transmute(infhandle), infsectionname.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(infsectionwithext)), infsectionwithext.len() as _, ::core::mem::transmute(requiredsize), ::core::mem::transmute(extension)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
#[inline]
pub unsafe fn SetupDiGetActualSectionToInstallExA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(infhandle: *const ::core::ffi::c_void, infsectionname: Param1, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, infsectionwithext: &mut [u8], requiredsize: *mut u32, extension: *mut ::windows::core::PSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetActualSectionToInstallExA(infhandle: *const ::core::ffi::c_void, infsectionname: ::windows::core::PCSTR, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, infsectionwithext: ::windows::core::PSTR, infsectionwithextsize: u32, requiredsize: *mut u32, extension: *mut ::windows::core::PSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetActualSectionToInstallExA(::core::mem::transmute(infhandle), infsectionname.into_param().abi(), ::core::mem::transmute(alternateplatforminfo), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(infsectionwithext)), infsectionwithext.len() as _, ::core::mem::transmute(requiredsize), ::core::mem::transmute(extension), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
#[inline]
pub unsafe fn SetupDiGetActualSectionToInstallExW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(infhandle: *const ::core::ffi::c_void, infsectionname: Param1, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, infsectionwithext: &mut [u16], requiredsize: *mut u32, extension: *mut ::windows::core::PWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetActualSectionToInstallExW(infhandle: *const ::core::ffi::c_void, infsectionname: ::windows::core::PCWSTR, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, infsectionwithext: ::windows::core::PWSTR, infsectionwithextsize: u32, requiredsize: *mut u32, extension: *mut ::windows::core::PWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetActualSectionToInstallExW(::core::mem::transmute(infhandle), infsectionname.into_param().abi(), ::core::mem::transmute(alternateplatforminfo), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(infsectionwithext)), infsectionwithext.len() as _, ::core::mem::transmute(requiredsize), ::core::mem::transmute(extension), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetActualSectionToInstallW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(infhandle: *const ::core::ffi::c_void, infsectionname: Param1, infsectionwithext: &mut [u16], requiredsize: *mut u32, extension: *mut ::windows::core::PWSTR) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetActualSectionToInstallW(infhandle: *const ::core::ffi::c_void, infsectionname: ::windows::core::PCWSTR, infsectionwithext: ::windows::core::PWSTR, infsectionwithextsize: u32, requiredsize: *mut u32, extension: *mut ::windows::core::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetActualSectionToInstallW(::core::mem::transmute(infhandle), infsectionname.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(infsectionwithext)), infsectionwithext.len() as _, ::core::mem::transmute(requiredsize), ::core::mem::transmute(extension)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetClassBitmapIndex(classguid: *const ::windows::core::GUID, miniiconindex: *mut i32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetClassBitmapIndex(classguid: *const ::windows::core::GUID, miniiconindex: *mut i32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetClassBitmapIndex(::core::mem::transmute(classguid), ::core::mem::transmute(miniiconindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetClassDescriptionA(classguid: *const ::windows::core::GUID, classdescription: &mut [u8], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetClassDescriptionA(classguid: *const ::windows::core::GUID, classdescription: ::windows::core::PSTR, classdescriptionsize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetClassDescriptionA(::core::mem::transmute(classguid), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(classdescription)), classdescription.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetClassDescriptionExA<'a, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(classguid: *const ::windows::core::GUID, classdescription: &mut [u8], requiredsize: *mut u32, machinename: Param4, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetClassDescriptionExA(classguid: *const ::windows::core::GUID, classdescription: ::windows::core::PSTR, classdescriptionsize: u32, requiredsize: *mut u32, machinename: ::windows::core::PCSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetClassDescriptionExA(::core::mem::transmute(classguid), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(classdescription)), classdescription.len() as _, ::core::mem::transmute(requiredsize), machinename.into_param().abi(), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetClassDescriptionExW<'a, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(classguid: *const ::windows::core::GUID, classdescription: &mut [u16], requiredsize: *mut u32, machinename: Param4, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetClassDescriptionExW(classguid: *const ::windows::core::GUID, classdescription: ::windows::core::PWSTR, classdescriptionsize: u32, requiredsize: *mut u32, machinename: ::windows::core::PCWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetClassDescriptionExW(::core::mem::transmute(classguid), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(classdescription)), classdescription.len() as _, ::core::mem::transmute(requiredsize), machinename.into_param().abi(), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetClassDescriptionW(classguid: *const ::windows::core::GUID, classdescription: &mut [u16], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetClassDescriptionW(classguid: *const ::windows::core::GUID, classdescription: ::windows::core::PWSTR, classdescriptionsize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetClassDescriptionW(::core::mem::transmute(classguid), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(classdescription)), classdescription.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn SetupDiGetClassDevPropertySheetsA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, propertysheetheader: *const super::super::UI::Controls::PROPSHEETHEADERA_V2, propertysheetheaderpagelistsize: u32, requiredsize: *mut u32, propertysheettype: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetClassDevPropertySheetsA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, propertysheetheader: *const super::super::UI::Controls::PROPSHEETHEADERA_V2, propertysheetheaderpagelistsize: u32, requiredsize: *mut u32, propertysheettype: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetClassDevPropertySheetsA(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(propertysheetheader), ::core::mem::transmute(propertysheetheaderpagelistsize), ::core::mem::transmute(requiredsize), ::core::mem::transmute(propertysheettype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn SetupDiGetClassDevPropertySheetsW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, propertysheetheader: *const super::super::UI::Controls::PROPSHEETHEADERW_V2, propertysheetheaderpagelistsize: u32, requiredsize: *mut u32, propertysheettype: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetClassDevPropertySheetsW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, propertysheetheader: *const super::super::UI::Controls::PROPSHEETHEADERW_V2, propertysheetheaderpagelistsize: u32, requiredsize: *mut u32, propertysheettype: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetClassDevPropertySheetsW(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(propertysheetheader), ::core::mem::transmute(propertysheetheaderpagelistsize), ::core::mem::transmute(requiredsize), ::core::mem::transmute(propertysheettype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetClassDevsA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(classguid: *const ::windows::core::GUID, enumerator: Param1, hwndparent: Param2, flags: u32) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetClassDevsA(classguid: *const ::windows::core::GUID, enumerator: ::windows::core::PCSTR, hwndparent: super::super::Foundation::HWND, flags: u32) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(SetupDiGetClassDevsA(::core::mem::transmute(classguid), enumerator.into_param().abi(), hwndparent.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetClassDevsExA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(classguid: *const ::windows::core::GUID, enumerator: Param1, hwndparent: Param2, flags: u32, deviceinfoset: *const ::core::ffi::c_void, machinename: Param5, reserved: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetClassDevsExA(classguid: *const ::windows::core::GUID, enumerator: ::windows::core::PCSTR, hwndparent: super::super::Foundation::HWND, flags: u32, deviceinfoset: *const ::core::ffi::c_void, machinename: ::windows::core::PCSTR, reserved: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(SetupDiGetClassDevsExA(::core::mem::transmute(classguid), enumerator.into_param().abi(), hwndparent.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(deviceinfoset), machinename.into_param().abi(), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetClassDevsExW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(classguid: *const ::windows::core::GUID, enumerator: Param1, hwndparent: Param2, flags: u32, deviceinfoset: *const ::core::ffi::c_void, machinename: Param5, reserved: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetClassDevsExW(classguid: *const ::windows::core::GUID, enumerator: ::windows::core::PCWSTR, hwndparent: super::super::Foundation::HWND, flags: u32, deviceinfoset: *const ::core::ffi::c_void, machinename: ::windows::core::PCWSTR, reserved: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(SetupDiGetClassDevsExW(::core::mem::transmute(classguid), enumerator.into_param().abi(), hwndparent.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(deviceinfoset), machinename.into_param().abi(), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetClassDevsW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(classguid: *const ::windows::core::GUID, enumerator: Param1, hwndparent: Param2, flags: u32) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetClassDevsW(classguid: *const ::windows::core::GUID, enumerator: ::windows::core::PCWSTR, hwndparent: super::super::Foundation::HWND, flags: u32) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(SetupDiGetClassDevsW(::core::mem::transmute(classguid), enumerator.into_param().abi(), hwndparent.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
#[inline]
pub unsafe fn SetupDiGetClassImageIndex(classimagelistdata: *const SP_CLASSIMAGELIST_DATA, classguid: *const ::windows::core::GUID, imageindex: *mut i32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetClassImageIndex(classimagelistdata: *const SP_CLASSIMAGELIST_DATA, classguid: *const ::windows::core::GUID, imageindex: *mut i32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetClassImageIndex(::core::mem::transmute(classimagelistdata), ::core::mem::transmute(classguid), ::core::mem::transmute(imageindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
#[inline]
pub unsafe fn SetupDiGetClassImageList(classimagelistdata: *mut SP_CLASSIMAGELIST_DATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetClassImageList(classimagelistdata: *mut SP_CLASSIMAGELIST_DATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetClassImageList(::core::mem::transmute(classimagelistdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
#[inline]
pub unsafe fn SetupDiGetClassImageListExA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(classimagelistdata: *mut SP_CLASSIMAGELIST_DATA, machinename: Param1, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetClassImageListExA(classimagelistdata: *mut SP_CLASSIMAGELIST_DATA, machinename: ::windows::core::PCSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetClassImageListExA(::core::mem::transmute(classimagelistdata), machinename.into_param().abi(), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
#[inline]
pub unsafe fn SetupDiGetClassImageListExW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(classimagelistdata: *mut SP_CLASSIMAGELIST_DATA, machinename: Param1, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetClassImageListExW(classimagelistdata: *mut SP_CLASSIMAGELIST_DATA, machinename: ::windows::core::PCWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetClassImageListExW(::core::mem::transmute(classimagelistdata), machinename.into_param().abi(), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetClassInstallParamsA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, classinstallparams: *mut SP_CLASSINSTALL_HEADER, classinstallparamssize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetClassInstallParamsA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, classinstallparams: *mut SP_CLASSINSTALL_HEADER, classinstallparamssize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetClassInstallParamsA(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(classinstallparams), ::core::mem::transmute(classinstallparamssize), ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetClassInstallParamsW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, classinstallparams: *mut SP_CLASSINSTALL_HEADER, classinstallparamssize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetClassInstallParamsW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, classinstallparams: *mut SP_CLASSINSTALL_HEADER, classinstallparamssize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetClassInstallParamsW(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(classinstallparams), ::core::mem::transmute(classinstallparamssize), ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`, `\"Win32_Foundation\"`*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
#[inline]
pub unsafe fn SetupDiGetClassPropertyExW<'a, Param7: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(classguid: *const ::windows::core::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32, flags: u32, machinename: Param7, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetClassPropertyExW(classguid: *const ::windows::core::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32, flags: u32, machinename: ::windows::core::PCWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetClassPropertyExW(::core::mem::transmute(classguid), ::core::mem::transmute(propertykey), ::core::mem::transmute(propertytype), ::core::mem::transmute(propertybuffer), ::core::mem::transmute(propertybuffersize), ::core::mem::transmute(requiredsize), ::core::mem::transmute(flags), machinename.into_param().abi(), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`, `\"Win32_Foundation\"`*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
#[inline]
pub unsafe fn SetupDiGetClassPropertyKeys(classguid: *const ::windows::core::GUID, propertykeyarray: &mut [super::Properties::DEVPROPKEY], requiredpropertykeycount: *mut u32, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetClassPropertyKeys(classguid: *const ::windows::core::GUID, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: u32, requiredpropertykeycount: *mut u32, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetClassPropertyKeys(::core::mem::transmute(classguid), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(propertykeyarray)), propertykeyarray.len() as _, ::core::mem::transmute(requiredpropertykeycount), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`, `\"Win32_Foundation\"`*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
#[inline]
pub unsafe fn SetupDiGetClassPropertyKeysExW<'a, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(classguid: *const ::windows::core::GUID, propertykeyarray: &mut [super::Properties::DEVPROPKEY], requiredpropertykeycount: *mut u32, flags: u32, machinename: Param5, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetClassPropertyKeysExW(classguid: *const ::windows::core::GUID, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: u32, requiredpropertykeycount: *mut u32, flags: u32, machinename: ::windows::core::PCWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetClassPropertyKeysExW(::core::mem::transmute(classguid), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(propertykeyarray)), propertykeyarray.len() as _, ::core::mem::transmute(requiredpropertykeycount), ::core::mem::transmute(flags), machinename.into_param().abi(), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`, `\"Win32_Foundation\"`*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
#[inline]
pub unsafe fn SetupDiGetClassPropertyW(classguid: *const ::windows::core::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetClassPropertyW(classguid: *const ::windows::core::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetClassPropertyW(::core::mem::transmute(classguid), ::core::mem::transmute(propertykey), ::core::mem::transmute(propertytype), ::core::mem::transmute(propertybuffer), ::core::mem::transmute(propertybuffersize), ::core::mem::transmute(requiredsize), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetClassRegistryPropertyA<'a, Param6: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(classguid: *const ::windows::core::GUID, property: u32, propertyregdatatype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32, machinename: Param6, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetClassRegistryPropertyA(classguid: *const ::windows::core::GUID, property: u32, propertyregdatatype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32, machinename: ::windows::core::PCSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetClassRegistryPropertyA(::core::mem::transmute(classguid), ::core::mem::transmute(property), ::core::mem::transmute(propertyregdatatype), ::core::mem::transmute(propertybuffer), ::core::mem::transmute(propertybuffersize), ::core::mem::transmute(requiredsize), machinename.into_param().abi(), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetClassRegistryPropertyW<'a, Param6: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(classguid: *const ::windows::core::GUID, property: u32, propertyregdatatype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32, machinename: Param6, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetClassRegistryPropertyW(classguid: *const ::windows::core::GUID, property: u32, propertyregdatatype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32, machinename: ::windows::core::PCWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetClassRegistryPropertyW(::core::mem::transmute(classguid), ::core::mem::transmute(property), ::core::mem::transmute(propertyregdatatype), ::core::mem::transmute(propertybuffer), ::core::mem::transmute(propertybuffersize), ::core::mem::transmute(requiredsize), machinename.into_param().abi(), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetCustomDevicePropertyA<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, custompropertyname: Param2, flags: u32, propertyregdatatype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetCustomDevicePropertyA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, custompropertyname: ::windows::core::PCSTR, flags: u32, propertyregdatatype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetCustomDevicePropertyA(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), custompropertyname.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(propertyregdatatype), ::core::mem::transmute(propertybuffer), ::core::mem::transmute(propertybuffersize), ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetCustomDevicePropertyW<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, custompropertyname: Param2, flags: u32, propertyregdatatype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetCustomDevicePropertyW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, custompropertyname: ::windows::core::PCWSTR, flags: u32, propertyregdatatype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetCustomDevicePropertyW(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), custompropertyname.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(propertyregdatatype), ::core::mem::transmute(propertybuffer), ::core::mem::transmute(propertybuffersize), ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetDeviceInfoListClass(deviceinfoset: *const ::core::ffi::c_void, classguid: *mut ::windows::core::GUID) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetDeviceInfoListClass(deviceinfoset: *const ::core::ffi::c_void, classguid: *mut ::windows::core::GUID) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetDeviceInfoListClass(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(classguid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetDeviceInfoListDetailA(deviceinfoset: *const ::core::ffi::c_void, deviceinfosetdetaildata: *mut SP_DEVINFO_LIST_DETAIL_DATA_A) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetDeviceInfoListDetailA(deviceinfoset: *const ::core::ffi::c_void, deviceinfosetdetaildata: *mut SP_DEVINFO_LIST_DETAIL_DATA_A) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetDeviceInfoListDetailA(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfosetdetaildata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetDeviceInfoListDetailW(deviceinfoset: *const ::core::ffi::c_void, deviceinfosetdetaildata: *mut SP_DEVINFO_LIST_DETAIL_DATA_W) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetDeviceInfoListDetailW(deviceinfoset: *const ::core::ffi::c_void, deviceinfosetdetaildata: *mut SP_DEVINFO_LIST_DETAIL_DATA_W) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetDeviceInfoListDetailW(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfosetdetaildata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetDeviceInstallParamsA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, deviceinstallparams: *mut SP_DEVINSTALL_PARAMS_A) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetDeviceInstallParamsA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, deviceinstallparams: *mut SP_DEVINSTALL_PARAMS_A) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetDeviceInstallParamsA(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(deviceinstallparams)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetDeviceInstallParamsW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, deviceinstallparams: *mut SP_DEVINSTALL_PARAMS_W) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetDeviceInstallParamsW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, deviceinstallparams: *mut SP_DEVINSTALL_PARAMS_W) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetDeviceInstallParamsW(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(deviceinstallparams)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetDeviceInstanceIdA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, deviceinstanceid: &mut [u8], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetDeviceInstanceIdA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, deviceinstanceid: ::windows::core::PSTR, deviceinstanceidsize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetDeviceInstanceIdA(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(deviceinstanceid)), deviceinstanceid.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetDeviceInstanceIdW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, deviceinstanceid: &mut [u16], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetDeviceInstanceIdW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, deviceinstanceid: ::windows::core::PWSTR, deviceinstanceidsize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetDeviceInstanceIdW(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(deviceinstanceid)), deviceinstanceid.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetDeviceInterfaceAlias(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, aliasinterfaceclassguid: *const ::windows::core::GUID, aliasdeviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetDeviceInterfaceAlias(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, aliasinterfaceclassguid: *const ::windows::core::GUID, aliasdeviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetDeviceInterfaceAlias(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinterfacedata), ::core::mem::transmute(aliasinterfaceclassguid), ::core::mem::transmute(aliasdeviceinterfacedata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetDeviceInterfaceDetailA(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, deviceinterfacedetaildata: *mut SP_DEVICE_INTERFACE_DETAIL_DATA_A, deviceinterfacedetaildatasize: u32, requiredsize: *mut u32, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetDeviceInterfaceDetailA(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, deviceinterfacedetaildata: *mut SP_DEVICE_INTERFACE_DETAIL_DATA_A, deviceinterfacedetaildatasize: u32, requiredsize: *mut u32, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetDeviceInterfaceDetailA(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinterfacedata), ::core::mem::transmute(deviceinterfacedetaildata), ::core::mem::transmute(deviceinterfacedetaildatasize), ::core::mem::transmute(requiredsize), ::core::mem::transmute(deviceinfodata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetDeviceInterfaceDetailW(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, deviceinterfacedetaildata: *mut SP_DEVICE_INTERFACE_DETAIL_DATA_W, deviceinterfacedetaildatasize: u32, requiredsize: *mut u32, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetDeviceInterfaceDetailW(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, deviceinterfacedetaildata: *mut SP_DEVICE_INTERFACE_DETAIL_DATA_W, deviceinterfacedetaildatasize: u32, requiredsize: *mut u32, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetDeviceInterfaceDetailW(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinterfacedata), ::core::mem::transmute(deviceinterfacedetaildata), ::core::mem::transmute(deviceinterfacedetaildatasize), ::core::mem::transmute(requiredsize), ::core::mem::transmute(deviceinfodata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`, `\"Win32_Foundation\"`*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
#[inline]
pub unsafe fn SetupDiGetDeviceInterfacePropertyKeys(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, propertykeyarray: &mut [super::Properties::DEVPROPKEY], requiredpropertykeycount: *mut u32, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetDeviceInterfacePropertyKeys(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: u32, requiredpropertykeycount: *mut u32, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetDeviceInterfacePropertyKeys(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinterfacedata), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(propertykeyarray)), propertykeyarray.len() as _, ::core::mem::transmute(requiredpropertykeycount), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`, `\"Win32_Foundation\"`*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
#[inline]
pub unsafe fn SetupDiGetDeviceInterfacePropertyW(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetDeviceInterfacePropertyW(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetDeviceInterfacePropertyW(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinterfacedata), ::core::mem::transmute(propertykey), ::core::mem::transmute(propertytype), ::core::mem::transmute(propertybuffer), ::core::mem::transmute(propertybuffersize), ::core::mem::transmute(requiredsize), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`, `\"Win32_Foundation\"`*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
#[inline]
pub unsafe fn SetupDiGetDevicePropertyKeys(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, propertykeyarray: &mut [super::Properties::DEVPROPKEY], requiredpropertykeycount: *mut u32, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetDevicePropertyKeys(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: u32, requiredpropertykeycount: *mut u32, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetDevicePropertyKeys(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(propertykeyarray)), propertykeyarray.len() as _, ::core::mem::transmute(requiredpropertykeycount), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`, `\"Win32_Foundation\"`*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
#[inline]
pub unsafe fn SetupDiGetDevicePropertyW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetDevicePropertyW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetDevicePropertyW(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(propertykey), ::core::mem::transmute(propertytype), ::core::mem::transmute(propertybuffer), ::core::mem::transmute(propertybuffersize), ::core::mem::transmute(requiredsize), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetDeviceRegistryPropertyA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, property: u32, propertyregdatatype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetDeviceRegistryPropertyA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, property: u32, propertyregdatatype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetDeviceRegistryPropertyA(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(property), ::core::mem::transmute(propertyregdatatype), ::core::mem::transmute(propertybuffer), ::core::mem::transmute(propertybuffersize), ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetDeviceRegistryPropertyW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, property: u32, propertyregdatatype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetDeviceRegistryPropertyW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, property: u32, propertyregdatatype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetDeviceRegistryPropertyW(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(property), ::core::mem::transmute(propertyregdatatype), ::core::mem::transmute(propertybuffer), ::core::mem::transmute(propertybuffersize), ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetDriverInfoDetailA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *const SP_DRVINFO_DATA_V2_A, driverinfodetaildata: *mut SP_DRVINFO_DETAIL_DATA_A, driverinfodetaildatasize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetDriverInfoDetailA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *const SP_DRVINFO_DATA_V2_A, driverinfodetaildata: *mut SP_DRVINFO_DETAIL_DATA_A, driverinfodetaildatasize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetDriverInfoDetailA(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(driverinfodata), ::core::mem::transmute(driverinfodetaildata), ::core::mem::transmute(driverinfodetaildatasize), ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetDriverInfoDetailW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *const SP_DRVINFO_DATA_V2_W, driverinfodetaildata: *mut SP_DRVINFO_DETAIL_DATA_W, driverinfodetaildatasize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetDriverInfoDetailW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *const SP_DRVINFO_DATA_V2_W, driverinfodetaildata: *mut SP_DRVINFO_DETAIL_DATA_W, driverinfodetaildatasize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetDriverInfoDetailW(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(driverinfodata), ::core::mem::transmute(driverinfodetaildata), ::core::mem::transmute(driverinfodetaildatasize), ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetDriverInstallParamsA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *const SP_DRVINFO_DATA_V2_A, driverinstallparams: *mut SP_DRVINSTALL_PARAMS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetDriverInstallParamsA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *const SP_DRVINFO_DATA_V2_A, driverinstallparams: *mut SP_DRVINSTALL_PARAMS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetDriverInstallParamsA(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(driverinfodata), ::core::mem::transmute(driverinstallparams)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetDriverInstallParamsW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *const SP_DRVINFO_DATA_V2_W, driverinstallparams: *mut SP_DRVINSTALL_PARAMS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetDriverInstallParamsW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *const SP_DRVINFO_DATA_V2_W, driverinstallparams: *mut SP_DRVINSTALL_PARAMS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetDriverInstallParamsW(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(driverinfodata), ::core::mem::transmute(driverinstallparams)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetHwProfileFriendlyNameA(hwprofile: u32, friendlyname: &mut [u8], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetHwProfileFriendlyNameA(hwprofile: u32, friendlyname: ::windows::core::PSTR, friendlynamesize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetHwProfileFriendlyNameA(::core::mem::transmute(hwprofile), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(friendlyname)), friendlyname.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetHwProfileFriendlyNameExA<'a, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hwprofile: u32, friendlyname: &mut [u8], requiredsize: *mut u32, machinename: Param4, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetHwProfileFriendlyNameExA(hwprofile: u32, friendlyname: ::windows::core::PSTR, friendlynamesize: u32, requiredsize: *mut u32, machinename: ::windows::core::PCSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetHwProfileFriendlyNameExA(::core::mem::transmute(hwprofile), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(friendlyname)), friendlyname.len() as _, ::core::mem::transmute(requiredsize), machinename.into_param().abi(), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetHwProfileFriendlyNameExW<'a, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hwprofile: u32, friendlyname: &mut [u16], requiredsize: *mut u32, machinename: Param4, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetHwProfileFriendlyNameExW(hwprofile: u32, friendlyname: ::windows::core::PWSTR, friendlynamesize: u32, requiredsize: *mut u32, machinename: ::windows::core::PCWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetHwProfileFriendlyNameExW(::core::mem::transmute(hwprofile), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(friendlyname)), friendlyname.len() as _, ::core::mem::transmute(requiredsize), machinename.into_param().abi(), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetHwProfileFriendlyNameW(hwprofile: u32, friendlyname: &mut [u16], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetHwProfileFriendlyNameW(hwprofile: u32, friendlyname: ::windows::core::PWSTR, friendlynamesize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetHwProfileFriendlyNameW(::core::mem::transmute(hwprofile), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(friendlyname)), friendlyname.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetHwProfileList(hwprofilelist: &mut [u32], requiredsize: *mut u32, currentlyactiveindex: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetHwProfileList(hwprofilelist: *mut u32, hwprofilelistsize: u32, requiredsize: *mut u32, currentlyactiveindex: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetHwProfileList(::core::mem::transmute(::windows::core::as_mut_ptr_or_null(hwprofilelist)), hwprofilelist.len() as _, ::core::mem::transmute(requiredsize), ::core::mem::transmute(currentlyactiveindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetHwProfileListExA<'a, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hwprofilelist: &mut [u32], requiredsize: *mut u32, currentlyactiveindex: *mut u32, machinename: Param4, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetHwProfileListExA(hwprofilelist: *mut u32, hwprofilelistsize: u32, requiredsize: *mut u32, currentlyactiveindex: *mut u32, machinename: ::windows::core::PCSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetHwProfileListExA(::core::mem::transmute(::windows::core::as_mut_ptr_or_null(hwprofilelist)), hwprofilelist.len() as _, ::core::mem::transmute(requiredsize), ::core::mem::transmute(currentlyactiveindex), machinename.into_param().abi(), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetHwProfileListExW<'a, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hwprofilelist: &mut [u32], requiredsize: *mut u32, currentlyactiveindex: *mut u32, machinename: Param4, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetHwProfileListExW(hwprofilelist: *mut u32, hwprofilelistsize: u32, requiredsize: *mut u32, currentlyactiveindex: *mut u32, machinename: ::windows::core::PCWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetHwProfileListExW(::core::mem::transmute(::windows::core::as_mut_ptr_or_null(hwprofilelist)), hwprofilelist.len() as _, ::core::mem::transmute(requiredsize), ::core::mem::transmute(currentlyactiveindex), machinename.into_param().abi(), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetINFClassA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(infname: Param0, classguid: *mut ::windows::core::GUID, classname: &mut [u8], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetINFClassA(infname: ::windows::core::PCSTR, classguid: *mut ::windows::core::GUID, classname: ::windows::core::PSTR, classnamesize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetINFClassA(infname.into_param().abi(), ::core::mem::transmute(classguid), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(classname)), classname.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetINFClassW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(infname: Param0, classguid: *mut ::windows::core::GUID, classname: &mut [u16], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetINFClassW(infname: ::windows::core::PCWSTR, classguid: *mut ::windows::core::GUID, classname: ::windows::core::PWSTR, classnamesize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetINFClassW(infname.into_param().abi(), ::core::mem::transmute(classguid), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(classname)), classname.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetSelectedDevice(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetSelectedDevice(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetSelectedDevice(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetSelectedDriverA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *mut SP_DRVINFO_DATA_V2_A) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetSelectedDriverA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *mut SP_DRVINFO_DATA_V2_A) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetSelectedDriverA(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(driverinfodata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiGetSelectedDriverW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *mut SP_DRVINFO_DATA_V2_W) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetSelectedDriverW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *mut SP_DRVINFO_DATA_V2_W) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiGetSelectedDriverW(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(driverinfodata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
#[inline]
pub unsafe fn SetupDiGetWizardPage(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, installwizarddata: *const SP_INSTALLWIZARD_DATA, pagetype: u32, flags: u32) -> super::super::UI::Controls::HPROPSHEETPAGE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiGetWizardPage(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, installwizarddata: *const SP_INSTALLWIZARD_DATA, pagetype: u32, flags: u32) -> super::super::UI::Controls::HPROPSHEETPAGE;
        }
        ::core::mem::transmute(SetupDiGetWizardPage(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(installwizarddata), ::core::mem::transmute(pagetype), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiInstallClassA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hwndparent: Param0, inffilename: Param1, flags: u32, filequeue: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiInstallClassA(hwndparent: super::super::Foundation::HWND, inffilename: ::windows::core::PCSTR, flags: u32, filequeue: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiInstallClassA(hwndparent.into_param().abi(), inffilename.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(filequeue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiInstallClassExA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hwndparent: Param0, inffilename: Param1, flags: u32, filequeue: *const ::core::ffi::c_void, interfaceclassguid: *const ::windows::core::GUID, reserved1: *mut ::core::ffi::c_void, reserved2: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiInstallClassExA(hwndparent: super::super::Foundation::HWND, inffilename: ::windows::core::PCSTR, flags: u32, filequeue: *const ::core::ffi::c_void, interfaceclassguid: *const ::windows::core::GUID, reserved1: *mut ::core::ffi::c_void, reserved2: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiInstallClassExA(hwndparent.into_param().abi(), inffilename.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(filequeue), ::core::mem::transmute(interfaceclassguid), ::core::mem::transmute(reserved1), ::core::mem::transmute(reserved2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiInstallClassExW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hwndparent: Param0, inffilename: Param1, flags: u32, filequeue: *const ::core::ffi::c_void, interfaceclassguid: *const ::windows::core::GUID, reserved1: *mut ::core::ffi::c_void, reserved2: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiInstallClassExW(hwndparent: super::super::Foundation::HWND, inffilename: ::windows::core::PCWSTR, flags: u32, filequeue: *const ::core::ffi::c_void, interfaceclassguid: *const ::windows::core::GUID, reserved1: *mut ::core::ffi::c_void, reserved2: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiInstallClassExW(hwndparent.into_param().abi(), inffilename.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(filequeue), ::core::mem::transmute(interfaceclassguid), ::core::mem::transmute(reserved1), ::core::mem::transmute(reserved2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiInstallClassW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hwndparent: Param0, inffilename: Param1, flags: u32, filequeue: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiInstallClassW(hwndparent: super::super::Foundation::HWND, inffilename: ::windows::core::PCWSTR, flags: u32, filequeue: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiInstallClassW(hwndparent.into_param().abi(), inffilename.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(filequeue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiInstallDevice(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiInstallDevice(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiInstallDevice(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiInstallDeviceInterfaces(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiInstallDeviceInterfaces(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiInstallDeviceInterfaces(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiInstallDriverFiles(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiInstallDriverFiles(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiInstallDriverFiles(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn SetupDiLoadClassIcon(classguid: *const ::windows::core::GUID, largeicon: *mut super::super::UI::WindowsAndMessaging::HICON, miniiconindex: *mut i32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiLoadClassIcon(classguid: *const ::windows::core::GUID, largeicon: *mut super::super::UI::WindowsAndMessaging::HICON, miniiconindex: *mut i32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiLoadClassIcon(::core::mem::transmute(classguid), ::core::mem::transmute(largeicon), ::core::mem::transmute(miniiconindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn SetupDiLoadDeviceIcon(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, cxicon: u32, cyicon: u32, flags: u32, hicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiLoadDeviceIcon(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, cxicon: u32, cyicon: u32, flags: u32, hicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiLoadDeviceIcon(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(cxicon), ::core::mem::transmute(cyicon), ::core::mem::transmute(flags), ::core::mem::transmute(hicon)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn SetupDiOpenClassRegKey(classguid: *const ::windows::core::GUID, samdesired: u32) -> super::super::System::Registry::HKEY {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiOpenClassRegKey(classguid: *const ::windows::core::GUID, samdesired: u32) -> super::super::System::Registry::HKEY;
        }
        ::core::mem::transmute(SetupDiOpenClassRegKey(::core::mem::transmute(classguid), ::core::mem::transmute(samdesired)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn SetupDiOpenClassRegKeyExA<'a, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(classguid: *const ::windows::core::GUID, samdesired: u32, flags: u32, machinename: Param3, reserved: *mut ::core::ffi::c_void) -> super::super::System::Registry::HKEY {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiOpenClassRegKeyExA(classguid: *const ::windows::core::GUID, samdesired: u32, flags: u32, machinename: ::windows::core::PCSTR, reserved: *mut ::core::ffi::c_void) -> super::super::System::Registry::HKEY;
        }
        ::core::mem::transmute(SetupDiOpenClassRegKeyExA(::core::mem::transmute(classguid), ::core::mem::transmute(samdesired), ::core::mem::transmute(flags), machinename.into_param().abi(), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn SetupDiOpenClassRegKeyExW<'a, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(classguid: *const ::windows::core::GUID, samdesired: u32, flags: u32, machinename: Param3, reserved: *mut ::core::ffi::c_void) -> super::super::System::Registry::HKEY {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiOpenClassRegKeyExW(classguid: *const ::windows::core::GUID, samdesired: u32, flags: u32, machinename: ::windows::core::PCWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::System::Registry::HKEY;
        }
        ::core::mem::transmute(SetupDiOpenClassRegKeyExW(::core::mem::transmute(classguid), ::core::mem::transmute(samdesired), ::core::mem::transmute(flags), machinename.into_param().abi(), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn SetupDiOpenDevRegKey(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, scope: u32, hwprofile: u32, keytype: u32, samdesired: u32) -> super::super::System::Registry::HKEY {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiOpenDevRegKey(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, scope: u32, hwprofile: u32, keytype: u32, samdesired: u32) -> super::super::System::Registry::HKEY;
        }
        ::core::mem::transmute(SetupDiOpenDevRegKey(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(scope), ::core::mem::transmute(hwprofile), ::core::mem::transmute(keytype), ::core::mem::transmute(samdesired)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiOpenDeviceInfoA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(deviceinfoset: *const ::core::ffi::c_void, deviceinstanceid: Param1, hwndparent: Param2, openflags: u32, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiOpenDeviceInfoA(deviceinfoset: *const ::core::ffi::c_void, deviceinstanceid: ::windows::core::PCSTR, hwndparent: super::super::Foundation::HWND, openflags: u32, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiOpenDeviceInfoA(::core::mem::transmute(deviceinfoset), deviceinstanceid.into_param().abi(), hwndparent.into_param().abi(), ::core::mem::transmute(openflags), ::core::mem::transmute(deviceinfodata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiOpenDeviceInfoW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(deviceinfoset: *const ::core::ffi::c_void, deviceinstanceid: Param1, hwndparent: Param2, openflags: u32, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiOpenDeviceInfoW(deviceinfoset: *const ::core::ffi::c_void, deviceinstanceid: ::windows::core::PCWSTR, hwndparent: super::super::Foundation::HWND, openflags: u32, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiOpenDeviceInfoW(::core::mem::transmute(deviceinfoset), deviceinstanceid.into_param().abi(), hwndparent.into_param().abi(), ::core::mem::transmute(openflags), ::core::mem::transmute(deviceinfodata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiOpenDeviceInterfaceA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(deviceinfoset: *const ::core::ffi::c_void, devicepath: Param1, openflags: u32, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiOpenDeviceInterfaceA(deviceinfoset: *const ::core::ffi::c_void, devicepath: ::windows::core::PCSTR, openflags: u32, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiOpenDeviceInterfaceA(::core::mem::transmute(deviceinfoset), devicepath.into_param().abi(), ::core::mem::transmute(openflags), ::core::mem::transmute(deviceinterfacedata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn SetupDiOpenDeviceInterfaceRegKey(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, reserved: u32, samdesired: u32) -> super::super::System::Registry::HKEY {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiOpenDeviceInterfaceRegKey(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, reserved: u32, samdesired: u32) -> super::super::System::Registry::HKEY;
        }
        ::core::mem::transmute(SetupDiOpenDeviceInterfaceRegKey(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinterfacedata), ::core::mem::transmute(reserved), ::core::mem::transmute(samdesired)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiOpenDeviceInterfaceW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(deviceinfoset: *const ::core::ffi::c_void, devicepath: Param1, openflags: u32, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiOpenDeviceInterfaceW(deviceinfoset: *const ::core::ffi::c_void, devicepath: ::windows::core::PCWSTR, openflags: u32, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiOpenDeviceInterfaceW(::core::mem::transmute(deviceinfoset), devicepath.into_param().abi(), ::core::mem::transmute(openflags), ::core::mem::transmute(deviceinterfacedata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiRegisterCoDeviceInstallers(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiRegisterCoDeviceInstallers(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiRegisterCoDeviceInstallers(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiRegisterDeviceInfo(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA, flags: u32, compareproc: PSP_DETSIG_CMPPROC, comparecontext: *const ::core::ffi::c_void, dupdeviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiRegisterDeviceInfo(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA, flags: u32, compareproc: ::windows::core::RawPtr, comparecontext: *const ::core::ffi::c_void, dupdeviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiRegisterDeviceInfo(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(flags), ::core::mem::transmute(compareproc), ::core::mem::transmute(comparecontext), ::core::mem::transmute(dupdeviceinfodata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiRemoveDevice(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiRemoveDevice(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiRemoveDevice(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiRemoveDeviceInterface(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiRemoveDeviceInterface(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiRemoveDeviceInterface(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinterfacedata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiRestartDevices(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiRestartDevices(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiRestartDevices(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiSelectBestCompatDrv(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiSelectBestCompatDrv(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiSelectBestCompatDrv(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiSelectDevice(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiSelectDevice(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiSelectDevice(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiSelectOEMDrv<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwndparent: Param0, deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiSelectOEMDrv(hwndparent: super::super::Foundation::HWND, deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiSelectOEMDrv(hwndparent.into_param().abi(), ::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiSetClassInstallParamsA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, classinstallparams: *const SP_CLASSINSTALL_HEADER, classinstallparamssize: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiSetClassInstallParamsA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, classinstallparams: *const SP_CLASSINSTALL_HEADER, classinstallparamssize: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiSetClassInstallParamsA(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(classinstallparams), ::core::mem::transmute(classinstallparamssize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiSetClassInstallParamsW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, classinstallparams: *const SP_CLASSINSTALL_HEADER, classinstallparamssize: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiSetClassInstallParamsW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, classinstallparams: *const SP_CLASSINSTALL_HEADER, classinstallparamssize: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiSetClassInstallParamsW(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(classinstallparams), ::core::mem::transmute(classinstallparamssize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`, `\"Win32_Foundation\"`*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
#[inline]
pub unsafe fn SetupDiSetClassPropertyExW<'a, Param6: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(classguid: *const ::windows::core::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, flags: u32, machinename: Param6, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiSetClassPropertyExW(classguid: *const ::windows::core::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, flags: u32, machinename: ::windows::core::PCWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiSetClassPropertyExW(::core::mem::transmute(classguid), ::core::mem::transmute(propertykey), ::core::mem::transmute(propertytype), ::core::mem::transmute(propertybuffer), ::core::mem::transmute(propertybuffersize), ::core::mem::transmute(flags), machinename.into_param().abi(), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`, `\"Win32_Foundation\"`*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
#[inline]
pub unsafe fn SetupDiSetClassPropertyW(classguid: *const ::windows::core::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiSetClassPropertyW(classguid: *const ::windows::core::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiSetClassPropertyW(::core::mem::transmute(classguid), ::core::mem::transmute(propertykey), ::core::mem::transmute(propertytype), ::core::mem::transmute(propertybuffer), ::core::mem::transmute(propertybuffersize), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiSetClassRegistryPropertyA<'a, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(classguid: *const ::windows::core::GUID, property: u32, propertybuffer: *const u8, propertybuffersize: u32, machinename: Param4, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiSetClassRegistryPropertyA(classguid: *const ::windows::core::GUID, property: u32, propertybuffer: *const u8, propertybuffersize: u32, machinename: ::windows::core::PCSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiSetClassRegistryPropertyA(::core::mem::transmute(classguid), ::core::mem::transmute(property), ::core::mem::transmute(propertybuffer), ::core::mem::transmute(propertybuffersize), machinename.into_param().abi(), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiSetClassRegistryPropertyW<'a, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(classguid: *const ::windows::core::GUID, property: u32, propertybuffer: *const u8, propertybuffersize: u32, machinename: Param4, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiSetClassRegistryPropertyW(classguid: *const ::windows::core::GUID, property: u32, propertybuffer: *const u8, propertybuffersize: u32, machinename: ::windows::core::PCWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiSetClassRegistryPropertyW(::core::mem::transmute(classguid), ::core::mem::transmute(property), ::core::mem::transmute(propertybuffer), ::core::mem::transmute(propertybuffersize), machinename.into_param().abi(), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiSetDeviceInstallParamsA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, deviceinstallparams: *const SP_DEVINSTALL_PARAMS_A) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiSetDeviceInstallParamsA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, deviceinstallparams: *const SP_DEVINSTALL_PARAMS_A) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiSetDeviceInstallParamsA(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(deviceinstallparams)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiSetDeviceInstallParamsW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, deviceinstallparams: *const SP_DEVINSTALL_PARAMS_W) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiSetDeviceInstallParamsW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, deviceinstallparams: *const SP_DEVINSTALL_PARAMS_W) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiSetDeviceInstallParamsW(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(deviceinstallparams)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiSetDeviceInterfaceDefault(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA, flags: u32, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiSetDeviceInterfaceDefault(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA, flags: u32, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiSetDeviceInterfaceDefault(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinterfacedata), ::core::mem::transmute(flags), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`, `\"Win32_Foundation\"`*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
#[inline]
pub unsafe fn SetupDiSetDeviceInterfacePropertyW(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiSetDeviceInterfacePropertyW(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiSetDeviceInterfacePropertyW(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinterfacedata), ::core::mem::transmute(propertykey), ::core::mem::transmute(propertytype), ::core::mem::transmute(propertybuffer), ::core::mem::transmute(propertybuffersize), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`, `\"Win32_Foundation\"`*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
#[inline]
pub unsafe fn SetupDiSetDevicePropertyW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiSetDevicePropertyW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiSetDevicePropertyW(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(propertykey), ::core::mem::transmute(propertytype), ::core::mem::transmute(propertybuffer), ::core::mem::transmute(propertybuffersize), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiSetDeviceRegistryPropertyA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA, property: u32, propertybuffer: *const u8, propertybuffersize: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiSetDeviceRegistryPropertyA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA, property: u32, propertybuffer: *const u8, propertybuffersize: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiSetDeviceRegistryPropertyA(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(property), ::core::mem::transmute(propertybuffer), ::core::mem::transmute(propertybuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiSetDeviceRegistryPropertyW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA, property: u32, propertybuffer: *const u8, propertybuffersize: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiSetDeviceRegistryPropertyW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA, property: u32, propertybuffer: *const u8, propertybuffersize: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiSetDeviceRegistryPropertyW(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(property), ::core::mem::transmute(propertybuffer), ::core::mem::transmute(propertybuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiSetDriverInstallParamsA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *const SP_DRVINFO_DATA_V2_A, driverinstallparams: *const SP_DRVINSTALL_PARAMS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiSetDriverInstallParamsA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *const SP_DRVINFO_DATA_V2_A, driverinstallparams: *const SP_DRVINSTALL_PARAMS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiSetDriverInstallParamsA(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(driverinfodata), ::core::mem::transmute(driverinstallparams)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiSetDriverInstallParamsW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *const SP_DRVINFO_DATA_V2_W, driverinstallparams: *const SP_DRVINSTALL_PARAMS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiSetDriverInstallParamsW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *const SP_DRVINFO_DATA_V2_W, driverinstallparams: *const SP_DRVINSTALL_PARAMS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiSetDriverInstallParamsW(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(driverinfodata), ::core::mem::transmute(driverinstallparams)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiSetSelectedDevice(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiSetSelectedDevice(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiSetSelectedDevice(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiSetSelectedDriverA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA, driverinfodata: *mut SP_DRVINFO_DATA_V2_A) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiSetSelectedDriverA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA, driverinfodata: *mut SP_DRVINFO_DATA_V2_A) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiSetSelectedDriverA(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(driverinfodata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiSetSelectedDriverW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA, driverinfodata: *mut SP_DRVINFO_DATA_V2_W) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiSetSelectedDriverW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA, driverinfodata: *mut SP_DRVINFO_DATA_V2_W) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiSetSelectedDriverW(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(driverinfodata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupDiUnremoveDevice(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDiUnremoveDevice(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupDiUnremoveDevice(::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn SetupDuplicateDiskSpaceListA(diskspace: *const ::core::ffi::c_void, reserved1: *mut ::core::ffi::c_void, reserved2: u32, flags: u32) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDuplicateDiskSpaceListA(diskspace: *const ::core::ffi::c_void, reserved1: *mut ::core::ffi::c_void, reserved2: u32, flags: u32) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(SetupDuplicateDiskSpaceListA(::core::mem::transmute(diskspace), ::core::mem::transmute(reserved1), ::core::mem::transmute(reserved2), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn SetupDuplicateDiskSpaceListW(diskspace: *const ::core::ffi::c_void, reserved1: *mut ::core::ffi::c_void, reserved2: u32, flags: u32) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupDuplicateDiskSpaceListW(diskspace: *const ::core::ffi::c_void, reserved1: *mut ::core::ffi::c_void, reserved2: u32, flags: u32) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(SetupDuplicateDiskSpaceListW(::core::mem::transmute(diskspace), ::core::mem::transmute(reserved1), ::core::mem::transmute(reserved2), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupEnumInfSectionsA(infhandle: *const ::core::ffi::c_void, index: u32, buffer: &mut [u8], sizeneeded: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupEnumInfSectionsA(infhandle: *const ::core::ffi::c_void, index: u32, buffer: ::windows::core::PSTR, size: u32, sizeneeded: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupEnumInfSectionsA(::core::mem::transmute(infhandle), ::core::mem::transmute(index), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(buffer)), buffer.len() as _, ::core::mem::transmute(sizeneeded)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupEnumInfSectionsW(infhandle: *const ::core::ffi::c_void, index: u32, buffer: &mut [u16], sizeneeded: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupEnumInfSectionsW(infhandle: *const ::core::ffi::c_void, index: u32, buffer: ::windows::core::PWSTR, size: u32, sizeneeded: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupEnumInfSectionsW(::core::mem::transmute(infhandle), ::core::mem::transmute(index), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(buffer)), buffer.len() as _, ::core::mem::transmute(sizeneeded)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SetupFileLogInfo(pub i32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SetupFileLogSourceFilename: SetupFileLogInfo = SetupFileLogInfo(0i32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SetupFileLogChecksum: SetupFileLogInfo = SetupFileLogInfo(1i32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SetupFileLogDiskTagfile: SetupFileLogInfo = SetupFileLogInfo(2i32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SetupFileLogDiskDescription: SetupFileLogInfo = SetupFileLogInfo(3i32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SetupFileLogOtherInfo: SetupFileLogInfo = SetupFileLogInfo(4i32);
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SetupFileLogMax: SetupFileLogInfo = SetupFileLogInfo(5i32);
impl ::core::marker::Copy for SetupFileLogInfo {}
impl ::core::clone::Clone for SetupFileLogInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SetupFileLogInfo {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SetupFileLogInfo {
    type Abi = Self;
}
impl ::core::fmt::Debug for SetupFileLogInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SetupFileLogInfo").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupFindFirstLineA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(infhandle: *const ::core::ffi::c_void, section: Param1, key: Param2, context: *mut INFCONTEXT) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupFindFirstLineA(infhandle: *const ::core::ffi::c_void, section: ::windows::core::PCSTR, key: ::windows::core::PCSTR, context: *mut INFCONTEXT) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupFindFirstLineA(::core::mem::transmute(infhandle), section.into_param().abi(), key.into_param().abi(), ::core::mem::transmute(context)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupFindFirstLineW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(infhandle: *const ::core::ffi::c_void, section: Param1, key: Param2, context: *mut INFCONTEXT) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupFindFirstLineW(infhandle: *const ::core::ffi::c_void, section: ::windows::core::PCWSTR, key: ::windows::core::PCWSTR, context: *mut INFCONTEXT) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupFindFirstLineW(::core::mem::transmute(infhandle), section.into_param().abi(), key.into_param().abi(), ::core::mem::transmute(context)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupFindNextLine(contextin: *const INFCONTEXT, contextout: *mut INFCONTEXT) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupFindNextLine(contextin: *const INFCONTEXT, contextout: *mut INFCONTEXT) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupFindNextLine(::core::mem::transmute(contextin), ::core::mem::transmute(contextout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupFindNextMatchLineA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(contextin: *const INFCONTEXT, key: Param1, contextout: *mut INFCONTEXT) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupFindNextMatchLineA(contextin: *const INFCONTEXT, key: ::windows::core::PCSTR, contextout: *mut INFCONTEXT) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupFindNextMatchLineA(::core::mem::transmute(contextin), key.into_param().abi(), ::core::mem::transmute(contextout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupFindNextMatchLineW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(contextin: *const INFCONTEXT, key: Param1, contextout: *mut INFCONTEXT) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupFindNextMatchLineW(contextin: *const INFCONTEXT, key: ::windows::core::PCWSTR, contextout: *mut INFCONTEXT) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupFindNextMatchLineW(::core::mem::transmute(contextin), key.into_param().abi(), ::core::mem::transmute(contextout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupFreeSourceListA(list: &mut [*mut ::windows::core::PSTR]) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupFreeSourceListA(list: *mut *mut ::windows::core::PSTR, count: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupFreeSourceListA(::core::mem::transmute(::windows::core::as_mut_ptr_or_null(list)), list.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupFreeSourceListW(list: &mut [*mut ::windows::core::PWSTR]) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupFreeSourceListW(list: *mut *mut ::windows::core::PWSTR, count: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupFreeSourceListW(::core::mem::transmute(::windows::core::as_mut_ptr_or_null(list)), list.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupGetBackupInformationA(queuehandle: *const ::core::ffi::c_void, backupparams: *mut SP_BACKUP_QUEUE_PARAMS_V2_A) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetBackupInformationA(queuehandle: *const ::core::ffi::c_void, backupparams: *mut SP_BACKUP_QUEUE_PARAMS_V2_A) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupGetBackupInformationA(::core::mem::transmute(queuehandle), ::core::mem::transmute(backupparams)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupGetBackupInformationW(queuehandle: *const ::core::ffi::c_void, backupparams: *mut SP_BACKUP_QUEUE_PARAMS_V2_W) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetBackupInformationW(queuehandle: *const ::core::ffi::c_void, backupparams: *mut SP_BACKUP_QUEUE_PARAMS_V2_W) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupGetBackupInformationW(::core::mem::transmute(queuehandle), ::core::mem::transmute(backupparams)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupGetBinaryField(context: *const INFCONTEXT, fieldindex: u32, returnbuffer: *mut u8, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetBinaryField(context: *const INFCONTEXT, fieldindex: u32, returnbuffer: *mut u8, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupGetBinaryField(::core::mem::transmute(context), ::core::mem::transmute(fieldindex), ::core::mem::transmute(returnbuffer), ::core::mem::transmute(returnbuffersize), ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn SetupGetFieldCount(context: *const INFCONTEXT) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetFieldCount(context: *const INFCONTEXT) -> u32;
        }
        ::core::mem::transmute(SetupGetFieldCount(::core::mem::transmute(context)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn SetupGetFileCompressionInfoA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(sourcefilename: Param0, actualsourcefilename: *mut ::windows::core::PSTR, sourcefilesize: *mut u32, targetfilesize: *mut u32, compressiontype: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetFileCompressionInfoA(sourcefilename: ::windows::core::PCSTR, actualsourcefilename: *mut ::windows::core::PSTR, sourcefilesize: *mut u32, targetfilesize: *mut u32, compressiontype: *mut u32) -> u32;
        }
        ::core::mem::transmute(SetupGetFileCompressionInfoA(sourcefilename.into_param().abi(), ::core::mem::transmute(actualsourcefilename), ::core::mem::transmute(sourcefilesize), ::core::mem::transmute(targetfilesize), ::core::mem::transmute(compressiontype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupGetFileCompressionInfoExA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(sourcefilename: Param0, actualsourcefilenamebuffer: &[u8], requiredbufferlen: *mut u32, sourcefilesize: *mut u32, targetfilesize: *mut u32, compressiontype: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetFileCompressionInfoExA(sourcefilename: ::windows::core::PCSTR, actualsourcefilenamebuffer: ::windows::core::PCSTR, actualsourcefilenamebufferlen: u32, requiredbufferlen: *mut u32, sourcefilesize: *mut u32, targetfilesize: *mut u32, compressiontype: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupGetFileCompressionInfoExA(sourcefilename.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(actualsourcefilenamebuffer)), actualsourcefilenamebuffer.len() as _, ::core::mem::transmute(requiredbufferlen), ::core::mem::transmute(sourcefilesize), ::core::mem::transmute(targetfilesize), ::core::mem::transmute(compressiontype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupGetFileCompressionInfoExW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(sourcefilename: Param0, actualsourcefilenamebuffer: &[u16], requiredbufferlen: *mut u32, sourcefilesize: *mut u32, targetfilesize: *mut u32, compressiontype: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetFileCompressionInfoExW(sourcefilename: ::windows::core::PCWSTR, actualsourcefilenamebuffer: ::windows::core::PCWSTR, actualsourcefilenamebufferlen: u32, requiredbufferlen: *mut u32, sourcefilesize: *mut u32, targetfilesize: *mut u32, compressiontype: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupGetFileCompressionInfoExW(sourcefilename.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(actualsourcefilenamebuffer)), actualsourcefilenamebuffer.len() as _, ::core::mem::transmute(requiredbufferlen), ::core::mem::transmute(sourcefilesize), ::core::mem::transmute(targetfilesize), ::core::mem::transmute(compressiontype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn SetupGetFileCompressionInfoW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(sourcefilename: Param0, actualsourcefilename: *mut ::windows::core::PWSTR, sourcefilesize: *mut u32, targetfilesize: *mut u32, compressiontype: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetFileCompressionInfoW(sourcefilename: ::windows::core::PCWSTR, actualsourcefilename: *mut ::windows::core::PWSTR, sourcefilesize: *mut u32, targetfilesize: *mut u32, compressiontype: *mut u32) -> u32;
        }
        ::core::mem::transmute(SetupGetFileCompressionInfoW(sourcefilename.into_param().abi(), ::core::mem::transmute(actualsourcefilename), ::core::mem::transmute(sourcefilesize), ::core::mem::transmute(targetfilesize), ::core::mem::transmute(compressiontype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupGetFileQueueCount(filequeue: *const ::core::ffi::c_void, subqueuefileop: u32, numoperations: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetFileQueueCount(filequeue: *const ::core::ffi::c_void, subqueuefileop: u32, numoperations: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupGetFileQueueCount(::core::mem::transmute(filequeue), ::core::mem::transmute(subqueuefileop), ::core::mem::transmute(numoperations)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupGetFileQueueFlags(filequeue: *const ::core::ffi::c_void, flags: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetFileQueueFlags(filequeue: *const ::core::ffi::c_void, flags: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupGetFileQueueFlags(::core::mem::transmute(filequeue), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
#[inline]
pub unsafe fn SetupGetInfDriverStoreLocationA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(filename: Param0, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, localename: Param2, returnbuffer: &mut [u8], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetInfDriverStoreLocationA(filename: ::windows::core::PCSTR, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, localename: ::windows::core::PCSTR, returnbuffer: ::windows::core::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupGetInfDriverStoreLocationA(filename.into_param().abi(), ::core::mem::transmute(alternateplatforminfo), localename.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(returnbuffer)), returnbuffer.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
#[inline]
pub unsafe fn SetupGetInfDriverStoreLocationW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(filename: Param0, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, localename: Param2, returnbuffer: &mut [u16], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetInfDriverStoreLocationW(filename: ::windows::core::PCWSTR, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, localename: ::windows::core::PCWSTR, returnbuffer: ::windows::core::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupGetInfDriverStoreLocationW(filename.into_param().abi(), ::core::mem::transmute(alternateplatforminfo), localename.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(returnbuffer)), returnbuffer.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupGetInfFileListA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(directorypath: Param0, infstyle: u32, returnbuffer: &mut [u8], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetInfFileListA(directorypath: ::windows::core::PCSTR, infstyle: u32, returnbuffer: ::windows::core::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupGetInfFileListA(directorypath.into_param().abi(), ::core::mem::transmute(infstyle), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(returnbuffer)), returnbuffer.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupGetInfFileListW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(directorypath: Param0, infstyle: u32, returnbuffer: &mut [u16], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetInfFileListW(directorypath: ::windows::core::PCWSTR, infstyle: u32, returnbuffer: ::windows::core::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupGetInfFileListW(directorypath.into_param().abi(), ::core::mem::transmute(infstyle), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(returnbuffer)), returnbuffer.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupGetInfInformationA(infspec: *const ::core::ffi::c_void, searchcontrol: u32, returnbuffer: *mut SP_INF_INFORMATION, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetInfInformationA(infspec: *const ::core::ffi::c_void, searchcontrol: u32, returnbuffer: *mut SP_INF_INFORMATION, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupGetInfInformationA(::core::mem::transmute(infspec), ::core::mem::transmute(searchcontrol), ::core::mem::transmute(returnbuffer), ::core::mem::transmute(returnbuffersize), ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupGetInfInformationW(infspec: *const ::core::ffi::c_void, searchcontrol: u32, returnbuffer: *mut SP_INF_INFORMATION, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetInfInformationW(infspec: *const ::core::ffi::c_void, searchcontrol: u32, returnbuffer: *mut SP_INF_INFORMATION, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupGetInfInformationW(::core::mem::transmute(infspec), ::core::mem::transmute(searchcontrol), ::core::mem::transmute(returnbuffer), ::core::mem::transmute(returnbuffersize), ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupGetInfPublishedNameA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(driverstorelocation: Param0, returnbuffer: &mut [u8], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetInfPublishedNameA(driverstorelocation: ::windows::core::PCSTR, returnbuffer: ::windows::core::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupGetInfPublishedNameA(driverstorelocation.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(returnbuffer)), returnbuffer.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupGetInfPublishedNameW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(driverstorelocation: Param0, returnbuffer: &mut [u16], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetInfPublishedNameW(driverstorelocation: ::windows::core::PCWSTR, returnbuffer: ::windows::core::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupGetInfPublishedNameW(driverstorelocation.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(returnbuffer)), returnbuffer.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupGetIntField(context: *const INFCONTEXT, fieldindex: u32, integervalue: *mut i32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetIntField(context: *const INFCONTEXT, fieldindex: u32, integervalue: *mut i32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupGetIntField(::core::mem::transmute(context), ::core::mem::transmute(fieldindex), ::core::mem::transmute(integervalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupGetLineByIndexA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(infhandle: *const ::core::ffi::c_void, section: Param1, index: u32, context: *mut INFCONTEXT) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetLineByIndexA(infhandle: *const ::core::ffi::c_void, section: ::windows::core::PCSTR, index: u32, context: *mut INFCONTEXT) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupGetLineByIndexA(::core::mem::transmute(infhandle), section.into_param().abi(), ::core::mem::transmute(index), ::core::mem::transmute(context)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupGetLineByIndexW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(infhandle: *const ::core::ffi::c_void, section: Param1, index: u32, context: *mut INFCONTEXT) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetLineByIndexW(infhandle: *const ::core::ffi::c_void, section: ::windows::core::PCWSTR, index: u32, context: *mut INFCONTEXT) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupGetLineByIndexW(::core::mem::transmute(infhandle), section.into_param().abi(), ::core::mem::transmute(index), ::core::mem::transmute(context)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn SetupGetLineCountA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(infhandle: *const ::core::ffi::c_void, section: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetLineCountA(infhandle: *const ::core::ffi::c_void, section: ::windows::core::PCSTR) -> i32;
        }
        ::core::mem::transmute(SetupGetLineCountA(::core::mem::transmute(infhandle), section.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn SetupGetLineCountW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(infhandle: *const ::core::ffi::c_void, section: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetLineCountW(infhandle: *const ::core::ffi::c_void, section: ::windows::core::PCWSTR) -> i32;
        }
        ::core::mem::transmute(SetupGetLineCountW(::core::mem::transmute(infhandle), section.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupGetLineTextA<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(context: *const INFCONTEXT, infhandle: *const ::core::ffi::c_void, section: Param2, key: Param3, returnbuffer: &mut [u8], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetLineTextA(context: *const INFCONTEXT, infhandle: *const ::core::ffi::c_void, section: ::windows::core::PCSTR, key: ::windows::core::PCSTR, returnbuffer: ::windows::core::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupGetLineTextA(::core::mem::transmute(context), ::core::mem::transmute(infhandle), section.into_param().abi(), key.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(returnbuffer)), returnbuffer.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupGetLineTextW<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(context: *const INFCONTEXT, infhandle: *const ::core::ffi::c_void, section: Param2, key: Param3, returnbuffer: &mut [u16], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetLineTextW(context: *const INFCONTEXT, infhandle: *const ::core::ffi::c_void, section: ::windows::core::PCWSTR, key: ::windows::core::PCWSTR, returnbuffer: ::windows::core::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupGetLineTextW(::core::mem::transmute(context), ::core::mem::transmute(infhandle), section.into_param().abi(), key.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(returnbuffer)), returnbuffer.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupGetMultiSzFieldA(context: *const INFCONTEXT, fieldindex: u32, returnbuffer: &mut [u8], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetMultiSzFieldA(context: *const INFCONTEXT, fieldindex: u32, returnbuffer: ::windows::core::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupGetMultiSzFieldA(::core::mem::transmute(context), ::core::mem::transmute(fieldindex), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(returnbuffer)), returnbuffer.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupGetMultiSzFieldW(context: *const INFCONTEXT, fieldindex: u32, returnbuffer: &mut [u16], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetMultiSzFieldW(context: *const INFCONTEXT, fieldindex: u32, returnbuffer: ::windows::core::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupGetMultiSzFieldW(::core::mem::transmute(context), ::core::mem::transmute(fieldindex), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(returnbuffer)), returnbuffer.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupGetNonInteractiveMode() -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetNonInteractiveMode() -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupGetNonInteractiveMode())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupGetSourceFileLocationA<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, filename: Param2, sourceid: *mut u32, returnbuffer: &mut [u8], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetSourceFileLocationA(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, filename: ::windows::core::PCSTR, sourceid: *mut u32, returnbuffer: ::windows::core::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupGetSourceFileLocationA(::core::mem::transmute(infhandle), ::core::mem::transmute(infcontext), filename.into_param().abi(), ::core::mem::transmute(sourceid), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(returnbuffer)), returnbuffer.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupGetSourceFileLocationW<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, filename: Param2, sourceid: *mut u32, returnbuffer: &mut [u16], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetSourceFileLocationW(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, filename: ::windows::core::PCWSTR, sourceid: *mut u32, returnbuffer: ::windows::core::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupGetSourceFileLocationW(::core::mem::transmute(infhandle), ::core::mem::transmute(infcontext), filename.into_param().abi(), ::core::mem::transmute(sourceid), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(returnbuffer)), returnbuffer.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupGetSourceFileSizeA<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, filename: Param2, section: Param3, filesize: *mut u32, roundingfactor: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetSourceFileSizeA(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, filename: ::windows::core::PCSTR, section: ::windows::core::PCSTR, filesize: *mut u32, roundingfactor: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupGetSourceFileSizeA(::core::mem::transmute(infhandle), ::core::mem::transmute(infcontext), filename.into_param().abi(), section.into_param().abi(), ::core::mem::transmute(filesize), ::core::mem::transmute(roundingfactor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupGetSourceFileSizeW<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, filename: Param2, section: Param3, filesize: *mut u32, roundingfactor: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetSourceFileSizeW(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, filename: ::windows::core::PCWSTR, section: ::windows::core::PCWSTR, filesize: *mut u32, roundingfactor: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupGetSourceFileSizeW(::core::mem::transmute(infhandle), ::core::mem::transmute(infcontext), filename.into_param().abi(), section.into_param().abi(), ::core::mem::transmute(filesize), ::core::mem::transmute(roundingfactor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupGetSourceInfoA(infhandle: *const ::core::ffi::c_void, sourceid: u32, infodesired: u32, returnbuffer: &mut [u8], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetSourceInfoA(infhandle: *const ::core::ffi::c_void, sourceid: u32, infodesired: u32, returnbuffer: ::windows::core::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupGetSourceInfoA(::core::mem::transmute(infhandle), ::core::mem::transmute(sourceid), ::core::mem::transmute(infodesired), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(returnbuffer)), returnbuffer.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupGetSourceInfoW(infhandle: *const ::core::ffi::c_void, sourceid: u32, infodesired: u32, returnbuffer: &mut [u16], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetSourceInfoW(infhandle: *const ::core::ffi::c_void, sourceid: u32, infodesired: u32, returnbuffer: ::windows::core::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupGetSourceInfoW(::core::mem::transmute(infhandle), ::core::mem::transmute(sourceid), ::core::mem::transmute(infodesired), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(returnbuffer)), returnbuffer.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupGetStringFieldA(context: *const INFCONTEXT, fieldindex: u32, returnbuffer: &mut [u8], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetStringFieldA(context: *const INFCONTEXT, fieldindex: u32, returnbuffer: ::windows::core::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupGetStringFieldA(::core::mem::transmute(context), ::core::mem::transmute(fieldindex), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(returnbuffer)), returnbuffer.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupGetStringFieldW(context: *const INFCONTEXT, fieldindex: u32, returnbuffer: &mut [u16], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetStringFieldW(context: *const INFCONTEXT, fieldindex: u32, returnbuffer: ::windows::core::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupGetStringFieldW(::core::mem::transmute(context), ::core::mem::transmute(fieldindex), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(returnbuffer)), returnbuffer.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupGetTargetPathA<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, section: Param2, returnbuffer: &mut [u8], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetTargetPathA(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, section: ::windows::core::PCSTR, returnbuffer: ::windows::core::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupGetTargetPathA(::core::mem::transmute(infhandle), ::core::mem::transmute(infcontext), section.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(returnbuffer)), returnbuffer.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupGetTargetPathW<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, section: Param2, returnbuffer: &mut [u16], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetTargetPathW(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, section: ::windows::core::PCWSTR, returnbuffer: ::windows::core::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupGetTargetPathW(::core::mem::transmute(infhandle), ::core::mem::transmute(infcontext), section.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(returnbuffer)), returnbuffer.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn SetupGetThreadLogToken() -> u64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupGetThreadLogToken() -> u64;
        }
        ::core::mem::transmute(SetupGetThreadLogToken())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupInitDefaultQueueCallback<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(ownerwindow: Param0) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupInitDefaultQueueCallback(ownerwindow: super::super::Foundation::HWND) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(SetupInitDefaultQueueCallback(ownerwindow.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupInitDefaultQueueCallbackEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(ownerwindow: Param0, alternateprogresswindow: Param1, progressmessage: u32, reserved1: u32, reserved2: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupInitDefaultQueueCallbackEx(ownerwindow: super::super::Foundation::HWND, alternateprogresswindow: super::super::Foundation::HWND, progressmessage: u32, reserved1: u32, reserved2: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(SetupInitDefaultQueueCallbackEx(ownerwindow.into_param().abi(), alternateprogresswindow.into_param().abi(), ::core::mem::transmute(progressmessage), ::core::mem::transmute(reserved1), ::core::mem::transmute(reserved2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn SetupInitializeFileLogA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(logfilename: Param0, flags: u32) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupInitializeFileLogA(logfilename: ::windows::core::PCSTR, flags: u32) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(SetupInitializeFileLogA(logfilename.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn SetupInitializeFileLogW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(logfilename: Param0, flags: u32) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupInitializeFileLogW(logfilename: ::windows::core::PCWSTR, flags: u32) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(SetupInitializeFileLogW(logfilename.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupInstallFileA<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, sourcefile: Param2, sourcepathroot: Param3, destinationname: Param4, copystyle: SP_COPY_STYLE, copymsghandler: PSP_FILE_CALLBACK_A, context: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupInstallFileA(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, sourcefile: ::windows::core::PCSTR, sourcepathroot: ::windows::core::PCSTR, destinationname: ::windows::core::PCSTR, copystyle: SP_COPY_STYLE, copymsghandler: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupInstallFileA(::core::mem::transmute(infhandle), ::core::mem::transmute(infcontext), sourcefile.into_param().abi(), sourcepathroot.into_param().abi(), destinationname.into_param().abi(), ::core::mem::transmute(copystyle), ::core::mem::transmute(copymsghandler), ::core::mem::transmute(context)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupInstallFileExA<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, sourcefile: Param2, sourcepathroot: Param3, destinationname: Param4, copystyle: SP_COPY_STYLE, copymsghandler: PSP_FILE_CALLBACK_A, context: *const ::core::ffi::c_void, filewasinuse: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupInstallFileExA(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, sourcefile: ::windows::core::PCSTR, sourcepathroot: ::windows::core::PCSTR, destinationname: ::windows::core::PCSTR, copystyle: SP_COPY_STYLE, copymsghandler: ::windows::core::RawPtr, context: *const ::core::ffi::c_void, filewasinuse: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupInstallFileExA(::core::mem::transmute(infhandle), ::core::mem::transmute(infcontext), sourcefile.into_param().abi(), sourcepathroot.into_param().abi(), destinationname.into_param().abi(), ::core::mem::transmute(copystyle), ::core::mem::transmute(copymsghandler), ::core::mem::transmute(context), ::core::mem::transmute(filewasinuse)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupInstallFileExW<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, sourcefile: Param2, sourcepathroot: Param3, destinationname: Param4, copystyle: SP_COPY_STYLE, copymsghandler: PSP_FILE_CALLBACK_W, context: *const ::core::ffi::c_void, filewasinuse: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupInstallFileExW(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, sourcefile: ::windows::core::PCWSTR, sourcepathroot: ::windows::core::PCWSTR, destinationname: ::windows::core::PCWSTR, copystyle: SP_COPY_STYLE, copymsghandler: ::windows::core::RawPtr, context: *const ::core::ffi::c_void, filewasinuse: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupInstallFileExW(::core::mem::transmute(infhandle), ::core::mem::transmute(infcontext), sourcefile.into_param().abi(), sourcepathroot.into_param().abi(), destinationname.into_param().abi(), ::core::mem::transmute(copystyle), ::core::mem::transmute(copymsghandler), ::core::mem::transmute(context), ::core::mem::transmute(filewasinuse)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupInstallFileW<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, sourcefile: Param2, sourcepathroot: Param3, destinationname: Param4, copystyle: SP_COPY_STYLE, copymsghandler: PSP_FILE_CALLBACK_W, context: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupInstallFileW(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, sourcefile: ::windows::core::PCWSTR, sourcepathroot: ::windows::core::PCWSTR, destinationname: ::windows::core::PCWSTR, copystyle: SP_COPY_STYLE, copymsghandler: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupInstallFileW(::core::mem::transmute(infhandle), ::core::mem::transmute(infcontext), sourcefile.into_param().abi(), sourcepathroot.into_param().abi(), destinationname.into_param().abi(), ::core::mem::transmute(copystyle), ::core::mem::transmute(copymsghandler), ::core::mem::transmute(context)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupInstallFilesFromInfSectionA<'a, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(infhandle: *const ::core::ffi::c_void, layoutinfhandle: *const ::core::ffi::c_void, filequeue: *const ::core::ffi::c_void, sectionname: Param3, sourcerootpath: Param4, copyflags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupInstallFilesFromInfSectionA(infhandle: *const ::core::ffi::c_void, layoutinfhandle: *const ::core::ffi::c_void, filequeue: *const ::core::ffi::c_void, sectionname: ::windows::core::PCSTR, sourcerootpath: ::windows::core::PCSTR, copyflags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupInstallFilesFromInfSectionA(::core::mem::transmute(infhandle), ::core::mem::transmute(layoutinfhandle), ::core::mem::transmute(filequeue), sectionname.into_param().abi(), sourcerootpath.into_param().abi(), ::core::mem::transmute(copyflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupInstallFilesFromInfSectionW<'a, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(infhandle: *const ::core::ffi::c_void, layoutinfhandle: *const ::core::ffi::c_void, filequeue: *const ::core::ffi::c_void, sectionname: Param3, sourcerootpath: Param4, copyflags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupInstallFilesFromInfSectionW(infhandle: *const ::core::ffi::c_void, layoutinfhandle: *const ::core::ffi::c_void, filequeue: *const ::core::ffi::c_void, sectionname: ::windows::core::PCWSTR, sourcerootpath: ::windows::core::PCWSTR, copyflags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupInstallFilesFromInfSectionW(::core::mem::transmute(infhandle), ::core::mem::transmute(layoutinfhandle), ::core::mem::transmute(filequeue), sectionname.into_param().abi(), sourcerootpath.into_param().abi(), ::core::mem::transmute(copyflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn SetupInstallFromInfSectionA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param4: ::windows::core::IntoParam<'a, super::super::System::Registry::HKEY>, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(owner: Param0, infhandle: *const ::core::ffi::c_void, sectionname: Param2, flags: u32, relativekeyroot: Param4, sourcerootpath: Param5, copyflags: u32, msghandler: PSP_FILE_CALLBACK_A, context: *const ::core::ffi::c_void, deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupInstallFromInfSectionA(owner: super::super::Foundation::HWND, infhandle: *const ::core::ffi::c_void, sectionname: ::windows::core::PCSTR, flags: u32, relativekeyroot: super::super::System::Registry::HKEY, sourcerootpath: ::windows::core::PCSTR, copyflags: u32, msghandler: ::windows::core::RawPtr, context: *const ::core::ffi::c_void, deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupInstallFromInfSectionA(owner.into_param().abi(), ::core::mem::transmute(infhandle), sectionname.into_param().abi(), ::core::mem::transmute(flags), relativekeyroot.into_param().abi(), sourcerootpath.into_param().abi(), ::core::mem::transmute(copyflags), ::core::mem::transmute(msghandler), ::core::mem::transmute(context), ::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn SetupInstallFromInfSectionW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::System::Registry::HKEY>, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(owner: Param0, infhandle: *const ::core::ffi::c_void, sectionname: Param2, flags: u32, relativekeyroot: Param4, sourcerootpath: Param5, copyflags: u32, msghandler: PSP_FILE_CALLBACK_W, context: *const ::core::ffi::c_void, deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupInstallFromInfSectionW(owner: super::super::Foundation::HWND, infhandle: *const ::core::ffi::c_void, sectionname: ::windows::core::PCWSTR, flags: u32, relativekeyroot: super::super::System::Registry::HKEY, sourcerootpath: ::windows::core::PCWSTR, copyflags: u32, msghandler: ::windows::core::RawPtr, context: *const ::core::ffi::c_void, deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupInstallFromInfSectionW(owner.into_param().abi(), ::core::mem::transmute(infhandle), sectionname.into_param().abi(), ::core::mem::transmute(flags), relativekeyroot.into_param().abi(), sourcerootpath.into_param().abi(), ::core::mem::transmute(copyflags), ::core::mem::transmute(msghandler), ::core::mem::transmute(context), ::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupInstallServicesFromInfSectionA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(infhandle: *const ::core::ffi::c_void, sectionname: Param1, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupInstallServicesFromInfSectionA(infhandle: *const ::core::ffi::c_void, sectionname: ::windows::core::PCSTR, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupInstallServicesFromInfSectionA(::core::mem::transmute(infhandle), sectionname.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupInstallServicesFromInfSectionExA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(infhandle: *const ::core::ffi::c_void, sectionname: Param1, flags: u32, deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, reserved1: *mut ::core::ffi::c_void, reserved2: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupInstallServicesFromInfSectionExA(infhandle: *const ::core::ffi::c_void, sectionname: ::windows::core::PCSTR, flags: u32, deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, reserved1: *mut ::core::ffi::c_void, reserved2: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupInstallServicesFromInfSectionExA(::core::mem::transmute(infhandle), sectionname.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(reserved1), ::core::mem::transmute(reserved2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupInstallServicesFromInfSectionExW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(infhandle: *const ::core::ffi::c_void, sectionname: Param1, flags: u32, deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, reserved1: *mut ::core::ffi::c_void, reserved2: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupInstallServicesFromInfSectionExW(infhandle: *const ::core::ffi::c_void, sectionname: ::windows::core::PCWSTR, flags: u32, deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, reserved1: *mut ::core::ffi::c_void, reserved2: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupInstallServicesFromInfSectionExW(::core::mem::transmute(infhandle), sectionname.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(deviceinfoset), ::core::mem::transmute(deviceinfodata), ::core::mem::transmute(reserved1), ::core::mem::transmute(reserved2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupInstallServicesFromInfSectionW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(infhandle: *const ::core::ffi::c_void, sectionname: Param1, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupInstallServicesFromInfSectionW(infhandle: *const ::core::ffi::c_void, sectionname: ::windows::core::PCWSTR, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupInstallServicesFromInfSectionW(::core::mem::transmute(infhandle), sectionname.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupIterateCabinetA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(cabinetfile: Param0, reserved: u32, msghandler: PSP_FILE_CALLBACK_A, context: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupIterateCabinetA(cabinetfile: ::windows::core::PCSTR, reserved: u32, msghandler: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupIterateCabinetA(cabinetfile.into_param().abi(), ::core::mem::transmute(reserved), ::core::mem::transmute(msghandler), ::core::mem::transmute(context)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupIterateCabinetW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(cabinetfile: Param0, reserved: u32, msghandler: PSP_FILE_CALLBACK_W, context: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupIterateCabinetW(cabinetfile: ::windows::core::PCWSTR, reserved: u32, msghandler: ::windows::core::RawPtr, context: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupIterateCabinetW(cabinetfile.into_param().abi(), ::core::mem::transmute(reserved), ::core::mem::transmute(msghandler), ::core::mem::transmute(context)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupLogErrorA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(messagestring: Param0, severity: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupLogErrorA(messagestring: ::windows::core::PCSTR, severity: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupLogErrorA(messagestring.into_param().abi(), ::core::mem::transmute(severity)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupLogErrorW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(messagestring: Param0, severity: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupLogErrorW(messagestring: ::windows::core::PCWSTR, severity: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupLogErrorW(messagestring.into_param().abi(), ::core::mem::transmute(severity)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupLogFileA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param6: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param7: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(fileloghandle: *const ::core::ffi::c_void, logsectionname: Param1, sourcefilename: Param2, targetfilename: Param3, checksum: u32, disktagfile: Param5, diskdescription: Param6, otherinfo: Param7, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupLogFileA(fileloghandle: *const ::core::ffi::c_void, logsectionname: ::windows::core::PCSTR, sourcefilename: ::windows::core::PCSTR, targetfilename: ::windows::core::PCSTR, checksum: u32, disktagfile: ::windows::core::PCSTR, diskdescription: ::windows::core::PCSTR, otherinfo: ::windows::core::PCSTR, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupLogFileA(::core::mem::transmute(fileloghandle), logsectionname.into_param().abi(), sourcefilename.into_param().abi(), targetfilename.into_param().abi(), ::core::mem::transmute(checksum), disktagfile.into_param().abi(), diskdescription.into_param().abi(), otherinfo.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupLogFileW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param6: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param7: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(fileloghandle: *const ::core::ffi::c_void, logsectionname: Param1, sourcefilename: Param2, targetfilename: Param3, checksum: u32, disktagfile: Param5, diskdescription: Param6, otherinfo: Param7, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupLogFileW(fileloghandle: *const ::core::ffi::c_void, logsectionname: ::windows::core::PCWSTR, sourcefilename: ::windows::core::PCWSTR, targetfilename: ::windows::core::PCWSTR, checksum: u32, disktagfile: ::windows::core::PCWSTR, diskdescription: ::windows::core::PCWSTR, otherinfo: ::windows::core::PCWSTR, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupLogFileW(::core::mem::transmute(fileloghandle), logsectionname.into_param().abi(), sourcefilename.into_param().abi(), targetfilename.into_param().abi(), ::core::mem::transmute(checksum), disktagfile.into_param().abi(), diskdescription.into_param().abi(), otherinfo.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupOpenAppendInfFileA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(filename: Param0, infhandle: *const ::core::ffi::c_void, errorline: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupOpenAppendInfFileA(filename: ::windows::core::PCSTR, infhandle: *const ::core::ffi::c_void, errorline: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupOpenAppendInfFileA(filename.into_param().abi(), ::core::mem::transmute(infhandle), ::core::mem::transmute(errorline)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupOpenAppendInfFileW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(filename: Param0, infhandle: *const ::core::ffi::c_void, errorline: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupOpenAppendInfFileW(filename: ::windows::core::PCWSTR, infhandle: *const ::core::ffi::c_void, errorline: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupOpenAppendInfFileW(filename.into_param().abi(), ::core::mem::transmute(infhandle), ::core::mem::transmute(errorline)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn SetupOpenFileQueue() -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupOpenFileQueue() -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(SetupOpenFileQueue())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn SetupOpenInfFileA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(filename: Param0, infclass: Param1, infstyle: u32, errorline: *mut u32) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupOpenInfFileA(filename: ::windows::core::PCSTR, infclass: ::windows::core::PCSTR, infstyle: u32, errorline: *mut u32) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(SetupOpenInfFileA(filename.into_param().abi(), infclass.into_param().abi(), ::core::mem::transmute(infstyle), ::core::mem::transmute(errorline)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn SetupOpenInfFileW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(filename: Param0, infclass: Param1, infstyle: u32, errorline: *mut u32) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupOpenInfFileW(filename: ::windows::core::PCWSTR, infclass: ::windows::core::PCWSTR, infstyle: u32, errorline: *mut u32) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(SetupOpenInfFileW(filename.into_param().abi(), infclass.into_param().abi(), ::core::mem::transmute(infstyle), ::core::mem::transmute(errorline)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupOpenLog<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(erase: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupOpenLog(erase: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupOpenLog(erase.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn SetupOpenMasterInf() -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupOpenMasterInf() -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(SetupOpenMasterInf())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupPrepareQueueForRestoreA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(queuehandle: *const ::core::ffi::c_void, backuppath: Param1, restoreflags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupPrepareQueueForRestoreA(queuehandle: *const ::core::ffi::c_void, backuppath: ::windows::core::PCSTR, restoreflags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupPrepareQueueForRestoreA(::core::mem::transmute(queuehandle), backuppath.into_param().abi(), ::core::mem::transmute(restoreflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupPrepareQueueForRestoreW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(queuehandle: *const ::core::ffi::c_void, backuppath: Param1, restoreflags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupPrepareQueueForRestoreW(queuehandle: *const ::core::ffi::c_void, backuppath: ::windows::core::PCWSTR, restoreflags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupPrepareQueueForRestoreW(::core::mem::transmute(queuehandle), backuppath.into_param().abi(), ::core::mem::transmute(restoreflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupPromptForDiskA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hwndparent: Param0, dialogtitle: Param1, diskname: Param2, pathtosource: Param3, filesought: Param4, tagfile: Param5, diskpromptstyle: u32, pathbuffer: &mut [u8], pathrequiredsize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupPromptForDiskA(hwndparent: super::super::Foundation::HWND, dialogtitle: ::windows::core::PCSTR, diskname: ::windows::core::PCSTR, pathtosource: ::windows::core::PCSTR, filesought: ::windows::core::PCSTR, tagfile: ::windows::core::PCSTR, diskpromptstyle: u32, pathbuffer: ::windows::core::PSTR, pathbuffersize: u32, pathrequiredsize: *mut u32) -> u32;
        }
        ::core::mem::transmute(SetupPromptForDiskA(hwndparent.into_param().abi(), dialogtitle.into_param().abi(), diskname.into_param().abi(), pathtosource.into_param().abi(), filesought.into_param().abi(), tagfile.into_param().abi(), ::core::mem::transmute(diskpromptstyle), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pathbuffer)), pathbuffer.len() as _, ::core::mem::transmute(pathrequiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupPromptForDiskW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hwndparent: Param0, dialogtitle: Param1, diskname: Param2, pathtosource: Param3, filesought: Param4, tagfile: Param5, diskpromptstyle: u32, pathbuffer: &mut [u16], pathrequiredsize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupPromptForDiskW(hwndparent: super::super::Foundation::HWND, dialogtitle: ::windows::core::PCWSTR, diskname: ::windows::core::PCWSTR, pathtosource: ::windows::core::PCWSTR, filesought: ::windows::core::PCWSTR, tagfile: ::windows::core::PCWSTR, diskpromptstyle: u32, pathbuffer: ::windows::core::PWSTR, pathbuffersize: u32, pathrequiredsize: *mut u32) -> u32;
        }
        ::core::mem::transmute(SetupPromptForDiskW(hwndparent.into_param().abi(), dialogtitle.into_param().abi(), diskname.into_param().abi(), pathtosource.into_param().abi(), filesought.into_param().abi(), tagfile.into_param().abi(), ::core::mem::transmute(diskpromptstyle), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pathbuffer)), pathbuffer.len() as _, ::core::mem::transmute(pathrequiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupPromptReboot<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(filequeue: *const ::core::ffi::c_void, owner: Param1, scanonly: Param2) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupPromptReboot(filequeue: *const ::core::ffi::c_void, owner: super::super::Foundation::HWND, scanonly: super::super::Foundation::BOOL) -> i32;
        }
        ::core::mem::transmute(SetupPromptReboot(::core::mem::transmute(filequeue), owner.into_param().abi(), scanonly.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupQueryDrivesInDiskSpaceListA(diskspace: *const ::core::ffi::c_void, returnbuffer: &mut [u8], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupQueryDrivesInDiskSpaceListA(diskspace: *const ::core::ffi::c_void, returnbuffer: ::windows::core::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupQueryDrivesInDiskSpaceListA(::core::mem::transmute(diskspace), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(returnbuffer)), returnbuffer.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupQueryDrivesInDiskSpaceListW(diskspace: *const ::core::ffi::c_void, returnbuffer: &mut [u16], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupQueryDrivesInDiskSpaceListW(diskspace: *const ::core::ffi::c_void, returnbuffer: ::windows::core::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupQueryDrivesInDiskSpaceListW(::core::mem::transmute(diskspace), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(returnbuffer)), returnbuffer.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupQueryFileLogA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(fileloghandle: *const ::core::ffi::c_void, logsectionname: Param1, targetfilename: Param2, desiredinfo: SetupFileLogInfo, dataout: &mut [u8], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupQueryFileLogA(fileloghandle: *const ::core::ffi::c_void, logsectionname: ::windows::core::PCSTR, targetfilename: ::windows::core::PCSTR, desiredinfo: SetupFileLogInfo, dataout: ::windows::core::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupQueryFileLogA(::core::mem::transmute(fileloghandle), logsectionname.into_param().abi(), targetfilename.into_param().abi(), ::core::mem::transmute(desiredinfo), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(dataout)), dataout.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupQueryFileLogW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(fileloghandle: *const ::core::ffi::c_void, logsectionname: Param1, targetfilename: Param2, desiredinfo: SetupFileLogInfo, dataout: &mut [u16], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupQueryFileLogW(fileloghandle: *const ::core::ffi::c_void, logsectionname: ::windows::core::PCWSTR, targetfilename: ::windows::core::PCWSTR, desiredinfo: SetupFileLogInfo, dataout: ::windows::core::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupQueryFileLogW(::core::mem::transmute(fileloghandle), logsectionname.into_param().abi(), targetfilename.into_param().abi(), ::core::mem::transmute(desiredinfo), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(dataout)), dataout.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupQueryInfFileInformationA(infinformation: *const SP_INF_INFORMATION, infindex: u32, returnbuffer: &mut [u8], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupQueryInfFileInformationA(infinformation: *const SP_INF_INFORMATION, infindex: u32, returnbuffer: ::windows::core::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupQueryInfFileInformationA(::core::mem::transmute(infinformation), ::core::mem::transmute(infindex), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(returnbuffer)), returnbuffer.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupQueryInfFileInformationW(infinformation: *const SP_INF_INFORMATION, infindex: u32, returnbuffer: &mut [u16], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupQueryInfFileInformationW(infinformation: *const SP_INF_INFORMATION, infindex: u32, returnbuffer: ::windows::core::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupQueryInfFileInformationW(::core::mem::transmute(infinformation), ::core::mem::transmute(infindex), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(returnbuffer)), returnbuffer.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
#[inline]
pub unsafe fn SetupQueryInfOriginalFileInformationA(infinformation: *const SP_INF_INFORMATION, infindex: u32, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, originalfileinfo: *mut SP_ORIGINAL_FILE_INFO_A) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupQueryInfOriginalFileInformationA(infinformation: *const SP_INF_INFORMATION, infindex: u32, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, originalfileinfo: *mut SP_ORIGINAL_FILE_INFO_A) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupQueryInfOriginalFileInformationA(::core::mem::transmute(infinformation), ::core::mem::transmute(infindex), ::core::mem::transmute(alternateplatforminfo), ::core::mem::transmute(originalfileinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
#[inline]
pub unsafe fn SetupQueryInfOriginalFileInformationW(infinformation: *const SP_INF_INFORMATION, infindex: u32, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, originalfileinfo: *mut SP_ORIGINAL_FILE_INFO_W) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupQueryInfOriginalFileInformationW(infinformation: *const SP_INF_INFORMATION, infindex: u32, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, originalfileinfo: *mut SP_ORIGINAL_FILE_INFO_W) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupQueryInfOriginalFileInformationW(::core::mem::transmute(infinformation), ::core::mem::transmute(infindex), ::core::mem::transmute(alternateplatforminfo), ::core::mem::transmute(originalfileinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupQueryInfVersionInformationA<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(infinformation: *const SP_INF_INFORMATION, infindex: u32, key: Param2, returnbuffer: &mut [u8], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupQueryInfVersionInformationA(infinformation: *const SP_INF_INFORMATION, infindex: u32, key: ::windows::core::PCSTR, returnbuffer: ::windows::core::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupQueryInfVersionInformationA(::core::mem::transmute(infinformation), ::core::mem::transmute(infindex), key.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(returnbuffer)), returnbuffer.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupQueryInfVersionInformationW<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(infinformation: *const SP_INF_INFORMATION, infindex: u32, key: Param2, returnbuffer: &mut [u16], requiredsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupQueryInfVersionInformationW(infinformation: *const SP_INF_INFORMATION, infindex: u32, key: ::windows::core::PCWSTR, returnbuffer: ::windows::core::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupQueryInfVersionInformationW(::core::mem::transmute(infinformation), ::core::mem::transmute(infindex), key.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(returnbuffer)), returnbuffer.len() as _, ::core::mem::transmute(requiredsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupQuerySourceListA(flags: u32, list: *mut *mut ::windows::core::PSTR, count: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupQuerySourceListA(flags: u32, list: *mut *mut ::windows::core::PSTR, count: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupQuerySourceListA(::core::mem::transmute(flags), ::core::mem::transmute(list), ::core::mem::transmute(count)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupQuerySourceListW(flags: u32, list: *mut *mut ::windows::core::PWSTR, count: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupQuerySourceListW(flags: u32, list: *mut *mut ::windows::core::PWSTR, count: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupQuerySourceListW(::core::mem::transmute(flags), ::core::mem::transmute(list), ::core::mem::transmute(count)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupQuerySpaceRequiredOnDriveA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(diskspace: *const ::core::ffi::c_void, drivespec: Param1, spacerequired: *mut i64, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupQuerySpaceRequiredOnDriveA(diskspace: *const ::core::ffi::c_void, drivespec: ::windows::core::PCSTR, spacerequired: *mut i64, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupQuerySpaceRequiredOnDriveA(::core::mem::transmute(diskspace), drivespec.into_param().abi(), ::core::mem::transmute(spacerequired), ::core::mem::transmute(reserved1), ::core::mem::transmute(reserved2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupQuerySpaceRequiredOnDriveW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(diskspace: *const ::core::ffi::c_void, drivespec: Param1, spacerequired: *mut i64, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupQuerySpaceRequiredOnDriveW(diskspace: *const ::core::ffi::c_void, drivespec: ::windows::core::PCWSTR, spacerequired: *mut i64, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupQuerySpaceRequiredOnDriveW(::core::mem::transmute(diskspace), drivespec.into_param().abi(), ::core::mem::transmute(spacerequired), ::core::mem::transmute(reserved1), ::core::mem::transmute(reserved2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupQueueCopyA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param6: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param7: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(queuehandle: *const ::core::ffi::c_void, sourcerootpath: Param1, sourcepath: Param2, sourcefilename: Param3, sourcedescription: Param4, sourcetagfile: Param5, targetdirectory: Param6, targetfilename: Param7, copystyle: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupQueueCopyA(queuehandle: *const ::core::ffi::c_void, sourcerootpath: ::windows::core::PCSTR, sourcepath: ::windows::core::PCSTR, sourcefilename: ::windows::core::PCSTR, sourcedescription: ::windows::core::PCSTR, sourcetagfile: ::windows::core::PCSTR, targetdirectory: ::windows::core::PCSTR, targetfilename: ::windows::core::PCSTR, copystyle: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupQueueCopyA(::core::mem::transmute(queuehandle), sourcerootpath.into_param().abi(), sourcepath.into_param().abi(), sourcefilename.into_param().abi(), sourcedescription.into_param().abi(), sourcetagfile.into_param().abi(), targetdirectory.into_param().abi(), targetfilename.into_param().abi(), ::core::mem::transmute(copystyle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupQueueCopyIndirectA(copyparams: *const SP_FILE_COPY_PARAMS_A) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupQueueCopyIndirectA(copyparams: *const SP_FILE_COPY_PARAMS_A) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupQueueCopyIndirectA(::core::mem::transmute(copyparams)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupQueueCopyIndirectW(copyparams: *const SP_FILE_COPY_PARAMS_W) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupQueueCopyIndirectW(copyparams: *const SP_FILE_COPY_PARAMS_W) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupQueueCopyIndirectW(::core::mem::transmute(copyparams)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupQueueCopySectionA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(queuehandle: *const ::core::ffi::c_void, sourcerootpath: Param1, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, section: Param4, copystyle: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupQueueCopySectionA(queuehandle: *const ::core::ffi::c_void, sourcerootpath: ::windows::core::PCSTR, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, section: ::windows::core::PCSTR, copystyle: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupQueueCopySectionA(::core::mem::transmute(queuehandle), sourcerootpath.into_param().abi(), ::core::mem::transmute(infhandle), ::core::mem::transmute(listinfhandle), section.into_param().abi(), ::core::mem::transmute(copystyle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupQueueCopySectionW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(queuehandle: *const ::core::ffi::c_void, sourcerootpath: Param1, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, section: Param4, copystyle: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupQueueCopySectionW(queuehandle: *const ::core::ffi::c_void, sourcerootpath: ::windows::core::PCWSTR, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, section: ::windows::core::PCWSTR, copystyle: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupQueueCopySectionW(::core::mem::transmute(queuehandle), sourcerootpath.into_param().abi(), ::core::mem::transmute(infhandle), ::core::mem::transmute(listinfhandle), section.into_param().abi(), ::core::mem::transmute(copystyle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupQueueCopyW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param6: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param7: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(queuehandle: *const ::core::ffi::c_void, sourcerootpath: Param1, sourcepath: Param2, sourcefilename: Param3, sourcedescription: Param4, sourcetagfile: Param5, targetdirectory: Param6, targetfilename: Param7, copystyle: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupQueueCopyW(queuehandle: *const ::core::ffi::c_void, sourcerootpath: ::windows::core::PCWSTR, sourcepath: ::windows::core::PCWSTR, sourcefilename: ::windows::core::PCWSTR, sourcedescription: ::windows::core::PCWSTR, sourcetagfile: ::windows::core::PCWSTR, targetdirectory: ::windows::core::PCWSTR, targetfilename: ::windows::core::PCWSTR, copystyle: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupQueueCopyW(::core::mem::transmute(queuehandle), sourcerootpath.into_param().abi(), sourcepath.into_param().abi(), sourcefilename.into_param().abi(), sourcedescription.into_param().abi(), sourcetagfile.into_param().abi(), targetdirectory.into_param().abi(), targetfilename.into_param().abi(), ::core::mem::transmute(copystyle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupQueueDefaultCopyA<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(queuehandle: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, sourcerootpath: Param2, sourcefilename: Param3, targetfilename: Param4, copystyle: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupQueueDefaultCopyA(queuehandle: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, sourcerootpath: ::windows::core::PCSTR, sourcefilename: ::windows::core::PCSTR, targetfilename: ::windows::core::PCSTR, copystyle: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupQueueDefaultCopyA(::core::mem::transmute(queuehandle), ::core::mem::transmute(infhandle), sourcerootpath.into_param().abi(), sourcefilename.into_param().abi(), targetfilename.into_param().abi(), ::core::mem::transmute(copystyle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupQueueDefaultCopyW<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(queuehandle: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, sourcerootpath: Param2, sourcefilename: Param3, targetfilename: Param4, copystyle: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupQueueDefaultCopyW(queuehandle: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, sourcerootpath: ::windows::core::PCWSTR, sourcefilename: ::windows::core::PCWSTR, targetfilename: ::windows::core::PCWSTR, copystyle: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupQueueDefaultCopyW(::core::mem::transmute(queuehandle), ::core::mem::transmute(infhandle), sourcerootpath.into_param().abi(), sourcefilename.into_param().abi(), targetfilename.into_param().abi(), ::core::mem::transmute(copystyle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupQueueDeleteA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(queuehandle: *const ::core::ffi::c_void, pathpart1: Param1, pathpart2: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupQueueDeleteA(queuehandle: *const ::core::ffi::c_void, pathpart1: ::windows::core::PCSTR, pathpart2: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupQueueDeleteA(::core::mem::transmute(queuehandle), pathpart1.into_param().abi(), pathpart2.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupQueueDeleteSectionA<'a, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(queuehandle: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, section: Param3) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupQueueDeleteSectionA(queuehandle: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, section: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupQueueDeleteSectionA(::core::mem::transmute(queuehandle), ::core::mem::transmute(infhandle), ::core::mem::transmute(listinfhandle), section.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupQueueDeleteSectionW<'a, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(queuehandle: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, section: Param3) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupQueueDeleteSectionW(queuehandle: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, section: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupQueueDeleteSectionW(::core::mem::transmute(queuehandle), ::core::mem::transmute(infhandle), ::core::mem::transmute(listinfhandle), section.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupQueueDeleteW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(queuehandle: *const ::core::ffi::c_void, pathpart1: Param1, pathpart2: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupQueueDeleteW(queuehandle: *const ::core::ffi::c_void, pathpart1: ::windows::core::PCWSTR, pathpart2: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupQueueDeleteW(::core::mem::transmute(queuehandle), pathpart1.into_param().abi(), pathpart2.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupQueueRenameA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(queuehandle: *const ::core::ffi::c_void, sourcepath: Param1, sourcefilename: Param2, targetpath: Param3, targetfilename: Param4) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupQueueRenameA(queuehandle: *const ::core::ffi::c_void, sourcepath: ::windows::core::PCSTR, sourcefilename: ::windows::core::PCSTR, targetpath: ::windows::core::PCSTR, targetfilename: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupQueueRenameA(::core::mem::transmute(queuehandle), sourcepath.into_param().abi(), sourcefilename.into_param().abi(), targetpath.into_param().abi(), targetfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupQueueRenameSectionA<'a, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(queuehandle: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, section: Param3) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupQueueRenameSectionA(queuehandle: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, section: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupQueueRenameSectionA(::core::mem::transmute(queuehandle), ::core::mem::transmute(infhandle), ::core::mem::transmute(listinfhandle), section.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupQueueRenameSectionW<'a, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(queuehandle: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, section: Param3) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupQueueRenameSectionW(queuehandle: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, section: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupQueueRenameSectionW(::core::mem::transmute(queuehandle), ::core::mem::transmute(infhandle), ::core::mem::transmute(listinfhandle), section.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupQueueRenameW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(queuehandle: *const ::core::ffi::c_void, sourcepath: Param1, sourcefilename: Param2, targetpath: Param3, targetfilename: Param4) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupQueueRenameW(queuehandle: *const ::core::ffi::c_void, sourcepath: ::windows::core::PCWSTR, sourcefilename: ::windows::core::PCWSTR, targetpath: ::windows::core::PCWSTR, targetfilename: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupQueueRenameW(::core::mem::transmute(queuehandle), sourcepath.into_param().abi(), sourcefilename.into_param().abi(), targetpath.into_param().abi(), targetfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupRemoveFileLogEntryA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(fileloghandle: *const ::core::ffi::c_void, logsectionname: Param1, targetfilename: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupRemoveFileLogEntryA(fileloghandle: *const ::core::ffi::c_void, logsectionname: ::windows::core::PCSTR, targetfilename: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupRemoveFileLogEntryA(::core::mem::transmute(fileloghandle), logsectionname.into_param().abi(), targetfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupRemoveFileLogEntryW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(fileloghandle: *const ::core::ffi::c_void, logsectionname: Param1, targetfilename: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupRemoveFileLogEntryW(fileloghandle: *const ::core::ffi::c_void, logsectionname: ::windows::core::PCWSTR, targetfilename: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupRemoveFileLogEntryW(::core::mem::transmute(fileloghandle), logsectionname.into_param().abi(), targetfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupRemoveFromDiskSpaceListA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(diskspace: *const ::core::ffi::c_void, targetfilespec: Param1, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupRemoveFromDiskSpaceListA(diskspace: *const ::core::ffi::c_void, targetfilespec: ::windows::core::PCSTR, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupRemoveFromDiskSpaceListA(::core::mem::transmute(diskspace), targetfilespec.into_param().abi(), ::core::mem::transmute(operation), ::core::mem::transmute(reserved1), ::core::mem::transmute(reserved2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupRemoveFromDiskSpaceListW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(diskspace: *const ::core::ffi::c_void, targetfilespec: Param1, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupRemoveFromDiskSpaceListW(diskspace: *const ::core::ffi::c_void, targetfilespec: ::windows::core::PCWSTR, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupRemoveFromDiskSpaceListW(::core::mem::transmute(diskspace), targetfilespec.into_param().abi(), ::core::mem::transmute(operation), ::core::mem::transmute(reserved1), ::core::mem::transmute(reserved2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupRemoveFromSourceListA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(flags: u32, source: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupRemoveFromSourceListA(flags: u32, source: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupRemoveFromSourceListA(::core::mem::transmute(flags), source.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupRemoveFromSourceListW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(flags: u32, source: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupRemoveFromSourceListW(flags: u32, source: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupRemoveFromSourceListW(::core::mem::transmute(flags), source.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupRemoveInstallSectionFromDiskSpaceListA<'a, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, layoutinfhandle: *const ::core::ffi::c_void, sectionname: Param3, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupRemoveInstallSectionFromDiskSpaceListA(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, layoutinfhandle: *const ::core::ffi::c_void, sectionname: ::windows::core::PCSTR, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupRemoveInstallSectionFromDiskSpaceListA(::core::mem::transmute(diskspace), ::core::mem::transmute(infhandle), ::core::mem::transmute(layoutinfhandle), sectionname.into_param().abi(), ::core::mem::transmute(reserved1), ::core::mem::transmute(reserved2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupRemoveInstallSectionFromDiskSpaceListW<'a, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, layoutinfhandle: *const ::core::ffi::c_void, sectionname: Param3, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupRemoveInstallSectionFromDiskSpaceListW(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, layoutinfhandle: *const ::core::ffi::c_void, sectionname: ::windows::core::PCWSTR, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupRemoveInstallSectionFromDiskSpaceListW(::core::mem::transmute(diskspace), ::core::mem::transmute(infhandle), ::core::mem::transmute(layoutinfhandle), sectionname.into_param().abi(), ::core::mem::transmute(reserved1), ::core::mem::transmute(reserved2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupRemoveSectionFromDiskSpaceListA<'a, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, sectionname: Param3, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupRemoveSectionFromDiskSpaceListA(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, sectionname: ::windows::core::PCSTR, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupRemoveSectionFromDiskSpaceListA(::core::mem::transmute(diskspace), ::core::mem::transmute(infhandle), ::core::mem::transmute(listinfhandle), sectionname.into_param().abi(), ::core::mem::transmute(operation), ::core::mem::transmute(reserved1), ::core::mem::transmute(reserved2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupRemoveSectionFromDiskSpaceListW<'a, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, sectionname: Param3, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupRemoveSectionFromDiskSpaceListW(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, sectionname: ::windows::core::PCWSTR, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupRemoveSectionFromDiskSpaceListW(::core::mem::transmute(diskspace), ::core::mem::transmute(infhandle), ::core::mem::transmute(listinfhandle), sectionname.into_param().abi(), ::core::mem::transmute(operation), ::core::mem::transmute(reserved1), ::core::mem::transmute(reserved2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupRenameErrorA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hwndparent: Param0, dialogtitle: Param1, sourcefile: Param2, targetfile: Param3, win32errorcode: u32, style: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupRenameErrorA(hwndparent: super::super::Foundation::HWND, dialogtitle: ::windows::core::PCSTR, sourcefile: ::windows::core::PCSTR, targetfile: ::windows::core::PCSTR, win32errorcode: u32, style: u32) -> u32;
        }
        ::core::mem::transmute(SetupRenameErrorA(hwndparent.into_param().abi(), dialogtitle.into_param().abi(), sourcefile.into_param().abi(), targetfile.into_param().abi(), ::core::mem::transmute(win32errorcode), ::core::mem::transmute(style)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupRenameErrorW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hwndparent: Param0, dialogtitle: Param1, sourcefile: Param2, targetfile: Param3, win32errorcode: u32, style: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupRenameErrorW(hwndparent: super::super::Foundation::HWND, dialogtitle: ::windows::core::PCWSTR, sourcefile: ::windows::core::PCWSTR, targetfile: ::windows::core::PCWSTR, win32errorcode: u32, style: u32) -> u32;
        }
        ::core::mem::transmute(SetupRenameErrorW(hwndparent.into_param().abi(), dialogtitle.into_param().abi(), sourcefile.into_param().abi(), targetfile.into_param().abi(), ::core::mem::transmute(win32errorcode), ::core::mem::transmute(style)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupScanFileQueueA<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(filequeue: *const ::core::ffi::c_void, flags: u32, window: Param2, callbackroutine: PSP_FILE_CALLBACK_A, callbackcontext: *const ::core::ffi::c_void, result: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupScanFileQueueA(filequeue: *const ::core::ffi::c_void, flags: u32, window: super::super::Foundation::HWND, callbackroutine: ::windows::core::RawPtr, callbackcontext: *const ::core::ffi::c_void, result: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupScanFileQueueA(::core::mem::transmute(filequeue), ::core::mem::transmute(flags), window.into_param().abi(), ::core::mem::transmute(callbackroutine), ::core::mem::transmute(callbackcontext), ::core::mem::transmute(result)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupScanFileQueueW<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(filequeue: *const ::core::ffi::c_void, flags: u32, window: Param2, callbackroutine: PSP_FILE_CALLBACK_W, callbackcontext: *const ::core::ffi::c_void, result: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupScanFileQueueW(filequeue: *const ::core::ffi::c_void, flags: u32, window: super::super::Foundation::HWND, callbackroutine: ::windows::core::RawPtr, callbackcontext: *const ::core::ffi::c_void, result: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupScanFileQueueW(::core::mem::transmute(filequeue), ::core::mem::transmute(flags), window.into_param().abi(), ::core::mem::transmute(callbackroutine), ::core::mem::transmute(callbackcontext), ::core::mem::transmute(result)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupSetDirectoryIdA<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(infhandle: *const ::core::ffi::c_void, id: u32, directory: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupSetDirectoryIdA(infhandle: *const ::core::ffi::c_void, id: u32, directory: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupSetDirectoryIdA(::core::mem::transmute(infhandle), ::core::mem::transmute(id), directory.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupSetDirectoryIdExA<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(infhandle: *const ::core::ffi::c_void, id: u32, directory: Param2, flags: u32, reserved1: u32, reserved2: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupSetDirectoryIdExA(infhandle: *const ::core::ffi::c_void, id: u32, directory: ::windows::core::PCSTR, flags: u32, reserved1: u32, reserved2: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupSetDirectoryIdExA(::core::mem::transmute(infhandle), ::core::mem::transmute(id), directory.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(reserved1), ::core::mem::transmute(reserved2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupSetDirectoryIdExW<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(infhandle: *const ::core::ffi::c_void, id: u32, directory: Param2, flags: u32, reserved1: u32, reserved2: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupSetDirectoryIdExW(infhandle: *const ::core::ffi::c_void, id: u32, directory: ::windows::core::PCWSTR, flags: u32, reserved1: u32, reserved2: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupSetDirectoryIdExW(::core::mem::transmute(infhandle), ::core::mem::transmute(id), directory.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(reserved1), ::core::mem::transmute(reserved2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupSetDirectoryIdW<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(infhandle: *const ::core::ffi::c_void, id: u32, directory: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupSetDirectoryIdW(infhandle: *const ::core::ffi::c_void, id: u32, directory: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupSetDirectoryIdW(::core::mem::transmute(infhandle), ::core::mem::transmute(id), directory.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
#[inline]
pub unsafe fn SetupSetFileQueueAlternatePlatformA<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(queuehandle: *const ::core::ffi::c_void, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, alternatedefaultcatalogfile: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupSetFileQueueAlternatePlatformA(queuehandle: *const ::core::ffi::c_void, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, alternatedefaultcatalogfile: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupSetFileQueueAlternatePlatformA(::core::mem::transmute(queuehandle), ::core::mem::transmute(alternateplatforminfo), alternatedefaultcatalogfile.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
#[inline]
pub unsafe fn SetupSetFileQueueAlternatePlatformW<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(queuehandle: *const ::core::ffi::c_void, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, alternatedefaultcatalogfile: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupSetFileQueueAlternatePlatformW(queuehandle: *const ::core::ffi::c_void, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, alternatedefaultcatalogfile: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupSetFileQueueAlternatePlatformW(::core::mem::transmute(queuehandle), ::core::mem::transmute(alternateplatforminfo), alternatedefaultcatalogfile.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupSetFileQueueFlags(filequeue: *const ::core::ffi::c_void, flagmask: u32, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupSetFileQueueFlags(filequeue: *const ::core::ffi::c_void, flagmask: u32, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupSetFileQueueFlags(::core::mem::transmute(filequeue), ::core::mem::transmute(flagmask), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupSetNonInteractiveMode<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(noninteractiveflag: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupSetNonInteractiveMode(noninteractiveflag: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupSetNonInteractiveMode(noninteractiveflag.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupSetPlatformPathOverrideA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(r#override: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupSetPlatformPathOverrideA(r#override: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupSetPlatformPathOverrideA(r#override.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupSetPlatformPathOverrideW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(r#override: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupSetPlatformPathOverrideW(r#override: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupSetPlatformPathOverrideW(r#override.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupSetSourceListA(flags: u32, sourcelist: &[::windows::core::PSTR]) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupSetSourceListA(flags: u32, sourcelist: *const ::windows::core::PSTR, sourcecount: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupSetSourceListA(::core::mem::transmute(flags), ::core::mem::transmute(::windows::core::as_ptr_or_null(sourcelist)), sourcelist.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupSetSourceListW(flags: u32, sourcelist: &[::windows::core::PWSTR]) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupSetSourceListW(flags: u32, sourcelist: *const ::windows::core::PWSTR, sourcecount: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupSetSourceListW(::core::mem::transmute(flags), ::core::mem::transmute(::windows::core::as_ptr_or_null(sourcelist)), sourcelist.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn SetupSetThreadLogToken(logtoken: u64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupSetThreadLogToken(logtoken: u64);
        }
        SetupSetThreadLogToken(::core::mem::transmute(logtoken))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn SetupTermDefaultQueueCallback(context: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupTermDefaultQueueCallback(context: *const ::core::ffi::c_void);
        }
        SetupTermDefaultQueueCallback(::core::mem::transmute(context))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupTerminateFileLog(fileloghandle: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupTerminateFileLog(fileloghandle: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupTerminateFileLog(::core::mem::transmute(fileloghandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupUninstallNewlyCopiedInfs(filequeue: *const ::core::ffi::c_void, flags: u32, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupUninstallNewlyCopiedInfs(filequeue: *const ::core::ffi::c_void, flags: u32, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupUninstallNewlyCopiedInfs(::core::mem::transmute(filequeue), ::core::mem::transmute(flags), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupUninstallOEMInfA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(inffilename: Param0, flags: u32, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupUninstallOEMInfA(inffilename: ::windows::core::PCSTR, flags: u32, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupUninstallOEMInfA(inffilename.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupUninstallOEMInfW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(inffilename: Param0, flags: u32, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupUninstallOEMInfW(inffilename: ::windows::core::PCWSTR, flags: u32, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupUninstallOEMInfW(inffilename.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
#[inline]
pub unsafe fn SetupVerifyInfFileA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(infname: Param0, altplatforminfo: *const SP_ALTPLATFORM_INFO_V2, infsignerinfo: *mut SP_INF_SIGNER_INFO_V2_A) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupVerifyInfFileA(infname: ::windows::core::PCSTR, altplatforminfo: *const SP_ALTPLATFORM_INFO_V2, infsignerinfo: *mut SP_INF_SIGNER_INFO_V2_A) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupVerifyInfFileA(infname.into_param().abi(), ::core::mem::transmute(altplatforminfo), ::core::mem::transmute(infsignerinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
#[inline]
pub unsafe fn SetupVerifyInfFileW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(infname: Param0, altplatforminfo: *const SP_ALTPLATFORM_INFO_V2, infsignerinfo: *mut SP_INF_SIGNER_INFO_V2_W) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupVerifyInfFileW(infname: ::windows::core::PCWSTR, altplatforminfo: *const SP_ALTPLATFORM_INFO_V2, infsignerinfo: *mut SP_INF_SIGNER_INFO_V2_W) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupVerifyInfFileW(infname.into_param().abi(), ::core::mem::transmute(altplatforminfo), ::core::mem::transmute(infsignerinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn SetupWriteTextLog<'a, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(logtoken: u64, category: u32, flags: u32, messagestr: Param3) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupWriteTextLog(logtoken: u64, category: u32, flags: u32, messagestr: ::windows::core::PCSTR);
        }
        SetupWriteTextLog(::core::mem::transmute(logtoken), ::core::mem::transmute(category), ::core::mem::transmute(flags), messagestr.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn SetupWriteTextLogError<'a, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(logtoken: u64, category: u32, logflags: u32, error: u32, messagestr: Param4) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupWriteTextLogError(logtoken: u64, category: u32, logflags: u32, error: u32, messagestr: ::windows::core::PCSTR);
        }
        SetupWriteTextLogError(::core::mem::transmute(logtoken), ::core::mem::transmute(category), ::core::mem::transmute(logflags), ::core::mem::transmute(error), messagestr.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[inline]
pub unsafe fn SetupWriteTextLogInfLine(logtoken: u64, flags: u32, infhandle: *const ::core::ffi::c_void, context: *const INFCONTEXT) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupWriteTextLogInfLine(logtoken: u64, flags: u32, infhandle: *const ::core::ffi::c_void, context: *const INFCONTEXT);
        }
        SetupWriteTextLogInfLine(::core::mem::transmute(logtoken), ::core::mem::transmute(flags), ::core::mem::transmute(infhandle), ::core::mem::transmute(context))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UpdateDriverForPlugAndPlayDevicesA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hwndparent: Param0, hardwareid: Param1, fullinfpath: Param2, installflags: u32, brebootrequired: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UpdateDriverForPlugAndPlayDevicesA(hwndparent: super::super::Foundation::HWND, hardwareid: ::windows::core::PCSTR, fullinfpath: ::windows::core::PCSTR, installflags: u32, brebootrequired: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(UpdateDriverForPlugAndPlayDevicesA(hwndparent.into_param().abi(), hardwareid.into_param().abi(), fullinfpath.into_param().abi(), ::core::mem::transmute(installflags), ::core::mem::transmute(brebootrequired)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UpdateDriverForPlugAndPlayDevicesW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hwndparent: Param0, hardwareid: Param1, fullinfpath: Param2, installflags: u32, brebootrequired: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UpdateDriverForPlugAndPlayDevicesW(hwndparent: super::super::Foundation::HWND, hardwareid: ::windows::core::PCWSTR, fullinfpath: ::windows::core::PCWSTR, installflags: u32, brebootrequired: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(UpdateDriverForPlugAndPlayDevicesW(hwndparent.into_param().abi(), hardwareid.into_param().abi(), fullinfpath.into_param().abi(), ::core::mem::transmute(installflags), ::core::mem::transmute(brebootrequired)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fDD_BYTE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fDD_BYTE_AND_WORD: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fDD_BusMaster: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fDD_DWORD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fDD_NoBusMaster: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fDD_TypeA: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fDD_TypeB: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fDD_TypeF: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fDD_TypeStandard: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fDD_WORD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fIOD_10_BIT_DECODE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fIOD_12_BIT_DECODE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fIOD_16_BIT_DECODE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fIOD_DECODE: u32 = 252u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fIOD_IO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fIOD_Memory: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fIOD_PASSIVE_DECODE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fIOD_PORT_BAR: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fIOD_POSITIVE_DECODE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fIOD_PortType: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fIOD_WINDOW_DECODE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fIRQD_Edge: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fIRQD_Exclusive: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fIRQD_Level: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fIRQD_Level_Bit: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fIRQD_Share: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fIRQD_Share_Bit: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_24: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_32: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_32_24: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_Cacheable: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_CombinedWrite: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_CombinedWriteAllowed: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_CombinedWriteDisallowed: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_MEMORY_BAR: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_MemoryType: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_NonCacheable: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_Pref: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_PrefetchAllowed: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_PrefetchDisallowed: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_Prefetchable: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_RAM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_ROM: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_ReadAllowed: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_ReadDisallowed: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_Readable: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_WINDOW_DECODE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_ATTRIBUTES_PER_WINDOW: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_IO1_16: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_IO1_SRC_16: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_IO1_WS_16: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_IO1_ZW_8: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_IO2_16: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_IO2_SRC_16: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_IO2_WS_16: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_IO2_ZW_8: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_IO_16: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_IO_8: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_IO_SRC_16: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_IO_WS_16: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_IO_ZW_8: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_MEM1_16: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_MEM1_A: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_MEM1_WS_ONE: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_MEM1_WS_THREE: u32 = 50331648u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_MEM1_WS_TWO: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_MEM2_16: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_MEM2_A: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_MEM2_WS_ONE: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_MEM2_WS_THREE: u32 = 805306368u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_MEM2_WS_TWO: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_MEM_16: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_MEM_8: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_MEM_A: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_MEM_WS_ONE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_MEM_WS_THREE: u32 = 768u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_MEM_WS_TWO: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPMF_AUDIO_ENABLE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const mDD_BusMaster: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const mDD_Type: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const mDD_Width: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const mIRQD_Edge_Level: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const mIRQD_Share: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const mMD_32_24: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const mMD_Cacheable: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const mMD_CombinedWrite: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const mMD_MemoryType: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const mMD_Prefetchable: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const mMD_Readable: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const mPCD_IO_8_16: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const mPCD_MEM1_WS: u32 = 50331648u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const mPCD_MEM2_WS: u32 = 805306368u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const mPCD_MEM_8_16: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const mPCD_MEM_A_C: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const mPCD_MEM_WS: u32 = 768u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const mPMF_AUDIO_ENABLE: u32 = 8u32;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
