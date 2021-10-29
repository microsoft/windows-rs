#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const CLSID_Sti: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3005479136, 11880, 4560, [144, 234, 0, 170, 0, 96, 248, 108]);
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CanSendToFaxRecipient() -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CanSendToFaxRecipient() -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CanSendToFaxRecipient())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_PropertiesSystem")]
pub const DEVPKEY_WIA_DeviceType: super::super::System::PropertiesSystem::PROPERTYKEY = super::super::System::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1809653702, 33039, 4560, [190, 199, 8, 0, 43, 226, 9, 47]),
    pid: 2u32,
};
#[cfg(feature = "Win32_System_PropertiesSystem")]
pub const DEVPKEY_WIA_USDClassId: super::super::System::PropertiesSystem::PROPERTYKEY = super::super::System::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1809653702, 33039, 4560, [190, 199, 8, 0, 43, 226, 9, 47]),
    pid: 3u32,
};
pub const FAXDEVRECEIVE_SIZE: u32 = 4096u32;
pub const FAXDEVREPORTSTATUS_SIZE: u32 = 4096u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FAXROUTE_ENABLE(pub i32);
pub const QUERY_STATUS: FAXROUTE_ENABLE = FAXROUTE_ENABLE(-1i32);
pub const STATUS_DISABLE: FAXROUTE_ENABLE = FAXROUTE_ENABLE(0i32);
pub const STATUS_ENABLE: FAXROUTE_ENABLE = FAXROUTE_ENABLE(1i32);
impl ::std::convert::From<i32> for FAXROUTE_ENABLE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FAXROUTE_ENABLE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FAX_ACCESS_RIGHTS_ENUM(pub i32);
pub const farSUBMIT_LOW: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(1i32);
pub const farSUBMIT_NORMAL: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(2i32);
pub const farSUBMIT_HIGH: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(4i32);
pub const farQUERY_JOBS: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(8i32);
pub const farMANAGE_JOBS: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(16i32);
pub const farQUERY_CONFIG: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(32i32);
pub const farMANAGE_CONFIG: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(64i32);
pub const farQUERY_IN_ARCHIVE: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(128i32);
pub const farMANAGE_IN_ARCHIVE: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(256i32);
pub const farQUERY_OUT_ARCHIVE: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(512i32);
pub const farMANAGE_OUT_ARCHIVE: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(1024i32);
impl ::std::convert::From<i32> for FAX_ACCESS_RIGHTS_ENUM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FAX_ACCESS_RIGHTS_ENUM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FAX_ACCESS_RIGHTS_ENUM_2(pub i32);
pub const far2SUBMIT_LOW: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(1i32);
pub const far2SUBMIT_NORMAL: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(2i32);
pub const far2SUBMIT_HIGH: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(4i32);
pub const far2QUERY_OUT_JOBS: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(8i32);
pub const far2MANAGE_OUT_JOBS: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(16i32);
pub const far2QUERY_CONFIG: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(32i32);
pub const far2MANAGE_CONFIG: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(64i32);
pub const far2QUERY_ARCHIVES: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(128i32);
pub const far2MANAGE_ARCHIVES: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(256i32);
pub const far2MANAGE_RECEIVE_FOLDER: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(512i32);
impl ::std::convert::From<i32> for FAX_ACCESS_RIGHTS_ENUM_2 {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FAX_ACCESS_RIGHTS_ENUM_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FAX_ACCOUNT_EVENTS_TYPE_ENUM(pub i32);
pub const faetNONE: FAX_ACCOUNT_EVENTS_TYPE_ENUM = FAX_ACCOUNT_EVENTS_TYPE_ENUM(0i32);
pub const faetIN_QUEUE: FAX_ACCOUNT_EVENTS_TYPE_ENUM = FAX_ACCOUNT_EVENTS_TYPE_ENUM(1i32);
pub const faetOUT_QUEUE: FAX_ACCOUNT_EVENTS_TYPE_ENUM = FAX_ACCOUNT_EVENTS_TYPE_ENUM(2i32);
pub const faetIN_ARCHIVE: FAX_ACCOUNT_EVENTS_TYPE_ENUM = FAX_ACCOUNT_EVENTS_TYPE_ENUM(4i32);
pub const faetOUT_ARCHIVE: FAX_ACCOUNT_EVENTS_TYPE_ENUM = FAX_ACCOUNT_EVENTS_TYPE_ENUM(8i32);
pub const faetFXSSVC_ENDED: FAX_ACCOUNT_EVENTS_TYPE_ENUM = FAX_ACCOUNT_EVENTS_TYPE_ENUM(16i32);
impl ::std::convert::From<i32> for FAX_ACCOUNT_EVENTS_TYPE_ENUM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FAX_ACCOUNT_EVENTS_TYPE_ENUM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
    pub ArchiveDirectory: super::super::Foundation::PSTR,
    pub Reserved: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl FAX_CONFIGURATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FAX_CONFIGURATIONA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FAX_CONFIGURATIONA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FAX_CONFIGURATIONA")
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
impl ::std::cmp::PartialEq for FAX_CONFIGURATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct
            && self.Retries == other.Retries
            && self.RetryDelay == other.RetryDelay
            && self.DirtyDays == other.DirtyDays
            && self.Branding == other.Branding
            && self.UseDeviceTsid == other.UseDeviceTsid
            && self.ServerCp == other.ServerCp
            && self.PauseServerQueue == other.PauseServerQueue
            && self.StartCheapTime == other.StartCheapTime
            && self.StopCheapTime == other.StopCheapTime
            && self.ArchiveOutgoingFaxes == other.ArchiveOutgoingFaxes
            && self.ArchiveDirectory == other.ArchiveDirectory
            && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FAX_CONFIGURATIONA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FAX_CONFIGURATIONA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
    pub ArchiveDirectory: super::super::Foundation::PWSTR,
    pub Reserved: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl FAX_CONFIGURATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FAX_CONFIGURATIONW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FAX_CONFIGURATIONW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FAX_CONFIGURATIONW")
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
impl ::std::cmp::PartialEq for FAX_CONFIGURATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct
            && self.Retries == other.Retries
            && self.RetryDelay == other.RetryDelay
            && self.DirtyDays == other.DirtyDays
            && self.Branding == other.Branding
            && self.UseDeviceTsid == other.UseDeviceTsid
            && self.ServerCp == other.ServerCp
            && self.PauseServerQueue == other.PauseServerQueue
            && self.StartCheapTime == other.StartCheapTime
            && self.StopCheapTime == other.StopCheapTime
            && self.ArchiveOutgoingFaxes == other.ArchiveOutgoingFaxes
            && self.ArchiveDirectory == other.ArchiveDirectory
            && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FAX_CONFIGURATIONW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FAX_CONFIGURATIONW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FAX_CONFIG_QUERY: u32 = 4u32;
pub const FAX_CONFIG_SET: u32 = 8u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct FAX_CONTEXT_INFOA {
    pub SizeOfStruct: u32,
    pub hDC: super::super::Graphics::Gdi::HDC,
    pub ServerName: [super::super::Foundation::CHAR; 16],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl FAX_CONTEXT_INFOA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for FAX_CONTEXT_INFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for FAX_CONTEXT_INFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FAX_CONTEXT_INFOA").field("SizeOfStruct", &self.SizeOfStruct).field("hDC", &self.hDC).field("ServerName", &self.ServerName).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for FAX_CONTEXT_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.hDC == other.hDC && self.ServerName == other.ServerName
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for FAX_CONTEXT_INFOA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for FAX_CONTEXT_INFOA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct FAX_CONTEXT_INFOW {
    pub SizeOfStruct: u32,
    pub hDC: super::super::Graphics::Gdi::HDC,
    pub ServerName: [u16; 16],
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl FAX_CONTEXT_INFOW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::default::Default for FAX_CONTEXT_INFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::fmt::Debug for FAX_CONTEXT_INFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FAX_CONTEXT_INFOW").field("SizeOfStruct", &self.SizeOfStruct).field("hDC", &self.hDC).field("ServerName", &self.ServerName).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::cmp::PartialEq for FAX_CONTEXT_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.hDC == other.hDC && self.ServerName == other.ServerName
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::cmp::Eq for FAX_CONTEXT_INFOW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows::runtime::Abi for FAX_CONTEXT_INFOW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_COVERPAGE_INFOA {
    pub SizeOfStruct: u32,
    pub CoverPageName: super::super::Foundation::PSTR,
    pub UseServerCoverPage: super::super::Foundation::BOOL,
    pub RecName: super::super::Foundation::PSTR,
    pub RecFaxNumber: super::super::Foundation::PSTR,
    pub RecCompany: super::super::Foundation::PSTR,
    pub RecStreetAddress: super::super::Foundation::PSTR,
    pub RecCity: super::super::Foundation::PSTR,
    pub RecState: super::super::Foundation::PSTR,
    pub RecZip: super::super::Foundation::PSTR,
    pub RecCountry: super::super::Foundation::PSTR,
    pub RecTitle: super::super::Foundation::PSTR,
    pub RecDepartment: super::super::Foundation::PSTR,
    pub RecOfficeLocation: super::super::Foundation::PSTR,
    pub RecHomePhone: super::super::Foundation::PSTR,
    pub RecOfficePhone: super::super::Foundation::PSTR,
    pub SdrName: super::super::Foundation::PSTR,
    pub SdrFaxNumber: super::super::Foundation::PSTR,
    pub SdrCompany: super::super::Foundation::PSTR,
    pub SdrAddress: super::super::Foundation::PSTR,
    pub SdrTitle: super::super::Foundation::PSTR,
    pub SdrDepartment: super::super::Foundation::PSTR,
    pub SdrOfficeLocation: super::super::Foundation::PSTR,
    pub SdrHomePhone: super::super::Foundation::PSTR,
    pub SdrOfficePhone: super::super::Foundation::PSTR,
    pub Note: super::super::Foundation::PSTR,
    pub Subject: super::super::Foundation::PSTR,
    pub TimeSent: super::super::Foundation::SYSTEMTIME,
    pub PageCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl FAX_COVERPAGE_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FAX_COVERPAGE_INFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FAX_COVERPAGE_INFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FAX_COVERPAGE_INFOA")
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
impl ::std::cmp::PartialEq for FAX_COVERPAGE_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct
            && self.CoverPageName == other.CoverPageName
            && self.UseServerCoverPage == other.UseServerCoverPage
            && self.RecName == other.RecName
            && self.RecFaxNumber == other.RecFaxNumber
            && self.RecCompany == other.RecCompany
            && self.RecStreetAddress == other.RecStreetAddress
            && self.RecCity == other.RecCity
            && self.RecState == other.RecState
            && self.RecZip == other.RecZip
            && self.RecCountry == other.RecCountry
            && self.RecTitle == other.RecTitle
            && self.RecDepartment == other.RecDepartment
            && self.RecOfficeLocation == other.RecOfficeLocation
            && self.RecHomePhone == other.RecHomePhone
            && self.RecOfficePhone == other.RecOfficePhone
            && self.SdrName == other.SdrName
            && self.SdrFaxNumber == other.SdrFaxNumber
            && self.SdrCompany == other.SdrCompany
            && self.SdrAddress == other.SdrAddress
            && self.SdrTitle == other.SdrTitle
            && self.SdrDepartment == other.SdrDepartment
            && self.SdrOfficeLocation == other.SdrOfficeLocation
            && self.SdrHomePhone == other.SdrHomePhone
            && self.SdrOfficePhone == other.SdrOfficePhone
            && self.Note == other.Note
            && self.Subject == other.Subject
            && self.TimeSent == other.TimeSent
            && self.PageCount == other.PageCount
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FAX_COVERPAGE_INFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FAX_COVERPAGE_INFOA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_COVERPAGE_INFOW {
    pub SizeOfStruct: u32,
    pub CoverPageName: super::super::Foundation::PWSTR,
    pub UseServerCoverPage: super::super::Foundation::BOOL,
    pub RecName: super::super::Foundation::PWSTR,
    pub RecFaxNumber: super::super::Foundation::PWSTR,
    pub RecCompany: super::super::Foundation::PWSTR,
    pub RecStreetAddress: super::super::Foundation::PWSTR,
    pub RecCity: super::super::Foundation::PWSTR,
    pub RecState: super::super::Foundation::PWSTR,
    pub RecZip: super::super::Foundation::PWSTR,
    pub RecCountry: super::super::Foundation::PWSTR,
    pub RecTitle: super::super::Foundation::PWSTR,
    pub RecDepartment: super::super::Foundation::PWSTR,
    pub RecOfficeLocation: super::super::Foundation::PWSTR,
    pub RecHomePhone: super::super::Foundation::PWSTR,
    pub RecOfficePhone: super::super::Foundation::PWSTR,
    pub SdrName: super::super::Foundation::PWSTR,
    pub SdrFaxNumber: super::super::Foundation::PWSTR,
    pub SdrCompany: super::super::Foundation::PWSTR,
    pub SdrAddress: super::super::Foundation::PWSTR,
    pub SdrTitle: super::super::Foundation::PWSTR,
    pub SdrDepartment: super::super::Foundation::PWSTR,
    pub SdrOfficeLocation: super::super::Foundation::PWSTR,
    pub SdrHomePhone: super::super::Foundation::PWSTR,
    pub SdrOfficePhone: super::super::Foundation::PWSTR,
    pub Note: super::super::Foundation::PWSTR,
    pub Subject: super::super::Foundation::PWSTR,
    pub TimeSent: super::super::Foundation::SYSTEMTIME,
    pub PageCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl FAX_COVERPAGE_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FAX_COVERPAGE_INFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FAX_COVERPAGE_INFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FAX_COVERPAGE_INFOW")
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
impl ::std::cmp::PartialEq for FAX_COVERPAGE_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct
            && self.CoverPageName == other.CoverPageName
            && self.UseServerCoverPage == other.UseServerCoverPage
            && self.RecName == other.RecName
            && self.RecFaxNumber == other.RecFaxNumber
            && self.RecCompany == other.RecCompany
            && self.RecStreetAddress == other.RecStreetAddress
            && self.RecCity == other.RecCity
            && self.RecState == other.RecState
            && self.RecZip == other.RecZip
            && self.RecCountry == other.RecCountry
            && self.RecTitle == other.RecTitle
            && self.RecDepartment == other.RecDepartment
            && self.RecOfficeLocation == other.RecOfficeLocation
            && self.RecHomePhone == other.RecHomePhone
            && self.RecOfficePhone == other.RecOfficePhone
            && self.SdrName == other.SdrName
            && self.SdrFaxNumber == other.SdrFaxNumber
            && self.SdrCompany == other.SdrCompany
            && self.SdrAddress == other.SdrAddress
            && self.SdrTitle == other.SdrTitle
            && self.SdrDepartment == other.SdrDepartment
            && self.SdrOfficeLocation == other.SdrOfficeLocation
            && self.SdrHomePhone == other.SdrHomePhone
            && self.SdrOfficePhone == other.SdrOfficePhone
            && self.Note == other.Note
            && self.Subject == other.Subject
            && self.TimeSent == other.TimeSent
            && self.PageCount == other.PageCount
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FAX_COVERPAGE_INFOW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FAX_COVERPAGE_INFOW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FAX_COVERPAGE_TYPE_ENUM(pub i32);
pub const fcptNONE: FAX_COVERPAGE_TYPE_ENUM = FAX_COVERPAGE_TYPE_ENUM(0i32);
pub const fcptLOCAL: FAX_COVERPAGE_TYPE_ENUM = FAX_COVERPAGE_TYPE_ENUM(1i32);
pub const fcptSERVER: FAX_COVERPAGE_TYPE_ENUM = FAX_COVERPAGE_TYPE_ENUM(2i32);
impl ::std::convert::From<i32> for FAX_COVERPAGE_TYPE_ENUM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FAX_COVERPAGE_TYPE_ENUM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FAX_DEVICE_RECEIVE_MODE_ENUM(pub i32);
pub const fdrmNO_ANSWER: FAX_DEVICE_RECEIVE_MODE_ENUM = FAX_DEVICE_RECEIVE_MODE_ENUM(0i32);
pub const fdrmAUTO_ANSWER: FAX_DEVICE_RECEIVE_MODE_ENUM = FAX_DEVICE_RECEIVE_MODE_ENUM(1i32);
pub const fdrmMANUAL_ANSWER: FAX_DEVICE_RECEIVE_MODE_ENUM = FAX_DEVICE_RECEIVE_MODE_ENUM(2i32);
impl ::std::convert::From<i32> for FAX_DEVICE_RECEIVE_MODE_ENUM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FAX_DEVICE_RECEIVE_MODE_ENUM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_DEVICE_STATUSA {
    pub SizeOfStruct: u32,
    pub CallerId: super::super::Foundation::PSTR,
    pub Csid: super::super::Foundation::PSTR,
    pub CurrentPage: u32,
    pub DeviceId: u32,
    pub DeviceName: super::super::Foundation::PSTR,
    pub DocumentName: super::super::Foundation::PSTR,
    pub JobType: u32,
    pub PhoneNumber: super::super::Foundation::PSTR,
    pub RoutingString: super::super::Foundation::PSTR,
    pub SenderName: super::super::Foundation::PSTR,
    pub RecipientName: super::super::Foundation::PSTR,
    pub Size: u32,
    pub StartTime: super::super::Foundation::FILETIME,
    pub Status: u32,
    pub StatusString: super::super::Foundation::PSTR,
    pub SubmittedTime: super::super::Foundation::FILETIME,
    pub TotalPages: u32,
    pub Tsid: super::super::Foundation::PSTR,
    pub UserName: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl FAX_DEVICE_STATUSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FAX_DEVICE_STATUSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FAX_DEVICE_STATUSA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FAX_DEVICE_STATUSA")
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
impl ::std::cmp::PartialEq for FAX_DEVICE_STATUSA {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct
            && self.CallerId == other.CallerId
            && self.Csid == other.Csid
            && self.CurrentPage == other.CurrentPage
            && self.DeviceId == other.DeviceId
            && self.DeviceName == other.DeviceName
            && self.DocumentName == other.DocumentName
            && self.JobType == other.JobType
            && self.PhoneNumber == other.PhoneNumber
            && self.RoutingString == other.RoutingString
            && self.SenderName == other.SenderName
            && self.RecipientName == other.RecipientName
            && self.Size == other.Size
            && self.StartTime == other.StartTime
            && self.Status == other.Status
            && self.StatusString == other.StatusString
            && self.SubmittedTime == other.SubmittedTime
            && self.TotalPages == other.TotalPages
            && self.Tsid == other.Tsid
            && self.UserName == other.UserName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FAX_DEVICE_STATUSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FAX_DEVICE_STATUSA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_DEVICE_STATUSW {
    pub SizeOfStruct: u32,
    pub CallerId: super::super::Foundation::PWSTR,
    pub Csid: super::super::Foundation::PWSTR,
    pub CurrentPage: u32,
    pub DeviceId: u32,
    pub DeviceName: super::super::Foundation::PWSTR,
    pub DocumentName: super::super::Foundation::PWSTR,
    pub JobType: u32,
    pub PhoneNumber: super::super::Foundation::PWSTR,
    pub RoutingString: super::super::Foundation::PWSTR,
    pub SenderName: super::super::Foundation::PWSTR,
    pub RecipientName: super::super::Foundation::PWSTR,
    pub Size: u32,
    pub StartTime: super::super::Foundation::FILETIME,
    pub Status: u32,
    pub StatusString: super::super::Foundation::PWSTR,
    pub SubmittedTime: super::super::Foundation::FILETIME,
    pub TotalPages: u32,
    pub Tsid: super::super::Foundation::PWSTR,
    pub UserName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl FAX_DEVICE_STATUSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FAX_DEVICE_STATUSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FAX_DEVICE_STATUSW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FAX_DEVICE_STATUSW")
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
impl ::std::cmp::PartialEq for FAX_DEVICE_STATUSW {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct
            && self.CallerId == other.CallerId
            && self.Csid == other.Csid
            && self.CurrentPage == other.CurrentPage
            && self.DeviceId == other.DeviceId
            && self.DeviceName == other.DeviceName
            && self.DocumentName == other.DocumentName
            && self.JobType == other.JobType
            && self.PhoneNumber == other.PhoneNumber
            && self.RoutingString == other.RoutingString
            && self.SenderName == other.SenderName
            && self.RecipientName == other.RecipientName
            && self.Size == other.Size
            && self.StartTime == other.StartTime
            && self.Status == other.Status
            && self.StatusString == other.StatusString
            && self.SubmittedTime == other.SubmittedTime
            && self.TotalPages == other.TotalPages
            && self.Tsid == other.Tsid
            && self.UserName == other.UserName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FAX_DEVICE_STATUSW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FAX_DEVICE_STATUSW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_DEV_STATUS {
    pub SizeOfStruct: u32,
    pub StatusId: u32,
    pub StringId: u32,
    pub PageCount: u32,
    pub CSI: super::super::Foundation::PWSTR,
    pub CallerId: super::super::Foundation::PWSTR,
    pub RoutingInfo: super::super::Foundation::PWSTR,
    pub ErrorCode: u32,
    pub Reserved: [u32; 3],
}
#[cfg(feature = "Win32_Foundation")]
impl FAX_DEV_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FAX_DEV_STATUS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FAX_DEV_STATUS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FAX_DEV_STATUS")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("StatusId", &self.StatusId)
            .field("StringId", &self.StringId)
            .field("PageCount", &self.PageCount)
            .field("CSI", &self.CSI)
            .field("CallerId", &self.CallerId)
            .field("RoutingInfo", &self.RoutingInfo)
            .field("ErrorCode", &self.ErrorCode)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FAX_DEV_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.StatusId == other.StatusId && self.StringId == other.StringId && self.PageCount == other.PageCount && self.CSI == other.CSI && self.CallerId == other.CallerId && self.RoutingInfo == other.RoutingInfo && self.ErrorCode == other.ErrorCode && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FAX_DEV_STATUS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FAX_DEV_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FAX_ENUM_DELIVERY_REPORT_TYPES(pub i32);
pub const DRT_NONE: FAX_ENUM_DELIVERY_REPORT_TYPES = FAX_ENUM_DELIVERY_REPORT_TYPES(0i32);
pub const DRT_EMAIL: FAX_ENUM_DELIVERY_REPORT_TYPES = FAX_ENUM_DELIVERY_REPORT_TYPES(1i32);
pub const DRT_INBOX: FAX_ENUM_DELIVERY_REPORT_TYPES = FAX_ENUM_DELIVERY_REPORT_TYPES(2i32);
impl ::std::convert::From<i32> for FAX_ENUM_DELIVERY_REPORT_TYPES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FAX_ENUM_DELIVERY_REPORT_TYPES {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FAX_ENUM_DEVICE_ID_SOURCE(pub i32);
pub const DEV_ID_SRC_FAX: FAX_ENUM_DEVICE_ID_SOURCE = FAX_ENUM_DEVICE_ID_SOURCE(0i32);
pub const DEV_ID_SRC_TAPI: FAX_ENUM_DEVICE_ID_SOURCE = FAX_ENUM_DEVICE_ID_SOURCE(1i32);
impl ::std::convert::From<i32> for FAX_ENUM_DEVICE_ID_SOURCE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FAX_ENUM_DEVICE_ID_SOURCE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FAX_ENUM_JOB_COMMANDS(pub i32);
pub const JC_UNKNOWN: FAX_ENUM_JOB_COMMANDS = FAX_ENUM_JOB_COMMANDS(0i32);
pub const JC_DELETE: FAX_ENUM_JOB_COMMANDS = FAX_ENUM_JOB_COMMANDS(1i32);
pub const JC_PAUSE: FAX_ENUM_JOB_COMMANDS = FAX_ENUM_JOB_COMMANDS(2i32);
pub const JC_RESUME: FAX_ENUM_JOB_COMMANDS = FAX_ENUM_JOB_COMMANDS(3i32);
impl ::std::convert::From<i32> for FAX_ENUM_JOB_COMMANDS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FAX_ENUM_JOB_COMMANDS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FAX_ENUM_JOB_SEND_ATTRIBUTES(pub i32);
pub const JSA_NOW: FAX_ENUM_JOB_SEND_ATTRIBUTES = FAX_ENUM_JOB_SEND_ATTRIBUTES(0i32);
pub const JSA_SPECIFIC_TIME: FAX_ENUM_JOB_SEND_ATTRIBUTES = FAX_ENUM_JOB_SEND_ATTRIBUTES(1i32);
pub const JSA_DISCOUNT_PERIOD: FAX_ENUM_JOB_SEND_ATTRIBUTES = FAX_ENUM_JOB_SEND_ATTRIBUTES(2i32);
impl ::std::convert::From<i32> for FAX_ENUM_JOB_SEND_ATTRIBUTES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FAX_ENUM_JOB_SEND_ATTRIBUTES {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FAX_ENUM_LOG_CATEGORIES(pub i32);
pub const FAXLOG_CATEGORY_INIT: FAX_ENUM_LOG_CATEGORIES = FAX_ENUM_LOG_CATEGORIES(1i32);
pub const FAXLOG_CATEGORY_OUTBOUND: FAX_ENUM_LOG_CATEGORIES = FAX_ENUM_LOG_CATEGORIES(2i32);
pub const FAXLOG_CATEGORY_INBOUND: FAX_ENUM_LOG_CATEGORIES = FAX_ENUM_LOG_CATEGORIES(3i32);
pub const FAXLOG_CATEGORY_UNKNOWN: FAX_ENUM_LOG_CATEGORIES = FAX_ENUM_LOG_CATEGORIES(4i32);
impl ::std::convert::From<i32> for FAX_ENUM_LOG_CATEGORIES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FAX_ENUM_LOG_CATEGORIES {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FAX_ENUM_LOG_LEVELS(pub i32);
pub const FAXLOG_LEVEL_NONE: FAX_ENUM_LOG_LEVELS = FAX_ENUM_LOG_LEVELS(0i32);
pub const FAXLOG_LEVEL_MIN: FAX_ENUM_LOG_LEVELS = FAX_ENUM_LOG_LEVELS(1i32);
pub const FAXLOG_LEVEL_MED: FAX_ENUM_LOG_LEVELS = FAX_ENUM_LOG_LEVELS(2i32);
pub const FAXLOG_LEVEL_MAX: FAX_ENUM_LOG_LEVELS = FAX_ENUM_LOG_LEVELS(3i32);
impl ::std::convert::From<i32> for FAX_ENUM_LOG_LEVELS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FAX_ENUM_LOG_LEVELS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FAX_ENUM_PORT_OPEN_TYPE(pub i32);
pub const PORT_OPEN_QUERY: FAX_ENUM_PORT_OPEN_TYPE = FAX_ENUM_PORT_OPEN_TYPE(1i32);
pub const PORT_OPEN_MODIFY: FAX_ENUM_PORT_OPEN_TYPE = FAX_ENUM_PORT_OPEN_TYPE(2i32);
impl ::std::convert::From<i32> for FAX_ENUM_PORT_OPEN_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FAX_ENUM_PORT_OPEN_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FAX_ERR_BAD_GROUP_CONFIGURATION: i32 = 7003i32;
pub const FAX_ERR_DEVICE_NUM_LIMIT_EXCEEDED: i32 = 7010i32;
pub const FAX_ERR_DIRECTORY_IN_USE: i32 = 7007i32;
pub const FAX_ERR_END: i32 = 7013i32;
pub const FAX_ERR_FILE_ACCESS_DENIED: i32 = 7008i32;
pub const FAX_ERR_GROUP_IN_USE: i32 = 7004i32;
pub const FAX_ERR_GROUP_NOT_FOUND: i32 = 7002i32;
pub const FAX_ERR_MESSAGE_NOT_FOUND: i32 = 7009i32;
pub const FAX_ERR_NOT_NTFS: i32 = 7006i32;
pub const FAX_ERR_NOT_SUPPORTED_ON_THIS_SKU: i32 = 7011i32;
pub const FAX_ERR_RECIPIENTS_LIMIT: i32 = 7013i32;
pub const FAX_ERR_RULE_NOT_FOUND: i32 = 7005i32;
pub const FAX_ERR_SRV_OUTOFMEMORY: i32 = 7001i32;
pub const FAX_ERR_START: i32 = 7001i32;
pub const FAX_ERR_VERSION_MISMATCH: i32 = 7012i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_EVENTA {
    pub SizeOfStruct: u32,
    pub TimeStamp: super::super::Foundation::FILETIME,
    pub DeviceId: u32,
    pub EventId: u32,
    pub JobId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl FAX_EVENTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FAX_EVENTA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FAX_EVENTA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FAX_EVENTA").field("SizeOfStruct", &self.SizeOfStruct).field("TimeStamp", &self.TimeStamp).field("DeviceId", &self.DeviceId).field("EventId", &self.EventId).field("JobId", &self.JobId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FAX_EVENTA {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.TimeStamp == other.TimeStamp && self.DeviceId == other.DeviceId && self.EventId == other.EventId && self.JobId == other.JobId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FAX_EVENTA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FAX_EVENTA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_EVENTW {
    pub SizeOfStruct: u32,
    pub TimeStamp: super::super::Foundation::FILETIME,
    pub DeviceId: u32,
    pub EventId: u32,
    pub JobId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl FAX_EVENTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FAX_EVENTW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FAX_EVENTW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FAX_EVENTW").field("SizeOfStruct", &self.SizeOfStruct).field("TimeStamp", &self.TimeStamp).field("DeviceId", &self.DeviceId).field("EventId", &self.EventId).field("JobId", &self.JobId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FAX_EVENTW {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.TimeStamp == other.TimeStamp && self.DeviceId == other.DeviceId && self.EventId == other.EventId && self.JobId == other.JobId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FAX_EVENTW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FAX_EVENTW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FAX_E_BAD_GROUP_CONFIGURATION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147214501i32 as _);
pub const FAX_E_DEVICE_NUM_LIMIT_EXCEEDED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147214494i32 as _);
pub const FAX_E_DIRECTORY_IN_USE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147214497i32 as _);
pub const FAX_E_FILE_ACCESS_DENIED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147214496i32 as _);
pub const FAX_E_GROUP_IN_USE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147214500i32 as _);
pub const FAX_E_GROUP_NOT_FOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147214502i32 as _);
pub const FAX_E_MESSAGE_NOT_FOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147214495i32 as _);
pub const FAX_E_NOT_NTFS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147214498i32 as _);
pub const FAX_E_NOT_SUPPORTED_ON_THIS_SKU: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147214493i32 as _);
pub const FAX_E_RECIPIENTS_LIMIT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147214491i32 as _);
pub const FAX_E_RULE_NOT_FOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147214499i32 as _);
pub const FAX_E_SRV_OUTOFMEMORY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147214503i32 as _);
pub const FAX_E_VERSION_MISMATCH: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147214492i32 as _);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_GLOBAL_ROUTING_INFOA {
    pub SizeOfStruct: u32,
    pub Priority: u32,
    pub Guid: super::super::Foundation::PSTR,
    pub FriendlyName: super::super::Foundation::PSTR,
    pub FunctionName: super::super::Foundation::PSTR,
    pub ExtensionImageName: super::super::Foundation::PSTR,
    pub ExtensionFriendlyName: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl FAX_GLOBAL_ROUTING_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FAX_GLOBAL_ROUTING_INFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FAX_GLOBAL_ROUTING_INFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FAX_GLOBAL_ROUTING_INFOA")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("Priority", &self.Priority)
            .field("Guid", &self.Guid)
            .field("FriendlyName", &self.FriendlyName)
            .field("FunctionName", &self.FunctionName)
            .field("ExtensionImageName", &self.ExtensionImageName)
            .field("ExtensionFriendlyName", &self.ExtensionFriendlyName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FAX_GLOBAL_ROUTING_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.Priority == other.Priority && self.Guid == other.Guid && self.FriendlyName == other.FriendlyName && self.FunctionName == other.FunctionName && self.ExtensionImageName == other.ExtensionImageName && self.ExtensionFriendlyName == other.ExtensionFriendlyName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FAX_GLOBAL_ROUTING_INFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FAX_GLOBAL_ROUTING_INFOA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_GLOBAL_ROUTING_INFOW {
    pub SizeOfStruct: u32,
    pub Priority: u32,
    pub Guid: super::super::Foundation::PWSTR,
    pub FriendlyName: super::super::Foundation::PWSTR,
    pub FunctionName: super::super::Foundation::PWSTR,
    pub ExtensionImageName: super::super::Foundation::PWSTR,
    pub ExtensionFriendlyName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl FAX_GLOBAL_ROUTING_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FAX_GLOBAL_ROUTING_INFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FAX_GLOBAL_ROUTING_INFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FAX_GLOBAL_ROUTING_INFOW")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("Priority", &self.Priority)
            .field("Guid", &self.Guid)
            .field("FriendlyName", &self.FriendlyName)
            .field("FunctionName", &self.FunctionName)
            .field("ExtensionImageName", &self.ExtensionImageName)
            .field("ExtensionFriendlyName", &self.ExtensionFriendlyName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FAX_GLOBAL_ROUTING_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.Priority == other.Priority && self.Guid == other.Guid && self.FriendlyName == other.FriendlyName && self.FunctionName == other.FunctionName && self.ExtensionImageName == other.ExtensionImageName && self.ExtensionFriendlyName == other.ExtensionFriendlyName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FAX_GLOBAL_ROUTING_INFOW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FAX_GLOBAL_ROUTING_INFOW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FAX_GROUP_STATUS_ENUM(pub i32);
pub const fgsALL_DEV_VALID: FAX_GROUP_STATUS_ENUM = FAX_GROUP_STATUS_ENUM(0i32);
pub const fgsEMPTY: FAX_GROUP_STATUS_ENUM = FAX_GROUP_STATUS_ENUM(1i32);
pub const fgsALL_DEV_NOT_VALID: FAX_GROUP_STATUS_ENUM = FAX_GROUP_STATUS_ENUM(2i32);
pub const fgsSOME_DEV_NOT_VALID: FAX_GROUP_STATUS_ENUM = FAX_GROUP_STATUS_ENUM(3i32);
impl ::std::convert::From<i32> for FAX_GROUP_STATUS_ENUM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FAX_GROUP_STATUS_ENUM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_JOB_ENTRYA {
    pub SizeOfStruct: u32,
    pub JobId: u32,
    pub UserName: super::super::Foundation::PSTR,
    pub JobType: u32,
    pub QueueStatus: u32,
    pub Status: u32,
    pub Size: u32,
    pub PageCount: u32,
    pub RecipientNumber: super::super::Foundation::PSTR,
    pub RecipientName: super::super::Foundation::PSTR,
    pub Tsid: super::super::Foundation::PSTR,
    pub SenderName: super::super::Foundation::PSTR,
    pub SenderCompany: super::super::Foundation::PSTR,
    pub SenderDept: super::super::Foundation::PSTR,
    pub BillingCode: super::super::Foundation::PSTR,
    pub ScheduleAction: u32,
    pub ScheduleTime: super::super::Foundation::SYSTEMTIME,
    pub DeliveryReportType: u32,
    pub DeliveryReportAddress: super::super::Foundation::PSTR,
    pub DocumentName: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl FAX_JOB_ENTRYA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FAX_JOB_ENTRYA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FAX_JOB_ENTRYA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FAX_JOB_ENTRYA")
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
impl ::std::cmp::PartialEq for FAX_JOB_ENTRYA {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct
            && self.JobId == other.JobId
            && self.UserName == other.UserName
            && self.JobType == other.JobType
            && self.QueueStatus == other.QueueStatus
            && self.Status == other.Status
            && self.Size == other.Size
            && self.PageCount == other.PageCount
            && self.RecipientNumber == other.RecipientNumber
            && self.RecipientName == other.RecipientName
            && self.Tsid == other.Tsid
            && self.SenderName == other.SenderName
            && self.SenderCompany == other.SenderCompany
            && self.SenderDept == other.SenderDept
            && self.BillingCode == other.BillingCode
            && self.ScheduleAction == other.ScheduleAction
            && self.ScheduleTime == other.ScheduleTime
            && self.DeliveryReportType == other.DeliveryReportType
            && self.DeliveryReportAddress == other.DeliveryReportAddress
            && self.DocumentName == other.DocumentName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FAX_JOB_ENTRYA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FAX_JOB_ENTRYA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_JOB_ENTRYW {
    pub SizeOfStruct: u32,
    pub JobId: u32,
    pub UserName: super::super::Foundation::PWSTR,
    pub JobType: u32,
    pub QueueStatus: u32,
    pub Status: u32,
    pub Size: u32,
    pub PageCount: u32,
    pub RecipientNumber: super::super::Foundation::PWSTR,
    pub RecipientName: super::super::Foundation::PWSTR,
    pub Tsid: super::super::Foundation::PWSTR,
    pub SenderName: super::super::Foundation::PWSTR,
    pub SenderCompany: super::super::Foundation::PWSTR,
    pub SenderDept: super::super::Foundation::PWSTR,
    pub BillingCode: super::super::Foundation::PWSTR,
    pub ScheduleAction: u32,
    pub ScheduleTime: super::super::Foundation::SYSTEMTIME,
    pub DeliveryReportType: u32,
    pub DeliveryReportAddress: super::super::Foundation::PWSTR,
    pub DocumentName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl FAX_JOB_ENTRYW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FAX_JOB_ENTRYW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FAX_JOB_ENTRYW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FAX_JOB_ENTRYW")
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
impl ::std::cmp::PartialEq for FAX_JOB_ENTRYW {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct
            && self.JobId == other.JobId
            && self.UserName == other.UserName
            && self.JobType == other.JobType
            && self.QueueStatus == other.QueueStatus
            && self.Status == other.Status
            && self.Size == other.Size
            && self.PageCount == other.PageCount
            && self.RecipientNumber == other.RecipientNumber
            && self.RecipientName == other.RecipientName
            && self.Tsid == other.Tsid
            && self.SenderName == other.SenderName
            && self.SenderCompany == other.SenderCompany
            && self.SenderDept == other.SenderDept
            && self.BillingCode == other.BillingCode
            && self.ScheduleAction == other.ScheduleAction
            && self.ScheduleTime == other.ScheduleTime
            && self.DeliveryReportType == other.DeliveryReportType
            && self.DeliveryReportAddress == other.DeliveryReportAddress
            && self.DocumentName == other.DocumentName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FAX_JOB_ENTRYW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FAX_JOB_ENTRYW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FAX_JOB_EXTENDED_STATUS_ENUM(pub i32);
pub const fjesNONE: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(0i32);
pub const fjesDISCONNECTED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(1i32);
pub const fjesINITIALIZING: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(2i32);
pub const fjesDIALING: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(3i32);
pub const fjesTRANSMITTING: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(4i32);
pub const fjesANSWERED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(5i32);
pub const fjesRECEIVING: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(6i32);
pub const fjesLINE_UNAVAILABLE: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(7i32);
pub const fjesBUSY: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(8i32);
pub const fjesNO_ANSWER: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(9i32);
pub const fjesBAD_ADDRESS: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(10i32);
pub const fjesNO_DIAL_TONE: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(11i32);
pub const fjesFATAL_ERROR: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(12i32);
pub const fjesCALL_DELAYED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(13i32);
pub const fjesCALL_BLACKLISTED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(14i32);
pub const fjesNOT_FAX_CALL: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(15i32);
pub const fjesPARTIALLY_RECEIVED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(16i32);
pub const fjesHANDLED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(17i32);
pub const fjesCALL_COMPLETED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(18i32);
pub const fjesCALL_ABORTED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(19i32);
pub const fjesPROPRIETARY: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(16777216i32);
impl ::std::convert::From<i32> for FAX_JOB_EXTENDED_STATUS_ENUM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FAX_JOB_EXTENDED_STATUS_ENUM {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FAX_JOB_MANAGE: u32 = 64u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FAX_JOB_OPERATIONS_ENUM(pub i32);
pub const fjoVIEW: FAX_JOB_OPERATIONS_ENUM = FAX_JOB_OPERATIONS_ENUM(1i32);
pub const fjoPAUSE: FAX_JOB_OPERATIONS_ENUM = FAX_JOB_OPERATIONS_ENUM(2i32);
pub const fjoRESUME: FAX_JOB_OPERATIONS_ENUM = FAX_JOB_OPERATIONS_ENUM(4i32);
pub const fjoRESTART: FAX_JOB_OPERATIONS_ENUM = FAX_JOB_OPERATIONS_ENUM(8i32);
pub const fjoDELETE: FAX_JOB_OPERATIONS_ENUM = FAX_JOB_OPERATIONS_ENUM(16i32);
pub const fjoRECIPIENT_INFO: FAX_JOB_OPERATIONS_ENUM = FAX_JOB_OPERATIONS_ENUM(32i32);
pub const fjoSENDER_INFO: FAX_JOB_OPERATIONS_ENUM = FAX_JOB_OPERATIONS_ENUM(64i32);
impl ::std::convert::From<i32> for FAX_JOB_OPERATIONS_ENUM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FAX_JOB_OPERATIONS_ENUM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_JOB_PARAMA {
    pub SizeOfStruct: u32,
    pub RecipientNumber: super::super::Foundation::PSTR,
    pub RecipientName: super::super::Foundation::PSTR,
    pub Tsid: super::super::Foundation::PSTR,
    pub SenderName: super::super::Foundation::PSTR,
    pub SenderCompany: super::super::Foundation::PSTR,
    pub SenderDept: super::super::Foundation::PSTR,
    pub BillingCode: super::super::Foundation::PSTR,
    pub ScheduleAction: u32,
    pub ScheduleTime: super::super::Foundation::SYSTEMTIME,
    pub DeliveryReportType: u32,
    pub DeliveryReportAddress: super::super::Foundation::PSTR,
    pub DocumentName: super::super::Foundation::PSTR,
    pub CallHandle: u32,
    pub Reserved: [usize; 3],
}
#[cfg(feature = "Win32_Foundation")]
impl FAX_JOB_PARAMA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FAX_JOB_PARAMA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FAX_JOB_PARAMA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FAX_JOB_PARAMA")
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
impl ::std::cmp::PartialEq for FAX_JOB_PARAMA {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct
            && self.RecipientNumber == other.RecipientNumber
            && self.RecipientName == other.RecipientName
            && self.Tsid == other.Tsid
            && self.SenderName == other.SenderName
            && self.SenderCompany == other.SenderCompany
            && self.SenderDept == other.SenderDept
            && self.BillingCode == other.BillingCode
            && self.ScheduleAction == other.ScheduleAction
            && self.ScheduleTime == other.ScheduleTime
            && self.DeliveryReportType == other.DeliveryReportType
            && self.DeliveryReportAddress == other.DeliveryReportAddress
            && self.DocumentName == other.DocumentName
            && self.CallHandle == other.CallHandle
            && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FAX_JOB_PARAMA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FAX_JOB_PARAMA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_JOB_PARAMW {
    pub SizeOfStruct: u32,
    pub RecipientNumber: super::super::Foundation::PWSTR,
    pub RecipientName: super::super::Foundation::PWSTR,
    pub Tsid: super::super::Foundation::PWSTR,
    pub SenderName: super::super::Foundation::PWSTR,
    pub SenderCompany: super::super::Foundation::PWSTR,
    pub SenderDept: super::super::Foundation::PWSTR,
    pub BillingCode: super::super::Foundation::PWSTR,
    pub ScheduleAction: u32,
    pub ScheduleTime: super::super::Foundation::SYSTEMTIME,
    pub DeliveryReportType: u32,
    pub DeliveryReportAddress: super::super::Foundation::PWSTR,
    pub DocumentName: super::super::Foundation::PWSTR,
    pub CallHandle: u32,
    pub Reserved: [usize; 3],
}
#[cfg(feature = "Win32_Foundation")]
impl FAX_JOB_PARAMW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FAX_JOB_PARAMW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FAX_JOB_PARAMW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FAX_JOB_PARAMW")
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
impl ::std::cmp::PartialEq for FAX_JOB_PARAMW {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct
            && self.RecipientNumber == other.RecipientNumber
            && self.RecipientName == other.RecipientName
            && self.Tsid == other.Tsid
            && self.SenderName == other.SenderName
            && self.SenderCompany == other.SenderCompany
            && self.SenderDept == other.SenderDept
            && self.BillingCode == other.BillingCode
            && self.ScheduleAction == other.ScheduleAction
            && self.ScheduleTime == other.ScheduleTime
            && self.DeliveryReportType == other.DeliveryReportType
            && self.DeliveryReportAddress == other.DeliveryReportAddress
            && self.DocumentName == other.DocumentName
            && self.CallHandle == other.CallHandle
            && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FAX_JOB_PARAMW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FAX_JOB_PARAMW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FAX_JOB_QUERY: u32 = 2u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FAX_JOB_STATUS_ENUM(pub i32);
pub const fjsPENDING: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(1i32);
pub const fjsINPROGRESS: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(2i32);
pub const fjsFAILED: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(8i32);
pub const fjsPAUSED: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(16i32);
pub const fjsNOLINE: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(32i32);
pub const fjsRETRYING: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(64i32);
pub const fjsRETRIES_EXCEEDED: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(128i32);
pub const fjsCOMPLETED: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(256i32);
pub const fjsCANCELED: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(512i32);
pub const fjsCANCELING: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(1024i32);
pub const fjsROUTING: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(2048i32);
impl ::std::convert::From<i32> for FAX_JOB_STATUS_ENUM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FAX_JOB_STATUS_ENUM {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FAX_JOB_SUBMIT: u32 = 1u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FAX_JOB_TYPE_ENUM(pub i32);
pub const fjtSEND: FAX_JOB_TYPE_ENUM = FAX_JOB_TYPE_ENUM(0i32);
pub const fjtRECEIVE: FAX_JOB_TYPE_ENUM = FAX_JOB_TYPE_ENUM(1i32);
pub const fjtROUTING: FAX_JOB_TYPE_ENUM = FAX_JOB_TYPE_ENUM(2i32);
impl ::std::convert::From<i32> for FAX_JOB_TYPE_ENUM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FAX_JOB_TYPE_ENUM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_LOG_CATEGORYA {
    pub Name: super::super::Foundation::PSTR,
    pub Category: u32,
    pub Level: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl FAX_LOG_CATEGORYA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FAX_LOG_CATEGORYA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FAX_LOG_CATEGORYA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FAX_LOG_CATEGORYA").field("Name", &self.Name).field("Category", &self.Category).field("Level", &self.Level).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FAX_LOG_CATEGORYA {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.Category == other.Category && self.Level == other.Level
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FAX_LOG_CATEGORYA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FAX_LOG_CATEGORYA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_LOG_CATEGORYW {
    pub Name: super::super::Foundation::PWSTR,
    pub Category: u32,
    pub Level: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl FAX_LOG_CATEGORYW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FAX_LOG_CATEGORYW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FAX_LOG_CATEGORYW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FAX_LOG_CATEGORYW").field("Name", &self.Name).field("Category", &self.Category).field("Level", &self.Level).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FAX_LOG_CATEGORYW {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.Category == other.Category && self.Level == other.Level
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FAX_LOG_CATEGORYW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FAX_LOG_CATEGORYW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FAX_LOG_LEVEL_ENUM(pub i32);
pub const fllNONE: FAX_LOG_LEVEL_ENUM = FAX_LOG_LEVEL_ENUM(0i32);
pub const fllMIN: FAX_LOG_LEVEL_ENUM = FAX_LOG_LEVEL_ENUM(1i32);
pub const fllMED: FAX_LOG_LEVEL_ENUM = FAX_LOG_LEVEL_ENUM(2i32);
pub const fllMAX: FAX_LOG_LEVEL_ENUM = FAX_LOG_LEVEL_ENUM(3i32);
impl ::std::convert::From<i32> for FAX_LOG_LEVEL_ENUM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FAX_LOG_LEVEL_ENUM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_PORT_INFOA {
    pub SizeOfStruct: u32,
    pub DeviceId: u32,
    pub State: u32,
    pub Flags: u32,
    pub Rings: u32,
    pub Priority: u32,
    pub DeviceName: super::super::Foundation::PSTR,
    pub Tsid: super::super::Foundation::PSTR,
    pub Csid: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl FAX_PORT_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FAX_PORT_INFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FAX_PORT_INFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FAX_PORT_INFOA")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("DeviceId", &self.DeviceId)
            .field("State", &self.State)
            .field("Flags", &self.Flags)
            .field("Rings", &self.Rings)
            .field("Priority", &self.Priority)
            .field("DeviceName", &self.DeviceName)
            .field("Tsid", &self.Tsid)
            .field("Csid", &self.Csid)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FAX_PORT_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.DeviceId == other.DeviceId && self.State == other.State && self.Flags == other.Flags && self.Rings == other.Rings && self.Priority == other.Priority && self.DeviceName == other.DeviceName && self.Tsid == other.Tsid && self.Csid == other.Csid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FAX_PORT_INFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FAX_PORT_INFOA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_PORT_INFOW {
    pub SizeOfStruct: u32,
    pub DeviceId: u32,
    pub State: u32,
    pub Flags: u32,
    pub Rings: u32,
    pub Priority: u32,
    pub DeviceName: super::super::Foundation::PWSTR,
    pub Tsid: super::super::Foundation::PWSTR,
    pub Csid: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl FAX_PORT_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FAX_PORT_INFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FAX_PORT_INFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FAX_PORT_INFOW")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("DeviceId", &self.DeviceId)
            .field("State", &self.State)
            .field("Flags", &self.Flags)
            .field("Rings", &self.Rings)
            .field("Priority", &self.Priority)
            .field("DeviceName", &self.DeviceName)
            .field("Tsid", &self.Tsid)
            .field("Csid", &self.Csid)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FAX_PORT_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.DeviceId == other.DeviceId && self.State == other.State && self.Flags == other.Flags && self.Rings == other.Rings && self.Priority == other.Priority && self.DeviceName == other.DeviceName && self.Tsid == other.Tsid && self.Csid == other.Csid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FAX_PORT_INFOW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FAX_PORT_INFOW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FAX_PORT_QUERY: u32 = 16u32;
pub const FAX_PORT_SET: u32 = 32u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_PRINT_INFOA {
    pub SizeOfStruct: u32,
    pub DocName: super::super::Foundation::PSTR,
    pub RecipientName: super::super::Foundation::PSTR,
    pub RecipientNumber: super::super::Foundation::PSTR,
    pub SenderName: super::super::Foundation::PSTR,
    pub SenderCompany: super::super::Foundation::PSTR,
    pub SenderDept: super::super::Foundation::PSTR,
    pub SenderBillingCode: super::super::Foundation::PSTR,
    pub Reserved: super::super::Foundation::PSTR,
    pub DrEmailAddress: super::super::Foundation::PSTR,
    pub OutputFileName: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl FAX_PRINT_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FAX_PRINT_INFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FAX_PRINT_INFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FAX_PRINT_INFOA")
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
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FAX_PRINT_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.DocName == other.DocName && self.RecipientName == other.RecipientName && self.RecipientNumber == other.RecipientNumber && self.SenderName == other.SenderName && self.SenderCompany == other.SenderCompany && self.SenderDept == other.SenderDept && self.SenderBillingCode == other.SenderBillingCode && self.Reserved == other.Reserved && self.DrEmailAddress == other.DrEmailAddress && self.OutputFileName == other.OutputFileName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FAX_PRINT_INFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FAX_PRINT_INFOA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_PRINT_INFOW {
    pub SizeOfStruct: u32,
    pub DocName: super::super::Foundation::PWSTR,
    pub RecipientName: super::super::Foundation::PWSTR,
    pub RecipientNumber: super::super::Foundation::PWSTR,
    pub SenderName: super::super::Foundation::PWSTR,
    pub SenderCompany: super::super::Foundation::PWSTR,
    pub SenderDept: super::super::Foundation::PWSTR,
    pub SenderBillingCode: super::super::Foundation::PWSTR,
    pub Reserved: super::super::Foundation::PWSTR,
    pub DrEmailAddress: super::super::Foundation::PWSTR,
    pub OutputFileName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl FAX_PRINT_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FAX_PRINT_INFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FAX_PRINT_INFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FAX_PRINT_INFOW")
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
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FAX_PRINT_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.DocName == other.DocName && self.RecipientName == other.RecipientName && self.RecipientNumber == other.RecipientNumber && self.SenderName == other.SenderName && self.SenderCompany == other.SenderCompany && self.SenderDept == other.SenderDept && self.SenderBillingCode == other.SenderBillingCode && self.Reserved == other.Reserved && self.DrEmailAddress == other.DrEmailAddress && self.OutputFileName == other.OutputFileName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FAX_PRINT_INFOW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FAX_PRINT_INFOW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FAX_PRIORITY_TYPE_ENUM(pub i32);
pub const fptLOW: FAX_PRIORITY_TYPE_ENUM = FAX_PRIORITY_TYPE_ENUM(0i32);
pub const fptNORMAL: FAX_PRIORITY_TYPE_ENUM = FAX_PRIORITY_TYPE_ENUM(1i32);
pub const fptHIGH: FAX_PRIORITY_TYPE_ENUM = FAX_PRIORITY_TYPE_ENUM(2i32);
impl ::std::convert::From<i32> for FAX_PRIORITY_TYPE_ENUM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FAX_PRIORITY_TYPE_ENUM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FAX_PROVIDER_STATUS_ENUM(pub i32);
pub const fpsSUCCESS: FAX_PROVIDER_STATUS_ENUM = FAX_PROVIDER_STATUS_ENUM(0i32);
pub const fpsSERVER_ERROR: FAX_PROVIDER_STATUS_ENUM = FAX_PROVIDER_STATUS_ENUM(1i32);
pub const fpsBAD_GUID: FAX_PROVIDER_STATUS_ENUM = FAX_PROVIDER_STATUS_ENUM(2i32);
pub const fpsBAD_VERSION: FAX_PROVIDER_STATUS_ENUM = FAX_PROVIDER_STATUS_ENUM(3i32);
pub const fpsCANT_LOAD: FAX_PROVIDER_STATUS_ENUM = FAX_PROVIDER_STATUS_ENUM(4i32);
pub const fpsCANT_LINK: FAX_PROVIDER_STATUS_ENUM = FAX_PROVIDER_STATUS_ENUM(5i32);
pub const fpsCANT_INIT: FAX_PROVIDER_STATUS_ENUM = FAX_PROVIDER_STATUS_ENUM(6i32);
impl ::std::convert::From<i32> for FAX_PROVIDER_STATUS_ENUM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FAX_PROVIDER_STATUS_ENUM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FAX_RECEIPT_TYPE_ENUM(pub i32);
pub const frtNONE: FAX_RECEIPT_TYPE_ENUM = FAX_RECEIPT_TYPE_ENUM(0i32);
pub const frtMAIL: FAX_RECEIPT_TYPE_ENUM = FAX_RECEIPT_TYPE_ENUM(1i32);
pub const frtMSGBOX: FAX_RECEIPT_TYPE_ENUM = FAX_RECEIPT_TYPE_ENUM(4i32);
impl ::std::convert::From<i32> for FAX_RECEIPT_TYPE_ENUM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FAX_RECEIPT_TYPE_ENUM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_RECEIVE {
    pub SizeOfStruct: u32,
    pub FileName: super::super::Foundation::PWSTR,
    pub ReceiverName: super::super::Foundation::PWSTR,
    pub ReceiverNumber: super::super::Foundation::PWSTR,
    pub Reserved: [u32; 4],
}
#[cfg(feature = "Win32_Foundation")]
impl FAX_RECEIVE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FAX_RECEIVE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FAX_RECEIVE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FAX_RECEIVE").field("SizeOfStruct", &self.SizeOfStruct).field("FileName", &self.FileName).field("ReceiverName", &self.ReceiverName).field("ReceiverNumber", &self.ReceiverNumber).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FAX_RECEIVE {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.FileName == other.FileName && self.ReceiverName == other.ReceiverName && self.ReceiverNumber == other.ReceiverNumber && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FAX_RECEIVE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FAX_RECEIVE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_ROUTE {
    pub SizeOfStruct: u32,
    pub JobId: u32,
    pub ElapsedTime: u64,
    pub ReceiveTime: u64,
    pub PageCount: u32,
    pub Csid: super::super::Foundation::PWSTR,
    pub Tsid: super::super::Foundation::PWSTR,
    pub CallerId: super::super::Foundation::PWSTR,
    pub RoutingInfo: super::super::Foundation::PWSTR,
    pub ReceiverName: super::super::Foundation::PWSTR,
    pub ReceiverNumber: super::super::Foundation::PWSTR,
    pub DeviceName: super::super::Foundation::PWSTR,
    pub DeviceId: u32,
    pub RoutingInfoData: *mut u8,
    pub RoutingInfoDataSize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl FAX_ROUTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FAX_ROUTE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FAX_ROUTE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FAX_ROUTE")
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
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FAX_ROUTE {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct
            && self.JobId == other.JobId
            && self.ElapsedTime == other.ElapsedTime
            && self.ReceiveTime == other.ReceiveTime
            && self.PageCount == other.PageCount
            && self.Csid == other.Csid
            && self.Tsid == other.Tsid
            && self.CallerId == other.CallerId
            && self.RoutingInfo == other.RoutingInfo
            && self.ReceiverName == other.ReceiverName
            && self.ReceiverNumber == other.ReceiverNumber
            && self.DeviceName == other.DeviceName
            && self.DeviceId == other.DeviceId
            && self.RoutingInfoData == other.RoutingInfoData
            && self.RoutingInfoDataSize == other.RoutingInfoDataSize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FAX_ROUTE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FAX_ROUTE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_ROUTE_CALLBACKROUTINES {
    pub SizeOfStruct: u32,
    pub FaxRouteAddFile: ::std::option::Option<PFAXROUTEADDFILE>,
    pub FaxRouteDeleteFile: ::std::option::Option<PFAXROUTEDELETEFILE>,
    pub FaxRouteGetFile: ::std::option::Option<PFAXROUTEGETFILE>,
    pub FaxRouteEnumFiles: ::std::option::Option<PFAXROUTEENUMFILES>,
    pub FaxRouteModifyRoutingData: ::std::option::Option<PFAXROUTEMODIFYROUTINGDATA>,
}
#[cfg(feature = "Win32_Foundation")]
impl FAX_ROUTE_CALLBACKROUTINES {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FAX_ROUTE_CALLBACKROUTINES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FAX_ROUTE_CALLBACKROUTINES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FAX_ROUTE_CALLBACKROUTINES").field("SizeOfStruct", &self.SizeOfStruct).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FAX_ROUTE_CALLBACKROUTINES {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct
            && self.FaxRouteAddFile.map(|f| f as usize) == other.FaxRouteAddFile.map(|f| f as usize)
            && self.FaxRouteDeleteFile.map(|f| f as usize) == other.FaxRouteDeleteFile.map(|f| f as usize)
            && self.FaxRouteGetFile.map(|f| f as usize) == other.FaxRouteGetFile.map(|f| f as usize)
            && self.FaxRouteEnumFiles.map(|f| f as usize) == other.FaxRouteEnumFiles.map(|f| f as usize)
            && self.FaxRouteModifyRoutingData.map(|f| f as usize) == other.FaxRouteModifyRoutingData.map(|f| f as usize)
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FAX_ROUTE_CALLBACKROUTINES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FAX_ROUTE_CALLBACKROUTINES {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_ROUTING_METHODA {
    pub SizeOfStruct: u32,
    pub DeviceId: u32,
    pub Enabled: super::super::Foundation::BOOL,
    pub DeviceName: super::super::Foundation::PSTR,
    pub Guid: super::super::Foundation::PSTR,
    pub FriendlyName: super::super::Foundation::PSTR,
    pub FunctionName: super::super::Foundation::PSTR,
    pub ExtensionImageName: super::super::Foundation::PSTR,
    pub ExtensionFriendlyName: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl FAX_ROUTING_METHODA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FAX_ROUTING_METHODA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FAX_ROUTING_METHODA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FAX_ROUTING_METHODA")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("DeviceId", &self.DeviceId)
            .field("Enabled", &self.Enabled)
            .field("DeviceName", &self.DeviceName)
            .field("Guid", &self.Guid)
            .field("FriendlyName", &self.FriendlyName)
            .field("FunctionName", &self.FunctionName)
            .field("ExtensionImageName", &self.ExtensionImageName)
            .field("ExtensionFriendlyName", &self.ExtensionFriendlyName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FAX_ROUTING_METHODA {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.DeviceId == other.DeviceId && self.Enabled == other.Enabled && self.DeviceName == other.DeviceName && self.Guid == other.Guid && self.FriendlyName == other.FriendlyName && self.FunctionName == other.FunctionName && self.ExtensionImageName == other.ExtensionImageName && self.ExtensionFriendlyName == other.ExtensionFriendlyName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FAX_ROUTING_METHODA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FAX_ROUTING_METHODA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_ROUTING_METHODW {
    pub SizeOfStruct: u32,
    pub DeviceId: u32,
    pub Enabled: super::super::Foundation::BOOL,
    pub DeviceName: super::super::Foundation::PWSTR,
    pub Guid: super::super::Foundation::PWSTR,
    pub FriendlyName: super::super::Foundation::PWSTR,
    pub FunctionName: super::super::Foundation::PWSTR,
    pub ExtensionImageName: super::super::Foundation::PWSTR,
    pub ExtensionFriendlyName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl FAX_ROUTING_METHODW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FAX_ROUTING_METHODW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FAX_ROUTING_METHODW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FAX_ROUTING_METHODW")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("DeviceId", &self.DeviceId)
            .field("Enabled", &self.Enabled)
            .field("DeviceName", &self.DeviceName)
            .field("Guid", &self.Guid)
            .field("FriendlyName", &self.FriendlyName)
            .field("FunctionName", &self.FunctionName)
            .field("ExtensionImageName", &self.ExtensionImageName)
            .field("ExtensionFriendlyName", &self.ExtensionFriendlyName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FAX_ROUTING_METHODW {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.DeviceId == other.DeviceId && self.Enabled == other.Enabled && self.DeviceName == other.DeviceName && self.Guid == other.Guid && self.FriendlyName == other.FriendlyName && self.FunctionName == other.FunctionName && self.ExtensionImageName == other.ExtensionImageName && self.ExtensionFriendlyName == other.ExtensionFriendlyName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FAX_ROUTING_METHODW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FAX_ROUTING_METHODW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FAX_ROUTING_RULE_CODE_ENUM(pub i32);
pub const frrcANY_CODE: FAX_ROUTING_RULE_CODE_ENUM = FAX_ROUTING_RULE_CODE_ENUM(0i32);
impl ::std::convert::From<i32> for FAX_ROUTING_RULE_CODE_ENUM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FAX_ROUTING_RULE_CODE_ENUM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FAX_RULE_STATUS_ENUM(pub i32);
pub const frsVALID: FAX_RULE_STATUS_ENUM = FAX_RULE_STATUS_ENUM(0i32);
pub const frsEMPTY_GROUP: FAX_RULE_STATUS_ENUM = FAX_RULE_STATUS_ENUM(1i32);
pub const frsALL_GROUP_DEV_NOT_VALID: FAX_RULE_STATUS_ENUM = FAX_RULE_STATUS_ENUM(2i32);
pub const frsSOME_GROUP_DEV_NOT_VALID: FAX_RULE_STATUS_ENUM = FAX_RULE_STATUS_ENUM(3i32);
pub const frsBAD_DEVICE: FAX_RULE_STATUS_ENUM = FAX_RULE_STATUS_ENUM(4i32);
impl ::std::convert::From<i32> for FAX_RULE_STATUS_ENUM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FAX_RULE_STATUS_ENUM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FAX_SCHEDULE_TYPE_ENUM(pub i32);
pub const fstNOW: FAX_SCHEDULE_TYPE_ENUM = FAX_SCHEDULE_TYPE_ENUM(0i32);
pub const fstSPECIFIC_TIME: FAX_SCHEDULE_TYPE_ENUM = FAX_SCHEDULE_TYPE_ENUM(1i32);
pub const fstDISCOUNT_PERIOD: FAX_SCHEDULE_TYPE_ENUM = FAX_SCHEDULE_TYPE_ENUM(2i32);
impl ::std::convert::From<i32> for FAX_SCHEDULE_TYPE_ENUM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FAX_SCHEDULE_TYPE_ENUM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FAX_SEND {
    pub SizeOfStruct: u32,
    pub FileName: super::super::Foundation::PWSTR,
    pub CallerName: super::super::Foundation::PWSTR,
    pub CallerNumber: super::super::Foundation::PWSTR,
    pub ReceiverName: super::super::Foundation::PWSTR,
    pub ReceiverNumber: super::super::Foundation::PWSTR,
    pub Branding: super::super::Foundation::BOOL,
    pub CallHandle: u32,
    pub Reserved: [u32; 3],
}
#[cfg(feature = "Win32_Foundation")]
impl FAX_SEND {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FAX_SEND {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FAX_SEND {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FAX_SEND")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("FileName", &self.FileName)
            .field("CallerName", &self.CallerName)
            .field("CallerNumber", &self.CallerNumber)
            .field("ReceiverName", &self.ReceiverName)
            .field("ReceiverNumber", &self.ReceiverNumber)
            .field("Branding", &self.Branding)
            .field("CallHandle", &self.CallHandle)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FAX_SEND {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.FileName == other.FileName && self.CallerName == other.CallerName && self.CallerNumber == other.CallerNumber && self.ReceiverName == other.ReceiverName && self.ReceiverNumber == other.ReceiverNumber && self.Branding == other.Branding && self.CallHandle == other.CallHandle && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FAX_SEND {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FAX_SEND {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FAX_SERVER_APIVERSION_ENUM(pub i32);
pub const fsAPI_VERSION_0: FAX_SERVER_APIVERSION_ENUM = FAX_SERVER_APIVERSION_ENUM(0i32);
pub const fsAPI_VERSION_1: FAX_SERVER_APIVERSION_ENUM = FAX_SERVER_APIVERSION_ENUM(65536i32);
pub const fsAPI_VERSION_2: FAX_SERVER_APIVERSION_ENUM = FAX_SERVER_APIVERSION_ENUM(131072i32);
pub const fsAPI_VERSION_3: FAX_SERVER_APIVERSION_ENUM = FAX_SERVER_APIVERSION_ENUM(196608i32);
impl ::std::convert::From<i32> for FAX_SERVER_APIVERSION_ENUM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FAX_SERVER_APIVERSION_ENUM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FAX_SERVER_EVENTS_TYPE_ENUM(pub i32);
pub const fsetNONE: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(0i32);
pub const fsetIN_QUEUE: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(1i32);
pub const fsetOUT_QUEUE: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(2i32);
pub const fsetCONFIG: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(4i32);
pub const fsetACTIVITY: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(8i32);
pub const fsetQUEUE_STATE: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(16i32);
pub const fsetIN_ARCHIVE: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(32i32);
pub const fsetOUT_ARCHIVE: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(64i32);
pub const fsetFXSSVC_ENDED: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(128i32);
pub const fsetDEVICE_STATUS: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(256i32);
pub const fsetINCOMING_CALL: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(512i32);
impl ::std::convert::From<i32> for FAX_SERVER_EVENTS_TYPE_ENUM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FAX_SERVER_EVENTS_TYPE_ENUM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FAX_SMTP_AUTHENTICATION_TYPE_ENUM(pub i32);
pub const fsatANONYMOUS: FAX_SMTP_AUTHENTICATION_TYPE_ENUM = FAX_SMTP_AUTHENTICATION_TYPE_ENUM(0i32);
pub const fsatBASIC: FAX_SMTP_AUTHENTICATION_TYPE_ENUM = FAX_SMTP_AUTHENTICATION_TYPE_ENUM(1i32);
pub const fsatNTLM: FAX_SMTP_AUTHENTICATION_TYPE_ENUM = FAX_SMTP_AUTHENTICATION_TYPE_ENUM(2i32);
impl ::std::convert::From<i32> for FAX_SMTP_AUTHENTICATION_TYPE_ENUM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FAX_SMTP_AUTHENTICATION_TYPE_ENUM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FAX_TIME {
    pub Hour: u16,
    pub Minute: u16,
}
impl FAX_TIME {}
impl ::std::default::Default for FAX_TIME {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FAX_TIME {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FAX_TIME").field("Hour", &self.Hour).field("Minute", &self.Minute).finish()
    }
}
impl ::std::cmp::PartialEq for FAX_TIME {
    fn eq(&self, other: &Self) -> bool {
        self.Hour == other.Hour && self.Minute == other.Minute
    }
}
impl ::std::cmp::Eq for FAX_TIME {}
unsafe impl ::windows::runtime::Abi for FAX_TIME {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FEI_ABORTING: u32 = 15u32;
pub const FEI_ANSWERED: u32 = 21u32;
pub const FEI_BAD_ADDRESS: u32 = 7u32;
pub const FEI_BUSY: u32 = 5u32;
pub const FEI_CALL_BLACKLISTED: u32 = 13u32;
pub const FEI_CALL_DELAYED: u32 = 12u32;
pub const FEI_COMPLETED: u32 = 4u32;
pub const FEI_DELETED: u32 = 23u32;
pub const FEI_DIALING: u32 = 1u32;
pub const FEI_DISCONNECTED: u32 = 9u32;
pub const FEI_FATAL_ERROR: u32 = 10u32;
pub const FEI_FAXSVC_ENDED: u32 = 20u32;
pub const FEI_FAXSVC_STARTED: u32 = 27u32;
pub const FEI_HANDLED: u32 = 26u32;
pub const FEI_IDLE: u32 = 19u32;
pub const FEI_INITIALIZING: u32 = 24u32;
pub const FEI_JOB_QUEUED: u32 = 22u32;
pub const FEI_LINE_UNAVAILABLE: u32 = 25u32;
pub const FEI_MODEM_POWERED_OFF: u32 = 18u32;
pub const FEI_MODEM_POWERED_ON: u32 = 17u32;
pub const FEI_NEVENTS: u32 = 27u32;
pub const FEI_NOT_FAX_CALL: u32 = 11u32;
pub const FEI_NO_ANSWER: u32 = 6u32;
pub const FEI_NO_DIAL_TONE: u32 = 8u32;
pub const FEI_RECEIVING: u32 = 3u32;
pub const FEI_RINGING: u32 = 14u32;
pub const FEI_ROUTING: u32 = 16u32;
pub const FEI_SENDING: u32 = 2u32;
pub const FPF_RECEIVE: u32 = 1u32;
pub const FPF_SEND: u32 = 2u32;
pub const FPF_VIRTUAL: u32 = 4u32;
pub const FPS_ABORTING: u32 = 538968064u32;
pub const FPS_ANSWERED: u32 = 545259520u32;
pub const FPS_AVAILABLE: u32 = 537919488u32;
pub const FPS_BAD_ADDRESS: u32 = 536871168u32;
pub const FPS_BUSY: u32 = 536870976u32;
pub const FPS_CALL_BLACKLISTED: u32 = 536887296u32;
pub const FPS_CALL_DELAYED: u32 = 536879104u32;
pub const FPS_COMPLETED: u32 = 536870920u32;
pub const FPS_DIALING: u32 = 536870913u32;
pub const FPS_DISCONNECTED: u32 = 536871936u32;
pub const FPS_FATAL_ERROR: u32 = 536872960u32;
pub const FPS_HANDLED: u32 = 536870928u32;
pub const FPS_INITIALIZING: u32 = 536903680u32;
pub const FPS_NOT_FAX_CALL: u32 = 536875008u32;
pub const FPS_NO_ANSWER: u32 = 536871040u32;
pub const FPS_NO_DIAL_TONE: u32 = 536871424u32;
pub const FPS_OFFLINE: u32 = 536936448u32;
pub const FPS_RECEIVING: u32 = 536870916u32;
pub const FPS_RINGING: u32 = 537001984u32;
pub const FPS_ROUTING: u32 = 541065216u32;
pub const FPS_SENDING: u32 = 536870914u32;
pub const FPS_UNAVAILABLE: u32 = 536870944u32;
pub const FS_ANSWERED: u32 = 545259520u32;
pub const FS_BAD_ADDRESS: u32 = 536871168u32;
pub const FS_BUSY: u32 = 536870976u32;
pub const FS_CALL_BLACKLISTED: u32 = 536887296u32;
pub const FS_CALL_DELAYED: u32 = 536879104u32;
pub const FS_COMPLETED: u32 = 536870920u32;
pub const FS_DIALING: u32 = 536870913u32;
pub const FS_DISCONNECTED: u32 = 536871936u32;
pub const FS_FATAL_ERROR: u32 = 536872960u32;
pub const FS_HANDLED: u32 = 536870928u32;
pub const FS_INITIALIZING: u32 = 536870912u32;
pub const FS_LINE_UNAVAILABLE: u32 = 536870944u32;
pub const FS_NOT_FAX_CALL: u32 = 536875008u32;
pub const FS_NO_ANSWER: u32 = 536871040u32;
pub const FS_NO_DIAL_TONE: u32 = 536871424u32;
pub const FS_RECEIVING: u32 = 536870916u32;
pub const FS_TRANSMITTING: u32 = 536870914u32;
pub const FS_USER_ABORT: u32 = 538968064u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxAbort<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, jobid: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxAbort(faxhandle: super::super::Foundation::HANDLE, jobid: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxAbort(faxhandle.into_param().abi(), ::std::mem::transmute(jobid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxAccessCheck<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, accessmask: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxAccessCheck(faxhandle: super::super::Foundation::HANDLE, accessmask: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxAccessCheck(faxhandle.into_param().abi(), ::std::mem::transmute(accessmask)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const FaxAccount: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2816500863, 17700, 17508, [165, 109, 185, 254, 102, 111, 113, 94]);
pub const FaxAccountFolders: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2235141961, 49204, 19007, [130, 28, 219, 125, 104, 94, 129, 41]);
pub const FaxAccountIncomingArchive: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(347291061, 19520, 20175, [158, 248, 163, 96, 203, 232, 9, 237]);
pub const FaxAccountIncomingQueue: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2614059156, 46298, 17908, [184, 214, 221, 235, 33, 134, 101, 44]);
pub const FaxAccountOutgoingArchive: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2233367285, 17210, 18233, [162, 223, 173, 36, 92, 44, 185, 142]);
pub const FaxAccountOutgoingQueue: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4276940539, 49481, 18618, [186, 184, 183, 145, 225, 1, 246, 47]);
pub const FaxAccountSet: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4223810635, 31200, 17041, [188, 86, 193, 46, 37, 59, 191, 58]);
pub const FaxAccounts: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3659502762, 60972, 18368, [143, 79, 42, 33, 112, 117, 183, 110]);
pub const FaxActivity: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3488570638, 59469, 17966, [170, 187, 135, 211, 30, 176, 79, 239]);
pub const FaxActivityLogging: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4037028174, 15293, 18616, [143, 19, 140, 89, 26, 85, 189, 188]);
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxClose<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxClose(faxhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxClose(faxhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxCompleteJobParamsA(jobparams: *mut *mut FAX_JOB_PARAMA, coverpageinfo: *mut *mut FAX_COVERPAGE_INFOA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxCompleteJobParamsA(jobparams: *mut *mut FAX_JOB_PARAMA, coverpageinfo: *mut *mut FAX_COVERPAGE_INFOA) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxCompleteJobParamsA(::std::mem::transmute(jobparams), ::std::mem::transmute(coverpageinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxCompleteJobParamsW(jobparams: *mut *mut FAX_JOB_PARAMW, coverpageinfo: *mut *mut FAX_COVERPAGE_INFOW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxCompleteJobParamsW(jobparams: *mut *mut FAX_JOB_PARAMW, coverpageinfo: *mut *mut FAX_COVERPAGE_INFOW) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxCompleteJobParamsW(::std::mem::transmute(jobparams), ::std::mem::transmute(coverpageinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const FaxConfiguration: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1482109551, 59315, 16807, [156, 25, 169, 27, 70, 62, 45, 86]);
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxConnectFaxServerA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(machinename: Param0, faxhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxConnectFaxServerA(machinename: super::super::Foundation::PSTR, faxhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxConnectFaxServerA(machinename.into_param().abi(), ::std::mem::transmute(faxhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxConnectFaxServerW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(machinename: Param0, faxhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxConnectFaxServerW(machinename: super::super::Foundation::PWSTR, faxhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxConnectFaxServerW(machinename.into_param().abi(), ::std::mem::transmute(faxhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const FaxDevice: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1508091314, 54902, 18507, [166, 222, 114, 11, 250, 137, 181, 175]);
pub const FaxDeviceIds: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3452254698, 29303, 17934, [141, 224, 72, 160, 165, 118, 13, 31]);
pub const FaxDeviceProvider: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(399448739, 62955, 18506, [156, 154, 68, 64, 165, 186, 171, 252]);
pub const FaxDeviceProviders: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3952076648, 34650, 20319, [130, 197, 3, 242, 58, 172, 27, 215]);
pub const FaxDevices: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1435099790, 9163, 18713, [136, 8, 230, 16, 24, 70, 232, 13]);
pub const FaxDocument: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(255827857, 51256, 16734, [164, 243, 62, 130, 140, 164, 69, 224]);
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxEnableRoutingMethodA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(faxporthandle: Param0, routingguid: Param1, enabled: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxEnableRoutingMethodA(faxporthandle: super::super::Foundation::HANDLE, routingguid: super::super::Foundation::PSTR, enabled: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxEnableRoutingMethodA(faxporthandle.into_param().abi(), routingguid.into_param().abi(), enabled.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxEnableRoutingMethodW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(faxporthandle: Param0, routingguid: Param1, enabled: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxEnableRoutingMethodW(faxporthandle: super::super::Foundation::HANDLE, routingguid: super::super::Foundation::PWSTR, enabled: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxEnableRoutingMethodW(faxporthandle.into_param().abi(), routingguid.into_param().abi(), enabled.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxEnumGlobalRoutingInfoA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, routinginfo: *mut *mut FAX_GLOBAL_ROUTING_INFOA, methodsreturned: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxEnumGlobalRoutingInfoA(faxhandle: super::super::Foundation::HANDLE, routinginfo: *mut *mut FAX_GLOBAL_ROUTING_INFOA, methodsreturned: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxEnumGlobalRoutingInfoA(faxhandle.into_param().abi(), ::std::mem::transmute(routinginfo), ::std::mem::transmute(methodsreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxEnumGlobalRoutingInfoW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, routinginfo: *mut *mut FAX_GLOBAL_ROUTING_INFOW, methodsreturned: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxEnumGlobalRoutingInfoW(faxhandle: super::super::Foundation::HANDLE, routinginfo: *mut *mut FAX_GLOBAL_ROUTING_INFOW, methodsreturned: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxEnumGlobalRoutingInfoW(faxhandle.into_param().abi(), ::std::mem::transmute(routinginfo), ::std::mem::transmute(methodsreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxEnumJobsA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, jobentry: *mut *mut FAX_JOB_ENTRYA, jobsreturned: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxEnumJobsA(faxhandle: super::super::Foundation::HANDLE, jobentry: *mut *mut FAX_JOB_ENTRYA, jobsreturned: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxEnumJobsA(faxhandle.into_param().abi(), ::std::mem::transmute(jobentry), ::std::mem::transmute(jobsreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxEnumJobsW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, jobentry: *mut *mut FAX_JOB_ENTRYW, jobsreturned: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxEnumJobsW(faxhandle: super::super::Foundation::HANDLE, jobentry: *mut *mut FAX_JOB_ENTRYW, jobsreturned: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxEnumJobsW(faxhandle.into_param().abi(), ::std::mem::transmute(jobentry), ::std::mem::transmute(jobsreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxEnumPortsA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, portinfo: *mut *mut FAX_PORT_INFOA, portsreturned: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxEnumPortsA(faxhandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOA, portsreturned: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxEnumPortsA(faxhandle.into_param().abi(), ::std::mem::transmute(portinfo), ::std::mem::transmute(portsreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxEnumPortsW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, portinfo: *mut *mut FAX_PORT_INFOW, portsreturned: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxEnumPortsW(faxhandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOW, portsreturned: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxEnumPortsW(faxhandle.into_param().abi(), ::std::mem::transmute(portinfo), ::std::mem::transmute(portsreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxEnumRoutingMethodsA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(faxporthandle: Param0, routingmethod: *mut *mut FAX_ROUTING_METHODA, methodsreturned: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxEnumRoutingMethodsA(faxporthandle: super::super::Foundation::HANDLE, routingmethod: *mut *mut FAX_ROUTING_METHODA, methodsreturned: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxEnumRoutingMethodsA(faxporthandle.into_param().abi(), ::std::mem::transmute(routingmethod), ::std::mem::transmute(methodsreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxEnumRoutingMethodsW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(faxporthandle: Param0, routingmethod: *mut *mut FAX_ROUTING_METHODW, methodsreturned: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxEnumRoutingMethodsW(faxporthandle: super::super::Foundation::HANDLE, routingmethod: *mut *mut FAX_ROUTING_METHODW, methodsreturned: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxEnumRoutingMethodsW(faxporthandle.into_param().abi(), ::std::mem::transmute(routingmethod), ::std::mem::transmute(methodsreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const FaxEventLogging: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2793736496, 41206, 19055, [149, 183, 219, 46, 191, 61, 2, 227]);
pub const FaxFolders: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3276935639, 22390, 18635, [175, 68, 195, 27, 227, 178, 207, 229]);
#[inline]
pub unsafe fn FaxFreeBuffer(buffer: *mut ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxFreeBuffer(buffer: *mut ::std::ffi::c_void);
        }
        ::std::mem::transmute(FaxFreeBuffer(::std::mem::transmute(buffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetConfigurationA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, faxconfig: *mut *mut FAX_CONFIGURATIONA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxGetConfigurationA(faxhandle: super::super::Foundation::HANDLE, faxconfig: *mut *mut FAX_CONFIGURATIONA) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxGetConfigurationA(faxhandle.into_param().abi(), ::std::mem::transmute(faxconfig)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetConfigurationW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, faxconfig: *mut *mut FAX_CONFIGURATIONW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxGetConfigurationW(faxhandle: super::super::Foundation::HANDLE, faxconfig: *mut *mut FAX_CONFIGURATIONW) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxGetConfigurationW(faxhandle.into_param().abi(), ::std::mem::transmute(faxconfig)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetDeviceStatusA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(faxporthandle: Param0, devicestatus: *mut *mut FAX_DEVICE_STATUSA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxGetDeviceStatusA(faxporthandle: super::super::Foundation::HANDLE, devicestatus: *mut *mut FAX_DEVICE_STATUSA) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxGetDeviceStatusA(faxporthandle.into_param().abi(), ::std::mem::transmute(devicestatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetDeviceStatusW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(faxporthandle: Param0, devicestatus: *mut *mut FAX_DEVICE_STATUSW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxGetDeviceStatusW(faxporthandle: super::super::Foundation::HANDLE, devicestatus: *mut *mut FAX_DEVICE_STATUSW) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxGetDeviceStatusW(faxporthandle.into_param().abi(), ::std::mem::transmute(devicestatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetJobA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, jobid: u32, jobentry: *mut *mut FAX_JOB_ENTRYA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxGetJobA(faxhandle: super::super::Foundation::HANDLE, jobid: u32, jobentry: *mut *mut FAX_JOB_ENTRYA) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxGetJobA(faxhandle.into_param().abi(), ::std::mem::transmute(jobid), ::std::mem::transmute(jobentry)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetJobW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, jobid: u32, jobentry: *mut *mut FAX_JOB_ENTRYW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxGetJobW(faxhandle: super::super::Foundation::HANDLE, jobid: u32, jobentry: *mut *mut FAX_JOB_ENTRYW) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxGetJobW(faxhandle.into_param().abi(), ::std::mem::transmute(jobid), ::std::mem::transmute(jobentry)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetLoggingCategoriesA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, categories: *mut *mut FAX_LOG_CATEGORYA, numbercategories: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxGetLoggingCategoriesA(faxhandle: super::super::Foundation::HANDLE, categories: *mut *mut FAX_LOG_CATEGORYA, numbercategories: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxGetLoggingCategoriesA(faxhandle.into_param().abi(), ::std::mem::transmute(categories), ::std::mem::transmute(numbercategories)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetLoggingCategoriesW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, categories: *mut *mut FAX_LOG_CATEGORYW, numbercategories: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxGetLoggingCategoriesW(faxhandle: super::super::Foundation::HANDLE, categories: *mut *mut FAX_LOG_CATEGORYW, numbercategories: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxGetLoggingCategoriesW(faxhandle.into_param().abi(), ::std::mem::transmute(categories), ::std::mem::transmute(numbercategories)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetPageData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, jobid: u32, buffer: *mut *mut u8, buffersize: *mut u32, imagewidth: *mut u32, imageheight: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxGetPageData(faxhandle: super::super::Foundation::HANDLE, jobid: u32, buffer: *mut *mut u8, buffersize: *mut u32, imagewidth: *mut u32, imageheight: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxGetPageData(faxhandle.into_param().abi(), ::std::mem::transmute(jobid), ::std::mem::transmute(buffer), ::std::mem::transmute(buffersize), ::std::mem::transmute(imagewidth), ::std::mem::transmute(imageheight)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetPortA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(faxporthandle: Param0, portinfo: *mut *mut FAX_PORT_INFOA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxGetPortA(faxporthandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOA) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxGetPortA(faxporthandle.into_param().abi(), ::std::mem::transmute(portinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetPortW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(faxporthandle: Param0, portinfo: *mut *mut FAX_PORT_INFOW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxGetPortW(faxporthandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOW) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxGetPortW(faxporthandle.into_param().abi(), ::std::mem::transmute(portinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetRoutingInfoA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(faxporthandle: Param0, routingguid: Param1, routinginfobuffer: *mut *mut u8, routinginfobuffersize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxGetRoutingInfoA(faxporthandle: super::super::Foundation::HANDLE, routingguid: super::super::Foundation::PSTR, routinginfobuffer: *mut *mut u8, routinginfobuffersize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxGetRoutingInfoA(faxporthandle.into_param().abi(), routingguid.into_param().abi(), ::std::mem::transmute(routinginfobuffer), ::std::mem::transmute(routinginfobuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxGetRoutingInfoW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(faxporthandle: Param0, routingguid: Param1, routinginfobuffer: *mut *mut u8, routinginfobuffersize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxGetRoutingInfoW(faxporthandle: super::super::Foundation::HANDLE, routingguid: super::super::Foundation::PWSTR, routinginfobuffer: *mut *mut u8, routinginfobuffersize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxGetRoutingInfoW(faxporthandle.into_param().abi(), routingguid.into_param().abi(), ::std::mem::transmute(routinginfobuffer), ::std::mem::transmute(routinginfobuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const FaxInboundRouting: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3892463853, 44389, 16920, [129, 8, 153, 25, 36, 212, 231, 237]);
pub const FaxInboundRoutingExtension: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(494795601, 29191, 17462, [160, 217, 36, 227, 46, 229, 105, 136]);
pub const FaxInboundRoutingExtensions: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(412764397, 25148, 19469, [128, 242, 214, 108, 123, 158, 254, 194]);
pub const FaxInboundRoutingMethod: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1268766556, 404, 19314, [156, 229, 2, 168, 32, 90, 199, 212]);
pub const FaxInboundRoutingMethods: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(637319018, 46928, 19330, [146, 102, 251, 187, 174, 137, 34, 186]);
pub const FaxIncomingArchive: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2217133418, 13729, 19567, [175, 147, 252, 149, 36, 34, 226, 194]);
pub const FaxIncomingJob: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3295875564, 44594, 16824, [174, 75, 62, 174, 6, 41, 208, 201]);
pub const FaxIncomingJobs: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2713422403, 34918, 20407, [161, 93, 98, 102, 200, 117, 165, 204]);
pub const FaxIncomingMessage: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(422771959, 40259, 19802, [137, 255, 3, 134, 27, 50, 23, 54]);
pub const FaxIncomingMessageIterator: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1619583448, 16328, 17858, [135, 177, 144, 154, 41, 96, 126, 169]);
pub const FaxIncomingQueue: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1762858775, 62449, 16611, [128, 157, 166, 203, 247, 189, 133, 229]);
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxInitializeEventQueue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(faxhandle: Param0, completionport: Param1, completionkey: usize, hwnd: Param3, messagestart: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxInitializeEventQueue(faxhandle: super::super::Foundation::HANDLE, completionport: super::super::Foundation::HANDLE, completionkey: usize, hwnd: super::super::Foundation::HWND, messagestart: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxInitializeEventQueue(faxhandle.into_param().abi(), completionport.into_param().abi(), ::std::mem::transmute(completionkey), hwnd.into_param().abi(), ::std::mem::transmute(messagestart)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const FaxJobStatus: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2079466228, 48781, 17455, [132, 29, 97, 50, 116, 36, 35, 187]);
pub const FaxLoggingOptions: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(469364390, 60640, 18309, [161, 139, 222, 86, 233, 238, 249, 106]);
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxOpenPort<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, deviceid: u32, flags: u32, faxporthandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxOpenPort(faxhandle: super::super::Foundation::HANDLE, deviceid: u32, flags: u32, faxporthandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxOpenPort(faxhandle.into_param().abi(), ::std::mem::transmute(deviceid), ::std::mem::transmute(flags), ::std::mem::transmute(faxporthandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const FaxOutboundRouting: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3357227102, 47209, 19197, [134, 192, 97, 100, 152, 237, 155, 226]);
pub const FaxOutboundRoutingGroup: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(34862048, 26513, 19831, [162, 113, 4, 210, 53, 124, 80, 214]);
pub const FaxOutboundRoutingGroups: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3435045285, 58036, 19287, [148, 33, 176, 75, 98, 137, 70, 75]);
pub const FaxOutboundRoutingRule: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1699344063, 2257, 18266, [130, 139, 59, 241, 5, 149, 47, 160]);
pub const FaxOutboundRoutingRules: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3548757706, 58916, 17523, [191, 170, 159, 64, 0, 131, 31, 84]);
pub const FaxOutgoingArchive: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1136821251, 57423, 18253, [153, 12, 185, 70, 105, 20, 143, 89]);
pub const FaxOutgoingJob: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1908097692, 3833, 18709, [190, 197, 165, 216, 151, 163, 233, 36]);
pub const FaxOutgoingJobs: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2462001772, 14270, 17402, [163, 125, 203, 14, 95, 117, 59, 53]);
pub const FaxOutgoingMessage: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2444534648, 19160, 19183, [164, 220, 151, 217, 110, 147, 154, 58]);
pub const FaxOutgoingMessageIterator: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2318542032, 54027, 18910, [152, 19, 203, 56, 87, 144, 251, 187]);
pub const FaxOutgoingQueue: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1948325534, 35907, 19213, [187, 22, 100, 92, 143, 164, 3, 87]);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn FaxPrintCoverPageA(faxcontextinfo: *const FAX_CONTEXT_INFOA, coverpageinfo: *const FAX_COVERPAGE_INFOA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxPrintCoverPageA(faxcontextinfo: *const FAX_CONTEXT_INFOA, coverpageinfo: *const FAX_COVERPAGE_INFOA) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxPrintCoverPageA(::std::mem::transmute(faxcontextinfo), ::std::mem::transmute(coverpageinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn FaxPrintCoverPageW(faxcontextinfo: *const FAX_CONTEXT_INFOW, coverpageinfo: *const FAX_COVERPAGE_INFOW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxPrintCoverPageW(faxcontextinfo: *const FAX_CONTEXT_INFOW, coverpageinfo: *const FAX_COVERPAGE_INFOW) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxPrintCoverPageW(::std::mem::transmute(faxcontextinfo), ::std::mem::transmute(coverpageinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const FaxReceiptOptions: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1770145915, 8827, 19606, [166, 28, 36, 131, 72, 176, 90, 182]);
pub const FaxRecipient: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1623143169, 32248, 19416, [145, 72, 123, 88, 1, 249, 239, 223]);
pub const FaxRecipients: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3936083795, 4265, 19791, [160, 103, 99, 200, 248, 79, 1, 176]);
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxRegisterRoutingExtensionW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(
    faxhandle: Param0,
    extensionname: Param1,
    friendlyname: Param2,
    imagename: Param3,
    callback: ::std::option::Option<PFAX_ROUTING_INSTALLATION_CALLBACKW>,
    context: *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxRegisterRoutingExtensionW(faxhandle: super::super::Foundation::HANDLE, extensionname: super::super::Foundation::PWSTR, friendlyname: super::super::Foundation::PWSTR, imagename: super::super::Foundation::PWSTR, callback: ::windows::runtime::RawPtr, context: *mut ::std::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxRegisterRoutingExtensionW(faxhandle.into_param().abi(), extensionname.into_param().abi(), friendlyname.into_param().abi(), imagename.into_param().abi(), ::std::mem::transmute(callback), ::std::mem::transmute(context)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxRegisterServiceProviderW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(deviceprovider: Param0, friendlyname: Param1, imagename: Param2, tspname: Param3) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxRegisterServiceProviderW(deviceprovider: super::super::Foundation::PWSTR, friendlyname: super::super::Foundation::PWSTR, imagename: super::super::Foundation::PWSTR, tspname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxRegisterServiceProviderW(deviceprovider.into_param().abi(), friendlyname.into_param().abi(), imagename.into_param().abi(), tspname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const FaxSecurity: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(281337310, 44016, 17375, [150, 79, 127, 58, 194, 26, 76, 123]);
pub const FaxSecurity2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1935413832, 60553, 19504, [161, 39, 101, 110, 146, 227, 196, 234]);
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSendDocumentA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(faxhandle: Param0, filename: Param1, jobparams: *mut FAX_JOB_PARAMA, coverpageinfo: *const FAX_COVERPAGE_INFOA, faxjobid: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxSendDocumentA(faxhandle: super::super::Foundation::HANDLE, filename: super::super::Foundation::PSTR, jobparams: *mut FAX_JOB_PARAMA, coverpageinfo: *const FAX_COVERPAGE_INFOA, faxjobid: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxSendDocumentA(faxhandle.into_param().abi(), filename.into_param().abi(), ::std::mem::transmute(jobparams), ::std::mem::transmute(coverpageinfo), ::std::mem::transmute(faxjobid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSendDocumentForBroadcastA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(faxhandle: Param0, filename: Param1, faxjobid: *mut u32, faxrecipientcallback: ::std::option::Option<PFAX_RECIPIENT_CALLBACKA>, context: *mut ::std::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxSendDocumentForBroadcastA(faxhandle: super::super::Foundation::HANDLE, filename: super::super::Foundation::PSTR, faxjobid: *mut u32, faxrecipientcallback: ::windows::runtime::RawPtr, context: *mut ::std::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxSendDocumentForBroadcastA(faxhandle.into_param().abi(), filename.into_param().abi(), ::std::mem::transmute(faxjobid), ::std::mem::transmute(faxrecipientcallback), ::std::mem::transmute(context)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSendDocumentForBroadcastW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(faxhandle: Param0, filename: Param1, faxjobid: *mut u32, faxrecipientcallback: ::std::option::Option<PFAX_RECIPIENT_CALLBACKW>, context: *mut ::std::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxSendDocumentForBroadcastW(faxhandle: super::super::Foundation::HANDLE, filename: super::super::Foundation::PWSTR, faxjobid: *mut u32, faxrecipientcallback: ::windows::runtime::RawPtr, context: *mut ::std::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxSendDocumentForBroadcastW(faxhandle.into_param().abi(), filename.into_param().abi(), ::std::mem::transmute(faxjobid), ::std::mem::transmute(faxrecipientcallback), ::std::mem::transmute(context)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSendDocumentW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(faxhandle: Param0, filename: Param1, jobparams: *mut FAX_JOB_PARAMW, coverpageinfo: *const FAX_COVERPAGE_INFOW, faxjobid: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxSendDocumentW(faxhandle: super::super::Foundation::HANDLE, filename: super::super::Foundation::PWSTR, jobparams: *mut FAX_JOB_PARAMW, coverpageinfo: *const FAX_COVERPAGE_INFOW, faxjobid: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxSendDocumentW(faxhandle.into_param().abi(), filename.into_param().abi(), ::std::mem::transmute(jobparams), ::std::mem::transmute(coverpageinfo), ::std::mem::transmute(faxjobid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const FaxSender: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(643663056, 6224, 17248, [183, 200, 117, 139, 187, 95, 11, 150]);
pub const FaxServer: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3450383536, 36085, 20332, [155, 162, 89, 49, 212, 12, 140, 174]);
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSetConfigurationA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, faxconfig: *const FAX_CONFIGURATIONA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxSetConfigurationA(faxhandle: super::super::Foundation::HANDLE, faxconfig: *const FAX_CONFIGURATIONA) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxSetConfigurationA(faxhandle.into_param().abi(), ::std::mem::transmute(faxconfig)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSetConfigurationW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, faxconfig: *const FAX_CONFIGURATIONW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxSetConfigurationW(faxhandle: super::super::Foundation::HANDLE, faxconfig: *const FAX_CONFIGURATIONW) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxSetConfigurationW(faxhandle.into_param().abi(), ::std::mem::transmute(faxconfig)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSetGlobalRoutingInfoA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, routinginfo: *const FAX_GLOBAL_ROUTING_INFOA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxSetGlobalRoutingInfoA(faxhandle: super::super::Foundation::HANDLE, routinginfo: *const FAX_GLOBAL_ROUTING_INFOA) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxSetGlobalRoutingInfoA(faxhandle.into_param().abi(), ::std::mem::transmute(routinginfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSetGlobalRoutingInfoW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, routinginfo: *const FAX_GLOBAL_ROUTING_INFOW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxSetGlobalRoutingInfoW(faxhandle: super::super::Foundation::HANDLE, routinginfo: *const FAX_GLOBAL_ROUTING_INFOW) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxSetGlobalRoutingInfoW(faxhandle.into_param().abi(), ::std::mem::transmute(routinginfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSetJobA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, jobid: u32, command: u32, jobentry: *const FAX_JOB_ENTRYA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxSetJobA(faxhandle: super::super::Foundation::HANDLE, jobid: u32, command: u32, jobentry: *const FAX_JOB_ENTRYA) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxSetJobA(faxhandle.into_param().abi(), ::std::mem::transmute(jobid), ::std::mem::transmute(command), ::std::mem::transmute(jobentry)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSetJobW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, jobid: u32, command: u32, jobentry: *const FAX_JOB_ENTRYW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxSetJobW(faxhandle: super::super::Foundation::HANDLE, jobid: u32, command: u32, jobentry: *const FAX_JOB_ENTRYW) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxSetJobW(faxhandle.into_param().abi(), ::std::mem::transmute(jobid), ::std::mem::transmute(command), ::std::mem::transmute(jobentry)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSetLoggingCategoriesA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, categories: *const FAX_LOG_CATEGORYA, numbercategories: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxSetLoggingCategoriesA(faxhandle: super::super::Foundation::HANDLE, categories: *const FAX_LOG_CATEGORYA, numbercategories: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxSetLoggingCategoriesA(faxhandle.into_param().abi(), ::std::mem::transmute(categories), ::std::mem::transmute(numbercategories)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSetLoggingCategoriesW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(faxhandle: Param0, categories: *const FAX_LOG_CATEGORYW, numbercategories: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxSetLoggingCategoriesW(faxhandle: super::super::Foundation::HANDLE, categories: *const FAX_LOG_CATEGORYW, numbercategories: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxSetLoggingCategoriesW(faxhandle.into_param().abi(), ::std::mem::transmute(categories), ::std::mem::transmute(numbercategories)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSetPortA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(faxporthandle: Param0, portinfo: *const FAX_PORT_INFOA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxSetPortA(faxporthandle: super::super::Foundation::HANDLE, portinfo: *const FAX_PORT_INFOA) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxSetPortA(faxporthandle.into_param().abi(), ::std::mem::transmute(portinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSetPortW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(faxporthandle: Param0, portinfo: *const FAX_PORT_INFOW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxSetPortW(faxporthandle: super::super::Foundation::HANDLE, portinfo: *const FAX_PORT_INFOW) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxSetPortW(faxporthandle.into_param().abi(), ::std::mem::transmute(portinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSetRoutingInfoA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(faxporthandle: Param0, routingguid: Param1, routinginfobuffer: *const u8, routinginfobuffersize: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxSetRoutingInfoA(faxporthandle: super::super::Foundation::HANDLE, routingguid: super::super::Foundation::PSTR, routinginfobuffer: *const u8, routinginfobuffersize: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxSetRoutingInfoA(faxporthandle.into_param().abi(), routingguid.into_param().abi(), ::std::mem::transmute(routinginfobuffer), ::std::mem::transmute(routinginfobuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxSetRoutingInfoW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(faxporthandle: Param0, routingguid: Param1, routinginfobuffer: *const u8, routinginfobuffersize: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxSetRoutingInfoW(faxporthandle: super::super::Foundation::HANDLE, routingguid: super::super::Foundation::PWSTR, routinginfobuffer: *const u8, routinginfobuffersize: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxSetRoutingInfoW(faxporthandle.into_param().abi(), routingguid.into_param().abi(), ::std::mem::transmute(routinginfobuffer), ::std::mem::transmute(routinginfobuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn FaxStartPrintJobA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(printername: Param0, printinfo: *const FAX_PRINT_INFOA, faxjobid: *mut u32, faxcontextinfo: *mut FAX_CONTEXT_INFOA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxStartPrintJobA(printername: super::super::Foundation::PSTR, printinfo: *const FAX_PRINT_INFOA, faxjobid: *mut u32, faxcontextinfo: *mut FAX_CONTEXT_INFOA) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxStartPrintJobA(printername.into_param().abi(), ::std::mem::transmute(printinfo), ::std::mem::transmute(faxjobid), ::std::mem::transmute(faxcontextinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn FaxStartPrintJobW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(printername: Param0, printinfo: *const FAX_PRINT_INFOW, faxjobid: *mut u32, faxcontextinfo: *mut FAX_CONTEXT_INFOW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxStartPrintJobW(printername: super::super::Foundation::PWSTR, printinfo: *const FAX_PRINT_INFOW, faxjobid: *mut u32, faxcontextinfo: *mut FAX_CONTEXT_INFOW) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxStartPrintJobW(printername.into_param().abi(), ::std::mem::transmute(printinfo), ::std::mem::transmute(faxjobid), ::std::mem::transmute(faxcontextinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaxUnregisterServiceProviderW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(deviceprovider: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaxUnregisterServiceProviderW(deviceprovider: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FaxUnregisterServiceProviderW(deviceprovider.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const GUID_DeviceArrivedLaunch: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1947049702, 28913, 4561, [173, 16, 0, 160, 36, 56, 173, 72]);
pub const GUID_STIUserDefined1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3222189973, 35950, 4562, [151, 122, 0, 0, 248, 122, 146, 111]);
pub const GUID_STIUserDefined2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3346721221, 35950, 4562, [151, 122, 0, 0, 248, 122, 146, 111]);
pub const GUID_STIUserDefined3: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3346721222, 35950, 4562, [151, 122, 0, 0, 248, 122, 146, 111]);
pub const GUID_ScanFaxImage: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3222189971, 35950, 4562, [151, 122, 0, 0, 248, 122, 146, 111]);
pub const GUID_ScanImage: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2797971221, 35950, 4562, [151, 122, 0, 0, 248, 122, 146, 111]);
pub const GUID_ScanPrintImage: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3024221221, 35950, 4562, [151, 122, 0, 0, 248, 122, 146, 111]);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxAccount(::windows::runtime::IUnknown);
impl IFaxAccount {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AccountName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Folders(&self) -> ::windows::runtime::Result<IFaxAccountFolders> {
        let mut result__: <IFaxAccountFolders as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxAccountFolders>(result__)
    }
    pub unsafe fn ListenToAccountEvents(&self, eventtypes: FAX_ACCOUNT_EVENTS_TYPE_ENUM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(eventtypes)).ok()
    }
    pub unsafe fn RegisteredEvents(&self) -> ::windows::runtime::Result<FAX_ACCOUNT_EVENTS_TYPE_ENUM> {
        let mut result__: <FAX_ACCOUNT_EVENTS_TYPE_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_ACCOUNT_EVENTS_TYPE_ENUM>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFaxAccount {
    type Vtable = IFaxAccount_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1750293299, 24004, 16518, [190, 38, 183, 111, 155, 113, 16, 6]);
}
impl ::std::convert::From<IFaxAccount> for ::windows::runtime::IUnknown {
    fn from(value: IFaxAccount) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxAccount> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxAccount) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxAccount {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxAccount {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxAccount> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxAccount) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxAccount> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxAccount) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxAccount {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxAccount {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxAccount_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstraccountname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfolders: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventtypes: FAX_ACCOUNT_EVENTS_TYPE_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pregisteredevents: *mut FAX_ACCOUNT_EVENTS_TYPE_ENUM) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxAccountFolders(::windows::runtime::IUnknown);
impl IFaxAccountFolders {
    pub unsafe fn OutgoingQueue(&self) -> ::windows::runtime::Result<IFaxAccountOutgoingQueue> {
        let mut result__: <IFaxAccountOutgoingQueue as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxAccountOutgoingQueue>(result__)
    }
    pub unsafe fn IncomingQueue(&self) -> ::windows::runtime::Result<IFaxAccountIncomingQueue> {
        let mut result__: <IFaxAccountIncomingQueue as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxAccountIncomingQueue>(result__)
    }
    pub unsafe fn IncomingArchive(&self) -> ::windows::runtime::Result<IFaxAccountIncomingArchive> {
        let mut result__: <IFaxAccountIncomingArchive as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxAccountIncomingArchive>(result__)
    }
    pub unsafe fn OutgoingArchive(&self) -> ::windows::runtime::Result<IFaxAccountOutgoingArchive> {
        let mut result__: <IFaxAccountOutgoingArchive as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxAccountOutgoingArchive>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFaxAccountFolders {
    type Vtable = IFaxAccountFolders_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1684273309, 9176, 18089, [143, 134, 196, 123, 119, 202, 121, 38]);
}
impl ::std::convert::From<IFaxAccountFolders> for ::windows::runtime::IUnknown {
    fn from(value: IFaxAccountFolders) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxAccountFolders> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxAccountFolders) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxAccountFolders {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxAccountFolders {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxAccountFolders> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxAccountFolders) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxAccountFolders> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxAccountFolders) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxAccountFolders {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxAccountFolders {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxAccountFolders_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxoutgoingqueue: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxincomingqueue: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxincomingarchive: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxoutgoingarchive: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxAccountIncomingArchive(::windows::runtime::IUnknown);
impl IFaxAccountIncomingArchive {
    pub unsafe fn SizeLow(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SizeHigh(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetMessages(&self, lprefetchsize: i32) -> ::windows::runtime::Result<IFaxIncomingMessageIterator> {
        let mut result__: <IFaxIncomingMessageIterator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(lprefetchsize), &mut result__).from_abi::<IFaxIncomingMessageIterator>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMessage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrmessageid: Param0) -> ::windows::runtime::Result<IFaxIncomingMessage> {
        let mut result__: <IFaxIncomingMessage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), bstrmessageid.into_param().abi(), &mut result__).from_abi::<IFaxIncomingMessage>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFaxAccountIncomingArchive {
    type Vtable = IFaxAccountIncomingArchive_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2829432559, 57558, 19182, [149, 92, 145, 98, 91, 236, 157, 180]);
}
impl ::std::convert::From<IFaxAccountIncomingArchive> for ::windows::runtime::IUnknown {
    fn from(value: IFaxAccountIncomingArchive) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxAccountIncomingArchive> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxAccountIncomingArchive) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxAccountIncomingArchive {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxAccountIncomingArchive {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxAccountIncomingArchive> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxAccountIncomingArchive) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxAccountIncomingArchive> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxAccountIncomingArchive) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxAccountIncomingArchive {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxAccountIncomingArchive {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxAccountIncomingArchive_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plsizelow: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plsizehigh: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lprefetchsize: i32, pfaxincomingmessageiterator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrmessageid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxincomingmessage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxAccountIncomingQueue(::windows::runtime::IUnknown);
impl IFaxAccountIncomingQueue {
    pub unsafe fn GetJobs(&self) -> ::windows::runtime::Result<IFaxIncomingJobs> {
        let mut result__: <IFaxIncomingJobs as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxIncomingJobs>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetJob<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrjobid: Param0) -> ::windows::runtime::Result<IFaxIncomingJob> {
        let mut result__: <IFaxIncomingJob as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), bstrjobid.into_param().abi(), &mut result__).from_abi::<IFaxIncomingJob>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFaxAccountIncomingQueue {
    type Vtable = IFaxAccountIncomingQueue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3709087122, 390, 19093, [160, 144, 203, 195, 234, 219, 166, 180]);
}
impl ::std::convert::From<IFaxAccountIncomingQueue> for ::windows::runtime::IUnknown {
    fn from(value: IFaxAccountIncomingQueue) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxAccountIncomingQueue> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxAccountIncomingQueue) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxAccountIncomingQueue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxAccountIncomingQueue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxAccountIncomingQueue> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxAccountIncomingQueue) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxAccountIncomingQueue> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxAccountIncomingQueue) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxAccountIncomingQueue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxAccountIncomingQueue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxAccountIncomingQueue_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxincomingjobs: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrjobid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxincomingjob: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxAccountNotify(::windows::runtime::IUnknown);
impl IFaxAccountNotify {}
unsafe impl ::windows::runtime::Interface for IFaxAccountNotify {
    type Vtable = IFaxAccountNotify_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(190733265, 47273, 18336, [163, 35, 239, 74, 41, 59, 160, 106]);
}
impl ::std::convert::From<IFaxAccountNotify> for ::windows::runtime::IUnknown {
    fn from(value: IFaxAccountNotify) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxAccountNotify> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxAccountNotify) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxAccountNotify {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxAccountNotify {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxAccountNotify> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxAccountNotify) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxAccountNotify> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxAccountNotify) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxAccountNotify {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxAccountNotify {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxAccountNotify_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxAccountOutgoingArchive(::windows::runtime::IUnknown);
impl IFaxAccountOutgoingArchive {
    pub unsafe fn SizeLow(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SizeHigh(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetMessages(&self, lprefetchsize: i32) -> ::windows::runtime::Result<IFaxOutgoingMessageIterator> {
        let mut result__: <IFaxOutgoingMessageIterator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(lprefetchsize), &mut result__).from_abi::<IFaxOutgoingMessageIterator>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMessage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrmessageid: Param0) -> ::windows::runtime::Result<IFaxOutgoingMessage> {
        let mut result__: <IFaxOutgoingMessage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), bstrmessageid.into_param().abi(), &mut result__).from_abi::<IFaxOutgoingMessage>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFaxAccountOutgoingArchive {
    type Vtable = IFaxAccountOutgoingArchive_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1415776109, 60436, 18719, [146, 110, 179, 206, 218, 94, 86, 98]);
}
impl ::std::convert::From<IFaxAccountOutgoingArchive> for ::windows::runtime::IUnknown {
    fn from(value: IFaxAccountOutgoingArchive) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxAccountOutgoingArchive> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxAccountOutgoingArchive) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxAccountOutgoingArchive {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxAccountOutgoingArchive {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxAccountOutgoingArchive> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxAccountOutgoingArchive) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxAccountOutgoingArchive> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxAccountOutgoingArchive) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxAccountOutgoingArchive {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxAccountOutgoingArchive {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxAccountOutgoingArchive_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plsizelow: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plsizehigh: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lprefetchsize: i32, pfaxoutgoingmessageiterator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrmessageid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxoutgoingmessage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxAccountOutgoingQueue(::windows::runtime::IUnknown);
impl IFaxAccountOutgoingQueue {
    pub unsafe fn GetJobs(&self) -> ::windows::runtime::Result<IFaxOutgoingJobs> {
        let mut result__: <IFaxOutgoingJobs as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxOutgoingJobs>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetJob<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrjobid: Param0) -> ::windows::runtime::Result<IFaxOutgoingJob> {
        let mut result__: <IFaxOutgoingJob as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), bstrjobid.into_param().abi(), &mut result__).from_abi::<IFaxOutgoingJob>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFaxAccountOutgoingQueue {
    type Vtable = IFaxAccountOutgoingQueue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(252978409, 61997, 17747, [183, 165, 13, 36, 189, 13, 126, 70]);
}
impl ::std::convert::From<IFaxAccountOutgoingQueue> for ::windows::runtime::IUnknown {
    fn from(value: IFaxAccountOutgoingQueue) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxAccountOutgoingQueue> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxAccountOutgoingQueue) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxAccountOutgoingQueue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxAccountOutgoingQueue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxAccountOutgoingQueue> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxAccountOutgoingQueue) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxAccountOutgoingQueue> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxAccountOutgoingQueue) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxAccountOutgoingQueue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxAccountOutgoingQueue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxAccountOutgoingQueue_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxoutgoingjobs: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrjobid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxoutgoingjob: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxAccountSet(::windows::runtime::IUnknown);
impl IFaxAccountSet {
    pub unsafe fn GetAccounts(&self) -> ::windows::runtime::Result<IFaxAccounts> {
        let mut result__: <IFaxAccounts as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxAccounts>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAccount<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstraccountname: Param0) -> ::windows::runtime::Result<IFaxAccount> {
        let mut result__: <IFaxAccount as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), bstraccountname.into_param().abi(), &mut result__).from_abi::<IFaxAccount>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddAccount<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstraccountname: Param0) -> ::windows::runtime::Result<IFaxAccount> {
        let mut result__: <IFaxAccount as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), bstraccountname.into_param().abi(), &mut result__).from_abi::<IFaxAccount>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoveAccount<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstraccountname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), bstraccountname.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFaxAccountSet {
    type Vtable = IFaxAccountSet_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1948842926, 33822, 18360, [134, 244, 34, 136, 148, 109, 202, 27]);
}
impl ::std::convert::From<IFaxAccountSet> for ::windows::runtime::IUnknown {
    fn from(value: IFaxAccountSet) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxAccountSet> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxAccountSet) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxAccountSet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxAccountSet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxAccountSet> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxAccountSet) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxAccountSet> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxAccountSet) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxAccountSet {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxAccountSet {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxAccountSet_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfaxaccounts: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstraccountname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxaccount: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstraccountname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxaccount: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstraccountname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxAccounts(::windows::runtime::IUnknown);
impl IFaxAccounts {
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Item<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, vindex: Param0) -> ::windows::runtime::Result<IFaxAccount> {
        let mut result__: <IFaxAccount as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), vindex.into_param().abi(), &mut result__).from_abi::<IFaxAccount>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFaxAccounts {
    type Vtable = IFaxAccounts_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2481619298, 35815, 17105, [174, 123, 236, 116, 226, 217, 137, 218]);
}
impl ::std::convert::From<IFaxAccounts> for ::windows::runtime::IUnknown {
    fn from(value: IFaxAccounts) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxAccounts> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxAccounts) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxAccounts {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxAccounts {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxAccounts> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxAccounts) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxAccounts> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxAccounts) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxAccounts {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxAccounts {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxAccounts_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vindex: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxaccount: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plcount: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxActivity(::windows::runtime::IUnknown);
impl IFaxActivity {
    pub unsafe fn IncomingMessages(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn RoutingMessages(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn OutgoingMessages(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn QueuedMessages(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFaxActivity {
    type Vtable = IFaxActivity_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1259368343, 15861, 16626, [188, 60, 68, 203, 129, 21, 235, 223]);
}
impl ::std::convert::From<IFaxActivity> for ::windows::runtime::IUnknown {
    fn from(value: IFaxActivity) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxActivity> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxActivity) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxActivity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxActivity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxActivity> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxActivity) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxActivity> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxActivity) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxActivity {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxActivity {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxActivity_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plincomingmessages: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plroutingmessages: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ploutgoingmessages: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plqueuedmessages: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxActivityLogging(::windows::runtime::IUnknown);
impl IFaxActivityLogging {
    pub unsafe fn LogIncoming(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetLogIncoming(&self, blogincoming: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(blogincoming)).ok()
    }
    pub unsafe fn LogOutgoing(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetLogOutgoing(&self, blogoutgoing: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(blogoutgoing)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DatabasePath(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDatabasePath<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdatabasepath: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), bstrdatabasepath.into_param().abi()).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Save(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFaxActivityLogging {
    type Vtable = IFaxActivityLogging_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(506005387, 23145, 18811, [149, 146, 73, 183, 231, 250, 221, 181]);
}
impl ::std::convert::From<IFaxActivityLogging> for ::windows::runtime::IUnknown {
    fn from(value: IFaxActivityLogging) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxActivityLogging> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxActivityLogging) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxActivityLogging {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxActivityLogging {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxActivityLogging> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxActivityLogging) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxActivityLogging> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxActivityLogging) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxActivityLogging {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxActivityLogging {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxActivityLogging_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pblogincoming: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, blogincoming: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pblogoutgoing: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, blogoutgoing: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdatabasepath: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrdatabasepath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxConfiguration(::windows::runtime::IUnknown);
impl IFaxConfiguration {
    pub unsafe fn UseArchive(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetUseArchive(&self, busearchive: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(busearchive)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ArchiveLocation(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetArchiveLocation<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrarchivelocation: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), bstrarchivelocation.into_param().abi()).ok()
    }
    pub unsafe fn SizeQuotaWarning(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetSizeQuotaWarning(&self, bsizequotawarning: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(bsizequotawarning)).ok()
    }
    pub unsafe fn HighQuotaWaterMark(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetHighQuotaWaterMark(&self, lhighquotawatermark: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(lhighquotawatermark)).ok()
    }
    pub unsafe fn LowQuotaWaterMark(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetLowQuotaWaterMark(&self, llowquotawatermark: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(llowquotawatermark)).ok()
    }
    pub unsafe fn ArchiveAgeLimit(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetArchiveAgeLimit(&self, larchiveagelimit: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(larchiveagelimit)).ok()
    }
    pub unsafe fn ArchiveSizeLow(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn ArchiveSizeHigh(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn OutgoingQueueBlocked(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetOutgoingQueueBlocked(&self, boutgoingblocked: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(boutgoingblocked)).ok()
    }
    pub unsafe fn OutgoingQueuePaused(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetOutgoingQueuePaused(&self, boutgoingpaused: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(boutgoingpaused)).ok()
    }
    pub unsafe fn AllowPersonalCoverPages(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetAllowPersonalCoverPages(&self, ballowpersonalcoverpages: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), ::std::mem::transmute(ballowpersonalcoverpages)).ok()
    }
    pub unsafe fn UseDeviceTSID(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetUseDeviceTSID(&self, busedevicetsid: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), ::std::mem::transmute(busedevicetsid)).ok()
    }
    pub unsafe fn Retries(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetRetries(&self, lretries: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), ::std::mem::transmute(lretries)).ok()
    }
    pub unsafe fn RetryDelay(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetRetryDelay(&self, lretrydelay: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), ::std::mem::transmute(lretrydelay)).ok()
    }
    pub unsafe fn DiscountRateStart(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn SetDiscountRateStart(&self, datediscountratestart: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), ::std::mem::transmute(datediscountratestart)).ok()
    }
    pub unsafe fn DiscountRateEnd(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn SetDiscountRateEnd(&self, datediscountrateend: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self), ::std::mem::transmute(datediscountrateend)).ok()
    }
    pub unsafe fn OutgoingQueueAgeLimit(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetOutgoingQueueAgeLimit(&self, loutgoingqueueagelimit: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self), ::std::mem::transmute(loutgoingqueueagelimit)).ok()
    }
    pub unsafe fn Branding(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetBranding(&self, bbranding: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(::std::mem::transmute_copy(self), ::std::mem::transmute(bbranding)).ok()
    }
    pub unsafe fn IncomingQueueBlocked(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).41)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetIncomingQueueBlocked(&self, bincomingblocked: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).42)(::std::mem::transmute_copy(self), ::std::mem::transmute(bincomingblocked)).ok()
    }
    pub unsafe fn AutoCreateAccountOnConnect(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).43)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetAutoCreateAccountOnConnect(&self, bautocreateaccountonconnect: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).44)(::std::mem::transmute_copy(self), ::std::mem::transmute(bautocreateaccountonconnect)).ok()
    }
    pub unsafe fn IncomingFaxesArePublic(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).45)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetIncomingFaxesArePublic(&self, bincomingfaxesarepublic: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).46)(::std::mem::transmute_copy(self), ::std::mem::transmute(bincomingfaxesarepublic)).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).47)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Save(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).48)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFaxConfiguration {
    type Vtable = IFaxConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(284479735, 2452, 17731, [171, 110, 80, 105, 73, 18, 140, 64]);
}
impl ::std::convert::From<IFaxConfiguration> for ::windows::runtime::IUnknown {
    fn from(value: IFaxConfiguration) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxConfiguration> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxConfiguration) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxConfiguration> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxConfiguration) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxConfiguration> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxConfiguration) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxConfiguration_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbusearchive: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, busearchive: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrarchivelocation: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrarchivelocation: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbsizequotawarning: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bsizequotawarning: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plhighquotawatermark: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lhighquotawatermark: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pllowquotawatermark: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, llowquotawatermark: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plarchiveagelimit: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, larchiveagelimit: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plsizelow: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plsizehigh: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pboutgoingblocked: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, boutgoingblocked: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pboutgoingpaused: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, boutgoingpaused: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pballowpersonalcoverpages: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ballowpersonalcoverpages: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbusedevicetsid: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, busedevicetsid: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plretries: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lretries: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plretrydelay: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lretrydelay: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatediscountratestart: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, datediscountratestart: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatediscountrateend: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, datediscountrateend: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ploutgoingqueueagelimit: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, loutgoingqueueagelimit: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbbranding: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bbranding: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbincomingblocked: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bincomingblocked: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbautocreateaccountonconnect: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bautocreateaccountonconnect: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbincomingfaxesarepublic: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bincomingfaxesarepublic: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxDevice(::windows::runtime::IUnknown);
impl IFaxDevice {
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeviceName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProviderUniqueName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn PoweredOff(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn ReceivingNow(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SendingNow(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn UsedRoutingMethods(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdescription: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn SendEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetSendEnabled(&self, bsendenabled: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(bsendenabled)).ok()
    }
    pub unsafe fn ReceiveMode(&self) -> ::windows::runtime::Result<FAX_DEVICE_RECEIVE_MODE_ENUM> {
        let mut result__: <FAX_DEVICE_RECEIVE_MODE_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_DEVICE_RECEIVE_MODE_ENUM>(result__)
    }
    pub unsafe fn SetReceiveMode(&self, receivemode: FAX_DEVICE_RECEIVE_MODE_ENUM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(receivemode)).ok()
    }
    pub unsafe fn RingsBeforeAnswer(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetRingsBeforeAnswer(&self, lringsbeforeanswer: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(lringsbeforeanswer)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CSID(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCSID<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrcsid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), bstrcsid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TSID(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTSID<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrtsid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), bstrtsid.into_param().abi()).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Save(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn GetExtensionProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrguid: Param0) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), bstrguid.into_param().abi(), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn SetExtensionProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, bstrguid: Param0, vproperty: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), bstrguid.into_param().abi(), vproperty.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UseRoutingMethod<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrmethodguid: Param0, buse: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), bstrmethodguid.into_param().abi(), ::std::mem::transmute(buse)).ok()
    }
    pub unsafe fn RingingNow(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn AnswerCall(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFaxDevice {
    type Vtable = IFaxDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1227910233, 46382, 18535, [157, 244, 202, 88, 65, 201, 86, 208]);
}
impl ::std::convert::From<IFaxDevice> for ::windows::runtime::IUnknown {
    fn from(value: IFaxDevice) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxDevice> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxDevice) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxDevice> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxDevice) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxDevice> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxDevice) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdevicename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrprovideruniquename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbpoweredoff: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbreceivingnow: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbsendingnow: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvusedroutingmethods: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdescription: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrdescription: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbsendenabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bsendenabled: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, preceivemode: *mut FAX_DEVICE_RECEIVE_MODE_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, receivemode: FAX_DEVICE_RECEIVE_MODE_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plringsbeforeanswer: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lringsbeforeanswer: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrcsid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrcsid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrtsid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrtsid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrguid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvproperty: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrguid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, vproperty: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrmethodguid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, buse: i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbringingnow: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxDeviceIds(::windows::runtime::IUnknown);
impl IFaxDeviceIds {
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    pub unsafe fn Item(&self, lindex: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(lindex), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn Add(&self, ldeviceid: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(ldeviceid)).ok()
    }
    pub unsafe fn Remove(&self, lindex: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(lindex)).ok()
    }
    pub unsafe fn SetOrder(&self, ldeviceid: i32, lneworder: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(ldeviceid), ::std::mem::transmute(lneworder)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFaxDeviceIds {
    type Vtable = IFaxDeviceIds_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(789545279, 19689, 17470, [140, 161, 115, 140, 250, 238, 225, 73]);
}
impl ::std::convert::From<IFaxDeviceIds> for ::windows::runtime::IUnknown {
    fn from(value: IFaxDeviceIds) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxDeviceIds> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxDeviceIds) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxDeviceIds {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxDeviceIds {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxDeviceIds> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxDeviceIds) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxDeviceIds> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxDeviceIds) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxDeviceIds {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxDeviceIds {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxDeviceIds_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lindex: i32, pldeviceid: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plcount: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ldeviceid: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lindex: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ldeviceid: i32, lneworder: i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxDeviceProvider(::windows::runtime::IUnknown);
impl IFaxDeviceProvider {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FriendlyName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImageName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UniqueName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TapiProviderName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn MajorVersion(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn MinorVersion(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn MajorBuild(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn MinorBuild(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn Debug(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn Status(&self) -> ::windows::runtime::Result<FAX_PROVIDER_STATUS_ENUM> {
        let mut result__: <FAX_PROVIDER_STATUS_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_PROVIDER_STATUS_ENUM>(result__)
    }
    pub unsafe fn InitErrorCode(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn DeviceIds(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFaxDeviceProvider {
    type Vtable = IFaxDeviceProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(688827491, 33772, 17564, [132, 23, 241, 72, 223, 140, 104, 42]);
}
impl ::std::convert::From<IFaxDeviceProvider> for ::windows::runtime::IUnknown {
    fn from(value: IFaxDeviceProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxDeviceProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxDeviceProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxDeviceProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxDeviceProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxDeviceProvider> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxDeviceProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxDeviceProvider> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxDeviceProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxDeviceProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxDeviceProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxDeviceProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrfriendlyname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrimagename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstruniquename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrtapiprovidername: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plmajorversion: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plminorversion: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plmajorbuild: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plminorbuild: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbdebug: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstatus: *mut FAX_PROVIDER_STATUS_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pliniterrorcode: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvdeviceids: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxDeviceProviders(::windows::runtime::IUnknown);
impl IFaxDeviceProviders {
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Item<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, vindex: Param0) -> ::windows::runtime::Result<IFaxDeviceProvider> {
        let mut result__: <IFaxDeviceProvider as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), vindex.into_param().abi(), &mut result__).from_abi::<IFaxDeviceProvider>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFaxDeviceProviders {
    type Vtable = IFaxDeviceProviders_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2679598946, 19582, 17317, [182, 253, 80, 40, 147, 247, 225, 62]);
}
impl ::std::convert::From<IFaxDeviceProviders> for ::windows::runtime::IUnknown {
    fn from(value: IFaxDeviceProviders) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxDeviceProviders> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxDeviceProviders) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxDeviceProviders {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxDeviceProviders {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxDeviceProviders> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxDeviceProviders) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxDeviceProviders> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxDeviceProviders) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxDeviceProviders {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxDeviceProviders {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxDeviceProviders_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vindex: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxdeviceprovider: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plcount: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxDevices(::windows::runtime::IUnknown);
impl IFaxDevices {
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Item<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, vindex: Param0) -> ::windows::runtime::Result<IFaxDevice> {
        let mut result__: <IFaxDevice as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), vindex.into_param().abi(), &mut result__).from_abi::<IFaxDevice>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn ItemById(&self, lid: i32) -> ::windows::runtime::Result<IFaxDevice> {
        let mut result__: <IFaxDevice as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(lid), &mut result__).from_abi::<IFaxDevice>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFaxDevices {
    type Vtable = IFaxDevices_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2655418430, 62287, 18478, [163, 96, 4, 22, 190, 203, 189, 150]);
}
impl ::std::convert::From<IFaxDevices> for ::windows::runtime::IUnknown {
    fn from(value: IFaxDevices) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxDevices> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxDevices) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxDevices {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxDevices {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxDevices> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxDevices) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxDevices> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxDevices) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxDevices {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxDevices {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxDevices_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vindex: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plcount: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lid: i32, ppfaxdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxDocument(::windows::runtime::IUnknown);
impl IFaxDocument {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Body(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBody<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrbody: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), bstrbody.into_param().abi()).ok()
    }
    pub unsafe fn Sender(&self) -> ::windows::runtime::Result<IFaxSender> {
        let mut result__: <IFaxSender as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxSender>(result__)
    }
    pub unsafe fn Recipients(&self) -> ::windows::runtime::Result<IFaxRecipients> {
        let mut result__: <IFaxRecipients as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxRecipients>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CoverPage(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCoverPage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrcoverpage: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), bstrcoverpage.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Subject(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSubject<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsubject: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), bstrsubject.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Note(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNote<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrnote: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), bstrnote.into_param().abi()).ok()
    }
    pub unsafe fn ScheduleTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn SetScheduleTime(&self, datescheduletime: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(datescheduletime)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReceiptAddress(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiptAddress<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrreceiptaddress: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), bstrreceiptaddress.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DocumentName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDocumentName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdocumentname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), bstrdocumentname.into_param().abi()).ok()
    }
    pub unsafe fn CallHandle(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetCallHandle(&self, lcallhandle: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(lcallhandle)).ok()
    }
    pub unsafe fn CoverPageType(&self) -> ::windows::runtime::Result<FAX_COVERPAGE_TYPE_ENUM> {
        let mut result__: <FAX_COVERPAGE_TYPE_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_COVERPAGE_TYPE_ENUM>(result__)
    }
    pub unsafe fn SetCoverPageType(&self, coverpagetype: FAX_COVERPAGE_TYPE_ENUM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), ::std::mem::transmute(coverpagetype)).ok()
    }
    pub unsafe fn ScheduleType(&self) -> ::windows::runtime::Result<FAX_SCHEDULE_TYPE_ENUM> {
        let mut result__: <FAX_SCHEDULE_TYPE_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_SCHEDULE_TYPE_ENUM>(result__)
    }
    pub unsafe fn SetScheduleType(&self, scheduletype: FAX_SCHEDULE_TYPE_ENUM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), ::std::mem::transmute(scheduletype)).ok()
    }
    pub unsafe fn ReceiptType(&self) -> ::windows::runtime::Result<FAX_RECEIPT_TYPE_ENUM> {
        let mut result__: <FAX_RECEIPT_TYPE_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_RECEIPT_TYPE_ENUM>(result__)
    }
    pub unsafe fn SetReceiptType(&self, receipttype: FAX_RECEIPT_TYPE_ENUM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), ::std::mem::transmute(receipttype)).ok()
    }
    pub unsafe fn GroupBroadcastReceipts(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetGroupBroadcastReceipts(&self, busegrouping: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), ::std::mem::transmute(busegrouping)).ok()
    }
    pub unsafe fn Priority(&self) -> ::windows::runtime::Result<FAX_PRIORITY_TYPE_ENUM> {
        let mut result__: <FAX_PRIORITY_TYPE_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_PRIORITY_TYPE_ENUM>(result__)
    }
    pub unsafe fn SetPriority(&self, priority: FAX_PRIORITY_TYPE_ENUM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), ::std::mem::transmute(priority)).ok()
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn TapiConnection(&self) -> ::windows::runtime::Result<super::super::System::Ole::Automation::IDispatch> {
        let mut result__: <super::super::System::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Ole::Automation::IDispatch>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn putref_TapiConnection<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch>>(&self, ptapiconnection: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self), ptapiconnection.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Submit<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrfaxservername: Param0) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self), bstrfaxservername.into_param().abi(), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn ConnectedSubmit<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxServer>>(&self, pfaxserver: Param0) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self), pfaxserver.into_param().abi(), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    pub unsafe fn AttachFaxToReceipt(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetAttachFaxToReceipt(&self, battachfax: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(::std::mem::transmute_copy(self), ::std::mem::transmute(battachfax)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFaxDocument {
    type Vtable = IFaxDocument_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2986844742, 2531, 19022, [167, 220, 254, 163, 29, 41, 69, 143]);
}
impl ::std::convert::From<IFaxDocument> for ::windows::runtime::IUnknown {
    fn from(value: IFaxDocument) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxDocument> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxDocument) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxDocument> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxDocument) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxDocument> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxDocument) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxDocument_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrbody: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrbody: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfaxsender: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfaxrecipients: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrcoverpage: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrcoverpage: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrsubject: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrsubject: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrnote: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrnote: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatescheduletime: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, datescheduletime: f64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrreceiptaddress: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrreceiptaddress: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdocumentname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrdocumentname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plcallhandle: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lcallhandle: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcoverpagetype: *mut FAX_COVERPAGE_TYPE_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, coverpagetype: FAX_COVERPAGE_TYPE_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pscheduletype: *mut FAX_SCHEDULE_TYPE_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scheduletype: FAX_SCHEDULE_TYPE_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, preceipttype: *mut FAX_RECEIPT_TYPE_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, receipttype: FAX_RECEIPT_TYPE_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbusegrouping: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, busegrouping: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppriority: *mut FAX_PRIORITY_TYPE_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, priority: FAX_PRIORITY_TYPE_ENUM) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pptapiconnection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptapiconnection: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrfaxservername: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvfaxoutgoingjobids: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxserver: ::windows::runtime::RawPtr, pvfaxoutgoingjobids: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbattachfax: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, battachfax: i16) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxDocument2(::windows::runtime::IUnknown);
impl IFaxDocument2 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Body(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBody<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrbody: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), bstrbody.into_param().abi()).ok()
    }
    pub unsafe fn Sender(&self) -> ::windows::runtime::Result<IFaxSender> {
        let mut result__: <IFaxSender as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxSender>(result__)
    }
    pub unsafe fn Recipients(&self) -> ::windows::runtime::Result<IFaxRecipients> {
        let mut result__: <IFaxRecipients as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxRecipients>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CoverPage(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCoverPage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrcoverpage: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), bstrcoverpage.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Subject(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSubject<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsubject: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), bstrsubject.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Note(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNote<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrnote: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), bstrnote.into_param().abi()).ok()
    }
    pub unsafe fn ScheduleTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn SetScheduleTime(&self, datescheduletime: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(datescheduletime)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReceiptAddress(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiptAddress<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrreceiptaddress: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), bstrreceiptaddress.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DocumentName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDocumentName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdocumentname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), bstrdocumentname.into_param().abi()).ok()
    }
    pub unsafe fn CallHandle(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetCallHandle(&self, lcallhandle: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(lcallhandle)).ok()
    }
    pub unsafe fn CoverPageType(&self) -> ::windows::runtime::Result<FAX_COVERPAGE_TYPE_ENUM> {
        let mut result__: <FAX_COVERPAGE_TYPE_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_COVERPAGE_TYPE_ENUM>(result__)
    }
    pub unsafe fn SetCoverPageType(&self, coverpagetype: FAX_COVERPAGE_TYPE_ENUM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), ::std::mem::transmute(coverpagetype)).ok()
    }
    pub unsafe fn ScheduleType(&self) -> ::windows::runtime::Result<FAX_SCHEDULE_TYPE_ENUM> {
        let mut result__: <FAX_SCHEDULE_TYPE_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_SCHEDULE_TYPE_ENUM>(result__)
    }
    pub unsafe fn SetScheduleType(&self, scheduletype: FAX_SCHEDULE_TYPE_ENUM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), ::std::mem::transmute(scheduletype)).ok()
    }
    pub unsafe fn ReceiptType(&self) -> ::windows::runtime::Result<FAX_RECEIPT_TYPE_ENUM> {
        let mut result__: <FAX_RECEIPT_TYPE_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_RECEIPT_TYPE_ENUM>(result__)
    }
    pub unsafe fn SetReceiptType(&self, receipttype: FAX_RECEIPT_TYPE_ENUM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), ::std::mem::transmute(receipttype)).ok()
    }
    pub unsafe fn GroupBroadcastReceipts(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetGroupBroadcastReceipts(&self, busegrouping: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), ::std::mem::transmute(busegrouping)).ok()
    }
    pub unsafe fn Priority(&self) -> ::windows::runtime::Result<FAX_PRIORITY_TYPE_ENUM> {
        let mut result__: <FAX_PRIORITY_TYPE_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_PRIORITY_TYPE_ENUM>(result__)
    }
    pub unsafe fn SetPriority(&self, priority: FAX_PRIORITY_TYPE_ENUM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), ::std::mem::transmute(priority)).ok()
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn TapiConnection(&self) -> ::windows::runtime::Result<super::super::System::Ole::Automation::IDispatch> {
        let mut result__: <super::super::System::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Ole::Automation::IDispatch>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn putref_TapiConnection<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch>>(&self, ptapiconnection: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self), ptapiconnection.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Submit<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrfaxservername: Param0) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self), bstrfaxservername.into_param().abi(), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn ConnectedSubmit<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxServer>>(&self, pfaxserver: Param0) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self), pfaxserver.into_param().abi(), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    pub unsafe fn AttachFaxToReceipt(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetAttachFaxToReceipt(&self, battachfax: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(::std::mem::transmute_copy(self), ::std::mem::transmute(battachfax)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SubmissionId(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).41)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Bodies(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).42)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn SetBodies<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, vbodies: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).43)(::std::mem::transmute_copy(self), vbodies.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Submit2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrfaxservername: Param0, pvfaxoutgoingjobids: *mut super::super::System::Com::VARIANT, plerrorbodyfile: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).44)(::std::mem::transmute_copy(self), bstrfaxservername.into_param().abi(), ::std::mem::transmute(pvfaxoutgoingjobids), ::std::mem::transmute(plerrorbodyfile)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn ConnectedSubmit2<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxServer>>(&self, pfaxserver: Param0, pvfaxoutgoingjobids: *mut super::super::System::Com::VARIANT, plerrorbodyfile: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).45)(::std::mem::transmute_copy(self), pfaxserver.into_param().abi(), ::std::mem::transmute(pvfaxoutgoingjobids), ::std::mem::transmute(plerrorbodyfile)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFaxDocument2 {
    type Vtable = IFaxDocument2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3778311777, 63983, 19821, [180, 165, 192, 160, 104, 182, 92, 255]);
}
impl ::std::convert::From<IFaxDocument2> for ::windows::runtime::IUnknown {
    fn from(value: IFaxDocument2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxDocument2> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxDocument2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxDocument2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxDocument2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFaxDocument2> for IFaxDocument {
    fn from(value: IFaxDocument2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxDocument2> for IFaxDocument {
    fn from(value: &IFaxDocument2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFaxDocument> for IFaxDocument2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFaxDocument> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFaxDocument>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFaxDocument> for &IFaxDocument2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFaxDocument> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFaxDocument>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxDocument2> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxDocument2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxDocument2> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxDocument2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxDocument2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxDocument2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxDocument2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrbody: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrbody: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfaxsender: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfaxrecipients: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrcoverpage: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrcoverpage: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrsubject: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrsubject: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrnote: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrnote: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatescheduletime: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, datescheduletime: f64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrreceiptaddress: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrreceiptaddress: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdocumentname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrdocumentname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plcallhandle: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lcallhandle: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcoverpagetype: *mut FAX_COVERPAGE_TYPE_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, coverpagetype: FAX_COVERPAGE_TYPE_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pscheduletype: *mut FAX_SCHEDULE_TYPE_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scheduletype: FAX_SCHEDULE_TYPE_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, preceipttype: *mut FAX_RECEIPT_TYPE_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, receipttype: FAX_RECEIPT_TYPE_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbusegrouping: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, busegrouping: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppriority: *mut FAX_PRIORITY_TYPE_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, priority: FAX_PRIORITY_TYPE_ENUM) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pptapiconnection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptapiconnection: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrfaxservername: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvfaxoutgoingjobids: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxserver: ::windows::runtime::RawPtr, pvfaxoutgoingjobids: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbattachfax: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, battachfax: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrsubmissionid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvbodies: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vbodies: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrfaxservername: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvfaxoutgoingjobids: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, plerrorbodyfile: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxserver: ::windows::runtime::RawPtr, pvfaxoutgoingjobids: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, plerrorbodyfile: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxEventLogging(::windows::runtime::IUnknown);
impl IFaxEventLogging {
    pub unsafe fn InitEventsLevel(&self) -> ::windows::runtime::Result<FAX_LOG_LEVEL_ENUM> {
        let mut result__: <FAX_LOG_LEVEL_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_LOG_LEVEL_ENUM>(result__)
    }
    pub unsafe fn SetInitEventsLevel(&self, initeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(initeventlevel)).ok()
    }
    pub unsafe fn InboundEventsLevel(&self) -> ::windows::runtime::Result<FAX_LOG_LEVEL_ENUM> {
        let mut result__: <FAX_LOG_LEVEL_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_LOG_LEVEL_ENUM>(result__)
    }
    pub unsafe fn SetInboundEventsLevel(&self, inboundeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(inboundeventlevel)).ok()
    }
    pub unsafe fn OutboundEventsLevel(&self) -> ::windows::runtime::Result<FAX_LOG_LEVEL_ENUM> {
        let mut result__: <FAX_LOG_LEVEL_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_LOG_LEVEL_ENUM>(result__)
    }
    pub unsafe fn SetOutboundEventsLevel(&self, outboundeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(outboundeventlevel)).ok()
    }
    pub unsafe fn GeneralEventsLevel(&self) -> ::windows::runtime::Result<FAX_LOG_LEVEL_ENUM> {
        let mut result__: <FAX_LOG_LEVEL_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_LOG_LEVEL_ENUM>(result__)
    }
    pub unsafe fn SetGeneralEventsLevel(&self, generaleventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(generaleventlevel)).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Save(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFaxEventLogging {
    type Vtable = IFaxEventLogging_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(142661989, 8424, 17124, [142, 23, 148, 79, 25, 44, 170, 212]);
}
impl ::std::convert::From<IFaxEventLogging> for ::windows::runtime::IUnknown {
    fn from(value: IFaxEventLogging) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxEventLogging> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxEventLogging) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxEventLogging {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxEventLogging {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxEventLogging> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxEventLogging) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxEventLogging> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxEventLogging) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxEventLogging {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxEventLogging {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxEventLogging_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, piniteventlevel: *mut FAX_LOG_LEVEL_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, initeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinboundeventlevel: *mut FAX_LOG_LEVEL_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inboundeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, poutboundeventlevel: *mut FAX_LOG_LEVEL_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, outboundeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pgeneraleventlevel: *mut FAX_LOG_LEVEL_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, generaleventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxFolders(::windows::runtime::IUnknown);
impl IFaxFolders {
    pub unsafe fn OutgoingQueue(&self) -> ::windows::runtime::Result<IFaxOutgoingQueue> {
        let mut result__: <IFaxOutgoingQueue as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxOutgoingQueue>(result__)
    }
    pub unsafe fn IncomingQueue(&self) -> ::windows::runtime::Result<IFaxIncomingQueue> {
        let mut result__: <IFaxIncomingQueue as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxIncomingQueue>(result__)
    }
    pub unsafe fn IncomingArchive(&self) -> ::windows::runtime::Result<IFaxIncomingArchive> {
        let mut result__: <IFaxIncomingArchive as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxIncomingArchive>(result__)
    }
    pub unsafe fn OutgoingArchive(&self) -> ::windows::runtime::Result<IFaxOutgoingArchive> {
        let mut result__: <IFaxOutgoingArchive as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxOutgoingArchive>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFaxFolders {
    type Vtable = IFaxFolders_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3705909928, 42923, 17084, [157, 10, 49, 73, 69, 114, 97, 160]);
}
impl ::std::convert::From<IFaxFolders> for ::windows::runtime::IUnknown {
    fn from(value: IFaxFolders) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxFolders> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxFolders) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxFolders {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxFolders {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxFolders> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxFolders) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxFolders> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxFolders) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxFolders {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxFolders {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxFolders_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxoutgoingqueue: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxincomingqueue: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxincomingarchive: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxoutgoingarchive: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxInboundRouting(::windows::runtime::IUnknown);
impl IFaxInboundRouting {
    pub unsafe fn GetExtensions(&self) -> ::windows::runtime::Result<IFaxInboundRoutingExtensions> {
        let mut result__: <IFaxInboundRoutingExtensions as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxInboundRoutingExtensions>(result__)
    }
    pub unsafe fn GetMethods(&self) -> ::windows::runtime::Result<IFaxInboundRoutingMethods> {
        let mut result__: <IFaxInboundRoutingMethods as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxInboundRoutingMethods>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFaxInboundRouting {
    type Vtable = IFaxInboundRouting_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2169029135, 40274, 17841, [191, 150, 56, 252, 18, 113, 53, 39]);
}
impl ::std::convert::From<IFaxInboundRouting> for ::windows::runtime::IUnknown {
    fn from(value: IFaxInboundRouting) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxInboundRouting> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxInboundRouting) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxInboundRouting {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxInboundRouting {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxInboundRouting> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxInboundRouting) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxInboundRouting> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxInboundRouting) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxInboundRouting {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxInboundRouting {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxInboundRouting_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxinboundroutingextensions: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxinboundroutingmethods: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxInboundRoutingExtension(::windows::runtime::IUnknown);
impl IFaxInboundRoutingExtension {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FriendlyName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImageName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UniqueName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn MajorVersion(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn MinorVersion(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn MajorBuild(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn MinorBuild(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn Debug(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn Status(&self) -> ::windows::runtime::Result<FAX_PROVIDER_STATUS_ENUM> {
        let mut result__: <FAX_PROVIDER_STATUS_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_PROVIDER_STATUS_ENUM>(result__)
    }
    pub unsafe fn InitErrorCode(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Methods(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFaxInboundRoutingExtension {
    type Vtable = IFaxInboundRoutingExtension_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2287689224, 49772, 20217, [175, 131, 81, 88, 10, 117, 11, 225]);
}
impl ::std::convert::From<IFaxInboundRoutingExtension> for ::windows::runtime::IUnknown {
    fn from(value: IFaxInboundRoutingExtension) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxInboundRoutingExtension> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxInboundRoutingExtension) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxInboundRoutingExtension {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxInboundRoutingExtension {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxInboundRoutingExtension> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxInboundRoutingExtension) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxInboundRoutingExtension> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxInboundRoutingExtension) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxInboundRoutingExtension {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxInboundRoutingExtension {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxInboundRoutingExtension_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrfriendlyname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrimagename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstruniquename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plmajorversion: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plminorversion: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plmajorbuild: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plminorbuild: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbdebug: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstatus: *mut FAX_PROVIDER_STATUS_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pliniterrorcode: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvmethods: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxInboundRoutingExtensions(::windows::runtime::IUnknown);
impl IFaxInboundRoutingExtensions {
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Item<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, vindex: Param0) -> ::windows::runtime::Result<IFaxInboundRoutingExtension> {
        let mut result__: <IFaxInboundRoutingExtension as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), vindex.into_param().abi(), &mut result__).from_abi::<IFaxInboundRoutingExtension>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFaxInboundRoutingExtensions {
    type Vtable = IFaxInboundRoutingExtensions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(795645555, 31526, 17118, [142, 176, 145, 93, 205, 42, 79, 76]);
}
impl ::std::convert::From<IFaxInboundRoutingExtensions> for ::windows::runtime::IUnknown {
    fn from(value: IFaxInboundRoutingExtensions) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxInboundRoutingExtensions> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxInboundRoutingExtensions) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxInboundRoutingExtensions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxInboundRoutingExtensions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxInboundRoutingExtensions> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxInboundRoutingExtensions) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxInboundRoutingExtensions> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxInboundRoutingExtensions) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxInboundRoutingExtensions {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxInboundRoutingExtensions {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxInboundRoutingExtensions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vindex: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxinboundroutingextension: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plcount: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxInboundRoutingMethod(::windows::runtime::IUnknown);
impl IFaxInboundRoutingMethod {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GUID(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FunctionName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExtensionFriendlyName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExtensionImageName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Priority(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPriority(&self, lpriority: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(lpriority)).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Save(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFaxInboundRoutingMethod {
    type Vtable = IFaxInboundRoutingMethod_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1164968033, 44445, 18294, [168, 196, 100, 6, 84, 146, 207, 75]);
}
impl ::std::convert::From<IFaxInboundRoutingMethod> for ::windows::runtime::IUnknown {
    fn from(value: IFaxInboundRoutingMethod) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxInboundRoutingMethod> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxInboundRoutingMethod) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxInboundRoutingMethod {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxInboundRoutingMethod {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxInboundRoutingMethod> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxInboundRoutingMethod) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxInboundRoutingMethod> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxInboundRoutingMethod) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxInboundRoutingMethod {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxInboundRoutingMethod {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxInboundRoutingMethod_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrguid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrfunctionname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrextensionfriendlyname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrextensionimagename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plpriority: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpriority: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxInboundRoutingMethods(::windows::runtime::IUnknown);
impl IFaxInboundRoutingMethods {
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Item<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, vindex: Param0) -> ::windows::runtime::Result<IFaxInboundRoutingMethod> {
        let mut result__: <IFaxInboundRoutingMethod as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), vindex.into_param().abi(), &mut result__).from_abi::<IFaxInboundRoutingMethod>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFaxInboundRoutingMethods {
    type Vtable = IFaxInboundRoutingMethods_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2017446416, 35080, 17523, [157, 105, 246, 127, 190, 160, 198, 185]);
}
impl ::std::convert::From<IFaxInboundRoutingMethods> for ::windows::runtime::IUnknown {
    fn from(value: IFaxInboundRoutingMethods) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxInboundRoutingMethods> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxInboundRoutingMethods) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxInboundRoutingMethods {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxInboundRoutingMethods {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxInboundRoutingMethods> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxInboundRoutingMethods) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxInboundRoutingMethods> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxInboundRoutingMethods) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxInboundRoutingMethods {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxInboundRoutingMethods {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxInboundRoutingMethods_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vindex: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxinboundroutingmethod: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plcount: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxIncomingArchive(::windows::runtime::IUnknown);
impl IFaxIncomingArchive {
    pub unsafe fn UseArchive(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetUseArchive(&self, busearchive: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(busearchive)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ArchiveFolder(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetArchiveFolder<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrarchivefolder: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), bstrarchivefolder.into_param().abi()).ok()
    }
    pub unsafe fn SizeQuotaWarning(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetSizeQuotaWarning(&self, bsizequotawarning: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(bsizequotawarning)).ok()
    }
    pub unsafe fn HighQuotaWaterMark(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetHighQuotaWaterMark(&self, lhighquotawatermark: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(lhighquotawatermark)).ok()
    }
    pub unsafe fn LowQuotaWaterMark(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetLowQuotaWaterMark(&self, llowquotawatermark: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(llowquotawatermark)).ok()
    }
    pub unsafe fn AgeLimit(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAgeLimit(&self, lagelimit: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(lagelimit)).ok()
    }
    pub unsafe fn SizeLow(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SizeHigh(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Save(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetMessages(&self, lprefetchsize: i32) -> ::windows::runtime::Result<IFaxIncomingMessageIterator> {
        let mut result__: <IFaxIncomingMessageIterator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(lprefetchsize), &mut result__).from_abi::<IFaxIncomingMessageIterator>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMessage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrmessageid: Param0) -> ::windows::runtime::Result<IFaxIncomingMessage> {
        let mut result__: <IFaxIncomingMessage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), bstrmessageid.into_param().abi(), &mut result__).from_abi::<IFaxIncomingMessage>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFaxIncomingArchive {
    type Vtable = IFaxIncomingArchive_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1980116167, 63252, 20413, [170, 6, 237, 110, 74, 75, 112, 243]);
}
impl ::std::convert::From<IFaxIncomingArchive> for ::windows::runtime::IUnknown {
    fn from(value: IFaxIncomingArchive) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxIncomingArchive> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxIncomingArchive) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxIncomingArchive {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxIncomingArchive {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxIncomingArchive> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxIncomingArchive) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxIncomingArchive> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxIncomingArchive) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxIncomingArchive {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxIncomingArchive {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxIncomingArchive_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbusearchive: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, busearchive: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrarchivefolder: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrarchivefolder: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbsizequotawarning: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bsizequotawarning: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plhighquotawatermark: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lhighquotawatermark: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pllowquotawatermark: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, llowquotawatermark: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plagelimit: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lagelimit: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plsizelow: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plsizehigh: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lprefetchsize: i32, pfaxincomingmessageiterator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrmessageid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxincomingmessage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxIncomingJob(::windows::runtime::IUnknown);
impl IFaxIncomingJob {
    pub unsafe fn Size(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn CurrentPage(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn DeviceId(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn Status(&self) -> ::windows::runtime::Result<FAX_JOB_STATUS_ENUM> {
        let mut result__: <FAX_JOB_STATUS_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_JOB_STATUS_ENUM>(result__)
    }
    pub unsafe fn ExtendedStatusCode(&self) -> ::windows::runtime::Result<FAX_JOB_EXTENDED_STATUS_ENUM> {
        let mut result__: <FAX_JOB_EXTENDED_STATUS_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_JOB_EXTENDED_STATUS_ENUM>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExtendedStatus(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn AvailableOperations(&self) -> ::windows::runtime::Result<FAX_JOB_OPERATIONS_ENUM> {
        let mut result__: <FAX_JOB_OPERATIONS_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_JOB_OPERATIONS_ENUM>(result__)
    }
    pub unsafe fn Retries(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn TransmissionStart(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn TransmissionEnd(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CSID(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TSID(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CallerId(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RoutingInformation(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn JobType(&self) -> ::windows::runtime::Result<FAX_JOB_TYPE_ENUM> {
        let mut result__: <FAX_JOB_TYPE_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_JOB_TYPE_ENUM>(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTiff<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrtiffpath: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), bstrtiffpath.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFaxIncomingJob {
    type Vtable = IFaxIncomingJob_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(544549350, 25930, 18710, [159, 136, 77, 35, 46, 232, 161, 7]);
}
impl ::std::convert::From<IFaxIncomingJob> for ::windows::runtime::IUnknown {
    fn from(value: IFaxIncomingJob) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxIncomingJob> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxIncomingJob) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxIncomingJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxIncomingJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxIncomingJob> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxIncomingJob) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxIncomingJob> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxIncomingJob) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxIncomingJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxIncomingJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxIncomingJob_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plsize: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plcurrentpage: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pldeviceid: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstatus: *mut FAX_JOB_STATUS_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pextendedstatuscode: *mut FAX_JOB_EXTENDED_STATUS_ENUM) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrextendedstatus: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pavailableoperations: *mut FAX_JOB_OPERATIONS_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plretries: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatetransmissionstart: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatetransmissionend: *mut f64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrcsid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrtsid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrcallerid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrroutinginformation: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pjobtype: *mut FAX_JOB_TYPE_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrtiffpath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxIncomingJobs(::windows::runtime::IUnknown);
impl IFaxIncomingJobs {
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Item<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, vindex: Param0) -> ::windows::runtime::Result<IFaxIncomingJob> {
        let mut result__: <IFaxIncomingJob as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), vindex.into_param().abi(), &mut result__).from_abi::<IFaxIncomingJob>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFaxIncomingJobs {
    type Vtable = IFaxIncomingJobs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(18810089, 20438, 19491, [149, 19, 182, 182, 107, 178, 107, 233]);
}
impl ::std::convert::From<IFaxIncomingJobs> for ::windows::runtime::IUnknown {
    fn from(value: IFaxIncomingJobs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxIncomingJobs> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxIncomingJobs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxIncomingJobs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxIncomingJobs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxIncomingJobs> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxIncomingJobs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxIncomingJobs> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxIncomingJobs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxIncomingJobs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxIncomingJobs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxIncomingJobs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vindex: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxincomingjob: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plcount: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxIncomingMessage(::windows::runtime::IUnknown);
impl IFaxIncomingMessage {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Pages(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn Size(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeviceName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Retries(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn TransmissionStart(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn TransmissionEnd(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CSID(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TSID(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CallerId(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RoutingInformation(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTiff<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrtiffpath: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), bstrtiffpath.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFaxIncomingMessage {
    type Vtable = IFaxIncomingMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2091616506, 12025, 18513, [178, 243, 29, 20, 143, 237, 132, 71]);
}
impl ::std::convert::From<IFaxIncomingMessage> for ::windows::runtime::IUnknown {
    fn from(value: IFaxIncomingMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxIncomingMessage> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxIncomingMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxIncomingMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxIncomingMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxIncomingMessage> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxIncomingMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxIncomingMessage> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxIncomingMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxIncomingMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxIncomingMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxIncomingMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plpages: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plsize: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdevicename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plretries: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatetransmissionstart: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatetransmissionend: *mut f64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrcsid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrtsid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrcallerid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrroutinginformation: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrtiffpath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxIncomingMessage2(::windows::runtime::IUnknown);
impl IFaxIncomingMessage2 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Pages(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn Size(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeviceName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Retries(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn TransmissionStart(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn TransmissionEnd(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CSID(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TSID(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CallerId(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RoutingInformation(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTiff<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrtiffpath: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), bstrtiffpath.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Subject(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSubject<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsubject: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), bstrsubject.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SenderName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSenderName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsendername: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), bstrsendername.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SenderFaxNumber(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSenderFaxNumber<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsenderfaxnumber: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), bstrsenderfaxnumber.into_param().abi()).ok()
    }
    pub unsafe fn HasCoverPage(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetHasCoverPage(&self, bhascoverpage: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), ::std::mem::transmute(bhascoverpage)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Recipients(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRecipients<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrrecipients: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), bstrrecipients.into_param().abi()).ok()
    }
    pub unsafe fn WasReAssigned(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn Read(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetRead(&self, bread: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), ::std::mem::transmute(bread)).ok()
    }
    pub unsafe fn ReAssign(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Save(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFaxIncomingMessage2 {
    type Vtable = IFaxIncomingMessage2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4179657987, 58044, 18675, [158, 192, 230, 35, 111, 155, 80, 154]);
}
impl ::std::convert::From<IFaxIncomingMessage2> for ::windows::runtime::IUnknown {
    fn from(value: IFaxIncomingMessage2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxIncomingMessage2> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxIncomingMessage2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxIncomingMessage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxIncomingMessage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFaxIncomingMessage2> for IFaxIncomingMessage {
    fn from(value: IFaxIncomingMessage2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxIncomingMessage2> for IFaxIncomingMessage {
    fn from(value: &IFaxIncomingMessage2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFaxIncomingMessage> for IFaxIncomingMessage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFaxIncomingMessage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFaxIncomingMessage>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFaxIncomingMessage> for &IFaxIncomingMessage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFaxIncomingMessage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFaxIncomingMessage>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxIncomingMessage2> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxIncomingMessage2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxIncomingMessage2> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxIncomingMessage2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxIncomingMessage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxIncomingMessage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxIncomingMessage2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plpages: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plsize: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdevicename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plretries: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatetransmissionstart: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatetransmissionend: *mut f64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrcsid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrtsid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrcallerid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrroutinginformation: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrtiffpath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrsubject: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrsubject: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrsendername: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrsendername: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrsenderfaxnumber: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrsenderfaxnumber: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbhascoverpage: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bhascoverpage: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrrecipients: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrrecipients: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbwasreassigned: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbread: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bread: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxIncomingMessageIterator(::windows::runtime::IUnknown);
impl IFaxIncomingMessageIterator {
    pub unsafe fn Message(&self) -> ::windows::runtime::Result<IFaxIncomingMessage> {
        let mut result__: <IFaxIncomingMessage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxIncomingMessage>(result__)
    }
    pub unsafe fn PrefetchSize(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPrefetchSize(&self, lprefetchsize: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(lprefetchsize)).ok()
    }
    pub unsafe fn AtEOF(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn MoveFirst(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn MoveNext(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFaxIncomingMessageIterator {
    type Vtable = IFaxIncomingMessageIterator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4252232900, 28422, 20306, [130, 168, 247, 186, 6, 174, 49, 8]);
}
impl ::std::convert::From<IFaxIncomingMessageIterator> for ::windows::runtime::IUnknown {
    fn from(value: IFaxIncomingMessageIterator) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxIncomingMessageIterator> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxIncomingMessageIterator) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxIncomingMessageIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxIncomingMessageIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxIncomingMessageIterator> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxIncomingMessageIterator) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxIncomingMessageIterator> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxIncomingMessageIterator) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxIncomingMessageIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxIncomingMessageIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxIncomingMessageIterator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxincomingmessage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plprefetchsize: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lprefetchsize: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbeof: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxIncomingQueue(::windows::runtime::IUnknown);
impl IFaxIncomingQueue {
    pub unsafe fn Blocked(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetBlocked(&self, bblocked: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(bblocked)).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Save(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetJobs(&self) -> ::windows::runtime::Result<IFaxIncomingJobs> {
        let mut result__: <IFaxIncomingJobs as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxIncomingJobs>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetJob<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrjobid: Param0) -> ::windows::runtime::Result<IFaxIncomingJob> {
        let mut result__: <IFaxIncomingJob as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), bstrjobid.into_param().abi(), &mut result__).from_abi::<IFaxIncomingJob>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFaxIncomingQueue {
    type Vtable = IFaxIncomingQueue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2418959599, 36824, 19317, [151, 37, 96, 20, 223, 22, 21, 69]);
}
impl ::std::convert::From<IFaxIncomingQueue> for ::windows::runtime::IUnknown {
    fn from(value: IFaxIncomingQueue) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxIncomingQueue> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxIncomingQueue) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxIncomingQueue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxIncomingQueue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxIncomingQueue> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxIncomingQueue) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxIncomingQueue> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxIncomingQueue) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxIncomingQueue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxIncomingQueue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxIncomingQueue_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbblocked: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bblocked: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxincomingjobs: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrjobid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxincomingjob: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxJobStatus(::windows::runtime::IUnknown);
impl IFaxJobStatus {
    pub unsafe fn Status(&self) -> ::windows::runtime::Result<FAX_JOB_STATUS_ENUM> {
        let mut result__: <FAX_JOB_STATUS_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_JOB_STATUS_ENUM>(result__)
    }
    pub unsafe fn Pages(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn Size(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn CurrentPage(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn DeviceId(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CSID(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TSID(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn ExtendedStatusCode(&self) -> ::windows::runtime::Result<FAX_JOB_EXTENDED_STATUS_ENUM> {
        let mut result__: <FAX_JOB_EXTENDED_STATUS_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_JOB_EXTENDED_STATUS_ENUM>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExtendedStatus(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn AvailableOperations(&self) -> ::windows::runtime::Result<FAX_JOB_OPERATIONS_ENUM> {
        let mut result__: <FAX_JOB_OPERATIONS_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_JOB_OPERATIONS_ENUM>(result__)
    }
    pub unsafe fn Retries(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn JobType(&self) -> ::windows::runtime::Result<FAX_JOB_TYPE_ENUM> {
        let mut result__: <FAX_JOB_TYPE_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_JOB_TYPE_ENUM>(result__)
    }
    pub unsafe fn ScheduledTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn TransmissionStart(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn TransmissionEnd(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CallerId(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RoutingInformation(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFaxJobStatus {
    type Vtable = IFaxJobStatus_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2340877445, 64895, 18468, [136, 107, 64, 197, 202, 166, 23, 204]);
}
impl ::std::convert::From<IFaxJobStatus> for ::windows::runtime::IUnknown {
    fn from(value: IFaxJobStatus) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxJobStatus> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxJobStatus) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxJobStatus {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxJobStatus {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxJobStatus> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxJobStatus) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxJobStatus> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxJobStatus) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxJobStatus {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxJobStatus {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxJobStatus_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstatus: *mut FAX_JOB_STATUS_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plpages: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plsize: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plcurrentpage: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pldeviceid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrcsid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrtsid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pextendedstatuscode: *mut FAX_JOB_EXTENDED_STATUS_ENUM) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrextendedstatus: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pavailableoperations: *mut FAX_JOB_OPERATIONS_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plretries: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pjobtype: *mut FAX_JOB_TYPE_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatescheduledtime: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatetransmissionstart: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatetransmissionend: *mut f64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrcallerid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrroutinginformation: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxLoggingOptions(::windows::runtime::IUnknown);
impl IFaxLoggingOptions {
    pub unsafe fn EventLogging(&self) -> ::windows::runtime::Result<IFaxEventLogging> {
        let mut result__: <IFaxEventLogging as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxEventLogging>(result__)
    }
    pub unsafe fn ActivityLogging(&self) -> ::windows::runtime::Result<IFaxActivityLogging> {
        let mut result__: <IFaxActivityLogging as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxActivityLogging>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFaxLoggingOptions {
    type Vtable = IFaxLoggingOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(887508921, 27441, 19762, [139, 39, 210, 134, 192, 195, 54, 6]);
}
impl ::std::convert::From<IFaxLoggingOptions> for ::windows::runtime::IUnknown {
    fn from(value: IFaxLoggingOptions) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxLoggingOptions> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxLoggingOptions) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxLoggingOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxLoggingOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxLoggingOptions> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxLoggingOptions) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxLoggingOptions> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxLoggingOptions) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxLoggingOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxLoggingOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxLoggingOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxeventlogging: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxactivitylogging: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxOutboundRouting(::windows::runtime::IUnknown);
impl IFaxOutboundRouting {
    pub unsafe fn GetGroups(&self) -> ::windows::runtime::Result<IFaxOutboundRoutingGroups> {
        let mut result__: <IFaxOutboundRoutingGroups as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxOutboundRoutingGroups>(result__)
    }
    pub unsafe fn GetRules(&self) -> ::windows::runtime::Result<IFaxOutboundRoutingRules> {
        let mut result__: <IFaxOutboundRoutingRules as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxOutboundRoutingRules>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFaxOutboundRouting {
    type Vtable = IFaxOutboundRouting_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(635176356, 39177, 16829, [169, 91, 126, 93, 29, 236, 29, 67]);
}
impl ::std::convert::From<IFaxOutboundRouting> for ::windows::runtime::IUnknown {
    fn from(value: IFaxOutboundRouting) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxOutboundRouting> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxOutboundRouting) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxOutboundRouting {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxOutboundRouting {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxOutboundRouting> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxOutboundRouting) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxOutboundRouting> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxOutboundRouting) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxOutboundRouting {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxOutboundRouting {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutboundRouting_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxoutboundroutinggroups: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxoutboundroutingrules: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxOutboundRoutingGroup(::windows::runtime::IUnknown);
impl IFaxOutboundRoutingGroup {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Status(&self) -> ::windows::runtime::Result<FAX_GROUP_STATUS_ENUM> {
        let mut result__: <FAX_GROUP_STATUS_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_GROUP_STATUS_ENUM>(result__)
    }
    pub unsafe fn DeviceIds(&self) -> ::windows::runtime::Result<IFaxDeviceIds> {
        let mut result__: <IFaxDeviceIds as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxDeviceIds>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFaxOutboundRoutingGroup {
    type Vtable = IFaxOutboundRoutingGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3395455393, 32293, 20359, [154, 11, 147, 54, 87, 52, 150, 44]);
}
impl ::std::convert::From<IFaxOutboundRoutingGroup> for ::windows::runtime::IUnknown {
    fn from(value: IFaxOutboundRoutingGroup) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxOutboundRoutingGroup> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxOutboundRoutingGroup) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxOutboundRoutingGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxOutboundRoutingGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxOutboundRoutingGroup> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxOutboundRoutingGroup) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxOutboundRoutingGroup> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxOutboundRoutingGroup) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxOutboundRoutingGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxOutboundRoutingGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutboundRoutingGroup_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstatus: *mut FAX_GROUP_STATUS_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxdeviceids: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxOutboundRoutingGroups(::windows::runtime::IUnknown);
impl IFaxOutboundRoutingGroups {
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Item<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, vindex: Param0) -> ::windows::runtime::Result<IFaxOutboundRoutingGroup> {
        let mut result__: <IFaxOutboundRoutingGroup as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), vindex.into_param().abi(), &mut result__).from_abi::<IFaxOutboundRoutingGroup>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Add<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0) -> ::windows::runtime::Result<IFaxOutboundRoutingGroup> {
        let mut result__: <IFaxOutboundRoutingGroup as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), bstrname.into_param().abi(), &mut result__).from_abi::<IFaxOutboundRoutingGroup>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Remove<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, vindex: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), vindex.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFaxOutboundRoutingGroups {
    type Vtable = IFaxOutboundRoutingGroups_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(593280759, 49886, 19453, [184, 218, 117, 9, 124, 130, 200, 127]);
}
impl ::std::convert::From<IFaxOutboundRoutingGroups> for ::windows::runtime::IUnknown {
    fn from(value: IFaxOutboundRoutingGroups) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxOutboundRoutingGroups> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxOutboundRoutingGroups) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxOutboundRoutingGroups {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxOutboundRoutingGroups {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxOutboundRoutingGroups> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxOutboundRoutingGroups) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxOutboundRoutingGroups> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxOutboundRoutingGroups) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxOutboundRoutingGroups {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxOutboundRoutingGroups {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutboundRoutingGroups_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vindex: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxoutboundroutinggroup: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plcount: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxoutboundroutinggroup: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vindex: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxOutboundRoutingRule(::windows::runtime::IUnknown);
impl IFaxOutboundRoutingRule {
    pub unsafe fn CountryCode(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn AreaCode(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn Status(&self) -> ::windows::runtime::Result<FAX_RULE_STATUS_ENUM> {
        let mut result__: <FAX_RULE_STATUS_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_RULE_STATUS_ENUM>(result__)
    }
    pub unsafe fn UseDevice(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetUseDevice(&self, busedevice: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(busedevice)).ok()
    }
    pub unsafe fn DeviceId(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetDeviceId(&self, deviceid: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(deviceid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GroupName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGroupName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrgroupname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), bstrgroupname.into_param().abi()).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Save(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFaxOutboundRoutingRule {
    type Vtable = IFaxOutboundRoutingRule_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3791099349, 1986, 18079, [176, 39, 172, 172, 194, 50, 25, 218]);
}
impl ::std::convert::From<IFaxOutboundRoutingRule> for ::windows::runtime::IUnknown {
    fn from(value: IFaxOutboundRoutingRule) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxOutboundRoutingRule> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxOutboundRoutingRule) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxOutboundRoutingRule {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxOutboundRoutingRule {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxOutboundRoutingRule> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxOutboundRoutingRule) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxOutboundRoutingRule> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxOutboundRoutingRule) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxOutboundRoutingRule {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxOutboundRoutingRule {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutboundRoutingRule_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plcountrycode: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plareacode: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstatus: *mut FAX_RULE_STATUS_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbusedevice: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, busedevice: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pldeviceid: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrgroupname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrgroupname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxOutboundRoutingRules(::windows::runtime::IUnknown);
impl IFaxOutboundRoutingRules {
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    pub unsafe fn Item(&self, lindex: i32) -> ::windows::runtime::Result<IFaxOutboundRoutingRule> {
        let mut result__: <IFaxOutboundRoutingRule as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(lindex), &mut result__).from_abi::<IFaxOutboundRoutingRule>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn ItemByCountryAndArea(&self, lcountrycode: i32, lareacode: i32) -> ::windows::runtime::Result<IFaxOutboundRoutingRule> {
        let mut result__: <IFaxOutboundRoutingRule as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(lcountrycode), ::std::mem::transmute(lareacode), &mut result__).from_abi::<IFaxOutboundRoutingRule>(result__)
    }
    pub unsafe fn RemoveByCountryAndArea(&self, lcountrycode: i32, lareacode: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(lcountrycode), ::std::mem::transmute(lareacode)).ok()
    }
    pub unsafe fn Remove(&self, lindex: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(lindex)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Add<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, lcountrycode: i32, lareacode: i32, busedevice: i16, bstrgroupname: Param3, ldeviceid: i32) -> ::windows::runtime::Result<IFaxOutboundRoutingRule> {
        let mut result__: <IFaxOutboundRoutingRule as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(lcountrycode), ::std::mem::transmute(lareacode), ::std::mem::transmute(busedevice), bstrgroupname.into_param().abi(), ::std::mem::transmute(ldeviceid), &mut result__).from_abi::<IFaxOutboundRoutingRule>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFaxOutboundRoutingRules {
    type Vtable = IFaxOutboundRoutingRules_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3706692071, 44669, 20182, [133, 33, 54, 158, 220, 202, 81, 32]);
}
impl ::std::convert::From<IFaxOutboundRoutingRules> for ::windows::runtime::IUnknown {
    fn from(value: IFaxOutboundRoutingRules) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxOutboundRoutingRules> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxOutboundRoutingRules) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxOutboundRoutingRules {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxOutboundRoutingRules {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxOutboundRoutingRules> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxOutboundRoutingRules) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxOutboundRoutingRules> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxOutboundRoutingRules) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxOutboundRoutingRules {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxOutboundRoutingRules {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutboundRoutingRules_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lindex: i32, pfaxoutboundroutingrule: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plcount: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lcountrycode: i32, lareacode: i32, pfaxoutboundroutingrule: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lcountrycode: i32, lareacode: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lindex: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lcountrycode: i32, lareacode: i32, busedevice: i16, bstrgroupname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, ldeviceid: i32, pfaxoutboundroutingrule: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxOutgoingArchive(::windows::runtime::IUnknown);
impl IFaxOutgoingArchive {
    pub unsafe fn UseArchive(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetUseArchive(&self, busearchive: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(busearchive)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ArchiveFolder(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetArchiveFolder<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrarchivefolder: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), bstrarchivefolder.into_param().abi()).ok()
    }
    pub unsafe fn SizeQuotaWarning(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetSizeQuotaWarning(&self, bsizequotawarning: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(bsizequotawarning)).ok()
    }
    pub unsafe fn HighQuotaWaterMark(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetHighQuotaWaterMark(&self, lhighquotawatermark: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(lhighquotawatermark)).ok()
    }
    pub unsafe fn LowQuotaWaterMark(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetLowQuotaWaterMark(&self, llowquotawatermark: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(llowquotawatermark)).ok()
    }
    pub unsafe fn AgeLimit(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAgeLimit(&self, lagelimit: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(lagelimit)).ok()
    }
    pub unsafe fn SizeLow(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SizeHigh(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Save(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetMessages(&self, lprefetchsize: i32) -> ::windows::runtime::Result<IFaxOutgoingMessageIterator> {
        let mut result__: <IFaxOutgoingMessageIterator as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(lprefetchsize), &mut result__).from_abi::<IFaxOutgoingMessageIterator>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMessage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrmessageid: Param0) -> ::windows::runtime::Result<IFaxOutgoingMessage> {
        let mut result__: <IFaxOutgoingMessage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), bstrmessageid.into_param().abi(), &mut result__).from_abi::<IFaxOutgoingMessage>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFaxOutgoingArchive {
    type Vtable = IFaxOutgoingArchive_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3384971072, 36224, 20051, [129, 15, 154, 121, 145, 155, 73, 253]);
}
impl ::std::convert::From<IFaxOutgoingArchive> for ::windows::runtime::IUnknown {
    fn from(value: IFaxOutgoingArchive) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxOutgoingArchive> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxOutgoingArchive) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxOutgoingArchive {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxOutgoingArchive {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxOutgoingArchive> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxOutgoingArchive) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxOutgoingArchive> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxOutgoingArchive) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxOutgoingArchive {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxOutgoingArchive {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutgoingArchive_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbusearchive: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, busearchive: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrarchivefolder: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrarchivefolder: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbsizequotawarning: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bsizequotawarning: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plhighquotawatermark: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lhighquotawatermark: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pllowquotawatermark: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, llowquotawatermark: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plagelimit: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lagelimit: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plsizelow: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plsizehigh: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lprefetchsize: i32, pfaxoutgoingmessageiterator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrmessageid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxoutgoingmessage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxOutgoingJob(::windows::runtime::IUnknown);
impl IFaxOutgoingJob {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Subject(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DocumentName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Pages(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn Size(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SubmissionId(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn OriginalScheduledTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn SubmissionTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn ReceiptType(&self) -> ::windows::runtime::Result<FAX_RECEIPT_TYPE_ENUM> {
        let mut result__: <FAX_RECEIPT_TYPE_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_RECEIPT_TYPE_ENUM>(result__)
    }
    pub unsafe fn Priority(&self) -> ::windows::runtime::Result<FAX_PRIORITY_TYPE_ENUM> {
        let mut result__: <FAX_PRIORITY_TYPE_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_PRIORITY_TYPE_ENUM>(result__)
    }
    pub unsafe fn Sender(&self) -> ::windows::runtime::Result<IFaxSender> {
        let mut result__: <IFaxSender as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxSender>(result__)
    }
    pub unsafe fn Recipient(&self) -> ::windows::runtime::Result<IFaxRecipient> {
        let mut result__: <IFaxRecipient as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxRecipient>(result__)
    }
    pub unsafe fn CurrentPage(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn DeviceId(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn Status(&self) -> ::windows::runtime::Result<FAX_JOB_STATUS_ENUM> {
        let mut result__: <FAX_JOB_STATUS_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_JOB_STATUS_ENUM>(result__)
    }
    pub unsafe fn ExtendedStatusCode(&self) -> ::windows::runtime::Result<FAX_JOB_EXTENDED_STATUS_ENUM> {
        let mut result__: <FAX_JOB_EXTENDED_STATUS_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_JOB_EXTENDED_STATUS_ENUM>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExtendedStatus(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn AvailableOperations(&self) -> ::windows::runtime::Result<FAX_JOB_OPERATIONS_ENUM> {
        let mut result__: <FAX_JOB_OPERATIONS_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_JOB_OPERATIONS_ENUM>(result__)
    }
    pub unsafe fn Retries(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn ScheduledTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn TransmissionStart(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn TransmissionEnd(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CSID(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TSID(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GroupBroadcastReceipts(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn Pause(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Restart(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTiff<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrtiffpath: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), bstrtiffpath.into_param().abi()).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFaxOutgoingJob {
    type Vtable = IFaxOutgoingJob_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1666636461, 26132, 17795, [191, 122, 58, 214, 123, 191, 199, 28]);
}
impl ::std::convert::From<IFaxOutgoingJob> for ::windows::runtime::IUnknown {
    fn from(value: IFaxOutgoingJob) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxOutgoingJob> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxOutgoingJob) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxOutgoingJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxOutgoingJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxOutgoingJob> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxOutgoingJob) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxOutgoingJob> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxOutgoingJob) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxOutgoingJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxOutgoingJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutgoingJob_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrsubject: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdocumentname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plpages: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plsize: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrsubmissionid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdateoriginalscheduledtime: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatesubmissiontime: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, preceipttype: *mut FAX_RECEIPT_TYPE_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppriority: *mut FAX_PRIORITY_TYPE_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfaxsender: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfaxrecipient: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plcurrentpage: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pldeviceid: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstatus: *mut FAX_JOB_STATUS_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pextendedstatuscode: *mut FAX_JOB_EXTENDED_STATUS_ENUM) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrextendedstatus: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pavailableoperations: *mut FAX_JOB_OPERATIONS_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plretries: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatescheduledtime: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatetransmissionstart: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatetransmissionend: *mut f64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrcsid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrtsid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbgroupbroadcastreceipts: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrtiffpath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxOutgoingJob2(::windows::runtime::IUnknown);
impl IFaxOutgoingJob2 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Subject(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DocumentName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Pages(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn Size(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SubmissionId(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn OriginalScheduledTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn SubmissionTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn ReceiptType(&self) -> ::windows::runtime::Result<FAX_RECEIPT_TYPE_ENUM> {
        let mut result__: <FAX_RECEIPT_TYPE_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_RECEIPT_TYPE_ENUM>(result__)
    }
    pub unsafe fn Priority(&self) -> ::windows::runtime::Result<FAX_PRIORITY_TYPE_ENUM> {
        let mut result__: <FAX_PRIORITY_TYPE_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_PRIORITY_TYPE_ENUM>(result__)
    }
    pub unsafe fn Sender(&self) -> ::windows::runtime::Result<IFaxSender> {
        let mut result__: <IFaxSender as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxSender>(result__)
    }
    pub unsafe fn Recipient(&self) -> ::windows::runtime::Result<IFaxRecipient> {
        let mut result__: <IFaxRecipient as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxRecipient>(result__)
    }
    pub unsafe fn CurrentPage(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn DeviceId(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn Status(&self) -> ::windows::runtime::Result<FAX_JOB_STATUS_ENUM> {
        let mut result__: <FAX_JOB_STATUS_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_JOB_STATUS_ENUM>(result__)
    }
    pub unsafe fn ExtendedStatusCode(&self) -> ::windows::runtime::Result<FAX_JOB_EXTENDED_STATUS_ENUM> {
        let mut result__: <FAX_JOB_EXTENDED_STATUS_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_JOB_EXTENDED_STATUS_ENUM>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExtendedStatus(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn AvailableOperations(&self) -> ::windows::runtime::Result<FAX_JOB_OPERATIONS_ENUM> {
        let mut result__: <FAX_JOB_OPERATIONS_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_JOB_OPERATIONS_ENUM>(result__)
    }
    pub unsafe fn Retries(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn ScheduledTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn TransmissionStart(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn TransmissionEnd(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CSID(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TSID(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GroupBroadcastReceipts(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn Pause(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Restart(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTiff<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrtiffpath: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), bstrtiffpath.into_param().abi()).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn HasCoverPage(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReceiptAddress(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn ScheduleType(&self) -> ::windows::runtime::Result<FAX_SCHEDULE_TYPE_ENUM> {
        let mut result__: <FAX_SCHEDULE_TYPE_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).40)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_SCHEDULE_TYPE_ENUM>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFaxOutgoingJob2 {
    type Vtable = IFaxOutgoingJob2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1099599254, 22944, 18313, [177, 118, 237, 243, 220, 143, 168, 247]);
}
impl ::std::convert::From<IFaxOutgoingJob2> for ::windows::runtime::IUnknown {
    fn from(value: IFaxOutgoingJob2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxOutgoingJob2> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxOutgoingJob2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxOutgoingJob2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxOutgoingJob2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFaxOutgoingJob2> for IFaxOutgoingJob {
    fn from(value: IFaxOutgoingJob2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxOutgoingJob2> for IFaxOutgoingJob {
    fn from(value: &IFaxOutgoingJob2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFaxOutgoingJob> for IFaxOutgoingJob2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFaxOutgoingJob> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFaxOutgoingJob>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFaxOutgoingJob> for &IFaxOutgoingJob2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFaxOutgoingJob> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFaxOutgoingJob>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxOutgoingJob2> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxOutgoingJob2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxOutgoingJob2> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxOutgoingJob2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxOutgoingJob2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxOutgoingJob2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutgoingJob2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrsubject: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdocumentname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plpages: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plsize: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrsubmissionid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdateoriginalscheduledtime: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatesubmissiontime: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, preceipttype: *mut FAX_RECEIPT_TYPE_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppriority: *mut FAX_PRIORITY_TYPE_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfaxsender: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfaxrecipient: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plcurrentpage: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pldeviceid: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstatus: *mut FAX_JOB_STATUS_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pextendedstatuscode: *mut FAX_JOB_EXTENDED_STATUS_ENUM) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrextendedstatus: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pavailableoperations: *mut FAX_JOB_OPERATIONS_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plretries: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatescheduledtime: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatetransmissionstart: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatetransmissionend: *mut f64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrcsid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrtsid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbgroupbroadcastreceipts: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrtiffpath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbhascoverpage: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrreceiptaddress: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pscheduletype: *mut FAX_SCHEDULE_TYPE_ENUM) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxOutgoingJobs(::windows::runtime::IUnknown);
impl IFaxOutgoingJobs {
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Item<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, vindex: Param0) -> ::windows::runtime::Result<IFaxOutgoingJob> {
        let mut result__: <IFaxOutgoingJob as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), vindex.into_param().abi(), &mut result__).from_abi::<IFaxOutgoingJob>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFaxOutgoingJobs {
    type Vtable = IFaxOutgoingJobs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(743889126, 35887, 17779, [148, 76, 229, 5, 248, 245, 174, 237]);
}
impl ::std::convert::From<IFaxOutgoingJobs> for ::windows::runtime::IUnknown {
    fn from(value: IFaxOutgoingJobs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxOutgoingJobs> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxOutgoingJobs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxOutgoingJobs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxOutgoingJobs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxOutgoingJobs> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxOutgoingJobs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxOutgoingJobs> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxOutgoingJobs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxOutgoingJobs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxOutgoingJobs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutgoingJobs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vindex: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxoutgoingjob: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plcount: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxOutgoingMessage(::windows::runtime::IUnknown);
impl IFaxOutgoingMessage {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SubmissionId(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Subject(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DocumentName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Retries(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn Pages(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn Size(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn OriginalScheduledTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn SubmissionTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn Priority(&self) -> ::windows::runtime::Result<FAX_PRIORITY_TYPE_ENUM> {
        let mut result__: <FAX_PRIORITY_TYPE_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_PRIORITY_TYPE_ENUM>(result__)
    }
    pub unsafe fn Sender(&self) -> ::windows::runtime::Result<IFaxSender> {
        let mut result__: <IFaxSender as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxSender>(result__)
    }
    pub unsafe fn Recipient(&self) -> ::windows::runtime::Result<IFaxRecipient> {
        let mut result__: <IFaxRecipient as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxRecipient>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeviceName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn TransmissionStart(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn TransmissionEnd(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CSID(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TSID(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTiff<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrtiffpath: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), bstrtiffpath.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFaxOutgoingMessage {
    type Vtable = IFaxOutgoingMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4041881054, 51877, 19068, [130, 199, 43, 96, 186, 95, 43, 226]);
}
impl ::std::convert::From<IFaxOutgoingMessage> for ::windows::runtime::IUnknown {
    fn from(value: IFaxOutgoingMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxOutgoingMessage> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxOutgoingMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxOutgoingMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxOutgoingMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxOutgoingMessage> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxOutgoingMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxOutgoingMessage> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxOutgoingMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxOutgoingMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxOutgoingMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutgoingMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrsubmissionid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrsubject: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdocumentname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plretries: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plpages: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plsize: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdateoriginalscheduledtime: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatesubmissiontime: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppriority: *mut FAX_PRIORITY_TYPE_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfaxsender: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfaxrecipient: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdevicename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatetransmissionstart: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatetransmissionend: *mut f64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrcsid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrtsid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrtiffpath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxOutgoingMessage2(::windows::runtime::IUnknown);
impl IFaxOutgoingMessage2 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SubmissionId(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Subject(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DocumentName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Retries(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn Pages(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn Size(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn OriginalScheduledTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn SubmissionTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn Priority(&self) -> ::windows::runtime::Result<FAX_PRIORITY_TYPE_ENUM> {
        let mut result__: <FAX_PRIORITY_TYPE_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_PRIORITY_TYPE_ENUM>(result__)
    }
    pub unsafe fn Sender(&self) -> ::windows::runtime::Result<IFaxSender> {
        let mut result__: <IFaxSender as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxSender>(result__)
    }
    pub unsafe fn Recipient(&self) -> ::windows::runtime::Result<IFaxRecipient> {
        let mut result__: <IFaxRecipient as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxRecipient>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeviceName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn TransmissionStart(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn TransmissionEnd(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CSID(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TSID(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTiff<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrtiffpath: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), bstrtiffpath.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn HasCoverPage(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn ReceiptType(&self) -> ::windows::runtime::Result<FAX_RECEIPT_TYPE_ENUM> {
        let mut result__: <FAX_RECEIPT_TYPE_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_RECEIPT_TYPE_ENUM>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReceiptAddress(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Read(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetRead(&self, bread: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), ::std::mem::transmute(bread)).ok()
    }
    pub unsafe fn Save(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFaxOutgoingMessage2 {
    type Vtable = IFaxOutgoingMessage2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3011376775, 48264, 19270, [179, 190, 180, 88, 179, 234, 158, 127]);
}
impl ::std::convert::From<IFaxOutgoingMessage2> for ::windows::runtime::IUnknown {
    fn from(value: IFaxOutgoingMessage2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxOutgoingMessage2> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxOutgoingMessage2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxOutgoingMessage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxOutgoingMessage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFaxOutgoingMessage2> for IFaxOutgoingMessage {
    fn from(value: IFaxOutgoingMessage2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxOutgoingMessage2> for IFaxOutgoingMessage {
    fn from(value: &IFaxOutgoingMessage2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFaxOutgoingMessage> for IFaxOutgoingMessage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFaxOutgoingMessage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFaxOutgoingMessage>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFaxOutgoingMessage> for &IFaxOutgoingMessage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFaxOutgoingMessage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFaxOutgoingMessage>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxOutgoingMessage2> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxOutgoingMessage2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxOutgoingMessage2> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxOutgoingMessage2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxOutgoingMessage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxOutgoingMessage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutgoingMessage2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrsubmissionid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrsubject: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdocumentname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plretries: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plpages: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plsize: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdateoriginalscheduledtime: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatesubmissiontime: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppriority: *mut FAX_PRIORITY_TYPE_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfaxsender: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfaxrecipient: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdevicename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatetransmissionstart: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatetransmissionend: *mut f64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrcsid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrtsid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrtiffpath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbhascoverpage: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, preceipttype: *mut FAX_RECEIPT_TYPE_ENUM) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrreceiptaddress: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbread: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bread: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxOutgoingMessageIterator(::windows::runtime::IUnknown);
impl IFaxOutgoingMessageIterator {
    pub unsafe fn Message(&self) -> ::windows::runtime::Result<IFaxOutgoingMessage> {
        let mut result__: <IFaxOutgoingMessage as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxOutgoingMessage>(result__)
    }
    pub unsafe fn AtEOF(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn PrefetchSize(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPrefetchSize(&self, lprefetchsize: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(lprefetchsize)).ok()
    }
    pub unsafe fn MoveFirst(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn MoveNext(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFaxOutgoingMessageIterator {
    type Vtable = IFaxOutgoingMessageIterator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4125908303, 47168, 17199, [153, 128, 17, 47, 228, 42, 155, 122]);
}
impl ::std::convert::From<IFaxOutgoingMessageIterator> for ::windows::runtime::IUnknown {
    fn from(value: IFaxOutgoingMessageIterator) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxOutgoingMessageIterator> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxOutgoingMessageIterator) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxOutgoingMessageIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxOutgoingMessageIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxOutgoingMessageIterator> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxOutgoingMessageIterator) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxOutgoingMessageIterator> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxOutgoingMessageIterator) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxOutgoingMessageIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxOutgoingMessageIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutgoingMessageIterator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxoutgoingmessage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbeof: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plprefetchsize: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lprefetchsize: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxOutgoingQueue(::windows::runtime::IUnknown);
impl IFaxOutgoingQueue {
    pub unsafe fn Blocked(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetBlocked(&self, bblocked: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(bblocked)).ok()
    }
    pub unsafe fn Paused(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetPaused(&self, bpaused: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(bpaused)).ok()
    }
    pub unsafe fn AllowPersonalCoverPages(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetAllowPersonalCoverPages(&self, ballowpersonalcoverpages: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(ballowpersonalcoverpages)).ok()
    }
    pub unsafe fn UseDeviceTSID(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetUseDeviceTSID(&self, busedevicetsid: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(busedevicetsid)).ok()
    }
    pub unsafe fn Retries(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetRetries(&self, lretries: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(lretries)).ok()
    }
    pub unsafe fn RetryDelay(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetRetryDelay(&self, lretrydelay: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(lretrydelay)).ok()
    }
    pub unsafe fn DiscountRateStart(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn SetDiscountRateStart(&self, datediscountratestart: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(datediscountratestart)).ok()
    }
    pub unsafe fn DiscountRateEnd(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn SetDiscountRateEnd(&self, datediscountrateend: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(datediscountrateend)).ok()
    }
    pub unsafe fn AgeLimit(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAgeLimit(&self, lagelimit: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(lagelimit)).ok()
    }
    pub unsafe fn Branding(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetBranding(&self, bbranding: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), ::std::mem::transmute(bbranding)).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Save(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetJobs(&self) -> ::windows::runtime::Result<IFaxOutgoingJobs> {
        let mut result__: <IFaxOutgoingJobs as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxOutgoingJobs>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetJob<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrjobid: Param0) -> ::windows::runtime::Result<IFaxOutgoingJob> {
        let mut result__: <IFaxOutgoingJob as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), bstrjobid.into_param().abi(), &mut result__).from_abi::<IFaxOutgoingJob>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFaxOutgoingQueue {
    type Vtable = IFaxOutgoingQueue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2159140644, 55724, 17203, [179, 115, 72, 124, 237, 200, 12, 229]);
}
impl ::std::convert::From<IFaxOutgoingQueue> for ::windows::runtime::IUnknown {
    fn from(value: IFaxOutgoingQueue) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxOutgoingQueue> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxOutgoingQueue) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxOutgoingQueue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxOutgoingQueue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxOutgoingQueue> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxOutgoingQueue) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxOutgoingQueue> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxOutgoingQueue) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxOutgoingQueue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxOutgoingQueue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxOutgoingQueue_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbblocked: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bblocked: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbpaused: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bpaused: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pballowpersonalcoverpages: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ballowpersonalcoverpages: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbusedevicetsid: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, busedevicetsid: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plretries: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lretries: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plretrydelay: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lretrydelay: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatediscountratestart: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, datediscountratestart: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatediscountrateend: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, datediscountrateend: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plagelimit: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lagelimit: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbbranding: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bbranding: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxoutgoingjobs: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrjobid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxoutgoingjob: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxReceiptOptions(::windows::runtime::IUnknown);
impl IFaxReceiptOptions {
    pub unsafe fn AuthenticationType(&self) -> ::windows::runtime::Result<FAX_SMTP_AUTHENTICATION_TYPE_ENUM> {
        let mut result__: <FAX_SMTP_AUTHENTICATION_TYPE_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_SMTP_AUTHENTICATION_TYPE_ENUM>(result__)
    }
    pub unsafe fn SetAuthenticationType(&self, r#type: FAX_SMTP_AUTHENTICATION_TYPE_ENUM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(r#type)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SMTPServer(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSMTPServer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsmtpserver: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), bstrsmtpserver.into_param().abi()).ok()
    }
    pub unsafe fn SMTPPort(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetSMTPPort(&self, lsmtpport: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(lsmtpport)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SMTPSender(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSMTPSender<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsmtpsender: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), bstrsmtpsender.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SMTPUser(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSMTPUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsmtpuser: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), bstrsmtpuser.into_param().abi()).ok()
    }
    pub unsafe fn AllowedReceipts(&self) -> ::windows::runtime::Result<FAX_RECEIPT_TYPE_ENUM> {
        let mut result__: <FAX_RECEIPT_TYPE_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_RECEIPT_TYPE_ENUM>(result__)
    }
    pub unsafe fn SetAllowedReceipts(&self, allowedreceipts: FAX_RECEIPT_TYPE_ENUM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(allowedreceipts)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SMTPPassword(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSMTPPassword<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsmtppassword: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), bstrsmtppassword.into_param().abi()).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Save(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn UseForInboundRouting(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetUseForInboundRouting(&self, buseforinboundrouting: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(buseforinboundrouting)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFaxReceiptOptions {
    type Vtable = IFaxReceiptOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(932117227, 24523, 19195, [178, 238, 225, 110, 128, 97, 68, 135]);
}
impl ::std::convert::From<IFaxReceiptOptions> for ::windows::runtime::IUnknown {
    fn from(value: IFaxReceiptOptions) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxReceiptOptions> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxReceiptOptions) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxReceiptOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxReceiptOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxReceiptOptions> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxReceiptOptions) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxReceiptOptions> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxReceiptOptions) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxReceiptOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxReceiptOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxReceiptOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptype: *mut FAX_SMTP_AUTHENTICATION_TYPE_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: FAX_SMTP_AUTHENTICATION_TYPE_ENUM) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrsmtpserver: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrsmtpserver: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plsmtpport: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lsmtpport: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrsmtpsender: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrsmtpsender: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrsmtpuser: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrsmtpuser: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pallowedreceipts: *mut FAX_RECEIPT_TYPE_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, allowedreceipts: FAX_RECEIPT_TYPE_ENUM) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrsmtppassword: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrsmtppassword: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbuseforinboundrouting: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buseforinboundrouting: i16) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxRecipient(::windows::runtime::IUnknown);
impl IFaxRecipient {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FaxNumber(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFaxNumber<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrfaxnumber: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), bstrfaxnumber.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), bstrname.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFaxRecipient {
    type Vtable = IFaxRecipient_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2587730848, 21389, 17078, [148, 68, 170, 165, 125, 12, 226, 188]);
}
impl ::std::convert::From<IFaxRecipient> for ::windows::runtime::IUnknown {
    fn from(value: IFaxRecipient) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxRecipient> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxRecipient) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxRecipient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxRecipient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxRecipient> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxRecipient) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxRecipient> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxRecipient) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxRecipient {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxRecipient {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxRecipient_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrfaxnumber: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrfaxnumber: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxRecipients(::windows::runtime::IUnknown);
impl IFaxRecipients {
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    pub unsafe fn Item(&self, lindex: i32) -> ::windows::runtime::Result<IFaxRecipient> {
        let mut result__: <IFaxRecipient as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(lindex), &mut result__).from_abi::<IFaxRecipient>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Add<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrfaxnumber: Param0, bstrrecipientname: Param1) -> ::windows::runtime::Result<IFaxRecipient> {
        let mut result__: <IFaxRecipient as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), bstrfaxnumber.into_param().abi(), bstrrecipientname.into_param().abi(), &mut result__).from_abi::<IFaxRecipient>(result__)
    }
    pub unsafe fn Remove(&self, lindex: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(lindex)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFaxRecipients {
    type Vtable = IFaxRecipients_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3117014618, 35150, 17554, [159, 163, 8, 198, 39, 193, 29, 93]);
}
impl ::std::convert::From<IFaxRecipients> for ::windows::runtime::IUnknown {
    fn from(value: IFaxRecipients) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxRecipients> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxRecipients) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxRecipients {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxRecipients {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxRecipients> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxRecipients) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxRecipients> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxRecipients) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxRecipients {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxRecipients {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxRecipients_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lindex: i32, ppfaxrecipient: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plcount: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrfaxnumber: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrecipientname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfaxrecipient: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lindex: i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxSecurity(::windows::runtime::IUnknown);
impl IFaxSecurity {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Descriptor(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn SetDescriptor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, vdescriptor: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), vdescriptor.into_param().abi()).ok()
    }
    pub unsafe fn GrantedRights(&self) -> ::windows::runtime::Result<FAX_ACCESS_RIGHTS_ENUM> {
        let mut result__: <FAX_ACCESS_RIGHTS_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_ACCESS_RIGHTS_ENUM>(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Save(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn InformationType(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetInformationType(&self, linformationtype: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(linformationtype)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFaxSecurity {
    type Vtable = IFaxSecurity_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2008352961, 2496, 18338, [145, 235, 252, 231, 253, 242, 105, 14]);
}
impl ::std::convert::From<IFaxSecurity> for ::windows::runtime::IUnknown {
    fn from(value: IFaxSecurity) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxSecurity> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxSecurity) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxSecurity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxSecurity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxSecurity> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxSecurity) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxSecurity> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxSecurity) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxSecurity {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxSecurity {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxSecurity_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvdescriptor: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vdescriptor: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pgrantedrights: *mut FAX_ACCESS_RIGHTS_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plinformationtype: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, linformationtype: i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxSecurity2(::windows::runtime::IUnknown);
impl IFaxSecurity2 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Descriptor(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn SetDescriptor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, vdescriptor: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), vdescriptor.into_param().abi()).ok()
    }
    pub unsafe fn GrantedRights(&self) -> ::windows::runtime::Result<FAX_ACCESS_RIGHTS_ENUM_2> {
        let mut result__: <FAX_ACCESS_RIGHTS_ENUM_2 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_ACCESS_RIGHTS_ENUM_2>(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Save(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn InformationType(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetInformationType(&self, linformationtype: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(linformationtype)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFaxSecurity2 {
    type Vtable = IFaxSecurity2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(400052724, 53403, 18684, [153, 201, 143, 36, 196, 219, 154, 177]);
}
impl ::std::convert::From<IFaxSecurity2> for ::windows::runtime::IUnknown {
    fn from(value: IFaxSecurity2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxSecurity2> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxSecurity2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxSecurity2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxSecurity2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxSecurity2> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxSecurity2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxSecurity2> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxSecurity2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxSecurity2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxSecurity2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxSecurity2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvdescriptor: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vdescriptor: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pgrantedrights: *mut FAX_ACCESS_RIGHTS_ENUM_2) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plinformationtype: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, linformationtype: i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxSender(::windows::runtime::IUnknown);
impl IFaxSender {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BillingCode(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBillingCode<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrbillingcode: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), bstrbillingcode.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn City(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCity<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrcity: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), bstrcity.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Company(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCompany<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrcompany: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), bstrcompany.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Country(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCountry<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrcountry: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), bstrcountry.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Department(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDepartment<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdepartment: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), bstrdepartment.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Email(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEmail<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstremail: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), bstremail.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FaxNumber(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFaxNumber<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrfaxnumber: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), bstrfaxnumber.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HomePhone(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetHomePhone<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrhomephone: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), bstrhomephone.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), bstrname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TSID(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTSID<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrtsid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), bstrtsid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OfficePhone(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOfficePhone<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrofficephone: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), bstrofficephone.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OfficeLocation(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOfficeLocation<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrofficelocation: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), bstrofficelocation.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn State(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetState<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrstate: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), bstrstate.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StreetAddress(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStreetAddress<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrstreetaddress: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), bstrstreetaddress.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Title(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTitle<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrtitle: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self), bstrtitle.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ZipCode(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetZipCode<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrzipcode: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self), bstrzipcode.into_param().abi()).ok()
    }
    pub unsafe fn LoadDefaultSender(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SaveDefaultSender(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFaxSender {
    type Vtable = IFaxSender_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(226991485, 62842, 19654, [166, 249, 62, 229, 213, 39, 180, 106]);
}
impl ::std::convert::From<IFaxSender> for ::windows::runtime::IUnknown {
    fn from(value: IFaxSender) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxSender> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxSender) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxSender {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxSender {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxSender> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxSender) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxSender> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxSender) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxSender {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxSender {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxSender_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrbillingcode: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrbillingcode: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrcity: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrcity: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrcompany: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrcompany: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrcountry: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrcountry: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdepartment: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrdepartment: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstremail: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstremail: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrfaxnumber: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrfaxnumber: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrhomephone: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrhomephone: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrtsid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrtsid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrofficephone: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrofficephone: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrofficelocation: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrofficelocation: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrstate: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrstate: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrstreetaddress: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrstreetaddress: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrtitle: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrtitle: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrzipcode: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrzipcode: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxServer(::windows::runtime::IUnknown);
impl IFaxServer {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Connect<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrservername: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), bstrservername.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServerName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetDeviceProviders(&self) -> ::windows::runtime::Result<IFaxDeviceProviders> {
        let mut result__: <IFaxDeviceProviders as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxDeviceProviders>(result__)
    }
    pub unsafe fn GetDevices(&self) -> ::windows::runtime::Result<IFaxDevices> {
        let mut result__: <IFaxDevices as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxDevices>(result__)
    }
    pub unsafe fn InboundRouting(&self) -> ::windows::runtime::Result<IFaxInboundRouting> {
        let mut result__: <IFaxInboundRouting as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxInboundRouting>(result__)
    }
    pub unsafe fn Folders(&self) -> ::windows::runtime::Result<IFaxFolders> {
        let mut result__: <IFaxFolders as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxFolders>(result__)
    }
    pub unsafe fn LoggingOptions(&self) -> ::windows::runtime::Result<IFaxLoggingOptions> {
        let mut result__: <IFaxLoggingOptions as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxLoggingOptions>(result__)
    }
    pub unsafe fn MajorVersion(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn MinorVersion(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn MajorBuild(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn MinorBuild(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn Debug(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn Activity(&self) -> ::windows::runtime::Result<IFaxActivity> {
        let mut result__: <IFaxActivity as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxActivity>(result__)
    }
    pub unsafe fn OutboundRouting(&self) -> ::windows::runtime::Result<IFaxOutboundRouting> {
        let mut result__: <IFaxOutboundRouting as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxOutboundRouting>(result__)
    }
    pub unsafe fn ReceiptOptions(&self) -> ::windows::runtime::Result<IFaxReceiptOptions> {
        let mut result__: <IFaxReceiptOptions as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxReceiptOptions>(result__)
    }
    pub unsafe fn Security(&self) -> ::windows::runtime::Result<IFaxSecurity> {
        let mut result__: <IFaxSecurity as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxSecurity>(result__)
    }
    pub unsafe fn Disconnect(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn GetExtensionProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrguid: Param0) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), bstrguid.into_param().abi(), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn SetExtensionProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, bstrguid: Param0, vproperty: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), bstrguid.into_param().abi(), vproperty.into_param().abi()).ok()
    }
    pub unsafe fn ListenToServerEvents(&self, eventtypes: FAX_SERVER_EVENTS_TYPE_ENUM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), ::std::mem::transmute(eventtypes)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterDeviceProvider<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrguid: Param0, bstrfriendlyname: Param1, bstrimagename: Param2, tspname: Param3, lfspiversion: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), bstrguid.into_param().abi(), bstrfriendlyname.into_param().abi(), bstrimagename.into_param().abi(), tspname.into_param().abi(), ::std::mem::transmute(lfspiversion)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UnregisterDeviceProvider<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstruniquename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), bstruniquename.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn RegisterInboundRoutingExtension<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, bstrextensionname: Param0, bstrfriendlyname: Param1, bstrimagename: Param2, vmethods: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), bstrextensionname.into_param().abi(), bstrfriendlyname.into_param().abi(), bstrimagename.into_param().abi(), vmethods.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UnregisterInboundRoutingExtension<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrextensionuniquename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), bstrextensionuniquename.into_param().abi()).ok()
    }
    pub unsafe fn RegisteredEvents(&self) -> ::windows::runtime::Result<FAX_SERVER_EVENTS_TYPE_ENUM> {
        let mut result__: <FAX_SERVER_EVENTS_TYPE_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_SERVER_EVENTS_TYPE_ENUM>(result__)
    }
    pub unsafe fn APIVersion(&self) -> ::windows::runtime::Result<FAX_SERVER_APIVERSION_ENUM> {
        let mut result__: <FAX_SERVER_APIVERSION_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_SERVER_APIVERSION_ENUM>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFaxServer {
    type Vtable = IFaxServer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1197171817, 37029, 18552, [165, 119, 23, 168, 110, 142, 52, 98]);
}
impl ::std::convert::From<IFaxServer> for ::windows::runtime::IUnknown {
    fn from(value: IFaxServer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxServer> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxServer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxServer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxServer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxServer> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxServer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxServer> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxServer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxServer {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxServer {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxServer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrservername: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrservername: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfaxdeviceproviders: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfaxdevices: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfaxinboundrouting: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxfolders: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfaxloggingoptions: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plmajorversion: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plminorversion: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plmajorbuild: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plminorbuild: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbdebug: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfaxactivity: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfaxoutboundrouting: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfaxreceiptoptions: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfaxsecurity: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrguid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvproperty: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrguid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, vproperty: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventtypes: FAX_SERVER_EVENTS_TYPE_ENUM) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrguid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfriendlyname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrimagename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, tspname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, lfspiversion: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstruniquename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrextensionname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfriendlyname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrimagename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, vmethods: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrextensionuniquename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, peventtypes: *mut FAX_SERVER_EVENTS_TYPE_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, papiversion: *mut FAX_SERVER_APIVERSION_ENUM) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxServer2(::windows::runtime::IUnknown);
impl IFaxServer2 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Connect<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrservername: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), bstrservername.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServerName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetDeviceProviders(&self) -> ::windows::runtime::Result<IFaxDeviceProviders> {
        let mut result__: <IFaxDeviceProviders as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxDeviceProviders>(result__)
    }
    pub unsafe fn GetDevices(&self) -> ::windows::runtime::Result<IFaxDevices> {
        let mut result__: <IFaxDevices as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxDevices>(result__)
    }
    pub unsafe fn InboundRouting(&self) -> ::windows::runtime::Result<IFaxInboundRouting> {
        let mut result__: <IFaxInboundRouting as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxInboundRouting>(result__)
    }
    pub unsafe fn Folders(&self) -> ::windows::runtime::Result<IFaxFolders> {
        let mut result__: <IFaxFolders as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxFolders>(result__)
    }
    pub unsafe fn LoggingOptions(&self) -> ::windows::runtime::Result<IFaxLoggingOptions> {
        let mut result__: <IFaxLoggingOptions as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxLoggingOptions>(result__)
    }
    pub unsafe fn MajorVersion(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn MinorVersion(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn MajorBuild(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn MinorBuild(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn Debug(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn Activity(&self) -> ::windows::runtime::Result<IFaxActivity> {
        let mut result__: <IFaxActivity as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxActivity>(result__)
    }
    pub unsafe fn OutboundRouting(&self) -> ::windows::runtime::Result<IFaxOutboundRouting> {
        let mut result__: <IFaxOutboundRouting as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxOutboundRouting>(result__)
    }
    pub unsafe fn ReceiptOptions(&self) -> ::windows::runtime::Result<IFaxReceiptOptions> {
        let mut result__: <IFaxReceiptOptions as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxReceiptOptions>(result__)
    }
    pub unsafe fn Security(&self) -> ::windows::runtime::Result<IFaxSecurity> {
        let mut result__: <IFaxSecurity as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxSecurity>(result__)
    }
    pub unsafe fn Disconnect(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn GetExtensionProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrguid: Param0) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), bstrguid.into_param().abi(), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn SetExtensionProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, bstrguid: Param0, vproperty: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), bstrguid.into_param().abi(), vproperty.into_param().abi()).ok()
    }
    pub unsafe fn ListenToServerEvents(&self, eventtypes: FAX_SERVER_EVENTS_TYPE_ENUM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), ::std::mem::transmute(eventtypes)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterDeviceProvider<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrguid: Param0, bstrfriendlyname: Param1, bstrimagename: Param2, tspname: Param3, lfspiversion: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), bstrguid.into_param().abi(), bstrfriendlyname.into_param().abi(), bstrimagename.into_param().abi(), tspname.into_param().abi(), ::std::mem::transmute(lfspiversion)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UnregisterDeviceProvider<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstruniquename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), bstruniquename.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn RegisterInboundRoutingExtension<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, bstrextensionname: Param0, bstrfriendlyname: Param1, bstrimagename: Param2, vmethods: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), bstrextensionname.into_param().abi(), bstrfriendlyname.into_param().abi(), bstrimagename.into_param().abi(), vmethods.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UnregisterInboundRoutingExtension<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrextensionuniquename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), bstrextensionuniquename.into_param().abi()).ok()
    }
    pub unsafe fn RegisteredEvents(&self) -> ::windows::runtime::Result<FAX_SERVER_EVENTS_TYPE_ENUM> {
        let mut result__: <FAX_SERVER_EVENTS_TYPE_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_SERVER_EVENTS_TYPE_ENUM>(result__)
    }
    pub unsafe fn APIVersion(&self) -> ::windows::runtime::Result<FAX_SERVER_APIVERSION_ENUM> {
        let mut result__: <FAX_SERVER_APIVERSION_ENUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FAX_SERVER_APIVERSION_ENUM>(result__)
    }
    pub unsafe fn Configuration(&self) -> ::windows::runtime::Result<IFaxConfiguration> {
        let mut result__: <IFaxConfiguration as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxConfiguration>(result__)
    }
    pub unsafe fn CurrentAccount(&self) -> ::windows::runtime::Result<IFaxAccount> {
        let mut result__: <IFaxAccount as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxAccount>(result__)
    }
    pub unsafe fn FaxAccountSet(&self) -> ::windows::runtime::Result<IFaxAccountSet> {
        let mut result__: <IFaxAccountSet as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxAccountSet>(result__)
    }
    pub unsafe fn Security2(&self) -> ::windows::runtime::Result<IFaxSecurity2> {
        let mut result__: <IFaxSecurity2 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFaxSecurity2>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFaxServer2 {
    type Vtable = IFaxServer2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1461513487, 22025, 20288, [145, 118, 84, 126, 58, 114, 202, 124]);
}
impl ::std::convert::From<IFaxServer2> for ::windows::runtime::IUnknown {
    fn from(value: IFaxServer2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxServer2> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxServer2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxServer2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxServer2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFaxServer2> for IFaxServer {
    fn from(value: IFaxServer2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxServer2> for IFaxServer {
    fn from(value: &IFaxServer2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFaxServer> for IFaxServer2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFaxServer> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFaxServer>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFaxServer> for &IFaxServer2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFaxServer> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFaxServer>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxServer2> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxServer2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxServer2> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxServer2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxServer2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxServer2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxServer2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrservername: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrservername: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfaxdeviceproviders: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfaxdevices: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfaxinboundrouting: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxfolders: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfaxloggingoptions: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plmajorversion: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plminorversion: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plmajorbuild: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plminorbuild: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbdebug: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfaxactivity: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfaxoutboundrouting: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfaxreceiptoptions: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfaxsecurity: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrguid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvproperty: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrguid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, vproperty: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventtypes: FAX_SERVER_EVENTS_TYPE_ENUM) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrguid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfriendlyname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrimagename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, tspname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, lfspiversion: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstruniquename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrextensionname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfriendlyname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrimagename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, vmethods: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrextensionuniquename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, peventtypes: *mut FAX_SERVER_EVENTS_TYPE_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, papiversion: *mut FAX_SERVER_APIVERSION_ENUM) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfaxconfiguration: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcurrentaccount: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfaxaccountset: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfaxsecurity2: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxServerNotify(::windows::runtime::IUnknown);
impl IFaxServerNotify {}
unsafe impl ::windows::runtime::Interface for IFaxServerNotify {
    type Vtable = IFaxServerNotify_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(771980071, 53130, 19133, [177, 224, 87, 4, 148, 59, 234, 111]);
}
impl ::std::convert::From<IFaxServerNotify> for ::windows::runtime::IUnknown {
    fn from(value: IFaxServerNotify) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxServerNotify> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxServerNotify) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxServerNotify {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxServerNotify {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxServerNotify> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxServerNotify) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxServerNotify> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxServerNotify) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxServerNotify {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxServerNotify {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxServerNotify_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFaxServerNotify2(::windows::runtime::IUnknown);
impl IFaxServerNotify2 {}
unsafe impl ::windows::runtime::Interface for IFaxServerNotify2 {
    type Vtable = IFaxServerNotify2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1634511062, 42874, 16482, [171, 253, 14, 71, 18, 65, 199, 170]);
}
impl ::std::convert::From<IFaxServerNotify2> for ::windows::runtime::IUnknown {
    fn from(value: IFaxServerNotify2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFaxServerNotify2> for ::windows::runtime::IUnknown {
    fn from(value: &IFaxServerNotify2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFaxServerNotify2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFaxServerNotify2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFaxServerNotify2> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFaxServerNotify2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFaxServerNotify2> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFaxServerNotify2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFaxServerNotify2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFaxServerNotify2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaxServerNotify2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
pub const IS_DIGITAL_CAMERA_VAL: u32 = 1u32;
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IStiDevice(::windows::runtime::IUnknown);
impl IStiDevice {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, hinst: Param0, pwszdevicename: Param1, dwversion: u32, dwmode: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), hinst.into_param().abi(), pwszdevicename.into_param().abi(), ::std::mem::transmute(dwversion), ::std::mem::transmute(dwmode)).ok()
    }
    pub unsafe fn GetCapabilities(&self, pdevcaps: *mut STI_DEV_CAPS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdevcaps)).ok()
    }
    pub unsafe fn GetStatus(&self, pdevstatus: *mut STI_DEVICE_STATUS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdevstatus)).ok()
    }
    pub unsafe fn DeviceReset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Diagnostic(&self, pbuffer: *mut STI_DIAG) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbuffer)).ok()
    }
    pub unsafe fn Escape(&self, escapefunction: u32, lpindata: *const ::std::ffi::c_void, cbindatasize: u32, poutdata: *mut ::std::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(escapefunction), ::std::mem::transmute(lpindata), ::std::mem::transmute(cbindatasize), ::std::mem::transmute(poutdata), ::std::mem::transmute(dwoutdatasize), ::std::mem::transmute(pdwactualdata)).ok()
    }
    pub unsafe fn GetLastError(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn LockDevice(&self, dwtimeout: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwtimeout)).ok()
    }
    pub unsafe fn UnLockDevice(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn RawReadData(&self, lpbuffer: *mut ::std::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(lpbuffer), ::std::mem::transmute(lpdwnumberofbytes), ::std::mem::transmute(lpoverlapped)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn RawWriteData(&self, lpbuffer: *const ::std::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(lpbuffer), ::std::mem::transmute(nnumberofbytes), ::std::mem::transmute(lpoverlapped)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn RawReadCommand(&self, lpbuffer: *mut ::std::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(lpbuffer), ::std::mem::transmute(lpdwnumberofbytes), ::std::mem::transmute(lpoverlapped)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn RawWriteCommand(&self, lpbuffer: *const ::std::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(lpbuffer), ::std::mem::transmute(nnumberofbytes), ::std::mem::transmute(lpoverlapped)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Subscribe(&self, lpsubsribe: *mut STISUBSCRIBE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(lpsubsribe)).ok()
    }
    pub unsafe fn GetLastNotificationData(&self) -> ::windows::runtime::Result<STINOTIFY> {
        let mut result__: <STINOTIFY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<STINOTIFY>(result__)
    }
    pub unsafe fn UnSubscribe(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetLastErrorInfo(&self) -> ::windows::runtime::Result<_ERROR_INFOW> {
        let mut result__: <_ERROR_INFOW as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<_ERROR_INFOW>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IStiDevice {
    type Vtable = IStiDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1828346496, 11720, 4560, [144, 234, 0, 170, 0, 96, 248, 108]);
}
impl ::std::convert::From<IStiDevice> for ::windows::runtime::IUnknown {
    fn from(value: IStiDevice) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IStiDevice> for ::windows::runtime::IUnknown {
    fn from(value: &IStiDevice) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IStiDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IStiDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStiDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hinst: super::super::Foundation::HINSTANCE, pwszdevicename: super::super::Foundation::PWSTR, dwversion: u32, dwmode: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdevcaps: *mut STI_DEV_CAPS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdevstatus: *mut STI_DEVICE_STATUS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbuffer: *mut STI_DIAG) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, escapefunction: u32, lpindata: *const ::std::ffi::c_void, cbindatasize: u32, poutdata: *mut ::std::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwlastdeviceerror: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwtimeout: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpbuffer: *mut ::std::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpbuffer: *const ::std::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpbuffer: *mut ::std::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpbuffer: *const ::std::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpsubsribe: *mut STISUBSCRIBE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpnotify: *mut STINOTIFY) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plasterrorinfo: *mut _ERROR_INFOW) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IStiDeviceControl(::windows::runtime::IUnknown);
impl IStiDeviceControl {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwdevicetype: u32, dwmode: u32, pwszportname: Param2, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwdevicetype), ::std::mem::transmute(dwmode), pwszportname.into_param().abi(), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn RawReadData(&self, lpbuffer: *mut ::std::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(lpbuffer), ::std::mem::transmute(lpdwnumberofbytes), ::std::mem::transmute(lpoverlapped)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn RawWriteData(&self, lpbuffer: *mut ::std::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(lpbuffer), ::std::mem::transmute(nnumberofbytes), ::std::mem::transmute(lpoverlapped)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn RawReadCommand(&self, lpbuffer: *mut ::std::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(lpbuffer), ::std::mem::transmute(lpdwnumberofbytes), ::std::mem::transmute(lpoverlapped)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn RawWriteCommand(&self, lpbuffer: *mut ::std::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(lpbuffer), ::std::mem::transmute(nnumberofbytes), ::std::mem::transmute(lpoverlapped)).ok()
    }
    pub unsafe fn RawDeviceControl(&self, escapefunction: u32, lpindata: *mut ::std::ffi::c_void, cbindatasize: u32, poutdata: *mut ::std::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(escapefunction), ::std::mem::transmute(lpindata), ::std::mem::transmute(cbindatasize), ::std::mem::transmute(poutdata), ::std::mem::transmute(dwoutdatasize), ::std::mem::transmute(pdwactualdata)).ok()
    }
    pub unsafe fn GetLastError(&self, lpdwlasterror: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(lpdwlasterror)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMyDevicePortName(&self, lpszdevicepath: super::super::Foundation::PWSTR, cwdevicepathsize: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(lpszdevicepath), ::std::mem::transmute(cwdevicepathsize)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMyDeviceHandle(&self, lph: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(lph)).ok()
    }
    pub unsafe fn GetMyDeviceOpenMode(&self, pdwopenmode: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwopenmode)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteToErrorLog<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwmessagetype: u32, pszmessage: Param1, dwerrorcode: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwmessagetype), pszmessage.into_param().abi(), ::std::mem::transmute(dwerrorcode)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IStiDeviceControl {
    type Vtable = IStiDeviceControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(311072864, 21212, 4560, [158, 223, 68, 69, 83, 84, 0, 0]);
}
impl ::std::convert::From<IStiDeviceControl> for ::windows::runtime::IUnknown {
    fn from(value: IStiDeviceControl) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IStiDeviceControl> for ::windows::runtime::IUnknown {
    fn from(value: &IStiDeviceControl) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IStiDeviceControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IStiDeviceControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStiDeviceControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwdevicetype: u32, dwmode: u32, pwszportname: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpbuffer: *mut ::std::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpbuffer: *mut ::std::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpbuffer: *mut ::std::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpbuffer: *mut ::std::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, escapefunction: u32, lpindata: *mut ::std::ffi::c_void, cbindatasize: u32, poutdata: *mut ::std::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpdwlasterror: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpszdevicepath: super::super::Foundation::PWSTR, cwdevicepathsize: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lph: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwopenmode: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwmessagetype: u32, pszmessage: super::super::Foundation::PWSTR, dwerrorcode: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct IStiDeviceW(pub u8);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IStiUSD(::windows::runtime::IUnknown);
impl IStiUSD {
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, IStiDeviceControl>, Param2: ::windows::runtime::IntoParam<'a, super::super::System::Registry::HKEY>>(&self, pheldcb: Param0, dwstiversion: u32, hparameterskey: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pheldcb.into_param().abi(), ::std::mem::transmute(dwstiversion), hparameterskey.into_param().abi()).ok()
    }
    pub unsafe fn GetCapabilities(&self) -> ::windows::runtime::Result<STI_USD_CAPS> {
        let mut result__: <STI_USD_CAPS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<STI_USD_CAPS>(result__)
    }
    pub unsafe fn GetStatus(&self, pdevstatus: *mut STI_DEVICE_STATUS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdevstatus)).ok()
    }
    pub unsafe fn DeviceReset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Diagnostic(&self, pbuffer: *mut STI_DIAG) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbuffer)).ok()
    }
    pub unsafe fn Escape(&self, escapefunction: u32, lpindata: *const ::std::ffi::c_void, cbindatasize: u32, poutdata: *mut ::std::ffi::c_void, cboutdatasize: u32, pdwactualdata: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(escapefunction), ::std::mem::transmute(lpindata), ::std::mem::transmute(cbindatasize), ::std::mem::transmute(poutdata), ::std::mem::transmute(cboutdatasize), ::std::mem::transmute(pdwactualdata)).ok()
    }
    pub unsafe fn GetLastError(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn LockDevice(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn UnLockDevice(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn RawReadData(&self, lpbuffer: *mut ::std::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(lpbuffer), ::std::mem::transmute(lpdwnumberofbytes), ::std::mem::transmute(lpoverlapped)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn RawWriteData(&self, lpbuffer: *const ::std::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(lpbuffer), ::std::mem::transmute(nnumberofbytes), ::std::mem::transmute(lpoverlapped)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn RawReadCommand(&self, lpbuffer: *mut ::std::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(lpbuffer), ::std::mem::transmute(lpdwnumberofbytes), ::std::mem::transmute(lpoverlapped)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn RawWriteCommand(&self, lpbuffer: *const ::std::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(lpbuffer), ::std::mem::transmute(nnumberofbytes), ::std::mem::transmute(lpoverlapped)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNotificationHandle<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, hevent: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), hevent.into_param().abi()).ok()
    }
    pub unsafe fn GetNotificationData(&self) -> ::windows::runtime::Result<STINOTIFY> {
        let mut result__: <STINOTIFY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<STINOTIFY>(result__)
    }
    pub unsafe fn GetLastErrorInfo(&self) -> ::windows::runtime::Result<_ERROR_INFOW> {
        let mut result__: <_ERROR_INFOW as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<_ERROR_INFOW>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IStiUSD {
    type Vtable = IStiUSD_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(211530848, 20908, 4560, [144, 234, 0, 170, 0, 96, 248, 108]);
}
impl ::std::convert::From<IStiUSD> for ::windows::runtime::IUnknown {
    fn from(value: IStiUSD) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IStiUSD> for ::windows::runtime::IUnknown {
    fn from(value: &IStiUSD) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IStiUSD {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IStiUSD {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStiUSD_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Registry")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pheldcb: ::windows::runtime::RawPtr, dwstiversion: u32, hparameterskey: super::super::System::Registry::HKEY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdevcaps: *mut STI_USD_CAPS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdevstatus: *mut STI_DEVICE_STATUS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbuffer: *mut STI_DIAG) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, escapefunction: u32, lpindata: *const ::std::ffi::c_void, cbindatasize: u32, poutdata: *mut ::std::ffi::c_void, cboutdatasize: u32, pdwactualdata: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwlastdeviceerror: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpbuffer: *mut ::std::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpbuffer: *const ::std::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpbuffer: *mut ::std::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpbuffer: *const ::std::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hevent: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpnotify: *mut STINOTIFY) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plasterrorinfo: *mut _ERROR_INFOW) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IStillImageW(::windows::runtime::IUnknown);
impl IStillImageW {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>>(&self, hinst: Param0, dwversion: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), hinst.into_param().abi(), ::std::mem::transmute(dwversion)).ok()
    }
    pub unsafe fn GetDeviceList(&self, dwtype: u32, dwflags: u32, pdwitemsreturned: *mut u32, ppbuffer: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwtype), ::std::mem::transmute(dwflags), ::std::mem::transmute(pdwitemsreturned), ::std::mem::transmute(ppbuffer)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDeviceInfo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszdevicename: Param0, ppbuffer: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pwszdevicename.into_param().abi(), ::std::mem::transmute(ppbuffer)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateDevice<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pwszdevicename: Param0, dwmode: u32, pdevice: *mut ::std::option::Option<IStiDevice>, punkouter: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pwszdevicename.into_param().abi(), ::std::mem::transmute(dwmode), ::std::mem::transmute(pdevice), punkouter.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDeviceValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszdevicename: Param0, pvaluename: Param1, ptype: *mut u32, pdata: *mut u8, cbdata: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pwszdevicename.into_param().abi(), pvaluename.into_param().abi(), ::std::mem::transmute(ptype), ::std::mem::transmute(pdata), ::std::mem::transmute(cbdata)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDeviceValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszdevicename: Param0, pvaluename: Param1, r#type: u32, pdata: *const u8, cbdata: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), pwszdevicename.into_param().abi(), pvaluename.into_param().abi(), ::std::mem::transmute(r#type), ::std::mem::transmute(pdata), ::std::mem::transmute(cbdata)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSTILaunchInformation(&self, pwszdevicename: super::super::Foundation::PWSTR, pdweventcode: *mut u32, pwszeventname: super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszdevicename), ::std::mem::transmute(pdweventcode), ::std::mem::transmute(pwszeventname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterLaunchApplication<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszappname: Param0, pwszcommandline: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), pwszappname.into_param().abi(), pwszcommandline.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UnregisterLaunchApplication<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszappname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), pwszappname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableHwNotifications<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pwszdevicename: Param0, bnewstate: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), pwszdevicename.into_param().abi(), bnewstate.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHwNotificationState<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszdevicename: Param0) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), pwszdevicename.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RefreshDeviceBus<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszdevicename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), pwszdevicename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LaunchApplicationForDevice<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszdevicename: Param0, pwszappname: Param1, pstinotify: *const STINOTIFY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), pwszdevicename.into_param().abi(), pwszappname.into_param().abi(), ::std::mem::transmute(pstinotify)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetupDeviceParameters(&self, param0: *mut STI_DEVICE_INFORMATIONW) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteToErrorLog<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwmessagetype: u32, pszmessage: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwmessagetype), pszmessage.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IStillImageW {
    type Vtable = IStillImageW_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1679546496, 11720, 4560, [144, 234, 0, 170, 0, 96, 248, 108]);
}
impl ::std::convert::From<IStillImageW> for ::windows::runtime::IUnknown {
    fn from(value: IStillImageW) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IStillImageW> for ::windows::runtime::IUnknown {
    fn from(value: &IStillImageW) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IStillImageW {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IStillImageW {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStillImageW_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hinst: super::super::Foundation::HINSTANCE, dwversion: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwtype: u32, dwflags: u32, pdwitemsreturned: *mut u32, ppbuffer: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszdevicename: super::super::Foundation::PWSTR, ppbuffer: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszdevicename: super::super::Foundation::PWSTR, dwmode: u32, pdevice: *mut ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszdevicename: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, ptype: *mut u32, pdata: *mut u8, cbdata: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszdevicename: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, r#type: u32, pdata: *const u8, cbdata: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszdevicename: super::super::Foundation::PWSTR, pdweventcode: *mut u32, pwszeventname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszappname: super::super::Foundation::PWSTR, pwszcommandline: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszappname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszdevicename: super::super::Foundation::PWSTR, bnewstate: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszdevicename: super::super::Foundation::PWSTR, pbcurrentstate: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszdevicename: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszdevicename: super::super::Foundation::PWSTR, pwszappname: super::super::Foundation::PWSTR, pstinotify: *const STINOTIFY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut STI_DEVICE_INFORMATIONW) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwmessagetype: u32, pszmessage: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
pub const JS_DELETING: u32 = 2u32;
pub const JS_FAILED: u32 = 4u32;
pub const JS_INPROGRESS: u32 = 1u32;
pub const JS_NOLINE: u32 = 16u32;
pub const JS_PAUSED: u32 = 8u32;
pub const JS_PENDING: u32 = 0u32;
pub const JS_RETRIES_EXCEEDED: u32 = 64u32;
pub const JS_RETRYING: u32 = 32u32;
pub const JT_FAIL_RECEIVE: u32 = 4u32;
pub const JT_RECEIVE: u32 = 2u32;
pub const JT_ROUTING: u32 = 3u32;
pub const JT_SEND: u32 = 1u32;
pub const JT_UNKNOWN: u32 = 0u32;
pub const MAX_NOTIFICATION_DATA: u32 = 64u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXABORT = unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobid: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXACCESSCHECK = unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, accessmask: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXCLOSE = unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXCOMPLETEJOBPARAMSA = unsafe extern "system" fn(jobparams: *mut *mut FAX_JOB_PARAMA, coverpageinfo: *mut *mut FAX_COVERPAGE_INFOA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXCOMPLETEJOBPARAMSW = unsafe extern "system" fn(jobparams: *mut *mut FAX_JOB_PARAMW, coverpageinfo: *mut *mut FAX_COVERPAGE_INFOW) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXCONNECTFAXSERVERA = unsafe extern "system" fn(machinename: super::super::Foundation::PSTR, faxhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXCONNECTFAXSERVERW = unsafe extern "system" fn(machinename: super::super::Foundation::PWSTR, faxhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXDEVABORTOPERATION = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub type PFAXDEVCONFIGURE = unsafe extern "system" fn(param0: *mut super::super::UI::Controls::HPROPSHEETPAGE) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXDEVENDJOB = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXDEVINITIALIZE = unsafe extern "system" fn(param0: u32, param1: super::super::Foundation::HANDLE, param2: *mut ::windows::runtime::RawPtr, param3: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXDEVRECEIVE = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: u32, param2: *mut FAX_RECEIVE) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXDEVREPORTSTATUS = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: *mut FAX_DEV_STATUS, param2: u32, param3: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXDEVSEND = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: *mut FAX_SEND, param2: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL;
pub type PFAXDEVSHUTDOWN = unsafe extern "system" fn() -> ::windows::runtime::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXDEVSTARTJOB = unsafe extern "system" fn(param0: u32, param1: u32, param2: *mut super::super::Foundation::HANDLE, param3: super::super::Foundation::HANDLE, param4: usize) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXDEVVIRTUALDEVICECREATION = unsafe extern "system" fn(devicecount: *mut u32, devicenameprefix: super::super::Foundation::PWSTR, deviceidprefix: *mut u32, completionport: super::super::Foundation::HANDLE, completionkey: usize) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENABLEROUTINGMETHODA = unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingguid: super::super::Foundation::PSTR, enabled: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENABLEROUTINGMETHODW = unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingguid: super::super::Foundation::PWSTR, enabled: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENUMGLOBALROUTINGINFOA = unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, routinginfo: *mut *mut FAX_GLOBAL_ROUTING_INFOA, methodsreturned: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENUMGLOBALROUTINGINFOW = unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, routinginfo: *mut *mut FAX_GLOBAL_ROUTING_INFOW, methodsreturned: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENUMJOBSA = unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobentry: *mut *mut FAX_JOB_ENTRYA, jobsreturned: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENUMJOBSW = unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobentry: *mut *mut FAX_JOB_ENTRYW, jobsreturned: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENUMPORTSA = unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOA, portsreturned: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENUMPORTSW = unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOW, portsreturned: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENUMROUTINGMETHODSA = unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingmethod: *mut *mut FAX_ROUTING_METHODA, methodsreturned: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXENUMROUTINGMETHODSW = unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingmethod: *mut *mut FAX_ROUTING_METHODW, methodsreturned: *mut u32) -> super::super::Foundation::BOOL;
pub type PFAXFREEBUFFER = unsafe extern "system" fn(buffer: *mut ::std::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETCONFIGURATIONA = unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, faxconfig: *mut *mut FAX_CONFIGURATIONA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETCONFIGURATIONW = unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, faxconfig: *mut *mut FAX_CONFIGURATIONW) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETDEVICESTATUSA = unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, devicestatus: *mut *mut FAX_DEVICE_STATUSA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETDEVICESTATUSW = unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, devicestatus: *mut *mut FAX_DEVICE_STATUSW) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETJOBA = unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobid: u32, jobentry: *mut *mut FAX_JOB_ENTRYA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETJOBW = unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobid: u32, jobentry: *mut *mut FAX_JOB_ENTRYW) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETLOGGINGCATEGORIESA = unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, categories: *mut *mut FAX_LOG_CATEGORYA, numbercategories: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETLOGGINGCATEGORIESW = unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, categories: *mut *mut FAX_LOG_CATEGORYW, numbercategories: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETPAGEDATA = unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobid: u32, buffer: *mut *mut u8, buffersize: *mut u32, imagewidth: *mut u32, imageheight: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETPORTA = unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETPORTW = unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOW) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETROUTINGINFOA = unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingguid: super::super::Foundation::PSTR, routinginfobuffer: *mut *mut u8, routinginfobuffersize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXGETROUTINGINFOW = unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingguid: super::super::Foundation::PWSTR, routinginfobuffer: *mut *mut u8, routinginfobuffersize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXINITIALIZEEVENTQUEUE = unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, completionport: super::super::Foundation::HANDLE, completionkey: usize, hwnd: super::super::Foundation::HWND, messagestart: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXOPENPORT = unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, deviceid: u32, flags: u32, faxporthandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFAXPRINTCOVERPAGEA = unsafe extern "system" fn(faxcontextinfo: *const FAX_CONTEXT_INFOA, coverpageinfo: *const FAX_COVERPAGE_INFOA) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFAXPRINTCOVERPAGEW = unsafe extern "system" fn(faxcontextinfo: *const FAX_CONTEXT_INFOW, coverpageinfo: *const FAX_COVERPAGE_INFOW) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXREGISTERROUTINGEXTENSIONW = unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, extensionname: super::super::Foundation::PWSTR, friendlyname: super::super::Foundation::PWSTR, imagename: super::super::Foundation::PWSTR, callback: ::windows::runtime::RawPtr, context: *mut ::std::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXREGISTERSERVICEPROVIDERW = unsafe extern "system" fn(deviceprovider: super::super::Foundation::PWSTR, friendlyname: super::super::Foundation::PWSTR, imagename: super::super::Foundation::PWSTR, tspname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEADDFILE = unsafe extern "system" fn(jobid: u32, filename: super::super::Foundation::PWSTR, guid: *mut ::windows::runtime::GUID) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEDELETEFILE = unsafe extern "system" fn(jobid: u32, filename: super::super::Foundation::PWSTR) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEDEVICECHANGENOTIFICATION = unsafe extern "system" fn(param0: u32, param1: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEDEVICEENABLE = unsafe extern "system" fn(param0: super::super::Foundation::PWSTR, param1: u32, param2: i32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEENUMFILE = unsafe extern "system" fn(jobid: u32, guidowner: *mut ::windows::runtime::GUID, guidcaller: *mut ::windows::runtime::GUID, filename: super::super::Foundation::PWSTR, context: *mut ::std::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEENUMFILES = unsafe extern "system" fn(jobid: u32, guid: *mut ::windows::runtime::GUID, fileenumerator: ::windows::runtime::RawPtr, context: *mut ::std::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEGETFILE = unsafe extern "system" fn(jobid: u32, index: u32, filenamebuffer: super::super::Foundation::PWSTR, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEGETROUTINGINFO = unsafe extern "system" fn(param0: super::super::Foundation::PWSTR, param1: u32, param2: *mut u8, param3: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEINITIALIZE = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: *mut ::std::mem::ManuallyDrop<FAX_ROUTE_CALLBACKROUTINES>) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEMETHOD = unsafe extern "system" fn(param0: *const FAX_ROUTE, param1: *mut *mut ::std::ffi::c_void, param2: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTEMODIFYROUTINGDATA = unsafe extern "system" fn(jobid: u32, routingguid: super::super::Foundation::PWSTR, routingdata: *mut u8, routingdatasize: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXROUTESETROUTINGINFO = unsafe extern "system" fn(param0: super::super::Foundation::PWSTR, param1: u32, param2: *const u8, param3: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSENDDOCUMENTA = unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, filename: super::super::Foundation::PSTR, jobparams: *mut FAX_JOB_PARAMA, coverpageinfo: *const FAX_COVERPAGE_INFOA, faxjobid: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSENDDOCUMENTFORBROADCASTA = unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, filename: super::super::Foundation::PSTR, faxjobid: *mut u32, faxrecipientcallback: ::windows::runtime::RawPtr, context: *mut ::std::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSENDDOCUMENTFORBROADCASTW = unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, filename: super::super::Foundation::PWSTR, faxjobid: *mut u32, faxrecipientcallback: ::windows::runtime::RawPtr, context: *mut ::std::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSENDDOCUMENTW = unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, filename: super::super::Foundation::PWSTR, jobparams: *mut FAX_JOB_PARAMW, coverpageinfo: *const FAX_COVERPAGE_INFOW, faxjobid: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETCONFIGURATIONA = unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, faxconfig: *const FAX_CONFIGURATIONA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETCONFIGURATIONW = unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, faxconfig: *const FAX_CONFIGURATIONW) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETGLOBALROUTINGINFOA = unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routinginfo: *const FAX_GLOBAL_ROUTING_INFOA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETGLOBALROUTINGINFOW = unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routinginfo: *const FAX_GLOBAL_ROUTING_INFOW) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETJOBA = unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobid: u32, command: u32, jobentry: *const FAX_JOB_ENTRYA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETJOBW = unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, jobid: u32, command: u32, jobentry: *const FAX_JOB_ENTRYW) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETLOGGINGCATEGORIESA = unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, categories: *const FAX_LOG_CATEGORYA, numbercategories: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETLOGGINGCATEGORIESW = unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, categories: *const FAX_LOG_CATEGORYW, numbercategories: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETPORTA = unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, portinfo: *const FAX_PORT_INFOA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETPORTW = unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, portinfo: *const FAX_PORT_INFOW) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETROUTINGINFOA = unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingguid: super::super::Foundation::PSTR, routinginfobuffer: *const u8, routinginfobuffersize: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXSETROUTINGINFOW = unsafe extern "system" fn(faxporthandle: super::super::Foundation::HANDLE, routingguid: super::super::Foundation::PWSTR, routinginfobuffer: *const u8, routinginfobuffersize: u32) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFAXSTARTPRINTJOBA = unsafe extern "system" fn(printername: super::super::Foundation::PSTR, printinfo: *const FAX_PRINT_INFOA, faxjobid: *mut u32, faxcontextinfo: *mut FAX_CONTEXT_INFOA) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFAXSTARTPRINTJOBW = unsafe extern "system" fn(printername: super::super::Foundation::PWSTR, printinfo: *const FAX_PRINT_INFOW, faxjobid: *mut u32, faxcontextinfo: *mut FAX_CONTEXT_INFOW) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAXUNREGISTERSERVICEPROVIDERW = unsafe extern "system" fn(deviceprovider: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_EXT_CONFIG_CHANGE = unsafe extern "system" fn(param0: u32, param1: super::super::Foundation::PWSTR, param2: *mut u8, param3: u32) -> ::windows::runtime::HRESULT;
pub type PFAX_EXT_FREE_BUFFER = unsafe extern "system" fn(param0: *mut ::std::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_EXT_GET_DATA = unsafe extern "system" fn(param0: u32, param1: FAX_ENUM_DEVICE_ID_SOURCE, param2: super::super::Foundation::PWSTR, param3: *mut *mut u8, param4: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_EXT_INITIALIZE_CONFIG = unsafe extern "system" fn(param0: ::windows::runtime::RawPtr, param1: ::windows::runtime::RawPtr, param2: ::windows::runtime::RawPtr, param3: ::windows::runtime::RawPtr, param4: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_EXT_REGISTER_FOR_EVENTS = unsafe extern "system" fn(param0: super::super::Foundation::HINSTANCE, param1: u32, param2: FAX_ENUM_DEVICE_ID_SOURCE, param3: super::super::Foundation::PWSTR, param4: ::windows::runtime::RawPtr) -> super::super::Foundation::HANDLE;
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_EXT_SET_DATA = unsafe extern "system" fn(param0: super::super::Foundation::HINSTANCE, param1: u32, param2: FAX_ENUM_DEVICE_ID_SOURCE, param3: super::super::Foundation::PWSTR, param4: *mut u8, param5: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_EXT_UNREGISTER_FOR_EVENTS = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_LINECALLBACK = unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, hdevice: u32, dwmessage: u32, dwinstance: usize, dwparam1: usize, dwparam2: usize, dwparam3: usize);
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_RECIPIENT_CALLBACKA = unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, recipientnumber: u32, context: *mut ::std::ffi::c_void, jobparams: *mut FAX_JOB_PARAMA, coverpageinfo: *mut FAX_COVERPAGE_INFOA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_RECIPIENT_CALLBACKW = unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, recipientnumber: u32, context: *mut ::std::ffi::c_void, jobparams: *mut FAX_JOB_PARAMW, coverpageinfo: *mut FAX_COVERPAGE_INFOW) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_ROUTING_INSTALLATION_CALLBACKW = unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, context: *mut ::std::ffi::c_void, methodname: super::super::Foundation::PWSTR, friendlyname: super::super::Foundation::PWSTR, functionname: super::super::Foundation::PWSTR, guid: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_SEND_CALLBACK = unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, callhandle: u32, reserved1: u32, reserved2: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFAX_SERVICE_CALLBACK = unsafe extern "system" fn(faxhandle: super::super::Foundation::HANDLE, deviceid: u32, param1: usize, param2: usize, param3: usize) -> super::super::Foundation::BOOL;
pub const STIEDFL_ALLDEVICES: u32 = 0u32;
pub const STIEDFL_ATTACHEDONLY: u32 = 1u32;
pub const STIERR_ALREADY_INITIALIZED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147023649i32 as _);
pub const STIERR_BADDRIVER: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147024777i32 as _);
pub const STIERR_BETA_VERSION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147023743i32 as _);
pub const STIERR_DEVICENOTREG: i32 = -2147221164i32;
pub const STIERR_DEVICE_LOCKED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147024863i32 as _);
pub const STIERR_DEVICE_NOTREADY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147024875i32 as _);
pub const STIERR_GENERIC: i32 = -2147467259i32;
pub const STIERR_HANDLEEXISTS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147024713i32 as _);
pub const STIERR_INVALID_DEVICE_NAME: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147024773i32 as _);
pub const STIERR_INVALID_HW_TYPE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147024883i32 as _);
pub const STIERR_INVALID_PARAM: i32 = -2147024809i32;
pub const STIERR_NEEDS_LOCK: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147024738i32 as _);
pub const STIERR_NOEVENTS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147024637i32 as _);
pub const STIERR_NOINTERFACE: i32 = -2147467262i32;
pub const STIERR_NOTINITIALIZED: i32 = -2147024891i32;
pub const STIERR_NOT_INITIALIZED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147024875i32 as _);
pub const STIERR_OBJECTNOTFOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147024894i32 as _);
pub const STIERR_OLD_VERSION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147023746i32 as _);
pub const STIERR_OUTOFMEMORY: i32 = -2147024882i32;
pub const STIERR_READONLY: i32 = -2147024891i32;
pub const STIERR_SHARING_VIOLATION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147024864i32 as _);
pub const STIERR_UNSUPPORTED: i32 = -2147467263i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct STINOTIFY {
    pub dwSize: u32,
    pub guidNotificationCode: ::windows::runtime::GUID,
    pub abNotificationData: [u8; 64],
}
impl STINOTIFY {}
impl ::std::default::Default for STINOTIFY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for STINOTIFY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STINOTIFY").field("dwSize", &self.dwSize).field("guidNotificationCode", &self.guidNotificationCode).field("abNotificationData", &self.abNotificationData).finish()
    }
}
impl ::std::cmp::PartialEq for STINOTIFY {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.guidNotificationCode == other.guidNotificationCode && self.abNotificationData == other.abNotificationData
    }
}
impl ::std::cmp::Eq for STINOTIFY {}
unsafe impl ::windows::runtime::Abi for STINOTIFY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl STISUBSCRIBE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for STISUBSCRIBE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for STISUBSCRIBE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STISUBSCRIBE").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwFilter", &self.dwFilter).field("hWndNotify", &self.hWndNotify).field("hEvent", &self.hEvent).field("uiNotificationMessage", &self.uiNotificationMessage).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for STISUBSCRIBE {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.dwFilter == other.dwFilter && self.hWndNotify == other.hWndNotify && self.hEvent == other.hEvent && self.uiNotificationMessage == other.uiNotificationMessage
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for STISUBSCRIBE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for STISUBSCRIBE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const STI_CHANGENOEFFECT: i32 = 1i32;
pub const STI_DEVICE_CREATE_BOTH: u32 = 3u32;
pub const STI_DEVICE_CREATE_DATA: u32 = 2u32;
pub const STI_DEVICE_CREATE_FOR_MONITOR: u32 = 16777216u32;
pub const STI_DEVICE_CREATE_MASK: u32 = 65535u32;
pub const STI_DEVICE_CREATE_STATUS: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STI_DEVICE_INFORMATIONW {
    pub dwSize: u32,
    pub DeviceType: u32,
    pub szDeviceInternalName: [u16; 128],
    pub DeviceCapabilitiesA: STI_DEV_CAPS,
    pub dwHardwareConfiguration: u32,
    pub pszVendorDescription: super::super::Foundation::PWSTR,
    pub pszDeviceDescription: super::super::Foundation::PWSTR,
    pub pszPortName: super::super::Foundation::PWSTR,
    pub pszPropProvider: super::super::Foundation::PWSTR,
    pub pszLocalName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl STI_DEVICE_INFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for STI_DEVICE_INFORMATIONW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for STI_DEVICE_INFORMATIONW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STI_DEVICE_INFORMATIONW")
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
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for STI_DEVICE_INFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.DeviceType == other.DeviceType
            && self.szDeviceInternalName == other.szDeviceInternalName
            && self.DeviceCapabilitiesA == other.DeviceCapabilitiesA
            && self.dwHardwareConfiguration == other.dwHardwareConfiguration
            && self.pszVendorDescription == other.pszVendorDescription
            && self.pszDeviceDescription == other.pszDeviceDescription
            && self.pszPortName == other.pszPortName
            && self.pszPropProvider == other.pszPropProvider
            && self.pszLocalName == other.pszLocalName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for STI_DEVICE_INFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for STI_DEVICE_INFORMATIONW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct STI_DEVICE_MJ_TYPE(pub i32);
pub const StiDeviceTypeDefault: STI_DEVICE_MJ_TYPE = STI_DEVICE_MJ_TYPE(0i32);
pub const StiDeviceTypeScanner: STI_DEVICE_MJ_TYPE = STI_DEVICE_MJ_TYPE(1i32);
pub const StiDeviceTypeDigitalCamera: STI_DEVICE_MJ_TYPE = STI_DEVICE_MJ_TYPE(2i32);
pub const StiDeviceTypeStreamingVideo: STI_DEVICE_MJ_TYPE = STI_DEVICE_MJ_TYPE(3i32);
impl ::std::convert::From<i32> for STI_DEVICE_MJ_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for STI_DEVICE_MJ_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct STI_DEVICE_STATUS {
    pub dwSize: u32,
    pub StatusMask: u32,
    pub dwOnlineState: u32,
    pub dwHardwareStatusCode: u32,
    pub dwEventHandlingState: u32,
    pub dwPollingInterval: u32,
}
impl STI_DEVICE_STATUS {}
impl ::std::default::Default for STI_DEVICE_STATUS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for STI_DEVICE_STATUS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STI_DEVICE_STATUS")
            .field("dwSize", &self.dwSize)
            .field("StatusMask", &self.StatusMask)
            .field("dwOnlineState", &self.dwOnlineState)
            .field("dwHardwareStatusCode", &self.dwHardwareStatusCode)
            .field("dwEventHandlingState", &self.dwEventHandlingState)
            .field("dwPollingInterval", &self.dwPollingInterval)
            .finish()
    }
}
impl ::std::cmp::PartialEq for STI_DEVICE_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.StatusMask == other.StatusMask && self.dwOnlineState == other.dwOnlineState && self.dwHardwareStatusCode == other.dwHardwareStatusCode && self.dwEventHandlingState == other.dwEventHandlingState && self.dwPollingInterval == other.dwPollingInterval
    }
}
impl ::std::cmp::Eq for STI_DEVICE_STATUS {}
unsafe impl ::windows::runtime::Abi for STI_DEVICE_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const STI_DEVSTATUS_EVENTS_STATE: u32 = 2u32;
pub const STI_DEVSTATUS_ONLINE_STATE: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct STI_DEV_CAPS {
    pub dwGeneric: u32,
}
impl STI_DEV_CAPS {}
impl ::std::default::Default for STI_DEV_CAPS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for STI_DEV_CAPS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STI_DEV_CAPS").field("dwGeneric", &self.dwGeneric).finish()
    }
}
impl ::std::cmp::PartialEq for STI_DEV_CAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwGeneric == other.dwGeneric
    }
}
impl ::std::cmp::Eq for STI_DEV_CAPS {}
unsafe impl ::windows::runtime::Abi for STI_DEV_CAPS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct STI_DIAG {
    pub dwSize: u32,
    pub dwBasicDiagCode: u32,
    pub dwVendorDiagCode: u32,
    pub dwStatusMask: u32,
    pub sErrorInfo: _ERROR_INFOW,
}
impl STI_DIAG {}
impl ::std::default::Default for STI_DIAG {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for STI_DIAG {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STI_DIAG").field("dwSize", &self.dwSize).field("dwBasicDiagCode", &self.dwBasicDiagCode).field("dwVendorDiagCode", &self.dwVendorDiagCode).field("dwStatusMask", &self.dwStatusMask).field("sErrorInfo", &self.sErrorInfo).finish()
    }
}
impl ::std::cmp::PartialEq for STI_DIAG {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwBasicDiagCode == other.dwBasicDiagCode && self.dwVendorDiagCode == other.dwVendorDiagCode && self.dwStatusMask == other.dwStatusMask && self.sErrorInfo == other.sErrorInfo
    }
}
impl ::std::cmp::Eq for STI_DIAG {}
unsafe impl ::windows::runtime::Abi for STI_DIAG {
    type Abi = Self;
    type DefaultType = Self;
}
pub const STI_DIAGCODE_HWPRESENCE: u32 = 1u32;
pub const STI_ERROR_NO_ERROR: i32 = 0i32;
pub const STI_EVENTHANDLING_ENABLED: u32 = 1u32;
pub const STI_EVENTHANDLING_PENDING: u32 = 4u32;
pub const STI_EVENTHANDLING_POLLING: u32 = 2u32;
pub const STI_GENCAP_AUTO_PORTSELECT: u32 = 8u32;
pub const STI_GENCAP_GENERATE_ARRIVALEVENT: u32 = 4u32;
pub const STI_GENCAP_NOTIFICATIONS: u32 = 1u32;
pub const STI_GENCAP_POLLING_NEEDED: u32 = 2u32;
pub const STI_GENCAP_SUBSET: u32 = 32u32;
pub const STI_GENCAP_WIA: u32 = 16u32;
pub const STI_HW_CONFIG_PARALLEL: u32 = 16u32;
pub const STI_HW_CONFIG_SCSI: u32 = 2u32;
pub const STI_HW_CONFIG_SERIAL: u32 = 8u32;
pub const STI_HW_CONFIG_UNKNOWN: u32 = 1u32;
pub const STI_HW_CONFIG_USB: u32 = 4u32;
pub const STI_MAX_INTERNAL_NAME_LENGTH: u32 = 128u32;
pub const STI_NOTCONNECTED: i32 = 1i32;
pub const STI_OK: i32 = 0i32;
pub const STI_ONLINESTATE_BUSY: u32 = 256u32;
pub const STI_ONLINESTATE_ERROR: u32 = 4u32;
pub const STI_ONLINESTATE_INITIALIZING: u32 = 1024u32;
pub const STI_ONLINESTATE_IO_ACTIVE: u32 = 128u32;
pub const STI_ONLINESTATE_OFFLINE: u32 = 64u32;
pub const STI_ONLINESTATE_OPERATIONAL: u32 = 1u32;
pub const STI_ONLINESTATE_PAPER_JAM: u32 = 16u32;
pub const STI_ONLINESTATE_PAPER_PROBLEM: u32 = 32u32;
pub const STI_ONLINESTATE_PAUSED: u32 = 8u32;
pub const STI_ONLINESTATE_PENDING: u32 = 2u32;
pub const STI_ONLINESTATE_POWER_SAVE: u32 = 8192u32;
pub const STI_ONLINESTATE_TRANSFERRING: u32 = 512u32;
pub const STI_ONLINESTATE_USER_INTERVENTION: u32 = 4096u32;
pub const STI_ONLINESTATE_WARMING_UP: u32 = 2048u32;
pub const STI_RAW_RESERVED: u32 = 4096u32;
pub const STI_SUBSCRIBE_FLAG_EVENT: u32 = 2u32;
pub const STI_SUBSCRIBE_FLAG_WINDOW: u32 = 1u32;
pub const STI_TRACE_ERROR: u32 = 4u32;
pub const STI_TRACE_INFORMATION: u32 = 1u32;
pub const STI_TRACE_WARNING: u32 = 2u32;
pub const STI_UNICODE: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct STI_USD_CAPS {
    pub dwVersion: u32,
    pub dwGenericCaps: u32,
}
impl STI_USD_CAPS {}
impl ::std::default::Default for STI_USD_CAPS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for STI_USD_CAPS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STI_USD_CAPS").field("dwVersion", &self.dwVersion).field("dwGenericCaps", &self.dwGenericCaps).finish()
    }
}
impl ::std::cmp::PartialEq for STI_USD_CAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.dwGenericCaps == other.dwGenericCaps
    }
}
impl ::std::cmp::Eq for STI_USD_CAPS {}
unsafe impl ::windows::runtime::Abi for STI_USD_CAPS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const STI_USD_GENCAP_NATIVE_PUSHSUPPORT: u32 = 1u32;
pub const STI_VERSION: u32 = 2u32;
pub const STI_VERSION_FLAG_MASK: u32 = 4278190080u32;
pub const STI_VERSION_FLAG_UNICODE: u32 = 16777216u32;
pub const STI_VERSION_MIN_ALLOWED: u32 = 2u32;
pub const STI_VERSION_REAL: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STI_WIA_DEVICE_INFORMATIONW {
    pub dwSize: u32,
    pub DeviceType: u32,
    pub szDeviceInternalName: [u16; 128],
    pub DeviceCapabilitiesA: STI_DEV_CAPS,
    pub dwHardwareConfiguration: u32,
    pub pszVendorDescription: super::super::Foundation::PWSTR,
    pub pszDeviceDescription: super::super::Foundation::PWSTR,
    pub pszPortName: super::super::Foundation::PWSTR,
    pub pszPropProvider: super::super::Foundation::PWSTR,
    pub pszLocalName: super::super::Foundation::PWSTR,
    pub pszUiDll: super::super::Foundation::PWSTR,
    pub pszServer: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl STI_WIA_DEVICE_INFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for STI_WIA_DEVICE_INFORMATIONW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for STI_WIA_DEVICE_INFORMATIONW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STI_WIA_DEVICE_INFORMATIONW")
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
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for STI_WIA_DEVICE_INFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.DeviceType == other.DeviceType
            && self.szDeviceInternalName == other.szDeviceInternalName
            && self.DeviceCapabilitiesA == other.DeviceCapabilitiesA
            && self.dwHardwareConfiguration == other.dwHardwareConfiguration
            && self.pszVendorDescription == other.pszVendorDescription
            && self.pszDeviceDescription == other.pszDeviceDescription
            && self.pszPortName == other.pszPortName
            && self.pszPropProvider == other.pszPropProvider
            && self.pszLocalName == other.pszLocalName
            && self.pszUiDll == other.pszUiDll
            && self.pszServer == other.pszServer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for STI_WIA_DEVICE_INFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for STI_WIA_DEVICE_INFORMATIONW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SUPPORTS_MSCPLUS_VAL: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SendToFaxRecipient<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(sndmode: SendToMode, lpfilename: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SendToFaxRecipient(sndmode: SendToMode, lpfilename: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(SendToFaxRecipient(::std::mem::transmute(sndmode), lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SendToMode(pub i32);
pub const SEND_TO_FAX_RECIPIENT_ATTACHMENT: SendToMode = SendToMode(0i32);
impl ::std::convert::From<i32> for SendToMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SendToMode {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StiCreateInstanceW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(hinst: Param0, dwver: u32, ppsti: *mut ::std::option::Option<IStillImageW>, punkouter: Param3) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StiCreateInstanceW(hinst: super::super::Foundation::HINSTANCE, dwver: u32, ppsti: *mut ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        StiCreateInstanceW(hinst.into_param().abi(), ::std::mem::transmute(dwver), ::std::mem::transmute(ppsti), punkouter.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const WIA_INCOMPAT_XP: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct _ERROR_INFOW {
    pub dwSize: u32,
    pub dwGenericError: u32,
    pub dwVendorError: u32,
    pub szExtendedErrorText: [u16; 255],
}
impl _ERROR_INFOW {}
impl ::std::default::Default for _ERROR_INFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for _ERROR_INFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_ERROR_INFOW").field("dwSize", &self.dwSize).field("dwGenericError", &self.dwGenericError).field("dwVendorError", &self.dwVendorError).field("szExtendedErrorText", &self.szExtendedErrorText).finish()
    }
}
impl ::std::cmp::PartialEq for _ERROR_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwGenericError == other.dwGenericError && self.dwVendorError == other.dwVendorError && self.szExtendedErrorText == other.szExtendedErrorText
    }
}
impl ::std::cmp::Eq for _ERROR_INFOW {}
unsafe impl ::windows::runtime::Abi for _ERROR_INFOW {
    type Abi = Self;
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct _IFaxAccountNotify(::windows::runtime::IUnknown);
impl _IFaxAccountNotify {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnIncomingJobAdded<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxAccount>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pfaxaccount: Param0, bstrjobid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pfaxaccount.into_param().abi(), bstrjobid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnIncomingJobRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxAccount>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pfaxaccount: Param0, bstrjobid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), pfaxaccount.into_param().abi(), bstrjobid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnIncomingJobChanged<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxAccount>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, IFaxJobStatus>>(&self, pfaxaccount: Param0, bstrjobid: Param1, pjobstatus: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pfaxaccount.into_param().abi(), bstrjobid.into_param().abi(), pjobstatus.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnOutgoingJobAdded<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxAccount>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pfaxaccount: Param0, bstrjobid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), pfaxaccount.into_param().abi(), bstrjobid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnOutgoingJobRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxAccount>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pfaxaccount: Param0, bstrjobid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), pfaxaccount.into_param().abi(), bstrjobid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnOutgoingJobChanged<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxAccount>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, IFaxJobStatus>>(&self, pfaxaccount: Param0, bstrjobid: Param1, pjobstatus: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), pfaxaccount.into_param().abi(), bstrjobid.into_param().abi(), pjobstatus.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnIncomingMessageAdded<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxAccount>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pfaxaccount: Param0, bstrmessageid: Param1, faddedtoreceivefolder: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), pfaxaccount.into_param().abi(), bstrmessageid.into_param().abi(), ::std::mem::transmute(faddedtoreceivefolder)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnIncomingMessageRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxAccount>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pfaxaccount: Param0, bstrmessageid: Param1, fremovedfromreceivefolder: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), pfaxaccount.into_param().abi(), bstrmessageid.into_param().abi(), ::std::mem::transmute(fremovedfromreceivefolder)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnOutgoingMessageAdded<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxAccount>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pfaxaccount: Param0, bstrmessageid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), pfaxaccount.into_param().abi(), bstrmessageid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnOutgoingMessageRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxAccount>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pfaxaccount: Param0, bstrmessageid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), pfaxaccount.into_param().abi(), bstrmessageid.into_param().abi()).ok()
    }
    pub unsafe fn OnServerShutDown<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxServer2>>(&self, pfaxserver: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), pfaxserver.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for _IFaxAccountNotify {
    type Vtable = _IFaxAccountNotify_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3115564161, 44059, 18163, [179, 157, 10, 220, 48, 225, 183, 136]);
}
impl ::std::convert::From<_IFaxAccountNotify> for ::windows::runtime::IUnknown {
    fn from(value: _IFaxAccountNotify) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&_IFaxAccountNotify> for ::windows::runtime::IUnknown {
    fn from(value: &_IFaxAccountNotify) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for _IFaxAccountNotify {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &_IFaxAccountNotify {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<_IFaxAccountNotify> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: _IFaxAccountNotify) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&_IFaxAccountNotify> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &_IFaxAccountNotify) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for _IFaxAccountNotify {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &_IFaxAccountNotify {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct _IFaxAccountNotify_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxaccount: ::windows::runtime::RawPtr, bstrjobid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxaccount: ::windows::runtime::RawPtr, bstrjobid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxaccount: ::windows::runtime::RawPtr, bstrjobid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pjobstatus: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxaccount: ::windows::runtime::RawPtr, bstrjobid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxaccount: ::windows::runtime::RawPtr, bstrjobid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxaccount: ::windows::runtime::RawPtr, bstrjobid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pjobstatus: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxaccount: ::windows::runtime::RawPtr, bstrmessageid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, faddedtoreceivefolder: i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxaccount: ::windows::runtime::RawPtr, bstrmessageid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, fremovedfromreceivefolder: i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxaccount: ::windows::runtime::RawPtr, bstrmessageid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxaccount: ::windows::runtime::RawPtr, bstrmessageid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxserver: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct _IFaxServerNotify2(::windows::runtime::IUnknown);
impl _IFaxServerNotify2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnIncomingJobAdded<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxServer2>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pfaxserver: Param0, bstrjobid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pfaxserver.into_param().abi(), bstrjobid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnIncomingJobRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxServer2>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pfaxserver: Param0, bstrjobid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), pfaxserver.into_param().abi(), bstrjobid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnIncomingJobChanged<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxServer2>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, IFaxJobStatus>>(&self, pfaxserver: Param0, bstrjobid: Param1, pjobstatus: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pfaxserver.into_param().abi(), bstrjobid.into_param().abi(), pjobstatus.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnOutgoingJobAdded<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxServer2>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pfaxserver: Param0, bstrjobid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), pfaxserver.into_param().abi(), bstrjobid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnOutgoingJobRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxServer2>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pfaxserver: Param0, bstrjobid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), pfaxserver.into_param().abi(), bstrjobid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnOutgoingJobChanged<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxServer2>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, IFaxJobStatus>>(&self, pfaxserver: Param0, bstrjobid: Param1, pjobstatus: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), pfaxserver.into_param().abi(), bstrjobid.into_param().abi(), pjobstatus.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnIncomingMessageAdded<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxServer2>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pfaxserver: Param0, bstrmessageid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), pfaxserver.into_param().abi(), bstrmessageid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnIncomingMessageRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxServer2>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pfaxserver: Param0, bstrmessageid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), pfaxserver.into_param().abi(), bstrmessageid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnOutgoingMessageAdded<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxServer2>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pfaxserver: Param0, bstrmessageid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), pfaxserver.into_param().abi(), bstrmessageid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnOutgoingMessageRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxServer2>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pfaxserver: Param0, bstrmessageid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), pfaxserver.into_param().abi(), bstrmessageid.into_param().abi()).ok()
    }
    pub unsafe fn OnReceiptOptionsChange<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxServer2>>(&self, pfaxserver: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), pfaxserver.into_param().abi()).ok()
    }
    pub unsafe fn OnActivityLoggingConfigChange<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxServer2>>(&self, pfaxserver: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), pfaxserver.into_param().abi()).ok()
    }
    pub unsafe fn OnSecurityConfigChange<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxServer2>>(&self, pfaxserver: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), pfaxserver.into_param().abi()).ok()
    }
    pub unsafe fn OnEventLoggingConfigChange<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxServer2>>(&self, pfaxserver: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), pfaxserver.into_param().abi()).ok()
    }
    pub unsafe fn OnOutgoingQueueConfigChange<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxServer2>>(&self, pfaxserver: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), pfaxserver.into_param().abi()).ok()
    }
    pub unsafe fn OnOutgoingArchiveConfigChange<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxServer2>>(&self, pfaxserver: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), pfaxserver.into_param().abi()).ok()
    }
    pub unsafe fn OnIncomingArchiveConfigChange<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxServer2>>(&self, pfaxserver: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), pfaxserver.into_param().abi()).ok()
    }
    pub unsafe fn OnDevicesConfigChange<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxServer2>>(&self, pfaxserver: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), pfaxserver.into_param().abi()).ok()
    }
    pub unsafe fn OnOutboundRoutingGroupsConfigChange<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxServer2>>(&self, pfaxserver: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), pfaxserver.into_param().abi()).ok()
    }
    pub unsafe fn OnOutboundRoutingRulesConfigChange<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxServer2>>(&self, pfaxserver: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), pfaxserver.into_param().abi()).ok()
    }
    pub unsafe fn OnServerActivityChange<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxServer2>>(&self, pfaxserver: Param0, lincomingmessages: i32, lroutingmessages: i32, loutgoingmessages: i32, lqueuedmessages: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), pfaxserver.into_param().abi(), ::std::mem::transmute(lincomingmessages), ::std::mem::transmute(lroutingmessages), ::std::mem::transmute(loutgoingmessages), ::std::mem::transmute(lqueuedmessages)).ok()
    }
    pub unsafe fn OnQueuesStatusChange<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxServer2>>(&self, pfaxserver: Param0, boutgoingqueueblocked: i16, boutgoingqueuepaused: i16, bincomingqueueblocked: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), pfaxserver.into_param().abi(), ::std::mem::transmute(boutgoingqueueblocked), ::std::mem::transmute(boutgoingqueuepaused), ::std::mem::transmute(bincomingqueueblocked)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnNewCall<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxServer2>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pfaxserver: Param0, lcallid: i32, ldeviceid: i32, bstrcallerid: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), pfaxserver.into_param().abi(), ::std::mem::transmute(lcallid), ::std::mem::transmute(ldeviceid), bstrcallerid.into_param().abi()).ok()
    }
    pub unsafe fn OnServerShutDown<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxServer2>>(&self, pfaxserver: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), pfaxserver.into_param().abi()).ok()
    }
    pub unsafe fn OnDeviceStatusChange<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxServer2>>(&self, pfaxserver: Param0, ldeviceid: i32, bpoweredoff: i16, bsending: i16, breceiving: i16, bringing: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), pfaxserver.into_param().abi(), ::std::mem::transmute(ldeviceid), ::std::mem::transmute(bpoweredoff), ::std::mem::transmute(bsending), ::std::mem::transmute(breceiving), ::std::mem::transmute(bringing)).ok()
    }
    pub unsafe fn OnGeneralServerConfigChanged<'a, Param0: ::windows::runtime::IntoParam<'a, IFaxServer2>>(&self, pfaxserver: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), pfaxserver.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for _IFaxServerNotify2 {
    type Vtable = _IFaxServerNotify2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3969673657, 24551, 18437, [148, 103, 130, 252, 217, 106, 249, 3]);
}
impl ::std::convert::From<_IFaxServerNotify2> for ::windows::runtime::IUnknown {
    fn from(value: _IFaxServerNotify2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&_IFaxServerNotify2> for ::windows::runtime::IUnknown {
    fn from(value: &_IFaxServerNotify2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for _IFaxServerNotify2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &_IFaxServerNotify2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<_IFaxServerNotify2> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: _IFaxServerNotify2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&_IFaxServerNotify2> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &_IFaxServerNotify2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for _IFaxServerNotify2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &_IFaxServerNotify2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct _IFaxServerNotify2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxserver: ::windows::runtime::RawPtr, bstrjobid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxserver: ::windows::runtime::RawPtr, bstrjobid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxserver: ::windows::runtime::RawPtr, bstrjobid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pjobstatus: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxserver: ::windows::runtime::RawPtr, bstrjobid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxserver: ::windows::runtime::RawPtr, bstrjobid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxserver: ::windows::runtime::RawPtr, bstrjobid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pjobstatus: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxserver: ::windows::runtime::RawPtr, bstrmessageid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxserver: ::windows::runtime::RawPtr, bstrmessageid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxserver: ::windows::runtime::RawPtr, bstrmessageid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxserver: ::windows::runtime::RawPtr, bstrmessageid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxserver: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxserver: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxserver: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxserver: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxserver: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxserver: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxserver: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxserver: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxserver: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxserver: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxserver: ::windows::runtime::RawPtr, lincomingmessages: i32, lroutingmessages: i32, loutgoingmessages: i32, lqueuedmessages: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxserver: ::windows::runtime::RawPtr, boutgoingqueueblocked: i16, boutgoingqueuepaused: i16, bincomingqueueblocked: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxserver: ::windows::runtime::RawPtr, lcallid: i32, ldeviceid: i32, bstrcallerid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxserver: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxserver: ::windows::runtime::RawPtr, ldeviceid: i32, bpoweredoff: i16, bsending: i16, breceiving: i16, bringing: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfaxserver: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
pub const lDEFAULT_PREFETCH_SIZE: i32 = 100i32;
pub const wcharREASSIGN_RECIPIENTS_DELIMITER: u16 = 59u16;
