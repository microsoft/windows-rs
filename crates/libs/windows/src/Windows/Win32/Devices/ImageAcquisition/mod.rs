#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const ADVANCED_DUP: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const ADVANCED_DUPLEX: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const ALL_PAGES: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const AUTO_ADVANCE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const AUTO_SOURCE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const BACK_FIRST: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const BACK_ONLY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const BARCODE_READER: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const BARCODE_READER_READY: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const BASE_VAL_WIA_ERROR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const BASE_VAL_WIA_SUCCESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const BOTTOM_JUSTIFIED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const BUS_TYPE_FIREWIRE: u32 = 203u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const BUS_TYPE_PARALLEL: u32 = 202u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const BUS_TYPE_SCSI: u32 = 200u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const BUS_TYPE_USB: u32 = 201u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CAPTUREMODE_BURST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CAPTUREMODE_NORMAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CAPTUREMODE_TIMELAPSE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CENTERED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CFSTR_WIAITEMNAMES: &'static str = "WIAItemNames";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CFSTR_WIAITEMPTR: &'static str = "WIAItemPointer";
pub const CLSID_WiaDefaultSegFilter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4f4d30b_0b29_4508_8922_0c5797d42765);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_GETADFAVAILABLE: u32 = 117u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_GETADFHASPAPER: u32 = 120u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_GETADFOPEN: u32 = 118u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_GETADFREADY: u32 = 119u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_GETADFSTATUS: u32 = 121u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_GETADFUNLOADREADY: u32 = 122u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_GETCAPABILITIES: u32 = 132u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_GETSUPPORTEDFILEFORMATS: u32 = 138u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_GETSUPPORTEDMEMORYFORMATS: u32 = 139u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_GETTPAAVAILABLE: u32 = 123u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_GETTPAOPENED: u32 = 124u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_GET_INTERRUPT_EVENT: u32 = 133u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_INITIALIZE: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_LOAD_ADF: u32 = 115u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_RESETSCANNER: u32 = 131u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SENDSCSICOMMAND: u32 = 127u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SETCOLORDITHER: u32 = 111u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SETCONTRAST: u32 = 104u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SETDATATYPE: u32 = 106u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SETDITHER: u32 = 107u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SETFILTER: u32 = 114u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SETFORMAT: u32 = 140u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SETGSDNAME: u32 = 134u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SETINTENSITY: u32 = 105u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SETLAMP: u32 = 126u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SETMATRIX: u32 = 112u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SETMIRROR: u32 = 108u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SETNEGATIVE: u32 = 109u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SETSCANMODE: u32 = 135u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SETSPEED: u32 = 113u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SETSTIDEVICEHKEY: u32 = 136u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SETTONEMAP: u32 = 110u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SETXRESOLUTION: u32 = 102u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_SETYRESOLUTION: u32 = 103u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_STI_DEVICERESET: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_STI_DIAGNOSTIC: u32 = 130u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_STI_GETSTATUS: u32 = 129u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_TPAREADY: u32 = 125u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_UNINITIALIZE: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const CMD_UNLOAD_ADF: u32 = 116u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const COPY_PARENT_PROPERTY_VALUES: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const DETECT_DUP: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const DETECT_DUP_AVAIL: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const DETECT_FEED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const DETECT_FEED_AVAIL: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const DETECT_FILM_TPA: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const DETECT_FLAT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const DETECT_SCAN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const DETECT_STOR: u32 = 4096u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DEVICEDIALOGDATA {
    pub cbSize: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub pIWiaItemRoot: ::core::option::Option<IWiaItem>,
    pub dwFlags: u32,
    pub lIntent: i32,
    pub lItemCount: i32,
    pub ppWiaItems: *mut ::core::option::Option<IWiaItem>,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DEVICEDIALOGDATA {
    fn clone(&self) -> Self {
        Self {
            cbSize: self.cbSize,
            hwndParent: self.hwndParent,
            pIWiaItemRoot: self.pIWiaItemRoot.clone(),
            dwFlags: self.dwFlags,
            lIntent: self.lIntent,
            lItemCount: self.lItemCount,
            ppWiaItems: self.ppWiaItems,
        }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DEVICEDIALOGDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICEDIALOGDATA").field("cbSize", &self.cbSize).field("hwndParent", &self.hwndParent).field("pIWiaItemRoot", &self.pIWiaItemRoot).field("dwFlags", &self.dwFlags).field("lIntent", &self.lIntent).field("lItemCount", &self.lItemCount).field("ppWiaItems", &self.ppWiaItems).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DEVICEDIALOGDATA {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEVICEDIALOGDATA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.hwndParent == other.hwndParent && self.pIWiaItemRoot == other.pIWiaItemRoot && self.dwFlags == other.dwFlags && self.lIntent == other.lIntent && self.lItemCount == other.lItemCount && self.ppWiaItems == other.ppWiaItems
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEVICEDIALOGDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEVICEDIALOGDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DEVICEDIALOGDATA2 {
    pub cbSize: u32,
    pub pIWiaItemRoot: ::core::option::Option<IWiaItem2>,
    pub dwFlags: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub bstrFolderName: super::super::Foundation::BSTR,
    pub bstrFilename: super::super::Foundation::BSTR,
    pub lNumFiles: i32,
    pub pbstrFilePaths: *mut super::super::Foundation::BSTR,
    pub pWiaItem: ::core::option::Option<IWiaItem2>,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DEVICEDIALOGDATA2 {
    fn clone(&self) -> Self {
        Self {
            cbSize: self.cbSize,
            pIWiaItemRoot: self.pIWiaItemRoot.clone(),
            dwFlags: self.dwFlags,
            hwndParent: self.hwndParent,
            bstrFolderName: self.bstrFolderName.clone(),
            bstrFilename: self.bstrFilename.clone(),
            lNumFiles: self.lNumFiles,
            pbstrFilePaths: self.pbstrFilePaths,
            pWiaItem: self.pWiaItem.clone(),
        }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DEVICEDIALOGDATA2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICEDIALOGDATA2").field("cbSize", &self.cbSize).field("pIWiaItemRoot", &self.pIWiaItemRoot).field("dwFlags", &self.dwFlags).field("hwndParent", &self.hwndParent).field("bstrFolderName", &self.bstrFolderName).field("bstrFilename", &self.bstrFilename).field("lNumFiles", &self.lNumFiles).field("pbstrFilePaths", &self.pbstrFilePaths).field("pWiaItem", &self.pWiaItem).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DEVICEDIALOGDATA2 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEVICEDIALOGDATA2 {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pIWiaItemRoot == other.pIWiaItemRoot && self.dwFlags == other.dwFlags && self.hwndParent == other.hwndParent && self.bstrFolderName == other.bstrFolderName && self.bstrFilename == other.bstrFilename && self.lNumFiles == other.lNumFiles && self.pbstrFilePaths == other.pbstrFilePaths && self.pWiaItem == other.pWiaItem
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEVICEDIALOGDATA2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEVICEDIALOGDATA2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const DEVICE_ATTENTION: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const DUP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const DUPLEX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const DUP_READY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DeviceDialogFunction = ::core::option::Option<unsafe extern "system" fn(param0: *mut DEVICEDIALOGDATA) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const EFFECTMODE_BW: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const EFFECTMODE_SEPIA: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const EFFECTMODE_STANDARD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const ENDORSER: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const ENDORSER_READY: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const ESC_TWAIN_CAPABILITY: u32 = 2001u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const ESC_TWAIN_PRIVATE_SUPPORTED_CAPS: u32 = 2002u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const EXPOSUREMETERING_AVERAGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const EXPOSUREMETERING_CENTERSPOT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const EXPOSUREMETERING_CENTERWEIGHT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const EXPOSUREMETERING_MULTISPOT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const EXPOSUREMODE_APERTURE_PRIORITY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const EXPOSUREMODE_AUTO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const EXPOSUREMODE_MANUAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const EXPOSUREMODE_PORTRAIT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const EXPOSUREMODE_PROGRAM_ACTION: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const EXPOSUREMODE_PROGRAM_CREATIVE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const EXPOSUREMODE_SHUTTER_PRIORITY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FEED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FEEDER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FEED_READY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FILM_TPA: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FILM_TPA_READY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FLASHMODE_AUTO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FLASHMODE_EXTERNALSYNC: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FLASHMODE_FILL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FLASHMODE_OFF: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FLASHMODE_REDEYE_AUTO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FLASHMODE_REDEYE_FILL: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FLAT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FLATBED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FLAT_COVER_UP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FLAT_READY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FOCUSMETERING_CENTERSPOT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FOCUSMETERING_MULTISPOT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FOCUSMODE_AUTO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FOCUSMODE_MACROAUTO: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FOCUSMODE_MANUAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FRONT_FIRST: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const FRONT_ONLY: u32 = 32u32;
pub const GUID_DEVINTERFACE_IMAGE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6bdd1fc6_810f_11d0_bec7_08002be2092f);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
#[repr(transparent)]
pub struct IEnumWIA_DEV_CAPS(::windows::core::IUnknown);
impl IEnumWIA_DEV_CAPS {
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut WIA_DEV_CAP, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumWIA_DEV_CAPS> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumWIA_DEV_CAPS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IEnumWIA_DEV_CAPS> for ::windows::core::IUnknown {
    fn from(value: IEnumWIA_DEV_CAPS) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumWIA_DEV_CAPS> for ::windows::core::IUnknown {
    fn from(value: &IEnumWIA_DEV_CAPS) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumWIA_DEV_CAPS {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumWIA_DEV_CAPS {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumWIA_DEV_CAPS {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumWIA_DEV_CAPS {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumWIA_DEV_CAPS {}
impl ::core::fmt::Debug for IEnumWIA_DEV_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumWIA_DEV_CAPS").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumWIA_DEV_CAPS {
    type Vtable = IEnumWIA_DEV_CAPS_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1fcc4287_aca6_11d2_a093_00c04f72dc3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWIA_DEV_CAPS_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut WIA_DEV_CAP, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelt: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
#[repr(transparent)]
pub struct IEnumWIA_DEV_INFO(::windows::core::IUnknown);
impl IEnumWIA_DEV_INFO {
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IWiaPropertyStorage>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumWIA_DEV_INFO> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumWIA_DEV_INFO>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IEnumWIA_DEV_INFO> for ::windows::core::IUnknown {
    fn from(value: IEnumWIA_DEV_INFO) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumWIA_DEV_INFO> for ::windows::core::IUnknown {
    fn from(value: &IEnumWIA_DEV_INFO) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumWIA_DEV_INFO {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumWIA_DEV_INFO {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumWIA_DEV_INFO {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumWIA_DEV_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumWIA_DEV_INFO {}
impl ::core::fmt::Debug for IEnumWIA_DEV_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumWIA_DEV_INFO").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumWIA_DEV_INFO {
    type Vtable = IEnumWIA_DEV_INFO_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e38b83c_8cf1_11d1_bf92_0060081ed811);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWIA_DEV_INFO_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
#[repr(transparent)]
pub struct IEnumWIA_FORMAT_INFO(::windows::core::IUnknown);
impl IEnumWIA_FORMAT_INFO {
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut WIA_FORMAT_INFO, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumWIA_FORMAT_INFO> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumWIA_FORMAT_INFO>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IEnumWIA_FORMAT_INFO> for ::windows::core::IUnknown {
    fn from(value: IEnumWIA_FORMAT_INFO) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumWIA_FORMAT_INFO> for ::windows::core::IUnknown {
    fn from(value: &IEnumWIA_FORMAT_INFO) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumWIA_FORMAT_INFO {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumWIA_FORMAT_INFO {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumWIA_FORMAT_INFO {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumWIA_FORMAT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumWIA_FORMAT_INFO {}
impl ::core::fmt::Debug for IEnumWIA_FORMAT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumWIA_FORMAT_INFO").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumWIA_FORMAT_INFO {
    type Vtable = IEnumWIA_FORMAT_INFO_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81befc5b_656d_44f1_b24c_d41d51b4dc81);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWIA_FORMAT_INFO_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut WIA_FORMAT_INFO, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelt: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
#[repr(transparent)]
pub struct IEnumWiaItem(::windows::core::IUnknown);
impl IEnumWiaItem {
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn Next(&self, celt: u32, ppiwiaitem: *mut ::core::option::Option<IWiaItem>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(ppiwiaitem), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumWiaItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumWiaItem>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IEnumWiaItem> for ::windows::core::IUnknown {
    fn from(value: IEnumWiaItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumWiaItem> for ::windows::core::IUnknown {
    fn from(value: &IEnumWiaItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumWiaItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumWiaItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumWiaItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumWiaItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumWiaItem {}
impl ::core::fmt::Debug for IEnumWiaItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumWiaItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumWiaItem {
    type Vtable = IEnumWiaItem_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e8383fc_3391_11d2_9a33_00c04fa36145);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWiaItem_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppiwiaitem: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
#[repr(transparent)]
pub struct IEnumWiaItem2(::windows::core::IUnknown);
impl IEnumWiaItem2 {
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn Next(&self, celt: u32, ppiwiaitem2: *mut ::core::option::Option<IWiaItem2>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(ppiwiaitem2), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumWiaItem2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumWiaItem2>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IEnumWiaItem2> for ::windows::core::IUnknown {
    fn from(value: IEnumWiaItem2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumWiaItem2> for ::windows::core::IUnknown {
    fn from(value: &IEnumWiaItem2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumWiaItem2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumWiaItem2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumWiaItem2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumWiaItem2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumWiaItem2 {}
impl ::core::fmt::Debug for IEnumWiaItem2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumWiaItem2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumWiaItem2 {
    type Vtable = IEnumWiaItem2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59970af4_cd0d_44d9_ab24_52295630e582);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWiaItem2_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppiwiaitem2: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const IMPRINTER: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const IMPRINTER_READY: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const IT_MSG_DATA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const IT_MSG_DATA_HEADER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const IT_MSG_FILE_PREVIEW_DATA: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const IT_MSG_FILE_PREVIEW_DATA_HEADER: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const IT_MSG_NEW_PAGE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const IT_MSG_STATUS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const IT_MSG_TERMINATION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const IT_STATUS_MASK: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const IT_STATUS_PROCESSING_DATA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const IT_STATUS_TRANSFER_FROM_DEVICE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const IT_STATUS_TRANSFER_TO_CLIENT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
#[repr(transparent)]
pub struct IWiaAppErrorHandler(::windows::core::IUnknown);
impl IWiaAppErrorHandler {
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__: super::super::Foundation::HWND = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetWindow)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn ReportStatus<'a, Param1: ::windows::core::IntoParam<'a, IWiaItem2>>(&self, lflags: i32, pwiaitem2: Param1, hrstatus: ::windows::core::HRESULT, lpercentcomplete: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReportStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), pwiaitem2.into_param().abi(), ::core::mem::transmute(hrstatus), ::core::mem::transmute(lpercentcomplete)).ok()
    }
}
impl ::core::convert::From<IWiaAppErrorHandler> for ::windows::core::IUnknown {
    fn from(value: IWiaAppErrorHandler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaAppErrorHandler> for ::windows::core::IUnknown {
    fn from(value: &IWiaAppErrorHandler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWiaAppErrorHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWiaAppErrorHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaAppErrorHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaAppErrorHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaAppErrorHandler {}
impl ::core::fmt::Debug for IWiaAppErrorHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaAppErrorHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWiaAppErrorHandler {
    type Vtable = IWiaAppErrorHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c16186c_d0a6_400c_80f4_d26986a0e734);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaAppErrorHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetWindow: usize,
    pub ReportStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pwiaitem2: ::windows::core::RawPtr, hrstatus: ::windows::core::HRESULT, lpercentcomplete: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
#[repr(transparent)]
pub struct IWiaDataCallback(::windows::core::IUnknown);
impl IWiaDataCallback {
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn BandedDataCallback(&self, lmessage: i32, lstatus: i32, lpercentcomplete: i32, loffset: i32, llength: i32, lreserved: i32, lreslength: i32, pbbuffer: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BandedDataCallback)(::core::mem::transmute_copy(self), ::core::mem::transmute(lmessage), ::core::mem::transmute(lstatus), ::core::mem::transmute(lpercentcomplete), ::core::mem::transmute(loffset), ::core::mem::transmute(llength), ::core::mem::transmute(lreserved), ::core::mem::transmute(lreslength), ::core::mem::transmute(pbbuffer)).ok()
    }
}
impl ::core::convert::From<IWiaDataCallback> for ::windows::core::IUnknown {
    fn from(value: IWiaDataCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaDataCallback> for ::windows::core::IUnknown {
    fn from(value: &IWiaDataCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWiaDataCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWiaDataCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaDataCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaDataCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaDataCallback {}
impl ::core::fmt::Debug for IWiaDataCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaDataCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWiaDataCallback {
    type Vtable = IWiaDataCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa558a866_a5b0_11d2_a08f_00c04f72dc3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaDataCallback_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub BandedDataCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmessage: i32, lstatus: i32, lpercentcomplete: i32, loffset: i32, llength: i32, lreserved: i32, lreslength: i32, pbbuffer: *mut u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
#[repr(transparent)]
pub struct IWiaDataTransfer(::windows::core::IUnknown);
impl IWiaDataTransfer {
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn idtGetData<'a, Param1: ::windows::core::IntoParam<'a, IWiaDataCallback>>(&self, pmedium: *mut super::super::System::Com::STGMEDIUM, piwiadatacallback: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).idtGetData)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmedium), piwiadatacallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn idtGetBandedData<'a, Param1: ::windows::core::IntoParam<'a, IWiaDataCallback>>(&self, pwiadatatransinfo: *mut WIA_DATA_TRANSFER_INFO, piwiadatacallback: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).idtGetBandedData)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwiadatatransinfo), piwiadatacallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn idtQueryGetData(&self, pfe: *const WIA_FORMAT_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).idtQueryGetData)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfe)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn idtEnumWIA_FORMAT_INFO(&self) -> ::windows::core::Result<IEnumWIA_FORMAT_INFO> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).idtEnumWIA_FORMAT_INFO)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumWIA_FORMAT_INFO>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn idtGetExtendedTransferInfo(&self) -> ::windows::core::Result<WIA_EXTENDED_TRANSFER_INFO> {
        let mut result__: WIA_EXTENDED_TRANSFER_INFO = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).idtGetExtendedTransferInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WIA_EXTENDED_TRANSFER_INFO>(result__)
    }
}
impl ::core::convert::From<IWiaDataTransfer> for ::windows::core::IUnknown {
    fn from(value: IWiaDataTransfer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaDataTransfer> for ::windows::core::IUnknown {
    fn from(value: &IWiaDataTransfer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWiaDataTransfer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWiaDataTransfer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaDataTransfer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaDataTransfer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaDataTransfer {}
impl ::core::fmt::Debug for IWiaDataTransfer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaDataTransfer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWiaDataTransfer {
    type Vtable = IWiaDataTransfer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6cef998_a5b0_11d2_a08f_00c04f72dc3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaDataTransfer_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub idtGetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmedium: *mut super::super::System::Com::STGMEDIUM, piwiadatacallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    idtGetData: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub idtGetBandedData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwiadatatransinfo: *mut WIA_DATA_TRANSFER_INFO, piwiadatacallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    idtGetBandedData: usize,
    pub idtQueryGetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfe: *const WIA_FORMAT_INFO) -> ::windows::core::HRESULT,
    pub idtEnumWIA_FORMAT_INFO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub idtGetExtendedTransferInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pextendedtransferinfo: *mut WIA_EXTENDED_TRANSFER_INFO) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
#[repr(transparent)]
pub struct IWiaDevMgr(::windows::core::IUnknown);
impl IWiaDevMgr {
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn EnumDeviceInfo(&self, lflag: i32) -> ::windows::core::Result<IEnumWIA_DEV_INFO> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumDeviceInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflag), ::core::mem::transmute(&mut result__)).from_abi::<IEnumWIA_DEV_INFO>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateDevice<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdeviceid: Param0) -> ::windows::core::Result<IWiaItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateDevice)(::core::mem::transmute_copy(self), bstrdeviceid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWiaItem>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SelectDeviceDlg<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndparent: Param0, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut super::super::Foundation::BSTR, ppitemroot: *mut ::core::option::Option<IWiaItem>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SelectDeviceDlg)(::core::mem::transmute_copy(self), hwndparent.into_param().abi(), ::core::mem::transmute(ldevicetype), ::core::mem::transmute(lflags), ::core::mem::transmute(pbstrdeviceid), ::core::mem::transmute(ppitemroot)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SelectDeviceDlgID<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndparent: Param0, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SelectDeviceDlgID)(::core::mem::transmute_copy(self), hwndparent.into_param().abi(), ::core::mem::transmute(ldevicetype), ::core::mem::transmute(lflags), ::core::mem::transmute(pbstrdeviceid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetImageDlg<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param4: ::windows::core::IntoParam<'a, IWiaItem>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, hwndparent: Param0, ldevicetype: i32, lflags: i32, lintent: i32, pitemroot: Param4, bstrfilename: Param5, pguidformat: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetImageDlg)(::core::mem::transmute_copy(self), hwndparent.into_param().abi(), ::core::mem::transmute(ldevicetype), ::core::mem::transmute(lflags), ::core::mem::transmute(lintent), pitemroot.into_param().abi(), bstrfilename.into_param().abi(), ::core::mem::transmute(pguidformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterEventCallbackProgram<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, lflags: i32, bstrdeviceid: Param1, peventguid: *const ::windows::core::GUID, bstrcommandline: Param3, bstrname: Param4, bstrdescription: Param5, bstricon: Param6) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterEventCallbackProgram)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), bstrdeviceid.into_param().abi(), ::core::mem::transmute(peventguid), bstrcommandline.into_param().abi(), bstrname.into_param().abi(), bstrdescription.into_param().abi(), bstricon.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterEventCallbackInterface<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, IWiaEventCallback>>(&self, lflags: i32, bstrdeviceid: Param1, peventguid: *const ::windows::core::GUID, piwiaeventcallback: Param3) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RegisterEventCallbackInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), bstrdeviceid.into_param().abi(), ::core::mem::transmute(peventguid), piwiaeventcallback.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterEventCallbackCLSID<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, lflags: i32, bstrdeviceid: Param1, peventguid: *const ::windows::core::GUID, pclsid: *const ::windows::core::GUID, bstrname: Param4, bstrdescription: Param5, bstricon: Param6) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterEventCallbackCLSID)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), bstrdeviceid.into_param().abi(), ::core::mem::transmute(peventguid), ::core::mem::transmute(pclsid), bstrname.into_param().abi(), bstrdescription.into_param().abi(), bstricon.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDeviceDlg<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndparent: Param0, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddDeviceDlg)(::core::mem::transmute_copy(self), hwndparent.into_param().abi(), ::core::mem::transmute(lflags)).ok()
    }
}
impl ::core::convert::From<IWiaDevMgr> for ::windows::core::IUnknown {
    fn from(value: IWiaDevMgr) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaDevMgr> for ::windows::core::IUnknown {
    fn from(value: &IWiaDevMgr) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWiaDevMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWiaDevMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaDevMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaDevMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaDevMgr {}
impl ::core::fmt::Debug for IWiaDevMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaDevMgr").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWiaDevMgr {
    type Vtable = IWiaDevMgr_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5eb2502a_8cf1_11d1_bf92_0060081ed811);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaDevMgr_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub EnumDeviceInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflag: i32, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwiaitemroot: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateDevice: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SelectDeviceDlg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut super::super::Foundation::BSTR, ppitemroot: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SelectDeviceDlg: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SelectDeviceDlgID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SelectDeviceDlgID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetImageDlg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, lintent: i32, pitemroot: ::windows::core::RawPtr, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pguidformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetImageDlg: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterEventCallbackProgram: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, peventguid: *const ::windows::core::GUID, bstrcommandline: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstricon: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterEventCallbackProgram: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterEventCallbackInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, peventguid: *const ::windows::core::GUID, piwiaeventcallback: ::windows::core::RawPtr, peventobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterEventCallbackInterface: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterEventCallbackCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, peventguid: *const ::windows::core::GUID, pclsid: *const ::windows::core::GUID, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstricon: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterEventCallbackCLSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AddDeviceDlg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, lflags: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddDeviceDlg: usize,
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
#[repr(transparent)]
pub struct IWiaDevMgr2(::windows::core::IUnknown);
impl IWiaDevMgr2 {
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn EnumDeviceInfo(&self, lflags: i32) -> ::windows::core::Result<IEnumWIA_DEV_INFO> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumDeviceInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(&mut result__)).from_abi::<IEnumWIA_DEV_INFO>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateDevice<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, lflags: i32, bstrdeviceid: Param1) -> ::windows::core::Result<IWiaItem2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateDevice)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), bstrdeviceid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWiaItem2>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SelectDeviceDlg<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndparent: Param0, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut super::super::Foundation::BSTR, ppitemroot: *mut ::core::option::Option<IWiaItem2>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SelectDeviceDlg)(::core::mem::transmute_copy(self), hwndparent.into_param().abi(), ::core::mem::transmute(ldevicetype), ::core::mem::transmute(lflags), ::core::mem::transmute(pbstrdeviceid), ::core::mem::transmute(ppitemroot)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SelectDeviceDlgID<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndparent: Param0, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SelectDeviceDlgID)(::core::mem::transmute_copy(self), hwndparent.into_param().abi(), ::core::mem::transmute(ldevicetype), ::core::mem::transmute(lflags), ::core::mem::transmute(pbstrdeviceid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterEventCallbackInterface<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, IWiaEventCallback>>(&self, lflags: i32, bstrdeviceid: Param1, peventguid: *const ::windows::core::GUID, piwiaeventcallback: Param3) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RegisterEventCallbackInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), bstrdeviceid.into_param().abi(), ::core::mem::transmute(peventguid), piwiaeventcallback.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterEventCallbackProgram<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param7: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, lflags: i32, bstrdeviceid: Param1, peventguid: *const ::windows::core::GUID, bstrfullappname: Param3, bstrcommandlinearg: Param4, bstrname: Param5, bstrdescription: Param6, bstricon: Param7) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterEventCallbackProgram)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), bstrdeviceid.into_param().abi(), ::core::mem::transmute(peventguid), bstrfullappname.into_param().abi(), bstrcommandlinearg.into_param().abi(), bstrname.into_param().abi(), bstrdescription.into_param().abi(), bstricon.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterEventCallbackCLSID<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, lflags: i32, bstrdeviceid: Param1, peventguid: *const ::windows::core::GUID, pclsid: *const ::windows::core::GUID, bstrname: Param4, bstrdescription: Param5, bstricon: Param6) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterEventCallbackCLSID)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), bstrdeviceid.into_param().abi(), ::core::mem::transmute(peventguid), ::core::mem::transmute(pclsid), bstrname.into_param().abi(), bstrdescription.into_param().abi(), bstricon.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetImageDlg<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, lflags: i32, bstrdeviceid: Param1, hwndparent: Param2, bstrfoldername: Param3, bstrfilename: Param4, plnumfiles: *mut i32, ppbstrfilepaths: *mut *mut super::super::Foundation::BSTR, ppitem: *mut ::core::option::Option<IWiaItem2>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetImageDlg)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), bstrdeviceid.into_param().abi(), hwndparent.into_param().abi(), bstrfoldername.into_param().abi(), bstrfilename.into_param().abi(), ::core::mem::transmute(plnumfiles), ::core::mem::transmute(ppbstrfilepaths), ::core::mem::transmute(ppitem)).ok()
    }
}
impl ::core::convert::From<IWiaDevMgr2> for ::windows::core::IUnknown {
    fn from(value: IWiaDevMgr2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaDevMgr2> for ::windows::core::IUnknown {
    fn from(value: &IWiaDevMgr2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWiaDevMgr2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWiaDevMgr2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaDevMgr2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaDevMgr2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaDevMgr2 {}
impl ::core::fmt::Debug for IWiaDevMgr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaDevMgr2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWiaDevMgr2 {
    type Vtable = IWiaDevMgr2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79c07cf1_cbdd_41ee_8ec3_f00080cada7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaDevMgr2_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub EnumDeviceInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwiaitem2root: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateDevice: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SelectDeviceDlg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut super::super::Foundation::BSTR, ppitemroot: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SelectDeviceDlg: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SelectDeviceDlgID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SelectDeviceDlgID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterEventCallbackInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, peventguid: *const ::windows::core::GUID, piwiaeventcallback: ::windows::core::RawPtr, peventobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterEventCallbackInterface: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterEventCallbackProgram: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, peventguid: *const ::windows::core::GUID, bstrfullappname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrcommandlinearg: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstricon: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterEventCallbackProgram: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterEventCallbackCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, peventguid: *const ::windows::core::GUID, pclsid: *const ::windows::core::GUID, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstricon: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterEventCallbackCLSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetImageDlg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, hwndparent: super::super::Foundation::HWND, bstrfoldername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plnumfiles: *mut i32, ppbstrfilepaths: *mut *mut super::super::Foundation::BSTR, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetImageDlg: usize,
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
#[repr(transparent)]
pub struct IWiaDrvItem(::windows::core::IUnknown);
impl IWiaDrvItem {
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn GetItemFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetItemFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn GetDeviceSpecContext(&self) -> ::windows::core::Result<*mut u8> {
        let mut result__: *mut u8 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDeviceSpecContext)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut u8>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFullItemName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFullItemName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetItemName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetItemName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn AddItemToFolder<'a, Param0: ::windows::core::IntoParam<'a, IWiaDrvItem>>(&self, __midl__iwiadrvitem0004: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddItemToFolder)(::core::mem::transmute_copy(self), __midl__iwiadrvitem0004.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn UnlinkItemTree(&self, __midl__iwiadrvitem0005: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnlinkItemTree)(::core::mem::transmute_copy(self), ::core::mem::transmute(__midl__iwiadrvitem0005)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn RemoveItemFromFolder(&self, __midl__iwiadrvitem0006: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveItemFromFolder)(::core::mem::transmute_copy(self), ::core::mem::transmute(__midl__iwiadrvitem0006)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindItemByName<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, __midl__iwiadrvitem0007: i32, __midl__iwiadrvitem0008: Param1) -> ::windows::core::Result<IWiaDrvItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FindItemByName)(::core::mem::transmute_copy(self), ::core::mem::transmute(__midl__iwiadrvitem0007), __midl__iwiadrvitem0008.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWiaDrvItem>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindChildItemByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, __midl__iwiadrvitem0010: Param0) -> ::windows::core::Result<IWiaDrvItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FindChildItemByName)(::core::mem::transmute_copy(self), __midl__iwiadrvitem0010.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWiaDrvItem>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn GetParentItem(&self) -> ::windows::core::Result<IWiaDrvItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetParentItem)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWiaDrvItem>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn GetFirstChildItem(&self) -> ::windows::core::Result<IWiaDrvItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFirstChildItem)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWiaDrvItem>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn GetNextSiblingItem(&self) -> ::windows::core::Result<IWiaDrvItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetNextSiblingItem)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWiaDrvItem>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DumpItemData(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DumpItemData)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<IWiaDrvItem> for ::windows::core::IUnknown {
    fn from(value: IWiaDrvItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaDrvItem> for ::windows::core::IUnknown {
    fn from(value: &IWiaDrvItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWiaDrvItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWiaDrvItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaDrvItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaDrvItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaDrvItem {}
impl ::core::fmt::Debug for IWiaDrvItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaDrvItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWiaDrvItem {
    type Vtable = IWiaDrvItem_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f02b5c5_b00c_11d2_a094_00c04f72dc3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaDrvItem_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetItemFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0000: *mut i32) -> ::windows::core::HRESULT,
    pub GetDeviceSpecContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0001: *mut *mut u8) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFullItemName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0002: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFullItemName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetItemName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0003: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetItemName: usize,
    pub AddItemToFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0004: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub UnlinkItemTree: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0005: i32) -> ::windows::core::HRESULT,
    pub RemoveItemFromFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0006: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FindItemByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0007: i32, __midl__iwiadrvitem0008: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, __midl__iwiadrvitem0009: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FindItemByName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FindChildItemByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0010: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, __midl__iwiadrvitem0011: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FindChildItemByName: usize,
    pub GetParentItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0012: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetFirstChildItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0013: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetNextSiblingItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0014: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DumpItemData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiadrvitem0015: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DumpItemData: usize,
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
#[repr(transparent)]
pub struct IWiaErrorHandler(::windows::core::IUnknown);
impl IWiaErrorHandler {
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportStatus<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param2: ::windows::core::IntoParam<'a, IWiaItem2>>(&self, lflags: i32, hwndparent: Param1, pwiaitem2: Param2, hrstatus: ::windows::core::HRESULT, lpercentcomplete: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReportStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), hwndparent.into_param().abi(), pwiaitem2.into_param().abi(), ::core::mem::transmute(hrstatus), ::core::mem::transmute(lpercentcomplete)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStatusDescription<'a, Param1: ::windows::core::IntoParam<'a, IWiaItem2>>(&self, lflags: i32, pwiaitem2: Param1, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStatusDescription)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), pwiaitem2.into_param().abi(), ::core::mem::transmute(hrstatus), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<IWiaErrorHandler> for ::windows::core::IUnknown {
    fn from(value: IWiaErrorHandler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaErrorHandler> for ::windows::core::IUnknown {
    fn from(value: &IWiaErrorHandler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWiaErrorHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWiaErrorHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaErrorHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaErrorHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaErrorHandler {}
impl ::core::fmt::Debug for IWiaErrorHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaErrorHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWiaErrorHandler {
    type Vtable = IWiaErrorHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e4a51b1_bc1f_443d_a835_72e890759ef3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaErrorHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ReportStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, hwndparent: super::super::Foundation::HWND, pwiaitem2: ::windows::core::RawPtr, hrstatus: ::windows::core::HRESULT, lpercentcomplete: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReportStatus: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetStatusDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pwiaitem2: ::windows::core::RawPtr, hrstatus: ::windows::core::HRESULT, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetStatusDescription: usize,
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
#[repr(transparent)]
pub struct IWiaEventCallback(::windows::core::IUnknown);
impl IWiaEventCallback {
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImageEventCallback<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, peventguid: *const ::windows::core::GUID, bstreventdescription: Param1, bstrdeviceid: Param2, bstrdevicedescription: Param3, dwdevicetype: u32, bstrfullitemname: Param5, puleventtype: *mut u32, ulreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ImageEventCallback)(::core::mem::transmute_copy(self), ::core::mem::transmute(peventguid), bstreventdescription.into_param().abi(), bstrdeviceid.into_param().abi(), bstrdevicedescription.into_param().abi(), ::core::mem::transmute(dwdevicetype), bstrfullitemname.into_param().abi(), ::core::mem::transmute(puleventtype), ::core::mem::transmute(ulreserved)).ok()
    }
}
impl ::core::convert::From<IWiaEventCallback> for ::windows::core::IUnknown {
    fn from(value: IWiaEventCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaEventCallback> for ::windows::core::IUnknown {
    fn from(value: &IWiaEventCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWiaEventCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWiaEventCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaEventCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaEventCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaEventCallback {}
impl ::core::fmt::Debug for IWiaEventCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaEventCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWiaEventCallback {
    type Vtable = IWiaEventCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae6287b0_0084_11d2_973b_00a0c9068f2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaEventCallback_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ImageEventCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peventguid: *const ::windows::core::GUID, bstreventdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdevicedescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwdevicetype: u32, bstrfullitemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, puleventtype: *mut u32, ulreserved: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ImageEventCallback: usize,
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
#[repr(transparent)]
pub struct IWiaImageFilter(::windows::core::IUnknown);
impl IWiaImageFilter {
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn InitializeFilter<'a, Param0: ::windows::core::IntoParam<'a, IWiaItem2>, Param1: ::windows::core::IntoParam<'a, IWiaTransferCallback>>(&self, pwiaitem2: Param0, pwiatransfercallback: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InitializeFilter)(::core::mem::transmute_copy(self), pwiaitem2.into_param().abi(), pwiatransfercallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn SetNewCallback<'a, Param0: ::windows::core::IntoParam<'a, IWiaTransferCallback>>(&self, pwiatransfercallback: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetNewCallback)(::core::mem::transmute_copy(self), pwiatransfercallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn FilterPreviewImage<'a, Param1: ::windows::core::IntoParam<'a, IWiaItem2>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::RECT>, Param3: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, lflags: i32, pwiachilditem2: Param1, inputimageextents: Param2, pinputstream: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FilterPreviewImage)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), pwiachilditem2.into_param().abi(), inputimageextents.into_param().abi(), pinputstream.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn ApplyProperties<'a, Param0: ::windows::core::IntoParam<'a, IWiaPropertyStorage>>(&self, pwiapropertystorage: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ApplyProperties)(::core::mem::transmute_copy(self), pwiapropertystorage.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWiaImageFilter> for ::windows::core::IUnknown {
    fn from(value: IWiaImageFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaImageFilter> for ::windows::core::IUnknown {
    fn from(value: &IWiaImageFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWiaImageFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWiaImageFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaImageFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaImageFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaImageFilter {}
impl ::core::fmt::Debug for IWiaImageFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaImageFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWiaImageFilter {
    type Vtable = IWiaImageFilter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8a79ffa_450b_41f1_8f87_849ccd94ebf6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaImageFilter_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub InitializeFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwiaitem2: ::windows::core::RawPtr, pwiatransfercallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetNewCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwiatransfercallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub FilterPreviewImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pwiachilditem2: ::windows::core::RawPtr, inputimageextents: super::super::Foundation::RECT, pinputstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    FilterPreviewImage: usize,
    pub ApplyProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwiapropertystorage: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
#[repr(transparent)]
pub struct IWiaItem(::windows::core::IUnknown);
impl IWiaItem {
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn GetItemType(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetItemType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn AnalyzeItem(&self, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AnalyzeItem)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn EnumChildItems(&self) -> ::windows::core::Result<IEnumWiaItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumChildItems)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumWiaItem>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn DeleteItem(&self, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteItem)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateChildItem<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, lflags: i32, bstritemname: Param1, bstrfullitemname: Param2) -> ::windows::core::Result<IWiaItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateChildItem)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), bstritemname.into_param().abi(), bstrfullitemname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWiaItem>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn EnumRegisterEventInfo(&self, lflags: i32, peventguid: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumWIA_DEV_CAPS> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumRegisterEventInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(peventguid), ::core::mem::transmute(&mut result__)).from_abi::<IEnumWIA_DEV_CAPS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindItemByName<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, lflags: i32, bstrfullitemname: Param1) -> ::windows::core::Result<IWiaItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FindItemByName)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), bstrfullitemname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWiaItem>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeviceDlg<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndparent: Param0, lflags: i32, lintent: i32, plitemcount: *mut i32, ppiwiaitem: *mut *mut ::core::option::Option<IWiaItem>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeviceDlg)(::core::mem::transmute_copy(self), hwndparent.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(lintent), ::core::mem::transmute(plitemcount), ::core::mem::transmute(ppiwiaitem)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn DeviceCommand(&self, lflags: i32, pcmdguid: *const ::windows::core::GUID, piwiaitem: *mut ::core::option::Option<IWiaItem>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeviceCommand)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(pcmdguid), ::core::mem::transmute(piwiaitem)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn GetRootItem(&self) -> ::windows::core::Result<IWiaItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRootItem)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWiaItem>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn EnumDeviceCapabilities(&self, lflags: i32) -> ::windows::core::Result<IEnumWIA_DEV_CAPS> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumDeviceCapabilities)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(&mut result__)).from_abi::<IEnumWIA_DEV_CAPS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DumpItemData(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DumpItemData)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DumpDrvItemData(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DumpDrvItemData)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DumpTreeItemData(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DumpTreeItemData)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn Diagnostic(&self, pbuffer: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Diagnostic)(::core::mem::transmute_copy(self), pbuffer.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pbuffer))).ok()
    }
}
impl ::core::convert::From<IWiaItem> for ::windows::core::IUnknown {
    fn from(value: IWiaItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaItem> for ::windows::core::IUnknown {
    fn from(value: &IWiaItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWiaItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWiaItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaItem {}
impl ::core::fmt::Debug for IWiaItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWiaItem {
    type Vtable = IWiaItem_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4db1ad10_3391_11d2_9a33_00c04fa36145);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaItem_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetItemType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitemtype: *mut i32) -> ::windows::core::HRESULT,
    pub AnalyzeItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT,
    pub EnumChildItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppienumwiaitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub DeleteItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateChildItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfullitemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppiwiaitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateChildItem: usize,
    pub EnumRegisterEventInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, peventguid: *const ::windows::core::GUID, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FindItemByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, bstrfullitemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppiwiaitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FindItemByName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeviceDlg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, lflags: i32, lintent: i32, plitemcount: *mut i32, ppiwiaitem: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeviceDlg: usize,
    pub DeviceCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pcmdguid: *const ::windows::core::GUID, piwiaitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetRootItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiwiaitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub EnumDeviceCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, ppienumwia_dev_caps: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DumpItemData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DumpItemData: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DumpDrvItemData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DumpDrvItemData: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DumpTreeItemData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DumpTreeItemData: usize,
    pub Diagnostic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulsize: u32, pbuffer: *const u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
#[repr(transparent)]
pub struct IWiaItem2(::windows::core::IUnknown);
impl IWiaItem2 {
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateChildItem<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, litemflags: i32, lcreationflags: i32, bstritemname: Param2) -> ::windows::core::Result<IWiaItem2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateChildItem)(::core::mem::transmute_copy(self), ::core::mem::transmute(litemflags), ::core::mem::transmute(lcreationflags), bstritemname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWiaItem2>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn DeleteItem(&self, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteItem)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn EnumChildItems(&self, pcategoryguid: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumWiaItem2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumChildItems)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcategoryguid), ::core::mem::transmute(&mut result__)).from_abi::<IEnumWiaItem2>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindItemByName<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, lflags: i32, bstrfullitemname: Param1) -> ::windows::core::Result<IWiaItem2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FindItemByName)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), bstrfullitemname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWiaItem2>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn GetItemCategory(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetItemCategory)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn GetItemType(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetItemType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeviceDlg<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, lflags: i32, hwndparent: Param1, bstrfoldername: Param2, bstrfilename: Param3, plnumfiles: *mut i32, ppbstrfilepaths: *mut *mut super::super::Foundation::BSTR, ppitem: *mut ::core::option::Option<IWiaItem2>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeviceDlg)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), hwndparent.into_param().abi(), bstrfoldername.into_param().abi(), bstrfilename.into_param().abi(), ::core::mem::transmute(plnumfiles), ::core::mem::transmute(ppbstrfilepaths), ::core::mem::transmute(ppitem)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn DeviceCommand(&self, lflags: i32, pcmdguid: *const ::windows::core::GUID, ppiwiaitem2: *mut ::core::option::Option<IWiaItem2>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeviceCommand)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(pcmdguid), ::core::mem::transmute(ppiwiaitem2)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn EnumDeviceCapabilities(&self, lflags: i32) -> ::windows::core::Result<IEnumWIA_DEV_CAPS> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumDeviceCapabilities)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(&mut result__)).from_abi::<IEnumWIA_DEV_CAPS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CheckExtension<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, lflags: i32, bstrname: Param1, riidextensioninterface: *const ::windows::core::GUID, pbextensionexists: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CheckExtension)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), bstrname.into_param().abi(), ::core::mem::transmute(riidextensioninterface), ::core::mem::transmute(pbextensionexists)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetExtension<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, lflags: i32, bstrname: Param1, riidextensioninterface: *const ::windows::core::GUID, ppout: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetExtension)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), bstrname.into_param().abi(), ::core::mem::transmute(riidextensioninterface), ::core::mem::transmute(ppout)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn GetParentItem(&self) -> ::windows::core::Result<IWiaItem2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetParentItem)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWiaItem2>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn GetRootItem(&self) -> ::windows::core::Result<IWiaItem2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRootItem)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWiaItem2>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn GetPreviewComponent(&self, lflags: i32) -> ::windows::core::Result<IWiaPreview> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPreviewComponent)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(&mut result__)).from_abi::<IWiaPreview>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn EnumRegisterEventInfo(&self, lflags: i32, peventguid: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumWIA_DEV_CAPS> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumRegisterEventInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(peventguid), ::core::mem::transmute(&mut result__)).from_abi::<IEnumWIA_DEV_CAPS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn Diagnostic(&self, pbuffer: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Diagnostic)(::core::mem::transmute_copy(self), pbuffer.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pbuffer))).ok()
    }
}
impl ::core::convert::From<IWiaItem2> for ::windows::core::IUnknown {
    fn from(value: IWiaItem2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaItem2> for ::windows::core::IUnknown {
    fn from(value: &IWiaItem2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWiaItem2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWiaItem2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaItem2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaItem2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaItem2 {}
impl ::core::fmt::Debug for IWiaItem2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaItem2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWiaItem2 {
    type Vtable = IWiaItem2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6cba0075_1287_407d_9b77_cf0e030435cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaItem2_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateChildItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, litemflags: i32, lcreationflags: i32, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppiwiaitem2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateChildItem: usize,
    pub DeleteItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT,
    pub EnumChildItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcategoryguid: *const ::windows::core::GUID, ppienumwiaitem2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FindItemByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, bstrfullitemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppiwiaitem2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FindItemByName: usize,
    pub GetItemCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitemcategoryguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetItemType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitemtype: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DeviceDlg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, hwndparent: super::super::Foundation::HWND, bstrfoldername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plnumfiles: *mut i32, ppbstrfilepaths: *mut *mut super::super::Foundation::BSTR, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeviceDlg: usize,
    pub DeviceCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pcmdguid: *const ::windows::core::GUID, ppiwiaitem2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub EnumDeviceCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, ppienumwia_dev_caps: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CheckExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, riidextensioninterface: *const ::windows::core::GUID, pbextensionexists: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CheckExtension: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, riidextensioninterface: *const ::windows::core::GUID, ppout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetExtension: usize,
    pub GetParentItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiwiaitem2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetRootItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiwiaitem2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetPreviewComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, ppwiapreview: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub EnumRegisterEventInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, peventguid: *const ::windows::core::GUID, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Diagnostic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulsize: u32, pbuffer: *const u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
#[repr(transparent)]
pub struct IWiaItemExtras(::windows::core::IUnknown);
impl IWiaItemExtras {
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetExtendedErrorInfo(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetExtendedErrorInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn Escape(&self, dwescapecode: u32, lpindata: &[u8], poutdata: *mut u8, dwoutdatasize: u32, pdwactualdatasize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Escape)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwescapecode), ::core::mem::transmute(::windows::core::as_ptr_or_null(lpindata)), lpindata.len() as _, ::core::mem::transmute(poutdata), ::core::mem::transmute(dwoutdatasize), ::core::mem::transmute(pdwactualdatasize)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn CancelPendingIO(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CancelPendingIO)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWiaItemExtras> for ::windows::core::IUnknown {
    fn from(value: IWiaItemExtras) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaItemExtras> for ::windows::core::IUnknown {
    fn from(value: &IWiaItemExtras) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWiaItemExtras {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWiaItemExtras {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaItemExtras {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaItemExtras {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaItemExtras {}
impl ::core::fmt::Debug for IWiaItemExtras {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaItemExtras").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWiaItemExtras {
    type Vtable = IWiaItemExtras_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6291ef2c_36ef_4532_876a_8e132593778d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaItemExtras_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetExtendedErrorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrerrortext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetExtendedErrorInfo: usize,
    pub Escape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwescapecode: u32, lpindata: *const u8, cbindatasize: u32, poutdata: *mut u8, dwoutdatasize: u32, pdwactualdatasize: *mut u32) -> ::windows::core::HRESULT,
    pub CancelPendingIO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
#[repr(transparent)]
pub struct IWiaLog(::windows::core::IUnknown);
impl IWiaLog {
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn InitializeLog(&self, hinstance: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InitializeLog)(::core::mem::transmute_copy(self), ::core::mem::transmute(hinstance)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn hResult(&self, hresult: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).hResult)(::core::mem::transmute_copy(self), ::core::mem::transmute(hresult)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Log<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, lflags: i32, lresid: i32, ldetail: i32, bstrtext: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Log)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(lresid), ::core::mem::transmute(ldetail), bstrtext.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWiaLog> for ::windows::core::IUnknown {
    fn from(value: IWiaLog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaLog> for ::windows::core::IUnknown {
    fn from(value: &IWiaLog) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWiaLog {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWiaLog {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaLog {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaLog {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaLog {}
impl ::core::fmt::Debug for IWiaLog {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaLog").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWiaLog {
    type Vtable = IWiaLog_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa00c10b6_82a1_452f_8b6c_86062aad6890);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaLog_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub InitializeLog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hinstance: i32) -> ::windows::core::HRESULT,
    pub hResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Log: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, lresid: i32, ldetail: i32, bstrtext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Log: usize,
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
#[repr(transparent)]
pub struct IWiaLogEx(::windows::core::IUnknown);
impl IWiaLogEx {
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn InitializeLogEx(&self, hinstance: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InitializeLogEx)(::core::mem::transmute_copy(self), ::core::mem::transmute(hinstance)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn hResult(&self, hresult: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).hResult)(::core::mem::transmute_copy(self), ::core::mem::transmute(hresult)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Log<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, lflags: i32, lresid: i32, ldetail: i32, bstrtext: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Log)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(lresid), ::core::mem::transmute(ldetail), bstrtext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn hResultEx(&self, lmethodid: i32, hresult: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).hResultEx)(::core::mem::transmute_copy(self), ::core::mem::transmute(lmethodid), ::core::mem::transmute(hresult)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LogEx<'a, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, lmethodid: i32, lflags: i32, lresid: i32, ldetail: i32, bstrtext: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LogEx)(::core::mem::transmute_copy(self), ::core::mem::transmute(lmethodid), ::core::mem::transmute(lflags), ::core::mem::transmute(lresid), ::core::mem::transmute(ldetail), bstrtext.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWiaLogEx> for ::windows::core::IUnknown {
    fn from(value: IWiaLogEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaLogEx> for ::windows::core::IUnknown {
    fn from(value: &IWiaLogEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWiaLogEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWiaLogEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaLogEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaLogEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaLogEx {}
impl ::core::fmt::Debug for IWiaLogEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaLogEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWiaLogEx {
    type Vtable = IWiaLogEx_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf1f22ac_7a40_4787_b421_aeb47a1fbd0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaLogEx_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub InitializeLogEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hinstance: *const u8) -> ::windows::core::HRESULT,
    pub hResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Log: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, lresid: i32, ldetail: i32, bstrtext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Log: usize,
    pub hResultEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmethodid: i32, hresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LogEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmethodid: i32, lflags: i32, lresid: i32, ldetail: i32, bstrtext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LogEx: usize,
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
#[repr(transparent)]
pub struct IWiaMiniDrv(::windows::core::IUnknown);
impl IWiaMiniDrv {
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn drvInitializeWia<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param5: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, __midl__iwiaminidrv0000: *const u8, __midl__iwiaminidrv0001: i32, __midl__iwiaminidrv0002: Param2, __midl__iwiaminidrv0003: Param3, __midl__iwiaminidrv0004: Param4, __midl__iwiaminidrv0005: Param5, __midl__iwiaminidrv0006: *mut ::core::option::Option<IWiaDrvItem>, __midl__iwiaminidrv0007: *mut ::core::option::Option<::windows::core::IUnknown>, __midl__iwiaminidrv0008: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).drvInitializeWia)(::core::mem::transmute_copy(self), ::core::mem::transmute(__midl__iwiaminidrv0000), ::core::mem::transmute(__midl__iwiaminidrv0001), __midl__iwiaminidrv0002.into_param().abi(), __midl__iwiaminidrv0003.into_param().abi(), __midl__iwiaminidrv0004.into_param().abi(), __midl__iwiaminidrv0005.into_param().abi(), ::core::mem::transmute(__midl__iwiaminidrv0006), ::core::mem::transmute(__midl__iwiaminidrv0007), ::core::mem::transmute(__midl__iwiaminidrv0008)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn drvAcquireItemData(&self, __midl__iwiaminidrv0009: *const u8, __midl__iwiaminidrv0010: i32, __midl__iwiaminidrv0011: *mut MINIDRV_TRANSFER_CONTEXT, __midl__iwiaminidrv0012: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).drvAcquireItemData)(::core::mem::transmute_copy(self), ::core::mem::transmute(__midl__iwiaminidrv0009), ::core::mem::transmute(__midl__iwiaminidrv0010), ::core::mem::transmute(__midl__iwiaminidrv0011), ::core::mem::transmute(__midl__iwiaminidrv0012)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn drvInitItemProperties(&self, __midl__iwiaminidrv0013: *const u8, __midl__iwiaminidrv0014: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).drvInitItemProperties)(::core::mem::transmute_copy(self), ::core::mem::transmute(__midl__iwiaminidrv0013), ::core::mem::transmute(__midl__iwiaminidrv0014), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn drvValidateItemProperties(&self, __midl__iwiaminidrv0016: *const u8, __midl__iwiaminidrv0017: i32, __midl__iwiaminidrv0018: u32, __midl__iwiaminidrv0019: *const super::super::System::Com::StructuredStorage::PROPSPEC) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).drvValidateItemProperties)(::core::mem::transmute_copy(self), ::core::mem::transmute(__midl__iwiaminidrv0016), ::core::mem::transmute(__midl__iwiaminidrv0017), ::core::mem::transmute(__midl__iwiaminidrv0018), ::core::mem::transmute(__midl__iwiaminidrv0019), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn drvWriteItemProperties(&self, __midl__iwiaminidrv0021: *const u8, __midl__iwiaminidrv0022: i32, __midl__iwiaminidrv0023: *const MINIDRV_TRANSFER_CONTEXT) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).drvWriteItemProperties)(::core::mem::transmute_copy(self), ::core::mem::transmute(__midl__iwiaminidrv0021), ::core::mem::transmute(__midl__iwiaminidrv0022), ::core::mem::transmute(__midl__iwiaminidrv0023), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn drvReadItemProperties(&self, __midl__iwiaminidrv0025: *const u8, __midl__iwiaminidrv0026: i32, __midl__iwiaminidrv0027: u32, __midl__iwiaminidrv0028: *const super::super::System::Com::StructuredStorage::PROPSPEC) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).drvReadItemProperties)(::core::mem::transmute_copy(self), ::core::mem::transmute(__midl__iwiaminidrv0025), ::core::mem::transmute(__midl__iwiaminidrv0026), ::core::mem::transmute(__midl__iwiaminidrv0027), ::core::mem::transmute(__midl__iwiaminidrv0028), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn drvLockWiaDevice(&self, __midl__iwiaminidrv0030: *const u8, __midl__iwiaminidrv0031: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).drvLockWiaDevice)(::core::mem::transmute_copy(self), ::core::mem::transmute(__midl__iwiaminidrv0030), ::core::mem::transmute(__midl__iwiaminidrv0031), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn drvUnLockWiaDevice(&self, __midl__iwiaminidrv0033: *const u8, __midl__iwiaminidrv0034: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).drvUnLockWiaDevice)(::core::mem::transmute_copy(self), ::core::mem::transmute(__midl__iwiaminidrv0033), ::core::mem::transmute(__midl__iwiaminidrv0034), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn drvAnalyzeItem(&self, __midl__iwiaminidrv0036: *const u8, __midl__iwiaminidrv0037: i32, __midl__iwiaminidrv0038: *const i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).drvAnalyzeItem)(::core::mem::transmute_copy(self), ::core::mem::transmute(__midl__iwiaminidrv0036), ::core::mem::transmute(__midl__iwiaminidrv0037), ::core::mem::transmute(__midl__iwiaminidrv0038)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn drvGetDeviceErrorStr(&self, __midl__iwiaminidrv0039: i32, __midl__iwiaminidrv0040: i32, __midl__iwiaminidrv0041: *mut ::windows::core::PWSTR, __midl__iwiaminidrv0042: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).drvGetDeviceErrorStr)(::core::mem::transmute_copy(self), ::core::mem::transmute(__midl__iwiaminidrv0039), ::core::mem::transmute(__midl__iwiaminidrv0040), ::core::mem::transmute(__midl__iwiaminidrv0041), ::core::mem::transmute(__midl__iwiaminidrv0042)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn drvDeviceCommand(&self, __midl__iwiaminidrv0043: *const u8, __midl__iwiaminidrv0044: i32, __midl__iwiaminidrv0045: *const ::windows::core::GUID, __midl__iwiaminidrv0046: *mut ::core::option::Option<IWiaDrvItem>, __midl__iwiaminidrv0047: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).drvDeviceCommand)(::core::mem::transmute_copy(self), ::core::mem::transmute(__midl__iwiaminidrv0043), ::core::mem::transmute(__midl__iwiaminidrv0044), ::core::mem::transmute(__midl__iwiaminidrv0045), ::core::mem::transmute(__midl__iwiaminidrv0046), ::core::mem::transmute(__midl__iwiaminidrv0047)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn drvGetCapabilities(&self, __midl__iwiaminidrv0048: *const u8, __midl__iwiaminidrv0049: i32, __midl__iwiaminidrv0050: *mut i32, __midl__iwiaminidrv0051: *mut *mut WIA_DEV_CAP_DRV, __midl__iwiaminidrv0052: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).drvGetCapabilities)(::core::mem::transmute_copy(self), ::core::mem::transmute(__midl__iwiaminidrv0048), ::core::mem::transmute(__midl__iwiaminidrv0049), ::core::mem::transmute(__midl__iwiaminidrv0050), ::core::mem::transmute(__midl__iwiaminidrv0051), ::core::mem::transmute(__midl__iwiaminidrv0052)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn drvDeleteItem(&self, __midl__iwiaminidrv0053: *const u8, __midl__iwiaminidrv0054: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).drvDeleteItem)(::core::mem::transmute_copy(self), ::core::mem::transmute(__midl__iwiaminidrv0053), ::core::mem::transmute(__midl__iwiaminidrv0054), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn drvFreeDrvItemContext(&self, __midl__iwiaminidrv0056: i32, __midl__iwiaminidrv0057: *const u8) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).drvFreeDrvItemContext)(::core::mem::transmute_copy(self), ::core::mem::transmute(__midl__iwiaminidrv0056), ::core::mem::transmute(__midl__iwiaminidrv0057), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn drvGetWiaFormatInfo(&self, __midl__iwiaminidrv0059: *const u8, __midl__iwiaminidrv0060: i32, __midl__iwiaminidrv0061: *mut i32, __midl__iwiaminidrv0062: *mut *mut WIA_FORMAT_INFO, __midl__iwiaminidrv0063: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).drvGetWiaFormatInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(__midl__iwiaminidrv0059), ::core::mem::transmute(__midl__iwiaminidrv0060), ::core::mem::transmute(__midl__iwiaminidrv0061), ::core::mem::transmute(__midl__iwiaminidrv0062), ::core::mem::transmute(__midl__iwiaminidrv0063)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn drvNotifyPnpEvent<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, peventguid: *const ::windows::core::GUID, bstrdeviceid: Param1, ulreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).drvNotifyPnpEvent)(::core::mem::transmute_copy(self), ::core::mem::transmute(peventguid), bstrdeviceid.into_param().abi(), ::core::mem::transmute(ulreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn drvUnInitializeWia(&self, __midl__iwiaminidrv0064: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).drvUnInitializeWia)(::core::mem::transmute_copy(self), ::core::mem::transmute(__midl__iwiaminidrv0064)).ok()
    }
}
impl ::core::convert::From<IWiaMiniDrv> for ::windows::core::IUnknown {
    fn from(value: IWiaMiniDrv) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaMiniDrv> for ::windows::core::IUnknown {
    fn from(value: &IWiaMiniDrv) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWiaMiniDrv {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWiaMiniDrv {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaMiniDrv {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaMiniDrv {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaMiniDrv {}
impl ::core::fmt::Debug for IWiaMiniDrv {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaMiniDrv").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWiaMiniDrv {
    type Vtable = IWiaMiniDrv_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8cdee14_3c6c_11d2_9a35_00c04fa36145);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaMiniDrv_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub drvInitializeWia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0000: *const u8, __midl__iwiaminidrv0001: i32, __midl__iwiaminidrv0002: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, __midl__iwiaminidrv0003: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, __midl__iwiaminidrv0004: *mut ::core::ffi::c_void, __midl__iwiaminidrv0005: *mut ::core::ffi::c_void, __midl__iwiaminidrv0006: *mut ::windows::core::RawPtr, __midl__iwiaminidrv0007: *mut *mut ::core::ffi::c_void, __midl__iwiaminidrv0008: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    drvInitializeWia: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub drvAcquireItemData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0009: *const u8, __midl__iwiaminidrv0010: i32, __midl__iwiaminidrv0011: *mut MINIDRV_TRANSFER_CONTEXT, __midl__iwiaminidrv0012: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    drvAcquireItemData: usize,
    pub drvInitItemProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0013: *const u8, __midl__iwiaminidrv0014: i32, __midl__iwiaminidrv0015: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub drvValidateItemProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0016: *const u8, __midl__iwiaminidrv0017: i32, __midl__iwiaminidrv0018: u32, __midl__iwiaminidrv0019: *const super::super::System::Com::StructuredStorage::PROPSPEC, __midl__iwiaminidrv0020: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    drvValidateItemProperties: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub drvWriteItemProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0021: *const u8, __midl__iwiaminidrv0022: i32, __midl__iwiaminidrv0023: *const MINIDRV_TRANSFER_CONTEXT, __midl__iwiaminidrv0024: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    drvWriteItemProperties: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub drvReadItemProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0025: *const u8, __midl__iwiaminidrv0026: i32, __midl__iwiaminidrv0027: u32, __midl__iwiaminidrv0028: *const super::super::System::Com::StructuredStorage::PROPSPEC, __midl__iwiaminidrv0029: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    drvReadItemProperties: usize,
    pub drvLockWiaDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0030: *const u8, __midl__iwiaminidrv0031: i32, __midl__iwiaminidrv0032: *mut i32) -> ::windows::core::HRESULT,
    pub drvUnLockWiaDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0033: *const u8, __midl__iwiaminidrv0034: i32, __midl__iwiaminidrv0035: *mut i32) -> ::windows::core::HRESULT,
    pub drvAnalyzeItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0036: *const u8, __midl__iwiaminidrv0037: i32, __midl__iwiaminidrv0038: *const i32) -> ::windows::core::HRESULT,
    pub drvGetDeviceErrorStr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0039: i32, __midl__iwiaminidrv0040: i32, __midl__iwiaminidrv0041: *mut ::windows::core::PWSTR, __midl__iwiaminidrv0042: *mut i32) -> ::windows::core::HRESULT,
    pub drvDeviceCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0043: *const u8, __midl__iwiaminidrv0044: i32, __midl__iwiaminidrv0045: *const ::windows::core::GUID, __midl__iwiaminidrv0046: *mut ::windows::core::RawPtr, __midl__iwiaminidrv0047: *mut i32) -> ::windows::core::HRESULT,
    pub drvGetCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0048: *const u8, __midl__iwiaminidrv0049: i32, __midl__iwiaminidrv0050: *mut i32, __midl__iwiaminidrv0051: *mut *mut WIA_DEV_CAP_DRV, __midl__iwiaminidrv0052: *mut i32) -> ::windows::core::HRESULT,
    pub drvDeleteItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0053: *const u8, __midl__iwiaminidrv0054: i32, __midl__iwiaminidrv0055: *mut i32) -> ::windows::core::HRESULT,
    pub drvFreeDrvItemContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0056: i32, __midl__iwiaminidrv0057: *const u8, __midl__iwiaminidrv0058: *mut i32) -> ::windows::core::HRESULT,
    pub drvGetWiaFormatInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0059: *const u8, __midl__iwiaminidrv0060: i32, __midl__iwiaminidrv0061: *mut i32, __midl__iwiaminidrv0062: *mut *mut WIA_FORMAT_INFO, __midl__iwiaminidrv0063: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub drvNotifyPnpEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peventguid: *const ::windows::core::GUID, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ulreserved: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    drvNotifyPnpEvent: usize,
    pub drvUnInitializeWia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__iwiaminidrv0064: *const u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
#[repr(transparent)]
pub struct IWiaMiniDrvCallBack(::windows::core::IUnknown);
impl IWiaMiniDrvCallBack {
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MiniDrvCallback(&self, lreason: i32, lstatus: i32, lpercentcomplete: i32, loffset: i32, llength: i32, ptranctx: *const MINIDRV_TRANSFER_CONTEXT, lreserved: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MiniDrvCallback)(::core::mem::transmute_copy(self), ::core::mem::transmute(lreason), ::core::mem::transmute(lstatus), ::core::mem::transmute(lpercentcomplete), ::core::mem::transmute(loffset), ::core::mem::transmute(llength), ::core::mem::transmute(ptranctx), ::core::mem::transmute(lreserved)).ok()
    }
}
impl ::core::convert::From<IWiaMiniDrvCallBack> for ::windows::core::IUnknown {
    fn from(value: IWiaMiniDrvCallBack) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaMiniDrvCallBack> for ::windows::core::IUnknown {
    fn from(value: &IWiaMiniDrvCallBack) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWiaMiniDrvCallBack {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWiaMiniDrvCallBack {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaMiniDrvCallBack {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaMiniDrvCallBack {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaMiniDrvCallBack {}
impl ::core::fmt::Debug for IWiaMiniDrvCallBack {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaMiniDrvCallBack").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWiaMiniDrvCallBack {
    type Vtable = IWiaMiniDrvCallBack_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33a57d5a_3de8_11d2_9a36_00c04fa36145);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaMiniDrvCallBack_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub MiniDrvCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lreason: i32, lstatus: i32, lpercentcomplete: i32, loffset: i32, llength: i32, ptranctx: *const MINIDRV_TRANSFER_CONTEXT, lreserved: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MiniDrvCallback: usize,
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
#[repr(transparent)]
pub struct IWiaMiniDrvTransferCallback(::windows::core::IUnknown);
impl IWiaMiniDrvTransferCallback {
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetNextStream<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, lflags: i32, bstritemname: Param1, bstrfullitemname: Param2) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetNextStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), bstritemname.into_param().abi(), bstrfullitemname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn SendMessage(&self, lflags: i32, pwiatransferparams: *const WiaTransferParams) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SendMessage)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(pwiatransferparams)).ok()
    }
}
impl ::core::convert::From<IWiaMiniDrvTransferCallback> for ::windows::core::IUnknown {
    fn from(value: IWiaMiniDrvTransferCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaMiniDrvTransferCallback> for ::windows::core::IUnknown {
    fn from(value: &IWiaMiniDrvTransferCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWiaMiniDrvTransferCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWiaMiniDrvTransferCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaMiniDrvTransferCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaMiniDrvTransferCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaMiniDrvTransferCallback {}
impl ::core::fmt::Debug for IWiaMiniDrvTransferCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaMiniDrvTransferCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWiaMiniDrvTransferCallback {
    type Vtable = IWiaMiniDrvTransferCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9d2ee89_2ce5_4ff0_8adb_c961d1d774ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaMiniDrvTransferCallback_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetNextStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfullitemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppistream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetNextStream: usize,
    pub SendMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pwiatransferparams: *const WiaTransferParams) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
#[repr(transparent)]
pub struct IWiaNotifyDevMgr(::windows::core::IUnknown);
impl IWiaNotifyDevMgr {
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn NewDeviceArrival(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NewDeviceArrival)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWiaNotifyDevMgr> for ::windows::core::IUnknown {
    fn from(value: IWiaNotifyDevMgr) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaNotifyDevMgr> for ::windows::core::IUnknown {
    fn from(value: &IWiaNotifyDevMgr) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWiaNotifyDevMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWiaNotifyDevMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaNotifyDevMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaNotifyDevMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaNotifyDevMgr {}
impl ::core::fmt::Debug for IWiaNotifyDevMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaNotifyDevMgr").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWiaNotifyDevMgr {
    type Vtable = IWiaNotifyDevMgr_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70681ea0_e7bf_4291_9fb1_4e8813a3f78e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaNotifyDevMgr_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub NewDeviceArrival: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
#[repr(transparent)]
pub struct IWiaPreview(::windows::core::IUnknown);
impl IWiaPreview {
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn GetNewPreview<'a, Param1: ::windows::core::IntoParam<'a, IWiaItem2>, Param2: ::windows::core::IntoParam<'a, IWiaTransferCallback>>(&self, lflags: i32, pwiaitem2: Param1, pwiatransfercallback: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetNewPreview)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), pwiaitem2.into_param().abi(), pwiatransfercallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn UpdatePreview<'a, Param1: ::windows::core::IntoParam<'a, IWiaItem2>, Param2: ::windows::core::IntoParam<'a, IWiaTransferCallback>>(&self, lflags: i32, pchildwiaitem2: Param1, pwiatransfercallback: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UpdatePreview)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), pchildwiaitem2.into_param().abi(), pwiatransfercallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn DetectRegions(&self, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DetectRegions)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Clear)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWiaPreview> for ::windows::core::IUnknown {
    fn from(value: IWiaPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaPreview> for ::windows::core::IUnknown {
    fn from(value: &IWiaPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWiaPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWiaPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaPreview {}
impl ::core::fmt::Debug for IWiaPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaPreview").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWiaPreview {
    type Vtable = IWiaPreview_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95c2b4fd_33f2_4d86_ad40_9431f0df08f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaPreview_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetNewPreview: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pwiaitem2: ::windows::core::RawPtr, pwiatransfercallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub UpdatePreview: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pchildwiaitem2: ::windows::core::RawPtr, pwiatransfercallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub DetectRegions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
#[repr(transparent)]
pub struct IWiaPropertyStorage(::windows::core::IUnknown);
impl IWiaPropertyStorage {
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn ReadMultiple<'a, const PARAM0: usize>(&self, rgpspec: &[super::super::System::Com::StructuredStorage::PROPSPEC; PARAM0], rgpropvar: &mut [super::super::System::Com::StructuredStorage::PROPVARIANT; PARAM0]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReadMultiple)(::core::mem::transmute_copy(self), PARAM0 as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(rgpspec)), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(rgpropvar))).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn WriteMultiple(&self, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC, rgpropvar: *const super::super::System::Com::StructuredStorage::PROPVARIANT, propidnamefirst: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteMultiple)(::core::mem::transmute_copy(self), ::core::mem::transmute(cpspec), ::core::mem::transmute(rgpspec), ::core::mem::transmute(rgpropvar), ::core::mem::transmute(propidnamefirst)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn DeleteMultiple(&self, rgpspec: &[super::super::System::Com::StructuredStorage::PROPSPEC]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteMultiple)(::core::mem::transmute_copy(self), rgpspec.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(rgpspec))).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn ReadPropertyNames<'a, const PARAM0: usize>(&self, rgpropid: &[u32; PARAM0], rglpwstrname: &mut [::windows::core::PWSTR; PARAM0]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReadPropertyNames)(::core::mem::transmute_copy(self), PARAM0 as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(rgpropid)), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(rglpwstrname))).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn WritePropertyNames<'a, const PARAM0: usize>(&self, rgpropid: &[u32; PARAM0], rglpwstrname: &[::windows::core::PWSTR; PARAM0]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WritePropertyNames)(::core::mem::transmute_copy(self), PARAM0 as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(rgpropid)), ::core::mem::transmute(::windows::core::as_ptr_or_null(rglpwstrname))).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn DeletePropertyNames(&self, rgpropid: &[u32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeletePropertyNames)(::core::mem::transmute_copy(self), rgpropid.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(rgpropid))).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn Commit(&self, grfcommitflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Commit)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfcommitflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn Revert(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Revert)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Enum(&self) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::IEnumSTATPROPSTG> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Enum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::StructuredStorage::IEnumSTATPROPSTG>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTimes(&self, pctime: *const super::super::Foundation::FILETIME, patime: *const super::super::Foundation::FILETIME, pmtime: *const super::super::Foundation::FILETIME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTimes)(::core::mem::transmute_copy(self), ::core::mem::transmute(pctime), ::core::mem::transmute(patime), ::core::mem::transmute(pmtime)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn SetClass(&self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetClass)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Stat(&self) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::STATPROPSETSTG> {
        let mut result__: super::super::System::Com::StructuredStorage::STATPROPSETSTG = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Stat)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::StructuredStorage::STATPROPSETSTG>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetPropertyAttributes<'a, const PARAM0: usize>(&self, rgpspec: &[super::super::System::Com::StructuredStorage::PROPSPEC; PARAM0], rgflags: &mut [u32; PARAM0], rgpropvar: &mut [super::super::System::Com::StructuredStorage::PROPVARIANT; PARAM0]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPropertyAttributes)(::core::mem::transmute_copy(self), PARAM0 as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(rgpspec)), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(rgflags)), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(rgpropvar))).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPropertyStream(&self, pcompatibilityid: *mut ::windows::core::GUID, ppistream: *mut ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPropertyStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcompatibilityid), ::core::mem::transmute(ppistream)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPropertyStream<'a, Param1: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, pcompatibilityid: *mut ::windows::core::GUID, pistream: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPropertyStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcompatibilityid), pistream.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWiaPropertyStorage> for ::windows::core::IUnknown {
    fn from(value: IWiaPropertyStorage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaPropertyStorage> for ::windows::core::IUnknown {
    fn from(value: &IWiaPropertyStorage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWiaPropertyStorage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWiaPropertyStorage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaPropertyStorage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaPropertyStorage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaPropertyStorage {}
impl ::core::fmt::Debug for IWiaPropertyStorage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaPropertyStorage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWiaPropertyStorage {
    type Vtable = IWiaPropertyStorage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98b5e8a0_29cc_491a_aac0_e6db4fdcceb6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaPropertyStorage_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub ReadMultiple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC, rgpropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    ReadMultiple: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub WriteMultiple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC, rgpropvar: *const super::super::System::Com::StructuredStorage::PROPVARIANT, propidnamefirst: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    WriteMultiple: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub DeleteMultiple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    DeleteMultiple: usize,
    pub ReadPropertyNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub WritePropertyNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32, rglpwstrname: *const ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub DeletePropertyNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32) -> ::windows::core::HRESULT,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfcommitflags: u32) -> ::windows::core::HRESULT,
    pub Revert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub Enum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    Enum: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTimes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctime: *const super::super::Foundation::FILETIME, patime: *const super::super::Foundation::FILETIME, pmtime: *const super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTimes: usize,
    pub SetClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub Stat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatpsstg: *mut super::super::System::Com::StructuredStorage::STATPROPSETSTG) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    Stat: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetPropertyAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC, rgflags: *mut u32, rgpropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetPropertyAttributes: usize,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulnumprops: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetPropertyStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcompatibilityid: *mut ::windows::core::GUID, ppistream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetPropertyStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetPropertyStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcompatibilityid: *mut ::windows::core::GUID, pistream: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetPropertyStream: usize,
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
#[repr(transparent)]
pub struct IWiaSegmentationFilter(::windows::core::IUnknown);
impl IWiaSegmentationFilter {
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DetectRegions<'a, Param1: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param2: ::windows::core::IntoParam<'a, IWiaItem2>>(&self, lflags: i32, pinputstream: Param1, pwiaitem2: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DetectRegions)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), pinputstream.into_param().abi(), pwiaitem2.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWiaSegmentationFilter> for ::windows::core::IUnknown {
    fn from(value: IWiaSegmentationFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaSegmentationFilter> for ::windows::core::IUnknown {
    fn from(value: &IWiaSegmentationFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWiaSegmentationFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWiaSegmentationFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaSegmentationFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaSegmentationFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaSegmentationFilter {}
impl ::core::fmt::Debug for IWiaSegmentationFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaSegmentationFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWiaSegmentationFilter {
    type Vtable = IWiaSegmentationFilter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec46a697_ac04_4447_8f65_ff63d5154b21);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaSegmentationFilter_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub DetectRegions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pinputstream: ::windows::core::RawPtr, pwiaitem2: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DetectRegions: usize,
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
#[repr(transparent)]
pub struct IWiaTransfer(::windows::core::IUnknown);
impl IWiaTransfer {
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn Download<'a, Param1: ::windows::core::IntoParam<'a, IWiaTransferCallback>>(&self, lflags: i32, piwiatransfercallback: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Download)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), piwiatransfercallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Upload<'a, Param1: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param2: ::windows::core::IntoParam<'a, IWiaTransferCallback>>(&self, lflags: i32, psource: Param1, piwiatransfercallback: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Upload)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), psource.into_param().abi(), piwiatransfercallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Cancel)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn EnumWIA_FORMAT_INFO(&self) -> ::windows::core::Result<IEnumWIA_FORMAT_INFO> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumWIA_FORMAT_INFO)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumWIA_FORMAT_INFO>(result__)
    }
}
impl ::core::convert::From<IWiaTransfer> for ::windows::core::IUnknown {
    fn from(value: IWiaTransfer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaTransfer> for ::windows::core::IUnknown {
    fn from(value: &IWiaTransfer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWiaTransfer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWiaTransfer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaTransfer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaTransfer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaTransfer {}
impl ::core::fmt::Debug for IWiaTransfer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaTransfer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWiaTransfer {
    type Vtable = IWiaTransfer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc39d6942_2f4e_4d04_92fe_4ef4d3a1de5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaTransfer_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Download: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, piwiatransfercallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Upload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, psource: ::windows::core::RawPtr, piwiatransfercallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Upload: usize,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumWIA_FORMAT_INFO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
#[repr(transparent)]
pub struct IWiaTransferCallback(::windows::core::IUnknown);
impl IWiaTransferCallback {
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn TransferCallback(&self, lflags: i32, pwiatransferparams: *const WiaTransferParams) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TransferCallback)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(pwiatransferparams)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetNextStream<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, lflags: i32, bstritemname: Param1, bstrfullitemname: Param2) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetNextStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), bstritemname.into_param().abi(), bstrfullitemname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IStream>(result__)
    }
}
impl ::core::convert::From<IWiaTransferCallback> for ::windows::core::IUnknown {
    fn from(value: IWiaTransferCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaTransferCallback> for ::windows::core::IUnknown {
    fn from(value: &IWiaTransferCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWiaTransferCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWiaTransferCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaTransferCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaTransferCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaTransferCallback {}
impl ::core::fmt::Debug for IWiaTransferCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaTransferCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWiaTransferCallback {
    type Vtable = IWiaTransferCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27d4eaaf_28a6_4ca5_9aab_e678168b9527);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaTransferCallback_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub TransferCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pwiatransferparams: *const WiaTransferParams) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetNextStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, bstritemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfullitemname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdestination: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetNextStream: usize,
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
#[repr(transparent)]
pub struct IWiaUIExtension(::windows::core::IUnknown);
impl IWiaUIExtension {
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeviceDialog(&self, pdevicedialogdata: *const DEVICEDIALOGDATA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeviceDialog)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdevicedialogdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn GetDeviceIcon<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdeviceid: Param0, phicon: *mut super::super::UI::WindowsAndMessaging::HICON, nsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDeviceIcon)(::core::mem::transmute_copy(self), bstrdeviceid.into_param().abi(), ::core::mem::transmute(phicon), ::core::mem::transmute(nsize)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDeviceBitmapLogo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdeviceid: Param0, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP, nmaxwidth: u32, nmaxheight: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDeviceBitmapLogo)(::core::mem::transmute_copy(self), bstrdeviceid.into_param().abi(), ::core::mem::transmute(phbitmap), ::core::mem::transmute(nmaxwidth), ::core::mem::transmute(nmaxheight)).ok()
    }
}
impl ::core::convert::From<IWiaUIExtension> for ::windows::core::IUnknown {
    fn from(value: IWiaUIExtension) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaUIExtension> for ::windows::core::IUnknown {
    fn from(value: &IWiaUIExtension) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWiaUIExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWiaUIExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaUIExtension {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaUIExtension {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaUIExtension {}
impl ::core::fmt::Debug for IWiaUIExtension {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaUIExtension").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWiaUIExtension {
    type Vtable = IWiaUIExtension_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda319113_50ee_4c80_b460_57d005d44a2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaUIExtension_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub DeviceDialog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevicedialogdata: *const DEVICEDIALOGDATA) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeviceDialog: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub GetDeviceIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, phicon: *mut super::super::UI::WindowsAndMessaging::HICON, nsize: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    GetDeviceIcon: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub GetDeviceBitmapLogo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP, nmaxwidth: u32, nmaxheight: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    GetDeviceBitmapLogo: usize,
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
#[repr(transparent)]
pub struct IWiaUIExtension2(::windows::core::IUnknown);
impl IWiaUIExtension2 {
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeviceDialog(&self, pdevicedialogdata: *const DEVICEDIALOGDATA2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeviceDialog)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdevicedialogdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn GetDeviceIcon<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdeviceid: Param0, phicon: *mut super::super::UI::WindowsAndMessaging::HICON, nsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDeviceIcon)(::core::mem::transmute_copy(self), bstrdeviceid.into_param().abi(), ::core::mem::transmute(phicon), ::core::mem::transmute(nsize)).ok()
    }
}
impl ::core::convert::From<IWiaUIExtension2> for ::windows::core::IUnknown {
    fn from(value: IWiaUIExtension2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaUIExtension2> for ::windows::core::IUnknown {
    fn from(value: &IWiaUIExtension2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWiaUIExtension2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWiaUIExtension2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaUIExtension2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaUIExtension2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaUIExtension2 {}
impl ::core::fmt::Debug for IWiaUIExtension2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaUIExtension2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWiaUIExtension2 {
    type Vtable = IWiaUIExtension2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x305600d7_5088_46d7_9a15_b77b09cdba7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaUIExtension2_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub DeviceDialog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevicedialogdata: *const DEVICEDIALOGDATA2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeviceDialog: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub GetDeviceIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, phicon: *mut super::super::UI::WindowsAndMessaging::HICON, nsize: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    GetDeviceIcon: usize,
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
#[repr(transparent)]
pub struct IWiaVideo(::windows::core::IUnknown);
impl IWiaVideo {
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PreviewVisible(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PreviewVisible)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPreviewVisible<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bpreviewvisible: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPreviewVisible)(::core::mem::transmute_copy(self), bpreviewvisible.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImagesDirectory(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ImagesDirectory)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetImagesDirectory<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrimagedirectory: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetImagesDirectory)(::core::mem::transmute_copy(self), bstrimagedirectory.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVideoByWiaDevID<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bstrwiadeviceid: Param0, hwndparent: Param1, bstretchtofitparent: Param2, bautobeginplayback: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateVideoByWiaDevID)(::core::mem::transmute_copy(self), bstrwiadeviceid.into_param().abi(), hwndparent.into_param().abi(), bstretchtofitparent.into_param().abi(), bautobeginplayback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVideoByDevNum<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, uidevicenumber: u32, hwndparent: Param1, bstretchtofitparent: Param2, bautobeginplayback: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateVideoByDevNum)(::core::mem::transmute_copy(self), ::core::mem::transmute(uidevicenumber), hwndparent.into_param().abi(), bstretchtofitparent.into_param().abi(), bautobeginplayback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVideoByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bstrfriendlyname: Param0, hwndparent: Param1, bstretchtofitparent: Param2, bautobeginplayback: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateVideoByName)(::core::mem::transmute_copy(self), bstrfriendlyname.into_param().abi(), hwndparent.into_param().abi(), bstretchtofitparent.into_param().abi(), bautobeginplayback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn DestroyVideo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DestroyVideo)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn Play(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Play)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn Pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Pause)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TakePicture(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TakePicture)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ResizeVideo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bstretchtofitparent: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResizeVideo)(::core::mem::transmute_copy(self), bstretchtofitparent.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
    pub unsafe fn GetCurrentState(&self) -> ::windows::core::Result<WIAVIDEO_STATE> {
        let mut result__: WIAVIDEO_STATE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCurrentState)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WIAVIDEO_STATE>(result__)
    }
}
impl ::core::convert::From<IWiaVideo> for ::windows::core::IUnknown {
    fn from(value: IWiaVideo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWiaVideo> for ::windows::core::IUnknown {
    fn from(value: &IWiaVideo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWiaVideo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWiaVideo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWiaVideo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWiaVideo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWiaVideo {}
impl ::core::fmt::Debug for IWiaVideo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWiaVideo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWiaVideo {
    type Vtable = IWiaVideo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd52920aa_db88_41f0_946c_e00dc0a19cfa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaVideo_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub PreviewVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbpreviewvisible: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PreviewVisible: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPreviewVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bpreviewvisible: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPreviewVisible: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ImagesDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrimagedirectory: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ImagesDirectory: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetImagesDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrimagedirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetImagesDirectory: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateVideoByWiaDevID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrwiadeviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, hwndparent: super::super::Foundation::HWND, bstretchtofitparent: super::super::Foundation::BOOL, bautobeginplayback: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateVideoByWiaDevID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateVideoByDevNum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uidevicenumber: u32, hwndparent: super::super::Foundation::HWND, bstretchtofitparent: super::super::Foundation::BOOL, bautobeginplayback: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateVideoByDevNum: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateVideoByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, hwndparent: super::super::Foundation::HWND, bstretchtofitparent: super::super::Foundation::BOOL, bautobeginplayback: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateVideoByName: usize,
    pub DestroyVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Play: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TakePicture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrnewimagefilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TakePicture: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ResizeVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstretchtofitparent: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ResizeVideo: usize,
    pub GetCurrentState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstate: *mut WIAVIDEO_STATE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const LAMP_ERR: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const LANDSCAPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const LANSCAPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const LEFT_JUSTIFIED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const LIGHT_SOURCE_DETECT_READY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const LIGHT_SOURCE_NEGATIVE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const LIGHT_SOURCE_POSITIVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const LIGHT_SOURCE_PRESENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const LIGHT_SOURCE_PRESENT_DETECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const LIGHT_SOURCE_READY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const LIGHT_SOURCE_SELECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const MAX_ANSI_CHAR: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const MAX_IO_HANDLES: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const MAX_RESERVED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const MCRO_ERROR_GENERAL_ERROR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const MCRO_ERROR_OFFLINE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const MCRO_ERROR_PAPER_EMPTY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const MCRO_ERROR_PAPER_JAM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const MCRO_ERROR_PAPER_PROBLEM: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const MCRO_ERROR_USER_INTERVENTION: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const MCRO_STATUS_OK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const MICR_READER: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const MICR_READER_READY: u32 = 65536u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MINIDRV_TRANSFER_CONTEXT {
    pub lSize: i32,
    pub lWidthInPixels: i32,
    pub lLines: i32,
    pub lDepth: i32,
    pub lXRes: i32,
    pub lYRes: i32,
    pub lCompression: i32,
    pub guidFormatID: ::windows::core::GUID,
    pub tymed: i32,
    pub hFile: isize,
    pub cbOffset: i32,
    pub lBufferSize: i32,
    pub lActiveBuffer: i32,
    pub lNumBuffers: i32,
    pub pBaseBuffer: *mut u8,
    pub pTransferBuffer: *mut u8,
    pub bTransferDataCB: super::super::Foundation::BOOL,
    pub bClassDrvAllocBuf: super::super::Foundation::BOOL,
    pub lClientAddress: isize,
    pub pIWiaMiniDrvCallBack: ::core::option::Option<IWiaMiniDrvCallBack>,
    pub lImageSize: i32,
    pub lHeaderSize: i32,
    pub lItemSize: i32,
    pub cbWidthInBytes: i32,
    pub lPage: i32,
    pub lCurIfdOffset: i32,
    pub lPrevIfdOffset: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MINIDRV_TRANSFER_CONTEXT {
    fn clone(&self) -> Self {
        Self {
            lSize: self.lSize,
            lWidthInPixels: self.lWidthInPixels,
            lLines: self.lLines,
            lDepth: self.lDepth,
            lXRes: self.lXRes,
            lYRes: self.lYRes,
            lCompression: self.lCompression,
            guidFormatID: self.guidFormatID,
            tymed: self.tymed,
            hFile: self.hFile,
            cbOffset: self.cbOffset,
            lBufferSize: self.lBufferSize,
            lActiveBuffer: self.lActiveBuffer,
            lNumBuffers: self.lNumBuffers,
            pBaseBuffer: self.pBaseBuffer,
            pTransferBuffer: self.pTransferBuffer,
            bTransferDataCB: self.bTransferDataCB,
            bClassDrvAllocBuf: self.bClassDrvAllocBuf,
            lClientAddress: self.lClientAddress,
            pIWiaMiniDrvCallBack: self.pIWiaMiniDrvCallBack.clone(),
            lImageSize: self.lImageSize,
            lHeaderSize: self.lHeaderSize,
            lItemSize: self.lItemSize,
            cbWidthInBytes: self.cbWidthInBytes,
            lPage: self.lPage,
            lCurIfdOffset: self.lCurIfdOffset,
            lPrevIfdOffset: self.lPrevIfdOffset,
        }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MINIDRV_TRANSFER_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MINIDRV_TRANSFER_CONTEXT")
            .field("lSize", &self.lSize)
            .field("lWidthInPixels", &self.lWidthInPixels)
            .field("lLines", &self.lLines)
            .field("lDepth", &self.lDepth)
            .field("lXRes", &self.lXRes)
            .field("lYRes", &self.lYRes)
            .field("lCompression", &self.lCompression)
            .field("guidFormatID", &self.guidFormatID)
            .field("tymed", &self.tymed)
            .field("hFile", &self.hFile)
            .field("cbOffset", &self.cbOffset)
            .field("lBufferSize", &self.lBufferSize)
            .field("lActiveBuffer", &self.lActiveBuffer)
            .field("lNumBuffers", &self.lNumBuffers)
            .field("pBaseBuffer", &self.pBaseBuffer)
            .field("pTransferBuffer", &self.pTransferBuffer)
            .field("bTransferDataCB", &self.bTransferDataCB)
            .field("bClassDrvAllocBuf", &self.bClassDrvAllocBuf)
            .field("lClientAddress", &self.lClientAddress)
            .field("pIWiaMiniDrvCallBack", &self.pIWiaMiniDrvCallBack)
            .field("lImageSize", &self.lImageSize)
            .field("lHeaderSize", &self.lHeaderSize)
            .field("lItemSize", &self.lItemSize)
            .field("cbWidthInBytes", &self.cbWidthInBytes)
            .field("lPage", &self.lPage)
            .field("lCurIfdOffset", &self.lCurIfdOffset)
            .field("lPrevIfdOffset", &self.lPrevIfdOffset)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MINIDRV_TRANSFER_CONTEXT {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MINIDRV_TRANSFER_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.lSize == other.lSize
            && self.lWidthInPixels == other.lWidthInPixels
            && self.lLines == other.lLines
            && self.lDepth == other.lDepth
            && self.lXRes == other.lXRes
            && self.lYRes == other.lYRes
            && self.lCompression == other.lCompression
            && self.guidFormatID == other.guidFormatID
            && self.tymed == other.tymed
            && self.hFile == other.hFile
            && self.cbOffset == other.cbOffset
            && self.lBufferSize == other.lBufferSize
            && self.lActiveBuffer == other.lActiveBuffer
            && self.lNumBuffers == other.lNumBuffers
            && self.pBaseBuffer == other.pBaseBuffer
            && self.pTransferBuffer == other.pTransferBuffer
            && self.bTransferDataCB == other.bTransferDataCB
            && self.bClassDrvAllocBuf == other.bClassDrvAllocBuf
            && self.lClientAddress == other.lClientAddress
            && self.pIWiaMiniDrvCallBack == other.pIWiaMiniDrvCallBack
            && self.lImageSize == other.lImageSize
            && self.lHeaderSize == other.lHeaderSize
            && self.lItemSize == other.lItemSize
            && self.cbWidthInBytes == other.cbWidthInBytes
            && self.lPage == other.lPage
            && self.lCurIfdOffset == other.lCurIfdOffset
            && self.lPrevIfdOffset == other.lPrevIfdOffset
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MINIDRV_TRANSFER_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MINIDRV_TRANSFER_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const MIRRORED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const MULTIPLE_FEED: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const NEXT_PAGE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const PAPER_JAM: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const PATCH_CODE_READER: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const PATCH_CODE_READER_READY: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const PATH_COVER_UP: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const PORTRAIT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const POWERMODE_BATTERY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const POWERMODE_LINE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const PREFEED: u32 = 256u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct RANGEVALUE {
    pub lMin: i32,
    pub lMax: i32,
    pub lStep: i32,
}
impl ::core::marker::Copy for RANGEVALUE {}
impl ::core::clone::Clone for RANGEVALUE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RANGEVALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RANGEVALUE").field("lMin", &self.lMin).field("lMax", &self.lMax).field("lStep", &self.lStep).finish()
    }
}
unsafe impl ::windows::core::Abi for RANGEVALUE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RANGEVALUE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RANGEVALUE>()) == 0 }
    }
}
impl ::core::cmp::Eq for RANGEVALUE {}
impl ::core::default::Default for RANGEVALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const RIGHT_JUSTIFIED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const ROT180: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const ROT270: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SCANINFO {
    pub ADF: i32,
    pub TPA: i32,
    pub Endorser: i32,
    pub OpticalXResolution: i32,
    pub OpticalYResolution: i32,
    pub BedWidth: i32,
    pub BedHeight: i32,
    pub IntensityRange: RANGEVALUE,
    pub ContrastRange: RANGEVALUE,
    pub SupportedCompressionType: i32,
    pub SupportedDataTypes: i32,
    pub WidthPixels: i32,
    pub WidthBytes: i32,
    pub Lines: i32,
    pub DataType: i32,
    pub PixelBits: i32,
    pub Intensity: i32,
    pub Contrast: i32,
    pub Xresolution: i32,
    pub Yresolution: i32,
    pub Window: SCANWINDOW,
    pub DitherPattern: i32,
    pub Negative: i32,
    pub Mirror: i32,
    pub AutoBack: i32,
    pub ColorDitherPattern: i32,
    pub ToneMap: i32,
    pub Compression: i32,
    pub RawDataFormat: i32,
    pub RawPixelOrder: i32,
    pub bNeedDataAlignment: i32,
    pub DelayBetweenRead: i32,
    pub MaxBufferSize: i32,
    pub DeviceIOHandles: [super::super::Foundation::HANDLE; 16],
    pub lReserved: [i32; 4],
    pub pMicroDriverContext: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SCANINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SCANINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SCANINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCANINFO")
            .field("ADF", &self.ADF)
            .field("TPA", &self.TPA)
            .field("Endorser", &self.Endorser)
            .field("OpticalXResolution", &self.OpticalXResolution)
            .field("OpticalYResolution", &self.OpticalYResolution)
            .field("BedWidth", &self.BedWidth)
            .field("BedHeight", &self.BedHeight)
            .field("IntensityRange", &self.IntensityRange)
            .field("ContrastRange", &self.ContrastRange)
            .field("SupportedCompressionType", &self.SupportedCompressionType)
            .field("SupportedDataTypes", &self.SupportedDataTypes)
            .field("WidthPixels", &self.WidthPixels)
            .field("WidthBytes", &self.WidthBytes)
            .field("Lines", &self.Lines)
            .field("DataType", &self.DataType)
            .field("PixelBits", &self.PixelBits)
            .field("Intensity", &self.Intensity)
            .field("Contrast", &self.Contrast)
            .field("Xresolution", &self.Xresolution)
            .field("Yresolution", &self.Yresolution)
            .field("Window", &self.Window)
            .field("DitherPattern", &self.DitherPattern)
            .field("Negative", &self.Negative)
            .field("Mirror", &self.Mirror)
            .field("AutoBack", &self.AutoBack)
            .field("ColorDitherPattern", &self.ColorDitherPattern)
            .field("ToneMap", &self.ToneMap)
            .field("Compression", &self.Compression)
            .field("RawDataFormat", &self.RawDataFormat)
            .field("RawPixelOrder", &self.RawPixelOrder)
            .field("bNeedDataAlignment", &self.bNeedDataAlignment)
            .field("DelayBetweenRead", &self.DelayBetweenRead)
            .field("MaxBufferSize", &self.MaxBufferSize)
            .field("DeviceIOHandles", &self.DeviceIOHandles)
            .field("lReserved", &self.lReserved)
            .field("pMicroDriverContext", &self.pMicroDriverContext)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SCANINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SCANINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCANINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SCANINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SCANINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const SCANMODE_FINALSCAN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const SCANMODE_PREVIEWSCAN: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct SCANWINDOW {
    pub xPos: i32,
    pub yPos: i32,
    pub xExtent: i32,
    pub yExtent: i32,
}
impl ::core::marker::Copy for SCANWINDOW {}
impl ::core::clone::Clone for SCANWINDOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCANWINDOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCANWINDOW").field("xPos", &self.xPos).field("yPos", &self.yPos).field("xExtent", &self.xExtent).field("yExtent", &self.yExtent).finish()
    }
}
unsafe impl ::windows::core::Abi for SCANWINDOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCANWINDOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCANWINDOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCANWINDOW {}
impl ::core::default::Default for SCANWINDOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const SCAN_FINISHED: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const SCAN_FIRST: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const SCAN_NEXT: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const SHELLEX_WIAUIEXTENSION_NAME: &'static str = "WiaDialogExtensionHandlers";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const STOR: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const STORAGE_FULL: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const STORAGE_READY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const SUPPORT_BW: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const SUPPORT_COLOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const SUPPORT_GRAYSCALE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const TOP_JUSTIFIED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const TRANSPARENCY_DYNAMIC_FRAME_SUPPORT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const TRANSPARENCY_STATIC_FRAME_SUPPORT: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct TWAIN_CAPABILITY {
    pub lSize: i32,
    pub lMSG: i32,
    pub lCapID: i32,
    pub lConType: i32,
    pub lRC: i32,
    pub lCC: i32,
    pub lDataSize: i32,
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for TWAIN_CAPABILITY {}
impl ::core::clone::Clone for TWAIN_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TWAIN_CAPABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TWAIN_CAPABILITY").field("lSize", &self.lSize).field("lMSG", &self.lMSG).field("lCapID", &self.lCapID).field("lConType", &self.lConType).field("lRC", &self.lRC).field("lCC", &self.lCC).field("lDataSize", &self.lDataSize).field("Data", &self.Data).finish()
    }
}
unsafe impl ::windows::core::Abi for TWAIN_CAPABILITY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TWAIN_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TWAIN_CAPABILITY>()) == 0 }
    }
}
impl ::core::cmp::Eq for TWAIN_CAPABILITY {}
impl ::core::default::Default for TWAIN_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const TYMED_CALLBACK: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const TYMED_MULTIPAGE_CALLBACK: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const TYMED_MULTIPAGE_FILE: u32 = 256u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VAL {
    pub lVal: i32,
    pub dblVal: f64,
    pub pGuid: *mut ::windows::core::GUID,
    pub pScanInfo: *mut SCANINFO,
    pub handle: isize,
    pub ppButtonNames: *mut *mut u16,
    pub pHandle: *mut super::super::Foundation::HANDLE,
    pub lReserved: i32,
    pub szVal: [super::super::Foundation::CHAR; 255],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VAL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VAL").field("lVal", &self.lVal).field("dblVal", &self.dblVal).field("pGuid", &self.pGuid).field("pScanInfo", &self.pScanInfo).field("handle", &self.handle).field("ppButtonNames", &self.ppButtonNames).field("pHandle", &self.pHandle).field("lReserved", &self.lReserved).field("szVal", &self.szVal).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for VAL {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VAL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VAL>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VAL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WHITEBALANCE_AUTO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WHITEBALANCE_DAYLIGHT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WHITEBALANCE_FLASH: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WHITEBALANCE_FLORESCENT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WHITEBALANCE_MANUAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WHITEBALANCE_ONEPUSH_AUTO: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WHITEBALANCE_TUNGSTEN: u32 = 6u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WIAS_CHANGED_VALUE_INFO {
    pub bChanged: super::super::Foundation::BOOL,
    pub vt: i32,
    pub Old: WIAS_CHANGED_VALUE_INFO_1,
    pub Current: WIAS_CHANGED_VALUE_INFO_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WIAS_CHANGED_VALUE_INFO {
    fn clone(&self) -> Self {
        Self { bChanged: self.bChanged, vt: self.vt, Old: self.Old.clone(), Current: self.Current.clone() }
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WIAS_CHANGED_VALUE_INFO {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WIAS_CHANGED_VALUE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.bChanged == other.bChanged && self.vt == other.vt && self.Old == other.Old && self.Current == other.Current
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WIAS_CHANGED_VALUE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WIAS_CHANGED_VALUE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union WIAS_CHANGED_VALUE_INFO_0 {
    pub lVal: i32,
    pub fltVal: f32,
    pub bstrVal: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    pub guidVal: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WIAS_CHANGED_VALUE_INFO_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WIAS_CHANGED_VALUE_INFO_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WIAS_CHANGED_VALUE_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIAS_CHANGED_VALUE_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WIAS_CHANGED_VALUE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WIAS_CHANGED_VALUE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union WIAS_CHANGED_VALUE_INFO_1 {
    pub lVal: i32,
    pub fltVal: f32,
    pub bstrVal: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    pub guidVal: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WIAS_CHANGED_VALUE_INFO_1 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WIAS_CHANGED_VALUE_INFO_1 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WIAS_CHANGED_VALUE_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIAS_CHANGED_VALUE_INFO_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WIAS_CHANGED_VALUE_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WIAS_CHANGED_VALUE_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct WIAS_DOWN_SAMPLE_INFO {
    pub ulOriginalWidth: u32,
    pub ulOriginalHeight: u32,
    pub ulBitsPerPixel: u32,
    pub ulXRes: u32,
    pub ulYRes: u32,
    pub ulDownSampledWidth: u32,
    pub ulDownSampledHeight: u32,
    pub ulActualSize: u32,
    pub ulDestBufSize: u32,
    pub ulSrcBufSize: u32,
    pub pSrcBuffer: *mut u8,
    pub pDestBuffer: *mut u8,
}
impl ::core::marker::Copy for WIAS_DOWN_SAMPLE_INFO {}
impl ::core::clone::Clone for WIAS_DOWN_SAMPLE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIAS_DOWN_SAMPLE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIAS_DOWN_SAMPLE_INFO")
            .field("ulOriginalWidth", &self.ulOriginalWidth)
            .field("ulOriginalHeight", &self.ulOriginalHeight)
            .field("ulBitsPerPixel", &self.ulBitsPerPixel)
            .field("ulXRes", &self.ulXRes)
            .field("ulYRes", &self.ulYRes)
            .field("ulDownSampledWidth", &self.ulDownSampledWidth)
            .field("ulDownSampledHeight", &self.ulDownSampledHeight)
            .field("ulActualSize", &self.ulActualSize)
            .field("ulDestBufSize", &self.ulDestBufSize)
            .field("ulSrcBufSize", &self.ulSrcBufSize)
            .field("pSrcBuffer", &self.pSrcBuffer)
            .field("pDestBuffer", &self.pDestBuffer)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for WIAS_DOWN_SAMPLE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIAS_DOWN_SAMPLE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIAS_DOWN_SAMPLE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIAS_DOWN_SAMPLE_INFO {}
impl ::core::default::Default for WIAS_DOWN_SAMPLE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct WIAS_ENDORSER_INFO {
    pub ulPageCount: u32,
    pub ulNumEndorserValues: u32,
    pub pEndorserValues: *mut WIAS_ENDORSER_VALUE,
}
impl ::core::marker::Copy for WIAS_ENDORSER_INFO {}
impl ::core::clone::Clone for WIAS_ENDORSER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIAS_ENDORSER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIAS_ENDORSER_INFO").field("ulPageCount", &self.ulPageCount).field("ulNumEndorserValues", &self.ulNumEndorserValues).field("pEndorserValues", &self.pEndorserValues).finish()
    }
}
unsafe impl ::windows::core::Abi for WIAS_ENDORSER_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIAS_ENDORSER_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIAS_ENDORSER_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIAS_ENDORSER_INFO {}
impl ::core::default::Default for WIAS_ENDORSER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct WIAS_ENDORSER_VALUE {
    pub wszTokenName: ::windows::core::PWSTR,
    pub wszValue: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for WIAS_ENDORSER_VALUE {}
impl ::core::clone::Clone for WIAS_ENDORSER_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIAS_ENDORSER_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIAS_ENDORSER_VALUE").field("wszTokenName", &self.wszTokenName).field("wszValue", &self.wszValue).finish()
    }
}
unsafe impl ::windows::core::Abi for WIAS_ENDORSER_VALUE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIAS_ENDORSER_VALUE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIAS_ENDORSER_VALUE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIAS_ENDORSER_VALUE {}
impl ::core::default::Default for WIAS_ENDORSER_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIAU_DEBUG_TSTR: &'static str = "S";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WIAVIDEO_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIAVIDEO_NO_VIDEO: WIAVIDEO_STATE = WIAVIDEO_STATE(1i32);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIAVIDEO_CREATING_VIDEO: WIAVIDEO_STATE = WIAVIDEO_STATE(2i32);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIAVIDEO_VIDEO_CREATED: WIAVIDEO_STATE = WIAVIDEO_STATE(3i32);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIAVIDEO_VIDEO_PLAYING: WIAVIDEO_STATE = WIAVIDEO_STATE(4i32);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIAVIDEO_VIDEO_PAUSED: WIAVIDEO_STATE = WIAVIDEO_STATE(5i32);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIAVIDEO_DESTROYING_VIDEO: WIAVIDEO_STATE = WIAVIDEO_STATE(6i32);
impl ::core::marker::Copy for WIAVIDEO_STATE {}
impl ::core::clone::Clone for WIAVIDEO_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WIAVIDEO_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WIAVIDEO_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WIAVIDEO_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WIAVIDEO_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ACTION_EVENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ADVANCED_PREVIEW: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ALARM_BEEP1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ALARM_BEEP10: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ALARM_BEEP2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ALARM_BEEP3: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ALARM_BEEP4: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ALARM_BEEP5: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ALARM_BEEP6: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ALARM_BEEP7: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ALARM_BEEP8: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ALARM_BEEP9: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ALARM_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_AUTO_CROP_DISABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_AUTO_CROP_MULTI: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_AUTO_CROP_SINGLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_AUTO_DESKEW_OFF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_AUTO_DESKEW_ON: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct WIA_BARCODES {
    pub Tag: u32,
    pub Version: u32,
    pub Size: u32,
    pub Count: u32,
    pub Barcodes: [WIA_BARCODE_INFO; 1],
}
impl ::core::marker::Copy for WIA_BARCODES {}
impl ::core::clone::Clone for WIA_BARCODES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIA_BARCODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_BARCODES").field("Tag", &self.Tag).field("Version", &self.Version).field("Size", &self.Size).field("Count", &self.Count).field("Barcodes", &self.Barcodes).finish()
    }
}
unsafe impl ::windows::core::Abi for WIA_BARCODES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIA_BARCODES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_BARCODES>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIA_BARCODES {}
impl ::core::default::Default for WIA_BARCODES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_AUTO_SEARCH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_AZTEC: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_CODABAR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_CODE128: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_CODE128A: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_CODE128B: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_CODE128C: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_CODE39: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_CODE39_FULLASCII: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_CODE39_MOD43: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_CODE93: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_CPCBINARY: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_CUSTOMBASE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_DATAMATRIX: u32 = 38u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_DATASTRIP: u32 = 39u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_EAN13: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_EAN8: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_EZCODE: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_FIM: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_GS1128: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_GS1DATABAR: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_HIGH_CAPACITY_COLOR: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_HORIZONTAL_SEARCH: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_HORIZONTAL_VERTICAL_SEARCH: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct WIA_BARCODE_INFO {
    pub Size: u32,
    pub Type: u32,
    pub Page: u32,
    pub Confidence: u32,
    pub XOffset: u32,
    pub YOffset: u32,
    pub Rotation: u32,
    pub Length: u32,
    pub Text: [u16; 1],
}
impl ::core::marker::Copy for WIA_BARCODE_INFO {}
impl ::core::clone::Clone for WIA_BARCODE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIA_BARCODE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_BARCODE_INFO").field("Size", &self.Size).field("Type", &self.Type).field("Page", &self.Page).field("Confidence", &self.Confidence).field("XOffset", &self.XOffset).field("YOffset", &self.YOffset).field("Rotation", &self.Rotation).field("Length", &self.Length).field("Text", &self.Text).finish()
    }
}
unsafe impl ::windows::core::Abi for WIA_BARCODE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIA_BARCODE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_BARCODE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIA_BARCODE_INFO {}
impl ::core::default::Default for WIA_BARCODE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_INTELLIGENT_MAIL: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_INTERLEAVED_2OF5: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_ITF14: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_JAN: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_MAXICODE: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_MSI: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_NONINTERLEAVED_2OF5: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_PDF417: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_PHARMACODE: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_PLANET: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_PLESSEY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_POSTBAR: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_POSTNETA: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_POSTNETB: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_POSTNETC: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_POSTNET_DPBC: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_QRCODE: u32 = 41u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_READER_AUTO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_READER_DISABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_READER_FEEDER_BACK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_READER_FEEDER_DUPLEX: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_READER_FEEDER_FRONT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_READER_FLATBED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_RM4SCC: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_SHOTCODE: u32 = 42u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_SMALLAZTEC: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_SPARQCODE: u32 = 43u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_TELEPEN: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_UPCA: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_UPCE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_VERTICAL_HORIZONTAL_SEARCH: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BARCODE_VERTICAL_SEARCH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BASIC_PREVIEW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BLANK_PAGE_DETECTION_DISABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BLANK_PAGE_DISCARD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_BLANK_PAGE_JOB_SEPARATOR: u32 = 2u32;
pub const WIA_CATEGORY_AUTO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdefe5fd8_6c97_4dde_b11e_cb509b270e11);
pub const WIA_CATEGORY_BARCODE_READER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36e178a0_473f_494b_af8f_6c3f6d7486fc);
pub const WIA_CATEGORY_ENDORSER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47102cc3_127f_4771_adfc_991ab8ee1e97);
pub const WIA_CATEGORY_FEEDER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe131934_f84c_42ad_8da4_6129cddd7288);
pub const WIA_CATEGORY_FEEDER_BACK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61ca74d4_39db_42aa_89b1_8c19c9cd4c23);
pub const WIA_CATEGORY_FEEDER_FRONT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4823175c_3b28_487b_a7e6_eebc17614fd1);
pub const WIA_CATEGORY_FILM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcf65be7_3ce3_4473_af85_f5d37d21b68a);
pub const WIA_CATEGORY_FINISHED_FILE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff2b77ca_cf84_432b_a735_3a130dde2a88);
pub const WIA_CATEGORY_FLATBED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb607b1f_43f3_488b_855b_fb703ec342a6);
pub const WIA_CATEGORY_FOLDER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc692a446_6f5a_481d_85bb_92e2e86fd30a);
pub const WIA_CATEGORY_IMPRINTER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc65016d_9202_43dd_91a7_64c2954cfb8b);
pub const WIA_CATEGORY_MICR_READER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b86c1ec_71bc_4645_b4d5_1b19da2be978);
pub const WIA_CATEGORY_PATCH_CODE_READER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8faa1a6d_9c8a_42cd_98b3_ee9700cbc74f);
pub const WIA_CATEGORY_ROOT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf193526f_59b8_4a26_9888_e16e4f97ce10);
pub const WIA_CMD_BUILD_DEVICE_TREE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9cba5ce0_dbea_11d2_8416_00c04fa36145);
pub const WIA_CMD_CHANGE_DOCUMENT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04e725b0_acae_11d2_a093_00c04f72dc3c);
pub const WIA_CMD_DELETE_ALL_ITEMS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe208c170_acad_11d2_a093_00c04f72dc3c);
pub const WIA_CMD_DELETE_DEVICE_TREE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73815942_dbea_11d2_8416_00c04fa36145);
pub const WIA_CMD_DIAGNOSTIC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10ff52f5_de04_4cf0_a5ad_691f8dce0141);
pub const WIA_CMD_FORMAT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3a693aa_f788_4d34_a5b0_be7190759a24);
pub const WIA_CMD_PAUSE_FEEDER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50985e4d_a5b2_4b71_9c95_6d7d7c469a43);
pub const WIA_CMD_START_FEEDER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a9df6c9_5f2d_4a39_9d6c_00456d047f00);
pub const WIA_CMD_STOP_FEEDER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd847b06d_3905_459c_9509_9b29cdb691e7);
pub const WIA_CMD_SYNCHRONIZE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b26b7b2_acad_11d2_a093_00c04f72dc3c);
pub const WIA_CMD_TAKE_PICTURE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf933cac_acad_11d2_a093_00c04f72dc3c);
pub const WIA_CMD_UNLOAD_DOCUMENT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f3b3d8e_acae_11d2_a093_00c04f72dc3c);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_COLOR_DROP_BLUE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_COLOR_DROP_DISABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_COLOR_DROP_GREEN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_COLOR_DROP_RED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_COLOR_DROP_RGB: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_COMPRESSION_AUTO: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_COMPRESSION_BI_RLE4: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_COMPRESSION_BI_RLE8: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_COMPRESSION_G3: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_COMPRESSION_G4: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_COMPRESSION_JBIG: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_COMPRESSION_JPEG: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_COMPRESSION_JPEG2K: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_COMPRESSION_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_COMPRESSION_PNG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DATA_AUTO: u32 = 100u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct WIA_DATA_CALLBACK_HEADER {
    pub lSize: i32,
    pub guidFormatID: ::windows::core::GUID,
    pub lBufferSize: i32,
    pub lPageCount: i32,
}
impl ::core::marker::Copy for WIA_DATA_CALLBACK_HEADER {}
impl ::core::clone::Clone for WIA_DATA_CALLBACK_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIA_DATA_CALLBACK_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_DATA_CALLBACK_HEADER").field("lSize", &self.lSize).field("guidFormatID", &self.guidFormatID).field("lBufferSize", &self.lBufferSize).field("lPageCount", &self.lPageCount).finish()
    }
}
unsafe impl ::windows::core::Abi for WIA_DATA_CALLBACK_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIA_DATA_CALLBACK_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_DATA_CALLBACK_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIA_DATA_CALLBACK_HEADER {}
impl ::core::default::Default for WIA_DATA_CALLBACK_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DATA_COLOR: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DATA_COLOR_DITHER: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DATA_COLOR_THRESHOLD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DATA_DITHER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DATA_GRAYSCALE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DATA_RAW_BGR: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DATA_RAW_CMY: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DATA_RAW_CMYK: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DATA_RAW_RGB: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DATA_RAW_YUV: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DATA_RAW_YUVK: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DATA_THRESHOLD: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WIA_DATA_TRANSFER_INFO {
    pub ulSize: u32,
    pub ulSection: u32,
    pub ulBufferSize: u32,
    pub bDoubleBuffer: super::super::Foundation::BOOL,
    pub ulReserved1: u32,
    pub ulReserved2: u32,
    pub ulReserved3: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WIA_DATA_TRANSFER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WIA_DATA_TRANSFER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WIA_DATA_TRANSFER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_DATA_TRANSFER_INFO").field("ulSize", &self.ulSize).field("ulSection", &self.ulSection).field("ulBufferSize", &self.ulBufferSize).field("bDoubleBuffer", &self.bDoubleBuffer).field("ulReserved1", &self.ulReserved1).field("ulReserved2", &self.ulReserved2).field("ulReserved3", &self.ulReserved3).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WIA_DATA_TRANSFER_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WIA_DATA_TRANSFER_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_DATA_TRANSFER_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WIA_DATA_TRANSFER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WIA_DATA_TRANSFER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DEPTH_AUTO: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DEVICE_COMMANDS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DEVICE_CONNECTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DEVICE_DIALOG_SINGLE_IMAGE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DEVICE_DIALOG_USE_COMMON_UI: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DEVICE_EVENTS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DEVICE_NOT_CONNECTED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DEVINFO_ENUM_ALL: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DEVINFO_ENUM_LOCAL: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WIA_DEV_CAP {
    pub guid: ::windows::core::GUID,
    pub ulFlags: u32,
    pub bstrName: super::super::Foundation::BSTR,
    pub bstrDescription: super::super::Foundation::BSTR,
    pub bstrIcon: super::super::Foundation::BSTR,
    pub bstrCommandline: super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WIA_DEV_CAP {
    fn clone(&self) -> Self {
        Self {
            guid: self.guid,
            ulFlags: self.ulFlags,
            bstrName: self.bstrName.clone(),
            bstrDescription: self.bstrDescription.clone(),
            bstrIcon: self.bstrIcon.clone(),
            bstrCommandline: self.bstrCommandline.clone(),
        }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WIA_DEV_CAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_DEV_CAP").field("guid", &self.guid).field("ulFlags", &self.ulFlags).field("bstrName", &self.bstrName).field("bstrDescription", &self.bstrDescription).field("bstrIcon", &self.bstrIcon).field("bstrCommandline", &self.bstrCommandline).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WIA_DEV_CAP {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WIA_DEV_CAP {
    fn eq(&self, other: &Self) -> bool {
        self.guid == other.guid && self.ulFlags == other.ulFlags && self.bstrName == other.bstrName && self.bstrDescription == other.bstrDescription && self.bstrIcon == other.bstrIcon && self.bstrCommandline == other.bstrCommandline
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WIA_DEV_CAP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WIA_DEV_CAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct WIA_DEV_CAP_DRV {
    pub guid: *mut ::windows::core::GUID,
    pub ulFlags: u32,
    pub wszName: ::windows::core::PWSTR,
    pub wszDescription: ::windows::core::PWSTR,
    pub wszIcon: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for WIA_DEV_CAP_DRV {}
impl ::core::clone::Clone for WIA_DEV_CAP_DRV {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIA_DEV_CAP_DRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_DEV_CAP_DRV").field("guid", &self.guid).field("ulFlags", &self.ulFlags).field("wszName", &self.wszName).field("wszDescription", &self.wszDescription).field("wszIcon", &self.wszIcon).finish()
    }
}
unsafe impl ::windows::core::Abi for WIA_DEV_CAP_DRV {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIA_DEV_CAP_DRV {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_DEV_CAP_DRV>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIA_DEV_CAP_DRV {}
impl ::core::default::Default for WIA_DEV_CAP_DRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_BAUDRATE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_BAUDRATE_STR: &'static str = "BaudRate";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_DEV_DESC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_DEV_DESC_STR: &'static str = "Description";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_DEV_ID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_DEV_ID_STR: &'static str = "Unique Device ID";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_DEV_NAME: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_DEV_NAME_STR: &'static str = "Name";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_DEV_TYPE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_DEV_TYPE_STR: &'static str = "Type";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_DRIVER_VERSION: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_DRIVER_VERSION_STR: &'static str = "Driver Version";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_FIRST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_HW_CONFIG: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_HW_CONFIG_STR: &'static str = "Hardware Configuration";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_PNP_ID: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_PNP_ID_STR: &'static str = "PnP ID String";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_PORT_NAME: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_PORT_NAME_STR: &'static str = "Port";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_REMOTE_DEV_ID: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_REMOTE_DEV_ID_STR: &'static str = "Remote Device ID";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_SERVER_NAME: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_SERVER_NAME_STR: &'static str = "Server";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_STI_DRIVER_VERSION: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_STI_DRIVER_VERSION_STR: &'static str = "STI Driver Version";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_STI_GEN_CAPABILITIES: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_STI_GEN_CAPABILITIES_STR: &'static str = "STI Generic Capabilities";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_UI_CLSID: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_UI_CLSID_STR: &'static str = "UI Class ID";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_VEND_DESC: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_VEND_DESC_STR: &'static str = "Manufacturer";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_WIA_VERSION: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DIP_WIA_VERSION_STR: &'static str = "WIA Version";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WIA_DITHER_PATTERN_DATA {
    pub lSize: i32,
    pub bstrPatternName: super::super::Foundation::BSTR,
    pub lPatternWidth: i32,
    pub lPatternLength: i32,
    pub cbPattern: i32,
    pub pbPattern: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WIA_DITHER_PATTERN_DATA {
    fn clone(&self) -> Self {
        Self {
            lSize: self.lSize,
            bstrPatternName: self.bstrPatternName.clone(),
            lPatternWidth: self.lPatternWidth,
            lPatternLength: self.lPatternLength,
            cbPattern: self.cbPattern,
            pbPattern: self.pbPattern,
        }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WIA_DITHER_PATTERN_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_DITHER_PATTERN_DATA").field("lSize", &self.lSize).field("bstrPatternName", &self.bstrPatternName).field("lPatternWidth", &self.lPatternWidth).field("lPatternLength", &self.lPatternLength).field("cbPattern", &self.cbPattern).field("pbPattern", &self.pbPattern).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WIA_DITHER_PATTERN_DATA {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WIA_DITHER_PATTERN_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.lSize == other.lSize && self.bstrPatternName == other.bstrPatternName && self.lPatternWidth == other.lPatternWidth && self.lPatternLength == other.lPatternLength && self.cbPattern == other.cbPattern && self.pbPattern == other.pbPattern
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WIA_DITHER_PATTERN_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WIA_DITHER_PATTERN_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DONT_SHOW_PREVIEW_CONTROL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DONT_USE_SEGMENTATION_FILTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPA_CONNECT_STATUS: u32 = 1027u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPA_CONNECT_STATUS_STR: &'static str = "Connect Status";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPA_DEVICE_TIME: u32 = 1028u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPA_DEVICE_TIME_STR: &'static str = "Device Time";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPA_FIRMWARE_VERSION: u32 = 1026u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPA_FIRMWARE_VERSION_STR: &'static str = "Firmware Version";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_ARTIST: u32 = 2091u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_ARTIST_STR: &'static str = "Artist";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_BATTERY_STATUS: u32 = 2065u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_BATTERY_STATUS_STR: &'static str = "Battery Status";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_BURST_INTERVAL: u32 = 2075u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_BURST_INTERVAL_STR: &'static str = "Burst Interval";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_BURST_NUMBER: u32 = 2076u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_BURST_NUMBER_STR: &'static str = "Burst Number";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_CAPTURE_DELAY: u32 = 2082u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_CAPTURE_DELAY_STR: &'static str = "Capture Delay";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_CAPTURE_MODE: u32 = 2081u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_CAPTURE_MODE_STR: &'static str = "Capture Mode";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_COMPRESSION_SETTING: u32 = 2071u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_COMPRESSION_SETTING_STR: &'static str = "Compression Setting";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_CONTRAST: u32 = 2080u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_CONTRAST_STR: &'static str = "Contrast";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_COPYRIGHT_INFO: u32 = 2092u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_COPYRIGHT_INFO_STR: &'static str = "Copyright Info";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_DIGITAL_ZOOM: u32 = 2078u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_DIGITAL_ZOOM_STR: &'static str = "Digital Zoom";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_DIMENSION: u32 = 2070u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_DIMENSION_STR: &'static str = "Dimension";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_EFFECT_MODE: u32 = 2077u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_EFFECT_MODE_STR: &'static str = "Effect Mode";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_EXPOSURE_COMP: u32 = 2053u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_EXPOSURE_COMP_STR: &'static str = "Exposure Compensation";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_EXPOSURE_INDEX: u32 = 2083u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_EXPOSURE_INDEX_STR: &'static str = "Exposure Index";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_EXPOSURE_METERING_MODE: u32 = 2084u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_EXPOSURE_METERING_MODE_STR: &'static str = "Exposure Metering Mode";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_EXPOSURE_MODE: u32 = 2052u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_EXPOSURE_MODE_STR: &'static str = "Exposure Mode";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_EXPOSURE_TIME: u32 = 2054u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_EXPOSURE_TIME_STR: &'static str = "Exposure Time";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_FLASH_MODE: u32 = 2056u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_FLASH_MODE_STR: &'static str = "Flash Mode";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_FNUMBER: u32 = 2055u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_FNUMBER_STR: &'static str = "F Number";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_FOCAL_LENGTH: u32 = 2087u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_FOCAL_LENGTH_STR: &'static str = "Focus Length";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_FOCUS_DISTANCE: u32 = 2086u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_FOCUS_DISTANCE_STR: &'static str = "Focus Distance";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_FOCUS_MANUAL_DIST: u32 = 2058u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_FOCUS_MANUAL_DIST_STR: &'static str = "Focus Manual Dist";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_FOCUS_METERING: u32 = 2072u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_FOCUS_METERING_MODE: u32 = 2085u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_FOCUS_METERING_MODE_STR: &'static str = "Focus Metering Mode";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_FOCUS_METERING_STR: &'static str = "Focus Metering Mode";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_FOCUS_MODE: u32 = 2057u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_FOCUS_MODE_STR: &'static str = "Focus Mode";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_PAN_POSITION: u32 = 2060u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_PAN_POSITION_STR: &'static str = "Pan Position";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_PICTURES_REMAINING: u32 = 2051u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_PICTURES_REMAINING_STR: &'static str = "Pictures Remaining";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_PICTURES_TAKEN: u32 = 2050u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_PICTURES_TAKEN_STR: &'static str = "Pictures Taken";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_PICT_HEIGHT: u32 = 2069u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_PICT_HEIGHT_STR: &'static str = "Picture Height";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_PICT_WIDTH: u32 = 2068u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_PICT_WIDTH_STR: &'static str = "Picture Width";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_POWER_MODE: u32 = 2064u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_POWER_MODE_STR: &'static str = "Power Mode";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_RGB_GAIN: u32 = 2088u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_RGB_GAIN_STR: &'static str = "RGB Gain";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_SHARPNESS: u32 = 2079u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_SHARPNESS_STR: &'static str = "Sharpness";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_THUMB_HEIGHT: u32 = 2067u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_THUMB_HEIGHT_STR: &'static str = "Thumbnail Height";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_THUMB_WIDTH: u32 = 2066u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_THUMB_WIDTH_STR: &'static str = "Thumbnail Width";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_TILT_POSITION: u32 = 2061u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_TILT_POSITION_STR: &'static str = "Tilt Position";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_TIMELAPSE_INTERVAL: u32 = 2073u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_TIMELAPSE_INTERVAL_STR: &'static str = "Timelapse Interval";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_TIMELAPSE_NUMBER: u32 = 2074u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_TIMELAPSE_NUMBER_STR: &'static str = "Timelapse Number";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_TIMER_MODE: u32 = 2062u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_TIMER_MODE_STR: &'static str = "Timer Mode";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_TIMER_VALUE: u32 = 2063u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_TIMER_VALUE_STR: &'static str = "Timer Value";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_UPLOAD_URL: u32 = 2090u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_UPLOAD_URL_STR: &'static str = "Upload URL";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_WHITE_BALANCE: u32 = 2089u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_WHITE_BALANCE_STR: &'static str = "White Balance";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_ZOOM_POSITION: u32 = 2059u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPC_ZOOM_POSITION_STR: &'static str = "Zoom Position";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPF_FIRST: u32 = 3330u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPF_MOUNT_POINT: u32 = 3330u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPF_MOUNT_POINT_STR: &'static str = "Directory mount point";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_DEVICE_ID: u32 = 3114u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_DEVICE_ID_STR: &'static str = "Device ID";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_DITHER_PATTERN_DATA: u32 = 3085u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_DITHER_PATTERN_DATA_STR: &'static str = "Dither Pattern Data";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_DITHER_SELECT: u32 = 3084u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_DITHER_SELECT_STR: &'static str = "Dither Select";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_DOCUMENT_HANDLING_CAPABILITIES: u32 = 3086u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_DOCUMENT_HANDLING_CAPABILITIES_STR: &'static str = "Document Handling Capabilities";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_DOCUMENT_HANDLING_CAPACITY: u32 = 3089u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_DOCUMENT_HANDLING_CAPACITY_STR: &'static str = "Document Handling Capacity";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_DOCUMENT_HANDLING_SELECT: u32 = 3088u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_DOCUMENT_HANDLING_SELECT_STR: &'static str = "Document Handling Select";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_DOCUMENT_HANDLING_STATUS: u32 = 3087u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_DOCUMENT_HANDLING_STATUS_STR: &'static str = "Document Handling Status";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_ENDORSER_CHARACTERS: u32 = 3092u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_ENDORSER_CHARACTERS_STR: &'static str = "Endorser Characters";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_ENDORSER_STRING: u32 = 3093u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_ENDORSER_STRING_STR: &'static str = "Endorser String";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_FILTER_SELECT: u32 = 3083u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_FILTER_SELECT_STR: &'static str = "Filter Select";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_FIRST: u32 = 3074u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_GLOBAL_IDENTITY: u32 = 3115u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_GLOBAL_IDENTITY_STR: &'static str = "Global Identity";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_HORIZONTAL_BED_REGISTRATION: u32 = 3079u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_HORIZONTAL_BED_REGISTRATION_STR: &'static str = "Horizontal Bed Registration";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_HORIZONTAL_BED_SIZE: u32 = 3074u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_HORIZONTAL_BED_SIZE_STR: &'static str = "Horizontal Bed Size";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_HORIZONTAL_SHEET_FEED_SIZE: u32 = 3076u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_HORIZONTAL_SHEET_FEED_SIZE_STR: &'static str = "Horizontal Sheet Feed Size";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_MAX_SCAN_TIME: u32 = 3095u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_MAX_SCAN_TIME_STR: &'static str = "Max Scan Time";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_MIN_HORIZONTAL_SHEET_FEED_SIZE: u32 = 3104u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_MIN_HORIZONTAL_SHEET_FEED_SIZE_STR: &'static str = "Minimum Horizontal Sheet Feed Size";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_MIN_VERTICAL_SHEET_FEED_SIZE: u32 = 3105u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_MIN_VERTICAL_SHEET_FEED_SIZE_STR: &'static str = "Minimum Vertical Sheet Feed Size";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_OPTICAL_XRES: u32 = 3090u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_OPTICAL_XRES_STR: &'static str = "Horizontal Optical Resolution";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_OPTICAL_YRES: u32 = 3091u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_OPTICAL_YRES_STR: &'static str = "Vertical Optical Resolution";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_PAD_COLOR: u32 = 3082u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_PAD_COLOR_STR: &'static str = "Pad Color";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_PAGES: u32 = 3096u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_PAGES_STR: &'static str = "Pages";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_PAGE_HEIGHT: u32 = 3099u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_PAGE_HEIGHT_STR: &'static str = "Page Height";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_PAGE_SIZE: u32 = 3097u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_PAGE_SIZE_STR: &'static str = "Page Size";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_PAGE_WIDTH: u32 = 3098u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_PAGE_WIDTH_STR: &'static str = "Page Width";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_PLATEN_COLOR: u32 = 3081u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_PLATEN_COLOR_STR: &'static str = "Platen Color";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_PREVIEW: u32 = 3100u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_PREVIEW_STR: &'static str = "Preview";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_SCAN_AHEAD_PAGES: u32 = 3094u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_SCAN_AHEAD_PAGES_STR: &'static str = "Scan Ahead Pages";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_SCAN_AVAILABLE_ITEM: u32 = 3116u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_SCAN_AVAILABLE_ITEM_STR: &'static str = "Scan Available Item";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_SERVICE_ID: u32 = 3113u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_SERVICE_ID_STR: &'static str = "Service ID";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_SHEET_FEEDER_REGISTRATION: u32 = 3078u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_SHEET_FEEDER_REGISTRATION_STR: &'static str = "Sheet Feeder Registration";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_SHOW_PREVIEW_CONTROL: u32 = 3103u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_SHOW_PREVIEW_CONTROL_STR: &'static str = "Show preview control";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_TRANSPARENCY: u32 = 3101u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_TRANSPARENCY_CAPABILITIES: u32 = 3106u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_TRANSPARENCY_CAPABILITIES_STR: &'static str = "Transparency Adapter Capabilities";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_TRANSPARENCY_SELECT: u32 = 3102u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_TRANSPARENCY_SELECT_STR: &'static str = "Transparency Adapter Select";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_TRANSPARENCY_STATUS: u32 = 3107u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_TRANSPARENCY_STATUS_STR: &'static str = "Transparency Adapter Status";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_TRANSPARENCY_STR: &'static str = "Transparency Adapter";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_USER_NAME: u32 = 3112u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_USER_NAME_STR: &'static str = "User Name";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_VERTICAL_BED_REGISTRATION: u32 = 3080u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_VERTICAL_BED_REGISTRATION_STR: &'static str = "Vertical Bed Registration";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_VERTICAL_BED_SIZE: u32 = 3075u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_VERTICAL_BED_SIZE_STR: &'static str = "Vertical Bed Size";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_VERTICAL_SHEET_FEED_SIZE: u32 = 3077u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPS_VERTICAL_SHEET_FEED_SIZE_STR: &'static str = "Vertical Sheet Feed Size";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPV_DSHOW_DEVICE_PATH: u32 = 3588u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPV_DSHOW_DEVICE_PATH_STR: &'static str = "Directshow Device Path";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPV_IMAGES_DIRECTORY: u32 = 3587u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPV_IMAGES_DIRECTORY_STR: &'static str = "Images Directory";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPV_LAST_PICTURE_TAKEN: u32 = 3586u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_DPV_LAST_PICTURE_TAKEN_STR: &'static str = "Last Picture Taken";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ENDORSER_TOK_DATE: &'static str = "$DATE$";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ENDORSER_TOK_DAY: &'static str = "$DAY$";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ENDORSER_TOK_MONTH: &'static str = "$MONTH$";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ENDORSER_TOK_PAGE_COUNT: &'static str = "$PAGE_COUNT$";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ENDORSER_TOK_TIME: &'static str = "$TIME$";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ENDORSER_TOK_YEAR: &'static str = "$YEAR$";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_BUSY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145320954i32);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_COVER_OPEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145320944i32);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_DESTINATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145320942i32);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_DEVICE_COMMUNICATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145320950i32);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_DEVICE_LOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145320947i32);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_EXCEPTION_IN_DRIVER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145320946i32);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_GENERAL_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145320959i32);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_INCORRECT_HARDWARE_SETTING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145320948i32);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_INVALID_COMMAND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145320949i32);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_INVALID_DRIVER_RESPONSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145320945i32);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_ITEM_DELETED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145320951i32);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_LAMP_OFF: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145320943i32);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_MAXIMUM_PRINTER_ENDORSER_COUNTER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145320939i32);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_MULTI_FEED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145320940i32);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_NETWORK_RESERVATION_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145320941i32);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_OFFLINE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145320955i32);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_PAPER_EMPTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145320957i32);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_PAPER_JAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145320958i32);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_PAPER_PROBLEM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145320956i32);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_USER_INTERVENTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145320952i32);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ERROR_WARMING_UP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145320953i32);
pub const WIA_EVENT_CANCEL_IO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc860f7b8_9ccd_41ea_bbbf_4dd09c5b1795);
pub const WIA_EVENT_COVER_CLOSED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6714a1e6_e285_468c_9b8c_da7dc4cbaa05);
pub const WIA_EVENT_COVER_OPEN: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x19a12136_fa1c_4f66_900f_8f914ec74ec9);
pub const WIA_EVENT_DEVICE_CONNECTED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa28bbade_64b6_11d2_a231_00c04fa31809);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_EVENT_DEVICE_CONNECTED_STR: &'static str = "Device Connected";
pub const WIA_EVENT_DEVICE_DISCONNECTED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x143e4e83_6497_11d2_a231_00c04fa31809);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_EVENT_DEVICE_DISCONNECTED_STR: &'static str = "Device Disconnected";
pub const WIA_EVENT_DEVICE_NOT_READY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8962d7e_e4dc_4b4d_ba29_668a87f42e6f);
pub const WIA_EVENT_DEVICE_READY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7523ec6c_988b_419e_9a0a_425ac31b37dc);
pub const WIA_EVENT_FEEDER_EMPTIED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe70b4b82_6dda_46bb_8ff9_53ceb1a03e35);
pub const WIA_EVENT_FEEDER_LOADED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc8d701e_9aba_481d_bf74_78f763dc342a);
pub const WIA_EVENT_FLATBED_LID_CLOSED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf879af0f_9b29_4283_ad95_d412164d39a9);
pub const WIA_EVENT_FLATBED_LID_OPEN: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba0a0623_437d_4f03_a97d_7793b123113c);
pub const WIA_EVENT_HANDLER_NO_ACTION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0372b7d_e115_4525_bc55_b629e68c745a);
pub const WIA_EVENT_HANDLER_PROMPT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f4baad0_4d59_4fcd_b213_783ce7a92f22);
pub const WIA_EVENT_ITEM_CREATED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c8f4ef5_e14f_11d2_b326_00c04f68ce61);
pub const WIA_EVENT_ITEM_DELETED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d22a559_e14f_11d2_b326_00c04f68ce61);
pub const WIA_EVENT_POWER_RESUME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x618f153e_f686_4350_9634_4115a304830c);
pub const WIA_EVENT_POWER_SUSPEND: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0922ff9_c3b4_411c_9e29_03a66993d2be);
pub const WIA_EVENT_SCAN_EMAIL_IMAGE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc686dcee_54f2_419e_9a27_2fc7f2e98f9e);
pub const WIA_EVENT_SCAN_FAX_IMAGE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc00eb793_8c6e_11d2_977a_0000f87a926f);
pub const WIA_EVENT_SCAN_FILM_IMAGE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b2b662c_6185_438c_b68b_e39ee25e71cb);
pub const WIA_EVENT_SCAN_IMAGE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6c5a715_8c6e_11d2_977a_0000f87a926f);
pub const WIA_EVENT_SCAN_IMAGE2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc4767c1_c8b3_48a2_9cfa_2e90cb3d3590);
pub const WIA_EVENT_SCAN_IMAGE3: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x154e27be_b617_4653_acc5_0fd7bd4c65ce);
pub const WIA_EVENT_SCAN_IMAGE4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa65b704a_7f3c_4447_a75d_8a26dfca1fdf);
pub const WIA_EVENT_SCAN_OCR_IMAGE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d095b89_37d6_4877_afed_62a297dc6dbe);
pub const WIA_EVENT_SCAN_PRINT_IMAGE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb441f425_8c6e_11d2_977a_0000f87a926f);
pub const WIA_EVENT_STI_PROXY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd711f81f_1f0d_422d_8641_927d1b93e5e5);
pub const WIA_EVENT_STORAGE_CREATED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x353308b2_fe73_46c8_895e_fa4551ccc85a);
pub const WIA_EVENT_STORAGE_DELETED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e41e75e_9390_44c5_9a51_e47019e390cf);
pub const WIA_EVENT_TREE_UPDATED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9859b91_4ab2_4cd6_a1fc_582eec55e585);
pub const WIA_EVENT_VOLUME_INSERT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9638bbfd_d1bd_11d2_b31f_00c04f68ce61);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct WIA_EXTENDED_TRANSFER_INFO {
    pub ulSize: u32,
    pub ulMinBufferSize: u32,
    pub ulOptimalBufferSize: u32,
    pub ulMaxBufferSize: u32,
    pub ulNumBuffers: u32,
}
impl ::core::marker::Copy for WIA_EXTENDED_TRANSFER_INFO {}
impl ::core::clone::Clone for WIA_EXTENDED_TRANSFER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIA_EXTENDED_TRANSFER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_EXTENDED_TRANSFER_INFO").field("ulSize", &self.ulSize).field("ulMinBufferSize", &self.ulMinBufferSize).field("ulOptimalBufferSize", &self.ulOptimalBufferSize).field("ulMaxBufferSize", &self.ulMaxBufferSize).field("ulNumBuffers", &self.ulNumBuffers).finish()
    }
}
unsafe impl ::windows::core::Abi for WIA_EXTENDED_TRANSFER_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIA_EXTENDED_TRANSFER_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_EXTENDED_TRANSFER_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIA_EXTENDED_TRANSFER_INFO {}
impl ::core::default::Default for WIA_EXTENDED_TRANSFER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_FEEDER_CONTROL_AUTO: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_FEEDER_CONTROL_MANUAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_FILM_BW_NEGATIVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_FILM_COLOR_NEGATIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_FILM_COLOR_SLIDE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_FINAL_SCAN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_FLAG_NOM: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_FLAG_NUM_ELEMS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_FLAG_VALUES: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct WIA_FORMAT_INFO {
    pub guidFormatID: ::windows::core::GUID,
    pub lTymed: i32,
}
impl ::core::marker::Copy for WIA_FORMAT_INFO {}
impl ::core::clone::Clone for WIA_FORMAT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIA_FORMAT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_FORMAT_INFO").field("guidFormatID", &self.guidFormatID).field("lTymed", &self.lTymed).finish()
    }
}
unsafe impl ::windows::core::Abi for WIA_FORMAT_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIA_FORMAT_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_FORMAT_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIA_FORMAT_INFO {}
impl ::core::default::Default for WIA_FORMAT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IMAGEPROC_FILTER_STR: &'static str = "ImageProcessingFilter";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_INTENT_BEST_PREVIEW: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_INTENT_IMAGE_TYPE_COLOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_INTENT_IMAGE_TYPE_GRAYSCALE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_INTENT_IMAGE_TYPE_MASK: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_INTENT_IMAGE_TYPE_TEXT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_INTENT_MAXIMIZE_QUALITY: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_INTENT_MINIMIZE_SIZE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_INTENT_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_INTENT_SIZE_MASK: u32 = 983040u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_ACCESS_RIGHTS: u32 = 4102u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_ACCESS_RIGHTS_STR: &'static str = "Access Rights";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_APP_COLOR_MAPPING: u32 = 4121u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_APP_COLOR_MAPPING_STR: &'static str = "Application Applies Color Mapping";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_BITS_PER_CHANNEL: u32 = 4110u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_BITS_PER_CHANNEL_STR: &'static str = "Bits Per Channel";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_BUFFER_SIZE: u32 = 4118u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_BUFFER_SIZE_STR: &'static str = "Buffer Size";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_BYTES_PER_LINE: u32 = 4113u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_BYTES_PER_LINE_STR: &'static str = "Bytes Per Line";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_CHANNELS_PER_PIXEL: u32 = 4109u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_CHANNELS_PER_PIXEL_STR: &'static str = "Channels Per Pixel";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_COLOR_PROFILE: u32 = 4117u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_COLOR_PROFILE_STR: &'static str = "Color Profiles";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_COMPRESSION: u32 = 4107u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_COMPRESSION_STR: &'static str = "Compression";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_DATATYPE: u32 = 4103u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_DATATYPE_STR: &'static str = "Data Type";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_DEPTH: u32 = 4104u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_DEPTH_STR: &'static str = "Bits Per Pixel";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_FILENAME_EXTENSION: u32 = 4123u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_FILENAME_EXTENSION_STR: &'static str = "Filename extension";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_FIRST: u32 = 4098u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_FORMAT: u32 = 4106u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_FORMAT_STR: &'static str = "Format";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_FULL_ITEM_NAME: u32 = 4099u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_FULL_ITEM_NAME_STR: &'static str = "Full Item Name";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_GAMMA_CURVES: u32 = 4115u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_GAMMA_CURVES_STR: &'static str = "Gamma Curves";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_ICM_PROFILE_NAME: u32 = 4120u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_ICM_PROFILE_NAME_STR: &'static str = "Color Profile Name";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_ITEMS_STORED: u32 = 4127u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_ITEMS_STORED_STR: &'static str = "Items Stored";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_ITEM_CATEGORY: u32 = 4125u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_ITEM_CATEGORY_STR: &'static str = "Item Category";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_ITEM_FLAGS: u32 = 4101u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_ITEM_FLAGS_STR: &'static str = "Item Flags";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_ITEM_NAME: u32 = 4098u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_ITEM_NAME_STR: &'static str = "Item Name";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_ITEM_SIZE: u32 = 4116u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_ITEM_SIZE_STR: &'static str = "Item Size";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_ITEM_TIME: u32 = 4100u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_ITEM_TIME_STR: &'static str = "Item Time Stamp";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_MIN_BUFFER_SIZE: u32 = 4118u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_MIN_BUFFER_SIZE_STR: &'static str = "Buffer Size";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_NUMBER_OF_LINES: u32 = 4114u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_NUMBER_OF_LINES_STR: &'static str = "Number of Lines";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_PIXELS_PER_LINE: u32 = 4112u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_PIXELS_PER_LINE_STR: &'static str = "Pixels Per Line";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_PLANAR: u32 = 4111u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_PLANAR_STR: &'static str = "Planar";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_PREFERRED_FORMAT: u32 = 4105u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_PREFERRED_FORMAT_STR: &'static str = "Preferred Format";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_PROP_STREAM_COMPAT_ID: u32 = 4122u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_PROP_STREAM_COMPAT_ID_STR: &'static str = "Stream Compatibility ID";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_RAW_BITS_PER_CHANNEL: u32 = 4128u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_RAW_BITS_PER_CHANNEL_STR: &'static str = "Raw Bits Per Channel";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_REGION_TYPE: u32 = 4119u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_REGION_TYPE_STR: &'static str = "Region Type";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_SUPPRESS_PROPERTY_PAGE: u32 = 4124u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_SUPPRESS_PROPERTY_PAGE_STR: &'static str = "Suppress a property page";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_TYMED: u32 = 4108u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_TYMED_STR: &'static str = "Media Type";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_UPLOAD_ITEM_SIZE: u32 = 4126u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPA_UPLOAD_ITEM_SIZE_STR: &'static str = "Upload Item Size";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_AUDIO_AVAILABLE: u32 = 5125u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_AUDIO_AVAILABLE_STR: &'static str = "Audio Available";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_AUDIO_DATA: u32 = 5127u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_AUDIO_DATA_FORMAT: u32 = 5126u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_AUDIO_DATA_FORMAT_STR: &'static str = "Audio Format";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_AUDIO_DATA_STR: &'static str = "Audio Data";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_FIRST: u32 = 5122u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_NUM_PICT_PER_ROW: u32 = 5128u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_NUM_PICT_PER_ROW_STR: &'static str = "Pictures per Row";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_SEQUENCE: u32 = 5129u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_SEQUENCE_STR: &'static str = "Sequence Number";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_THUMBNAIL: u32 = 5122u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_THUMBNAIL_STR: &'static str = "Thumbnail Data";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_THUMB_HEIGHT: u32 = 5124u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_THUMB_HEIGHT_STR: &'static str = "Thumbnail Height";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_THUMB_WIDTH: u32 = 5123u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_THUMB_WIDTH_STR: &'static str = "Thumbnail Width";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_TIMEDELAY: u32 = 5130u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPC_TIMEDELAY_STR: &'static str = "Time Delay";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_ALARM: u32 = 4185u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_ALARM_STR: &'static str = "Alarm";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_AUTO_CROP: u32 = 4170u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_AUTO_CROP_STR: &'static str = "Auto-Crop";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_AUTO_DESKEW: u32 = 3107u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_AUTO_DESKEW_STR: &'static str = "Automatic Deskew";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_BARCODE_READER: u32 = 4150u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_BARCODE_READER_STR: &'static str = "Barcode Reader";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_BARCODE_SEARCH_DIRECTION: u32 = 4152u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_BARCODE_SEARCH_DIRECTION_STR: &'static str = "Barcode Search Direction";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_BARCODE_SEARCH_TIMEOUT: u32 = 4154u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_BARCODE_SEARCH_TIMEOUT_STR: &'static str = "Barcode Search Timeout";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_BLANK_PAGES: u32 = 4167u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_BLANK_PAGES_SENSITIVITY: u32 = 4192u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_BLANK_PAGES_SENSITIVITY_STR: &'static str = "Blank Pages Sensitivity";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_BLANK_PAGES_STR: &'static str = "Blank Pages";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_BRIGHTNESS: u32 = 6154u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_BRIGHTNESS_STR: &'static str = "Brightness";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_COLOR_DROP: u32 = 4176u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_COLOR_DROP_BLUE: u32 = 4179u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_COLOR_DROP_BLUE_STR: &'static str = "Color Drop Blue";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_COLOR_DROP_GREEN: u32 = 4178u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_COLOR_DROP_GREEN_STR: &'static str = "Color Drop Green";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_COLOR_DROP_MULTI: u32 = 4191u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_COLOR_DROP_MULTI_STR: &'static str = "Color Drop Multiple";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_COLOR_DROP_RED: u32 = 4177u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_COLOR_DROP_RED_STR: &'static str = "Color Drop Red";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_COLOR_DROP_STR: &'static str = "Color Drop";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_CONTRAST: u32 = 6155u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_CONTRAST_STR: &'static str = "Contrast";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_CUR_INTENT: u32 = 6146u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_CUR_INTENT_STR: &'static str = "Current Intent";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_DESKEW_X: u32 = 6162u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_DESKEW_X_STR: &'static str = "DeskewX";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_DESKEW_Y: u32 = 6163u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_DESKEW_Y_STR: &'static str = "DeskewY";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_DOCUMENT_HANDLING_SELECT: u32 = 3088u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_DOCUMENT_HANDLING_SELECT_STR: &'static str = "Document Handling Select";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_ENABLED_BARCODE_TYPES: u32 = 4156u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_ENABLED_BARCODE_TYPES_STR: &'static str = "Enabled Barcode Types";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_ENABLED_PATCH_CODE_TYPES: u32 = 4163u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_ENABLED_PATCH_CODE_TYPES_STR: &'static str = "Enabled Path Code Types";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_FEEDER_CONTROL: u32 = 4182u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_FEEDER_CONTROL_STR: &'static str = "Feeder Control";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_FILM_NODE_NAME: u32 = 4129u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_FILM_NODE_NAME_STR: &'static str = "Film Node Name";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_FILM_SCAN_MODE: u32 = 3104u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_FILM_SCAN_MODE_STR: &'static str = "Film Scan Mode";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_FIRST: u32 = 6146u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_INVERT: u32 = 6160u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_INVERT_STR: &'static str = "Invert";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_JOB_SEPARATORS: u32 = 4165u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_JOB_SEPARATORS_STR: &'static str = "Job Separators";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_LAMP: u32 = 3105u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_LAMP_AUTO_OFF: u32 = 3106u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_LAMP_AUTO_OFF_STR: &'static str = "Lamp Auto Off";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_LAMP_STR: &'static str = "Lamp";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_LONG_DOCUMENT: u32 = 4166u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_LONG_DOCUMENT_STR: &'static str = "Long Document";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MAXIMUM_BARCODES_PER_PAGE: u32 = 4151u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MAXIMUM_BARCODES_PER_PAGE_STR: &'static str = "Maximum Barcodes Per Page";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MAXIMUM_BARCODE_SEARCH_RETRIES: u32 = 4153u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MAXIMUM_BARCODE_SEARCH_RETRIES_STR: &'static str = "Barcode Search Retries";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MAX_HORIZONTAL_SIZE: u32 = 6165u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MAX_HORIZONTAL_SIZE_STR: &'static str = "Maximum Horizontal Scan Size";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MAX_VERTICAL_SIZE: u32 = 6166u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MAX_VERTICAL_SIZE_STR: &'static str = "Maximum Vertical Scan Size";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MICR_READER: u32 = 4164u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MICR_READER_STR: &'static str = "MICR Reader";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MIN_HORIZONTAL_SIZE: u32 = 6167u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MIN_HORIZONTAL_SIZE_STR: &'static str = "Minimum Horizontal Scan Size";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MIN_VERTICAL_SIZE: u32 = 6168u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MIN_VERTICAL_SIZE_STR: &'static str = "Minimum Vertical Scan Size";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MIRROR: u32 = 6158u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MIRROR_STR: &'static str = "Mirror";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MULTI_FEED: u32 = 4168u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MULTI_FEED_DETECT_METHOD: u32 = 4193u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MULTI_FEED_DETECT_METHOD_STR: &'static str = "Multi-Feed Detection Method";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MULTI_FEED_SENSITIVITY: u32 = 4169u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MULTI_FEED_SENSITIVITY_STR: &'static str = "Multi-Feed Sensitivity";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_MULTI_FEED_STR: &'static str = "Multi-Feed";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_OPTICAL_XRES: u32 = 3090u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_OPTICAL_XRES_STR: &'static str = "Horizontal Optical Resolution";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_OPTICAL_YRES: u32 = 3091u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_OPTICAL_YRES_STR: &'static str = "Vertical Optical Resolution";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_ORIENTATION: u32 = 6156u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_ORIENTATION_STR: &'static str = "Orientation";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_OVER_SCAN: u32 = 4171u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_OVER_SCAN_BOTTOM: u32 = 4175u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_OVER_SCAN_BOTTOM_STR: &'static str = "Overscan Bottom";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_OVER_SCAN_LEFT: u32 = 4172u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_OVER_SCAN_LEFT_STR: &'static str = "Overscan Left";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_OVER_SCAN_RIGHT: u32 = 4173u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_OVER_SCAN_RIGHT_STR: &'static str = "Overscan Right";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_OVER_SCAN_STR: &'static str = "Overscan";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_OVER_SCAN_TOP: u32 = 4174u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_OVER_SCAN_TOP_STR: &'static str = "Overscan Top";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PAGES: u32 = 3096u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PAGES_STR: &'static str = "Pages";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PAGE_HEIGHT: u32 = 3099u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PAGE_HEIGHT_STR: &'static str = "Page Height";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PAGE_SIZE: u32 = 3097u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PAGE_SIZE_STR: &'static str = "Page Size";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PAGE_WIDTH: u32 = 3098u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PAGE_WIDTH_STR: &'static str = "Page Width";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PATCH_CODE_READER: u32 = 4157u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PATCH_CODE_READER_STR: &'static str = "Patch Code Reader";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PHOTOMETRIC_INTERP: u32 = 6153u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PHOTOMETRIC_INTERP_STR: &'static str = "Photometric Interpretation";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PREVIEW: u32 = 3100u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PREVIEW_STR: &'static str = "Preview";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PREVIEW_TYPE: u32 = 3111u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PREVIEW_TYPE_STR: &'static str = "Preview Type";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER: u32 = 4130u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_CHARACTER_ROTATION: u32 = 4187u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_CHARACTER_ROTATION_STR: &'static str = "Printer/Endorser Character Rotation";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_COUNTER: u32 = 4132u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_COUNTER_DIGITS: u32 = 4190u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_COUNTER_DIGITS_STR: &'static str = "Printer/Endorser Counter Digits";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_COUNTER_STR: &'static str = "Printer/Endorser Counter";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_FONT_TYPE: u32 = 4184u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_FONT_TYPE_STR: &'static str = "Printer/Endorser Font Type";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS: u32 = 4142u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_DOWNLOAD: u32 = 4149u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_DOWNLOAD_STR: &'static str = "Printer/Endorser Graphics Download";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MAX_HEIGHT: u32 = 4147u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MAX_HEIGHT_STR: &'static str = "Printer/Endorser Graphics Maximum Height";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MAX_WIDTH: u32 = 4145u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MAX_WIDTH_STR: &'static str = "Printer/Endorser Graphics Maximum Width";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MIN_HEIGHT: u32 = 4146u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MIN_HEIGHT_STR: &'static str = "Printer/Endorser Graphics Minimum Height";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MIN_WIDTH: u32 = 4144u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MIN_WIDTH_STR: &'static str = "Printer/Endorser Graphics Minimum Width";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_POSITION: u32 = 4143u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_POSITION_STR: &'static str = "Printer/Endorser Graphics Position";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_STR: &'static str = "Printer/Endorser Graphics";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_UPLOAD: u32 = 4148u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_UPLOAD_STR: &'static str = "Printer/Endorser Graphics Upload";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_INK: u32 = 4186u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_INK_STR: &'static str = "Printer/Endorser Ink";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_MAX_CHARACTERS: u32 = 4188u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_MAX_CHARACTERS_STR: &'static str = "Printer/Endorser Maximum Characters";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_MAX_GRAPHICS: u32 = 4189u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_MAX_GRAPHICS_STR: &'static str = "Printer/Endorser Maximum Graphics";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_NUM_LINES: u32 = 4136u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_NUM_LINES_STR: &'static str = "Printer/Endorser Lines";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_ORDER: u32 = 4131u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_ORDER_STR: &'static str = "Printer/Endorser Order";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_PADDING: u32 = 4183u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_PADDING_STR: &'static str = "Printer/Endorser Padding";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_STEP: u32 = 4133u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_STEP_STR: &'static str = "Printer/Endorser Step";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_STR: &'static str = "Printer/Endorser";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_STRING: u32 = 4137u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_STRING_STR: &'static str = "Printer/Endorser String";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_TEXT_DOWNLOAD: u32 = 4141u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_TEXT_DOWNLOAD_STR: &'static str = "Printer/Endorser Text Download";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_TEXT_UPLOAD: u32 = 4140u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_TEXT_UPLOAD_STR: &'static str = "Printer/Endorser Text Upload";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_VALID_CHARACTERS: u32 = 4138u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_VALID_CHARACTERS_STR: &'static str = "Printer/Endorser Valid Characters";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_VALID_FORMAT_SPECIFIERS: u32 = 4139u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_VALID_FORMAT_SPECIFIERS_STR: &'static str = "Printer/Endorser Valid Format Specifiers";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_XOFFSET: u32 = 4134u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_XOFFSET_STR: &'static str = "Printer/Endorser Horizontal Offset";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_YOFFSET: u32 = 4135u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_PRINTER_ENDORSER_YOFFSET_STR: &'static str = "Printer/Endorser Vertical Offset";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_ROTATION: u32 = 6157u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_ROTATION_STR: &'static str = "Rotation";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_SCAN_AHEAD: u32 = 4180u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_SCAN_AHEAD_CAPACITY: u32 = 4181u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_SCAN_AHEAD_CAPACITY_STR: &'static str = "Scan Ahead Capacity";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_SCAN_AHEAD_STR: &'static str = "Scan Ahead";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_SEGMENTATION: u32 = 6164u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_SEGMENTATION_STR: &'static str = "Segmentation";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_SHEET_FEEDER_REGISTRATION: u32 = 3078u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_SHEET_FEEDER_REGISTRATION_STR: &'static str = "Sheet Feeder Registration";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_SHOW_PREVIEW_CONTROL: u32 = 3103u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_SHOW_PREVIEW_CONTROL_STR: &'static str = "Show preview control";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_SUPPORTED_BARCODE_TYPES: u32 = 4155u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_SUPPORTED_BARCODE_TYPES_STR: &'static str = "Supported Barcode Types";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_SUPPORTED_PATCH_CODE_TYPES: u32 = 4162u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_SUPPORTED_PATCH_CODE_TYPES_STR: &'static str = "Supported Patch Code Types";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_SUPPORTS_CHILD_ITEM_CREATION: u32 = 3108u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_SUPPORTS_CHILD_ITEM_CREATION_STR: &'static str = "Supports Child Item Creation";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_THRESHOLD: u32 = 6159u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_THRESHOLD_STR: &'static str = "Threshold";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_TRANSFER_CAPABILITIES: u32 = 6169u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_TRANSFER_CAPABILITIES_STR: &'static str = "Transfer Capabilities";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_WARM_UP_TIME: u32 = 6161u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_WARM_UP_TIME_STR: &'static str = "Lamp Warm up Time";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_XEXTENT: u32 = 6151u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_XEXTENT_STR: &'static str = "Horizontal Extent";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_XPOS: u32 = 6149u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_XPOS_STR: &'static str = "Horizontal Start Position";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_XRES: u32 = 6147u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_XRES_STR: &'static str = "Horizontal Resolution";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_XSCALING: u32 = 3109u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_XSCALING_STR: &'static str = "Horizontal Scaling";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_YEXTENT: u32 = 6152u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_YEXTENT_STR: &'static str = "Vertical Extent";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_YPOS: u32 = 6150u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_YPOS_STR: &'static str = "Vertical Start Position";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_YRES: u32 = 6148u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_YRES_STR: &'static str = "Vertical Resolution";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_YSCALING: u32 = 3110u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IPS_YSCALING_STR: &'static str = "Vertical Scaling";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_IS_DEFAULT_HANDLER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ITEM_CAN_BE_DELETED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ITEM_READ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ITEM_WRITE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_LAMP_OFF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_LAMP_ON: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_LINE_ORDER_BOTTOM_TO_TOP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_LINE_ORDER_TOP_TO_BOTTOM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_LIST_COUNT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_LIST_NOM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_LIST_NUM_ELEMS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_LIST_VALUES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_LONG_DOCUMENT_DISABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_LONG_DOCUMENT_ENABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_LONG_DOCUMENT_SPLIT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_MAJOR_EVENT_DEVICE_CONNECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_MAJOR_EVENT_DEVICE_DISCONNECT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_MAJOR_EVENT_PICTURE_DELETED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_MAJOR_EVENT_PICTURE_TAKEN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_MAX_CTX_SIZE: u32 = 16777216u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct WIA_MICR {
    pub Tag: u32,
    pub Version: u32,
    pub Size: u32,
    pub Placeholder: u16,
    pub Reserved: u16,
    pub Count: u32,
    pub Micr: [WIA_MICR_INFO; 1],
}
impl ::core::marker::Copy for WIA_MICR {}
impl ::core::clone::Clone for WIA_MICR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIA_MICR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_MICR").field("Tag", &self.Tag).field("Version", &self.Version).field("Size", &self.Size).field("Placeholder", &self.Placeholder).field("Reserved", &self.Reserved).field("Count", &self.Count).field("Micr", &self.Micr).finish()
    }
}
unsafe impl ::windows::core::Abi for WIA_MICR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIA_MICR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_MICR>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIA_MICR {}
impl ::core::default::Default for WIA_MICR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct WIA_MICR_INFO {
    pub Size: u32,
    pub Page: u32,
    pub Length: u32,
    pub Text: [u16; 1],
}
impl ::core::marker::Copy for WIA_MICR_INFO {}
impl ::core::clone::Clone for WIA_MICR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIA_MICR_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_MICR_INFO").field("Size", &self.Size).field("Page", &self.Page).field("Length", &self.Length).field("Text", &self.Text).finish()
    }
}
unsafe impl ::windows::core::Abi for WIA_MICR_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIA_MICR_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_MICR_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIA_MICR_INFO {}
impl ::core::default::Default for WIA_MICR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_MICR_READER_AUTO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_MICR_READER_DISABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_MICR_READER_FEEDER_BACK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_MICR_READER_FEEDER_DUPLEX: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_MICR_READER_FEEDER_FRONT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_MICR_READER_FLATBED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_MULTI_FEED_DETECT_CONTINUE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_MULTI_FEED_DETECT_DISABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_MULTI_FEED_DETECT_METHOD_LENGTH: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_MULTI_FEED_DETECT_METHOD_OVERLAP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_MULTI_FEED_DETECT_STOP_ERROR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_MULTI_FEED_DETECT_STOP_SUCCESS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_NOTIFICATION_EVENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_NUM_DIP: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_NUM_IPC: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ORDER_BGR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_ORDER_RGB: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_OVER_SCAN_ALL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_OVER_SCAN_DISABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_OVER_SCAN_LEFT_RIGHT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_OVER_SCAN_TOP_BOTTOM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PACKED_PIXEL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_A4: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_AUTO: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_BUSINESSCARD: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_CUSTOM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_CUSTOM_BASE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_DIN_2B: u32 = 52u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_DIN_4B: u32 = 53u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_A0: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_A1: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_A10: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_A2: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_A3: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_A4: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_A5: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_A6: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_A7: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_A8: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_A9: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_B0: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_B1: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_B10: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_B2: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_B3: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_B4: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_B5: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_B6: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_B7: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_B8: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_B9: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_C0: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_C1: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_C10: u32 = 38u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_C2: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_C3: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_C4: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_C5: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_C6: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_C7: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_C8: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_ISO_C9: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_JIS_2A: u32 = 50u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_JIS_4A: u32 = 51u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_JIS_B0: u32 = 39u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_JIS_B1: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_JIS_B10: u32 = 49u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_JIS_B2: u32 = 41u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_JIS_B3: u32 = 42u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_JIS_B4: u32 = 43u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_JIS_B5: u32 = 44u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_JIS_B6: u32 = 45u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_JIS_B7: u32 = 46u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_JIS_B8: u32 = 47u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_JIS_B9: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_LETTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_USLEDGER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_USLEGAL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_USLETTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PAGE_USSTATEMENT: u32 = 5u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct WIA_PATCH_CODES {
    pub Tag: u32,
    pub Version: u32,
    pub Size: u32,
    pub Count: u32,
    pub PatchCodes: [WIA_PATCH_CODE_INFO; 1],
}
impl ::core::marker::Copy for WIA_PATCH_CODES {}
impl ::core::clone::Clone for WIA_PATCH_CODES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIA_PATCH_CODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_PATCH_CODES").field("Tag", &self.Tag).field("Version", &self.Version).field("Size", &self.Size).field("Count", &self.Count).field("PatchCodes", &self.PatchCodes).finish()
    }
}
unsafe impl ::windows::core::Abi for WIA_PATCH_CODES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIA_PATCH_CODES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_PATCH_CODES>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIA_PATCH_CODES {}
impl ::core::default::Default for WIA_PATCH_CODES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_10: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_11: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_12: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_13: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_14: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_3: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_4: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_6: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_7: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_8: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_9: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_CUSTOM_BASE: u32 = 32768u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct WIA_PATCH_CODE_INFO {
    pub Type: u32,
}
impl ::core::marker::Copy for WIA_PATCH_CODE_INFO {}
impl ::core::clone::Clone for WIA_PATCH_CODE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIA_PATCH_CODE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_PATCH_CODE_INFO").field("Type", &self.Type).finish()
    }
}
unsafe impl ::windows::core::Abi for WIA_PATCH_CODE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIA_PATCH_CODE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_PATCH_CODE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIA_PATCH_CODE_INFO {}
impl ::core::default::Default for WIA_PATCH_CODE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_READER_AUTO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_READER_DISABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_READER_FEEDER_BACK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_READER_FEEDER_DUPLEX: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_READER_FEEDER_FRONT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_READER_FLATBED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_T: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PATCH_CODE_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PHOTO_WHITE_0: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PHOTO_WHITE_1: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PLANAR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PREVIEW_SCAN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_AFTER_SCAN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_AUTO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_BEFORE_SCAN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_DIGITAL: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_DISABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_FEEDER_BACK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_FEEDER_DUPLEX: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_FEEDER_FRONT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_FLATBED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_GRAPHICS_BACKGROUND: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_GRAPHICS_BOTTOM: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_GRAPHICS_BOTTOM_LEFT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_GRAPHICS_BOTTOM_RIGHT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_GRAPHICS_DEVICE_DEFAULT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_GRAPHICS_LEFT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_GRAPHICS_RIGHT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_GRAPHICS_TOP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_GRAPHICS_TOP_LEFT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINTER_ENDORSER_GRAPHICS_TOP_RIGHT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_AM_PM: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_DATE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_DAY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_FONT_BOLD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_FONT_EXTRA_BOLD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_FONT_ITALIC: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_FONT_ITALIC_BOLD: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_FONT_ITALIC_EXTRA_BOLD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_FONT_LARGE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_FONT_LARGE_BOLD: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_FONT_LARGE_EXTRA_BOLD: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_FONT_LARGE_ITALIC: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_FONT_LARGE_ITALIC_BOLD: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_FONT_LARGE_ITALIC_EXTRA_BOLD: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_FONT_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_FONT_SMALL: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_FONT_SMALL_BOLD: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_FONT_SMALL_EXTRA_BOLD: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_FONT_SMALL_ITALIC: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_FONT_SMALL_ITALIC_BOLD: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_FONT_SMALL_ITALIC_EXTRA_BOLD: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_HOUR_12H: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_HOUR_24H: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_IMAGE: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_MILLISECOND: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_MINUTE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_MONTH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_MONTH_NAME: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_MONTH_SHORT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_PADDING_BLANK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_PADDING_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_PADDING_ZERO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_PAGE_COUNT: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_SECOND: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_TIME_12H: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_TIME_24H: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_WEEK_DAY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_WEEK_DAY_SHORT: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRINT_YEAR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRIVATE_DEVPROP: u32 = 38914u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PRIVATE_ITEMPROP: u32 = 71682u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WIA_PROPERTY_CONTEXT {
    pub cProps: u32,
    pub pProps: *mut u32,
    pub pChanged: *mut super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WIA_PROPERTY_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WIA_PROPERTY_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WIA_PROPERTY_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_PROPERTY_CONTEXT").field("cProps", &self.cProps).field("pProps", &self.pProps).field("pChanged", &self.pChanged).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WIA_PROPERTY_CONTEXT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WIA_PROPERTY_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_PROPERTY_CONTEXT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WIA_PROPERTY_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WIA_PROPERTY_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WIA_PROPERTY_INFO {
    pub lAccessFlags: u32,
    pub vt: u16,
    pub ValidVal: WIA_PROPERTY_INFO_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WIA_PROPERTY_INFO {
    fn clone(&self) -> Self {
        Self { lAccessFlags: self.lAccessFlags, vt: self.vt, ValidVal: self.ValidVal.clone() }
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WIA_PROPERTY_INFO {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WIA_PROPERTY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.lAccessFlags == other.lAccessFlags && self.vt == other.vt && self.ValidVal == other.ValidVal
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WIA_PROPERTY_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WIA_PROPERTY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union WIA_PROPERTY_INFO_0 {
    pub Range: WIA_PROPERTY_INFO_0_7,
    pub RangeFloat: WIA_PROPERTY_INFO_0_6,
    pub List: WIA_PROPERTY_INFO_0_4,
    pub ListFloat: WIA_PROPERTY_INFO_0_2,
    pub ListGuid: WIA_PROPERTY_INFO_0_3,
    pub ListBStr: ::core::mem::ManuallyDrop<WIA_PROPERTY_INFO_0_1>,
    pub Flag: WIA_PROPERTY_INFO_0_0,
    pub None: WIA_PROPERTY_INFO_0_5,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WIA_PROPERTY_INFO_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WIA_PROPERTY_INFO_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WIA_PROPERTY_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_PROPERTY_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WIA_PROPERTY_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WIA_PROPERTY_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WIA_PROPERTY_INFO_0_0 {
    pub Nom: i32,
    pub ValidBits: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WIA_PROPERTY_INFO_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WIA_PROPERTY_INFO_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WIA_PROPERTY_INFO_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_PROPERTY_INFO_0_0").field("Nom", &self.Nom).field("ValidBits", &self.ValidBits).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WIA_PROPERTY_INFO_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WIA_PROPERTY_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_PROPERTY_INFO_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WIA_PROPERTY_INFO_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WIA_PROPERTY_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WIA_PROPERTY_INFO_0_1 {
    pub cNumList: i32,
    pub Nom: super::super::Foundation::BSTR,
    pub pList: *mut super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WIA_PROPERTY_INFO_0_1 {
    fn clone(&self) -> Self {
        Self { cNumList: self.cNumList, Nom: self.Nom.clone(), pList: self.pList }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WIA_PROPERTY_INFO_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_PROPERTY_INFO_0_1").field("cNumList", &self.cNumList).field("Nom", &self.Nom).field("pList", &self.pList).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WIA_PROPERTY_INFO_0_1 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WIA_PROPERTY_INFO_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.cNumList == other.cNumList && self.Nom == other.Nom && self.pList == other.pList
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WIA_PROPERTY_INFO_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WIA_PROPERTY_INFO_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WIA_PROPERTY_INFO_0_2 {
    pub cNumList: i32,
    pub Nom: f64,
    pub pList: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WIA_PROPERTY_INFO_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WIA_PROPERTY_INFO_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WIA_PROPERTY_INFO_0_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_PROPERTY_INFO_0_2").field("cNumList", &self.cNumList).field("Nom", &self.Nom).field("pList", &self.pList).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WIA_PROPERTY_INFO_0_2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WIA_PROPERTY_INFO_0_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_PROPERTY_INFO_0_2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WIA_PROPERTY_INFO_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WIA_PROPERTY_INFO_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WIA_PROPERTY_INFO_0_3 {
    pub cNumList: i32,
    pub Nom: ::windows::core::GUID,
    pub pList: *mut ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WIA_PROPERTY_INFO_0_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WIA_PROPERTY_INFO_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WIA_PROPERTY_INFO_0_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_PROPERTY_INFO_0_3").field("cNumList", &self.cNumList).field("Nom", &self.Nom).field("pList", &self.pList).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WIA_PROPERTY_INFO_0_3 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WIA_PROPERTY_INFO_0_3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_PROPERTY_INFO_0_3>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WIA_PROPERTY_INFO_0_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WIA_PROPERTY_INFO_0_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WIA_PROPERTY_INFO_0_4 {
    pub cNumList: i32,
    pub Nom: i32,
    pub pList: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WIA_PROPERTY_INFO_0_4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WIA_PROPERTY_INFO_0_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WIA_PROPERTY_INFO_0_4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_PROPERTY_INFO_0_4").field("cNumList", &self.cNumList).field("Nom", &self.Nom).field("pList", &self.pList).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WIA_PROPERTY_INFO_0_4 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WIA_PROPERTY_INFO_0_4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_PROPERTY_INFO_0_4>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WIA_PROPERTY_INFO_0_4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WIA_PROPERTY_INFO_0_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WIA_PROPERTY_INFO_0_5 {
    pub Dummy: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WIA_PROPERTY_INFO_0_5 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WIA_PROPERTY_INFO_0_5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WIA_PROPERTY_INFO_0_5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_PROPERTY_INFO_0_5").field("Dummy", &self.Dummy).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WIA_PROPERTY_INFO_0_5 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WIA_PROPERTY_INFO_0_5 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_PROPERTY_INFO_0_5>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WIA_PROPERTY_INFO_0_5 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WIA_PROPERTY_INFO_0_5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WIA_PROPERTY_INFO_0_6 {
    pub Min: f64,
    pub Nom: f64,
    pub Max: f64,
    pub Inc: f64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WIA_PROPERTY_INFO_0_6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WIA_PROPERTY_INFO_0_6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WIA_PROPERTY_INFO_0_6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_PROPERTY_INFO_0_6").field("Min", &self.Min).field("Nom", &self.Nom).field("Max", &self.Max).field("Inc", &self.Inc).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WIA_PROPERTY_INFO_0_6 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WIA_PROPERTY_INFO_0_6 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_PROPERTY_INFO_0_6>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WIA_PROPERTY_INFO_0_6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WIA_PROPERTY_INFO_0_6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WIA_PROPERTY_INFO_0_7 {
    pub Min: i32,
    pub Nom: i32,
    pub Max: i32,
    pub Inc: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WIA_PROPERTY_INFO_0_7 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WIA_PROPERTY_INFO_0_7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WIA_PROPERTY_INFO_0_7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_PROPERTY_INFO_0_7").field("Min", &self.Min).field("Nom", &self.Nom).field("Max", &self.Max).field("Inc", &self.Inc).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WIA_PROPERTY_INFO_0_7 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WIA_PROPERTY_INFO_0_7 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_PROPERTY_INFO_0_7>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WIA_PROPERTY_INFO_0_7 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WIA_PROPERTY_INFO_0_7 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct WIA_PROPID_TO_NAME {
    pub propid: u32,
    pub pszName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for WIA_PROPID_TO_NAME {}
impl ::core::clone::Clone for WIA_PROPID_TO_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIA_PROPID_TO_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_PROPID_TO_NAME").field("propid", &self.propid).field("pszName", &self.pszName).finish()
    }
}
unsafe impl ::windows::core::Abi for WIA_PROPID_TO_NAME {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIA_PROPID_TO_NAME {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_PROPID_TO_NAME>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIA_PROPID_TO_NAME {}
impl ::core::default::Default for WIA_PROPID_TO_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PROPPAGE_CAMERA_ITEM_GENERAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PROPPAGE_DEVICE_GENERAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PROPPAGE_SCANNER_ITEM_GENERAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PROP_CACHEABLE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PROP_FLAG: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PROP_LIST: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PROP_NONE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PROP_RANGE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PROP_READ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PROP_SYNC_REQUIRED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_PROP_WRITE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_RANGE_MAX: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_RANGE_MIN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_RANGE_NOM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_RANGE_NUM_ELEMS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_RANGE_STEP: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct WIA_RAW_HEADER {
    pub Tag: u32,
    pub Version: u32,
    pub HeaderSize: u32,
    pub XRes: u32,
    pub YRes: u32,
    pub XExtent: u32,
    pub YExtent: u32,
    pub BytesPerLine: u32,
    pub BitsPerPixel: u32,
    pub ChannelsPerPixel: u32,
    pub DataType: u32,
    pub BitsPerChannel: [u8; 8],
    pub Compression: u32,
    pub PhotometricInterp: u32,
    pub LineOrder: u32,
    pub RawDataOffset: u32,
    pub RawDataSize: u32,
    pub PaletteOffset: u32,
    pub PaletteSize: u32,
}
impl ::core::marker::Copy for WIA_RAW_HEADER {}
impl ::core::clone::Clone for WIA_RAW_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIA_RAW_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIA_RAW_HEADER")
            .field("Tag", &self.Tag)
            .field("Version", &self.Version)
            .field("HeaderSize", &self.HeaderSize)
            .field("XRes", &self.XRes)
            .field("YRes", &self.YRes)
            .field("XExtent", &self.XExtent)
            .field("YExtent", &self.YExtent)
            .field("BytesPerLine", &self.BytesPerLine)
            .field("BitsPerPixel", &self.BitsPerPixel)
            .field("ChannelsPerPixel", &self.ChannelsPerPixel)
            .field("DataType", &self.DataType)
            .field("BitsPerChannel", &self.BitsPerChannel)
            .field("Compression", &self.Compression)
            .field("PhotometricInterp", &self.PhotometricInterp)
            .field("LineOrder", &self.LineOrder)
            .field("RawDataOffset", &self.RawDataOffset)
            .field("RawDataSize", &self.RawDataSize)
            .field("PaletteOffset", &self.PaletteOffset)
            .field("PaletteSize", &self.PaletteSize)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for WIA_RAW_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIA_RAW_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIA_RAW_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIA_RAW_HEADER {}
impl ::core::default::Default for WIA_RAW_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_REGISTER_EVENT_CALLBACK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_RESERVED_FOR_NEW_PROPS: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_SCAN_AHEAD_ALL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_SCAN_AHEAD_DISABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_SCAN_AHEAD_ENABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_SEGMENTATION_FILTER_STR: &'static str = "SegmentationFilter";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_SELECT_DEVICE_NODEFAULT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_SEPARATOR_DETECT_NOSCAN_CONTINUE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_SEPARATOR_DETECT_NOSCAN_STOP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_SEPARATOR_DETECT_SCAN_CONTINUE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_SEPARATOR_DETECT_SCAN_STOP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_SEPARATOR_DISABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_SET_DEFAULT_HANDLER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_SHOW_PREVIEW_CONTROL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_STATUS_CALIBRATING: ::windows::core::HRESULT = ::windows::core::HRESULT(2162691i32);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_STATUS_CLEAR: ::windows::core::HRESULT = ::windows::core::HRESULT(2162696i32);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_STATUS_END_OF_MEDIA: ::windows::core::HRESULT = ::windows::core::HRESULT(2162689i32);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_STATUS_NETWORK_DEVICE_RESERVED: ::windows::core::HRESULT = ::windows::core::HRESULT(2162695i32);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_STATUS_NOT_HANDLED: ::windows::core::HRESULT = ::windows::core::HRESULT(2162698i32);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_STATUS_RESERVING_NETWORK_DEVICE: ::windows::core::HRESULT = ::windows::core::HRESULT(2162694i32);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_STATUS_SKIP_ITEM: ::windows::core::HRESULT = ::windows::core::HRESULT(2162697i32);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_STATUS_WARMING_UP: ::windows::core::HRESULT = ::windows::core::HRESULT(2162690i32);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_S_CHANGE_DEVICE: ::windows::core::HRESULT = ::windows::core::HRESULT(2162699i32);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_S_NO_DEVICE_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2145320939i32);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_TRANSFER_ACQUIRE_CHILDREN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_TRANSFER_CHILDREN_SINGLE_SCAN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_TRANSFER_MSG_DEVICE_STATUS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_TRANSFER_MSG_END_OF_STREAM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_TRANSFER_MSG_END_OF_TRANSFER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_TRANSFER_MSG_NEW_PAGE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_TRANSFER_MSG_STATUS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_UNREGISTER_EVENT_CALLBACK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_USE_SEGMENTATION_FILTER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_WSD_FRIENDLY_NAME: u32 = 38920u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_WSD_FRIENDLY_NAME_STR: &'static str = "Friendly name";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_WSD_MANUFACTURER: u32 = 38914u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_WSD_MANUFACTURER_STR: &'static str = "Device manufacturer";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_WSD_MANUFACTURER_URL: u32 = 38915u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_WSD_MANUFACTURER_URL_STR: &'static str = "Manufacurer URL";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_WSD_MODEL_NAME: u32 = 38916u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_WSD_MODEL_NAME_STR: &'static str = "Model name";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_WSD_MODEL_NUMBER: u32 = 38917u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_WSD_MODEL_NUMBER_STR: &'static str = "Model number";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_WSD_MODEL_URL: u32 = 38918u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_WSD_MODEL_URL_STR: &'static str = "Model URL";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_WSD_PRESENTATION_URL: u32 = 38919u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_WSD_PRESENTATION_URL_STR: &'static str = "Presentation URL";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_WSD_SCAN_AVAILABLE_ITEM: u32 = 38922u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_WSD_SCAN_AVAILABLE_ITEM_STR: &'static str = "Scan Available Item";
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_WSD_SERIAL_NUMBER: u32 = 38921u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WIA_WSD_SERIAL_NUMBER_STR: &'static str = "Serial number";
pub const WiaAudFmt_AIFF: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66e2bf4f_b6fc_443f_94c8_2f33c8a65aaf);
pub const WiaAudFmt_MP3: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0fbc71fb_43bf_49f2_9190_e6fecff37e54);
pub const WiaAudFmt_WAV: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf818e146_07af_40ff_ae55_be8f2c065dbe);
pub const WiaAudFmt_WMA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd61d6413_8bc2_438f_93ad_21bd484db6a1);
pub const WiaDevMgr: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1f4e726_8cf1_11d1_bf92_0060081ed811);
pub const WiaDevMgr2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6c292bc_7c88_41ee_8b54_8ec92617e599);
pub const WiaImgFmt_ASF: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d948ee9_d0aa_4a12_9d9a_9cc5de36199b);
pub const WiaImgFmt_AVI: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32f8ca14_087c_4908_b7c4_6757fe7e90ab);
pub const WiaImgFmt_BMP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb96b3cab_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_CIFF: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9821a8ab_3a7e_4215_94e0_d27a460c03b2);
pub const WiaImgFmt_CSV: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x355bda24_5a9f_4494_80dc_be752cecbc8c);
pub const WiaImgFmt_DPOF: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x369eeeab_a0e8_45ca_86a6_a83ce5697e28);
pub const WiaImgFmt_EMF: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb96b3cac_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_EXEC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x485da097_141e_4aa5_bb3b_a5618d95d02b);
pub const WiaImgFmt_EXIF: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb96b3cb2_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_FLASHPIX: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb96b3cb4_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_GIF: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb96b3cb0_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_HTML: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc99a4e62_99de_4a94_acca_71956ac2977d);
pub const WiaImgFmt_ICO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb96b3cb5_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_JBIG: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41e8dd92_2f0a_43d4_8636_f1614ba11e46);
pub const WiaImgFmt_JBIG2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb8e7e67_283c_4235_9e59_0b9bf94ca687);
pub const WiaImgFmt_JPEG: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb96b3cae_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_JPEG2K: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x344ee2b2_39db_4dde_8173_c4b75f8f1e49);
pub const WiaImgFmt_JPEG2KX: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43e14614_c80a_4850_baf3_4b152dc8da27);
pub const WiaImgFmt_MEMORYBMP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb96b3caa_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_MPG: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecd757e4_d2ec_4f57_955d_bcf8a97c4e52);
pub const WiaImgFmt_OXPS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c7b1240_c14d_4109_9755_04b89025153a);
pub const WiaImgFmt_PDFA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9980bd5b_3463_43c7_bdca_3caa146f229f);
pub const WiaImgFmt_PHOTOCD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb96b3cb3_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_PICT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6bc85d8_6b3e_40ee_a95c_25d482e41adc);
pub const WiaImgFmt_PNG: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb96b3caf_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_RAW: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f120719_f1a8_4e07_9ade_9b64c63a3dcc);
pub const WiaImgFmt_RAWBAR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda63f833_d26e_451e_90d2_ea55a1365d62);
pub const WiaImgFmt_RAWMIC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22c4f058_0d88_409c_ac1c_eec12b0ea680);
pub const WiaImgFmt_RAWPAT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7760507c_5064_400c_9a17_575624d8824b);
pub const WiaImgFmt_RAWRGB: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbca48b55_f272_4371_b0f1_4a150d057bb4);
pub const WiaImgFmt_RTF: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x573dd6a3_4834_432d_a9b5_e198dd9e890d);
pub const WiaImgFmt_SCRIPT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe7d6c53_2dac_446a_b0bd_d73e21e924c9);
pub const WiaImgFmt_TIFF: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb96b3cb1_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_TXT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfafd4d82_723f_421f_9318_30501ac44b59);
pub const WiaImgFmt_UNDEFINED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb96b3ca9_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_UNICODE16: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b7639b6_6357_47d1_9a07_12452dc073e9);
pub const WiaImgFmt_WMF: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb96b3cad_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_XML: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9171457_dac8_4884_b393_15b471d5f07e);
pub const WiaImgFmt_XMLBAR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6235701c_3a98_484c_b2a8_fdffd87e6b16);
pub const WiaImgFmt_XMLMIC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d164c61_b9ae_4b23_8973_c7067e1fbd31);
pub const WiaImgFmt_XMLPAT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8986f55_f052_460d_9523_3a7dfedbb33c);
pub const WiaImgFmt_XPS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x700b4a0f_2011_411c_b430_d1e0b2e10b28);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeAnalyze: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeAudio: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeBurst: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeDeleted: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeDevice: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeDisconnected: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeDocument: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeFile: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeFolder: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeFree: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeGenerated: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeHPanorama: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeHasAttachments: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeImage: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeMask: u32 = 2148532223u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeProgrammableDataSource: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeRemoved: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeRoot: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeStorage: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeTransfer: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeTwainCapabilityPassThrough: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeVPanorama: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const WiaItemTypeVideo: u32 = 65536u32;
pub const WiaLog: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1e75357_881a_419e_83e2_bb16db197c68);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub struct WiaTransferParams {
    pub lMessage: i32,
    pub lPercentComplete: i32,
    pub ulTransferredBytes: u64,
    pub hrErrorStatus: ::windows::core::HRESULT,
}
impl ::core::marker::Copy for WiaTransferParams {}
impl ::core::clone::Clone for WiaTransferParams {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WiaTransferParams {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WiaTransferParams").field("lMessage", &self.lMessage).field("lPercentComplete", &self.lPercentComplete).field("ulTransferredBytes", &self.ulTransferredBytes).field("hrErrorStatus", &self.hrErrorStatus).finish()
    }
}
unsafe impl ::windows::core::Abi for WiaTransferParams {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WiaTransferParams {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WiaTransferParams>()) == 0 }
    }
}
impl ::core::cmp::Eq for WiaTransferParams {}
impl ::core::default::Default for WiaTransferParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WiaVideo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3908c3cd_4478_4536_af2f_10c25d4ef89a);
#[doc = "*Required features: `\"Win32_Devices_ImageAcquisition\"`*"]
pub const g_dwDebugFlags: u32 = 0u32;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
