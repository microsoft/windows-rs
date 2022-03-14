#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const CF_MSFAXSRV_DEVICE_ID: &'static str = "FAXSRV_DeviceID";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const CF_MSFAXSRV_FSP_GUID: &'static str = "FAXSRV_FSPGuid";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const CF_MSFAXSRV_ROUTEEXT_NAME: &'static str = "FAXSRV_RoutingExtName";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const CF_MSFAXSRV_ROUTING_METHOD_GUID: &'static str = "FAXSRV_RoutingMethodGuid";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const CF_MSFAXSRV_SERVER_NAME: &'static str = "FAXSRV_ServerName";
pub const CLSID_Sti: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb323f8e0_2e68_11d0_90ea_00aa0060f86c);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CanSendToFaxRecipient() -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CanSendToFaxRecipient() -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CanSendToFaxRecipient())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WIA_DeviceType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x6bdd1fc6_810f_11d0_bec7_08002be2092f), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WIA_USDClassId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x6bdd1fc6_810f_11d0_bec7_08002be2092f), pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAXDEVRECEIVE_SIZE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAXDEVREPORTSTATUS_SIZE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FAXROUTE_ENABLE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const QUERY_STATUS: FAXROUTE_ENABLE = FAXROUTE_ENABLE(-1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STATUS_DISABLE: FAXROUTE_ENABLE = FAXROUTE_ENABLE(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STATUS_ENABLE: FAXROUTE_ENABLE = FAXROUTE_ENABLE(1i32);
impl ::core::marker::Copy for FAXROUTE_ENABLE {}
impl ::core::clone::Clone for FAXROUTE_ENABLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAXROUTE_ENABLE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FAXROUTE_ENABLE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FAXROUTE_ENABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAXROUTE_ENABLE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FAX_ACCESS_RIGHTS_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const farSUBMIT_LOW: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const farSUBMIT_NORMAL: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const farSUBMIT_HIGH: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(4i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const farQUERY_JOBS: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(8i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const farMANAGE_JOBS: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(16i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const farQUERY_CONFIG: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(32i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const farMANAGE_CONFIG: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(64i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const farQUERY_IN_ARCHIVE: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(128i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const farMANAGE_IN_ARCHIVE: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(256i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const farQUERY_OUT_ARCHIVE: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(512i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const farMANAGE_OUT_ARCHIVE: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(1024i32);
impl ::core::marker::Copy for FAX_ACCESS_RIGHTS_ENUM {}
impl ::core::clone::Clone for FAX_ACCESS_RIGHTS_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_ACCESS_RIGHTS_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FAX_ACCESS_RIGHTS_ENUM {
    type Abi = Self;
}
impl ::core::fmt::Debug for FAX_ACCESS_RIGHTS_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_ACCESS_RIGHTS_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FAX_ACCESS_RIGHTS_ENUM_2(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const far2SUBMIT_LOW: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const far2SUBMIT_NORMAL: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const far2SUBMIT_HIGH: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(4i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const far2QUERY_OUT_JOBS: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(8i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const far2MANAGE_OUT_JOBS: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(16i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const far2QUERY_CONFIG: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(32i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const far2MANAGE_CONFIG: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(64i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const far2QUERY_ARCHIVES: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(128i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const far2MANAGE_ARCHIVES: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(256i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const far2MANAGE_RECEIVE_FOLDER: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(512i32);
impl ::core::marker::Copy for FAX_ACCESS_RIGHTS_ENUM_2 {}
impl ::core::clone::Clone for FAX_ACCESS_RIGHTS_ENUM_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_ACCESS_RIGHTS_ENUM_2 {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FAX_ACCESS_RIGHTS_ENUM_2 {
    type Abi = Self;
}
impl ::core::fmt::Debug for FAX_ACCESS_RIGHTS_ENUM_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_ACCESS_RIGHTS_ENUM_2").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FAX_ACCOUNT_EVENTS_TYPE_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const faetNONE: FAX_ACCOUNT_EVENTS_TYPE_ENUM = FAX_ACCOUNT_EVENTS_TYPE_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const faetIN_QUEUE: FAX_ACCOUNT_EVENTS_TYPE_ENUM = FAX_ACCOUNT_EVENTS_TYPE_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const faetOUT_QUEUE: FAX_ACCOUNT_EVENTS_TYPE_ENUM = FAX_ACCOUNT_EVENTS_TYPE_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const faetIN_ARCHIVE: FAX_ACCOUNT_EVENTS_TYPE_ENUM = FAX_ACCOUNT_EVENTS_TYPE_ENUM(4i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const faetOUT_ARCHIVE: FAX_ACCOUNT_EVENTS_TYPE_ENUM = FAX_ACCOUNT_EVENTS_TYPE_ENUM(8i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const faetFXSSVC_ENDED: FAX_ACCOUNT_EVENTS_TYPE_ENUM = FAX_ACCOUNT_EVENTS_TYPE_ENUM(16i32);
impl ::core::marker::Copy for FAX_ACCOUNT_EVENTS_TYPE_ENUM {}
impl ::core::clone::Clone for FAX_ACCOUNT_EVENTS_TYPE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_ACCOUNT_EVENTS_TYPE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FAX_ACCOUNT_EVENTS_TYPE_ENUM {
    type Abi = Self;
}
impl ::core::fmt::Debug for FAX_ACCOUNT_EVENTS_TYPE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_ACCOUNT_EVENTS_TYPE_ENUM").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_CONFIGURATIONA {
    pub SizeOfStruct: u32,
    pub Retries: u32,
    pub RetryDelay: u32,
    pub DirtyDays: u32,
    pub Branding: super::super::Foundation::BOOL,
    pub UseDeviceTsid: super::super::Foundation::BOOL,
    pub ServerCp: super::super::Foundation::BOOL,
    pub PauseServerQueue: super::super::Foundation::BOOL,
    pub StartCheapTime: FAX_TIME,
    pub StopCheapTime: FAX_TIME,
    pub ArchiveOutgoingFaxes: super::super::Foundation::BOOL,
    pub ArchiveDirectory: ::windows::core::PCSTR,
    pub Reserved: ::windows::core::PCSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_CONFIGURATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_CONFIGURATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FAX_CONFIGURATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_CONFIGURATIONA")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("Retries", &self.Retries)
            .field("RetryDelay", &self.RetryDelay)
            .field("DirtyDays", &self.DirtyDays)
            .field("Branding", &self.Branding)
            .field("UseDeviceTsid", &self.UseDeviceTsid)
            .field("ServerCp", &self.ServerCp)
            .field("PauseServerQueue", &self.PauseServerQueue)
            .field("StartCheapTime", &self.StartCheapTime)
            .field("StopCheapTime", &self.StopCheapTime)
            .field("ArchiveOutgoingFaxes", &self.ArchiveOutgoingFaxes)
            .field("ArchiveDirectory", &self.ArchiveDirectory)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FAX_CONFIGURATIONA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FAX_CONFIGURATIONA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FAX_CONFIGURATIONA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FAX_CONFIGURATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FAX_CONFIGURATIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_CONFIGURATIONW {
    pub SizeOfStruct: u32,
    pub Retries: u32,
    pub RetryDelay: u32,
    pub DirtyDays: u32,
    pub Branding: super::super::Foundation::BOOL,
    pub UseDeviceTsid: super::super::Foundation::BOOL,
    pub ServerCp: super::super::Foundation::BOOL,
    pub PauseServerQueue: super::super::Foundation::BOOL,
    pub StartCheapTime: FAX_TIME,
    pub StopCheapTime: FAX_TIME,
    pub ArchiveOutgoingFaxes: super::super::Foundation::BOOL,
    pub ArchiveDirectory: ::windows::core::PCWSTR,
    pub Reserved: ::windows::core::PCWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_CONFIGURATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_CONFIGURATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FAX_CONFIGURATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_CONFIGURATIONW")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("Retries", &self.Retries)
            .field("RetryDelay", &self.RetryDelay)
            .field("DirtyDays", &self.DirtyDays)
            .field("Branding", &self.Branding)
            .field("UseDeviceTsid", &self.UseDeviceTsid)
            .field("ServerCp", &self.ServerCp)
            .field("PauseServerQueue", &self.PauseServerQueue)
            .field("StartCheapTime", &self.StartCheapTime)
            .field("StopCheapTime", &self.StopCheapTime)
            .field("ArchiveOutgoingFaxes", &self.ArchiveOutgoingFaxes)
            .field("ArchiveDirectory", &self.ArchiveDirectory)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FAX_CONFIGURATIONW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FAX_CONFIGURATIONW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FAX_CONFIGURATIONW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FAX_CONFIGURATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FAX_CONFIGURATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_CONFIG_QUERY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_CONFIG_SET: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct FAX_CONTEXT_INFOA {
    pub SizeOfStruct: u32,
    pub hDC: super::super::Graphics::Gdi::HDC,
    pub ServerName: [super::super::Foundation::CHAR; 16],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for FAX_CONTEXT_INFOA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for FAX_CONTEXT_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for FAX_CONTEXT_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_CONTEXT_INFOA").field("SizeOfStruct", &self.SizeOfStruct).field("hDC", &self.hDC).field("ServerName", &self.ServerName).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for FAX_CONTEXT_INFOA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for FAX_CONTEXT_INFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FAX_CONTEXT_INFOA>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for FAX_CONTEXT_INFOA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for FAX_CONTEXT_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct FAX_CONTEXT_INFOW {
    pub SizeOfStruct: u32,
    pub hDC: super::super::Graphics::Gdi::HDC,
    pub ServerName: [u16; 16],
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for FAX_CONTEXT_INFOW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for FAX_CONTEXT_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for FAX_CONTEXT_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_CONTEXT_INFOW").field("SizeOfStruct", &self.SizeOfStruct).field("hDC", &self.hDC).field("ServerName", &self.ServerName).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows::core::Abi for FAX_CONTEXT_INFOW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for FAX_CONTEXT_INFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FAX_CONTEXT_INFOW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for FAX_CONTEXT_INFOW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for FAX_CONTEXT_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_COVERPAGE_INFOA {
    pub SizeOfStruct: u32,
    pub CoverPageName: ::windows::core::PCSTR,
    pub UseServerCoverPage: super::super::Foundation::BOOL,
    pub RecName: ::windows::core::PCSTR,
    pub RecFaxNumber: ::windows::core::PCSTR,
    pub RecCompany: ::windows::core::PCSTR,
    pub RecStreetAddress: ::windows::core::PCSTR,
    pub RecCity: ::windows::core::PCSTR,
    pub RecState: ::windows::core::PCSTR,
    pub RecZip: ::windows::core::PCSTR,
    pub RecCountry: ::windows::core::PCSTR,
    pub RecTitle: ::windows::core::PCSTR,
    pub RecDepartment: ::windows::core::PCSTR,
    pub RecOfficeLocation: ::windows::core::PCSTR,
    pub RecHomePhone: ::windows::core::PCSTR,
    pub RecOfficePhone: ::windows::core::PCSTR,
    pub SdrName: ::windows::core::PCSTR,
    pub SdrFaxNumber: ::windows::core::PCSTR,
    pub SdrCompany: ::windows::core::PCSTR,
    pub SdrAddress: ::windows::core::PCSTR,
    pub SdrTitle: ::windows::core::PCSTR,
    pub SdrDepartment: ::windows::core::PCSTR,
    pub SdrOfficeLocation: ::windows::core::PCSTR,
    pub SdrHomePhone: ::windows::core::PCSTR,
    pub SdrOfficePhone: ::windows::core::PCSTR,
    pub Note: ::windows::core::PCSTR,
    pub Subject: ::windows::core::PCSTR,
    pub TimeSent: super::super::Foundation::SYSTEMTIME,
    pub PageCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_COVERPAGE_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_COVERPAGE_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FAX_COVERPAGE_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_COVERPAGE_INFOA")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("CoverPageName", &self.CoverPageName)
            .field("UseServerCoverPage", &self.UseServerCoverPage)
            .field("RecName", &self.RecName)
            .field("RecFaxNumber", &self.RecFaxNumber)
            .field("RecCompany", &self.RecCompany)
            .field("RecStreetAddress", &self.RecStreetAddress)
            .field("RecCity", &self.RecCity)
            .field("RecState", &self.RecState)
            .field("RecZip", &self.RecZip)
            .field("RecCountry", &self.RecCountry)
            .field("RecTitle", &self.RecTitle)
            .field("RecDepartment", &self.RecDepartment)
            .field("RecOfficeLocation", &self.RecOfficeLocation)
            .field("RecHomePhone", &self.RecHomePhone)
            .field("RecOfficePhone", &self.RecOfficePhone)
            .field("SdrName", &self.SdrName)
            .field("SdrFaxNumber", &self.SdrFaxNumber)
            .field("SdrCompany", &self.SdrCompany)
            .field("SdrAddress", &self.SdrAddress)
            .field("SdrTitle", &self.SdrTitle)
            .field("SdrDepartment", &self.SdrDepartment)
            .field("SdrOfficeLocation", &self.SdrOfficeLocation)
            .field("SdrHomePhone", &self.SdrHomePhone)
            .field("SdrOfficePhone", &self.SdrOfficePhone)
            .field("Note", &self.Note)
            .field("Subject", &self.Subject)
            .field("TimeSent", &self.TimeSent)
            .field("PageCount", &self.PageCount)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FAX_COVERPAGE_INFOA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FAX_COVERPAGE_INFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FAX_COVERPAGE_INFOA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FAX_COVERPAGE_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FAX_COVERPAGE_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_COVERPAGE_INFOW {
    pub SizeOfStruct: u32,
    pub CoverPageName: ::windows::core::PCWSTR,
    pub UseServerCoverPage: super::super::Foundation::BOOL,
    pub RecName: ::windows::core::PCWSTR,
    pub RecFaxNumber: ::windows::core::PCWSTR,
    pub RecCompany: ::windows::core::PCWSTR,
    pub RecStreetAddress: ::windows::core::PCWSTR,
    pub RecCity: ::windows::core::PCWSTR,
    pub RecState: ::windows::core::PCWSTR,
    pub RecZip: ::windows::core::PCWSTR,
    pub RecCountry: ::windows::core::PCWSTR,
    pub RecTitle: ::windows::core::PCWSTR,
    pub RecDepartment: ::windows::core::PCWSTR,
    pub RecOfficeLocation: ::windows::core::PCWSTR,
    pub RecHomePhone: ::windows::core::PCWSTR,
    pub RecOfficePhone: ::windows::core::PCWSTR,
    pub SdrName: ::windows::core::PCWSTR,
    pub SdrFaxNumber: ::windows::core::PCWSTR,
    pub SdrCompany: ::windows::core::PCWSTR,
    pub SdrAddress: ::windows::core::PCWSTR,
    pub SdrTitle: ::windows::core::PCWSTR,
    pub SdrDepartment: ::windows::core::PCWSTR,
    pub SdrOfficeLocation: ::windows::core::PCWSTR,
    pub SdrHomePhone: ::windows::core::PCWSTR,
    pub SdrOfficePhone: ::windows::core::PCWSTR,
    pub Note: ::windows::core::PCWSTR,
    pub Subject: ::windows::core::PCWSTR,
    pub TimeSent: super::super::Foundation::SYSTEMTIME,
    pub PageCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_COVERPAGE_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_COVERPAGE_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FAX_COVERPAGE_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_COVERPAGE_INFOW")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("CoverPageName", &self.CoverPageName)
            .field("UseServerCoverPage", &self.UseServerCoverPage)
            .field("RecName", &self.RecName)
            .field("RecFaxNumber", &self.RecFaxNumber)
            .field("RecCompany", &self.RecCompany)
            .field("RecStreetAddress", &self.RecStreetAddress)
            .field("RecCity", &self.RecCity)
            .field("RecState", &self.RecState)
            .field("RecZip", &self.RecZip)
            .field("RecCountry", &self.RecCountry)
            .field("RecTitle", &self.RecTitle)
            .field("RecDepartment", &self.RecDepartment)
            .field("RecOfficeLocation", &self.RecOfficeLocation)
            .field("RecHomePhone", &self.RecHomePhone)
            .field("RecOfficePhone", &self.RecOfficePhone)
            .field("SdrName", &self.SdrName)
            .field("SdrFaxNumber", &self.SdrFaxNumber)
            .field("SdrCompany", &self.SdrCompany)
            .field("SdrAddress", &self.SdrAddress)
            .field("SdrTitle", &self.SdrTitle)
            .field("SdrDepartment", &self.SdrDepartment)
            .field("SdrOfficeLocation", &self.SdrOfficeLocation)
            .field("SdrHomePhone", &self.SdrHomePhone)
            .field("SdrOfficePhone", &self.SdrOfficePhone)
            .field("Note", &self.Note)
            .field("Subject", &self.Subject)
            .field("TimeSent", &self.TimeSent)
            .field("PageCount", &self.PageCount)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FAX_COVERPAGE_INFOW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FAX_COVERPAGE_INFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FAX_COVERPAGE_INFOW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FAX_COVERPAGE_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FAX_COVERPAGE_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FAX_COVERPAGE_TYPE_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fcptNONE: FAX_COVERPAGE_TYPE_ENUM = FAX_COVERPAGE_TYPE_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fcptLOCAL: FAX_COVERPAGE_TYPE_ENUM = FAX_COVERPAGE_TYPE_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fcptSERVER: FAX_COVERPAGE_TYPE_ENUM = FAX_COVERPAGE_TYPE_ENUM(2i32);
impl ::core::marker::Copy for FAX_COVERPAGE_TYPE_ENUM {}
impl ::core::clone::Clone for FAX_COVERPAGE_TYPE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_COVERPAGE_TYPE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FAX_COVERPAGE_TYPE_ENUM {
    type Abi = Self;
}
impl ::core::fmt::Debug for FAX_COVERPAGE_TYPE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_COVERPAGE_TYPE_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FAX_DEVICE_RECEIVE_MODE_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fdrmNO_ANSWER: FAX_DEVICE_RECEIVE_MODE_ENUM = FAX_DEVICE_RECEIVE_MODE_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fdrmAUTO_ANSWER: FAX_DEVICE_RECEIVE_MODE_ENUM = FAX_DEVICE_RECEIVE_MODE_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fdrmMANUAL_ANSWER: FAX_DEVICE_RECEIVE_MODE_ENUM = FAX_DEVICE_RECEIVE_MODE_ENUM(2i32);
impl ::core::marker::Copy for FAX_DEVICE_RECEIVE_MODE_ENUM {}
impl ::core::clone::Clone for FAX_DEVICE_RECEIVE_MODE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_DEVICE_RECEIVE_MODE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FAX_DEVICE_RECEIVE_MODE_ENUM {
    type Abi = Self;
}
impl ::core::fmt::Debug for FAX_DEVICE_RECEIVE_MODE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_DEVICE_RECEIVE_MODE_ENUM").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_DEVICE_STATUSA {
    pub SizeOfStruct: u32,
    pub CallerId: ::windows::core::PCSTR,
    pub Csid: ::windows::core::PCSTR,
    pub CurrentPage: u32,
    pub DeviceId: u32,
    pub DeviceName: ::windows::core::PCSTR,
    pub DocumentName: ::windows::core::PCSTR,
    pub JobType: u32,
    pub PhoneNumber: ::windows::core::PCSTR,
    pub RoutingString: ::windows::core::PCSTR,
    pub SenderName: ::windows::core::PCSTR,
    pub RecipientName: ::windows::core::PCSTR,
    pub Size: u32,
    pub StartTime: super::super::Foundation::FILETIME,
    pub Status: u32,
    pub StatusString: ::windows::core::PCSTR,
    pub SubmittedTime: super::super::Foundation::FILETIME,
    pub TotalPages: u32,
    pub Tsid: ::windows::core::PCSTR,
    pub UserName: ::windows::core::PCSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_DEVICE_STATUSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_DEVICE_STATUSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FAX_DEVICE_STATUSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_DEVICE_STATUSA")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("CallerId", &self.CallerId)
            .field("Csid", &self.Csid)
            .field("CurrentPage", &self.CurrentPage)
            .field("DeviceId", &self.DeviceId)
            .field("DeviceName", &self.DeviceName)
            .field("DocumentName", &self.DocumentName)
            .field("JobType", &self.JobType)
            .field("PhoneNumber", &self.PhoneNumber)
            .field("RoutingString", &self.RoutingString)
            .field("SenderName", &self.SenderName)
            .field("RecipientName", &self.RecipientName)
            .field("Size", &self.Size)
            .field("StartTime", &self.StartTime)
            .field("Status", &self.Status)
            .field("StatusString", &self.StatusString)
            .field("SubmittedTime", &self.SubmittedTime)
            .field("TotalPages", &self.TotalPages)
            .field("Tsid", &self.Tsid)
            .field("UserName", &self.UserName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FAX_DEVICE_STATUSA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FAX_DEVICE_STATUSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FAX_DEVICE_STATUSA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FAX_DEVICE_STATUSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FAX_DEVICE_STATUSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_DEVICE_STATUSW {
    pub SizeOfStruct: u32,
    pub CallerId: ::windows::core::PCWSTR,
    pub Csid: ::windows::core::PCWSTR,
    pub CurrentPage: u32,
    pub DeviceId: u32,
    pub DeviceName: ::windows::core::PCWSTR,
    pub DocumentName: ::windows::core::PCWSTR,
    pub JobType: u32,
    pub PhoneNumber: ::windows::core::PCWSTR,
    pub RoutingString: ::windows::core::PCWSTR,
    pub SenderName: ::windows::core::PCWSTR,
    pub RecipientName: ::windows::core::PCWSTR,
    pub Size: u32,
    pub StartTime: super::super::Foundation::FILETIME,
    pub Status: u32,
    pub StatusString: ::windows::core::PCWSTR,
    pub SubmittedTime: super::super::Foundation::FILETIME,
    pub TotalPages: u32,
    pub Tsid: ::windows::core::PCWSTR,
    pub UserName: ::windows::core::PCWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_DEVICE_STATUSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_DEVICE_STATUSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FAX_DEVICE_STATUSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_DEVICE_STATUSW")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("CallerId", &self.CallerId)
            .field("Csid", &self.Csid)
            .field("CurrentPage", &self.CurrentPage)
            .field("DeviceId", &self.DeviceId)
            .field("DeviceName", &self.DeviceName)
            .field("DocumentName", &self.DocumentName)
            .field("JobType", &self.JobType)
            .field("PhoneNumber", &self.PhoneNumber)
            .field("RoutingString", &self.RoutingString)
            .field("SenderName", &self.SenderName)
            .field("RecipientName", &self.RecipientName)
            .field("Size", &self.Size)
            .field("StartTime", &self.StartTime)
            .field("Status", &self.Status)
            .field("StatusString", &self.StatusString)
            .field("SubmittedTime", &self.SubmittedTime)
            .field("TotalPages", &self.TotalPages)
            .field("Tsid", &self.Tsid)
            .field("UserName", &self.UserName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FAX_DEVICE_STATUSW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FAX_DEVICE_STATUSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FAX_DEVICE_STATUSW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FAX_DEVICE_STATUSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FAX_DEVICE_STATUSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct FAX_DEV_STATUS {
    pub SizeOfStruct: u32,
    pub StatusId: u32,
    pub StringId: u32,
    pub PageCount: u32,
    pub CSI: ::windows::core::PWSTR,
    pub CallerId: ::windows::core::PWSTR,
    pub RoutingInfo: ::windows::core::PWSTR,
    pub ErrorCode: u32,
    pub Reserved: [u32; 3],
}
impl ::core::marker::Copy for FAX_DEV_STATUS {}
impl ::core::clone::Clone for FAX_DEV_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FAX_DEV_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_DEV_STATUS").field("SizeOfStruct", &self.SizeOfStruct).field("StatusId", &self.StatusId).field("StringId", &self.StringId).field("PageCount", &self.PageCount).field("CSI", &self.CSI).field("CallerId", &self.CallerId).field("RoutingInfo", &self.RoutingInfo).field("ErrorCode", &self.ErrorCode).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for FAX_DEV_STATUS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FAX_DEV_STATUS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FAX_DEV_STATUS>()) == 0 }
    }
}
impl ::core::cmp::Eq for FAX_DEV_STATUS {}
impl ::core::default::Default for FAX_DEV_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FAX_ENUM_DELIVERY_REPORT_TYPES(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const DRT_NONE: FAX_ENUM_DELIVERY_REPORT_TYPES = FAX_ENUM_DELIVERY_REPORT_TYPES(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const DRT_EMAIL: FAX_ENUM_DELIVERY_REPORT_TYPES = FAX_ENUM_DELIVERY_REPORT_TYPES(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const DRT_INBOX: FAX_ENUM_DELIVERY_REPORT_TYPES = FAX_ENUM_DELIVERY_REPORT_TYPES(2i32);
impl ::core::marker::Copy for FAX_ENUM_DELIVERY_REPORT_TYPES {}
impl ::core::clone::Clone for FAX_ENUM_DELIVERY_REPORT_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_ENUM_DELIVERY_REPORT_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FAX_ENUM_DELIVERY_REPORT_TYPES {
    type Abi = Self;
}
impl ::core::fmt::Debug for FAX_ENUM_DELIVERY_REPORT_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_ENUM_DELIVERY_REPORT_TYPES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FAX_ENUM_DEVICE_ID_SOURCE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const DEV_ID_SRC_FAX: FAX_ENUM_DEVICE_ID_SOURCE = FAX_ENUM_DEVICE_ID_SOURCE(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const DEV_ID_SRC_TAPI: FAX_ENUM_DEVICE_ID_SOURCE = FAX_ENUM_DEVICE_ID_SOURCE(1i32);
impl ::core::marker::Copy for FAX_ENUM_DEVICE_ID_SOURCE {}
impl ::core::clone::Clone for FAX_ENUM_DEVICE_ID_SOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_ENUM_DEVICE_ID_SOURCE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FAX_ENUM_DEVICE_ID_SOURCE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FAX_ENUM_DEVICE_ID_SOURCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_ENUM_DEVICE_ID_SOURCE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FAX_ENUM_JOB_COMMANDS(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JC_UNKNOWN: FAX_ENUM_JOB_COMMANDS = FAX_ENUM_JOB_COMMANDS(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JC_DELETE: FAX_ENUM_JOB_COMMANDS = FAX_ENUM_JOB_COMMANDS(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JC_PAUSE: FAX_ENUM_JOB_COMMANDS = FAX_ENUM_JOB_COMMANDS(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JC_RESUME: FAX_ENUM_JOB_COMMANDS = FAX_ENUM_JOB_COMMANDS(3i32);
impl ::core::marker::Copy for FAX_ENUM_JOB_COMMANDS {}
impl ::core::clone::Clone for FAX_ENUM_JOB_COMMANDS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_ENUM_JOB_COMMANDS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FAX_ENUM_JOB_COMMANDS {
    type Abi = Self;
}
impl ::core::fmt::Debug for FAX_ENUM_JOB_COMMANDS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_ENUM_JOB_COMMANDS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FAX_ENUM_JOB_SEND_ATTRIBUTES(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JSA_NOW: FAX_ENUM_JOB_SEND_ATTRIBUTES = FAX_ENUM_JOB_SEND_ATTRIBUTES(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JSA_SPECIFIC_TIME: FAX_ENUM_JOB_SEND_ATTRIBUTES = FAX_ENUM_JOB_SEND_ATTRIBUTES(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JSA_DISCOUNT_PERIOD: FAX_ENUM_JOB_SEND_ATTRIBUTES = FAX_ENUM_JOB_SEND_ATTRIBUTES(2i32);
impl ::core::marker::Copy for FAX_ENUM_JOB_SEND_ATTRIBUTES {}
impl ::core::clone::Clone for FAX_ENUM_JOB_SEND_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_ENUM_JOB_SEND_ATTRIBUTES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FAX_ENUM_JOB_SEND_ATTRIBUTES {
    type Abi = Self;
}
impl ::core::fmt::Debug for FAX_ENUM_JOB_SEND_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_ENUM_JOB_SEND_ATTRIBUTES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FAX_ENUM_LOG_CATEGORIES(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAXLOG_CATEGORY_INIT: FAX_ENUM_LOG_CATEGORIES = FAX_ENUM_LOG_CATEGORIES(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAXLOG_CATEGORY_OUTBOUND: FAX_ENUM_LOG_CATEGORIES = FAX_ENUM_LOG_CATEGORIES(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAXLOG_CATEGORY_INBOUND: FAX_ENUM_LOG_CATEGORIES = FAX_ENUM_LOG_CATEGORIES(3i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAXLOG_CATEGORY_UNKNOWN: FAX_ENUM_LOG_CATEGORIES = FAX_ENUM_LOG_CATEGORIES(4i32);
impl ::core::marker::Copy for FAX_ENUM_LOG_CATEGORIES {}
impl ::core::clone::Clone for FAX_ENUM_LOG_CATEGORIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_ENUM_LOG_CATEGORIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FAX_ENUM_LOG_CATEGORIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for FAX_ENUM_LOG_CATEGORIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_ENUM_LOG_CATEGORIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FAX_ENUM_LOG_LEVELS(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAXLOG_LEVEL_NONE: FAX_ENUM_LOG_LEVELS = FAX_ENUM_LOG_LEVELS(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAXLOG_LEVEL_MIN: FAX_ENUM_LOG_LEVELS = FAX_ENUM_LOG_LEVELS(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAXLOG_LEVEL_MED: FAX_ENUM_LOG_LEVELS = FAX_ENUM_LOG_LEVELS(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAXLOG_LEVEL_MAX: FAX_ENUM_LOG_LEVELS = FAX_ENUM_LOG_LEVELS(3i32);
impl ::core::marker::Copy for FAX_ENUM_LOG_LEVELS {}
impl ::core::clone::Clone for FAX_ENUM_LOG_LEVELS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_ENUM_LOG_LEVELS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FAX_ENUM_LOG_LEVELS {
    type Abi = Self;
}
impl ::core::fmt::Debug for FAX_ENUM_LOG_LEVELS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_ENUM_LOG_LEVELS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FAX_ENUM_PORT_OPEN_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const PORT_OPEN_QUERY: FAX_ENUM_PORT_OPEN_TYPE = FAX_ENUM_PORT_OPEN_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const PORT_OPEN_MODIFY: FAX_ENUM_PORT_OPEN_TYPE = FAX_ENUM_PORT_OPEN_TYPE(2i32);
impl ::core::marker::Copy for FAX_ENUM_PORT_OPEN_TYPE {}
impl ::core::clone::Clone for FAX_ENUM_PORT_OPEN_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_ENUM_PORT_OPEN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FAX_ENUM_PORT_OPEN_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FAX_ENUM_PORT_OPEN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_ENUM_PORT_OPEN_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_BAD_GROUP_CONFIGURATION: i32 = 7003i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_DEVICE_NUM_LIMIT_EXCEEDED: i32 = 7010i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_DIRECTORY_IN_USE: i32 = 7007i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_END: i32 = 7013i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_FILE_ACCESS_DENIED: i32 = 7008i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_GROUP_IN_USE: i32 = 7004i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_GROUP_NOT_FOUND: i32 = 7002i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_MESSAGE_NOT_FOUND: i32 = 7009i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_NOT_NTFS: i32 = 7006i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_NOT_SUPPORTED_ON_THIS_SKU: i32 = 7011i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_RECIPIENTS_LIMIT: i32 = 7013i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_RULE_NOT_FOUND: i32 = 7005i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_SRV_OUTOFMEMORY: i32 = 7001i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_START: i32 = 7001i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_ERR_VERSION_MISMATCH: i32 = 7012i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_EVENTA {
    pub SizeOfStruct: u32,
    pub TimeStamp: super::super::Foundation::FILETIME,
    pub DeviceId: u32,
    pub EventId: u32,
    pub JobId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_EVENTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_EVENTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FAX_EVENTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_EVENTA").field("SizeOfStruct", &self.SizeOfStruct).field("TimeStamp", &self.TimeStamp).field("DeviceId", &self.DeviceId).field("EventId", &self.EventId).field("JobId", &self.JobId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FAX_EVENTA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FAX_EVENTA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FAX_EVENTA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FAX_EVENTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FAX_EVENTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_EVENTW {
    pub SizeOfStruct: u32,
    pub TimeStamp: super::super::Foundation::FILETIME,
    pub DeviceId: u32,
    pub EventId: u32,
    pub JobId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_EVENTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_EVENTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FAX_EVENTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_EVENTW").field("SizeOfStruct", &self.SizeOfStruct).field("TimeStamp", &self.TimeStamp).field("DeviceId", &self.DeviceId).field("EventId", &self.EventId).field("JobId", &self.JobId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FAX_EVENTW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FAX_EVENTW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FAX_EVENTW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FAX_EVENTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FAX_EVENTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_BAD_GROUP_CONFIGURATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147214501i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_DEVICE_NUM_LIMIT_EXCEEDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147214494i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_DIRECTORY_IN_USE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147214497i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_FILE_ACCESS_DENIED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147214496i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_GROUP_IN_USE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147214500i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_GROUP_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147214502i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_MESSAGE_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147214495i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_NOT_NTFS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147214498i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_NOT_SUPPORTED_ON_THIS_SKU: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147214493i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_RECIPIENTS_LIMIT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147214491i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_RULE_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147214499i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_SRV_OUTOFMEMORY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147214503i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_E_VERSION_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147214492i32);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct FAX_GLOBAL_ROUTING_INFOA {
    pub SizeOfStruct: u32,
    pub Priority: u32,
    pub Guid: ::windows::core::PCSTR,
    pub FriendlyName: ::windows::core::PCSTR,
    pub FunctionName: ::windows::core::PCSTR,
    pub ExtensionImageName: ::windows::core::PCSTR,
    pub ExtensionFriendlyName: ::windows::core::PCSTR,
}
impl ::core::marker::Copy for FAX_GLOBAL_ROUTING_INFOA {}
impl ::core::clone::Clone for FAX_GLOBAL_ROUTING_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FAX_GLOBAL_ROUTING_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_GLOBAL_ROUTING_INFOA").field("SizeOfStruct", &self.SizeOfStruct).field("Priority", &self.Priority).field("Guid", &self.Guid).field("FriendlyName", &self.FriendlyName).field("FunctionName", &self.FunctionName).field("ExtensionImageName", &self.ExtensionImageName).field("ExtensionFriendlyName", &self.ExtensionFriendlyName).finish()
    }
}
unsafe impl ::windows::core::Abi for FAX_GLOBAL_ROUTING_INFOA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FAX_GLOBAL_ROUTING_INFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FAX_GLOBAL_ROUTING_INFOA>()) == 0 }
    }
}
impl ::core::cmp::Eq for FAX_GLOBAL_ROUTING_INFOA {}
impl ::core::default::Default for FAX_GLOBAL_ROUTING_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct FAX_GLOBAL_ROUTING_INFOW {
    pub SizeOfStruct: u32,
    pub Priority: u32,
    pub Guid: ::windows::core::PCWSTR,
    pub FriendlyName: ::windows::core::PCWSTR,
    pub FunctionName: ::windows::core::PCWSTR,
    pub ExtensionImageName: ::windows::core::PCWSTR,
    pub ExtensionFriendlyName: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for FAX_GLOBAL_ROUTING_INFOW {}
impl ::core::clone::Clone for FAX_GLOBAL_ROUTING_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FAX_GLOBAL_ROUTING_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_GLOBAL_ROUTING_INFOW").field("SizeOfStruct", &self.SizeOfStruct).field("Priority", &self.Priority).field("Guid", &self.Guid).field("FriendlyName", &self.FriendlyName).field("FunctionName", &self.FunctionName).field("ExtensionImageName", &self.ExtensionImageName).field("ExtensionFriendlyName", &self.ExtensionFriendlyName).finish()
    }
}
unsafe impl ::windows::core::Abi for FAX_GLOBAL_ROUTING_INFOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FAX_GLOBAL_ROUTING_INFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FAX_GLOBAL_ROUTING_INFOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for FAX_GLOBAL_ROUTING_INFOW {}
impl ::core::default::Default for FAX_GLOBAL_ROUTING_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FAX_GROUP_STATUS_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fgsALL_DEV_VALID: FAX_GROUP_STATUS_ENUM = FAX_GROUP_STATUS_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fgsEMPTY: FAX_GROUP_STATUS_ENUM = FAX_GROUP_STATUS_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fgsALL_DEV_NOT_VALID: FAX_GROUP_STATUS_ENUM = FAX_GROUP_STATUS_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fgsSOME_DEV_NOT_VALID: FAX_GROUP_STATUS_ENUM = FAX_GROUP_STATUS_ENUM(3i32);
impl ::core::marker::Copy for FAX_GROUP_STATUS_ENUM {}
impl ::core::clone::Clone for FAX_GROUP_STATUS_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_GROUP_STATUS_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FAX_GROUP_STATUS_ENUM {
    type Abi = Self;
}
impl ::core::fmt::Debug for FAX_GROUP_STATUS_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_GROUP_STATUS_ENUM").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_JOB_ENTRYA {
    pub SizeOfStruct: u32,
    pub JobId: u32,
    pub UserName: ::windows::core::PCSTR,
    pub JobType: u32,
    pub QueueStatus: u32,
    pub Status: u32,
    pub Size: u32,
    pub PageCount: u32,
    pub RecipientNumber: ::windows::core::PCSTR,
    pub RecipientName: ::windows::core::PCSTR,
    pub Tsid: ::windows::core::PCSTR,
    pub SenderName: ::windows::core::PCSTR,
    pub SenderCompany: ::windows::core::PCSTR,
    pub SenderDept: ::windows::core::PCSTR,
    pub BillingCode: ::windows::core::PCSTR,
    pub ScheduleAction: u32,
    pub ScheduleTime: super::super::Foundation::SYSTEMTIME,
    pub DeliveryReportType: u32,
    pub DeliveryReportAddress: ::windows::core::PCSTR,
    pub DocumentName: ::windows::core::PCSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_JOB_ENTRYA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_JOB_ENTRYA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FAX_JOB_ENTRYA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_JOB_ENTRYA")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("JobId", &self.JobId)
            .field("UserName", &self.UserName)
            .field("JobType", &self.JobType)
            .field("QueueStatus", &self.QueueStatus)
            .field("Status", &self.Status)
            .field("Size", &self.Size)
            .field("PageCount", &self.PageCount)
            .field("RecipientNumber", &self.RecipientNumber)
            .field("RecipientName", &self.RecipientName)
            .field("Tsid", &self.Tsid)
            .field("SenderName", &self.SenderName)
            .field("SenderCompany", &self.SenderCompany)
            .field("SenderDept", &self.SenderDept)
            .field("BillingCode", &self.BillingCode)
            .field("ScheduleAction", &self.ScheduleAction)
            .field("ScheduleTime", &self.ScheduleTime)
            .field("DeliveryReportType", &self.DeliveryReportType)
            .field("DeliveryReportAddress", &self.DeliveryReportAddress)
            .field("DocumentName", &self.DocumentName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FAX_JOB_ENTRYA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FAX_JOB_ENTRYA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FAX_JOB_ENTRYA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FAX_JOB_ENTRYA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FAX_JOB_ENTRYA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_JOB_ENTRYW {
    pub SizeOfStruct: u32,
    pub JobId: u32,
    pub UserName: ::windows::core::PCWSTR,
    pub JobType: u32,
    pub QueueStatus: u32,
    pub Status: u32,
    pub Size: u32,
    pub PageCount: u32,
    pub RecipientNumber: ::windows::core::PCWSTR,
    pub RecipientName: ::windows::core::PCWSTR,
    pub Tsid: ::windows::core::PCWSTR,
    pub SenderName: ::windows::core::PCWSTR,
    pub SenderCompany: ::windows::core::PCWSTR,
    pub SenderDept: ::windows::core::PCWSTR,
    pub BillingCode: ::windows::core::PCWSTR,
    pub ScheduleAction: u32,
    pub ScheduleTime: super::super::Foundation::SYSTEMTIME,
    pub DeliveryReportType: u32,
    pub DeliveryReportAddress: ::windows::core::PCWSTR,
    pub DocumentName: ::windows::core::PCWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_JOB_ENTRYW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_JOB_ENTRYW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FAX_JOB_ENTRYW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_JOB_ENTRYW")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("JobId", &self.JobId)
            .field("UserName", &self.UserName)
            .field("JobType", &self.JobType)
            .field("QueueStatus", &self.QueueStatus)
            .field("Status", &self.Status)
            .field("Size", &self.Size)
            .field("PageCount", &self.PageCount)
            .field("RecipientNumber", &self.RecipientNumber)
            .field("RecipientName", &self.RecipientName)
            .field("Tsid", &self.Tsid)
            .field("SenderName", &self.SenderName)
            .field("SenderCompany", &self.SenderCompany)
            .field("SenderDept", &self.SenderDept)
            .field("BillingCode", &self.BillingCode)
            .field("ScheduleAction", &self.ScheduleAction)
            .field("ScheduleTime", &self.ScheduleTime)
            .field("DeliveryReportType", &self.DeliveryReportType)
            .field("DeliveryReportAddress", &self.DeliveryReportAddress)
            .field("DocumentName", &self.DocumentName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FAX_JOB_ENTRYW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FAX_JOB_ENTRYW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FAX_JOB_ENTRYW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FAX_JOB_ENTRYW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FAX_JOB_ENTRYW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FAX_JOB_EXTENDED_STATUS_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesNONE: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesDISCONNECTED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesINITIALIZING: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesDIALING: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(3i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesTRANSMITTING: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(4i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesANSWERED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(5i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesRECEIVING: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(6i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesLINE_UNAVAILABLE: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(7i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesBUSY: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(8i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesNO_ANSWER: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(9i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesBAD_ADDRESS: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(10i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesNO_DIAL_TONE: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(11i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesFATAL_ERROR: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(12i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesCALL_DELAYED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(13i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesCALL_BLACKLISTED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(14i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesNOT_FAX_CALL: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(15i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesPARTIALLY_RECEIVED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(16i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesHANDLED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(17i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesCALL_COMPLETED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(18i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesCALL_ABORTED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(19i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjesPROPRIETARY: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(16777216i32);
impl ::core::marker::Copy for FAX_JOB_EXTENDED_STATUS_ENUM {}
impl ::core::clone::Clone for FAX_JOB_EXTENDED_STATUS_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_JOB_EXTENDED_STATUS_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FAX_JOB_EXTENDED_STATUS_ENUM {
    type Abi = Self;
}
impl ::core::fmt::Debug for FAX_JOB_EXTENDED_STATUS_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_JOB_EXTENDED_STATUS_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_JOB_MANAGE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FAX_JOB_OPERATIONS_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjoVIEW: FAX_JOB_OPERATIONS_ENUM = FAX_JOB_OPERATIONS_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjoPAUSE: FAX_JOB_OPERATIONS_ENUM = FAX_JOB_OPERATIONS_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjoRESUME: FAX_JOB_OPERATIONS_ENUM = FAX_JOB_OPERATIONS_ENUM(4i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjoRESTART: FAX_JOB_OPERATIONS_ENUM = FAX_JOB_OPERATIONS_ENUM(8i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjoDELETE: FAX_JOB_OPERATIONS_ENUM = FAX_JOB_OPERATIONS_ENUM(16i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjoRECIPIENT_INFO: FAX_JOB_OPERATIONS_ENUM = FAX_JOB_OPERATIONS_ENUM(32i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjoSENDER_INFO: FAX_JOB_OPERATIONS_ENUM = FAX_JOB_OPERATIONS_ENUM(64i32);
impl ::core::marker::Copy for FAX_JOB_OPERATIONS_ENUM {}
impl ::core::clone::Clone for FAX_JOB_OPERATIONS_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_JOB_OPERATIONS_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FAX_JOB_OPERATIONS_ENUM {
    type Abi = Self;
}
impl ::core::fmt::Debug for FAX_JOB_OPERATIONS_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_JOB_OPERATIONS_ENUM").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_JOB_PARAMA {
    pub SizeOfStruct: u32,
    pub RecipientNumber: ::windows::core::PCSTR,
    pub RecipientName: ::windows::core::PCSTR,
    pub Tsid: ::windows::core::PCSTR,
    pub SenderName: ::windows::core::PCSTR,
    pub SenderCompany: ::windows::core::PCSTR,
    pub SenderDept: ::windows::core::PCSTR,
    pub BillingCode: ::windows::core::PCSTR,
    pub ScheduleAction: u32,
    pub ScheduleTime: super::super::Foundation::SYSTEMTIME,
    pub DeliveryReportType: u32,
    pub DeliveryReportAddress: ::windows::core::PCSTR,
    pub DocumentName: ::windows::core::PCSTR,
    pub CallHandle: u32,
    pub Reserved: [usize; 3],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_JOB_PARAMA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_JOB_PARAMA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FAX_JOB_PARAMA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_JOB_PARAMA")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("RecipientNumber", &self.RecipientNumber)
            .field("RecipientName", &self.RecipientName)
            .field("Tsid", &self.Tsid)
            .field("SenderName", &self.SenderName)
            .field("SenderCompany", &self.SenderCompany)
            .field("SenderDept", &self.SenderDept)
            .field("BillingCode", &self.BillingCode)
            .field("ScheduleAction", &self.ScheduleAction)
            .field("ScheduleTime", &self.ScheduleTime)
            .field("DeliveryReportType", &self.DeliveryReportType)
            .field("DeliveryReportAddress", &self.DeliveryReportAddress)
            .field("DocumentName", &self.DocumentName)
            .field("CallHandle", &self.CallHandle)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FAX_JOB_PARAMA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FAX_JOB_PARAMA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FAX_JOB_PARAMA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FAX_JOB_PARAMA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FAX_JOB_PARAMA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_JOB_PARAMW {
    pub SizeOfStruct: u32,
    pub RecipientNumber: ::windows::core::PCWSTR,
    pub RecipientName: ::windows::core::PCWSTR,
    pub Tsid: ::windows::core::PCWSTR,
    pub SenderName: ::windows::core::PCWSTR,
    pub SenderCompany: ::windows::core::PCWSTR,
    pub SenderDept: ::windows::core::PCWSTR,
    pub BillingCode: ::windows::core::PCWSTR,
    pub ScheduleAction: u32,
    pub ScheduleTime: super::super::Foundation::SYSTEMTIME,
    pub DeliveryReportType: u32,
    pub DeliveryReportAddress: ::windows::core::PCWSTR,
    pub DocumentName: ::windows::core::PCWSTR,
    pub CallHandle: u32,
    pub Reserved: [usize; 3],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_JOB_PARAMW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_JOB_PARAMW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FAX_JOB_PARAMW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_JOB_PARAMW")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("RecipientNumber", &self.RecipientNumber)
            .field("RecipientName", &self.RecipientName)
            .field("Tsid", &self.Tsid)
            .field("SenderName", &self.SenderName)
            .field("SenderCompany", &self.SenderCompany)
            .field("SenderDept", &self.SenderDept)
            .field("BillingCode", &self.BillingCode)
            .field("ScheduleAction", &self.ScheduleAction)
            .field("ScheduleTime", &self.ScheduleTime)
            .field("DeliveryReportType", &self.DeliveryReportType)
            .field("DeliveryReportAddress", &self.DeliveryReportAddress)
            .field("DocumentName", &self.DocumentName)
            .field("CallHandle", &self.CallHandle)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FAX_JOB_PARAMW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FAX_JOB_PARAMW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FAX_JOB_PARAMW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FAX_JOB_PARAMW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FAX_JOB_PARAMW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_JOB_QUERY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FAX_JOB_STATUS_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjsPENDING: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjsINPROGRESS: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjsFAILED: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(8i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjsPAUSED: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(16i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjsNOLINE: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(32i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjsRETRYING: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(64i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjsRETRIES_EXCEEDED: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(128i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjsCOMPLETED: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(256i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjsCANCELED: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(512i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjsCANCELING: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(1024i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjsROUTING: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(2048i32);
impl ::core::marker::Copy for FAX_JOB_STATUS_ENUM {}
impl ::core::clone::Clone for FAX_JOB_STATUS_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_JOB_STATUS_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FAX_JOB_STATUS_ENUM {
    type Abi = Self;
}
impl ::core::fmt::Debug for FAX_JOB_STATUS_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_JOB_STATUS_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_JOB_SUBMIT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FAX_JOB_TYPE_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjtSEND: FAX_JOB_TYPE_ENUM = FAX_JOB_TYPE_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjtRECEIVE: FAX_JOB_TYPE_ENUM = FAX_JOB_TYPE_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fjtROUTING: FAX_JOB_TYPE_ENUM = FAX_JOB_TYPE_ENUM(2i32);
impl ::core::marker::Copy for FAX_JOB_TYPE_ENUM {}
impl ::core::clone::Clone for FAX_JOB_TYPE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_JOB_TYPE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FAX_JOB_TYPE_ENUM {
    type Abi = Self;
}
impl ::core::fmt::Debug for FAX_JOB_TYPE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_JOB_TYPE_ENUM").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct FAX_LOG_CATEGORYA {
    pub Name: ::windows::core::PCSTR,
    pub Category: u32,
    pub Level: u32,
}
impl ::core::marker::Copy for FAX_LOG_CATEGORYA {}
impl ::core::clone::Clone for FAX_LOG_CATEGORYA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FAX_LOG_CATEGORYA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_LOG_CATEGORYA").field("Name", &self.Name).field("Category", &self.Category).field("Level", &self.Level).finish()
    }
}
unsafe impl ::windows::core::Abi for FAX_LOG_CATEGORYA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FAX_LOG_CATEGORYA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FAX_LOG_CATEGORYA>()) == 0 }
    }
}
impl ::core::cmp::Eq for FAX_LOG_CATEGORYA {}
impl ::core::default::Default for FAX_LOG_CATEGORYA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct FAX_LOG_CATEGORYW {
    pub Name: ::windows::core::PCWSTR,
    pub Category: u32,
    pub Level: u32,
}
impl ::core::marker::Copy for FAX_LOG_CATEGORYW {}
impl ::core::clone::Clone for FAX_LOG_CATEGORYW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FAX_LOG_CATEGORYW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_LOG_CATEGORYW").field("Name", &self.Name).field("Category", &self.Category).field("Level", &self.Level).finish()
    }
}
unsafe impl ::windows::core::Abi for FAX_LOG_CATEGORYW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FAX_LOG_CATEGORYW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FAX_LOG_CATEGORYW>()) == 0 }
    }
}
impl ::core::cmp::Eq for FAX_LOG_CATEGORYW {}
impl ::core::default::Default for FAX_LOG_CATEGORYW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FAX_LOG_LEVEL_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fllNONE: FAX_LOG_LEVEL_ENUM = FAX_LOG_LEVEL_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fllMIN: FAX_LOG_LEVEL_ENUM = FAX_LOG_LEVEL_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fllMED: FAX_LOG_LEVEL_ENUM = FAX_LOG_LEVEL_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fllMAX: FAX_LOG_LEVEL_ENUM = FAX_LOG_LEVEL_ENUM(3i32);
impl ::core::marker::Copy for FAX_LOG_LEVEL_ENUM {}
impl ::core::clone::Clone for FAX_LOG_LEVEL_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_LOG_LEVEL_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FAX_LOG_LEVEL_ENUM {
    type Abi = Self;
}
impl ::core::fmt::Debug for FAX_LOG_LEVEL_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_LOG_LEVEL_ENUM").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct FAX_PORT_INFOA {
    pub SizeOfStruct: u32,
    pub DeviceId: u32,
    pub State: u32,
    pub Flags: u32,
    pub Rings: u32,
    pub Priority: u32,
    pub DeviceName: ::windows::core::PCSTR,
    pub Tsid: ::windows::core::PCSTR,
    pub Csid: ::windows::core::PCSTR,
}
impl ::core::marker::Copy for FAX_PORT_INFOA {}
impl ::core::clone::Clone for FAX_PORT_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FAX_PORT_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_PORT_INFOA").field("SizeOfStruct", &self.SizeOfStruct).field("DeviceId", &self.DeviceId).field("State", &self.State).field("Flags", &self.Flags).field("Rings", &self.Rings).field("Priority", &self.Priority).field("DeviceName", &self.DeviceName).field("Tsid", &self.Tsid).field("Csid", &self.Csid).finish()
    }
}
unsafe impl ::windows::core::Abi for FAX_PORT_INFOA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FAX_PORT_INFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FAX_PORT_INFOA>()) == 0 }
    }
}
impl ::core::cmp::Eq for FAX_PORT_INFOA {}
impl ::core::default::Default for FAX_PORT_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct FAX_PORT_INFOW {
    pub SizeOfStruct: u32,
    pub DeviceId: u32,
    pub State: u32,
    pub Flags: u32,
    pub Rings: u32,
    pub Priority: u32,
    pub DeviceName: ::windows::core::PCWSTR,
    pub Tsid: ::windows::core::PCWSTR,
    pub Csid: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for FAX_PORT_INFOW {}
impl ::core::clone::Clone for FAX_PORT_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FAX_PORT_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_PORT_INFOW").field("SizeOfStruct", &self.SizeOfStruct).field("DeviceId", &self.DeviceId).field("State", &self.State).field("Flags", &self.Flags).field("Rings", &self.Rings).field("Priority", &self.Priority).field("DeviceName", &self.DeviceName).field("Tsid", &self.Tsid).field("Csid", &self.Csid).finish()
    }
}
unsafe impl ::windows::core::Abi for FAX_PORT_INFOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FAX_PORT_INFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FAX_PORT_INFOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for FAX_PORT_INFOW {}
impl ::core::default::Default for FAX_PORT_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_PORT_QUERY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FAX_PORT_SET: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct FAX_PRINT_INFOA {
    pub SizeOfStruct: u32,
    pub DocName: ::windows::core::PCSTR,
    pub RecipientName: ::windows::core::PCSTR,
    pub RecipientNumber: ::windows::core::PCSTR,
    pub SenderName: ::windows::core::PCSTR,
    pub SenderCompany: ::windows::core::PCSTR,
    pub SenderDept: ::windows::core::PCSTR,
    pub SenderBillingCode: ::windows::core::PCSTR,
    pub Reserved: ::windows::core::PCSTR,
    pub DrEmailAddress: ::windows::core::PCSTR,
    pub OutputFileName: ::windows::core::PCSTR,
}
impl ::core::marker::Copy for FAX_PRINT_INFOA {}
impl ::core::clone::Clone for FAX_PRINT_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FAX_PRINT_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_PRINT_INFOA")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("DocName", &self.DocName)
            .field("RecipientName", &self.RecipientName)
            .field("RecipientNumber", &self.RecipientNumber)
            .field("SenderName", &self.SenderName)
            .field("SenderCompany", &self.SenderCompany)
            .field("SenderDept", &self.SenderDept)
            .field("SenderBillingCode", &self.SenderBillingCode)
            .field("Reserved", &self.Reserved)
            .field("DrEmailAddress", &self.DrEmailAddress)
            .field("OutputFileName", &self.OutputFileName)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for FAX_PRINT_INFOA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FAX_PRINT_INFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FAX_PRINT_INFOA>()) == 0 }
    }
}
impl ::core::cmp::Eq for FAX_PRINT_INFOA {}
impl ::core::default::Default for FAX_PRINT_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct FAX_PRINT_INFOW {
    pub SizeOfStruct: u32,
    pub DocName: ::windows::core::PCWSTR,
    pub RecipientName: ::windows::core::PCWSTR,
    pub RecipientNumber: ::windows::core::PCWSTR,
    pub SenderName: ::windows::core::PCWSTR,
    pub SenderCompany: ::windows::core::PCWSTR,
    pub SenderDept: ::windows::core::PCWSTR,
    pub SenderBillingCode: ::windows::core::PCWSTR,
    pub Reserved: ::windows::core::PCWSTR,
    pub DrEmailAddress: ::windows::core::PCWSTR,
    pub OutputFileName: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for FAX_PRINT_INFOW {}
impl ::core::clone::Clone for FAX_PRINT_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FAX_PRINT_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_PRINT_INFOW")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("DocName", &self.DocName)
            .field("RecipientName", &self.RecipientName)
            .field("RecipientNumber", &self.RecipientNumber)
            .field("SenderName", &self.SenderName)
            .field("SenderCompany", &self.SenderCompany)
            .field("SenderDept", &self.SenderDept)
            .field("SenderBillingCode", &self.SenderBillingCode)
            .field("Reserved", &self.Reserved)
            .field("DrEmailAddress", &self.DrEmailAddress)
            .field("OutputFileName", &self.OutputFileName)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for FAX_PRINT_INFOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FAX_PRINT_INFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FAX_PRINT_INFOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for FAX_PRINT_INFOW {}
impl ::core::default::Default for FAX_PRINT_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FAX_PRIORITY_TYPE_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fptLOW: FAX_PRIORITY_TYPE_ENUM = FAX_PRIORITY_TYPE_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fptNORMAL: FAX_PRIORITY_TYPE_ENUM = FAX_PRIORITY_TYPE_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fptHIGH: FAX_PRIORITY_TYPE_ENUM = FAX_PRIORITY_TYPE_ENUM(2i32);
impl ::core::marker::Copy for FAX_PRIORITY_TYPE_ENUM {}
impl ::core::clone::Clone for FAX_PRIORITY_TYPE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_PRIORITY_TYPE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FAX_PRIORITY_TYPE_ENUM {
    type Abi = Self;
}
impl ::core::fmt::Debug for FAX_PRIORITY_TYPE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_PRIORITY_TYPE_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FAX_PROVIDER_STATUS_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fpsSUCCESS: FAX_PROVIDER_STATUS_ENUM = FAX_PROVIDER_STATUS_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fpsSERVER_ERROR: FAX_PROVIDER_STATUS_ENUM = FAX_PROVIDER_STATUS_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fpsBAD_GUID: FAX_PROVIDER_STATUS_ENUM = FAX_PROVIDER_STATUS_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fpsBAD_VERSION: FAX_PROVIDER_STATUS_ENUM = FAX_PROVIDER_STATUS_ENUM(3i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fpsCANT_LOAD: FAX_PROVIDER_STATUS_ENUM = FAX_PROVIDER_STATUS_ENUM(4i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fpsCANT_LINK: FAX_PROVIDER_STATUS_ENUM = FAX_PROVIDER_STATUS_ENUM(5i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fpsCANT_INIT: FAX_PROVIDER_STATUS_ENUM = FAX_PROVIDER_STATUS_ENUM(6i32);
impl ::core::marker::Copy for FAX_PROVIDER_STATUS_ENUM {}
impl ::core::clone::Clone for FAX_PROVIDER_STATUS_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_PROVIDER_STATUS_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FAX_PROVIDER_STATUS_ENUM {
    type Abi = Self;
}
impl ::core::fmt::Debug for FAX_PROVIDER_STATUS_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_PROVIDER_STATUS_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FAX_RECEIPT_TYPE_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const frtNONE: FAX_RECEIPT_TYPE_ENUM = FAX_RECEIPT_TYPE_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const frtMAIL: FAX_RECEIPT_TYPE_ENUM = FAX_RECEIPT_TYPE_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const frtMSGBOX: FAX_RECEIPT_TYPE_ENUM = FAX_RECEIPT_TYPE_ENUM(4i32);
impl ::core::marker::Copy for FAX_RECEIPT_TYPE_ENUM {}
impl ::core::clone::Clone for FAX_RECEIPT_TYPE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_RECEIPT_TYPE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FAX_RECEIPT_TYPE_ENUM {
    type Abi = Self;
}
impl ::core::fmt::Debug for FAX_RECEIPT_TYPE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_RECEIPT_TYPE_ENUM").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct FAX_RECEIVE {
    pub SizeOfStruct: u32,
    pub FileName: ::windows::core::PWSTR,
    pub ReceiverName: ::windows::core::PWSTR,
    pub ReceiverNumber: ::windows::core::PWSTR,
    pub Reserved: [u32; 4],
}
impl ::core::marker::Copy for FAX_RECEIVE {}
impl ::core::clone::Clone for FAX_RECEIVE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FAX_RECEIVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_RECEIVE").field("SizeOfStruct", &self.SizeOfStruct).field("FileName", &self.FileName).field("ReceiverName", &self.ReceiverName).field("ReceiverNumber", &self.ReceiverNumber).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for FAX_RECEIVE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FAX_RECEIVE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FAX_RECEIVE>()) == 0 }
    }
}
impl ::core::cmp::Eq for FAX_RECEIVE {}
impl ::core::default::Default for FAX_RECEIVE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct FAX_ROUTE {
    pub SizeOfStruct: u32,
    pub JobId: u32,
    pub ElapsedTime: u64,
    pub ReceiveTime: u64,
    pub PageCount: u32,
    pub Csid: ::windows::core::PCWSTR,
    pub Tsid: ::windows::core::PCWSTR,
    pub CallerId: ::windows::core::PCWSTR,
    pub RoutingInfo: ::windows::core::PCWSTR,
    pub ReceiverName: ::windows::core::PCWSTR,
    pub ReceiverNumber: ::windows::core::PCWSTR,
    pub DeviceName: ::windows::core::PCWSTR,
    pub DeviceId: u32,
    pub RoutingInfoData: *mut u8,
    pub RoutingInfoDataSize: u32,
}
impl ::core::marker::Copy for FAX_ROUTE {}
impl ::core::clone::Clone for FAX_ROUTE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FAX_ROUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_ROUTE")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("JobId", &self.JobId)
            .field("ElapsedTime", &self.ElapsedTime)
            .field("ReceiveTime", &self.ReceiveTime)
            .field("PageCount", &self.PageCount)
            .field("Csid", &self.Csid)
            .field("Tsid", &self.Tsid)
            .field("CallerId", &self.CallerId)
            .field("RoutingInfo", &self.RoutingInfo)
            .field("ReceiverName", &self.ReceiverName)
            .field("ReceiverNumber", &self.ReceiverNumber)
            .field("DeviceName", &self.DeviceName)
            .field("DeviceId", &self.DeviceId)
            .field("RoutingInfoData", &self.RoutingInfoData)
            .field("RoutingInfoDataSize", &self.RoutingInfoDataSize)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for FAX_ROUTE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FAX_ROUTE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FAX_ROUTE>()) == 0 }
    }
}
impl ::core::cmp::Eq for FAX_ROUTE {}
impl ::core::default::Default for FAX_ROUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_ROUTE_CALLBACKROUTINES {
    pub SizeOfStruct: u32,
    pub FaxRouteAddFile: PFAXROUTEADDFILE,
    pub FaxRouteDeleteFile: PFAXROUTEDELETEFILE,
    pub FaxRouteGetFile: PFAXROUTEGETFILE,
    pub FaxRouteEnumFiles: PFAXROUTEENUMFILES,
    pub FaxRouteModifyRoutingData: PFAXROUTEMODIFYROUTINGDATA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_ROUTE_CALLBACKROUTINES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_ROUTE_CALLBACKROUTINES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FAX_ROUTE_CALLBACKROUTINES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_ROUTE_CALLBACKROUTINES").field("SizeOfStruct", &self.SizeOfStruct).field("FaxRouteAddFile", &self.FaxRouteAddFile.map(|f| f as usize)).field("FaxRouteDeleteFile", &self.FaxRouteDeleteFile.map(|f| f as usize)).field("FaxRouteGetFile", &self.FaxRouteGetFile.map(|f| f as usize)).field("FaxRouteEnumFiles", &self.FaxRouteEnumFiles.map(|f| f as usize)).field("FaxRouteModifyRoutingData", &self.FaxRouteModifyRoutingData.map(|f| f as usize)).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FAX_ROUTE_CALLBACKROUTINES {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FAX_ROUTE_CALLBACKROUTINES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FAX_ROUTE_CALLBACKROUTINES>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FAX_ROUTE_CALLBACKROUTINES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FAX_ROUTE_CALLBACKROUTINES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_ROUTING_METHODA {
    pub SizeOfStruct: u32,
    pub DeviceId: u32,
    pub Enabled: super::super::Foundation::BOOL,
    pub DeviceName: ::windows::core::PCSTR,
    pub Guid: ::windows::core::PCSTR,
    pub FriendlyName: ::windows::core::PCSTR,
    pub FunctionName: ::windows::core::PCSTR,
    pub ExtensionImageName: ::windows::core::PCSTR,
    pub ExtensionFriendlyName: ::windows::core::PCSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_ROUTING_METHODA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_ROUTING_METHODA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FAX_ROUTING_METHODA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_ROUTING_METHODA").field("SizeOfStruct", &self.SizeOfStruct).field("DeviceId", &self.DeviceId).field("Enabled", &self.Enabled).field("DeviceName", &self.DeviceName).field("Guid", &self.Guid).field("FriendlyName", &self.FriendlyName).field("FunctionName", &self.FunctionName).field("ExtensionImageName", &self.ExtensionImageName).field("ExtensionFriendlyName", &self.ExtensionFriendlyName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FAX_ROUTING_METHODA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FAX_ROUTING_METHODA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FAX_ROUTING_METHODA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FAX_ROUTING_METHODA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FAX_ROUTING_METHODA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_ROUTING_METHODW {
    pub SizeOfStruct: u32,
    pub DeviceId: u32,
    pub Enabled: super::super::Foundation::BOOL,
    pub DeviceName: ::windows::core::PCWSTR,
    pub Guid: ::windows::core::PCWSTR,
    pub FriendlyName: ::windows::core::PCWSTR,
    pub FunctionName: ::windows::core::PCWSTR,
    pub ExtensionImageName: ::windows::core::PCWSTR,
    pub ExtensionFriendlyName: ::windows::core::PCWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_ROUTING_METHODW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_ROUTING_METHODW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FAX_ROUTING_METHODW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_ROUTING_METHODW").field("SizeOfStruct", &self.SizeOfStruct).field("DeviceId", &self.DeviceId).field("Enabled", &self.Enabled).field("DeviceName", &self.DeviceName).field("Guid", &self.Guid).field("FriendlyName", &self.FriendlyName).field("FunctionName", &self.FunctionName).field("ExtensionImageName", &self.ExtensionImageName).field("ExtensionFriendlyName", &self.ExtensionFriendlyName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FAX_ROUTING_METHODW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FAX_ROUTING_METHODW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FAX_ROUTING_METHODW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FAX_ROUTING_METHODW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FAX_ROUTING_METHODW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FAX_ROUTING_RULE_CODE_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const frrcANY_CODE: FAX_ROUTING_RULE_CODE_ENUM = FAX_ROUTING_RULE_CODE_ENUM(0i32);
impl ::core::marker::Copy for FAX_ROUTING_RULE_CODE_ENUM {}
impl ::core::clone::Clone for FAX_ROUTING_RULE_CODE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_ROUTING_RULE_CODE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FAX_ROUTING_RULE_CODE_ENUM {
    type Abi = Self;
}
impl ::core::fmt::Debug for FAX_ROUTING_RULE_CODE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_ROUTING_RULE_CODE_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FAX_RULE_STATUS_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const frsVALID: FAX_RULE_STATUS_ENUM = FAX_RULE_STATUS_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const frsEMPTY_GROUP: FAX_RULE_STATUS_ENUM = FAX_RULE_STATUS_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const frsALL_GROUP_DEV_NOT_VALID: FAX_RULE_STATUS_ENUM = FAX_RULE_STATUS_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const frsSOME_GROUP_DEV_NOT_VALID: FAX_RULE_STATUS_ENUM = FAX_RULE_STATUS_ENUM(3i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const frsBAD_DEVICE: FAX_RULE_STATUS_ENUM = FAX_RULE_STATUS_ENUM(4i32);
impl ::core::marker::Copy for FAX_RULE_STATUS_ENUM {}
impl ::core::clone::Clone for FAX_RULE_STATUS_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_RULE_STATUS_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FAX_RULE_STATUS_ENUM {
    type Abi = Self;
}
impl ::core::fmt::Debug for FAX_RULE_STATUS_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_RULE_STATUS_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FAX_SCHEDULE_TYPE_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fstNOW: FAX_SCHEDULE_TYPE_ENUM = FAX_SCHEDULE_TYPE_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fstSPECIFIC_TIME: FAX_SCHEDULE_TYPE_ENUM = FAX_SCHEDULE_TYPE_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fstDISCOUNT_PERIOD: FAX_SCHEDULE_TYPE_ENUM = FAX_SCHEDULE_TYPE_ENUM(2i32);
impl ::core::marker::Copy for FAX_SCHEDULE_TYPE_ENUM {}
impl ::core::clone::Clone for FAX_SCHEDULE_TYPE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_SCHEDULE_TYPE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FAX_SCHEDULE_TYPE_ENUM {
    type Abi = Self;
}
impl ::core::fmt::Debug for FAX_SCHEDULE_TYPE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_SCHEDULE_TYPE_ENUM").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_SEND {
    pub SizeOfStruct: u32,
    pub FileName: ::windows::core::PWSTR,
    pub CallerName: ::windows::core::PWSTR,
    pub CallerNumber: ::windows::core::PWSTR,
    pub ReceiverName: ::windows::core::PWSTR,
    pub ReceiverNumber: ::windows::core::PWSTR,
    pub Branding: super::super::Foundation::BOOL,
    pub CallHandle: u32,
    pub Reserved: [u32; 3],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FAX_SEND {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FAX_SEND {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FAX_SEND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_SEND").field("SizeOfStruct", &self.SizeOfStruct).field("FileName", &self.FileName).field("CallerName", &self.CallerName).field("CallerNumber", &self.CallerNumber).field("ReceiverName", &self.ReceiverName).field("ReceiverNumber", &self.ReceiverNumber).field("Branding", &self.Branding).field("CallHandle", &self.CallHandle).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FAX_SEND {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FAX_SEND {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FAX_SEND>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FAX_SEND {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FAX_SEND {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FAX_SERVER_APIVERSION_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsAPI_VERSION_0: FAX_SERVER_APIVERSION_ENUM = FAX_SERVER_APIVERSION_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsAPI_VERSION_1: FAX_SERVER_APIVERSION_ENUM = FAX_SERVER_APIVERSION_ENUM(65536i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsAPI_VERSION_2: FAX_SERVER_APIVERSION_ENUM = FAX_SERVER_APIVERSION_ENUM(131072i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsAPI_VERSION_3: FAX_SERVER_APIVERSION_ENUM = FAX_SERVER_APIVERSION_ENUM(196608i32);
impl ::core::marker::Copy for FAX_SERVER_APIVERSION_ENUM {}
impl ::core::clone::Clone for FAX_SERVER_APIVERSION_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_SERVER_APIVERSION_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FAX_SERVER_APIVERSION_ENUM {
    type Abi = Self;
}
impl ::core::fmt::Debug for FAX_SERVER_APIVERSION_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_SERVER_APIVERSION_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FAX_SERVER_EVENTS_TYPE_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsetNONE: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsetIN_QUEUE: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsetOUT_QUEUE: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsetCONFIG: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(4i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsetACTIVITY: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(8i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsetQUEUE_STATE: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(16i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsetIN_ARCHIVE: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(32i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsetOUT_ARCHIVE: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(64i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsetFXSSVC_ENDED: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(128i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsetDEVICE_STATUS: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(256i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsetINCOMING_CALL: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(512i32);
impl ::core::marker::Copy for FAX_SERVER_EVENTS_TYPE_ENUM {}
impl ::core::clone::Clone for FAX_SERVER_EVENTS_TYPE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_SERVER_EVENTS_TYPE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FAX_SERVER_EVENTS_TYPE_ENUM {
    type Abi = Self;
}
impl ::core::fmt::Debug for FAX_SERVER_EVENTS_TYPE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_SERVER_EVENTS_TYPE_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FAX_SMTP_AUTHENTICATION_TYPE_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsatANONYMOUS: FAX_SMTP_AUTHENTICATION_TYPE_ENUM = FAX_SMTP_AUTHENTICATION_TYPE_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsatBASIC: FAX_SMTP_AUTHENTICATION_TYPE_ENUM = FAX_SMTP_AUTHENTICATION_TYPE_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const fsatNTLM: FAX_SMTP_AUTHENTICATION_TYPE_ENUM = FAX_SMTP_AUTHENTICATION_TYPE_ENUM(2i32);
impl ::core::marker::Copy for FAX_SMTP_AUTHENTICATION_TYPE_ENUM {}
impl ::core::clone::Clone for FAX_SMTP_AUTHENTICATION_TYPE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FAX_SMTP_AUTHENTICATION_TYPE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FAX_SMTP_AUTHENTICATION_TYPE_ENUM {
    type Abi = Self;
}
impl ::core::fmt::Debug for FAX_SMTP_AUTHENTICATION_TYPE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FAX_SMTP_AUTHENTICATION_TYPE_ENUM").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct FAX_TIME {
    pub Hour: u16,
    pub Minute: u16,
}
impl ::core::marker::Copy for FAX_TIME {}
impl ::core::clone::Clone for FAX_TIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FAX_TIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAX_TIME").field("Hour", &self.Hour).field("Minute", &self.Minute).finish()
    }
}
unsafe impl ::windows::core::Abi for FAX_TIME {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FAX_TIME {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FAX_TIME>()) == 0 }
    }
}
impl ::core::cmp::Eq for FAX_TIME {}
impl ::core::default::Default for FAX_TIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_ABORTING: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_ANSWERED: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_BAD_ADDRESS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_BUSY: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_CALL_BLACKLISTED: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_CALL_DELAYED: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_COMPLETED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_DELETED: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_DIALING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_DISCONNECTED: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_FATAL_ERROR: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_FAXSVC_ENDED: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_FAXSVC_STARTED: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_HANDLED: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_IDLE: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_INITIALIZING: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_JOB_QUEUED: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_LINE_UNAVAILABLE: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_MODEM_POWERED_OFF: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_MODEM_POWERED_ON: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_NEVENTS: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_NOT_FAX_CALL: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_NO_ANSWER: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_NO_DIAL_TONE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_RECEIVING: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_RINGING: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_ROUTING: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FEI_SENDING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPF_RECEIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPF_SEND: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPF_VIRTUAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_ABORTING: u32 = 538968064u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_ANSWERED: u32 = 545259520u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_AVAILABLE: u32 = 537919488u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_BAD_ADDRESS: u32 = 536871168u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_BUSY: u32 = 536870976u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_CALL_BLACKLISTED: u32 = 536887296u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_CALL_DELAYED: u32 = 536879104u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_COMPLETED: u32 = 536870920u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_DIALING: u32 = 536870913u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_DISCONNECTED: u32 = 536871936u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_FATAL_ERROR: u32 = 536872960u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_HANDLED: u32 = 536870928u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_INITIALIZING: u32 = 536903680u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_NOT_FAX_CALL: u32 = 536875008u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_NO_ANSWER: u32 = 536871040u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_NO_DIAL_TONE: u32 = 536871424u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_OFFLINE: u32 = 536936448u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_RECEIVING: u32 = 536870916u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_RINGING: u32 = 537001984u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_ROUTING: u32 = 541065216u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_SENDING: u32 = 536870914u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FPS_UNAVAILABLE: u32 = 536870944u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_ANSWERED: u32 = 545259520u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_BAD_ADDRESS: u32 = 536871168u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_BUSY: u32 = 536870976u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_CALL_BLACKLISTED: u32 = 536887296u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_CALL_DELAYED: u32 = 536879104u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_COMPLETED: u32 = 536870920u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_DIALING: u32 = 536870913u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_DISCONNECTED: u32 = 536871936u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_FATAL_ERROR: u32 = 536872960u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_HANDLED: u32 = 536870928u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_INITIALIZING: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_LINE_UNAVAILABLE: u32 = 536870944u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_NOT_FAX_CALL: u32 = 536875008u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_NO_ANSWER: u32 = 536871040u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_NO_DIAL_TONE: u32 = 536871424u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_RECEIVING: u32 = 536870916u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_TRANSMITTING: u32 = 536870914u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const FS_USER_ABORT: u32 = 538968064u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxAbort<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, jobid: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxAbort(faxhandle: super::super::Foundation::HANDLE, jobid: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxAbort(faxhandle.into_param().abi(), ::core::mem::transmute(jobid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxAccessCheck<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, accessmask: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxAccessCheck(faxhandle: super::super::Foundation::HANDLE, accessmask: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxAccessCheck(faxhandle.into_param().abi(), ::core::mem::transmute(accessmask)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const FaxAccount: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7e0647f_4524_4464_a56d_b9fe666f715e);
pub const FaxAccountFolders: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85398f49_c034_4a3f_821c_db7d685e8129);
pub const FaxAccountIncomingArchive: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14b33db5_4c40_4ecf_9ef8_a360cbe809ed);
pub const FaxAccountIncomingQueue: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9bcf6094_b4da_45f4_b8d6_ddeb2186652c);
pub const FaxAccountOutgoingArchive: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x851e7af5_433a_4739_a2df_ad245c2cb98e);
pub const FaxAccountOutgoingQueue: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfeeceefb_c149_48ba_bab8_b791e101f62f);
pub const FaxAccountSet: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbc23c4b_79e0_4291_bc56_c12e253bbf3a);
pub const FaxAccounts: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda1f94aa_ee2c_47c0_8f4f_2a217075b76e);
pub const FaxActivity: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcfef5d0e_e84d_462e_aabb_87d31eb04fef);
pub const FaxActivityLogging: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0a0294e_3bbd_48b8_8f13_8c591a55bdbc);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxClose<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxClose(faxhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxClose(faxhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxCompleteJobParamsA(jobparams: *mut *mut FAX_JOB_PARAMA, coverpageinfo: *mut *mut FAX_COVERPAGE_INFOA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxCompleteJobParamsA(jobparams: *mut *mut FAX_JOB_PARAMA, coverpageinfo: *mut *mut FAX_COVERPAGE_INFOA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxCompleteJobParamsA(::core::mem::transmute(jobparams), ::core::mem::transmute(coverpageinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxCompleteJobParamsW(jobparams: *mut *mut FAX_JOB_PARAMW, coverpageinfo: *mut *mut FAX_COVERPAGE_INFOW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxCompleteJobParamsW(jobparams: *mut *mut FAX_JOB_PARAMW, coverpageinfo: *mut *mut FAX_COVERPAGE_INFOW) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxCompleteJobParamsW(::core::mem::transmute(jobparams), ::core::mem::transmute(coverpageinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const FaxConfiguration: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5857326f_e7b3_41a7_9c19_a91b463e2d56);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxConnectFaxServerA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(machinename: Param0, faxhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxConnectFaxServerA(machinename: ::windows::core::PCSTR, faxhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxConnectFaxServerA(machinename.into_param().abi(), ::core::mem::transmute(faxhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxConnectFaxServerW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(machinename: Param0, faxhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxConnectFaxServerW(machinename: ::windows::core::PCWSTR, faxhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxConnectFaxServerW(machinename.into_param().abi(), ::core::mem::transmute(faxhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const FaxDevice: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59e3a5b2_d676_484b_a6de_720bfa89b5af);
pub const FaxDeviceIds: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcdc539ea_7277_460e_8de0_48a0a5760d1f);
pub const FaxDeviceProvider: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17cf1aa3_f5eb_484a_9c9a_4440a5baabfc);
pub const FaxDeviceProviders: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb8fe768_875a_4f5f_82c5_03f23aac1bd7);
pub const FaxDevices: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5589e28e_23cb_4919_8808_e6101846e80d);
pub const FaxDocument: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f3f9f91_c838_415e_a4f3_3e828ca445e0);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxEnableRoutingMethodA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(faxporthandle: Param0, routingguid: Param1, enabled: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxEnableRoutingMethodA(faxporthandle: super::super::Foundation::HANDLE, routingguid: ::windows::core::PCSTR, enabled: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxEnableRoutingMethodA(faxporthandle.into_param().abi(), routingguid.into_param().abi(), enabled.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxEnableRoutingMethodW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(faxporthandle: Param0, routingguid: Param1, enabled: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxEnableRoutingMethodW(faxporthandle: super::super::Foundation::HANDLE, routingguid: ::windows::core::PCWSTR, enabled: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxEnableRoutingMethodW(faxporthandle.into_param().abi(), routingguid.into_param().abi(), enabled.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxEnumGlobalRoutingInfoA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, routinginfo: *mut *mut FAX_GLOBAL_ROUTING_INFOA, methodsreturned: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxEnumGlobalRoutingInfoA(faxhandle: super::super::Foundation::HANDLE, routinginfo: *mut *mut FAX_GLOBAL_ROUTING_INFOA, methodsreturned: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxEnumGlobalRoutingInfoA(faxhandle.into_param().abi(), ::core::mem::transmute(routinginfo), ::core::mem::transmute(methodsreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxEnumGlobalRoutingInfoW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, routinginfo: *mut *mut FAX_GLOBAL_ROUTING_INFOW, methodsreturned: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxEnumGlobalRoutingInfoW(faxhandle: super::super::Foundation::HANDLE, routinginfo: *mut *mut FAX_GLOBAL_ROUTING_INFOW, methodsreturned: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxEnumGlobalRoutingInfoW(faxhandle.into_param().abi(), ::core::mem::transmute(routinginfo), ::core::mem::transmute(methodsreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxEnumJobsA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, jobentry: *mut *mut FAX_JOB_ENTRYA, jobsreturned: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxEnumJobsA(faxhandle: super::super::Foundation::HANDLE, jobentry: *mut *mut FAX_JOB_ENTRYA, jobsreturned: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxEnumJobsA(faxhandle.into_param().abi(), ::core::mem::transmute(jobentry), ::core::mem::transmute(jobsreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxEnumJobsW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, jobentry: *mut *mut FAX_JOB_ENTRYW, jobsreturned: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxEnumJobsW(faxhandle: super::super::Foundation::HANDLE, jobentry: *mut *mut FAX_JOB_ENTRYW, jobsreturned: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxEnumJobsW(faxhandle.into_param().abi(), ::core::mem::transmute(jobentry), ::core::mem::transmute(jobsreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxEnumPortsA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, portinfo: *mut *mut FAX_PORT_INFOA, portsreturned: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxEnumPortsA(faxhandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOA, portsreturned: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxEnumPortsA(faxhandle.into_param().abi(), ::core::mem::transmute(portinfo), ::core::mem::transmute(portsreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxEnumPortsW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, portinfo: *mut *mut FAX_PORT_INFOW, portsreturned: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxEnumPortsW(faxhandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOW, portsreturned: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxEnumPortsW(faxhandle.into_param().abi(), ::core::mem::transmute(portinfo), ::core::mem::transmute(portsreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxEnumRoutingMethodsA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(faxporthandle: Param0, routingmethod: *mut *mut FAX_ROUTING_METHODA, methodsreturned: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxEnumRoutingMethodsA(faxporthandle: super::super::Foundation::HANDLE, routingmethod: *mut *mut FAX_ROUTING_METHODA, methodsreturned: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxEnumRoutingMethodsA(faxporthandle.into_param().abi(), ::core::mem::transmute(routingmethod), ::core::mem::transmute(methodsreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxEnumRoutingMethodsW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(faxporthandle: Param0, routingmethod: *mut *mut FAX_ROUTING_METHODW, methodsreturned: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxEnumRoutingMethodsW(faxporthandle: super::super::Foundation::HANDLE, routingmethod: *mut *mut FAX_ROUTING_METHODW, methodsreturned: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxEnumRoutingMethodsW(faxporthandle.into_param().abi(), ::core::mem::transmute(routingmethod), ::core::mem::transmute(methodsreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const FaxEventLogging: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6850930_a0f6_4a6f_95b7_db2ebf3d02e3);
pub const FaxFolders: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc35211d7_5776_48cb_af44_c31be3b2cfe5);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[inline]
pub unsafe fn FaxFreeBuffer(buffer: *mut ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxFreeBuffer(buffer: *mut ::core::ffi::c_void);
        }
        FaxFreeBuffer(::core::mem::transmute(buffer))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetConfigurationA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, faxconfig: *mut *mut FAX_CONFIGURATIONA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxGetConfigurationA(faxhandle: super::super::Foundation::HANDLE, faxconfig: *mut *mut FAX_CONFIGURATIONA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxGetConfigurationA(faxhandle.into_param().abi(), ::core::mem::transmute(faxconfig)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetConfigurationW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, faxconfig: *mut *mut FAX_CONFIGURATIONW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxGetConfigurationW(faxhandle: super::super::Foundation::HANDLE, faxconfig: *mut *mut FAX_CONFIGURATIONW) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxGetConfigurationW(faxhandle.into_param().abi(), ::core::mem::transmute(faxconfig)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetDeviceStatusA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(faxporthandle: Param0, devicestatus: *mut *mut FAX_DEVICE_STATUSA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxGetDeviceStatusA(faxporthandle: super::super::Foundation::HANDLE, devicestatus: *mut *mut FAX_DEVICE_STATUSA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxGetDeviceStatusA(faxporthandle.into_param().abi(), ::core::mem::transmute(devicestatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetDeviceStatusW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(faxporthandle: Param0, devicestatus: *mut *mut FAX_DEVICE_STATUSW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxGetDeviceStatusW(faxporthandle: super::super::Foundation::HANDLE, devicestatus: *mut *mut FAX_DEVICE_STATUSW) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxGetDeviceStatusW(faxporthandle.into_param().abi(), ::core::mem::transmute(devicestatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetJobA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, jobid: u32, jobentry: *mut *mut FAX_JOB_ENTRYA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxGetJobA(faxhandle: super::super::Foundation::HANDLE, jobid: u32, jobentry: *mut *mut FAX_JOB_ENTRYA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxGetJobA(faxhandle.into_param().abi(), ::core::mem::transmute(jobid), ::core::mem::transmute(jobentry)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetJobW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, jobid: u32, jobentry: *mut *mut FAX_JOB_ENTRYW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxGetJobW(faxhandle: super::super::Foundation::HANDLE, jobid: u32, jobentry: *mut *mut FAX_JOB_ENTRYW) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxGetJobW(faxhandle.into_param().abi(), ::core::mem::transmute(jobid), ::core::mem::transmute(jobentry)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetLoggingCategoriesA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, categories: *mut *mut FAX_LOG_CATEGORYA, numbercategories: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxGetLoggingCategoriesA(faxhandle: super::super::Foundation::HANDLE, categories: *mut *mut FAX_LOG_CATEGORYA, numbercategories: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxGetLoggingCategoriesA(faxhandle.into_param().abi(), ::core::mem::transmute(categories), ::core::mem::transmute(numbercategories)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetLoggingCategoriesW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, categories: *mut *mut FAX_LOG_CATEGORYW, numbercategories: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxGetLoggingCategoriesW(faxhandle: super::super::Foundation::HANDLE, categories: *mut *mut FAX_LOG_CATEGORYW, numbercategories: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxGetLoggingCategoriesW(faxhandle.into_param().abi(), ::core::mem::transmute(categories), ::core::mem::transmute(numbercategories)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetPageData<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, jobid: u32, buffer: *mut *mut u8, buffersize: *mut u32, imagewidth: *mut u32, imageheight: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxGetPageData(faxhandle: super::super::Foundation::HANDLE, jobid: u32, buffer: *mut *mut u8, buffersize: *mut u32, imagewidth: *mut u32, imageheight: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxGetPageData(faxhandle.into_param().abi(), ::core::mem::transmute(jobid), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize), ::core::mem::transmute(imagewidth), ::core::mem::transmute(imageheight)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetPortA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(faxporthandle: Param0, portinfo: *mut *mut FAX_PORT_INFOA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxGetPortA(faxporthandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxGetPortA(faxporthandle.into_param().abi(), ::core::mem::transmute(portinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetPortW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(faxporthandle: Param0, portinfo: *mut *mut FAX_PORT_INFOW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxGetPortW(faxporthandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOW) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxGetPortW(faxporthandle.into_param().abi(), ::core::mem::transmute(portinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetRoutingInfoA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(faxporthandle: Param0, routingguid: Param1, routinginfobuffer: *mut *mut u8, routinginfobuffersize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxGetRoutingInfoA(faxporthandle: super::super::Foundation::HANDLE, routingguid: ::windows::core::PCSTR, routinginfobuffer: *mut *mut u8, routinginfobuffersize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxGetRoutingInfoA(faxporthandle.into_param().abi(), routingguid.into_param().abi(), ::core::mem::transmute(routinginfobuffer), ::core::mem::transmute(routinginfobuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetRoutingInfoW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(faxporthandle: Param0, routingguid: Param1, routinginfobuffer: *mut *mut u8, routinginfobuffersize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxGetRoutingInfoW(faxporthandle: super::super::Foundation::HANDLE, routingguid: ::windows::core::PCWSTR, routinginfobuffer: *mut *mut u8, routinginfobuffersize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxGetRoutingInfoW(faxporthandle.into_param().abi(), routingguid.into_param().abi(), ::core::mem::transmute(routinginfobuffer), ::core::mem::transmute(routinginfobuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const FaxInboundRouting: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe80248ed_ad65_4218_8108_991924d4e7ed);
pub const FaxInboundRoutingExtension: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d7dfb51_7207_4436_a0d9_24e32ee56988);
pub const FaxInboundRoutingExtensions: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x189a48ed_623c_4c0d_80f2_d66c7b9efec2);
pub const FaxInboundRoutingMethod: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b9fd75c_0194_4b72_9ce5_02a8205ac7d4);
pub const FaxInboundRoutingMethods: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25fcb76a_b750_4b82_9266_fbbbae8922ba);
pub const FaxIncomingArchive: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8426c56a_35a1_4c6f_af93_fc952422e2c2);
pub const FaxIncomingJob: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc47311ec_ae32_41b8_ae4b_3eae0629d0c9);
pub const FaxIncomingJobs: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1bb8a43_8866_4fb7_a15d_6266c875a5cc);
pub const FaxIncomingMessage: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1932fcf7_9d43_4d5a_89ff_03861b321736);
pub const FaxIncomingMessageIterator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6088e1d8_3fc8_45c2_87b1_909a29607ea9);
pub const FaxIncomingQueue: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69131717_f3f1_40e3_809d_a6cbf7bd85e5);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxInitializeEventQueue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(faxhandle: Param0, completionport: Param1, completionkey: usize, hwnd: Param3, messagestart: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxInitializeEventQueue(faxhandle: super::super::Foundation::HANDLE, completionport: super::super::Foundation::HANDLE, completionkey: usize, hwnd: super::super::Foundation::HWND, messagestart: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxInitializeEventQueue(faxhandle.into_param().abi(), completionport.into_param().abi(), ::core::mem::transmute(completionkey), hwnd.into_param().abi(), ::core::mem::transmute(messagestart)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const FaxJobStatus: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7bf222f4_be8d_442f_841d_6132742423bb);
pub const FaxLoggingOptions: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bf9eea6_ece0_4785_a18b_de56e9eef96a);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxOpenPort<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, deviceid: u32, flags: u32, faxporthandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxOpenPort(faxhandle: super::super::Foundation::HANDLE, deviceid: u32, flags: u32, faxporthandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxOpenPort(faxhandle.into_param().abi(), ::core::mem::transmute(deviceid), ::core::mem::transmute(flags), ::core::mem::transmute(faxporthandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const FaxOutboundRouting: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc81b385e_b869_4afd_86c0_616498ed9be2);
pub const FaxOutboundRoutingGroup: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0213f3e0_6791_4d77_a271_04d2357c50d6);
pub const FaxOutboundRoutingGroups: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xccbea1a5_e2b4_4b57_9421_b04b6289464b);
pub const FaxOutboundRoutingRule: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6549eebf_08d1_475a_828b_3bf105952fa0);
pub const FaxOutboundRoutingRules: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd385beca_e624_4473_bfaa_9f4000831f54);
pub const FaxOutgoingArchive: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43c28403_e04f_474d_990c_b94669148f59);
pub const FaxOutgoingJob: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71bb429c_0ef9_4915_bec5_a5d897a3e924);
pub const FaxOutgoingJobs: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92bf2a6c_37be_43fa_a37d_cb0e5f753b35);
pub const FaxOutgoingMessage: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91b4a378_4ad8_4aef_a4dc_97d96e939a3a);
pub const FaxOutgoingMessageIterator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a3224d0_d30b_49de_9813_cb385790fbbb);
pub const FaxOutgoingQueue: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7421169e_8c43_4b0d_bb16_645c8fa40357);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn FaxPrintCoverPageA(faxcontextinfo: *const FAX_CONTEXT_INFOA, coverpageinfo: *const FAX_COVERPAGE_INFOA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxPrintCoverPageA(faxcontextinfo: *const FAX_CONTEXT_INFOA, coverpageinfo: *const FAX_COVERPAGE_INFOA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxPrintCoverPageA(::core::mem::transmute(faxcontextinfo), ::core::mem::transmute(coverpageinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn FaxPrintCoverPageW(faxcontextinfo: *const FAX_CONTEXT_INFOW, coverpageinfo: *const FAX_COVERPAGE_INFOW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxPrintCoverPageW(faxcontextinfo: *const FAX_CONTEXT_INFOW, coverpageinfo: *const FAX_COVERPAGE_INFOW) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxPrintCoverPageW(::core::mem::transmute(faxcontextinfo), ::core::mem::transmute(coverpageinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const FaxReceiptOptions: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6982487b_227b_4c96_a61c_248348b05ab6);
pub const FaxRecipient: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x60bf3301_7df8_4bd8_9148_7b5801f9efdf);
pub const FaxRecipients: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea9bdf53_10a9_4d4f_a067_63c8f84f01b0);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxRegisterRoutingExtensionW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(faxhandle: Param0, extensionname: Param1, friendlyname: Param2, imagename: Param3, callback: PFAX_ROUTING_INSTALLATION_CALLBACKW, context: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxRegisterRoutingExtensionW(faxhandle: super::super::Foundation::HANDLE, extensionname: ::windows::core::PCWSTR, friendlyname: ::windows::core::PCWSTR, imagename: ::windows::core::PCWSTR, callback: ::windows::core::RawPtr, context: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxRegisterRoutingExtensionW(faxhandle.into_param().abi(), extensionname.into_param().abi(), friendlyname.into_param().abi(), imagename.into_param().abi(), ::core::mem::transmute(callback), ::core::mem::transmute(context)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxRegisterServiceProviderW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(deviceprovider: Param0, friendlyname: Param1, imagename: Param2, tspname: Param3) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxRegisterServiceProviderW(deviceprovider: ::windows::core::PCWSTR, friendlyname: ::windows::core::PCWSTR, imagename: ::windows::core::PCWSTR, tspname: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxRegisterServiceProviderW(deviceprovider.into_param().abi(), friendlyname.into_param().abi(), imagename.into_param().abi(), tspname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const FaxSecurity: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10c4ddde_abf0_43df_964f_7f3ac21a4c7b);
pub const FaxSecurity2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x735c1248_ec89_4c30_a127_656e92e3c4ea);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSendDocumentA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(faxhandle: Param0, filename: Param1, jobparams: *mut FAX_JOB_PARAMA, coverpageinfo: *const FAX_COVERPAGE_INFOA, faxjobid: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxSendDocumentA(faxhandle: super::super::Foundation::HANDLE, filename: ::windows::core::PCSTR, jobparams: *mut FAX_JOB_PARAMA, coverpageinfo: *const FAX_COVERPAGE_INFOA, faxjobid: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxSendDocumentA(faxhandle.into_param().abi(), filename.into_param().abi(), ::core::mem::transmute(jobparams), ::core::mem::transmute(coverpageinfo), ::core::mem::transmute(faxjobid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSendDocumentForBroadcastA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(faxhandle: Param0, filename: Param1, faxjobid: *mut u32, faxrecipientcallback: PFAX_RECIPIENT_CALLBACKA, context: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxSendDocumentForBroadcastA(faxhandle: super::super::Foundation::HANDLE, filename: ::windows::core::PCSTR, faxjobid: *mut u32, faxrecipientcallback: ::windows::core::RawPtr, context: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxSendDocumentForBroadcastA(faxhandle.into_param().abi(), filename.into_param().abi(), ::core::mem::transmute(faxjobid), ::core::mem::transmute(faxrecipientcallback), ::core::mem::transmute(context)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSendDocumentForBroadcastW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(faxhandle: Param0, filename: Param1, faxjobid: *mut u32, faxrecipientcallback: PFAX_RECIPIENT_CALLBACKW, context: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxSendDocumentForBroadcastW(faxhandle: super::super::Foundation::HANDLE, filename: ::windows::core::PCWSTR, faxjobid: *mut u32, faxrecipientcallback: ::windows::core::RawPtr, context: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxSendDocumentForBroadcastW(faxhandle.into_param().abi(), filename.into_param().abi(), ::core::mem::transmute(faxjobid), ::core::mem::transmute(faxrecipientcallback), ::core::mem::transmute(context)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSendDocumentW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(faxhandle: Param0, filename: Param1, jobparams: *mut FAX_JOB_PARAMW, coverpageinfo: *const FAX_COVERPAGE_INFOW, faxjobid: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxSendDocumentW(faxhandle: super::super::Foundation::HANDLE, filename: ::windows::core::PCWSTR, jobparams: *mut FAX_JOB_PARAMW, coverpageinfo: *const FAX_COVERPAGE_INFOW, faxjobid: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxSendDocumentW(faxhandle.into_param().abi(), filename.into_param().abi(), ::core::mem::transmute(jobparams), ::core::mem::transmute(coverpageinfo), ::core::mem::transmute(faxjobid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const FaxSender: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x265d84d0_1850_4360_b7c8_758bbb5f0b96);
pub const FaxServer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcda8acb0_8cf5_4f6c_9ba2_5931d40c8cae);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSetConfigurationA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, faxconfig: *const FAX_CONFIGURATIONA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxSetConfigurationA(faxhandle: super::super::Foundation::HANDLE, faxconfig: *const FAX_CONFIGURATIONA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxSetConfigurationA(faxhandle.into_param().abi(), ::core::mem::transmute(faxconfig)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSetConfigurationW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, faxconfig: *const FAX_CONFIGURATIONW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxSetConfigurationW(faxhandle: super::super::Foundation::HANDLE, faxconfig: *const FAX_CONFIGURATIONW) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxSetConfigurationW(faxhandle.into_param().abi(), ::core::mem::transmute(faxconfig)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSetGlobalRoutingInfoA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, routinginfo: *const FAX_GLOBAL_ROUTING_INFOA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxSetGlobalRoutingInfoA(faxhandle: super::super::Foundation::HANDLE, routinginfo: *const FAX_GLOBAL_ROUTING_INFOA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxSetGlobalRoutingInfoA(faxhandle.into_param().abi(), ::core::mem::transmute(routinginfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSetGlobalRoutingInfoW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, routinginfo: *const FAX_GLOBAL_ROUTING_INFOW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxSetGlobalRoutingInfoW(faxhandle: super::super::Foundation::HANDLE, routinginfo: *const FAX_GLOBAL_ROUTING_INFOW) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxSetGlobalRoutingInfoW(faxhandle.into_param().abi(), ::core::mem::transmute(routinginfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSetJobA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, jobid: u32, command: u32, jobentry: *const FAX_JOB_ENTRYA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxSetJobA(faxhandle: super::super::Foundation::HANDLE, jobid: u32, command: u32, jobentry: *const FAX_JOB_ENTRYA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxSetJobA(faxhandle.into_param().abi(), ::core::mem::transmute(jobid), ::core::mem::transmute(command), ::core::mem::transmute(jobentry)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSetJobW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, jobid: u32, command: u32, jobentry: *const FAX_JOB_ENTRYW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxSetJobW(faxhandle: super::super::Foundation::HANDLE, jobid: u32, command: u32, jobentry: *const FAX_JOB_ENTRYW) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxSetJobW(faxhandle.into_param().abi(), ::core::mem::transmute(jobid), ::core::mem::transmute(command), ::core::mem::transmute(jobentry)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSetLoggingCategoriesA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, categories: *const FAX_LOG_CATEGORYA, numbercategories: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxSetLoggingCategoriesA(faxhandle: super::super::Foundation::HANDLE, categories: *const FAX_LOG_CATEGORYA, numbercategories: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxSetLoggingCategoriesA(faxhandle.into_param().abi(), ::core::mem::transmute(categories), ::core::mem::transmute(numbercategories)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSetLoggingCategoriesW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, categories: *const FAX_LOG_CATEGORYW, numbercategories: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxSetLoggingCategoriesW(faxhandle: super::super::Foundation::HANDLE, categories: *const FAX_LOG_CATEGORYW, numbercategories: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxSetLoggingCategoriesW(faxhandle.into_param().abi(), ::core::mem::transmute(categories), ::core::mem::transmute(numbercategories)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSetPortA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(faxporthandle: Param0, portinfo: *const FAX_PORT_INFOA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxSetPortA(faxporthandle: super::super::Foundation::HANDLE, portinfo: *const FAX_PORT_INFOA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxSetPortA(faxporthandle.into_param().abi(), ::core::mem::transmute(portinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSetPortW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(faxporthandle: Param0, portinfo: *const FAX_PORT_INFOW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxSetPortW(faxporthandle: super::super::Foundation::HANDLE, portinfo: *const FAX_PORT_INFOW) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxSetPortW(faxporthandle.into_param().abi(), ::core::mem::transmute(portinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSetRoutingInfoA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(faxporthandle: Param0, routingguid: Param1, routinginfobuffer: *const u8, routinginfobuffersize: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxSetRoutingInfoA(faxporthandle: super::super::Foundation::HANDLE, routingguid: ::windows::core::PCSTR, routinginfobuffer: *const u8, routinginfobuffersize: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxSetRoutingInfoA(faxporthandle.into_param().abi(), routingguid.into_param().abi(), ::core::mem::transmute(routinginfobuffer), ::core::mem::transmute(routinginfobuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSetRoutingInfoW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(faxporthandle: Param0, routingguid: Param1, routinginfobuffer: *const u8, routinginfobuffersize: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxSetRoutingInfoW(faxporthandle: super::super::Foundation::HANDLE, routingguid: ::windows::core::PCWSTR, routinginfobuffer: *const u8, routinginfobuffersize: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxSetRoutingInfoW(faxporthandle.into_param().abi(), routingguid.into_param().abi(), ::core::mem::transmute(routinginfobuffer), ::core::mem::transmute(routinginfobuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn FaxStartPrintJobA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(printername: Param0, printinfo: *const FAX_PRINT_INFOA, faxjobid: *mut u32, faxcontextinfo: *mut FAX_CONTEXT_INFOA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxStartPrintJobA(printername: ::windows::core::PCSTR, printinfo: *const FAX_PRINT_INFOA, faxjobid: *mut u32, faxcontextinfo: *mut FAX_CONTEXT_INFOA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxStartPrintJobA(printername.into_param().abi(), ::core::mem::transmute(printinfo), ::core::mem::transmute(faxjobid), ::core::mem::transmute(faxcontextinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn FaxStartPrintJobW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(printername: Param0, printinfo: *const FAX_PRINT_INFOW, faxjobid: *mut u32, faxcontextinfo: *mut FAX_CONTEXT_INFOW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxStartPrintJobW(printername: ::windows::core::PCWSTR, printinfo: *const FAX_PRINT_INFOW, faxjobid: *mut u32, faxcontextinfo: *mut FAX_CONTEXT_INFOW) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxStartPrintJobW(printername.into_param().abi(), ::core::mem::transmute(printinfo), ::core::mem::transmute(faxjobid), ::core::mem::transmute(faxcontextinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxUnregisterServiceProviderW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(deviceprovider: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxUnregisterServiceProviderW(deviceprovider: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FaxUnregisterServiceProviderW(deviceprovider.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const GUID_DeviceArrivedLaunch: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x740d9ee6_70f1_11d1_ad10_00a02438ad48);
pub const GUID_STIUserDefined1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc00eb795_8c6e_11d2_977a_0000f87a926f);
pub const GUID_STIUserDefined2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc77ae9c5_8c6e_11d2_977a_0000f87a926f);
pub const GUID_STIUserDefined3: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc77ae9c6_8c6e_11d2_977a_0000f87a926f);
pub const GUID_ScanFaxImage: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc00eb793_8c6e_11d2_977a_0000f87a926f);
pub const GUID_ScanImage: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6c5a715_8c6e_11d2_977a_0000f87a926f);
pub const GUID_ScanPrintImage: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb441f425_8c6e_11d2_977a_0000f87a926f);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxAccount(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccount {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AccountName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AccountName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Folders(&self) -> ::windows::core::Result<IFaxAccountFolders> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Folders)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxAccountFolders>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn ListenToAccountEvents(&self, eventtypes: FAX_ACCOUNT_EVENTS_TYPE_ENUM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ListenToAccountEvents)(::core::mem::transmute_copy(self), ::core::mem::transmute(eventtypes)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn RegisteredEvents(&self) -> ::windows::core::Result<FAX_ACCOUNT_EVENTS_TYPE_ENUM> {
        let mut result__: FAX_ACCOUNT_EVENTS_TYPE_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RegisteredEvents)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_ACCOUNT_EVENTS_TYPE_ENUM>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxAccount> for ::windows::core::IUnknown {
    fn from(value: IFaxAccount) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxAccount> for ::windows::core::IUnknown {
    fn from(value: &IFaxAccount) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxAccount {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxAccount {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxAccount> for super::super::System::Com::IDispatch {
    fn from(value: IFaxAccount) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxAccount> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxAccount) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxAccount {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxAccount {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxAccount {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxAccount {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxAccount {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxAccount {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxAccount").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxAccount {
    type Vtable = IFaxAccount_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68535b33_5dc4_4086_be26_b76f9b711006);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxAccount_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub AccountName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstraccountname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AccountName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Folders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfolders: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Folders: usize,
    pub ListenToAccountEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventtypes: FAX_ACCOUNT_EVENTS_TYPE_ENUM) -> ::windows::core::HRESULT,
    pub RegisteredEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pregisteredevents: *mut FAX_ACCOUNT_EVENTS_TYPE_ENUM) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxAccountFolders(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccountFolders {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OutgoingQueue(&self) -> ::windows::core::Result<IFaxAccountOutgoingQueue> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).OutgoingQueue)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxAccountOutgoingQueue>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IncomingQueue(&self) -> ::windows::core::Result<IFaxAccountIncomingQueue> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IncomingQueue)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxAccountIncomingQueue>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IncomingArchive(&self) -> ::windows::core::Result<IFaxAccountIncomingArchive> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IncomingArchive)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxAccountIncomingArchive>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OutgoingArchive(&self) -> ::windows::core::Result<IFaxAccountOutgoingArchive> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).OutgoingArchive)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxAccountOutgoingArchive>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxAccountFolders> for ::windows::core::IUnknown {
    fn from(value: IFaxAccountFolders) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxAccountFolders> for ::windows::core::IUnknown {
    fn from(value: &IFaxAccountFolders) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxAccountFolders {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxAccountFolders {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxAccountFolders> for super::super::System::Com::IDispatch {
    fn from(value: IFaxAccountFolders) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxAccountFolders> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxAccountFolders) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxAccountFolders {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxAccountFolders {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxAccountFolders {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxAccountFolders {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxAccountFolders {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxAccountFolders {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxAccountFolders").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxAccountFolders {
    type Vtable = IFaxAccountFolders_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6463f89d_23d8_46a9_8f86_c47b77ca7926);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxAccountFolders_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OutgoingQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxoutgoingqueue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OutgoingQueue: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub IncomingQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxincomingqueue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IncomingQueue: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub IncomingArchive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxincomingarchive: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IncomingArchive: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OutgoingArchive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxoutgoingarchive: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OutgoingArchive: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxAccountIncomingArchive(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccountIncomingArchive {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SizeLow(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SizeLow)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SizeHigh(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SizeHigh)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Refresh)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetMessages(&self, lprefetchsize: i32) -> ::windows::core::Result<IFaxIncomingMessageIterator> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMessages)(::core::mem::transmute_copy(self), ::core::mem::transmute(lprefetchsize), ::core::mem::transmute(&mut result__)).from_abi::<IFaxIncomingMessageIterator>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetMessage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrmessageid: Param0) -> ::windows::core::Result<IFaxIncomingMessage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMessage)(::core::mem::transmute_copy(self), bstrmessageid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFaxIncomingMessage>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxAccountIncomingArchive> for ::windows::core::IUnknown {
    fn from(value: IFaxAccountIncomingArchive) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxAccountIncomingArchive> for ::windows::core::IUnknown {
    fn from(value: &IFaxAccountIncomingArchive) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxAccountIncomingArchive {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxAccountIncomingArchive {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxAccountIncomingArchive> for super::super::System::Com::IDispatch {
    fn from(value: IFaxAccountIncomingArchive) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxAccountIncomingArchive> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxAccountIncomingArchive) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxAccountIncomingArchive {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxAccountIncomingArchive {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxAccountIncomingArchive {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxAccountIncomingArchive {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxAccountIncomingArchive {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxAccountIncomingArchive {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxAccountIncomingArchive").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxAccountIncomingArchive {
    type Vtable = IFaxAccountIncomingArchive_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8a5b6ef_e0d6_4aee_955c_91625bec9db4);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxAccountIncomingArchive_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    pub SizeLow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsizelow: *mut i32) -> ::windows::core::HRESULT,
    pub SizeHigh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsizehigh: *mut i32) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprefetchsize: i32, pfaxincomingmessageiterator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetMessages: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxincomingmessage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetMessage: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxAccountIncomingQueue(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccountIncomingQueue {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetJobs(&self) -> ::windows::core::Result<IFaxIncomingJobs> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetJobs)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxIncomingJobs>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetJob<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrjobid: Param0) -> ::windows::core::Result<IFaxIncomingJob> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetJob)(::core::mem::transmute_copy(self), bstrjobid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFaxIncomingJob>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxAccountIncomingQueue> for ::windows::core::IUnknown {
    fn from(value: IFaxAccountIncomingQueue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxAccountIncomingQueue> for ::windows::core::IUnknown {
    fn from(value: &IFaxAccountIncomingQueue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxAccountIncomingQueue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxAccountIncomingQueue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxAccountIncomingQueue> for super::super::System::Com::IDispatch {
    fn from(value: IFaxAccountIncomingQueue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxAccountIncomingQueue> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxAccountIncomingQueue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxAccountIncomingQueue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxAccountIncomingQueue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxAccountIncomingQueue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxAccountIncomingQueue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxAccountIncomingQueue {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxAccountIncomingQueue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxAccountIncomingQueue").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxAccountIncomingQueue {
    type Vtable = IFaxAccountIncomingQueue_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd142d92_0186_4a95_a090_cbc3eadba6b4);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxAccountIncomingQueue_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetJobs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxincomingjobs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetJobs: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxincomingjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetJob: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxAccountNotify(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccountNotify {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxAccountNotify> for ::windows::core::IUnknown {
    fn from(value: IFaxAccountNotify) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxAccountNotify> for ::windows::core::IUnknown {
    fn from(value: &IFaxAccountNotify) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxAccountNotify {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxAccountNotify {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxAccountNotify> for super::super::System::Com::IDispatch {
    fn from(value: IFaxAccountNotify) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxAccountNotify> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxAccountNotify) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxAccountNotify {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxAccountNotify {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxAccountNotify {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxAccountNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxAccountNotify {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxAccountNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxAccountNotify").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxAccountNotify {
    type Vtable = IFaxAccountNotify_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b5e5bd1_b8a9_47a0_a323_ef4a293ba06a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxAccountNotify_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxAccountOutgoingArchive(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccountOutgoingArchive {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SizeLow(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SizeLow)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SizeHigh(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SizeHigh)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Refresh)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetMessages(&self, lprefetchsize: i32) -> ::windows::core::Result<IFaxOutgoingMessageIterator> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMessages)(::core::mem::transmute_copy(self), ::core::mem::transmute(lprefetchsize), ::core::mem::transmute(&mut result__)).from_abi::<IFaxOutgoingMessageIterator>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetMessage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrmessageid: Param0) -> ::windows::core::Result<IFaxOutgoingMessage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMessage)(::core::mem::transmute_copy(self), bstrmessageid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFaxOutgoingMessage>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxAccountOutgoingArchive> for ::windows::core::IUnknown {
    fn from(value: IFaxAccountOutgoingArchive) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxAccountOutgoingArchive> for ::windows::core::IUnknown {
    fn from(value: &IFaxAccountOutgoingArchive) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxAccountOutgoingArchive {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxAccountOutgoingArchive {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxAccountOutgoingArchive> for super::super::System::Com::IDispatch {
    fn from(value: IFaxAccountOutgoingArchive) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxAccountOutgoingArchive> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxAccountOutgoingArchive) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxAccountOutgoingArchive {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxAccountOutgoingArchive {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxAccountOutgoingArchive {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxAccountOutgoingArchive {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxAccountOutgoingArchive {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxAccountOutgoingArchive {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxAccountOutgoingArchive").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxAccountOutgoingArchive {
    type Vtable = IFaxAccountOutgoingArchive_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5463076d_ec14_491f_926e_b3ceda5e5662);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxAccountOutgoingArchive_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    pub SizeLow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsizelow: *mut i32) -> ::windows::core::HRESULT,
    pub SizeHigh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsizehigh: *mut i32) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprefetchsize: i32, pfaxoutgoingmessageiterator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetMessages: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxoutgoingmessage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetMessage: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxAccountOutgoingQueue(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccountOutgoingQueue {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetJobs(&self) -> ::windows::core::Result<IFaxOutgoingJobs> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetJobs)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxOutgoingJobs>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetJob<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrjobid: Param0) -> ::windows::core::Result<IFaxOutgoingJob> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetJob)(::core::mem::transmute_copy(self), bstrjobid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFaxOutgoingJob>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxAccountOutgoingQueue> for ::windows::core::IUnknown {
    fn from(value: IFaxAccountOutgoingQueue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxAccountOutgoingQueue> for ::windows::core::IUnknown {
    fn from(value: &IFaxAccountOutgoingQueue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxAccountOutgoingQueue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxAccountOutgoingQueue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxAccountOutgoingQueue> for super::super::System::Com::IDispatch {
    fn from(value: IFaxAccountOutgoingQueue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxAccountOutgoingQueue> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxAccountOutgoingQueue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxAccountOutgoingQueue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxAccountOutgoingQueue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxAccountOutgoingQueue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxAccountOutgoingQueue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxAccountOutgoingQueue {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxAccountOutgoingQueue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxAccountOutgoingQueue").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxAccountOutgoingQueue {
    type Vtable = IFaxAccountOutgoingQueue_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f1424e9_f22d_4553_b7a5_0d24bd0d7e46);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxAccountOutgoingQueue_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetJobs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxoutgoingjobs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetJobs: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxoutgoingjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetJob: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxAccountSet(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccountSet {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAccounts(&self) -> ::windows::core::Result<IFaxAccounts> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAccounts)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxAccounts>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetAccount<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstraccountname: Param0) -> ::windows::core::Result<IFaxAccount> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAccount)(::core::mem::transmute_copy(self), bstraccountname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFaxAccount>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddAccount<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstraccountname: Param0) -> ::windows::core::Result<IFaxAccount> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AddAccount)(::core::mem::transmute_copy(self), bstraccountname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFaxAccount>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoveAccount<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstraccountname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAccount)(::core::mem::transmute_copy(self), bstraccountname.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxAccountSet> for ::windows::core::IUnknown {
    fn from(value: IFaxAccountSet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxAccountSet> for ::windows::core::IUnknown {
    fn from(value: &IFaxAccountSet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxAccountSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxAccountSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxAccountSet> for super::super::System::Com::IDispatch {
    fn from(value: IFaxAccountSet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxAccountSet> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxAccountSet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxAccountSet {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxAccountSet {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxAccountSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxAccountSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxAccountSet {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxAccountSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxAccountSet").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxAccountSet {
    type Vtable = IFaxAccountSet_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7428fbae_841e_47b8_86f4_2288946dca1b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxAccountSet_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetAccounts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfaxaccounts: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetAccounts: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstraccountname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxaccount: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetAccount: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AddAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstraccountname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxaccount: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AddAccount: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoveAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstraccountname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoveAccount: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxAccounts(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccounts {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, vindex: Param0) -> ::windows::core::Result<IFaxAccount> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::core::mem::transmute_copy(self), vindex.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFaxAccount>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxAccounts> for ::windows::core::IUnknown {
    fn from(value: IFaxAccounts) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxAccounts> for ::windows::core::IUnknown {
    fn from(value: &IFaxAccounts) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxAccounts {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxAccounts {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxAccounts> for super::super::System::Com::IDispatch {
    fn from(value: IFaxAccounts) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxAccounts> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxAccounts) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxAccounts {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxAccounts {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxAccounts {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxAccounts {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxAccounts {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxAccounts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxAccounts").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxAccounts {
    type Vtable = IFaxAccounts_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93ea8162_8be7_42d1_ae7b_ec74e2d989da);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxAccounts_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxaccount: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxActivity(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxActivity {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn IncomingMessages(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IncomingMessages)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn RoutingMessages(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RoutingMessages)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn OutgoingMessages(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).OutgoingMessages)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn QueuedMessages(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).QueuedMessages)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Refresh)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxActivity> for ::windows::core::IUnknown {
    fn from(value: IFaxActivity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxActivity> for ::windows::core::IUnknown {
    fn from(value: &IFaxActivity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxActivity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxActivity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxActivity> for super::super::System::Com::IDispatch {
    fn from(value: IFaxActivity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxActivity> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxActivity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxActivity {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxActivity {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxActivity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxActivity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxActivity {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxActivity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxActivity").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxActivity {
    type Vtable = IFaxActivity_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b106f97_3df5_40f2_bc3c_44cb8115ebdf);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxActivity_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    pub IncomingMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plincomingmessages: *mut i32) -> ::windows::core::HRESULT,
    pub RoutingMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plroutingmessages: *mut i32) -> ::windows::core::HRESULT,
    pub OutgoingMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ploutgoingmessages: *mut i32) -> ::windows::core::HRESULT,
    pub QueuedMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plqueuedmessages: *mut i32) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxActivityLogging(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxActivityLogging {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn LogIncoming(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LogIncoming)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetLogIncoming(&self, blogincoming: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLogIncoming)(::core::mem::transmute_copy(self), ::core::mem::transmute(blogincoming)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn LogOutgoing(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LogOutgoing)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetLogOutgoing(&self, blogoutgoing: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLogOutgoing)(::core::mem::transmute_copy(self), ::core::mem::transmute(blogoutgoing)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DatabasePath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DatabasePath)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDatabasePath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdatabasepath: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDatabasePath)(::core::mem::transmute_copy(self), bstrdatabasepath.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Refresh)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Save(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Save)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxActivityLogging> for ::windows::core::IUnknown {
    fn from(value: IFaxActivityLogging) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxActivityLogging> for ::windows::core::IUnknown {
    fn from(value: &IFaxActivityLogging) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxActivityLogging {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxActivityLogging {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxActivityLogging> for super::super::System::Com::IDispatch {
    fn from(value: IFaxActivityLogging) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxActivityLogging> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxActivityLogging) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxActivityLogging {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxActivityLogging {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxActivityLogging {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxActivityLogging {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxActivityLogging {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxActivityLogging {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxActivityLogging").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxActivityLogging {
    type Vtable = IFaxActivityLogging_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e29078b_5a69_497b_9592_49b7e7faddb5);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxActivityLogging_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    pub LogIncoming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblogincoming: *mut i16) -> ::windows::core::HRESULT,
    pub SetLogIncoming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blogincoming: i16) -> ::windows::core::HRESULT,
    pub LogOutgoing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblogoutgoing: *mut i16) -> ::windows::core::HRESULT,
    pub SetLogOutgoing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blogoutgoing: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DatabasePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdatabasepath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DatabasePath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDatabasePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdatabasepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDatabasePath: usize,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxConfiguration(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxConfiguration {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn UseArchive(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).UseArchive)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetUseArchive(&self, busearchive: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUseArchive)(::core::mem::transmute_copy(self), ::core::mem::transmute(busearchive)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ArchiveLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ArchiveLocation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetArchiveLocation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrarchivelocation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetArchiveLocation)(::core::mem::transmute_copy(self), bstrarchivelocation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SizeQuotaWarning(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SizeQuotaWarning)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetSizeQuotaWarning(&self, bsizequotawarning: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSizeQuotaWarning)(::core::mem::transmute_copy(self), ::core::mem::transmute(bsizequotawarning)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn HighQuotaWaterMark(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).HighQuotaWaterMark)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetHighQuotaWaterMark(&self, lhighquotawatermark: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetHighQuotaWaterMark)(::core::mem::transmute_copy(self), ::core::mem::transmute(lhighquotawatermark)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn LowQuotaWaterMark(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LowQuotaWaterMark)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetLowQuotaWaterMark(&self, llowquotawatermark: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLowQuotaWaterMark)(::core::mem::transmute_copy(self), ::core::mem::transmute(llowquotawatermark)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn ArchiveAgeLimit(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ArchiveAgeLimit)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetArchiveAgeLimit(&self, larchiveagelimit: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetArchiveAgeLimit)(::core::mem::transmute_copy(self), ::core::mem::transmute(larchiveagelimit)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn ArchiveSizeLow(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ArchiveSizeLow)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn ArchiveSizeHigh(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ArchiveSizeHigh)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn OutgoingQueueBlocked(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).OutgoingQueueBlocked)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetOutgoingQueueBlocked(&self, boutgoingblocked: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOutgoingQueueBlocked)(::core::mem::transmute_copy(self), ::core::mem::transmute(boutgoingblocked)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn OutgoingQueuePaused(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).OutgoingQueuePaused)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetOutgoingQueuePaused(&self, boutgoingpaused: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOutgoingQueuePaused)(::core::mem::transmute_copy(self), ::core::mem::transmute(boutgoingpaused)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn AllowPersonalCoverPages(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AllowPersonalCoverPages)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetAllowPersonalCoverPages(&self, ballowpersonalcoverpages: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllowPersonalCoverPages)(::core::mem::transmute_copy(self), ::core::mem::transmute(ballowpersonalcoverpages)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn UseDeviceTSID(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).UseDeviceTSID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetUseDeviceTSID(&self, busedevicetsid: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUseDeviceTSID)(::core::mem::transmute_copy(self), ::core::mem::transmute(busedevicetsid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Retries(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Retries)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetRetries(&self, lretries: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRetries)(::core::mem::transmute_copy(self), ::core::mem::transmute(lretries)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn RetryDelay(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RetryDelay)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetRetryDelay(&self, lretrydelay: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRetryDelay)(::core::mem::transmute_copy(self), ::core::mem::transmute(lretrydelay)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn DiscountRateStart(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DiscountRateStart)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetDiscountRateStart(&self, datediscountratestart: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDiscountRateStart)(::core::mem::transmute_copy(self), ::core::mem::transmute(datediscountratestart)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn DiscountRateEnd(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DiscountRateEnd)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetDiscountRateEnd(&self, datediscountrateend: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDiscountRateEnd)(::core::mem::transmute_copy(self), ::core::mem::transmute(datediscountrateend)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn OutgoingQueueAgeLimit(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).OutgoingQueueAgeLimit)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetOutgoingQueueAgeLimit(&self, loutgoingqueueagelimit: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOutgoingQueueAgeLimit)(::core::mem::transmute_copy(self), ::core::mem::transmute(loutgoingqueueagelimit)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Branding(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Branding)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetBranding(&self, bbranding: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBranding)(::core::mem::transmute_copy(self), ::core::mem::transmute(bbranding)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn IncomingQueueBlocked(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IncomingQueueBlocked)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetIncomingQueueBlocked(&self, bincomingblocked: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetIncomingQueueBlocked)(::core::mem::transmute_copy(self), ::core::mem::transmute(bincomingblocked)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn AutoCreateAccountOnConnect(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AutoCreateAccountOnConnect)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetAutoCreateAccountOnConnect(&self, bautocreateaccountonconnect: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAutoCreateAccountOnConnect)(::core::mem::transmute_copy(self), ::core::mem::transmute(bautocreateaccountonconnect)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn IncomingFaxesArePublic(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IncomingFaxesArePublic)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetIncomingFaxesArePublic(&self, bincomingfaxesarepublic: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetIncomingFaxesArePublic)(::core::mem::transmute_copy(self), ::core::mem::transmute(bincomingfaxesarepublic)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Refresh)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Save(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Save)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxConfiguration> for ::windows::core::IUnknown {
    fn from(value: IFaxConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxConfiguration> for ::windows::core::IUnknown {
    fn from(value: &IFaxConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxConfiguration> for super::super::System::Com::IDispatch {
    fn from(value: IFaxConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxConfiguration> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxConfiguration {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxConfiguration").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxConfiguration {
    type Vtable = IFaxConfiguration_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10f4d0f7_0994_4543_ab6e_506949128c40);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxConfiguration_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    pub UseArchive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbusearchive: *mut i16) -> ::windows::core::HRESULT,
    pub SetUseArchive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, busearchive: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ArchiveLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrarchivelocation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ArchiveLocation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetArchiveLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrarchivelocation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetArchiveLocation: usize,
    pub SizeQuotaWarning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsizequotawarning: *mut i16) -> ::windows::core::HRESULT,
    pub SetSizeQuotaWarning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bsizequotawarning: i16) -> ::windows::core::HRESULT,
    pub HighQuotaWaterMark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plhighquotawatermark: *mut i32) -> ::windows::core::HRESULT,
    pub SetHighQuotaWaterMark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lhighquotawatermark: i32) -> ::windows::core::HRESULT,
    pub LowQuotaWaterMark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pllowquotawatermark: *mut i32) -> ::windows::core::HRESULT,
    pub SetLowQuotaWaterMark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, llowquotawatermark: i32) -> ::windows::core::HRESULT,
    pub ArchiveAgeLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plarchiveagelimit: *mut i32) -> ::windows::core::HRESULT,
    pub SetArchiveAgeLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, larchiveagelimit: i32) -> ::windows::core::HRESULT,
    pub ArchiveSizeLow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsizelow: *mut i32) -> ::windows::core::HRESULT,
    pub ArchiveSizeHigh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsizehigh: *mut i32) -> ::windows::core::HRESULT,
    pub OutgoingQueueBlocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pboutgoingblocked: *mut i16) -> ::windows::core::HRESULT,
    pub SetOutgoingQueueBlocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, boutgoingblocked: i16) -> ::windows::core::HRESULT,
    pub OutgoingQueuePaused: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pboutgoingpaused: *mut i16) -> ::windows::core::HRESULT,
    pub SetOutgoingQueuePaused: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, boutgoingpaused: i16) -> ::windows::core::HRESULT,
    pub AllowPersonalCoverPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pballowpersonalcoverpages: *mut i16) -> ::windows::core::HRESULT,
    pub SetAllowPersonalCoverPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ballowpersonalcoverpages: i16) -> ::windows::core::HRESULT,
    pub UseDeviceTSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbusedevicetsid: *mut i16) -> ::windows::core::HRESULT,
    pub SetUseDeviceTSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, busedevicetsid: i16) -> ::windows::core::HRESULT,
    pub Retries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows::core::HRESULT,
    pub SetRetries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lretries: i32) -> ::windows::core::HRESULT,
    pub RetryDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plretrydelay: *mut i32) -> ::windows::core::HRESULT,
    pub SetRetryDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lretrydelay: i32) -> ::windows::core::HRESULT,
    pub DiscountRateStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatediscountratestart: *mut f64) -> ::windows::core::HRESULT,
    pub SetDiscountRateStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datediscountratestart: f64) -> ::windows::core::HRESULT,
    pub DiscountRateEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatediscountrateend: *mut f64) -> ::windows::core::HRESULT,
    pub SetDiscountRateEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datediscountrateend: f64) -> ::windows::core::HRESULT,
    pub OutgoingQueueAgeLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ploutgoingqueueagelimit: *mut i32) -> ::windows::core::HRESULT,
    pub SetOutgoingQueueAgeLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loutgoingqueueagelimit: i32) -> ::windows::core::HRESULT,
    pub Branding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbbranding: *mut i16) -> ::windows::core::HRESULT,
    pub SetBranding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bbranding: i16) -> ::windows::core::HRESULT,
    pub IncomingQueueBlocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbincomingblocked: *mut i16) -> ::windows::core::HRESULT,
    pub SetIncomingQueueBlocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bincomingblocked: i16) -> ::windows::core::HRESULT,
    pub AutoCreateAccountOnConnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbautocreateaccountonconnect: *mut i16) -> ::windows::core::HRESULT,
    pub SetAutoCreateAccountOnConnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bautocreateaccountonconnect: i16) -> ::windows::core::HRESULT,
    pub IncomingFaxesArePublic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbincomingfaxesarepublic: *mut i16) -> ::windows::core::HRESULT,
    pub SetIncomingFaxesArePublic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bincomingfaxesarepublic: i16) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxDevice(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxDevice {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Id(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Id)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeviceName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DeviceName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProviderUniqueName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ProviderUniqueName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn PoweredOff(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PoweredOff)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn ReceivingNow(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ReceivingNow)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SendingNow(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SendingNow)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn UsedRoutingMethods(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).UsedRoutingMethods)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdescription: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDescription)(::core::mem::transmute_copy(self), bstrdescription.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SendEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SendEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetSendEnabled(&self, bsendenabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSendEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(bsendenabled)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn ReceiveMode(&self) -> ::windows::core::Result<FAX_DEVICE_RECEIVE_MODE_ENUM> {
        let mut result__: FAX_DEVICE_RECEIVE_MODE_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ReceiveMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_DEVICE_RECEIVE_MODE_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetReceiveMode(&self, receivemode: FAX_DEVICE_RECEIVE_MODE_ENUM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetReceiveMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(receivemode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn RingsBeforeAnswer(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RingsBeforeAnswer)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetRingsBeforeAnswer(&self, lringsbeforeanswer: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRingsBeforeAnswer)(::core::mem::transmute_copy(self), ::core::mem::transmute(lringsbeforeanswer)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CSID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CSID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCSID<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrcsid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCSID)(::core::mem::transmute_copy(self), bstrcsid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TSID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TSID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTSID<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrtsid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTSID)(::core::mem::transmute_copy(self), bstrtsid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Refresh)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Save(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Save)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetExtensionProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrguid: Param0) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetExtensionProperty)(::core::mem::transmute_copy(self), bstrguid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetExtensionProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, bstrguid: Param0, vproperty: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetExtensionProperty)(::core::mem::transmute_copy(self), bstrguid.into_param().abi(), vproperty.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UseRoutingMethod<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrmethodguid: Param0, buse: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UseRoutingMethod)(::core::mem::transmute_copy(self), bstrmethodguid.into_param().abi(), ::core::mem::transmute(buse)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn RingingNow(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RingingNow)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn AnswerCall(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AnswerCall)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxDevice> for ::windows::core::IUnknown {
    fn from(value: IFaxDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxDevice> for ::windows::core::IUnknown {
    fn from(value: &IFaxDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxDevice> for super::super::System::Com::IDispatch {
    fn from(value: IFaxDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxDevice> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxDevice {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxDevice {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxDevice {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxDevice").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxDevice {
    type Vtable = IFaxDevice_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49306c59_b52e_4867_9df4_ca5841c956d0);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxDevice_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DeviceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeviceName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ProviderUniqueName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprovideruniquename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ProviderUniqueName: usize,
    pub PoweredOff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbpoweredoff: *mut i16) -> ::windows::core::HRESULT,
    pub ReceivingNow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbreceivingnow: *mut i16) -> ::windows::core::HRESULT,
    pub SendingNow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsendingnow: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub UsedRoutingMethods: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvusedroutingmethods: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    UsedRoutingMethods: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
    pub SendEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsendenabled: *mut i16) -> ::windows::core::HRESULT,
    pub SetSendEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bsendenabled: i16) -> ::windows::core::HRESULT,
    pub ReceiveMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preceivemode: *mut FAX_DEVICE_RECEIVE_MODE_ENUM) -> ::windows::core::HRESULT,
    pub SetReceiveMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, receivemode: FAX_DEVICE_RECEIVE_MODE_ENUM) -> ::windows::core::HRESULT,
    pub RingsBeforeAnswer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plringsbeforeanswer: *mut i32) -> ::windows::core::HRESULT,
    pub SetRingsBeforeAnswer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lringsbeforeanswer: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTSID: usize,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetExtensionProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvproperty: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetExtensionProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetExtensionProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, vproperty: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetExtensionProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UseRoutingMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmethodguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, buse: i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UseRoutingMethod: usize,
    pub RingingNow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbringingnow: *mut i16) -> ::windows::core::HRESULT,
    pub AnswerCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxDeviceIds(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxDeviceIds {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Item(&self, lindex: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::core::mem::transmute_copy(self), ::core::mem::transmute(lindex), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Add(&self, ldeviceid: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Add)(::core::mem::transmute_copy(self), ::core::mem::transmute(ldeviceid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Remove(&self, lindex: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Remove)(::core::mem::transmute_copy(self), ::core::mem::transmute(lindex)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetOrder(&self, ldeviceid: i32, lneworder: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOrder)(::core::mem::transmute_copy(self), ::core::mem::transmute(ldeviceid), ::core::mem::transmute(lneworder)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxDeviceIds> for ::windows::core::IUnknown {
    fn from(value: IFaxDeviceIds) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxDeviceIds> for ::windows::core::IUnknown {
    fn from(value: &IFaxDeviceIds) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxDeviceIds {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxDeviceIds {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxDeviceIds> for super::super::System::Com::IDispatch {
    fn from(value: IFaxDeviceIds) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxDeviceIds> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxDeviceIds) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxDeviceIds {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxDeviceIds {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxDeviceIds {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxDeviceIds {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxDeviceIds {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxDeviceIds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxDeviceIds").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxDeviceIds {
    type Vtable = IFaxDeviceIds_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f0f813f_4ce9_443e_8ca1_738cfaeee149);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxDeviceIds_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pldeviceid: *mut i32) -> ::windows::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ldeviceid: i32) -> ::windows::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows::core::HRESULT,
    pub SetOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ldeviceid: i32, lneworder: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxDeviceProvider(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxDeviceProvider {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FriendlyName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FriendlyName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImageName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ImageName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UniqueName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).UniqueName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TapiProviderName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TapiProviderName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn MajorVersion(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MajorVersion)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn MinorVersion(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MinorVersion)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn MajorBuild(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MajorBuild)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn MinorBuild(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MinorBuild)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Debug(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Debug)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Status(&self) -> ::windows::core::Result<FAX_PROVIDER_STATUS_ENUM> {
        let mut result__: FAX_PROVIDER_STATUS_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Status)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_PROVIDER_STATUS_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn InitErrorCode(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).InitErrorCode)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeviceIds(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DeviceIds)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxDeviceProvider> for ::windows::core::IUnknown {
    fn from(value: IFaxDeviceProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxDeviceProvider> for ::windows::core::IUnknown {
    fn from(value: &IFaxDeviceProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxDeviceProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxDeviceProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxDeviceProvider> for super::super::System::Com::IDispatch {
    fn from(value: IFaxDeviceProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxDeviceProvider> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxDeviceProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxDeviceProvider {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxDeviceProvider {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxDeviceProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxDeviceProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxDeviceProvider {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxDeviceProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxDeviceProvider").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxDeviceProvider {
    type Vtable = IFaxDeviceProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x290eac63_83ec_449c_8417_f148df8c682a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxDeviceProvider_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub FriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfriendlyname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FriendlyName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ImageName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrimagename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ImageName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UniqueName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstruniquename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UniqueName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TapiProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtapiprovidername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TapiProviderName: usize,
    pub MajorVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32) -> ::windows::core::HRESULT,
    pub MinorVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plminorversion: *mut i32) -> ::windows::core::HRESULT,
    pub MajorBuild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmajorbuild: *mut i32) -> ::windows::core::HRESULT,
    pub MinorBuild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plminorbuild: *mut i32) -> ::windows::core::HRESULT,
    pub Debug: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbdebug: *mut i16) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_PROVIDER_STATUS_ENUM) -> ::windows::core::HRESULT,
    pub InitErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pliniterrorcode: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeviceIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvdeviceids: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeviceIds: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxDeviceProviders(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxDeviceProviders {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, vindex: Param0) -> ::windows::core::Result<IFaxDeviceProvider> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::core::mem::transmute_copy(self), vindex.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFaxDeviceProvider>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxDeviceProviders> for ::windows::core::IUnknown {
    fn from(value: IFaxDeviceProviders) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxDeviceProviders> for ::windows::core::IUnknown {
    fn from(value: &IFaxDeviceProviders) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxDeviceProviders {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxDeviceProviders {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxDeviceProviders> for super::super::System::Com::IDispatch {
    fn from(value: IFaxDeviceProviders) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxDeviceProviders> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxDeviceProviders) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxDeviceProviders {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxDeviceProviders {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxDeviceProviders {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxDeviceProviders {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxDeviceProviders {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxDeviceProviders {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxDeviceProviders").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxDeviceProviders {
    type Vtable = IFaxDeviceProviders_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9fb76f62_4c7e_43a5_b6fd_502893f7e13e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxDeviceProviders_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxdeviceprovider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxDevices(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxDevices {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, vindex: Param0) -> ::windows::core::Result<IFaxDevice> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::core::mem::transmute_copy(self), vindex.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFaxDevice>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ItemById(&self, lid: i32) -> ::windows::core::Result<IFaxDevice> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ItemById)(::core::mem::transmute_copy(self), ::core::mem::transmute(lid), ::core::mem::transmute(&mut result__)).from_abi::<IFaxDevice>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxDevices> for ::windows::core::IUnknown {
    fn from(value: IFaxDevices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxDevices> for ::windows::core::IUnknown {
    fn from(value: &IFaxDevices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxDevices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxDevices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxDevices> for super::super::System::Com::IDispatch {
    fn from(value: IFaxDevices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxDevices> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxDevices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxDevices {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxDevices {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxDevices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxDevices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxDevices {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxDevices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxDevices").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxDevices {
    type Vtable = IFaxDevices_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e46783e_f34f_482e_a360_0416becbbd96);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxDevices_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ItemById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lid: i32, ppfaxdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ItemById: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxDocument(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxDocument {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Body(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Body)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBody<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrbody: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBody)(::core::mem::transmute_copy(self), bstrbody.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Sender(&self) -> ::windows::core::Result<IFaxSender> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Sender)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxSender>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Recipients(&self) -> ::windows::core::Result<IFaxRecipients> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Recipients)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxRecipients>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CoverPage(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CoverPage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCoverPage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrcoverpage: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCoverPage)(::core::mem::transmute_copy(self), bstrcoverpage.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Subject(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Subject)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSubject<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsubject: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSubject)(::core::mem::transmute_copy(self), bstrsubject.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Note(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Note)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNote<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrnote: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetNote)(::core::mem::transmute_copy(self), bstrnote.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn ScheduleTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ScheduleTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetScheduleTime(&self, datescheduletime: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetScheduleTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(datescheduletime)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReceiptAddress(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ReceiptAddress)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiptAddress<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrreceiptaddress: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetReceiptAddress)(::core::mem::transmute_copy(self), bstrreceiptaddress.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DocumentName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DocumentName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDocumentName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdocumentname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDocumentName)(::core::mem::transmute_copy(self), bstrdocumentname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn CallHandle(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CallHandle)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetCallHandle(&self, lcallhandle: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCallHandle)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcallhandle)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn CoverPageType(&self) -> ::windows::core::Result<FAX_COVERPAGE_TYPE_ENUM> {
        let mut result__: FAX_COVERPAGE_TYPE_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CoverPageType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_COVERPAGE_TYPE_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetCoverPageType(&self, coverpagetype: FAX_COVERPAGE_TYPE_ENUM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCoverPageType)(::core::mem::transmute_copy(self), ::core::mem::transmute(coverpagetype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn ScheduleType(&self) -> ::windows::core::Result<FAX_SCHEDULE_TYPE_ENUM> {
        let mut result__: FAX_SCHEDULE_TYPE_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ScheduleType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_SCHEDULE_TYPE_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetScheduleType(&self, scheduletype: FAX_SCHEDULE_TYPE_ENUM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetScheduleType)(::core::mem::transmute_copy(self), ::core::mem::transmute(scheduletype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn ReceiptType(&self) -> ::windows::core::Result<FAX_RECEIPT_TYPE_ENUM> {
        let mut result__: FAX_RECEIPT_TYPE_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ReceiptType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_RECEIPT_TYPE_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetReceiptType(&self, receipttype: FAX_RECEIPT_TYPE_ENUM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetReceiptType)(::core::mem::transmute_copy(self), ::core::mem::transmute(receipttype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn GroupBroadcastReceipts(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GroupBroadcastReceipts)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetGroupBroadcastReceipts(&self, busegrouping: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGroupBroadcastReceipts)(::core::mem::transmute_copy(self), ::core::mem::transmute(busegrouping)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Priority(&self) -> ::windows::core::Result<FAX_PRIORITY_TYPE_ENUM> {
        let mut result__: FAX_PRIORITY_TYPE_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Priority)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_PRIORITY_TYPE_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetPriority(&self, priority: FAX_PRIORITY_TYPE_ENUM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPriority)(::core::mem::transmute_copy(self), ::core::mem::transmute(priority)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TapiConnection(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TapiConnection)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_TapiConnection<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch>>(&self, ptapiconnection: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).putref_TapiConnection)(::core::mem::transmute_copy(self), ptapiconnection.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Submit<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrfaxservername: Param0) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Submit)(::core::mem::transmute_copy(self), bstrfaxservername.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ConnectedSubmit<'a, Param0: ::windows::core::IntoParam<'a, IFaxServer>>(&self, pfaxserver: Param0) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ConnectedSubmit)(::core::mem::transmute_copy(self), pfaxserver.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn AttachFaxToReceipt(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AttachFaxToReceipt)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetAttachFaxToReceipt(&self, battachfax: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAttachFaxToReceipt)(::core::mem::transmute_copy(self), ::core::mem::transmute(battachfax)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxDocument> for ::windows::core::IUnknown {
    fn from(value: IFaxDocument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxDocument> for ::windows::core::IUnknown {
    fn from(value: &IFaxDocument) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxDocument> for super::super::System::Com::IDispatch {
    fn from(value: IFaxDocument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxDocument> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxDocument) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxDocument {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxDocument {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxDocument {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxDocument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxDocument {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxDocument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxDocument").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxDocument {
    type Vtable = IFaxDocument_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb207a246_09e3_4a4e_a7dc_fea31d29458f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxDocument_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Body: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrbody: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Body: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrbody: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBody: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Sender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfaxsender: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Sender: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Recipients: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfaxrecipients: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Recipients: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CoverPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcoverpage: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CoverPage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCoverPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcoverpage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCoverPage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsubject: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Subject: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSubject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsubject: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSubject: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Note: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrnote: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Note: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNote: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrnote: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNote: usize,
    pub ScheduleTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatescheduletime: *mut f64) -> ::windows::core::HRESULT,
    pub SetScheduleTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datescheduletime: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ReceiptAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrreceiptaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReceiptAddress: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetReceiptAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrreceiptaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetReceiptAddress: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DocumentName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdocumentname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DocumentName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDocumentName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdocumentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDocumentName: usize,
    pub CallHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcallhandle: *mut i32) -> ::windows::core::HRESULT,
    pub SetCallHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcallhandle: i32) -> ::windows::core::HRESULT,
    pub CoverPageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcoverpagetype: *mut FAX_COVERPAGE_TYPE_ENUM) -> ::windows::core::HRESULT,
    pub SetCoverPageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coverpagetype: FAX_COVERPAGE_TYPE_ENUM) -> ::windows::core::HRESULT,
    pub ScheduleType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pscheduletype: *mut FAX_SCHEDULE_TYPE_ENUM) -> ::windows::core::HRESULT,
    pub SetScheduleType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scheduletype: FAX_SCHEDULE_TYPE_ENUM) -> ::windows::core::HRESULT,
    pub ReceiptType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preceipttype: *mut FAX_RECEIPT_TYPE_ENUM) -> ::windows::core::HRESULT,
    pub SetReceiptType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, receipttype: FAX_RECEIPT_TYPE_ENUM) -> ::windows::core::HRESULT,
    pub GroupBroadcastReceipts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbusegrouping: *mut i16) -> ::windows::core::HRESULT,
    pub SetGroupBroadcastReceipts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, busegrouping: i16) -> ::windows::core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppriority: *mut FAX_PRIORITY_TYPE_ENUM) -> ::windows::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, priority: FAX_PRIORITY_TYPE_ENUM) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub TapiConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptapiconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TapiConnection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_TapiConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptapiconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_TapiConnection: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Submit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfaxservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvfaxoutgoingjobids: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Submit: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ConnectedSubmit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, pvfaxoutgoingjobids: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ConnectedSubmit: usize,
    pub AttachFaxToReceipt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbattachfax: *mut i16) -> ::windows::core::HRESULT,
    pub SetAttachFaxToReceipt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, battachfax: i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxDocument2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxDocument2 {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Body(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Body)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBody<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrbody: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetBody)(::core::mem::transmute_copy(self), bstrbody.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Sender(&self) -> ::windows::core::Result<IFaxSender> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Sender)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxSender>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Recipients(&self) -> ::windows::core::Result<IFaxRecipients> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Recipients)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxRecipients>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CoverPage(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CoverPage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCoverPage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrcoverpage: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetCoverPage)(::core::mem::transmute_copy(self), bstrcoverpage.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Subject(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Subject)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSubject<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsubject: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetSubject)(::core::mem::transmute_copy(self), bstrsubject.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Note(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Note)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNote<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrnote: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetNote)(::core::mem::transmute_copy(self), bstrnote.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn ScheduleTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ScheduleTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetScheduleTime(&self, datescheduletime: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetScheduleTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(datescheduletime)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReceiptAddress(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ReceiptAddress)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiptAddress<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrreceiptaddress: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetReceiptAddress)(::core::mem::transmute_copy(self), bstrreceiptaddress.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DocumentName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.DocumentName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDocumentName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdocumentname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetDocumentName)(::core::mem::transmute_copy(self), bstrdocumentname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn CallHandle(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CallHandle)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetCallHandle(&self, lcallhandle: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetCallHandle)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcallhandle)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn CoverPageType(&self) -> ::windows::core::Result<FAX_COVERPAGE_TYPE_ENUM> {
        let mut result__: FAX_COVERPAGE_TYPE_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CoverPageType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_COVERPAGE_TYPE_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetCoverPageType(&self, coverpagetype: FAX_COVERPAGE_TYPE_ENUM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetCoverPageType)(::core::mem::transmute_copy(self), ::core::mem::transmute(coverpagetype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn ScheduleType(&self) -> ::windows::core::Result<FAX_SCHEDULE_TYPE_ENUM> {
        let mut result__: FAX_SCHEDULE_TYPE_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ScheduleType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_SCHEDULE_TYPE_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetScheduleType(&self, scheduletype: FAX_SCHEDULE_TYPE_ENUM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetScheduleType)(::core::mem::transmute_copy(self), ::core::mem::transmute(scheduletype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn ReceiptType(&self) -> ::windows::core::Result<FAX_RECEIPT_TYPE_ENUM> {
        let mut result__: FAX_RECEIPT_TYPE_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ReceiptType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_RECEIPT_TYPE_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetReceiptType(&self, receipttype: FAX_RECEIPT_TYPE_ENUM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetReceiptType)(::core::mem::transmute_copy(self), ::core::mem::transmute(receipttype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn GroupBroadcastReceipts(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GroupBroadcastReceipts)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetGroupBroadcastReceipts(&self, busegrouping: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetGroupBroadcastReceipts)(::core::mem::transmute_copy(self), ::core::mem::transmute(busegrouping)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Priority(&self) -> ::windows::core::Result<FAX_PRIORITY_TYPE_ENUM> {
        let mut result__: FAX_PRIORITY_TYPE_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Priority)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_PRIORITY_TYPE_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetPriority(&self, priority: FAX_PRIORITY_TYPE_ENUM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetPriority)(::core::mem::transmute_copy(self), ::core::mem::transmute(priority)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TapiConnection(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.TapiConnection)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_TapiConnection<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch>>(&self, ptapiconnection: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.putref_TapiConnection)(::core::mem::transmute_copy(self), ptapiconnection.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Submit<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrfaxservername: Param0) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Submit)(::core::mem::transmute_copy(self), bstrfaxservername.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ConnectedSubmit<'a, Param0: ::windows::core::IntoParam<'a, IFaxServer>>(&self, pfaxserver: Param0) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ConnectedSubmit)(::core::mem::transmute_copy(self), pfaxserver.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn AttachFaxToReceipt(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.AttachFaxToReceipt)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetAttachFaxToReceipt(&self, battachfax: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetAttachFaxToReceipt)(::core::mem::transmute_copy(self), ::core::mem::transmute(battachfax)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SubmissionId(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SubmissionId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Bodies(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Bodies)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetBodies<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, vbodies: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBodies)(::core::mem::transmute_copy(self), vbodies.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Submit2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrfaxservername: Param0, pvfaxoutgoingjobids: *mut super::super::System::Com::VARIANT, plerrorbodyfile: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Submit2)(::core::mem::transmute_copy(self), bstrfaxservername.into_param().abi(), ::core::mem::transmute(pvfaxoutgoingjobids), ::core::mem::transmute(plerrorbodyfile)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ConnectedSubmit2<'a, Param0: ::windows::core::IntoParam<'a, IFaxServer>>(&self, pfaxserver: Param0, pvfaxoutgoingjobids: *mut super::super::System::Com::VARIANT, plerrorbodyfile: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ConnectedSubmit2)(::core::mem::transmute_copy(self), pfaxserver.into_param().abi(), ::core::mem::transmute(pvfaxoutgoingjobids), ::core::mem::transmute(plerrorbodyfile)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxDocument2> for ::windows::core::IUnknown {
    fn from(value: IFaxDocument2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxDocument2> for ::windows::core::IUnknown {
    fn from(value: &IFaxDocument2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxDocument2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxDocument2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxDocument2> for super::super::System::Com::IDispatch {
    fn from(value: IFaxDocument2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxDocument2> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxDocument2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxDocument2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxDocument2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxDocument2> for IFaxDocument {
    fn from(value: IFaxDocument2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxDocument2> for IFaxDocument {
    fn from(value: &IFaxDocument2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFaxDocument> for IFaxDocument2 {
    fn into_param(self) -> ::windows::core::Param<'a, IFaxDocument> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFaxDocument> for &'a IFaxDocument2 {
    fn into_param(self) -> ::windows::core::Param<'a, IFaxDocument> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxDocument2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxDocument2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxDocument2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxDocument2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxDocument2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxDocument2 {
    type Vtable = IFaxDocument2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1347661_f9ef_4d6d_b4a5_c0a068b65cff);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxDocument2_Vtbl {
    pub base: IFaxDocument_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SubmissionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsubmissionid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SubmissionId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Bodies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvbodies: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Bodies: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetBodies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vbodies: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetBodies: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Submit2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfaxservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvfaxoutgoingjobids: *mut super::super::System::Com::VARIANT, plerrorbodyfile: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Submit2: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ConnectedSubmit2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, pvfaxoutgoingjobids: *mut super::super::System::Com::VARIANT, plerrorbodyfile: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ConnectedSubmit2: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxEventLogging(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxEventLogging {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn InitEventsLevel(&self) -> ::windows::core::Result<FAX_LOG_LEVEL_ENUM> {
        let mut result__: FAX_LOG_LEVEL_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).InitEventsLevel)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_LOG_LEVEL_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetInitEventsLevel(&self, initeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInitEventsLevel)(::core::mem::transmute_copy(self), ::core::mem::transmute(initeventlevel)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn InboundEventsLevel(&self) -> ::windows::core::Result<FAX_LOG_LEVEL_ENUM> {
        let mut result__: FAX_LOG_LEVEL_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).InboundEventsLevel)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_LOG_LEVEL_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetInboundEventsLevel(&self, inboundeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInboundEventsLevel)(::core::mem::transmute_copy(self), ::core::mem::transmute(inboundeventlevel)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn OutboundEventsLevel(&self) -> ::windows::core::Result<FAX_LOG_LEVEL_ENUM> {
        let mut result__: FAX_LOG_LEVEL_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).OutboundEventsLevel)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_LOG_LEVEL_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetOutboundEventsLevel(&self, outboundeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOutboundEventsLevel)(::core::mem::transmute_copy(self), ::core::mem::transmute(outboundeventlevel)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn GeneralEventsLevel(&self) -> ::windows::core::Result<FAX_LOG_LEVEL_ENUM> {
        let mut result__: FAX_LOG_LEVEL_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GeneralEventsLevel)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_LOG_LEVEL_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetGeneralEventsLevel(&self, generaleventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGeneralEventsLevel)(::core::mem::transmute_copy(self), ::core::mem::transmute(generaleventlevel)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Refresh)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Save(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Save)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxEventLogging> for ::windows::core::IUnknown {
    fn from(value: IFaxEventLogging) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxEventLogging> for ::windows::core::IUnknown {
    fn from(value: &IFaxEventLogging) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxEventLogging {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxEventLogging {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxEventLogging> for super::super::System::Com::IDispatch {
    fn from(value: IFaxEventLogging) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxEventLogging> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxEventLogging) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxEventLogging {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxEventLogging {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxEventLogging {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxEventLogging {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxEventLogging {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxEventLogging {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxEventLogging").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxEventLogging {
    type Vtable = IFaxEventLogging_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0880d965_20e8_42e4_8e17_944f192caad4);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxEventLogging_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    pub InitEventsLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piniteventlevel: *mut FAX_LOG_LEVEL_ENUM) -> ::windows::core::HRESULT,
    pub SetInitEventsLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows::core::HRESULT,
    pub InboundEventsLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinboundeventlevel: *mut FAX_LOG_LEVEL_ENUM) -> ::windows::core::HRESULT,
    pub SetInboundEventsLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inboundeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows::core::HRESULT,
    pub OutboundEventsLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poutboundeventlevel: *mut FAX_LOG_LEVEL_ENUM) -> ::windows::core::HRESULT,
    pub SetOutboundEventsLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outboundeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows::core::HRESULT,
    pub GeneralEventsLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgeneraleventlevel: *mut FAX_LOG_LEVEL_ENUM) -> ::windows::core::HRESULT,
    pub SetGeneralEventsLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, generaleventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxFolders(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxFolders {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OutgoingQueue(&self) -> ::windows::core::Result<IFaxOutgoingQueue> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).OutgoingQueue)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxOutgoingQueue>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IncomingQueue(&self) -> ::windows::core::Result<IFaxIncomingQueue> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IncomingQueue)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxIncomingQueue>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IncomingArchive(&self) -> ::windows::core::Result<IFaxIncomingArchive> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IncomingArchive)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxIncomingArchive>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OutgoingArchive(&self) -> ::windows::core::Result<IFaxOutgoingArchive> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).OutgoingArchive)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxOutgoingArchive>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxFolders> for ::windows::core::IUnknown {
    fn from(value: IFaxFolders) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxFolders> for ::windows::core::IUnknown {
    fn from(value: &IFaxFolders) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxFolders {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxFolders {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxFolders> for super::super::System::Com::IDispatch {
    fn from(value: IFaxFolders) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxFolders> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxFolders) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxFolders {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxFolders {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxFolders {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxFolders {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxFolders {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxFolders {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxFolders").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxFolders {
    type Vtable = IFaxFolders_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdce3b2a8_a7ab_42bc_9d0a_3149457261a0);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxFolders_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OutgoingQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxoutgoingqueue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OutgoingQueue: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub IncomingQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxincomingqueue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IncomingQueue: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub IncomingArchive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxincomingarchive: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IncomingArchive: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OutgoingArchive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxoutgoingarchive: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OutgoingArchive: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxInboundRouting(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxInboundRouting {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetExtensions(&self) -> ::windows::core::Result<IFaxInboundRoutingExtensions> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetExtensions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxInboundRoutingExtensions>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetMethods(&self) -> ::windows::core::Result<IFaxInboundRoutingMethods> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMethods)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxInboundRoutingMethods>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxInboundRouting> for ::windows::core::IUnknown {
    fn from(value: IFaxInboundRouting) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxInboundRouting> for ::windows::core::IUnknown {
    fn from(value: &IFaxInboundRouting) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxInboundRouting {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxInboundRouting {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxInboundRouting> for super::super::System::Com::IDispatch {
    fn from(value: IFaxInboundRouting) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxInboundRouting> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxInboundRouting) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxInboundRouting {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxInboundRouting {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxInboundRouting {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxInboundRouting {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxInboundRouting {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxInboundRouting {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxInboundRouting").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxInboundRouting {
    type Vtable = IFaxInboundRouting_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8148c20f_9d52_45b1_bf96_38fc12713527);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxInboundRouting_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxinboundroutingextensions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetExtensions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetMethods: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxinboundroutingmethods: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetMethods: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxInboundRoutingExtension(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxInboundRoutingExtension {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FriendlyName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FriendlyName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImageName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ImageName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UniqueName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).UniqueName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn MajorVersion(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MajorVersion)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn MinorVersion(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MinorVersion)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn MajorBuild(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MajorBuild)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn MinorBuild(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MinorBuild)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Debug(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Debug)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Status(&self) -> ::windows::core::Result<FAX_PROVIDER_STATUS_ENUM> {
        let mut result__: FAX_PROVIDER_STATUS_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Status)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_PROVIDER_STATUS_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn InitErrorCode(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).InitErrorCode)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Methods(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Methods)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxInboundRoutingExtension> for ::windows::core::IUnknown {
    fn from(value: IFaxInboundRoutingExtension) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxInboundRoutingExtension> for ::windows::core::IUnknown {
    fn from(value: &IFaxInboundRoutingExtension) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxInboundRoutingExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxInboundRoutingExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxInboundRoutingExtension> for super::super::System::Com::IDispatch {
    fn from(value: IFaxInboundRoutingExtension) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxInboundRoutingExtension> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxInboundRoutingExtension) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxInboundRoutingExtension {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxInboundRoutingExtension {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxInboundRoutingExtension {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxInboundRoutingExtension {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxInboundRoutingExtension {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxInboundRoutingExtension {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxInboundRoutingExtension").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxInboundRoutingExtension {
    type Vtable = IFaxInboundRoutingExtension_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x885b5e08_c26c_4ef9_af83_51580a750be1);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxInboundRoutingExtension_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub FriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfriendlyname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FriendlyName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ImageName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrimagename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ImageName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UniqueName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstruniquename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UniqueName: usize,
    pub MajorVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32) -> ::windows::core::HRESULT,
    pub MinorVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plminorversion: *mut i32) -> ::windows::core::HRESULT,
    pub MajorBuild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmajorbuild: *mut i32) -> ::windows::core::HRESULT,
    pub MinorBuild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plminorbuild: *mut i32) -> ::windows::core::HRESULT,
    pub Debug: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbdebug: *mut i16) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_PROVIDER_STATUS_ENUM) -> ::windows::core::HRESULT,
    pub InitErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pliniterrorcode: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Methods: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvmethods: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Methods: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxInboundRoutingExtensions(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxInboundRoutingExtensions {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, vindex: Param0) -> ::windows::core::Result<IFaxInboundRoutingExtension> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::core::mem::transmute_copy(self), vindex.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFaxInboundRoutingExtension>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxInboundRoutingExtensions> for ::windows::core::IUnknown {
    fn from(value: IFaxInboundRoutingExtensions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxInboundRoutingExtensions> for ::windows::core::IUnknown {
    fn from(value: &IFaxInboundRoutingExtensions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxInboundRoutingExtensions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxInboundRoutingExtensions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxInboundRoutingExtensions> for super::super::System::Com::IDispatch {
    fn from(value: IFaxInboundRoutingExtensions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxInboundRoutingExtensions> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxInboundRoutingExtensions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxInboundRoutingExtensions {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxInboundRoutingExtensions {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxInboundRoutingExtensions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxInboundRoutingExtensions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxInboundRoutingExtensions {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxInboundRoutingExtensions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxInboundRoutingExtensions").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxInboundRoutingExtensions {
    type Vtable = IFaxInboundRoutingExtensions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f6c9673_7b26_42de_8eb0_915dcd2a4f4c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxInboundRoutingExtensions_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxinboundroutingextension: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxInboundRoutingMethod(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxInboundRoutingMethod {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Name)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GUID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GUID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FunctionName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FunctionName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExtensionFriendlyName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ExtensionFriendlyName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExtensionImageName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ExtensionImageName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Priority(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Priority)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetPriority(&self, lpriority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPriority)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpriority)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Refresh)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Save(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Save)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxInboundRoutingMethod> for ::windows::core::IUnknown {
    fn from(value: IFaxInboundRoutingMethod) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxInboundRoutingMethod> for ::windows::core::IUnknown {
    fn from(value: &IFaxInboundRoutingMethod) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxInboundRoutingMethod {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxInboundRoutingMethod {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxInboundRoutingMethod> for super::super::System::Com::IDispatch {
    fn from(value: IFaxInboundRoutingMethod) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxInboundRoutingMethod> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxInboundRoutingMethod) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxInboundRoutingMethod {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxInboundRoutingMethod {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxInboundRoutingMethod {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxInboundRoutingMethod {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxInboundRoutingMethod {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxInboundRoutingMethod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxInboundRoutingMethod").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxInboundRoutingMethod {
    type Vtable = IFaxInboundRoutingMethod_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45700061_ad9d_4776_a8c4_64065492cf4b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxInboundRoutingMethod_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GUID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrguid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GUID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FunctionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfunctionname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FunctionName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExtensionFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrextensionfriendlyname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExtensionFriendlyName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExtensionImageName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrextensionimagename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExtensionImageName: usize,
    pub Priority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxInboundRoutingMethods(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxInboundRoutingMethods {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, vindex: Param0) -> ::windows::core::Result<IFaxInboundRoutingMethod> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::core::mem::transmute_copy(self), vindex.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFaxInboundRoutingMethod>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxInboundRoutingMethods> for ::windows::core::IUnknown {
    fn from(value: IFaxInboundRoutingMethods) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxInboundRoutingMethods> for ::windows::core::IUnknown {
    fn from(value: &IFaxInboundRoutingMethods) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxInboundRoutingMethods {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxInboundRoutingMethods {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxInboundRoutingMethods> for super::super::System::Com::IDispatch {
    fn from(value: IFaxInboundRoutingMethods) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxInboundRoutingMethods> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxInboundRoutingMethods) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxInboundRoutingMethods {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxInboundRoutingMethods {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxInboundRoutingMethods {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxInboundRoutingMethods {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxInboundRoutingMethods {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxInboundRoutingMethods {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxInboundRoutingMethods").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxInboundRoutingMethods {
    type Vtable = IFaxInboundRoutingMethods_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x783fca10_8908_4473_9d69_f67fbea0c6b9);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxInboundRoutingMethods_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxinboundroutingmethod: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxIncomingArchive(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxIncomingArchive {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn UseArchive(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).UseArchive)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetUseArchive(&self, busearchive: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUseArchive)(::core::mem::transmute_copy(self), ::core::mem::transmute(busearchive)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ArchiveFolder(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ArchiveFolder)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetArchiveFolder<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrarchivefolder: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetArchiveFolder)(::core::mem::transmute_copy(self), bstrarchivefolder.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SizeQuotaWarning(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SizeQuotaWarning)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetSizeQuotaWarning(&self, bsizequotawarning: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSizeQuotaWarning)(::core::mem::transmute_copy(self), ::core::mem::transmute(bsizequotawarning)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn HighQuotaWaterMark(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).HighQuotaWaterMark)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetHighQuotaWaterMark(&self, lhighquotawatermark: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetHighQuotaWaterMark)(::core::mem::transmute_copy(self), ::core::mem::transmute(lhighquotawatermark)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn LowQuotaWaterMark(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LowQuotaWaterMark)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetLowQuotaWaterMark(&self, llowquotawatermark: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLowQuotaWaterMark)(::core::mem::transmute_copy(self), ::core::mem::transmute(llowquotawatermark)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn AgeLimit(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AgeLimit)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetAgeLimit(&self, lagelimit: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAgeLimit)(::core::mem::transmute_copy(self), ::core::mem::transmute(lagelimit)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SizeLow(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SizeLow)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SizeHigh(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SizeHigh)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Refresh)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Save(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Save)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetMessages(&self, lprefetchsize: i32) -> ::windows::core::Result<IFaxIncomingMessageIterator> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMessages)(::core::mem::transmute_copy(self), ::core::mem::transmute(lprefetchsize), ::core::mem::transmute(&mut result__)).from_abi::<IFaxIncomingMessageIterator>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetMessage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrmessageid: Param0) -> ::windows::core::Result<IFaxIncomingMessage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMessage)(::core::mem::transmute_copy(self), bstrmessageid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFaxIncomingMessage>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxIncomingArchive> for ::windows::core::IUnknown {
    fn from(value: IFaxIncomingArchive) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxIncomingArchive> for ::windows::core::IUnknown {
    fn from(value: &IFaxIncomingArchive) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxIncomingArchive {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxIncomingArchive {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxIncomingArchive> for super::super::System::Com::IDispatch {
    fn from(value: IFaxIncomingArchive) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxIncomingArchive> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxIncomingArchive) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxIncomingArchive {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxIncomingArchive {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxIncomingArchive {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxIncomingArchive {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxIncomingArchive {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxIncomingArchive {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxIncomingArchive").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxIncomingArchive {
    type Vtable = IFaxIncomingArchive_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76062cc7_f714_4fbd_aa06_ed6e4a4b70f3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxIncomingArchive_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    pub UseArchive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbusearchive: *mut i16) -> ::windows::core::HRESULT,
    pub SetUseArchive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, busearchive: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ArchiveFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrarchivefolder: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ArchiveFolder: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetArchiveFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrarchivefolder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetArchiveFolder: usize,
    pub SizeQuotaWarning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsizequotawarning: *mut i16) -> ::windows::core::HRESULT,
    pub SetSizeQuotaWarning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bsizequotawarning: i16) -> ::windows::core::HRESULT,
    pub HighQuotaWaterMark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plhighquotawatermark: *mut i32) -> ::windows::core::HRESULT,
    pub SetHighQuotaWaterMark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lhighquotawatermark: i32) -> ::windows::core::HRESULT,
    pub LowQuotaWaterMark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pllowquotawatermark: *mut i32) -> ::windows::core::HRESULT,
    pub SetLowQuotaWaterMark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, llowquotawatermark: i32) -> ::windows::core::HRESULT,
    pub AgeLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plagelimit: *mut i32) -> ::windows::core::HRESULT,
    pub SetAgeLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lagelimit: i32) -> ::windows::core::HRESULT,
    pub SizeLow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsizelow: *mut i32) -> ::windows::core::HRESULT,
    pub SizeHigh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsizehigh: *mut i32) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprefetchsize: i32, pfaxincomingmessageiterator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetMessages: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxincomingmessage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetMessage: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxIncomingJob(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxIncomingJob {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Size(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Size)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Id(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Id)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn CurrentPage(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CurrentPage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn DeviceId(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DeviceId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Status(&self) -> ::windows::core::Result<FAX_JOB_STATUS_ENUM> {
        let mut result__: FAX_JOB_STATUS_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Status)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_JOB_STATUS_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn ExtendedStatusCode(&self) -> ::windows::core::Result<FAX_JOB_EXTENDED_STATUS_ENUM> {
        let mut result__: FAX_JOB_EXTENDED_STATUS_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ExtendedStatusCode)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_JOB_EXTENDED_STATUS_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExtendedStatus(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ExtendedStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn AvailableOperations(&self) -> ::windows::core::Result<FAX_JOB_OPERATIONS_ENUM> {
        let mut result__: FAX_JOB_OPERATIONS_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AvailableOperations)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_JOB_OPERATIONS_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Retries(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Retries)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn TransmissionStart(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TransmissionStart)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn TransmissionEnd(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TransmissionEnd)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CSID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CSID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TSID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TSID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CallerId(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CallerId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RoutingInformation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RoutingInformation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn JobType(&self) -> ::windows::core::Result<FAX_JOB_TYPE_ENUM> {
        let mut result__: FAX_JOB_TYPE_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).JobType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_JOB_TYPE_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Cancel)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Refresh)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTiff<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrtiffpath: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CopyTiff)(::core::mem::transmute_copy(self), bstrtiffpath.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxIncomingJob> for ::windows::core::IUnknown {
    fn from(value: IFaxIncomingJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxIncomingJob> for ::windows::core::IUnknown {
    fn from(value: &IFaxIncomingJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxIncomingJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxIncomingJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxIncomingJob> for super::super::System::Com::IDispatch {
    fn from(value: IFaxIncomingJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxIncomingJob> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxIncomingJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxIncomingJob {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxIncomingJob {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxIncomingJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxIncomingJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxIncomingJob {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxIncomingJob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxIncomingJob").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxIncomingJob {
    type Vtable = IFaxIncomingJob_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x207529e6_654a_4916_9f88_4d232ee8a107);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxIncomingJob_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Id: usize,
    pub CurrentPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcurrentpage: *mut i32) -> ::windows::core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pldeviceid: *mut i32) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_JOB_STATUS_ENUM) -> ::windows::core::HRESULT,
    pub ExtendedStatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pextendedstatuscode: *mut FAX_JOB_EXTENDED_STATUS_ENUM) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ExtendedStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrextendedstatus: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExtendedStatus: usize,
    pub AvailableOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pavailableoperations: *mut FAX_JOB_OPERATIONS_ENUM) -> ::windows::core::HRESULT,
    pub Retries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows::core::HRESULT,
    pub TransmissionStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatetransmissionstart: *mut f64) -> ::windows::core::HRESULT,
    pub TransmissionEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatetransmissionend: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CallerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcallerid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CallerId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RoutingInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrroutinginformation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RoutingInformation: usize,
    pub JobType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pjobtype: *mut FAX_JOB_TYPE_ENUM) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CopyTiff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtiffpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CopyTiff: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxIncomingJobs(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxIncomingJobs {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, vindex: Param0) -> ::windows::core::Result<IFaxIncomingJob> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::core::mem::transmute_copy(self), vindex.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFaxIncomingJob>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxIncomingJobs> for ::windows::core::IUnknown {
    fn from(value: IFaxIncomingJobs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxIncomingJobs> for ::windows::core::IUnknown {
    fn from(value: &IFaxIncomingJobs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxIncomingJobs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxIncomingJobs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxIncomingJobs> for super::super::System::Com::IDispatch {
    fn from(value: IFaxIncomingJobs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxIncomingJobs> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxIncomingJobs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxIncomingJobs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxIncomingJobs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxIncomingJobs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxIncomingJobs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxIncomingJobs {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxIncomingJobs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxIncomingJobs").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxIncomingJobs {
    type Vtable = IFaxIncomingJobs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x011f04e9_4fd6_4c23_9513_b6b66bb26be9);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxIncomingJobs_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxincomingjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxIncomingMessage(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxIncomingMessage {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Id(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Id)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Pages(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Pages)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Size(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Size)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeviceName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DeviceName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Retries(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Retries)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn TransmissionStart(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TransmissionStart)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn TransmissionEnd(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TransmissionEnd)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CSID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CSID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TSID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TSID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CallerId(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CallerId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RoutingInformation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RoutingInformation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTiff<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrtiffpath: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CopyTiff)(::core::mem::transmute_copy(self), bstrtiffpath.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Delete)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxIncomingMessage> for ::windows::core::IUnknown {
    fn from(value: IFaxIncomingMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxIncomingMessage> for ::windows::core::IUnknown {
    fn from(value: &IFaxIncomingMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxIncomingMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxIncomingMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxIncomingMessage> for super::super::System::Com::IDispatch {
    fn from(value: IFaxIncomingMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxIncomingMessage> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxIncomingMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxIncomingMessage {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxIncomingMessage {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxIncomingMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxIncomingMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxIncomingMessage {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxIncomingMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxIncomingMessage").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxIncomingMessage {
    type Vtable = IFaxIncomingMessage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7cab88fa_2ef9_4851_b2f3_1d148fed8447);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxIncomingMessage_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Id: usize,
    pub Pages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plpages: *mut i32) -> ::windows::core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DeviceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeviceName: usize,
    pub Retries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows::core::HRESULT,
    pub TransmissionStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatetransmissionstart: *mut f64) -> ::windows::core::HRESULT,
    pub TransmissionEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatetransmissionend: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CallerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcallerid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CallerId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RoutingInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrroutinginformation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RoutingInformation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CopyTiff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtiffpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CopyTiff: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxIncomingMessage2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxIncomingMessage2 {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Id(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Id)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Pages(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Pages)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Size(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Size)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeviceName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.DeviceName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Retries(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Retries)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn TransmissionStart(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.TransmissionStart)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn TransmissionEnd(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.TransmissionEnd)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CSID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CSID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TSID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.TSID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CallerId(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CallerId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RoutingInformation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.RoutingInformation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTiff<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrtiffpath: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.CopyTiff)(::core::mem::transmute_copy(self), bstrtiffpath.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Subject(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Subject)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSubject<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsubject: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSubject)(::core::mem::transmute_copy(self), bstrsubject.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SenderName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SenderName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSenderName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsendername: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSenderName)(::core::mem::transmute_copy(self), bstrsendername.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SenderFaxNumber(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SenderFaxNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSenderFaxNumber<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsenderfaxnumber: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSenderFaxNumber)(::core::mem::transmute_copy(self), bstrsenderfaxnumber.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn HasCoverPage(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).HasCoverPage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetHasCoverPage(&self, bhascoverpage: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetHasCoverPage)(::core::mem::transmute_copy(self), ::core::mem::transmute(bhascoverpage)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Recipients(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Recipients)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRecipients<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrrecipients: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRecipients)(::core::mem::transmute_copy(self), bstrrecipients.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn WasReAssigned(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).WasReAssigned)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Read(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Read)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetRead(&self, bread: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRead)(::core::mem::transmute_copy(self), ::core::mem::transmute(bread)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn ReAssign(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReAssign)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Save(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Save)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Refresh)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxIncomingMessage2> for ::windows::core::IUnknown {
    fn from(value: IFaxIncomingMessage2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxIncomingMessage2> for ::windows::core::IUnknown {
    fn from(value: &IFaxIncomingMessage2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxIncomingMessage2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxIncomingMessage2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxIncomingMessage2> for super::super::System::Com::IDispatch {
    fn from(value: IFaxIncomingMessage2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxIncomingMessage2> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxIncomingMessage2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxIncomingMessage2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxIncomingMessage2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxIncomingMessage2> for IFaxIncomingMessage {
    fn from(value: IFaxIncomingMessage2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxIncomingMessage2> for IFaxIncomingMessage {
    fn from(value: &IFaxIncomingMessage2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFaxIncomingMessage> for IFaxIncomingMessage2 {
    fn into_param(self) -> ::windows::core::Param<'a, IFaxIncomingMessage> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFaxIncomingMessage> for &'a IFaxIncomingMessage2 {
    fn into_param(self) -> ::windows::core::Param<'a, IFaxIncomingMessage> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxIncomingMessage2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxIncomingMessage2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxIncomingMessage2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxIncomingMessage2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxIncomingMessage2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxIncomingMessage2 {
    type Vtable = IFaxIncomingMessage2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9208503_e2bc_48f3_9ec0_e6236f9b509a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxIncomingMessage2_Vtbl {
    pub base: IFaxIncomingMessage_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsubject: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Subject: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSubject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsubject: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSubject: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SenderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsendername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SenderName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSenderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsendername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSenderName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SenderFaxNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsenderfaxnumber: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SenderFaxNumber: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSenderFaxNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsenderfaxnumber: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSenderFaxNumber: usize,
    pub HasCoverPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbhascoverpage: *mut i16) -> ::windows::core::HRESULT,
    pub SetHasCoverPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bhascoverpage: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Recipients: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrrecipients: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Recipients: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRecipients: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrecipients: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRecipients: usize,
    pub WasReAssigned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbwasreassigned: *mut i16) -> ::windows::core::HRESULT,
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbread: *mut i16) -> ::windows::core::HRESULT,
    pub SetRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bread: i16) -> ::windows::core::HRESULT,
    pub ReAssign: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxIncomingMessageIterator(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxIncomingMessageIterator {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Message(&self) -> ::windows::core::Result<IFaxIncomingMessage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Message)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxIncomingMessage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn PrefetchSize(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PrefetchSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetPrefetchSize(&self, lprefetchsize: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPrefetchSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(lprefetchsize)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn AtEOF(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AtEOF)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn MoveFirst(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MoveFirst)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MoveNext)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxIncomingMessageIterator> for ::windows::core::IUnknown {
    fn from(value: IFaxIncomingMessageIterator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxIncomingMessageIterator> for ::windows::core::IUnknown {
    fn from(value: &IFaxIncomingMessageIterator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxIncomingMessageIterator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxIncomingMessageIterator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxIncomingMessageIterator> for super::super::System::Com::IDispatch {
    fn from(value: IFaxIncomingMessageIterator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxIncomingMessageIterator> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxIncomingMessageIterator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxIncomingMessageIterator {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxIncomingMessageIterator {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxIncomingMessageIterator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxIncomingMessageIterator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxIncomingMessageIterator {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxIncomingMessageIterator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxIncomingMessageIterator").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxIncomingMessageIterator {
    type Vtable = IFaxIncomingMessageIterator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd73ecc4_6f06_4f52_82a8_f7ba06ae3108);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxIncomingMessageIterator_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxincomingmessage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Message: usize,
    pub PrefetchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprefetchsize: *mut i32) -> ::windows::core::HRESULT,
    pub SetPrefetchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprefetchsize: i32) -> ::windows::core::HRESULT,
    pub AtEOF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbeof: *mut i16) -> ::windows::core::HRESULT,
    pub MoveFirst: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxIncomingQueue(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxIncomingQueue {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Blocked(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Blocked)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetBlocked(&self, bblocked: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBlocked)(::core::mem::transmute_copy(self), ::core::mem::transmute(bblocked)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Refresh)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Save(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Save)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetJobs(&self) -> ::windows::core::Result<IFaxIncomingJobs> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetJobs)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxIncomingJobs>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetJob<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrjobid: Param0) -> ::windows::core::Result<IFaxIncomingJob> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetJob)(::core::mem::transmute_copy(self), bstrjobid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFaxIncomingJob>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxIncomingQueue> for ::windows::core::IUnknown {
    fn from(value: IFaxIncomingQueue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxIncomingQueue> for ::windows::core::IUnknown {
    fn from(value: &IFaxIncomingQueue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxIncomingQueue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxIncomingQueue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxIncomingQueue> for super::super::System::Com::IDispatch {
    fn from(value: IFaxIncomingQueue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxIncomingQueue> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxIncomingQueue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxIncomingQueue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxIncomingQueue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxIncomingQueue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxIncomingQueue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxIncomingQueue {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxIncomingQueue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxIncomingQueue").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxIncomingQueue {
    type Vtable = IFaxIncomingQueue_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x902e64ef_8fd8_4b75_9725_6014df161545);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxIncomingQueue_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    pub Blocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbblocked: *mut i16) -> ::windows::core::HRESULT,
    pub SetBlocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bblocked: i16) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetJobs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxincomingjobs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetJobs: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxincomingjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetJob: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxJobStatus(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxJobStatus {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Status(&self) -> ::windows::core::Result<FAX_JOB_STATUS_ENUM> {
        let mut result__: FAX_JOB_STATUS_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Status)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_JOB_STATUS_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Pages(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Pages)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Size(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Size)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn CurrentPage(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CurrentPage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn DeviceId(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DeviceId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CSID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CSID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TSID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TSID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn ExtendedStatusCode(&self) -> ::windows::core::Result<FAX_JOB_EXTENDED_STATUS_ENUM> {
        let mut result__: FAX_JOB_EXTENDED_STATUS_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ExtendedStatusCode)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_JOB_EXTENDED_STATUS_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExtendedStatus(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ExtendedStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn AvailableOperations(&self) -> ::windows::core::Result<FAX_JOB_OPERATIONS_ENUM> {
        let mut result__: FAX_JOB_OPERATIONS_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AvailableOperations)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_JOB_OPERATIONS_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Retries(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Retries)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn JobType(&self) -> ::windows::core::Result<FAX_JOB_TYPE_ENUM> {
        let mut result__: FAX_JOB_TYPE_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).JobType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_JOB_TYPE_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn ScheduledTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ScheduledTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn TransmissionStart(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TransmissionStart)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn TransmissionEnd(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TransmissionEnd)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CallerId(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CallerId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RoutingInformation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RoutingInformation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxJobStatus> for ::windows::core::IUnknown {
    fn from(value: IFaxJobStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxJobStatus> for ::windows::core::IUnknown {
    fn from(value: &IFaxJobStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxJobStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxJobStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxJobStatus> for super::super::System::Com::IDispatch {
    fn from(value: IFaxJobStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxJobStatus> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxJobStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxJobStatus {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxJobStatus {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxJobStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxJobStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxJobStatus {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxJobStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxJobStatus").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxJobStatus {
    type Vtable = IFaxJobStatus_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8b86f485_fd7f_4824_886b_40c5caa617cc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxJobStatus_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_JOB_STATUS_ENUM) -> ::windows::core::HRESULT,
    pub Pages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plpages: *mut i32) -> ::windows::core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows::core::HRESULT,
    pub CurrentPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcurrentpage: *mut i32) -> ::windows::core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pldeviceid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TSID: usize,
    pub ExtendedStatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pextendedstatuscode: *mut FAX_JOB_EXTENDED_STATUS_ENUM) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ExtendedStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrextendedstatus: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExtendedStatus: usize,
    pub AvailableOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pavailableoperations: *mut FAX_JOB_OPERATIONS_ENUM) -> ::windows::core::HRESULT,
    pub Retries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows::core::HRESULT,
    pub JobType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pjobtype: *mut FAX_JOB_TYPE_ENUM) -> ::windows::core::HRESULT,
    pub ScheduledTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatescheduledtime: *mut f64) -> ::windows::core::HRESULT,
    pub TransmissionStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatetransmissionstart: *mut f64) -> ::windows::core::HRESULT,
    pub TransmissionEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatetransmissionend: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CallerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcallerid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CallerId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RoutingInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrroutinginformation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RoutingInformation: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxLoggingOptions(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxLoggingOptions {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EventLogging(&self) -> ::windows::core::Result<IFaxEventLogging> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EventLogging)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxEventLogging>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActivityLogging(&self) -> ::windows::core::Result<IFaxActivityLogging> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ActivityLogging)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxActivityLogging>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxLoggingOptions> for ::windows::core::IUnknown {
    fn from(value: IFaxLoggingOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxLoggingOptions> for ::windows::core::IUnknown {
    fn from(value: &IFaxLoggingOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxLoggingOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxLoggingOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxLoggingOptions> for super::super::System::Com::IDispatch {
    fn from(value: IFaxLoggingOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxLoggingOptions> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxLoggingOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxLoggingOptions {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxLoggingOptions {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxLoggingOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxLoggingOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxLoggingOptions {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxLoggingOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxLoggingOptions").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxLoggingOptions {
    type Vtable = IFaxLoggingOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34e64fb9_6b31_4d32_8b27_d286c0c33606);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxLoggingOptions_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub EventLogging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxeventlogging: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EventLogging: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ActivityLogging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxactivitylogging: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ActivityLogging: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxOutboundRouting(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutboundRouting {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetGroups(&self) -> ::windows::core::Result<IFaxOutboundRoutingGroups> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetGroups)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxOutboundRoutingGroups>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRules(&self) -> ::windows::core::Result<IFaxOutboundRoutingRules> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRules)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxOutboundRoutingRules>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxOutboundRouting> for ::windows::core::IUnknown {
    fn from(value: IFaxOutboundRouting) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxOutboundRouting> for ::windows::core::IUnknown {
    fn from(value: &IFaxOutboundRouting) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxOutboundRouting {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxOutboundRouting {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxOutboundRouting> for super::super::System::Com::IDispatch {
    fn from(value: IFaxOutboundRouting) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxOutboundRouting> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxOutboundRouting) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxOutboundRouting {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxOutboundRouting {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxOutboundRouting {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxOutboundRouting {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxOutboundRouting {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxOutboundRouting {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxOutboundRouting").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxOutboundRouting {
    type Vtable = IFaxOutboundRouting_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25dc05a4_9909_41bd_a95b_7e5d1dec1d43);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutboundRouting_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxoutboundroutinggroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetGroups: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRules: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxoutboundroutingrules: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRules: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxOutboundRoutingGroup(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutboundRoutingGroup {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Name)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Status(&self) -> ::windows::core::Result<FAX_GROUP_STATUS_ENUM> {
        let mut result__: FAX_GROUP_STATUS_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Status)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_GROUP_STATUS_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeviceIds(&self) -> ::windows::core::Result<IFaxDeviceIds> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DeviceIds)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxDeviceIds>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxOutboundRoutingGroup> for ::windows::core::IUnknown {
    fn from(value: IFaxOutboundRoutingGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxOutboundRoutingGroup> for ::windows::core::IUnknown {
    fn from(value: &IFaxOutboundRoutingGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxOutboundRoutingGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxOutboundRoutingGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxOutboundRoutingGroup> for super::super::System::Com::IDispatch {
    fn from(value: IFaxOutboundRoutingGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxOutboundRoutingGroup> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxOutboundRoutingGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxOutboundRoutingGroup {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxOutboundRoutingGroup {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxOutboundRoutingGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxOutboundRoutingGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxOutboundRoutingGroup {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxOutboundRoutingGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxOutboundRoutingGroup").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxOutboundRoutingGroup {
    type Vtable = IFaxOutboundRoutingGroup_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca6289a1_7e25_4f87_9a0b_93365734962c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutboundRoutingGroup_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_GROUP_STATUS_ENUM) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DeviceIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxdeviceids: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeviceIds: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxOutboundRoutingGroups(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutboundRoutingGroups {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, vindex: Param0) -> ::windows::core::Result<IFaxOutboundRoutingGroup> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::core::mem::transmute_copy(self), vindex.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFaxOutboundRoutingGroup>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0) -> ::windows::core::Result<IFaxOutboundRoutingGroup> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Add)(::core::mem::transmute_copy(self), bstrname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFaxOutboundRoutingGroup>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, vindex: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Remove)(::core::mem::transmute_copy(self), vindex.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxOutboundRoutingGroups> for ::windows::core::IUnknown {
    fn from(value: IFaxOutboundRoutingGroups) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxOutboundRoutingGroups> for ::windows::core::IUnknown {
    fn from(value: &IFaxOutboundRoutingGroups) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxOutboundRoutingGroups {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxOutboundRoutingGroups {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxOutboundRoutingGroups> for super::super::System::Com::IDispatch {
    fn from(value: IFaxOutboundRoutingGroups) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxOutboundRoutingGroups> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxOutboundRoutingGroups) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxOutboundRoutingGroups {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxOutboundRoutingGroups {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxOutboundRoutingGroups {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxOutboundRoutingGroups {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxOutboundRoutingGroups {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxOutboundRoutingGroups {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxOutboundRoutingGroups").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxOutboundRoutingGroups {
    type Vtable = IFaxOutboundRoutingGroups_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x235cbef7_c2de_4bfd_b8da_75097c82c87f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutboundRoutingGroups_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxoutboundroutinggroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxoutboundroutinggroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Add: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Remove: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxOutboundRoutingRule(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutboundRoutingRule {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn CountryCode(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CountryCode)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn AreaCode(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AreaCode)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Status(&self) -> ::windows::core::Result<FAX_RULE_STATUS_ENUM> {
        let mut result__: FAX_RULE_STATUS_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Status)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_RULE_STATUS_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn UseDevice(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).UseDevice)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetUseDevice(&self, busedevice: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUseDevice)(::core::mem::transmute_copy(self), ::core::mem::transmute(busedevice)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn DeviceId(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DeviceId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetDeviceId(&self, deviceid: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDeviceId)(::core::mem::transmute_copy(self), ::core::mem::transmute(deviceid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GroupName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GroupName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGroupName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrgroupname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGroupName)(::core::mem::transmute_copy(self), bstrgroupname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Refresh)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Save(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Save)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxOutboundRoutingRule> for ::windows::core::IUnknown {
    fn from(value: IFaxOutboundRoutingRule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxOutboundRoutingRule> for ::windows::core::IUnknown {
    fn from(value: &IFaxOutboundRoutingRule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxOutboundRoutingRule {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxOutboundRoutingRule {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxOutboundRoutingRule> for super::super::System::Com::IDispatch {
    fn from(value: IFaxOutboundRoutingRule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxOutboundRoutingRule> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxOutboundRoutingRule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxOutboundRoutingRule {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxOutboundRoutingRule {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxOutboundRoutingRule {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxOutboundRoutingRule {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxOutboundRoutingRule {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxOutboundRoutingRule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxOutboundRoutingRule").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxOutboundRoutingRule {
    type Vtable = IFaxOutboundRoutingRule_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1f795d5_07c2_469f_b027_acacc23219da);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutboundRoutingRule_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    pub CountryCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcountrycode: *mut i32) -> ::windows::core::HRESULT,
    pub AreaCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plareacode: *mut i32) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_RULE_STATUS_ENUM) -> ::windows::core::HRESULT,
    pub UseDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbusedevice: *mut i16) -> ::windows::core::HRESULT,
    pub SetUseDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, busedevice: i16) -> ::windows::core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pldeviceid: *mut i32) -> ::windows::core::HRESULT,
    pub SetDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GroupName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrgroupname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GroupName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetGroupName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetGroupName: usize,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxOutboundRoutingRules(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutboundRoutingRules {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, lindex: i32) -> ::windows::core::Result<IFaxOutboundRoutingRule> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::core::mem::transmute_copy(self), ::core::mem::transmute(lindex), ::core::mem::transmute(&mut result__)).from_abi::<IFaxOutboundRoutingRule>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ItemByCountryAndArea(&self, lcountrycode: i32, lareacode: i32) -> ::windows::core::Result<IFaxOutboundRoutingRule> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ItemByCountryAndArea)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcountrycode), ::core::mem::transmute(lareacode), ::core::mem::transmute(&mut result__)).from_abi::<IFaxOutboundRoutingRule>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn RemoveByCountryAndArea(&self, lcountrycode: i32, lareacode: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveByCountryAndArea)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcountrycode), ::core::mem::transmute(lareacode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Remove(&self, lindex: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Remove)(::core::mem::transmute_copy(self), ::core::mem::transmute(lindex)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Add<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, lcountrycode: i32, lareacode: i32, busedevice: i16, bstrgroupname: Param3, ldeviceid: i32) -> ::windows::core::Result<IFaxOutboundRoutingRule> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Add)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcountrycode), ::core::mem::transmute(lareacode), ::core::mem::transmute(busedevice), bstrgroupname.into_param().abi(), ::core::mem::transmute(ldeviceid), ::core::mem::transmute(&mut result__)).from_abi::<IFaxOutboundRoutingRule>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxOutboundRoutingRules> for ::windows::core::IUnknown {
    fn from(value: IFaxOutboundRoutingRules) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxOutboundRoutingRules> for ::windows::core::IUnknown {
    fn from(value: &IFaxOutboundRoutingRules) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxOutboundRoutingRules {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxOutboundRoutingRules {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxOutboundRoutingRules> for super::super::System::Com::IDispatch {
    fn from(value: IFaxOutboundRoutingRules) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxOutboundRoutingRules> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxOutboundRoutingRules) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxOutboundRoutingRules {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxOutboundRoutingRules {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxOutboundRoutingRules {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxOutboundRoutingRules {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxOutboundRoutingRules {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxOutboundRoutingRules {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxOutboundRoutingRules").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxOutboundRoutingRules {
    type Vtable = IFaxOutboundRoutingRules_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcefa1e7_ae7d_4ed6_8521_369edcca5120);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutboundRoutingRules_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pfaxoutboundroutingrule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ItemByCountryAndArea: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcountrycode: i32, lareacode: i32, pfaxoutboundroutingrule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ItemByCountryAndArea: usize,
    pub RemoveByCountryAndArea: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcountrycode: i32, lareacode: i32) -> ::windows::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcountrycode: i32, lareacode: i32, busedevice: i16, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ldeviceid: i32, pfaxoutboundroutingrule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Add: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxOutgoingArchive(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingArchive {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn UseArchive(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).UseArchive)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetUseArchive(&self, busearchive: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUseArchive)(::core::mem::transmute_copy(self), ::core::mem::transmute(busearchive)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ArchiveFolder(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ArchiveFolder)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetArchiveFolder<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrarchivefolder: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetArchiveFolder)(::core::mem::transmute_copy(self), bstrarchivefolder.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SizeQuotaWarning(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SizeQuotaWarning)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetSizeQuotaWarning(&self, bsizequotawarning: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSizeQuotaWarning)(::core::mem::transmute_copy(self), ::core::mem::transmute(bsizequotawarning)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn HighQuotaWaterMark(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).HighQuotaWaterMark)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetHighQuotaWaterMark(&self, lhighquotawatermark: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetHighQuotaWaterMark)(::core::mem::transmute_copy(self), ::core::mem::transmute(lhighquotawatermark)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn LowQuotaWaterMark(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LowQuotaWaterMark)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetLowQuotaWaterMark(&self, llowquotawatermark: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLowQuotaWaterMark)(::core::mem::transmute_copy(self), ::core::mem::transmute(llowquotawatermark)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn AgeLimit(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AgeLimit)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetAgeLimit(&self, lagelimit: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAgeLimit)(::core::mem::transmute_copy(self), ::core::mem::transmute(lagelimit)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SizeLow(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SizeLow)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SizeHigh(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SizeHigh)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Refresh)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Save(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Save)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetMessages(&self, lprefetchsize: i32) -> ::windows::core::Result<IFaxOutgoingMessageIterator> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMessages)(::core::mem::transmute_copy(self), ::core::mem::transmute(lprefetchsize), ::core::mem::transmute(&mut result__)).from_abi::<IFaxOutgoingMessageIterator>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetMessage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrmessageid: Param0) -> ::windows::core::Result<IFaxOutgoingMessage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMessage)(::core::mem::transmute_copy(self), bstrmessageid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFaxOutgoingMessage>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxOutgoingArchive> for ::windows::core::IUnknown {
    fn from(value: IFaxOutgoingArchive) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxOutgoingArchive> for ::windows::core::IUnknown {
    fn from(value: &IFaxOutgoingArchive) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxOutgoingArchive {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxOutgoingArchive {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxOutgoingArchive> for super::super::System::Com::IDispatch {
    fn from(value: IFaxOutgoingArchive) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxOutgoingArchive> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxOutgoingArchive) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxOutgoingArchive {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxOutgoingArchive {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxOutgoingArchive {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxOutgoingArchive {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxOutgoingArchive {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxOutgoingArchive {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxOutgoingArchive").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxOutgoingArchive {
    type Vtable = IFaxOutgoingArchive_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9c28f40_8d80_4e53_810f_9a79919b49fd);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutgoingArchive_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    pub UseArchive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbusearchive: *mut i16) -> ::windows::core::HRESULT,
    pub SetUseArchive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, busearchive: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ArchiveFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrarchivefolder: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ArchiveFolder: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetArchiveFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrarchivefolder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetArchiveFolder: usize,
    pub SizeQuotaWarning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsizequotawarning: *mut i16) -> ::windows::core::HRESULT,
    pub SetSizeQuotaWarning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bsizequotawarning: i16) -> ::windows::core::HRESULT,
    pub HighQuotaWaterMark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plhighquotawatermark: *mut i32) -> ::windows::core::HRESULT,
    pub SetHighQuotaWaterMark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lhighquotawatermark: i32) -> ::windows::core::HRESULT,
    pub LowQuotaWaterMark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pllowquotawatermark: *mut i32) -> ::windows::core::HRESULT,
    pub SetLowQuotaWaterMark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, llowquotawatermark: i32) -> ::windows::core::HRESULT,
    pub AgeLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plagelimit: *mut i32) -> ::windows::core::HRESULT,
    pub SetAgeLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lagelimit: i32) -> ::windows::core::HRESULT,
    pub SizeLow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsizelow: *mut i32) -> ::windows::core::HRESULT,
    pub SizeHigh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsizehigh: *mut i32) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprefetchsize: i32, pfaxoutgoingmessageiterator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetMessages: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxoutgoingmessage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetMessage: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxOutgoingJob(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingJob {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Subject(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Subject)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DocumentName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DocumentName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Pages(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Pages)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Size(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Size)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SubmissionId(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SubmissionId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Id(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Id)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn OriginalScheduledTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).OriginalScheduledTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SubmissionTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SubmissionTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn ReceiptType(&self) -> ::windows::core::Result<FAX_RECEIPT_TYPE_ENUM> {
        let mut result__: FAX_RECEIPT_TYPE_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ReceiptType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_RECEIPT_TYPE_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Priority(&self) -> ::windows::core::Result<FAX_PRIORITY_TYPE_ENUM> {
        let mut result__: FAX_PRIORITY_TYPE_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Priority)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_PRIORITY_TYPE_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Sender(&self) -> ::windows::core::Result<IFaxSender> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Sender)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxSender>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Recipient(&self) -> ::windows::core::Result<IFaxRecipient> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Recipient)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxRecipient>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn CurrentPage(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CurrentPage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn DeviceId(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DeviceId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Status(&self) -> ::windows::core::Result<FAX_JOB_STATUS_ENUM> {
        let mut result__: FAX_JOB_STATUS_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Status)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_JOB_STATUS_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn ExtendedStatusCode(&self) -> ::windows::core::Result<FAX_JOB_EXTENDED_STATUS_ENUM> {
        let mut result__: FAX_JOB_EXTENDED_STATUS_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ExtendedStatusCode)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_JOB_EXTENDED_STATUS_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExtendedStatus(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ExtendedStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn AvailableOperations(&self) -> ::windows::core::Result<FAX_JOB_OPERATIONS_ENUM> {
        let mut result__: FAX_JOB_OPERATIONS_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AvailableOperations)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_JOB_OPERATIONS_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Retries(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Retries)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn ScheduledTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ScheduledTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn TransmissionStart(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TransmissionStart)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn TransmissionEnd(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TransmissionEnd)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CSID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CSID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TSID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TSID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn GroupBroadcastReceipts(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GroupBroadcastReceipts)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Pause)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Resume)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Restart(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Restart)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTiff<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrtiffpath: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CopyTiff)(::core::mem::transmute_copy(self), bstrtiffpath.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Refresh)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Cancel)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxOutgoingJob> for ::windows::core::IUnknown {
    fn from(value: IFaxOutgoingJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxOutgoingJob> for ::windows::core::IUnknown {
    fn from(value: &IFaxOutgoingJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxOutgoingJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxOutgoingJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxOutgoingJob> for super::super::System::Com::IDispatch {
    fn from(value: IFaxOutgoingJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxOutgoingJob> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxOutgoingJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxOutgoingJob {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxOutgoingJob {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxOutgoingJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxOutgoingJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxOutgoingJob {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxOutgoingJob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxOutgoingJob").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxOutgoingJob {
    type Vtable = IFaxOutgoingJob_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6356daad_6614_4583_bf7a_3ad67bbfc71c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutgoingJob_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsubject: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Subject: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DocumentName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdocumentname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DocumentName: usize,
    pub Pages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plpages: *mut i32) -> ::windows::core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SubmissionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsubmissionid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SubmissionId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Id: usize,
    pub OriginalScheduledTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdateoriginalscheduledtime: *mut f64) -> ::windows::core::HRESULT,
    pub SubmissionTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatesubmissiontime: *mut f64) -> ::windows::core::HRESULT,
    pub ReceiptType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preceipttype: *mut FAX_RECEIPT_TYPE_ENUM) -> ::windows::core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppriority: *mut FAX_PRIORITY_TYPE_ENUM) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Sender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfaxsender: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Sender: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Recipient: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfaxrecipient: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Recipient: usize,
    pub CurrentPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcurrentpage: *mut i32) -> ::windows::core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pldeviceid: *mut i32) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_JOB_STATUS_ENUM) -> ::windows::core::HRESULT,
    pub ExtendedStatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pextendedstatuscode: *mut FAX_JOB_EXTENDED_STATUS_ENUM) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ExtendedStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrextendedstatus: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExtendedStatus: usize,
    pub AvailableOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pavailableoperations: *mut FAX_JOB_OPERATIONS_ENUM) -> ::windows::core::HRESULT,
    pub Retries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows::core::HRESULT,
    pub ScheduledTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatescheduledtime: *mut f64) -> ::windows::core::HRESULT,
    pub TransmissionStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatetransmissionstart: *mut f64) -> ::windows::core::HRESULT,
    pub TransmissionEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatetransmissionend: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TSID: usize,
    pub GroupBroadcastReceipts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbgroupbroadcastreceipts: *mut i16) -> ::windows::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Restart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CopyTiff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtiffpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CopyTiff: usize,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxOutgoingJob2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingJob2 {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Subject(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Subject)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DocumentName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.DocumentName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Pages(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Pages)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Size(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Size)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SubmissionId(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SubmissionId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Id(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Id)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn OriginalScheduledTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.OriginalScheduledTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SubmissionTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SubmissionTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn ReceiptType(&self) -> ::windows::core::Result<FAX_RECEIPT_TYPE_ENUM> {
        let mut result__: FAX_RECEIPT_TYPE_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ReceiptType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_RECEIPT_TYPE_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Priority(&self) -> ::windows::core::Result<FAX_PRIORITY_TYPE_ENUM> {
        let mut result__: FAX_PRIORITY_TYPE_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Priority)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_PRIORITY_TYPE_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Sender(&self) -> ::windows::core::Result<IFaxSender> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Sender)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxSender>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Recipient(&self) -> ::windows::core::Result<IFaxRecipient> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Recipient)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxRecipient>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn CurrentPage(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CurrentPage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn DeviceId(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.DeviceId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Status(&self) -> ::windows::core::Result<FAX_JOB_STATUS_ENUM> {
        let mut result__: FAX_JOB_STATUS_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Status)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_JOB_STATUS_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn ExtendedStatusCode(&self) -> ::windows::core::Result<FAX_JOB_EXTENDED_STATUS_ENUM> {
        let mut result__: FAX_JOB_EXTENDED_STATUS_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ExtendedStatusCode)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_JOB_EXTENDED_STATUS_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExtendedStatus(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ExtendedStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn AvailableOperations(&self) -> ::windows::core::Result<FAX_JOB_OPERATIONS_ENUM> {
        let mut result__: FAX_JOB_OPERATIONS_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.AvailableOperations)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_JOB_OPERATIONS_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Retries(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Retries)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn ScheduledTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ScheduledTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn TransmissionStart(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.TransmissionStart)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn TransmissionEnd(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.TransmissionEnd)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CSID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CSID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TSID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.TSID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn GroupBroadcastReceipts(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GroupBroadcastReceipts)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Pause)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Resume)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Restart(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Restart)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTiff<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrtiffpath: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.CopyTiff)(::core::mem::transmute_copy(self), bstrtiffpath.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Refresh)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Cancel)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn HasCoverPage(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).HasCoverPage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReceiptAddress(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ReceiptAddress)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn ScheduleType(&self) -> ::windows::core::Result<FAX_SCHEDULE_TYPE_ENUM> {
        let mut result__: FAX_SCHEDULE_TYPE_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ScheduleType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_SCHEDULE_TYPE_ENUM>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxOutgoingJob2> for ::windows::core::IUnknown {
    fn from(value: IFaxOutgoingJob2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxOutgoingJob2> for ::windows::core::IUnknown {
    fn from(value: &IFaxOutgoingJob2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxOutgoingJob2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxOutgoingJob2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxOutgoingJob2> for super::super::System::Com::IDispatch {
    fn from(value: IFaxOutgoingJob2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxOutgoingJob2> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxOutgoingJob2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxOutgoingJob2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxOutgoingJob2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxOutgoingJob2> for IFaxOutgoingJob {
    fn from(value: IFaxOutgoingJob2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxOutgoingJob2> for IFaxOutgoingJob {
    fn from(value: &IFaxOutgoingJob2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFaxOutgoingJob> for IFaxOutgoingJob2 {
    fn into_param(self) -> ::windows::core::Param<'a, IFaxOutgoingJob> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFaxOutgoingJob> for &'a IFaxOutgoingJob2 {
    fn into_param(self) -> ::windows::core::Param<'a, IFaxOutgoingJob> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxOutgoingJob2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxOutgoingJob2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxOutgoingJob2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxOutgoingJob2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxOutgoingJob2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxOutgoingJob2 {
    type Vtable = IFaxOutgoingJob2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x418a8d96_59a0_4789_b176_edf3dc8fa8f7);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutgoingJob2_Vtbl {
    pub base: IFaxOutgoingJob_Vtbl,
    pub HasCoverPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbhascoverpage: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ReceiptAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrreceiptaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReceiptAddress: usize,
    pub ScheduleType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pscheduletype: *mut FAX_SCHEDULE_TYPE_ENUM) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxOutgoingJobs(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingJobs {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, vindex: Param0) -> ::windows::core::Result<IFaxOutgoingJob> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::core::mem::transmute_copy(self), vindex.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFaxOutgoingJob>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxOutgoingJobs> for ::windows::core::IUnknown {
    fn from(value: IFaxOutgoingJobs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxOutgoingJobs> for ::windows::core::IUnknown {
    fn from(value: &IFaxOutgoingJobs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxOutgoingJobs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxOutgoingJobs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxOutgoingJobs> for super::super::System::Com::IDispatch {
    fn from(value: IFaxOutgoingJobs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxOutgoingJobs> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxOutgoingJobs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxOutgoingJobs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxOutgoingJobs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxOutgoingJobs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxOutgoingJobs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxOutgoingJobs {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxOutgoingJobs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxOutgoingJobs").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxOutgoingJobs {
    type Vtable = IFaxOutgoingJobs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c56d8e6_8c2f_4573_944c_e505f8f5aeed);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutgoingJobs_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxoutgoingjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxOutgoingMessage(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingMessage {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SubmissionId(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SubmissionId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Id(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Id)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Subject(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Subject)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DocumentName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DocumentName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Retries(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Retries)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Pages(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Pages)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Size(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Size)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn OriginalScheduledTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).OriginalScheduledTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SubmissionTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SubmissionTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Priority(&self) -> ::windows::core::Result<FAX_PRIORITY_TYPE_ENUM> {
        let mut result__: FAX_PRIORITY_TYPE_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Priority)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_PRIORITY_TYPE_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Sender(&self) -> ::windows::core::Result<IFaxSender> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Sender)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxSender>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Recipient(&self) -> ::windows::core::Result<IFaxRecipient> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Recipient)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxRecipient>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeviceName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DeviceName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn TransmissionStart(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TransmissionStart)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn TransmissionEnd(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TransmissionEnd)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CSID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CSID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TSID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TSID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTiff<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrtiffpath: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CopyTiff)(::core::mem::transmute_copy(self), bstrtiffpath.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Delete)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxOutgoingMessage> for ::windows::core::IUnknown {
    fn from(value: IFaxOutgoingMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxOutgoingMessage> for ::windows::core::IUnknown {
    fn from(value: &IFaxOutgoingMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxOutgoingMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxOutgoingMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxOutgoingMessage> for super::super::System::Com::IDispatch {
    fn from(value: IFaxOutgoingMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxOutgoingMessage> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxOutgoingMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxOutgoingMessage {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxOutgoingMessage {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxOutgoingMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxOutgoingMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxOutgoingMessage {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxOutgoingMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxOutgoingMessage").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxOutgoingMessage {
    type Vtable = IFaxOutgoingMessage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0ea35de_caa5_4a7c_82c7_2b60ba5f2be2);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutgoingMessage_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SubmissionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsubmissionid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SubmissionId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Id: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsubject: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Subject: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DocumentName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdocumentname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DocumentName: usize,
    pub Retries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows::core::HRESULT,
    pub Pages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plpages: *mut i32) -> ::windows::core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows::core::HRESULT,
    pub OriginalScheduledTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdateoriginalscheduledtime: *mut f64) -> ::windows::core::HRESULT,
    pub SubmissionTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatesubmissiontime: *mut f64) -> ::windows::core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppriority: *mut FAX_PRIORITY_TYPE_ENUM) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Sender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfaxsender: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Sender: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Recipient: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfaxrecipient: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Recipient: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeviceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeviceName: usize,
    pub TransmissionStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatetransmissionstart: *mut f64) -> ::windows::core::HRESULT,
    pub TransmissionEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatetransmissionend: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CopyTiff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtiffpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CopyTiff: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxOutgoingMessage2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingMessage2 {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SubmissionId(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SubmissionId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Id(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Id)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Subject(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Subject)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DocumentName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.DocumentName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Retries(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Retries)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Pages(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Pages)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Size(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Size)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn OriginalScheduledTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.OriginalScheduledTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SubmissionTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SubmissionTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Priority(&self) -> ::windows::core::Result<FAX_PRIORITY_TYPE_ENUM> {
        let mut result__: FAX_PRIORITY_TYPE_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Priority)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_PRIORITY_TYPE_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Sender(&self) -> ::windows::core::Result<IFaxSender> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Sender)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxSender>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Recipient(&self) -> ::windows::core::Result<IFaxRecipient> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Recipient)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxRecipient>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeviceName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.DeviceName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn TransmissionStart(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.TransmissionStart)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn TransmissionEnd(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.TransmissionEnd)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CSID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CSID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TSID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.TSID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTiff<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrtiffpath: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.CopyTiff)(::core::mem::transmute_copy(self), bstrtiffpath.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn HasCoverPage(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).HasCoverPage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn ReceiptType(&self) -> ::windows::core::Result<FAX_RECEIPT_TYPE_ENUM> {
        let mut result__: FAX_RECEIPT_TYPE_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ReceiptType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_RECEIPT_TYPE_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReceiptAddress(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ReceiptAddress)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Read(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Read)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetRead(&self, bread: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRead)(::core::mem::transmute_copy(self), ::core::mem::transmute(bread)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Save(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Save)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Refresh)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxOutgoingMessage2> for ::windows::core::IUnknown {
    fn from(value: IFaxOutgoingMessage2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxOutgoingMessage2> for ::windows::core::IUnknown {
    fn from(value: &IFaxOutgoingMessage2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxOutgoingMessage2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxOutgoingMessage2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxOutgoingMessage2> for super::super::System::Com::IDispatch {
    fn from(value: IFaxOutgoingMessage2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxOutgoingMessage2> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxOutgoingMessage2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxOutgoingMessage2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxOutgoingMessage2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxOutgoingMessage2> for IFaxOutgoingMessage {
    fn from(value: IFaxOutgoingMessage2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxOutgoingMessage2> for IFaxOutgoingMessage {
    fn from(value: &IFaxOutgoingMessage2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFaxOutgoingMessage> for IFaxOutgoingMessage2 {
    fn into_param(self) -> ::windows::core::Param<'a, IFaxOutgoingMessage> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFaxOutgoingMessage> for &'a IFaxOutgoingMessage2 {
    fn into_param(self) -> ::windows::core::Param<'a, IFaxOutgoingMessage> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxOutgoingMessage2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxOutgoingMessage2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxOutgoingMessage2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxOutgoingMessage2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxOutgoingMessage2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxOutgoingMessage2 {
    type Vtable = IFaxOutgoingMessage2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb37df687_bc88_4b46_b3be_b458b3ea9e7f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutgoingMessage2_Vtbl {
    pub base: IFaxOutgoingMessage_Vtbl,
    pub HasCoverPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbhascoverpage: *mut i16) -> ::windows::core::HRESULT,
    pub ReceiptType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preceipttype: *mut FAX_RECEIPT_TYPE_ENUM) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ReceiptAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrreceiptaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReceiptAddress: usize,
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbread: *mut i16) -> ::windows::core::HRESULT,
    pub SetRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bread: i16) -> ::windows::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxOutgoingMessageIterator(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingMessageIterator {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Message(&self) -> ::windows::core::Result<IFaxOutgoingMessage> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Message)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxOutgoingMessage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn AtEOF(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AtEOF)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn PrefetchSize(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PrefetchSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetPrefetchSize(&self, lprefetchsize: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPrefetchSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(lprefetchsize)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn MoveFirst(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MoveFirst)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MoveNext)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxOutgoingMessageIterator> for ::windows::core::IUnknown {
    fn from(value: IFaxOutgoingMessageIterator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxOutgoingMessageIterator> for ::windows::core::IUnknown {
    fn from(value: &IFaxOutgoingMessageIterator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxOutgoingMessageIterator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxOutgoingMessageIterator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxOutgoingMessageIterator> for super::super::System::Com::IDispatch {
    fn from(value: IFaxOutgoingMessageIterator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxOutgoingMessageIterator> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxOutgoingMessageIterator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxOutgoingMessageIterator {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxOutgoingMessageIterator {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxOutgoingMessageIterator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxOutgoingMessageIterator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxOutgoingMessageIterator {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxOutgoingMessageIterator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxOutgoingMessageIterator").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxOutgoingMessageIterator {
    type Vtable = IFaxOutgoingMessageIterator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5ec5d4f_b840_432f_9980_112fe42a9b7a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutgoingMessageIterator_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxoutgoingmessage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Message: usize,
    pub AtEOF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbeof: *mut i16) -> ::windows::core::HRESULT,
    pub PrefetchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprefetchsize: *mut i32) -> ::windows::core::HRESULT,
    pub SetPrefetchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprefetchsize: i32) -> ::windows::core::HRESULT,
    pub MoveFirst: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxOutgoingQueue(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingQueue {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Blocked(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Blocked)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetBlocked(&self, bblocked: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBlocked)(::core::mem::transmute_copy(self), ::core::mem::transmute(bblocked)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Paused(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Paused)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetPaused(&self, bpaused: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPaused)(::core::mem::transmute_copy(self), ::core::mem::transmute(bpaused)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn AllowPersonalCoverPages(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AllowPersonalCoverPages)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetAllowPersonalCoverPages(&self, ballowpersonalcoverpages: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllowPersonalCoverPages)(::core::mem::transmute_copy(self), ::core::mem::transmute(ballowpersonalcoverpages)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn UseDeviceTSID(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).UseDeviceTSID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetUseDeviceTSID(&self, busedevicetsid: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUseDeviceTSID)(::core::mem::transmute_copy(self), ::core::mem::transmute(busedevicetsid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Retries(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Retries)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetRetries(&self, lretries: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRetries)(::core::mem::transmute_copy(self), ::core::mem::transmute(lretries)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn RetryDelay(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RetryDelay)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetRetryDelay(&self, lretrydelay: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRetryDelay)(::core::mem::transmute_copy(self), ::core::mem::transmute(lretrydelay)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn DiscountRateStart(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DiscountRateStart)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetDiscountRateStart(&self, datediscountratestart: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDiscountRateStart)(::core::mem::transmute_copy(self), ::core::mem::transmute(datediscountratestart)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn DiscountRateEnd(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DiscountRateEnd)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetDiscountRateEnd(&self, datediscountrateend: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDiscountRateEnd)(::core::mem::transmute_copy(self), ::core::mem::transmute(datediscountrateend)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn AgeLimit(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AgeLimit)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetAgeLimit(&self, lagelimit: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAgeLimit)(::core::mem::transmute_copy(self), ::core::mem::transmute(lagelimit)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Branding(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Branding)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetBranding(&self, bbranding: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBranding)(::core::mem::transmute_copy(self), ::core::mem::transmute(bbranding)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Refresh)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Save(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Save)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetJobs(&self) -> ::windows::core::Result<IFaxOutgoingJobs> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetJobs)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxOutgoingJobs>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetJob<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrjobid: Param0) -> ::windows::core::Result<IFaxOutgoingJob> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetJob)(::core::mem::transmute_copy(self), bstrjobid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFaxOutgoingJob>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxOutgoingQueue> for ::windows::core::IUnknown {
    fn from(value: IFaxOutgoingQueue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxOutgoingQueue> for ::windows::core::IUnknown {
    fn from(value: &IFaxOutgoingQueue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxOutgoingQueue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxOutgoingQueue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxOutgoingQueue> for super::super::System::Com::IDispatch {
    fn from(value: IFaxOutgoingQueue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxOutgoingQueue> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxOutgoingQueue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxOutgoingQueue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxOutgoingQueue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxOutgoingQueue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxOutgoingQueue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxOutgoingQueue {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxOutgoingQueue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxOutgoingQueue").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxOutgoingQueue {
    type Vtable = IFaxOutgoingQueue_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80b1df24_d9ac_4333_b373_487cedc80ce5);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutgoingQueue_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    pub Blocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbblocked: *mut i16) -> ::windows::core::HRESULT,
    pub SetBlocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bblocked: i16) -> ::windows::core::HRESULT,
    pub Paused: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbpaused: *mut i16) -> ::windows::core::HRESULT,
    pub SetPaused: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bpaused: i16) -> ::windows::core::HRESULT,
    pub AllowPersonalCoverPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pballowpersonalcoverpages: *mut i16) -> ::windows::core::HRESULT,
    pub SetAllowPersonalCoverPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ballowpersonalcoverpages: i16) -> ::windows::core::HRESULT,
    pub UseDeviceTSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbusedevicetsid: *mut i16) -> ::windows::core::HRESULT,
    pub SetUseDeviceTSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, busedevicetsid: i16) -> ::windows::core::HRESULT,
    pub Retries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows::core::HRESULT,
    pub SetRetries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lretries: i32) -> ::windows::core::HRESULT,
    pub RetryDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plretrydelay: *mut i32) -> ::windows::core::HRESULT,
    pub SetRetryDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lretrydelay: i32) -> ::windows::core::HRESULT,
    pub DiscountRateStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatediscountratestart: *mut f64) -> ::windows::core::HRESULT,
    pub SetDiscountRateStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datediscountratestart: f64) -> ::windows::core::HRESULT,
    pub DiscountRateEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatediscountrateend: *mut f64) -> ::windows::core::HRESULT,
    pub SetDiscountRateEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datediscountrateend: f64) -> ::windows::core::HRESULT,
    pub AgeLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plagelimit: *mut i32) -> ::windows::core::HRESULT,
    pub SetAgeLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lagelimit: i32) -> ::windows::core::HRESULT,
    pub Branding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbbranding: *mut i16) -> ::windows::core::HRESULT,
    pub SetBranding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bbranding: i16) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetJobs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxoutgoingjobs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetJobs: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxoutgoingjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetJob: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxReceiptOptions(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxReceiptOptions {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn AuthenticationType(&self) -> ::windows::core::Result<FAX_SMTP_AUTHENTICATION_TYPE_ENUM> {
        let mut result__: FAX_SMTP_AUTHENTICATION_TYPE_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AuthenticationType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_SMTP_AUTHENTICATION_TYPE_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetAuthenticationType(&self, r#type: FAX_SMTP_AUTHENTICATION_TYPE_ENUM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAuthenticationType)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SMTPServer(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SMTPServer)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSMTPServer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsmtpserver: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSMTPServer)(::core::mem::transmute_copy(self), bstrsmtpserver.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SMTPPort(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SMTPPort)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetSMTPPort(&self, lsmtpport: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSMTPPort)(::core::mem::transmute_copy(self), ::core::mem::transmute(lsmtpport)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SMTPSender(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SMTPSender)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSMTPSender<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsmtpsender: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSMTPSender)(::core::mem::transmute_copy(self), bstrsmtpsender.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SMTPUser(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SMTPUser)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSMTPUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsmtpuser: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSMTPUser)(::core::mem::transmute_copy(self), bstrsmtpuser.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn AllowedReceipts(&self) -> ::windows::core::Result<FAX_RECEIPT_TYPE_ENUM> {
        let mut result__: FAX_RECEIPT_TYPE_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AllowedReceipts)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_RECEIPT_TYPE_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetAllowedReceipts(&self, allowedreceipts: FAX_RECEIPT_TYPE_ENUM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllowedReceipts)(::core::mem::transmute_copy(self), ::core::mem::transmute(allowedreceipts)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SMTPPassword(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SMTPPassword)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSMTPPassword<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsmtppassword: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSMTPPassword)(::core::mem::transmute_copy(self), bstrsmtppassword.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Refresh)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Save(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Save)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn UseForInboundRouting(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).UseForInboundRouting)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetUseForInboundRouting(&self, buseforinboundrouting: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUseForInboundRouting)(::core::mem::transmute_copy(self), ::core::mem::transmute(buseforinboundrouting)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxReceiptOptions> for ::windows::core::IUnknown {
    fn from(value: IFaxReceiptOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxReceiptOptions> for ::windows::core::IUnknown {
    fn from(value: &IFaxReceiptOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxReceiptOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxReceiptOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxReceiptOptions> for super::super::System::Com::IDispatch {
    fn from(value: IFaxReceiptOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxReceiptOptions> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxReceiptOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxReceiptOptions {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxReceiptOptions {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxReceiptOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxReceiptOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxReceiptOptions {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxReceiptOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxReceiptOptions").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxReceiptOptions {
    type Vtable = IFaxReceiptOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x378efaeb_5fcb_4afb_b2ee_e16e80614487);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxReceiptOptions_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    pub AuthenticationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut FAX_SMTP_AUTHENTICATION_TYPE_ENUM) -> ::windows::core::HRESULT,
    pub SetAuthenticationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: FAX_SMTP_AUTHENTICATION_TYPE_ENUM) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SMTPServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsmtpserver: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SMTPServer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSMTPServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsmtpserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSMTPServer: usize,
    pub SMTPPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsmtpport: *mut i32) -> ::windows::core::HRESULT,
    pub SetSMTPPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lsmtpport: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SMTPSender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsmtpsender: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SMTPSender: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSMTPSender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsmtpsender: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSMTPSender: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SMTPUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsmtpuser: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SMTPUser: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSMTPUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsmtpuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSMTPUser: usize,
    pub AllowedReceipts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pallowedreceipts: *mut FAX_RECEIPT_TYPE_ENUM) -> ::windows::core::HRESULT,
    pub SetAllowedReceipts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allowedreceipts: FAX_RECEIPT_TYPE_ENUM) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SMTPPassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsmtppassword: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SMTPPassword: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSMTPPassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsmtppassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSMTPPassword: usize,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UseForInboundRouting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuseforinboundrouting: *mut i16) -> ::windows::core::HRESULT,
    pub SetUseForInboundRouting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buseforinboundrouting: i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxRecipient(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxRecipient {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FaxNumber(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FaxNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFaxNumber<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrfaxnumber: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFaxNumber)(::core::mem::transmute_copy(self), bstrfaxnumber.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Name)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetName)(::core::mem::transmute_copy(self), bstrname.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxRecipient> for ::windows::core::IUnknown {
    fn from(value: IFaxRecipient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxRecipient> for ::windows::core::IUnknown {
    fn from(value: &IFaxRecipient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxRecipient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxRecipient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxRecipient> for super::super::System::Com::IDispatch {
    fn from(value: IFaxRecipient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxRecipient> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxRecipient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxRecipient {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxRecipient {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxRecipient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxRecipient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxRecipient {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxRecipient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxRecipient").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxRecipient {
    type Vtable = IFaxRecipient_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a3da3a0_538d_42b6_9444_aaa57d0ce2bc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxRecipient_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub FaxNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfaxnumber: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FaxNumber: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFaxNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfaxnumber: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFaxNumber: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxRecipients(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxRecipients {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, lindex: i32) -> ::windows::core::Result<IFaxRecipient> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::core::mem::transmute_copy(self), ::core::mem::transmute(lindex), ::core::mem::transmute(&mut result__)).from_abi::<IFaxRecipient>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrfaxnumber: Param0, bstrrecipientname: Param1) -> ::windows::core::Result<IFaxRecipient> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Add)(::core::mem::transmute_copy(self), bstrfaxnumber.into_param().abi(), bstrrecipientname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFaxRecipient>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Remove(&self, lindex: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Remove)(::core::mem::transmute_copy(self), ::core::mem::transmute(lindex)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxRecipients> for ::windows::core::IUnknown {
    fn from(value: IFaxRecipients) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxRecipients> for ::windows::core::IUnknown {
    fn from(value: &IFaxRecipients) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxRecipients {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxRecipients {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxRecipients> for super::super::System::Com::IDispatch {
    fn from(value: IFaxRecipients) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxRecipients> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxRecipients) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxRecipients {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxRecipients {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxRecipients {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxRecipients {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxRecipients {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxRecipients {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxRecipients").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxRecipients {
    type Vtable = IFaxRecipients_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9c9de5a_894e_4492_9fa3_08c627c11d5d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxRecipients_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, ppfaxrecipient: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfaxnumber: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrecipientname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfaxrecipient: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxSecurity(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxSecurity {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Descriptor(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Descriptor)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetDescriptor<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, vdescriptor: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDescriptor)(::core::mem::transmute_copy(self), vdescriptor.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn GrantedRights(&self) -> ::windows::core::Result<FAX_ACCESS_RIGHTS_ENUM> {
        let mut result__: FAX_ACCESS_RIGHTS_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GrantedRights)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_ACCESS_RIGHTS_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Refresh)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Save(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Save)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn InformationType(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).InformationType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetInformationType(&self, linformationtype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInformationType)(::core::mem::transmute_copy(self), ::core::mem::transmute(linformationtype)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxSecurity> for ::windows::core::IUnknown {
    fn from(value: IFaxSecurity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxSecurity> for ::windows::core::IUnknown {
    fn from(value: &IFaxSecurity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxSecurity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxSecurity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxSecurity> for super::super::System::Com::IDispatch {
    fn from(value: IFaxSecurity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxSecurity> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxSecurity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxSecurity {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxSecurity {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxSecurity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxSecurity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxSecurity {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxSecurity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxSecurity").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxSecurity {
    type Vtable = IFaxSecurity_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77b508c1_09c0_47a2_91eb_fce7fdf2690e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxSecurity_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Descriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvdescriptor: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Descriptor: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vdescriptor: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetDescriptor: usize,
    pub GrantedRights: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgrantedrights: *mut FAX_ACCESS_RIGHTS_ENUM) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InformationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plinformationtype: *mut i32) -> ::windows::core::HRESULT,
    pub SetInformationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linformationtype: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxSecurity2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxSecurity2 {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Descriptor(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Descriptor)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetDescriptor<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, vdescriptor: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDescriptor)(::core::mem::transmute_copy(self), vdescriptor.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn GrantedRights(&self) -> ::windows::core::Result<FAX_ACCESS_RIGHTS_ENUM_2> {
        let mut result__: FAX_ACCESS_RIGHTS_ENUM_2 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GrantedRights)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_ACCESS_RIGHTS_ENUM_2>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Refresh)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Save(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Save)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn InformationType(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).InformationType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetInformationType(&self, linformationtype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInformationType)(::core::mem::transmute_copy(self), ::core::mem::transmute(linformationtype)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxSecurity2> for ::windows::core::IUnknown {
    fn from(value: IFaxSecurity2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxSecurity2> for ::windows::core::IUnknown {
    fn from(value: &IFaxSecurity2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxSecurity2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxSecurity2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxSecurity2> for super::super::System::Com::IDispatch {
    fn from(value: IFaxSecurity2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxSecurity2> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxSecurity2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxSecurity2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxSecurity2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxSecurity2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxSecurity2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxSecurity2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxSecurity2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxSecurity2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxSecurity2 {
    type Vtable = IFaxSecurity2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17d851f4_d09b_48fc_99c9_8f24c4db9ab1);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxSecurity2_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Descriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvdescriptor: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Descriptor: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vdescriptor: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetDescriptor: usize,
    pub GrantedRights: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgrantedrights: *mut FAX_ACCESS_RIGHTS_ENUM_2) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InformationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plinformationtype: *mut i32) -> ::windows::core::HRESULT,
    pub SetInformationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linformationtype: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxSender(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxSender {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BillingCode(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).BillingCode)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBillingCode<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrbillingcode: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBillingCode)(::core::mem::transmute_copy(self), bstrbillingcode.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn City(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).City)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCity<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrcity: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCity)(::core::mem::transmute_copy(self), bstrcity.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Company(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Company)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCompany<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrcompany: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCompany)(::core::mem::transmute_copy(self), bstrcompany.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Country(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Country)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCountry<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrcountry: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCountry)(::core::mem::transmute_copy(self), bstrcountry.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Department(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Department)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDepartment<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdepartment: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDepartment)(::core::mem::transmute_copy(self), bstrdepartment.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Email(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Email)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEmail<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstremail: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEmail)(::core::mem::transmute_copy(self), bstremail.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FaxNumber(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FaxNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFaxNumber<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrfaxnumber: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFaxNumber)(::core::mem::transmute_copy(self), bstrfaxnumber.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HomePhone(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).HomePhone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetHomePhone<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrhomephone: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetHomePhone)(::core::mem::transmute_copy(self), bstrhomephone.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Name)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetName)(::core::mem::transmute_copy(self), bstrname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TSID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TSID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTSID<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrtsid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTSID)(::core::mem::transmute_copy(self), bstrtsid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OfficePhone(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).OfficePhone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOfficePhone<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrofficephone: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOfficePhone)(::core::mem::transmute_copy(self), bstrofficephone.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OfficeLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).OfficeLocation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOfficeLocation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrofficelocation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOfficeLocation)(::core::mem::transmute_copy(self), bstrofficelocation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn State(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).State)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetState<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrstate: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetState)(::core::mem::transmute_copy(self), bstrstate.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StreetAddress(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).StreetAddress)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStreetAddress<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrstreetaddress: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStreetAddress)(::core::mem::transmute_copy(self), bstrstreetaddress.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Title(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Title)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTitle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrtitle: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTitle)(::core::mem::transmute_copy(self), bstrtitle.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ZipCode(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ZipCode)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetZipCode<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrzipcode: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetZipCode)(::core::mem::transmute_copy(self), bstrzipcode.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn LoadDefaultSender(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LoadDefaultSender)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SaveDefaultSender(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SaveDefaultSender)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxSender> for ::windows::core::IUnknown {
    fn from(value: IFaxSender) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxSender> for ::windows::core::IUnknown {
    fn from(value: &IFaxSender) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxSender {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxSender {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxSender> for super::super::System::Com::IDispatch {
    fn from(value: IFaxSender) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxSender> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxSender) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxSender {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxSender {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxSender {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxSender {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxSender {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxSender {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxSender").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxSender {
    type Vtable = IFaxSender_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d879d7d_f57a_4cc6_a6f9_3ee5d527b46a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxSender_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub BillingCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrbillingcode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BillingCode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBillingCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrbillingcode: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBillingCode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub City: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcity: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    City: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcity: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCity: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Company: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcompany: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Company: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCompany: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcompany: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCompany: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Country: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcountry: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Country: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCountry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcountry: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCountry: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Department: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdepartment: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Department: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDepartment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdepartment: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDepartment: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Email: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstremail: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Email: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEmail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstremail: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEmail: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FaxNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfaxnumber: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FaxNumber: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFaxNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfaxnumber: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFaxNumber: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub HomePhone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrhomephone: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HomePhone: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetHomePhone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrhomephone: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetHomePhone: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OfficePhone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrofficephone: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OfficePhone: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetOfficePhone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrofficephone: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetOfficePhone: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OfficeLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrofficelocation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OfficeLocation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetOfficeLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrofficelocation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetOfficeLocation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrstate: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    State: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrstate: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub StreetAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrstreetaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StreetAddress: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetStreetAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrstreetaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetStreetAddress: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Title: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTitle: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ZipCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrzipcode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ZipCode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetZipCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrzipcode: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetZipCode: usize,
    pub LoadDefaultSender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SaveDefaultSender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxServer(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxServer {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Connect<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrservername: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Connect)(::core::mem::transmute_copy(self), bstrservername.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServerName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ServerName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDeviceProviders(&self) -> ::windows::core::Result<IFaxDeviceProviders> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDeviceProviders)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxDeviceProviders>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDevices(&self) -> ::windows::core::Result<IFaxDevices> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDevices)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxDevices>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InboundRouting(&self) -> ::windows::core::Result<IFaxInboundRouting> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).InboundRouting)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxInboundRouting>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Folders(&self) -> ::windows::core::Result<IFaxFolders> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Folders)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxFolders>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LoggingOptions(&self) -> ::windows::core::Result<IFaxLoggingOptions> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LoggingOptions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxLoggingOptions>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn MajorVersion(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MajorVersion)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn MinorVersion(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MinorVersion)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn MajorBuild(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MajorBuild)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn MinorBuild(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MinorBuild)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Debug(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Debug)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Activity(&self) -> ::windows::core::Result<IFaxActivity> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Activity)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxActivity>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OutboundRouting(&self) -> ::windows::core::Result<IFaxOutboundRouting> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).OutboundRouting)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxOutboundRouting>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReceiptOptions(&self) -> ::windows::core::Result<IFaxReceiptOptions> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ReceiptOptions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxReceiptOptions>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Security(&self) -> ::windows::core::Result<IFaxSecurity> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Security)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxSecurity>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Disconnect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Disconnect)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetExtensionProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrguid: Param0) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetExtensionProperty)(::core::mem::transmute_copy(self), bstrguid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetExtensionProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, bstrguid: Param0, vproperty: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetExtensionProperty)(::core::mem::transmute_copy(self), bstrguid.into_param().abi(), vproperty.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn ListenToServerEvents(&self, eventtypes: FAX_SERVER_EVENTS_TYPE_ENUM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ListenToServerEvents)(::core::mem::transmute_copy(self), ::core::mem::transmute(eventtypes)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterDeviceProvider<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrguid: Param0, bstrfriendlyname: Param1, bstrimagename: Param2, tspname: Param3, lfspiversion: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterDeviceProvider)(::core::mem::transmute_copy(self), bstrguid.into_param().abi(), bstrfriendlyname.into_param().abi(), bstrimagename.into_param().abi(), tspname.into_param().abi(), ::core::mem::transmute(lfspiversion)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UnregisterDeviceProvider<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstruniquename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnregisterDeviceProvider)(::core::mem::transmute_copy(self), bstruniquename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RegisterInboundRoutingExtension<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, bstrextensionname: Param0, bstrfriendlyname: Param1, bstrimagename: Param2, vmethods: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterInboundRoutingExtension)(::core::mem::transmute_copy(self), bstrextensionname.into_param().abi(), bstrfriendlyname.into_param().abi(), bstrimagename.into_param().abi(), vmethods.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UnregisterInboundRoutingExtension<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrextensionuniquename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnregisterInboundRoutingExtension)(::core::mem::transmute_copy(self), bstrextensionuniquename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn RegisteredEvents(&self) -> ::windows::core::Result<FAX_SERVER_EVENTS_TYPE_ENUM> {
        let mut result__: FAX_SERVER_EVENTS_TYPE_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RegisteredEvents)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_SERVER_EVENTS_TYPE_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn APIVersion(&self) -> ::windows::core::Result<FAX_SERVER_APIVERSION_ENUM> {
        let mut result__: FAX_SERVER_APIVERSION_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).APIVersion)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_SERVER_APIVERSION_ENUM>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxServer> for ::windows::core::IUnknown {
    fn from(value: IFaxServer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxServer> for ::windows::core::IUnknown {
    fn from(value: &IFaxServer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxServer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxServer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxServer> for super::super::System::Com::IDispatch {
    fn from(value: IFaxServer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxServer> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxServer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxServer {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxServer {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxServer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxServer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxServer {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxServer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxServer").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxServer {
    type Vtable = IFaxServer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x475b6469_90a5_4878_a577_17a86e8e3462);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxServer_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Connect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ServerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrservername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ServerName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDeviceProviders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfaxdeviceproviders: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDeviceProviders: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfaxdevices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDevices: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InboundRouting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfaxinboundrouting: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InboundRouting: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Folders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxfolders: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Folders: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub LoggingOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfaxloggingoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LoggingOptions: usize,
    pub MajorVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32) -> ::windows::core::HRESULT,
    pub MinorVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plminorversion: *mut i32) -> ::windows::core::HRESULT,
    pub MajorBuild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmajorbuild: *mut i32) -> ::windows::core::HRESULT,
    pub MinorBuild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plminorbuild: *mut i32) -> ::windows::core::HRESULT,
    pub Debug: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbdebug: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Activity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfaxactivity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Activity: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OutboundRouting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfaxoutboundrouting: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OutboundRouting: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ReceiptOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfaxreceiptoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReceiptOptions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Security: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfaxsecurity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Security: usize,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetExtensionProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvproperty: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetExtensionProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetExtensionProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, vproperty: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetExtensionProperty: usize,
    pub ListenToServerEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventtypes: FAX_SERVER_EVENTS_TYPE_ENUM) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterDeviceProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrimagename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, tspname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lfspiversion: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterDeviceProvider: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UnregisterDeviceProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstruniquename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UnregisterDeviceProvider: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RegisterInboundRoutingExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrextensionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrimagename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, vmethods: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RegisterInboundRoutingExtension: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UnregisterInboundRoutingExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrextensionuniquename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UnregisterInboundRoutingExtension: usize,
    pub RegisteredEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peventtypes: *mut FAX_SERVER_EVENTS_TYPE_ENUM) -> ::windows::core::HRESULT,
    pub APIVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, papiversion: *mut FAX_SERVER_APIVERSION_ENUM) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxServer2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxServer2 {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Connect<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrservername: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Connect)(::core::mem::transmute_copy(self), bstrservername.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServerName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ServerName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDeviceProviders(&self) -> ::windows::core::Result<IFaxDeviceProviders> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetDeviceProviders)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxDeviceProviders>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDevices(&self) -> ::windows::core::Result<IFaxDevices> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetDevices)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxDevices>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InboundRouting(&self) -> ::windows::core::Result<IFaxInboundRouting> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.InboundRouting)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxInboundRouting>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Folders(&self) -> ::windows::core::Result<IFaxFolders> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Folders)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxFolders>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LoggingOptions(&self) -> ::windows::core::Result<IFaxLoggingOptions> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.LoggingOptions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxLoggingOptions>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn MajorVersion(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.MajorVersion)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn MinorVersion(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.MinorVersion)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn MajorBuild(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.MajorBuild)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn MinorBuild(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.MinorBuild)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Debug(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Debug)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Activity(&self) -> ::windows::core::Result<IFaxActivity> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Activity)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxActivity>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OutboundRouting(&self) -> ::windows::core::Result<IFaxOutboundRouting> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.OutboundRouting)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxOutboundRouting>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReceiptOptions(&self) -> ::windows::core::Result<IFaxReceiptOptions> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ReceiptOptions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxReceiptOptions>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Security(&self) -> ::windows::core::Result<IFaxSecurity> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Security)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxSecurity>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Disconnect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Disconnect)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetExtensionProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrguid: Param0) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetExtensionProperty)(::core::mem::transmute_copy(self), bstrguid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetExtensionProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, bstrguid: Param0, vproperty: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetExtensionProperty)(::core::mem::transmute_copy(self), bstrguid.into_param().abi(), vproperty.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn ListenToServerEvents(&self, eventtypes: FAX_SERVER_EVENTS_TYPE_ENUM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.ListenToServerEvents)(::core::mem::transmute_copy(self), ::core::mem::transmute(eventtypes)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterDeviceProvider<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrguid: Param0, bstrfriendlyname: Param1, bstrimagename: Param2, tspname: Param3, lfspiversion: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.RegisterDeviceProvider)(::core::mem::transmute_copy(self), bstrguid.into_param().abi(), bstrfriendlyname.into_param().abi(), bstrimagename.into_param().abi(), tspname.into_param().abi(), ::core::mem::transmute(lfspiversion)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UnregisterDeviceProvider<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstruniquename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.UnregisterDeviceProvider)(::core::mem::transmute_copy(self), bstruniquename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RegisterInboundRoutingExtension<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, bstrextensionname: Param0, bstrfriendlyname: Param1, bstrimagename: Param2, vmethods: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.RegisterInboundRoutingExtension)(::core::mem::transmute_copy(self), bstrextensionname.into_param().abi(), bstrfriendlyname.into_param().abi(), bstrimagename.into_param().abi(), vmethods.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UnregisterInboundRoutingExtension<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrextensionuniquename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.UnregisterInboundRoutingExtension)(::core::mem::transmute_copy(self), bstrextensionuniquename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn RegisteredEvents(&self) -> ::windows::core::Result<FAX_SERVER_EVENTS_TYPE_ENUM> {
        let mut result__: FAX_SERVER_EVENTS_TYPE_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.RegisteredEvents)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_SERVER_EVENTS_TYPE_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn APIVersion(&self) -> ::windows::core::Result<FAX_SERVER_APIVERSION_ENUM> {
        let mut result__: FAX_SERVER_APIVERSION_ENUM = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.APIVersion)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FAX_SERVER_APIVERSION_ENUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Configuration(&self) -> ::windows::core::Result<IFaxConfiguration> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Configuration)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxConfiguration>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CurrentAccount(&self) -> ::windows::core::Result<IFaxAccount> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CurrentAccount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxAccount>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FaxAccountSet(&self) -> ::windows::core::Result<IFaxAccountSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FaxAccountSet)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxAccountSet>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Security2(&self) -> ::windows::core::Result<IFaxSecurity2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Security2)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFaxSecurity2>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxServer2> for ::windows::core::IUnknown {
    fn from(value: IFaxServer2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxServer2> for ::windows::core::IUnknown {
    fn from(value: &IFaxServer2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxServer2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxServer2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxServer2> for super::super::System::Com::IDispatch {
    fn from(value: IFaxServer2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxServer2> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxServer2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxServer2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxServer2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxServer2> for IFaxServer {
    fn from(value: IFaxServer2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxServer2> for IFaxServer {
    fn from(value: &IFaxServer2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFaxServer> for IFaxServer2 {
    fn into_param(self) -> ::windows::core::Param<'a, IFaxServer> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFaxServer> for &'a IFaxServer2 {
    fn into_param(self) -> ::windows::core::Param<'a, IFaxServer> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxServer2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxServer2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxServer2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxServer2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxServer2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxServer2 {
    type Vtable = IFaxServer2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x571ced0f_5609_4f40_9176_547e3a72ca7c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxServer2_Vtbl {
    pub base: IFaxServer_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfaxconfiguration: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Configuration: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CurrentAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcurrentaccount: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CurrentAccount: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub FaxAccountSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfaxaccountset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FaxAccountSet: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Security2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfaxsecurity2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Security2: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxServerNotify(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxServerNotify {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxServerNotify> for ::windows::core::IUnknown {
    fn from(value: IFaxServerNotify) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxServerNotify> for ::windows::core::IUnknown {
    fn from(value: &IFaxServerNotify) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxServerNotify {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxServerNotify {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxServerNotify> for super::super::System::Com::IDispatch {
    fn from(value: IFaxServerNotify) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxServerNotify> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxServerNotify) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxServerNotify {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxServerNotify {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxServerNotify {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxServerNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxServerNotify {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxServerNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxServerNotify").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxServerNotify {
    type Vtable = IFaxServerNotify_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e037b27_cf8a_4abd_b1e0_5704943bea6f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxServerNotify_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFaxServerNotify2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFaxServerNotify2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxServerNotify2> for ::windows::core::IUnknown {
    fn from(value: IFaxServerNotify2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxServerNotify2> for ::windows::core::IUnknown {
    fn from(value: &IFaxServerNotify2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFaxServerNotify2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFaxServerNotify2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFaxServerNotify2> for super::super::System::Com::IDispatch {
    fn from(value: IFaxServerNotify2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFaxServerNotify2> for super::super::System::Com::IDispatch {
    fn from(value: &IFaxServerNotify2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFaxServerNotify2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFaxServerNotify2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFaxServerNotify2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFaxServerNotify2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFaxServerNotify2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFaxServerNotify2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFaxServerNotify2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFaxServerNotify2 {
    type Vtable = IFaxServerNotify2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x616ca8d6_a77a_4062_abfd_0e471241c7aa);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFaxServerNotify2_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const IS_DIGITAL_CAMERA_STR: &'static str = "IsDigitalCamera";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const IS_DIGITAL_CAMERA_VAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
pub struct IStiDevice(::windows::core::IUnknown);
impl IStiDevice {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hinst: Param0, pwszdevicename: Param1, dwversion: u32, dwmode: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::core::mem::transmute_copy(self), hinst.into_param().abi(), pwszdevicename.into_param().abi(), ::core::mem::transmute(dwversion), ::core::mem::transmute(dwmode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn GetCapabilities(&self, pdevcaps: *mut STI_DEV_CAPS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCapabilities)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdevcaps)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn GetStatus(&self, pdevstatus: *mut STI_DEVICE_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdevstatus)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn DeviceReset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeviceReset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Diagnostic(&self, pbuffer: *mut STI_DIAG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Diagnostic)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbuffer)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Escape(&self, escapefunction: u32, lpindata: *const ::core::ffi::c_void, cbindatasize: u32, poutdata: *mut ::core::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Escape)(::core::mem::transmute_copy(self), ::core::mem::transmute(escapefunction), ::core::mem::transmute(lpindata), ::core::mem::transmute(cbindatasize), ::core::mem::transmute(poutdata), ::core::mem::transmute(dwoutdatasize), ::core::mem::transmute(pdwactualdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn GetLastError(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLastError)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn LockDevice(&self, dwtimeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LockDevice)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwtimeout)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn UnLockDevice(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnLockDevice)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn RawReadData(&self, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RawReadData)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(lpdwnumberofbytes), ::core::mem::transmute(lpoverlapped)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn RawWriteData(&self, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RawWriteData)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(nnumberofbytes), ::core::mem::transmute(lpoverlapped)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn RawReadCommand(&self, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RawReadCommand)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(lpdwnumberofbytes), ::core::mem::transmute(lpoverlapped)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn RawWriteCommand(&self, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RawWriteCommand)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(nnumberofbytes), ::core::mem::transmute(lpoverlapped)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Subscribe(&self, lpsubsribe: *mut STISUBSCRIBE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Subscribe)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpsubsribe)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn GetLastNotificationData(&self) -> ::windows::core::Result<STINOTIFY> {
        let mut result__: STINOTIFY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLastNotificationData)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<STINOTIFY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn UnSubscribe(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnSubscribe)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn GetLastErrorInfo(&self) -> ::windows::core::Result<_ERROR_INFOW> {
        let mut result__: _ERROR_INFOW = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLastErrorInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<_ERROR_INFOW>(result__)
    }
}
impl ::core::convert::From<IStiDevice> for ::windows::core::IUnknown {
    fn from(value: IStiDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStiDevice> for ::windows::core::IUnknown {
    fn from(value: &IStiDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStiDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStiDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStiDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStiDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStiDevice {}
impl ::core::fmt::Debug for IStiDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStiDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IStiDevice {
    type Vtable = IStiDevice_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6cfa5a80_2dc8_11d0_90ea_00aa0060f86c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStiDevice_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hinst: super::super::Foundation::HINSTANCE, pwszdevicename: ::windows::core::PCWSTR, dwversion: u32, dwmode: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    pub GetCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevcaps: *mut STI_DEV_CAPS) -> ::windows::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevstatus: *mut STI_DEVICE_STATUS) -> ::windows::core::HRESULT,
    pub DeviceReset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Diagnostic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffer: *mut STI_DIAG) -> ::windows::core::HRESULT,
    pub Escape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, escapefunction: u32, lpindata: *const ::core::ffi::c_void, cbindatasize: u32, poutdata: *mut ::core::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> ::windows::core::HRESULT,
    pub GetLastError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlastdeviceerror: *mut u32) -> ::windows::core::HRESULT,
    pub LockDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwtimeout: u32) -> ::windows::core::HRESULT,
    pub UnLockDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub RawReadData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    RawReadData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub RawWriteData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    RawWriteData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub RawReadCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    RawReadCommand: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub RawWriteCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    RawWriteCommand: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Subscribe: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpsubsribe: *mut STISUBSCRIBE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Subscribe: usize,
    pub GetLastNotificationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpnotify: *mut STINOTIFY) -> ::windows::core::HRESULT,
    pub UnSubscribe: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetLastErrorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plasterrorinfo: *mut _ERROR_INFOW) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
pub struct IStiDeviceControl(::windows::core::IUnknown);
impl IStiDeviceControl {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Initialize<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, dwdevicetype: u32, dwmode: u32, pwszportname: Param2, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwdevicetype), ::core::mem::transmute(dwmode), pwszportname.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn RawReadData(&self, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RawReadData)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(lpdwnumberofbytes), ::core::mem::transmute(lpoverlapped)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn RawWriteData(&self, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RawWriteData)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(nnumberofbytes), ::core::mem::transmute(lpoverlapped)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn RawReadCommand(&self, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RawReadCommand)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(lpdwnumberofbytes), ::core::mem::transmute(lpoverlapped)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn RawWriteCommand(&self, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RawWriteCommand)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(nnumberofbytes), ::core::mem::transmute(lpoverlapped)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn RawDeviceControl(&self, escapefunction: u32, lpindata: *mut ::core::ffi::c_void, cbindatasize: u32, poutdata: *mut ::core::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RawDeviceControl)(::core::mem::transmute_copy(self), ::core::mem::transmute(escapefunction), ::core::mem::transmute(lpindata), ::core::mem::transmute(cbindatasize), ::core::mem::transmute(poutdata), ::core::mem::transmute(dwoutdatasize), ::core::mem::transmute(pdwactualdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn GetLastError(&self, lpdwlasterror: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLastError)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpdwlasterror)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn GetMyDevicePortName(&self, lpszdevicepath: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMyDevicePortName)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpszdevicepath)), lpszdevicepath.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMyDeviceHandle(&self, lph: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMyDeviceHandle)(::core::mem::transmute_copy(self), ::core::mem::transmute(lph)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn GetMyDeviceOpenMode(&self, pdwopenmode: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMyDeviceOpenMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwopenmode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn WriteToErrorLog<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, dwmessagetype: u32, pszmessage: Param1, dwerrorcode: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteToErrorLog)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwmessagetype), pszmessage.into_param().abi(), ::core::mem::transmute(dwerrorcode)).ok()
    }
}
impl ::core::convert::From<IStiDeviceControl> for ::windows::core::IUnknown {
    fn from(value: IStiDeviceControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStiDeviceControl> for ::windows::core::IUnknown {
    fn from(value: &IStiDeviceControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStiDeviceControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStiDeviceControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStiDeviceControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStiDeviceControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStiDeviceControl {}
impl ::core::fmt::Debug for IStiDeviceControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStiDeviceControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IStiDeviceControl {
    type Vtable = IStiDeviceControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x128a9860_52dc_11d0_9edf_444553540000);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStiDeviceControl_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdevicetype: u32, dwmode: u32, pwszportname: ::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub RawReadData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    RawReadData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub RawWriteData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    RawWriteData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub RawReadCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    RawReadCommand: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub RawWriteCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    RawWriteCommand: usize,
    pub RawDeviceControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, escapefunction: u32, lpindata: *mut ::core::ffi::c_void, cbindatasize: u32, poutdata: *mut ::core::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> ::windows::core::HRESULT,
    pub GetLastError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpdwlasterror: *mut u32) -> ::windows::core::HRESULT,
    pub GetMyDevicePortName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszdevicepath: ::windows::core::PWSTR, cwdevicepathsize: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMyDeviceHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lph: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMyDeviceHandle: usize,
    pub GetMyDeviceOpenMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwopenmode: *mut u32) -> ::windows::core::HRESULT,
    pub WriteToErrorLog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmessagetype: u32, pszmessage: ::windows::core::PCWSTR, dwerrorcode: u32) -> ::windows::core::HRESULT,
}
#[repr(C)]
pub struct IStiDeviceW(pub u8);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
pub struct IStiUSD(::windows::core::IUnknown);
impl IStiUSD {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Registry\"`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, IStiDeviceControl>, Param2: ::windows::core::IntoParam<'a, super::super::System::Registry::HKEY>>(&self, pheldcb: Param0, dwstiversion: u32, hparameterskey: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::core::mem::transmute_copy(self), pheldcb.into_param().abi(), ::core::mem::transmute(dwstiversion), hparameterskey.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn GetCapabilities(&self) -> ::windows::core::Result<STI_USD_CAPS> {
        let mut result__: STI_USD_CAPS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCapabilities)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<STI_USD_CAPS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn GetStatus(&self, pdevstatus: *mut STI_DEVICE_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdevstatus)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn DeviceReset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeviceReset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Diagnostic(&self, pbuffer: *mut STI_DIAG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Diagnostic)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbuffer)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn Escape(&self, escapefunction: u32, lpindata: *const ::core::ffi::c_void, cbindatasize: u32, poutdata: *mut ::core::ffi::c_void, cboutdatasize: u32, pdwactualdata: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Escape)(::core::mem::transmute_copy(self), ::core::mem::transmute(escapefunction), ::core::mem::transmute(lpindata), ::core::mem::transmute(cbindatasize), ::core::mem::transmute(poutdata), ::core::mem::transmute(cboutdatasize), ::core::mem::transmute(pdwactualdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn GetLastError(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLastError)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn LockDevice(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LockDevice)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn UnLockDevice(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnLockDevice)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn RawReadData(&self, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RawReadData)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(lpdwnumberofbytes), ::core::mem::transmute(lpoverlapped)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn RawWriteData(&self, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RawWriteData)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(nnumberofbytes), ::core::mem::transmute(lpoverlapped)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn RawReadCommand(&self, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RawReadCommand)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(lpdwnumberofbytes), ::core::mem::transmute(lpoverlapped)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn RawWriteCommand(&self, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RawWriteCommand)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(nnumberofbytes), ::core::mem::transmute(lpoverlapped)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNotificationHandle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, hevent: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetNotificationHandle)(::core::mem::transmute_copy(self), hevent.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn GetNotificationData(&self) -> ::windows::core::Result<STINOTIFY> {
        let mut result__: STINOTIFY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetNotificationData)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<STINOTIFY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn GetLastErrorInfo(&self) -> ::windows::core::Result<_ERROR_INFOW> {
        let mut result__: _ERROR_INFOW = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLastErrorInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<_ERROR_INFOW>(result__)
    }
}
impl ::core::convert::From<IStiUSD> for ::windows::core::IUnknown {
    fn from(value: IStiUSD) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStiUSD> for ::windows::core::IUnknown {
    fn from(value: &IStiUSD) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStiUSD {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStiUSD {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStiUSD {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStiUSD {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStiUSD {}
impl ::core::fmt::Debug for IStiUSD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStiUSD").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IStiUSD {
    type Vtable = IStiUSD_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c9bb460_51ac_11d0_90ea_00aa0060f86c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStiUSD_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Registry")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pheldcb: ::windows::core::RawPtr, dwstiversion: u32, hparameterskey: super::super::System::Registry::HKEY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    Initialize: usize,
    pub GetCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevcaps: *mut STI_USD_CAPS) -> ::windows::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevstatus: *mut STI_DEVICE_STATUS) -> ::windows::core::HRESULT,
    pub DeviceReset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Diagnostic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffer: *mut STI_DIAG) -> ::windows::core::HRESULT,
    pub Escape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, escapefunction: u32, lpindata: *const ::core::ffi::c_void, cbindatasize: u32, poutdata: *mut ::core::ffi::c_void, cboutdatasize: u32, pdwactualdata: *mut u32) -> ::windows::core::HRESULT,
    pub GetLastError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlastdeviceerror: *mut u32) -> ::windows::core::HRESULT,
    pub LockDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UnLockDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub RawReadData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    RawReadData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub RawWriteData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    RawWriteData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub RawReadCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    RawReadCommand: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub RawWriteCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    RawWriteCommand: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNotificationHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNotificationHandle: usize,
    pub GetNotificationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpnotify: *mut STINOTIFY) -> ::windows::core::HRESULT,
    pub GetLastErrorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plasterrorinfo: *mut _ERROR_INFOW) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
pub struct IStillImageW(::windows::core::IUnknown);
impl IStillImageW {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(&self, hinst: Param0, dwversion: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::core::mem::transmute_copy(self), hinst.into_param().abi(), ::core::mem::transmute(dwversion)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn GetDeviceList(&self, dwtype: u32, dwflags: u32, pdwitemsreturned: *mut u32, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDeviceList)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwtype), ::core::mem::transmute(dwflags), ::core::mem::transmute(pdwitemsreturned), ::core::mem::transmute(ppbuffer)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn GetDeviceInfo<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszdevicename: Param0, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDeviceInfo)(::core::mem::transmute_copy(self), pwszdevicename.into_param().abi(), ::core::mem::transmute(ppbuffer)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn CreateDevice<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pwszdevicename: Param0, dwmode: u32, pdevice: *mut ::core::option::Option<IStiDevice>, punkouter: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateDevice)(::core::mem::transmute_copy(self), pwszdevicename.into_param().abi(), ::core::mem::transmute(dwmode), ::core::mem::transmute(pdevice), punkouter.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn GetDeviceValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszdevicename: Param0, pvaluename: Param1, ptype: *mut u32, pdata: *mut u8, cbdata: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDeviceValue)(::core::mem::transmute_copy(self), pwszdevicename.into_param().abi(), pvaluename.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pdata), ::core::mem::transmute(cbdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetDeviceValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszdevicename: Param0, pvaluename: Param1, r#type: u32, pdata: *const u8, cbdata: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDeviceValue)(::core::mem::transmute_copy(self), pwszdevicename.into_param().abi(), pvaluename.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(pdata), ::core::mem::transmute(cbdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn GetSTILaunchInformation(&self, pwszdevicename: &mut [u16; 128], pdweventcode: *mut u32, pwszeventname: &mut [u16; 128]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSTILaunchInformation)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pwszdevicename)), ::core::mem::transmute(pdweventcode), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pwszeventname))).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn RegisterLaunchApplication<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszappname: Param0, pwszcommandline: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterLaunchApplication)(::core::mem::transmute_copy(self), pwszappname.into_param().abi(), pwszcommandline.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn UnregisterLaunchApplication<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszappname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnregisterLaunchApplication)(::core::mem::transmute_copy(self), pwszappname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableHwNotifications<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pwszdevicename: Param0, bnewstate: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnableHwNotifications)(::core::mem::transmute_copy(self), pwszdevicename.into_param().abi(), bnewstate.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHwNotificationState<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszdevicename: Param0) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetHwNotificationState)(::core::mem::transmute_copy(self), pwszdevicename.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn RefreshDeviceBus<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszdevicename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RefreshDeviceBus)(::core::mem::transmute_copy(self), pwszdevicename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn LaunchApplicationForDevice<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszdevicename: Param0, pwszappname: Param1, pstinotify: *const STINOTIFY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LaunchApplicationForDevice)(::core::mem::transmute_copy(self), pwszdevicename.into_param().abi(), pwszappname.into_param().abi(), ::core::mem::transmute(pstinotify)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn SetupDeviceParameters(&self, param0: *mut STI_DEVICE_INFORMATIONW) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetupDeviceParameters)(::core::mem::transmute_copy(self), ::core::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
    pub unsafe fn WriteToErrorLog<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, dwmessagetype: u32, pszmessage: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteToErrorLog)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwmessagetype), pszmessage.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IStillImageW> for ::windows::core::IUnknown {
    fn from(value: IStillImageW) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStillImageW> for ::windows::core::IUnknown {
    fn from(value: &IStillImageW) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStillImageW {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStillImageW {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStillImageW {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStillImageW {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStillImageW {}
impl ::core::fmt::Debug for IStillImageW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStillImageW").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IStillImageW {
    type Vtable = IStillImageW_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x641bd880_2dc8_11d0_90ea_00aa0060f86c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStillImageW_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hinst: super::super::Foundation::HINSTANCE, dwversion: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    pub GetDeviceList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwtype: u32, dwflags: u32, pdwitemsreturned: *mut u32, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDeviceInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdevicename: ::windows::core::PCWSTR, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdevicename: ::windows::core::PCWSTR, dwmode: u32, pdevice: *mut ::windows::core::RawPtr, punkouter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDeviceValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdevicename: ::windows::core::PCWSTR, pvaluename: ::windows::core::PCWSTR, ptype: *mut u32, pdata: *mut u8, cbdata: *mut u32) -> ::windows::core::HRESULT,
    pub SetDeviceValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdevicename: ::windows::core::PCWSTR, pvaluename: ::windows::core::PCWSTR, r#type: u32, pdata: *const u8, cbdata: u32) -> ::windows::core::HRESULT,
    pub GetSTILaunchInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdevicename: ::windows::core::PWSTR, pdweventcode: *mut u32, pwszeventname: ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub RegisterLaunchApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszappname: ::windows::core::PCWSTR, pwszcommandline: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub UnregisterLaunchApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszappname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnableHwNotifications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdevicename: ::windows::core::PCWSTR, bnewstate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnableHwNotifications: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHwNotificationState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdevicename: ::windows::core::PCWSTR, pbcurrentstate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHwNotificationState: usize,
    pub RefreshDeviceBus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdevicename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub LaunchApplicationForDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdevicename: ::windows::core::PCWSTR, pwszappname: ::windows::core::PCWSTR, pstinotify: *const STINOTIFY) -> ::windows::core::HRESULT,
    pub SetupDeviceParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut STI_DEVICE_INFORMATIONW) -> ::windows::core::HRESULT,
    pub WriteToErrorLog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmessagetype: u32, pszmessage: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JS_DELETING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JS_FAILED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JS_INPROGRESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JS_NOLINE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JS_PAUSED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JS_PENDING: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JS_RETRIES_EXCEEDED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JS_RETRYING: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JT_FAIL_RECEIVE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JT_RECEIVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JT_ROUTING: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JT_SEND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const JT_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const MAX_NOTIFICATION_DATA: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const MS_FAXROUTE_EMAIL_GUID: &'static str = "{6bbf7bfe-9af2-11d0-abf7-00c04fd91a4e}";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const MS_FAXROUTE_FOLDER_GUID: &'static str = "{92041a90-9af2-11d0-abf7-00c04fd91a4e}";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const MS_FAXROUTE_PRINTING_GUID: &'static str = "{aec1b37c-9af2-11d0-abf7-00c04fd91a4e}";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXABORT = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobid: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXACCESSCHECK = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, accessmask: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXCLOSE = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXCOMPLETEJOBPARAMSA = ::core::option::Option<unsafe extern "system" fn(jobparams: *mut *mut FAX_JOB_PARAMA, coverpageinfo: *mut *mut FAX_COVERPAGE_INFOA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXCOMPLETEJOBPARAMSW = ::core::option::Option<unsafe extern "system" fn(jobparams: *mut *mut FAX_JOB_PARAMW, coverpageinfo: *mut *mut FAX_COVERPAGE_INFOW) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXCONNECTFAXSERVERA = ::core::option::Option<unsafe extern "system" fn(machinename: ::windows::core::PCSTR, faxhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXCONNECTFAXSERVERW = ::core::option::Option<unsafe extern "system" fn(machinename: ::windows::core::PCWSTR, faxhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXDEVABORTOPERATION = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub type PFAXDEVCONFIGURE = ::core::option::Option<unsafe extern "system" fn(param0: *mut super::super::UI::Controls::HPROPSHEETPAGE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXDEVENDJOB = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXDEVINITIALIZE = ::core::option::Option<unsafe extern "system" fn(param0: u32, param1: super::super::Foundation::HANDLE, param2: *mut PFAX_LINECALLBACK, param3: PFAX_SERVICE_CALLBACK) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXDEVRECEIVE = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: u32, param2: *mut FAX_RECEIVE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXDEVREPORTSTATUS = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: *mut FAX_DEV_STATUS, param2: u32, param3: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXDEVSEND = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: *mut FAX_SEND, param2: PFAX_SEND_CALLBACK) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type PFAXDEVSHUTDOWN = ::core::option::Option<unsafe extern "system" fn() -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXDEVSTARTJOB = ::core::option::Option<unsafe extern "system" fn(param0: u32, param1: u32, param2: *mut super::super::Foundation::HANDLE, param3: super::super::Foundation::HANDLE, param4: usize) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXDEVVIRTUALDEVICECREATION = ::core::option::Option<unsafe extern "system" fn(devicecount: *mut u32, devicenameprefix: ::windows::core::PWSTR, deviceidprefix: *mut u32, completionport: super::super::Foundation::HANDLE, completionkey: usize) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENABLEROUTINGMETHODA = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingguid: ::windows::core::PCSTR, enabled: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENABLEROUTINGMETHODW = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingguid: ::windows::core::PCWSTR, enabled: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENUMGLOBALROUTINGINFOA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, routinginfo: *mut *mut FAX_GLOBAL_ROUTING_INFOA, methodsreturned: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENUMGLOBALROUTINGINFOW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, routinginfo: *mut *mut FAX_GLOBAL_ROUTING_INFOW, methodsreturned: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENUMJOBSA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobentry: *mut *mut FAX_JOB_ENTRYA, jobsreturned: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENUMJOBSW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobentry: *mut *mut FAX_JOB_ENTRYW, jobsreturned: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENUMPORTSA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOA, portsreturned: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENUMPORTSW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOW, portsreturned: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENUMROUTINGMETHODSA = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingmethod: *mut *mut FAX_ROUTING_METHODA, methodsreturned: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENUMROUTINGMETHODSW = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingmethod: *mut *mut FAX_ROUTING_METHODW, methodsreturned: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type PFAXFREEBUFFER = ::core::option::Option<unsafe extern "system" fn(buffer: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETCONFIGURATIONA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, faxconfig: *mut *mut FAX_CONFIGURATIONA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETCONFIGURATIONW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, faxconfig: *mut *mut FAX_CONFIGURATIONW) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETDEVICESTATUSA = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, devicestatus: *mut *mut FAX_DEVICE_STATUSA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETDEVICESTATUSW = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, devicestatus: *mut *mut FAX_DEVICE_STATUSW) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETJOBA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobid: u32, jobentry: *mut *mut FAX_JOB_ENTRYA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETJOBW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobid: u32, jobentry: *mut *mut FAX_JOB_ENTRYW) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETLOGGINGCATEGORIESA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, categories: *mut *mut FAX_LOG_CATEGORYA, numbercategories: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETLOGGINGCATEGORIESW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, categories: *mut *mut FAX_LOG_CATEGORYW, numbercategories: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETPAGEDATA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobid: u32, buffer: *mut *mut u8, buffersize: *mut u32, imagewidth: *mut u32, imageheight: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETPORTA = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETPORTW = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOW) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETROUTINGINFOA = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingguid: ::windows::core::PCSTR, routinginfobuffer: *mut *mut u8, routinginfobuffersize: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETROUTINGINFOW = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingguid: ::windows::core::PCWSTR, routinginfobuffer: *mut *mut u8, routinginfobuffersize: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXINITIALIZEEVENTQUEUE = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, completionport: super::super::Foundation::HANDLE, completionkey: usize, hwnd: super::super::Foundation::HWND, messagestart: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXOPENPORT = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, deviceid: u32, flags: u32, faxporthandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFAXPRINTCOVERPAGEA = ::core::option::Option<unsafe extern "system" fn(faxcontextinfo: *const FAX_CONTEXT_INFOA, coverpageinfo: *const FAX_COVERPAGE_INFOA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFAXPRINTCOVERPAGEW = ::core::option::Option<unsafe extern "system" fn(faxcontextinfo: *const FAX_CONTEXT_INFOW, coverpageinfo: *const FAX_COVERPAGE_INFOW) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXREGISTERROUTINGEXTENSIONW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, extensionname: ::windows::core::PCWSTR, friendlyname: ::windows::core::PCWSTR, imagename: ::windows::core::PCWSTR, callback: PFAX_ROUTING_INSTALLATION_CALLBACKW, context: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXREGISTERSERVICEPROVIDERW = ::core::option::Option<unsafe extern "system" fn(deviceprovider: ::windows::core::PCWSTR, friendlyname: ::windows::core::PCWSTR, imagename: ::windows::core::PCWSTR, tspname: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type PFAXROUTEADDFILE = ::core::option::Option<unsafe extern "system" fn(jobid: u32, filename: ::windows::core::PCWSTR, guid: *mut ::windows::core::GUID) -> i32>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type PFAXROUTEDELETEFILE = ::core::option::Option<unsafe extern "system" fn(jobid: u32, filename: ::windows::core::PCWSTR) -> i32>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEDEVICECHANGENOTIFICATION = ::core::option::Option<unsafe extern "system" fn(param0: u32, param1: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEDEVICEENABLE = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCWSTR, param1: u32, param2: i32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEENUMFILE = ::core::option::Option<unsafe extern "system" fn(jobid: u32, guidowner: *mut ::windows::core::GUID, guidcaller: *mut ::windows::core::GUID, filename: ::windows::core::PCWSTR, context: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEENUMFILES = ::core::option::Option<unsafe extern "system" fn(jobid: u32, guid: *mut ::windows::core::GUID, fileenumerator: PFAXROUTEENUMFILE, context: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEGETFILE = ::core::option::Option<unsafe extern "system" fn(jobid: u32, index: u32, filenamebuffer: ::windows::core::PWSTR, requiredsize: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEGETROUTINGINFO = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCWSTR, param1: u32, param2: *mut u8, param3: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEINITIALIZE = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: *mut FAX_ROUTE_CALLBACKROUTINES) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEMETHOD = ::core::option::Option<unsafe extern "system" fn(param0: *const FAX_ROUTE, param1: *mut *mut ::core::ffi::c_void, param2: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEMODIFYROUTINGDATA = ::core::option::Option<unsafe extern "system" fn(jobid: u32, routingguid: ::windows::core::PCWSTR, routingdata: *mut u8, routingdatasize: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTESETROUTINGINFO = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCWSTR, param1: u32, param2: *const u8, param3: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSENDDOCUMENTA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, filename: ::windows::core::PCSTR, jobparams: *mut FAX_JOB_PARAMA, coverpageinfo: *const FAX_COVERPAGE_INFOA, faxjobid: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSENDDOCUMENTFORBROADCASTA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, filename: ::windows::core::PCSTR, faxjobid: *mut u32, faxrecipientcallback: PFAX_RECIPIENT_CALLBACKA, context: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSENDDOCUMENTFORBROADCASTW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, filename: ::windows::core::PCWSTR, faxjobid: *mut u32, faxrecipientcallback: PFAX_RECIPIENT_CALLBACKW, context: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSENDDOCUMENTW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, filename: ::windows::core::PCWSTR, jobparams: *mut FAX_JOB_PARAMW, coverpageinfo: *const FAX_COVERPAGE_INFOW, faxjobid: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETCONFIGURATIONA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, faxconfig: *const FAX_CONFIGURATIONA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETCONFIGURATIONW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, faxconfig: *const FAX_CONFIGURATIONW) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETGLOBALROUTINGINFOA = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routinginfo: *const FAX_GLOBAL_ROUTING_INFOA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETGLOBALROUTINGINFOW = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routinginfo: *const FAX_GLOBAL_ROUTING_INFOW) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETJOBA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobid: u32, command: u32, jobentry: *const FAX_JOB_ENTRYA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETJOBW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobid: u32, command: u32, jobentry: *const FAX_JOB_ENTRYW) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETLOGGINGCATEGORIESA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, categories: *const FAX_LOG_CATEGORYA, numbercategories: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETLOGGINGCATEGORIESW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, categories: *const FAX_LOG_CATEGORYW, numbercategories: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETPORTA = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, portinfo: *const FAX_PORT_INFOA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETPORTW = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, portinfo: *const FAX_PORT_INFOW) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETROUTINGINFOA = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingguid: ::windows::core::PCSTR, routinginfobuffer: *const u8, routinginfobuffersize: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETROUTINGINFOW = ::core::option::Option<unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingguid: ::windows::core::PCWSTR, routinginfobuffer: *const u8, routinginfobuffersize: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFAXSTARTPRINTJOBA = ::core::option::Option<unsafe extern "system" fn(printername: ::windows::core::PCSTR, printinfo: *const FAX_PRINT_INFOA, faxjobid: *mut u32, faxcontextinfo: *mut FAX_CONTEXT_INFOA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFAXSTARTPRINTJOBW = ::core::option::Option<unsafe extern "system" fn(printername: ::windows::core::PCWSTR, printinfo: *const FAX_PRINT_INFOW, faxjobid: *mut u32, faxcontextinfo: *mut FAX_CONTEXT_INFOW) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAXUNREGISTERSERVICEPROVIDERW = ::core::option::Option<unsafe extern "system" fn(deviceprovider: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type PFAX_EXT_CONFIG_CHANGE = ::core::option::Option<unsafe extern "system" fn(param0: u32, param1: ::windows::core::PCWSTR, param2: *mut u8, param3: u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type PFAX_EXT_FREE_BUFFER = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub type PFAX_EXT_GET_DATA = ::core::option::Option<unsafe extern "system" fn(param0: u32, param1: FAX_ENUM_DEVICE_ID_SOURCE, param2: ::windows::core::PCWSTR, param3: *mut *mut u8, param4: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_EXT_INITIALIZE_CONFIG = ::core::option::Option<unsafe extern "system" fn(param0: PFAX_EXT_GET_DATA, param1: PFAX_EXT_SET_DATA, param2: PFAX_EXT_REGISTER_FOR_EVENTS, param3: PFAX_EXT_UNREGISTER_FOR_EVENTS, param4: PFAX_EXT_FREE_BUFFER) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_EXT_REGISTER_FOR_EVENTS = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HINSTANCE, param1: u32, param2: FAX_ENUM_DEVICE_ID_SOURCE, param3: ::windows::core::PCWSTR, param4: PFAX_EXT_CONFIG_CHANGE) -> super::super::Foundation::HANDLE>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_EXT_SET_DATA = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HINSTANCE, param1: u32, param2: FAX_ENUM_DEVICE_ID_SOURCE, param3: ::windows::core::PCWSTR, param4: *mut u8, param5: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_EXT_UNREGISTER_FOR_EVENTS = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE) -> u32>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_LINECALLBACK = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, hdevice: u32, dwmessage: u32, dwinstance: usize, dwparam1: usize, dwparam2: usize, dwparam3: usize)>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_RECIPIENT_CALLBACKA = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, recipientnumber: u32, context: *mut ::core::ffi::c_void, jobparams: *mut FAX_JOB_PARAMA, coverpageinfo: *mut FAX_COVERPAGE_INFOA) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_RECIPIENT_CALLBACKW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, recipientnumber: u32, context: *mut ::core::ffi::c_void, jobparams: *mut FAX_JOB_PARAMW, coverpageinfo: *mut FAX_COVERPAGE_INFOW) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_ROUTING_INSTALLATION_CALLBACKW = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, context: *mut ::core::ffi::c_void, methodname: ::windows::core::PCWSTR, friendlyname: ::windows::core::PCWSTR, functionname: ::windows::core::PCWSTR, guid: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_SEND_CALLBACK = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, callhandle: u32, reserved1: u32, reserved2: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_SERVICE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, deviceid: u32, param1: usize, param2: usize, param3: usize) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_BAUDRATE: &'static str = "BaudRate";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_BAUDRATE_A: &'static str = "BaudRate";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_DATA_W: &'static str = "DeviceData";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_DEVICESUBTYPE_W: &'static str = "DeviceSubType";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_DEVICETYPE_W: &'static str = "DeviceType";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_DEVICE_NAME_W: &'static str = "DriverDesc";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_DEV_NAME_W: &'static str = "DeviceName";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_DRIVER_DESC_W: &'static str = "DriverDesc";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_FRIENDLY_NAME_W: &'static str = "FriendlyName";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_GENERIC_CAPS_W: &'static str = "Capabilities";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_GUID: &'static str = "GUID";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_GUID_W: &'static str = "GUID";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_HARDWARE: &'static str = "HardwareConfig";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_HARDWARE_W: &'static str = "HardwareConfig";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_LAUNCHABLE: &'static str = "Launchable";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_LAUNCHABLE_W: &'static str = "Launchable";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_LAUNCH_APPS: &'static str = "LaunchApplications";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_LAUNCH_APPS_W: &'static str = "LaunchApplications";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_SHUTDOWNDELAY: &'static str = "ShutdownIfUnusedDelay";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_SHUTDOWNDELAY_W: &'static str = "ShutdownIfUnusedDelay";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_TYPE_W: &'static str = "Type";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const REGSTR_VAL_VENDOR_NAME_W: &'static str = "Vendor";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIEDFL_ALLDEVICES: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIEDFL_ATTACHEDONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_ALREADY_INITIALIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147023649i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_BADDRIVER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147024777i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_BETA_VERSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147023743i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_DEVICENOTREG: i32 = -2147221164i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_DEVICE_LOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147024863i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_DEVICE_NOTREADY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147024875i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_GENERIC: i32 = -2147467259i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_HANDLEEXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147024713i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_INVALID_DEVICE_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147024773i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_INVALID_HW_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147024883i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_INVALID_PARAM: i32 = -2147024809i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_NEEDS_LOCK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147024738i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_NOEVENTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147024637i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_NOINTERFACE: i32 = -2147467262i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_NOTINITIALIZED: i32 = -2147024891i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_NOT_INITIALIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147024875i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_OBJECTNOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147024894i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_OLD_VERSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147023746i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_OUTOFMEMORY: i32 = -2147024882i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_READONLY: i32 = -2147024891i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_SHARING_VIOLATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147024864i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STIERR_UNSUPPORTED: i32 = -2147467263i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct STINOTIFY {
    pub dwSize: u32,
    pub guidNotificationCode: ::windows::core::GUID,
    pub abNotificationData: [u8; 64],
}
impl ::core::marker::Copy for STINOTIFY {}
impl ::core::clone::Clone for STINOTIFY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STINOTIFY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STINOTIFY").field("dwSize", &self.dwSize).field("guidNotificationCode", &self.guidNotificationCode).field("abNotificationData", &self.abNotificationData).finish()
    }
}
unsafe impl ::windows::core::Abi for STINOTIFY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STINOTIFY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STINOTIFY>()) == 0 }
    }
}
impl ::core::cmp::Eq for STINOTIFY {}
impl ::core::default::Default for STINOTIFY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STISUBSCRIBE {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwFilter: u32,
    pub hWndNotify: super::super::Foundation::HWND,
    pub hEvent: super::super::Foundation::HANDLE,
    pub uiNotificationMessage: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STISUBSCRIBE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STISUBSCRIBE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STISUBSCRIBE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STISUBSCRIBE").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwFilter", &self.dwFilter).field("hWndNotify", &self.hWndNotify).field("hEvent", &self.hEvent).field("uiNotificationMessage", &self.uiNotificationMessage).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for STISUBSCRIBE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STISUBSCRIBE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STISUBSCRIBE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STISUBSCRIBE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STISUBSCRIBE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ADD_DEVICE_BROADCAST_ACTION: &'static str = "Arrival";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ADD_DEVICE_BROADCAST_STRING: &'static str = "STI\\";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_CHANGENOEFFECT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_CREATE_BOTH: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_CREATE_DATA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_CREATE_FOR_MONITOR: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_CREATE_MASK: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_CREATE_STATUS: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct STI_DEVICE_INFORMATIONW {
    pub dwSize: u32,
    pub DeviceType: u32,
    pub szDeviceInternalName: [u16; 128],
    pub DeviceCapabilitiesA: STI_DEV_CAPS,
    pub dwHardwareConfiguration: u32,
    pub pszVendorDescription: ::windows::core::PWSTR,
    pub pszDeviceDescription: ::windows::core::PWSTR,
    pub pszPortName: ::windows::core::PWSTR,
    pub pszPropProvider: ::windows::core::PWSTR,
    pub pszLocalName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for STI_DEVICE_INFORMATIONW {}
impl ::core::clone::Clone for STI_DEVICE_INFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STI_DEVICE_INFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STI_DEVICE_INFORMATIONW")
            .field("dwSize", &self.dwSize)
            .field("DeviceType", &self.DeviceType)
            .field("szDeviceInternalName", &self.szDeviceInternalName)
            .field("DeviceCapabilitiesA", &self.DeviceCapabilitiesA)
            .field("dwHardwareConfiguration", &self.dwHardwareConfiguration)
            .field("pszVendorDescription", &self.pszVendorDescription)
            .field("pszDeviceDescription", &self.pszDeviceDescription)
            .field("pszPortName", &self.pszPortName)
            .field("pszPropProvider", &self.pszPropProvider)
            .field("pszLocalName", &self.pszLocalName)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for STI_DEVICE_INFORMATIONW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STI_DEVICE_INFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STI_DEVICE_INFORMATIONW>()) == 0 }
    }
}
impl ::core::cmp::Eq for STI_DEVICE_INFORMATIONW {}
impl ::core::default::Default for STI_DEVICE_INFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STI_DEVICE_MJ_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const StiDeviceTypeDefault: STI_DEVICE_MJ_TYPE = STI_DEVICE_MJ_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const StiDeviceTypeScanner: STI_DEVICE_MJ_TYPE = STI_DEVICE_MJ_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const StiDeviceTypeDigitalCamera: STI_DEVICE_MJ_TYPE = STI_DEVICE_MJ_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const StiDeviceTypeStreamingVideo: STI_DEVICE_MJ_TYPE = STI_DEVICE_MJ_TYPE(3i32);
impl ::core::marker::Copy for STI_DEVICE_MJ_TYPE {}
impl ::core::clone::Clone for STI_DEVICE_MJ_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STI_DEVICE_MJ_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STI_DEVICE_MJ_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for STI_DEVICE_MJ_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STI_DEVICE_MJ_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct STI_DEVICE_STATUS {
    pub dwSize: u32,
    pub StatusMask: u32,
    pub dwOnlineState: u32,
    pub dwHardwareStatusCode: u32,
    pub dwEventHandlingState: u32,
    pub dwPollingInterval: u32,
}
impl ::core::marker::Copy for STI_DEVICE_STATUS {}
impl ::core::clone::Clone for STI_DEVICE_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STI_DEVICE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STI_DEVICE_STATUS").field("dwSize", &self.dwSize).field("StatusMask", &self.StatusMask).field("dwOnlineState", &self.dwOnlineState).field("dwHardwareStatusCode", &self.dwHardwareStatusCode).field("dwEventHandlingState", &self.dwEventHandlingState).field("dwPollingInterval", &self.dwPollingInterval).finish()
    }
}
unsafe impl ::windows::core::Abi for STI_DEVICE_STATUS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STI_DEVICE_STATUS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STI_DEVICE_STATUS>()) == 0 }
    }
}
impl ::core::cmp::Eq for STI_DEVICE_STATUS {}
impl ::core::default::Default for STI_DEVICE_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_VALUE_DEFAULT_LAUNCHAPP: &'static str = "DefaultLaunchApp";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_VALUE_DEFAULT_LAUNCHAPP_A: &'static str = "DefaultLaunchApp";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_VALUE_DISABLE_NOTIFICATIONS: &'static str = "DisableNotifications";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_VALUE_DISABLE_NOTIFICATIONS_A: &'static str = "DisableNotifications";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_VALUE_ICM_PROFILE: &'static str = "ICMProfile";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_VALUE_ICM_PROFILE_A: &'static str = "ICMProfile";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_VALUE_ISIS_NAME: &'static str = "ISISDriverName";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_VALUE_ISIS_NAME_A: &'static str = "ISISDriverName";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_VALUE_TIMEOUT: &'static str = "PollTimeout";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_VALUE_TIMEOUT_A: &'static str = "PollTimeout";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_VALUE_TWAIN_NAME: &'static str = "TwainDS";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVICE_VALUE_TWAIN_NAME_A: &'static str = "TwainDS";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVSTATUS_EVENTS_STATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DEVSTATUS_ONLINE_STATE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct STI_DEV_CAPS {
    pub dwGeneric: u32,
}
impl ::core::marker::Copy for STI_DEV_CAPS {}
impl ::core::clone::Clone for STI_DEV_CAPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STI_DEV_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STI_DEV_CAPS").field("dwGeneric", &self.dwGeneric).finish()
    }
}
unsafe impl ::windows::core::Abi for STI_DEV_CAPS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STI_DEV_CAPS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STI_DEV_CAPS>()) == 0 }
    }
}
impl ::core::cmp::Eq for STI_DEV_CAPS {}
impl ::core::default::Default for STI_DEV_CAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct STI_DIAG {
    pub dwSize: u32,
    pub dwBasicDiagCode: u32,
    pub dwVendorDiagCode: u32,
    pub dwStatusMask: u32,
    pub sErrorInfo: _ERROR_INFOW,
}
impl ::core::marker::Copy for STI_DIAG {}
impl ::core::clone::Clone for STI_DIAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STI_DIAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STI_DIAG").field("dwSize", &self.dwSize).field("dwBasicDiagCode", &self.dwBasicDiagCode).field("dwVendorDiagCode", &self.dwVendorDiagCode).field("dwStatusMask", &self.dwStatusMask).field("sErrorInfo", &self.sErrorInfo).finish()
    }
}
unsafe impl ::windows::core::Abi for STI_DIAG {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STI_DIAG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STI_DIAG>()) == 0 }
    }
}
impl ::core::cmp::Eq for STI_DIAG {}
impl ::core::default::Default for STI_DIAG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_DIAGCODE_HWPRESENCE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ERROR_NO_ERROR: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_EVENTHANDLING_ENABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_EVENTHANDLING_PENDING: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_EVENTHANDLING_POLLING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_GENCAP_AUTO_PORTSELECT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_GENCAP_GENERATE_ARRIVALEVENT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_GENCAP_NOTIFICATIONS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_GENCAP_POLLING_NEEDED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_GENCAP_SUBSET: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_GENCAP_WIA: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_HW_CONFIG_PARALLEL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_HW_CONFIG_SCSI: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_HW_CONFIG_SERIAL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_HW_CONFIG_UNKNOWN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_HW_CONFIG_USB: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_MAX_INTERNAL_NAME_LENGTH: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_NOTCONNECTED: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_OK: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_BUSY: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_ERROR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_INITIALIZING: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_IO_ACTIVE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_OFFLINE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_OPERATIONAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_PAPER_JAM: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_PAPER_PROBLEM: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_PAUSED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_PENDING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_POWER_SAVE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_TRANSFERRING: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_USER_INTERVENTION: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_ONLINESTATE_WARMING_UP: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_RAW_RESERVED: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_REMOVE_DEVICE_BROADCAST_ACTION: &'static str = "Removal";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_REMOVE_DEVICE_BROADCAST_STRING: &'static str = "STI\\";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_SUBSCRIBE_FLAG_EVENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_SUBSCRIBE_FLAG_WINDOW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_TRACE_ERROR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_TRACE_INFORMATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_TRACE_WARNING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_UNICODE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct STI_USD_CAPS {
    pub dwVersion: u32,
    pub dwGenericCaps: u32,
}
impl ::core::marker::Copy for STI_USD_CAPS {}
impl ::core::clone::Clone for STI_USD_CAPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STI_USD_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STI_USD_CAPS").field("dwVersion", &self.dwVersion).field("dwGenericCaps", &self.dwGenericCaps).finish()
    }
}
unsafe impl ::windows::core::Abi for STI_USD_CAPS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STI_USD_CAPS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STI_USD_CAPS>()) == 0 }
    }
}
impl ::core::cmp::Eq for STI_USD_CAPS {}
impl ::core::default::Default for STI_USD_CAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_USD_GENCAP_NATIVE_PUSHSUPPORT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_VERSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_VERSION_FLAG_MASK: u32 = 4278190080u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_VERSION_FLAG_UNICODE: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_VERSION_MIN_ALLOWED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const STI_VERSION_REAL: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct STI_WIA_DEVICE_INFORMATIONW {
    pub dwSize: u32,
    pub DeviceType: u32,
    pub szDeviceInternalName: [u16; 128],
    pub DeviceCapabilitiesA: STI_DEV_CAPS,
    pub dwHardwareConfiguration: u32,
    pub pszVendorDescription: ::windows::core::PWSTR,
    pub pszDeviceDescription: ::windows::core::PWSTR,
    pub pszPortName: ::windows::core::PWSTR,
    pub pszPropProvider: ::windows::core::PWSTR,
    pub pszLocalName: ::windows::core::PWSTR,
    pub pszUiDll: ::windows::core::PWSTR,
    pub pszServer: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for STI_WIA_DEVICE_INFORMATIONW {}
impl ::core::clone::Clone for STI_WIA_DEVICE_INFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STI_WIA_DEVICE_INFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STI_WIA_DEVICE_INFORMATIONW")
            .field("dwSize", &self.dwSize)
            .field("DeviceType", &self.DeviceType)
            .field("szDeviceInternalName", &self.szDeviceInternalName)
            .field("DeviceCapabilitiesA", &self.DeviceCapabilitiesA)
            .field("dwHardwareConfiguration", &self.dwHardwareConfiguration)
            .field("pszVendorDescription", &self.pszVendorDescription)
            .field("pszDeviceDescription", &self.pszDeviceDescription)
            .field("pszPortName", &self.pszPortName)
            .field("pszPropProvider", &self.pszPropProvider)
            .field("pszLocalName", &self.pszLocalName)
            .field("pszUiDll", &self.pszUiDll)
            .field("pszServer", &self.pszServer)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for STI_WIA_DEVICE_INFORMATIONW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STI_WIA_DEVICE_INFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STI_WIA_DEVICE_INFORMATIONW>()) == 0 }
    }
}
impl ::core::cmp::Eq for STI_WIA_DEVICE_INFORMATIONW {}
impl ::core::default::Default for STI_WIA_DEVICE_INFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const SUPPORTS_MSCPLUS_STR: &'static str = "SupportsMSCPlus";
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const SUPPORTS_MSCPLUS_VAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[inline]
pub unsafe fn SendToFaxRecipient<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(sndmode: SendToMode, lpfilename: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SendToFaxRecipient(sndmode: SendToMode, lpfilename: ::windows::core::PCWSTR) -> u32;
        }
        ::core::mem::transmute(SendToFaxRecipient(::core::mem::transmute(sndmode), lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SendToMode(pub i32);
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const SEND_TO_FAX_RECIPIENT_ATTACHMENT: SendToMode = SendToMode(0i32);
impl ::core::marker::Copy for SendToMode {}
impl ::core::clone::Clone for SendToMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SendToMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SendToMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for SendToMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SendToMode").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StiCreateInstanceW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param3: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(hinst: Param0, dwver: u32, ppsti: *mut ::core::option::Option<IStillImageW>, punkouter: Param3) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StiCreateInstanceW(hinst: super::super::Foundation::HINSTANCE, dwver: u32, ppsti: *mut ::windows::core::RawPtr, punkouter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        StiCreateInstanceW(hinst.into_param().abi(), ::core::mem::transmute(dwver), ::core::mem::transmute(ppsti), punkouter.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const WIA_INCOMPAT_XP: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub struct _ERROR_INFOW {
    pub dwSize: u32,
    pub dwGenericError: u32,
    pub dwVendorError: u32,
    pub szExtendedErrorText: [u16; 255],
}
impl ::core::marker::Copy for _ERROR_INFOW {}
impl ::core::clone::Clone for _ERROR_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for _ERROR_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_ERROR_INFOW").field("dwSize", &self.dwSize).field("dwGenericError", &self.dwGenericError).field("dwVendorError", &self.dwVendorError).field("szExtendedErrorText", &self.szExtendedErrorText).finish()
    }
}
unsafe impl ::windows::core::Abi for _ERROR_INFOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for _ERROR_INFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<_ERROR_INFOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for _ERROR_INFOW {}
impl ::core::default::Default for _ERROR_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct _IFaxAccountNotify(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl _IFaxAccountNotify {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn OnIncomingJobAdded<'a, Param0: ::windows::core::IntoParam<'a, IFaxAccount>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pfaxaccount: Param0, bstrjobid: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnIncomingJobAdded)(::core::mem::transmute_copy(self), pfaxaccount.into_param().abi(), bstrjobid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn OnIncomingJobRemoved<'a, Param0: ::windows::core::IntoParam<'a, IFaxAccount>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pfaxaccount: Param0, bstrjobid: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnIncomingJobRemoved)(::core::mem::transmute_copy(self), pfaxaccount.into_param().abi(), bstrjobid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn OnIncomingJobChanged<'a, Param0: ::windows::core::IntoParam<'a, IFaxAccount>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, IFaxJobStatus>>(&self, pfaxaccount: Param0, bstrjobid: Param1, pjobstatus: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnIncomingJobChanged)(::core::mem::transmute_copy(self), pfaxaccount.into_param().abi(), bstrjobid.into_param().abi(), pjobstatus.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn OnOutgoingJobAdded<'a, Param0: ::windows::core::IntoParam<'a, IFaxAccount>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pfaxaccount: Param0, bstrjobid: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnOutgoingJobAdded)(::core::mem::transmute_copy(self), pfaxaccount.into_param().abi(), bstrjobid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn OnOutgoingJobRemoved<'a, Param0: ::windows::core::IntoParam<'a, IFaxAccount>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pfaxaccount: Param0, bstrjobid: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnOutgoingJobRemoved)(::core::mem::transmute_copy(self), pfaxaccount.into_param().abi(), bstrjobid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn OnOutgoingJobChanged<'a, Param0: ::windows::core::IntoParam<'a, IFaxAccount>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, IFaxJobStatus>>(&self, pfaxaccount: Param0, bstrjobid: Param1, pjobstatus: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnOutgoingJobChanged)(::core::mem::transmute_copy(self), pfaxaccount.into_param().abi(), bstrjobid.into_param().abi(), pjobstatus.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn OnIncomingMessageAdded<'a, Param0: ::windows::core::IntoParam<'a, IFaxAccount>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pfaxaccount: Param0, bstrmessageid: Param1, faddedtoreceivefolder: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnIncomingMessageAdded)(::core::mem::transmute_copy(self), pfaxaccount.into_param().abi(), bstrmessageid.into_param().abi(), ::core::mem::transmute(faddedtoreceivefolder)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn OnIncomingMessageRemoved<'a, Param0: ::windows::core::IntoParam<'a, IFaxAccount>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pfaxaccount: Param0, bstrmessageid: Param1, fremovedfromreceivefolder: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnIncomingMessageRemoved)(::core::mem::transmute_copy(self), pfaxaccount.into_param().abi(), bstrmessageid.into_param().abi(), ::core::mem::transmute(fremovedfromreceivefolder)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn OnOutgoingMessageAdded<'a, Param0: ::windows::core::IntoParam<'a, IFaxAccount>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pfaxaccount: Param0, bstrmessageid: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnOutgoingMessageAdded)(::core::mem::transmute_copy(self), pfaxaccount.into_param().abi(), bstrmessageid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn OnOutgoingMessageRemoved<'a, Param0: ::windows::core::IntoParam<'a, IFaxAccount>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pfaxaccount: Param0, bstrmessageid: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnOutgoingMessageRemoved)(::core::mem::transmute_copy(self), pfaxaccount.into_param().abi(), bstrmessageid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnServerShutDown<'a, Param0: ::windows::core::IntoParam<'a, IFaxServer2>>(&self, pfaxserver: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnServerShutDown)(::core::mem::transmute_copy(self), pfaxserver.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<_IFaxAccountNotify> for ::windows::core::IUnknown {
    fn from(value: _IFaxAccountNotify) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&_IFaxAccountNotify> for ::windows::core::IUnknown {
    fn from(value: &_IFaxAccountNotify) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for _IFaxAccountNotify {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a _IFaxAccountNotify {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<_IFaxAccountNotify> for super::super::System::Com::IDispatch {
    fn from(value: _IFaxAccountNotify) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&_IFaxAccountNotify> for super::super::System::Com::IDispatch {
    fn from(value: &_IFaxAccountNotify) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for _IFaxAccountNotify {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a _IFaxAccountNotify {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for _IFaxAccountNotify {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for _IFaxAccountNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for _IFaxAccountNotify {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for _IFaxAccountNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_IFaxAccountNotify").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for _IFaxAccountNotify {
    type Vtable = _IFaxAccountNotify_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9b3bc81_ac1b_46f3_b39d_0adc30e1b788);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct _IFaxAccountNotify_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnIncomingJobAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxaccount: ::windows::core::RawPtr, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnIncomingJobAdded: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnIncomingJobRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxaccount: ::windows::core::RawPtr, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnIncomingJobRemoved: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnIncomingJobChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxaccount: ::windows::core::RawPtr, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pjobstatus: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnIncomingJobChanged: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnOutgoingJobAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxaccount: ::windows::core::RawPtr, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnOutgoingJobAdded: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnOutgoingJobRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxaccount: ::windows::core::RawPtr, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnOutgoingJobRemoved: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnOutgoingJobChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxaccount: ::windows::core::RawPtr, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pjobstatus: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnOutgoingJobChanged: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnIncomingMessageAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxaccount: ::windows::core::RawPtr, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, faddedtoreceivefolder: i16) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnIncomingMessageAdded: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnIncomingMessageRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxaccount: ::windows::core::RawPtr, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fremovedfromreceivefolder: i16) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnIncomingMessageRemoved: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnOutgoingMessageAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxaccount: ::windows::core::RawPtr, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnOutgoingMessageAdded: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnOutgoingMessageRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxaccount: ::windows::core::RawPtr, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnOutgoingMessageRemoved: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnServerShutDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnServerShutDown: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct _IFaxServerNotify2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl _IFaxServerNotify2 {
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn OnIncomingJobAdded<'a, Param0: ::windows::core::IntoParam<'a, IFaxServer2>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pfaxserver: Param0, bstrjobid: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnIncomingJobAdded)(::core::mem::transmute_copy(self), pfaxserver.into_param().abi(), bstrjobid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn OnIncomingJobRemoved<'a, Param0: ::windows::core::IntoParam<'a, IFaxServer2>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pfaxserver: Param0, bstrjobid: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnIncomingJobRemoved)(::core::mem::transmute_copy(self), pfaxserver.into_param().abi(), bstrjobid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn OnIncomingJobChanged<'a, Param0: ::windows::core::IntoParam<'a, IFaxServer2>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, IFaxJobStatus>>(&self, pfaxserver: Param0, bstrjobid: Param1, pjobstatus: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnIncomingJobChanged)(::core::mem::transmute_copy(self), pfaxserver.into_param().abi(), bstrjobid.into_param().abi(), pjobstatus.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn OnOutgoingJobAdded<'a, Param0: ::windows::core::IntoParam<'a, IFaxServer2>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pfaxserver: Param0, bstrjobid: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnOutgoingJobAdded)(::core::mem::transmute_copy(self), pfaxserver.into_param().abi(), bstrjobid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn OnOutgoingJobRemoved<'a, Param0: ::windows::core::IntoParam<'a, IFaxServer2>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pfaxserver: Param0, bstrjobid: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnOutgoingJobRemoved)(::core::mem::transmute_copy(self), pfaxserver.into_param().abi(), bstrjobid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn OnOutgoingJobChanged<'a, Param0: ::windows::core::IntoParam<'a, IFaxServer2>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, IFaxJobStatus>>(&self, pfaxserver: Param0, bstrjobid: Param1, pjobstatus: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnOutgoingJobChanged)(::core::mem::transmute_copy(self), pfaxserver.into_param().abi(), bstrjobid.into_param().abi(), pjobstatus.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn OnIncomingMessageAdded<'a, Param0: ::windows::core::IntoParam<'a, IFaxServer2>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pfaxserver: Param0, bstrmessageid: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnIncomingMessageAdded)(::core::mem::transmute_copy(self), pfaxserver.into_param().abi(), bstrmessageid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn OnIncomingMessageRemoved<'a, Param0: ::windows::core::IntoParam<'a, IFaxServer2>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pfaxserver: Param0, bstrmessageid: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnIncomingMessageRemoved)(::core::mem::transmute_copy(self), pfaxserver.into_param().abi(), bstrmessageid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn OnOutgoingMessageAdded<'a, Param0: ::windows::core::IntoParam<'a, IFaxServer2>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pfaxserver: Param0, bstrmessageid: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnOutgoingMessageAdded)(::core::mem::transmute_copy(self), pfaxserver.into_param().abi(), bstrmessageid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn OnOutgoingMessageRemoved<'a, Param0: ::windows::core::IntoParam<'a, IFaxServer2>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pfaxserver: Param0, bstrmessageid: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnOutgoingMessageRemoved)(::core::mem::transmute_copy(self), pfaxserver.into_param().abi(), bstrmessageid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnReceiptOptionsChange<'a, Param0: ::windows::core::IntoParam<'a, IFaxServer2>>(&self, pfaxserver: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnReceiptOptionsChange)(::core::mem::transmute_copy(self), pfaxserver.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnActivityLoggingConfigChange<'a, Param0: ::windows::core::IntoParam<'a, IFaxServer2>>(&self, pfaxserver: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnActivityLoggingConfigChange)(::core::mem::transmute_copy(self), pfaxserver.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnSecurityConfigChange<'a, Param0: ::windows::core::IntoParam<'a, IFaxServer2>>(&self, pfaxserver: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnSecurityConfigChange)(::core::mem::transmute_copy(self), pfaxserver.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnEventLoggingConfigChange<'a, Param0: ::windows::core::IntoParam<'a, IFaxServer2>>(&self, pfaxserver: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnEventLoggingConfigChange)(::core::mem::transmute_copy(self), pfaxserver.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnOutgoingQueueConfigChange<'a, Param0: ::windows::core::IntoParam<'a, IFaxServer2>>(&self, pfaxserver: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnOutgoingQueueConfigChange)(::core::mem::transmute_copy(self), pfaxserver.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnOutgoingArchiveConfigChange<'a, Param0: ::windows::core::IntoParam<'a, IFaxServer2>>(&self, pfaxserver: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnOutgoingArchiveConfigChange)(::core::mem::transmute_copy(self), pfaxserver.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnIncomingArchiveConfigChange<'a, Param0: ::windows::core::IntoParam<'a, IFaxServer2>>(&self, pfaxserver: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnIncomingArchiveConfigChange)(::core::mem::transmute_copy(self), pfaxserver.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnDevicesConfigChange<'a, Param0: ::windows::core::IntoParam<'a, IFaxServer2>>(&self, pfaxserver: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnDevicesConfigChange)(::core::mem::transmute_copy(self), pfaxserver.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnOutboundRoutingGroupsConfigChange<'a, Param0: ::windows::core::IntoParam<'a, IFaxServer2>>(&self, pfaxserver: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnOutboundRoutingGroupsConfigChange)(::core::mem::transmute_copy(self), pfaxserver.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnOutboundRoutingRulesConfigChange<'a, Param0: ::windows::core::IntoParam<'a, IFaxServer2>>(&self, pfaxserver: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnOutboundRoutingRulesConfigChange)(::core::mem::transmute_copy(self), pfaxserver.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnServerActivityChange<'a, Param0: ::windows::core::IntoParam<'a, IFaxServer2>>(&self, pfaxserver: Param0, lincomingmessages: i32, lroutingmessages: i32, loutgoingmessages: i32, lqueuedmessages: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnServerActivityChange)(::core::mem::transmute_copy(self), pfaxserver.into_param().abi(), ::core::mem::transmute(lincomingmessages), ::core::mem::transmute(lroutingmessages), ::core::mem::transmute(loutgoingmessages), ::core::mem::transmute(lqueuedmessages)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnQueuesStatusChange<'a, Param0: ::windows::core::IntoParam<'a, IFaxServer2>>(&self, pfaxserver: Param0, boutgoingqueueblocked: i16, boutgoingqueuepaused: i16, bincomingqueueblocked: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnQueuesStatusChange)(::core::mem::transmute_copy(self), pfaxserver.into_param().abi(), ::core::mem::transmute(boutgoingqueueblocked), ::core::mem::transmute(boutgoingqueuepaused), ::core::mem::transmute(bincomingqueueblocked)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn OnNewCall<'a, Param0: ::windows::core::IntoParam<'a, IFaxServer2>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pfaxserver: Param0, lcallid: i32, ldeviceid: i32, bstrcallerid: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnNewCall)(::core::mem::transmute_copy(self), pfaxserver.into_param().abi(), ::core::mem::transmute(lcallid), ::core::mem::transmute(ldeviceid), bstrcallerid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnServerShutDown<'a, Param0: ::windows::core::IntoParam<'a, IFaxServer2>>(&self, pfaxserver: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnServerShutDown)(::core::mem::transmute_copy(self), pfaxserver.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnDeviceStatusChange<'a, Param0: ::windows::core::IntoParam<'a, IFaxServer2>>(&self, pfaxserver: Param0, ldeviceid: i32, bpoweredoff: i16, bsending: i16, breceiving: i16, bringing: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnDeviceStatusChange)(::core::mem::transmute_copy(self), pfaxserver.into_param().abi(), ::core::mem::transmute(ldeviceid), ::core::mem::transmute(bpoweredoff), ::core::mem::transmute(bsending), ::core::mem::transmute(breceiving), ::core::mem::transmute(bringing)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnGeneralServerConfigChanged<'a, Param0: ::windows::core::IntoParam<'a, IFaxServer2>>(&self, pfaxserver: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnGeneralServerConfigChanged)(::core::mem::transmute_copy(self), pfaxserver.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<_IFaxServerNotify2> for ::windows::core::IUnknown {
    fn from(value: _IFaxServerNotify2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&_IFaxServerNotify2> for ::windows::core::IUnknown {
    fn from(value: &_IFaxServerNotify2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for _IFaxServerNotify2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a _IFaxServerNotify2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<_IFaxServerNotify2> for super::super::System::Com::IDispatch {
    fn from(value: _IFaxServerNotify2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&_IFaxServerNotify2> for super::super::System::Com::IDispatch {
    fn from(value: &_IFaxServerNotify2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for _IFaxServerNotify2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a _IFaxServerNotify2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for _IFaxServerNotify2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for _IFaxServerNotify2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for _IFaxServerNotify2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for _IFaxServerNotify2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_IFaxServerNotify2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for _IFaxServerNotify2 {
    type Vtable = _IFaxServerNotify2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec9c69b9_5fe7_4805_9467_82fcd96af903);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct _IFaxServerNotify2_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnIncomingJobAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnIncomingJobAdded: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnIncomingJobRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnIncomingJobRemoved: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnIncomingJobChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pjobstatus: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnIncomingJobChanged: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnOutgoingJobAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnOutgoingJobAdded: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnOutgoingJobRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnOutgoingJobRemoved: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnOutgoingJobChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pjobstatus: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnOutgoingJobChanged: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnIncomingMessageAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnIncomingMessageAdded: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnIncomingMessageRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnIncomingMessageRemoved: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnOutgoingMessageAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnOutgoingMessageAdded: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnOutgoingMessageRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnOutgoingMessageRemoved: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnReceiptOptionsChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnReceiptOptionsChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnActivityLoggingConfigChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnActivityLoggingConfigChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnSecurityConfigChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnSecurityConfigChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnEventLoggingConfigChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnEventLoggingConfigChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnOutgoingQueueConfigChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnOutgoingQueueConfigChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnOutgoingArchiveConfigChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnOutgoingArchiveConfigChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnIncomingArchiveConfigChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnIncomingArchiveConfigChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnDevicesConfigChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnDevicesConfigChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnOutboundRoutingGroupsConfigChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnOutboundRoutingGroupsConfigChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnOutboundRoutingRulesConfigChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnOutboundRoutingRulesConfigChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnServerActivityChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, lincomingmessages: i32, lroutingmessages: i32, loutgoingmessages: i32, lqueuedmessages: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnServerActivityChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnQueuesStatusChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, boutgoingqueueblocked: i16, boutgoingqueuepaused: i16, bincomingqueueblocked: i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnQueuesStatusChange: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnNewCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, lcallid: i32, ldeviceid: i32, bstrcallerid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnNewCall: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnServerShutDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnServerShutDown: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnDeviceStatusChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, ldeviceid: i32, bpoweredoff: i16, bsending: i16, breceiving: i16, bringing: i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnDeviceStatusChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnGeneralServerConfigChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnGeneralServerConfigChanged: usize,
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const lDEFAULT_PREFETCH_SIZE: i32 = 100i32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const prv_DEFAULT_PREFETCH_SIZE: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Devices_Fax\"`*"]
pub const wcharREASSIGN_RECIPIENTS_DELIMITER: u16 = 59u16;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
