#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub const ADVANCED_DUP: u32 = 8192u32;
pub const ADVANCED_DUPLEX: u32 = 1024u32;
pub const ALL_PAGES: u32 = 0u32;
pub const AUTO_ADVANCE: u32 = 512u32;
pub const AUTO_SOURCE: u32 = 32768u32;
pub const BACK_FIRST: u32 = 16u32;
pub const BACK_ONLY: u32 = 64u32;
pub const BARCODE_READER: u32 = 262144u32;
pub const BARCODE_READER_READY: u32 = 16384u32;
pub const BASE_VAL_WIA_ERROR: u32 = 0u32;
pub const BASE_VAL_WIA_SUCCESS: u32 = 0u32;
pub const BOTTOM_JUSTIFIED: u32 = 2u32;
pub const BUS_TYPE_FIREWIRE: u32 = 203u32;
pub const BUS_TYPE_PARALLEL: u32 = 202u32;
pub const BUS_TYPE_SCSI: u32 = 200u32;
pub const BUS_TYPE_USB: u32 = 201u32;
pub const CAPTUREMODE_BURST: u32 = 2u32;
pub const CAPTUREMODE_NORMAL: u32 = 1u32;
pub const CAPTUREMODE_TIMELAPSE: u32 = 3u32;
pub const CENTERED: u32 = 1u32;
pub const CLSID_WiaDefaultSegFilter: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3572814603,
        2857,
        17672,
        [137, 34, 12, 87, 151, 212, 39, 101],
    );
pub const CMD_GETADFAVAILABLE: u32 = 117u32;
pub const CMD_GETADFHASPAPER: u32 = 120u32;
pub const CMD_GETADFOPEN: u32 = 118u32;
pub const CMD_GETADFREADY: u32 = 119u32;
pub const CMD_GETADFSTATUS: u32 = 121u32;
pub const CMD_GETADFUNLOADREADY: u32 = 122u32;
pub const CMD_GETCAPABILITIES: u32 = 132u32;
pub const CMD_GETSUPPORTEDFILEFORMATS: u32 = 138u32;
pub const CMD_GETSUPPORTEDMEMORYFORMATS: u32 = 139u32;
pub const CMD_GETTPAAVAILABLE: u32 = 123u32;
pub const CMD_GETTPAOPENED: u32 = 124u32;
pub const CMD_GET_INTERRUPT_EVENT: u32 = 133u32;
pub const CMD_INITIALIZE: u32 = 100u32;
pub const CMD_LOAD_ADF: u32 = 115u32;
pub const CMD_RESETSCANNER: u32 = 131u32;
pub const CMD_SENDSCSICOMMAND: u32 = 127u32;
pub const CMD_SETCOLORDITHER: u32 = 111u32;
pub const CMD_SETCONTRAST: u32 = 104u32;
pub const CMD_SETDATATYPE: u32 = 106u32;
pub const CMD_SETDITHER: u32 = 107u32;
pub const CMD_SETFILTER: u32 = 114u32;
pub const CMD_SETFORMAT: u32 = 140u32;
pub const CMD_SETGSDNAME: u32 = 134u32;
pub const CMD_SETINTENSITY: u32 = 105u32;
pub const CMD_SETLAMP: u32 = 126u32;
pub const CMD_SETMATRIX: u32 = 112u32;
pub const CMD_SETMIRROR: u32 = 108u32;
pub const CMD_SETNEGATIVE: u32 = 109u32;
pub const CMD_SETSCANMODE: u32 = 135u32;
pub const CMD_SETSPEED: u32 = 113u32;
pub const CMD_SETSTIDEVICEHKEY: u32 = 136u32;
pub const CMD_SETTONEMAP: u32 = 110u32;
pub const CMD_SETXRESOLUTION: u32 = 102u32;
pub const CMD_SETYRESOLUTION: u32 = 103u32;
pub const CMD_STI_DEVICERESET: u32 = 128u32;
pub const CMD_STI_DIAGNOSTIC: u32 = 130u32;
pub const CMD_STI_GETSTATUS: u32 = 129u32;
pub const CMD_TPAREADY: u32 = 125u32;
pub const CMD_UNINITIALIZE: u32 = 101u32;
pub const CMD_UNLOAD_ADF: u32 = 116u32;
pub const COPY_PARENT_PROPERTY_VALUES: u32 = 1073741824u32;
pub const DETECT_DUP: u32 = 64u32;
pub const DETECT_DUP_AVAIL: u32 = 256u32;
pub const DETECT_FEED: u32 = 32u32;
pub const DETECT_FEED_AVAIL: u32 = 128u32;
pub const DETECT_FILM_TPA: u32 = 1024u32;
pub const DETECT_FLAT: u32 = 8u32;
pub const DETECT_SCAN: u32 = 16u32;
pub const DETECT_STOR: u32 = 4096u32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DEVICEDIALOGDATA {
    pub cbSize: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub pIWiaItemRoot: ::std::option::Option<IWiaItem>,
    pub dwFlags: u32,
    pub lIntent: i32,
    pub lItemCount: i32,
    pub ppWiaItems: *mut ::std::option::Option<IWiaItem>,
}
#[cfg(feature = "Win32_Foundation")]
impl DEVICEDIALOGDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DEVICEDIALOGDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DEVICEDIALOGDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DEVICEDIALOGDATA")
            .field("cbSize", &self.cbSize)
            .field("hwndParent", &self.hwndParent)
            .field("pIWiaItemRoot", &self.pIWiaItemRoot)
            .field("dwFlags", &self.dwFlags)
            .field("lIntent", &self.lIntent)
            .field("lItemCount", &self.lItemCount)
            .field("ppWiaItems", &self.ppWiaItems)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DEVICEDIALOGDATA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.hwndParent == other.hwndParent
            && self.pIWiaItemRoot == other.pIWiaItemRoot
            && self.dwFlags == other.dwFlags
            && self.lIntent == other.lIntent
            && self.lItemCount == other.lItemCount
            && self.ppWiaItems == other.ppWiaItems
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DEVICEDIALOGDATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DEVICEDIALOGDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DEVICEDIALOGDATA2 {
    pub cbSize: u32,
    pub pIWiaItemRoot: ::std::option::Option<IWiaItem2>,
    pub dwFlags: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub bstrFolderName: super::super::Foundation::BSTR,
    pub bstrFilename: super::super::Foundation::BSTR,
    pub lNumFiles: i32,
    pub pbstrFilePaths: *mut super::super::Foundation::BSTR,
    pub pWiaItem: ::std::option::Option<IWiaItem2>,
}
#[cfg(feature = "Win32_Foundation")]
impl DEVICEDIALOGDATA2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DEVICEDIALOGDATA2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DEVICEDIALOGDATA2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DEVICEDIALOGDATA2")
            .field("cbSize", &self.cbSize)
            .field("pIWiaItemRoot", &self.pIWiaItemRoot)
            .field("dwFlags", &self.dwFlags)
            .field("hwndParent", &self.hwndParent)
            .field("bstrFolderName", &self.bstrFolderName)
            .field("bstrFilename", &self.bstrFilename)
            .field("lNumFiles", &self.lNumFiles)
            .field("pbstrFilePaths", &self.pbstrFilePaths)
            .field("pWiaItem", &self.pWiaItem)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DEVICEDIALOGDATA2 {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.pIWiaItemRoot == other.pIWiaItemRoot
            && self.dwFlags == other.dwFlags
            && self.hwndParent == other.hwndParent
            && self.bstrFolderName == other.bstrFolderName
            && self.bstrFilename == other.bstrFilename
            && self.lNumFiles == other.lNumFiles
            && self.pbstrFilePaths == other.pbstrFilePaths
            && self.pWiaItem == other.pWiaItem
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DEVICEDIALOGDATA2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DEVICEDIALOGDATA2 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
pub const DEVICE_ATTENTION: u32 = 1024u32;
pub const DUP: u32 = 4u32;
pub const DUPLEX: u32 = 4u32;
pub const DUP_READY: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
pub type DeviceDialogFunction = unsafe extern "system" fn(
    param0: *mut ::std::mem::ManuallyDrop<DEVICEDIALOGDATA>,
) -> ::windows::runtime::HRESULT;
pub const EFFECTMODE_BW: u32 = 2u32;
pub const EFFECTMODE_SEPIA: u32 = 3u32;
pub const EFFECTMODE_STANDARD: u32 = 1u32;
pub const ENDORSER: u32 = 131072u32;
pub const ENDORSER_READY: u32 = 8192u32;
pub const ESC_TWAIN_CAPABILITY: u32 = 2001u32;
pub const ESC_TWAIN_PRIVATE_SUPPORTED_CAPS: u32 = 2002u32;
pub const EXPOSUREMETERING_AVERAGE: u32 = 1u32;
pub const EXPOSUREMETERING_CENTERSPOT: u32 = 4u32;
pub const EXPOSUREMETERING_CENTERWEIGHT: u32 = 2u32;
pub const EXPOSUREMETERING_MULTISPOT: u32 = 3u32;
pub const EXPOSUREMODE_APERTURE_PRIORITY: u32 = 3u32;
pub const EXPOSUREMODE_AUTO: u32 = 2u32;
pub const EXPOSUREMODE_MANUAL: u32 = 1u32;
pub const EXPOSUREMODE_PORTRAIT: u32 = 7u32;
pub const EXPOSUREMODE_PROGRAM_ACTION: u32 = 6u32;
pub const EXPOSUREMODE_PROGRAM_CREATIVE: u32 = 5u32;
pub const EXPOSUREMODE_SHUTTER_PRIORITY: u32 = 4u32;
pub const FEED: u32 = 1u32;
pub const FEEDER: u32 = 1u32;
pub const FEED_READY: u32 = 1u32;
pub const FILM_TPA: u32 = 512u32;
pub const FILM_TPA_READY: u32 = 64u32;
pub const FLASHMODE_AUTO: u32 = 1u32;
pub const FLASHMODE_EXTERNALSYNC: u32 = 6u32;
pub const FLASHMODE_FILL: u32 = 3u32;
pub const FLASHMODE_OFF: u32 = 2u32;
pub const FLASHMODE_REDEYE_AUTO: u32 = 4u32;
pub const FLASHMODE_REDEYE_FILL: u32 = 5u32;
pub const FLAT: u32 = 2u32;
pub const FLATBED: u32 = 2u32;
pub const FLAT_COVER_UP: u32 = 8u32;
pub const FLAT_READY: u32 = 2u32;
pub const FOCUSMETERING_CENTERSPOT: u32 = 1u32;
pub const FOCUSMETERING_MULTISPOT: u32 = 2u32;
pub const FOCUSMODE_AUTO: u32 = 2u32;
pub const FOCUSMODE_MACROAUTO: u32 = 3u32;
pub const FOCUSMODE_MANUAL: u32 = 1u32;
pub const FRONT_FIRST: u32 = 8u32;
pub const FRONT_ONLY: u32 = 32u32;
pub const GUID_DEVINTERFACE_IMAGE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1809653702,
    33039,
    4560,
    [190, 199, 8, 0, 43, 226, 9, 47],
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IEnumWIA_DEV_CAPS(::windows::runtime::IUnknown);
impl IEnumWIA_DEV_CAPS {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Next(
        &self,
        celt: u32,
        rgelt: *mut WIA_DEV_CAP,
        pceltfetched: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(celt),
            ::std::mem::transmute(rgelt),
            ::std::mem::transmute(pceltfetched),
        )
        .ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(celt),
        )
        .ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumWIA_DEV_CAPS> {
        let mut result__: <IEnumWIA_DEV_CAPS as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IEnumWIA_DEV_CAPS>(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumWIA_DEV_CAPS {
    type Vtable = IEnumWIA_DEV_CAPS_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        533480071,
        44198,
        4562,
        [160, 147, 0, 192, 79, 114, 220, 60],
    );
}
impl ::std::convert::From<IEnumWIA_DEV_CAPS> for ::windows::runtime::IUnknown {
    fn from(value: IEnumWIA_DEV_CAPS) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IEnumWIA_DEV_CAPS> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumWIA_DEV_CAPS) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumWIA_DEV_CAPS {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IEnumWIA_DEV_CAPS {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWIA_DEV_CAPS_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        celt: u32,
        rgelt: *mut ::std::mem::ManuallyDrop<WIA_DEV_CAP>,
        pceltfetched: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        celt: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppienum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcelt: *mut u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IEnumWIA_DEV_INFO(::windows::runtime::IUnknown);
impl IEnumWIA_DEV_INFO {
    pub unsafe fn Next(
        &self,
        celt: u32,
        rgelt: *mut ::std::option::Option<IWiaPropertyStorage>,
        pceltfetched: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(celt),
            ::std::mem::transmute(rgelt),
            ::std::mem::transmute(pceltfetched),
        )
        .ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(celt),
        )
        .ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumWIA_DEV_INFO> {
        let mut result__: <IEnumWIA_DEV_INFO as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IEnumWIA_DEV_INFO>(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumWIA_DEV_INFO {
    type Vtable = IEnumWIA_DEV_INFO_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1580775484,
        36081,
        4561,
        [191, 146, 0, 96, 8, 30, 216, 17],
    );
}
impl ::std::convert::From<IEnumWIA_DEV_INFO> for ::windows::runtime::IUnknown {
    fn from(value: IEnumWIA_DEV_INFO) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IEnumWIA_DEV_INFO> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumWIA_DEV_INFO) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumWIA_DEV_INFO {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IEnumWIA_DEV_INFO {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWIA_DEV_INFO_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        celt: u32,
        rgelt: *mut ::windows::runtime::RawPtr,
        pceltfetched: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        celt: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppienum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        celt: *mut u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IEnumWIA_FORMAT_INFO(::windows::runtime::IUnknown);
impl IEnumWIA_FORMAT_INFO {
    pub unsafe fn Next(
        &self,
        celt: u32,
        rgelt: *mut WIA_FORMAT_INFO,
        pceltfetched: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(celt),
            ::std::mem::transmute(rgelt),
            ::std::mem::transmute(pceltfetched),
        )
        .ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(celt),
        )
        .ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumWIA_FORMAT_INFO> {
        let mut result__: <IEnumWIA_FORMAT_INFO as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IEnumWIA_FORMAT_INFO>(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumWIA_FORMAT_INFO {
    type Vtable = IEnumWIA_FORMAT_INFO_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2176777307,
        25965,
        17649,
        [178, 76, 212, 29, 81, 180, 220, 129],
    );
}
impl ::std::convert::From<IEnumWIA_FORMAT_INFO> for ::windows::runtime::IUnknown {
    fn from(value: IEnumWIA_FORMAT_INFO) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IEnumWIA_FORMAT_INFO> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumWIA_FORMAT_INFO) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumWIA_FORMAT_INFO {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IEnumWIA_FORMAT_INFO {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWIA_FORMAT_INFO_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        celt: u32,
        rgelt: *mut WIA_FORMAT_INFO,
        pceltfetched: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        celt: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppienum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcelt: *mut u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IEnumWiaItem(::windows::runtime::IUnknown);
impl IEnumWiaItem {
    pub unsafe fn Next(
        &self,
        celt: u32,
        ppiwiaitem: *mut ::std::option::Option<IWiaItem>,
        pceltfetched: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(celt),
            ::std::mem::transmute(ppiwiaitem),
            ::std::mem::transmute(pceltfetched),
        )
        .ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(celt),
        )
        .ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumWiaItem> {
        let mut result__: <IEnumWiaItem as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IEnumWiaItem>(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumWiaItem {
    type Vtable = IEnumWiaItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1585677308,
        13201,
        4562,
        [154, 51, 0, 192, 79, 163, 97, 69],
    );
}
impl ::std::convert::From<IEnumWiaItem> for ::windows::runtime::IUnknown {
    fn from(value: IEnumWiaItem) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IEnumWiaItem> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumWiaItem) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumWiaItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IEnumWiaItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWiaItem_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        celt: u32,
        ppiwiaitem: *mut ::windows::runtime::RawPtr,
        pceltfetched: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        celt: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppienum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        celt: *mut u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IEnumWiaItem2(::windows::runtime::IUnknown);
impl IEnumWiaItem2 {
    pub unsafe fn Next(
        &self,
        celt: u32,
        ppiwiaitem2: *mut ::std::option::Option<IWiaItem2>,
        pceltfetched: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(celt),
            ::std::mem::transmute(ppiwiaitem2),
            ::std::mem::transmute(pceltfetched),
        )
        .ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(celt),
        )
        .ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumWiaItem2> {
        let mut result__: <IEnumWiaItem2 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IEnumWiaItem2>(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumWiaItem2 {
    type Vtable = IEnumWiaItem2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1503070964,
        52493,
        17625,
        [171, 36, 82, 41, 86, 48, 229, 130],
    );
}
impl ::std::convert::From<IEnumWiaItem2> for ::windows::runtime::IUnknown {
    fn from(value: IEnumWiaItem2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IEnumWiaItem2> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumWiaItem2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumWiaItem2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IEnumWiaItem2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWiaItem2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        celt: u32,
        ppiwiaitem2: *mut ::windows::runtime::RawPtr,
        pceltfetched: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        celt: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppienum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        celt: *mut u32,
    ) -> ::windows::runtime::HRESULT,
);
pub const IMPRINTER: u32 = 65536u32;
pub const IMPRINTER_READY: u32 = 4096u32;
pub const IT_MSG_DATA: u32 = 2u32;
pub const IT_MSG_DATA_HEADER: u32 = 1u32;
pub const IT_MSG_FILE_PREVIEW_DATA: u32 = 6u32;
pub const IT_MSG_FILE_PREVIEW_DATA_HEADER: u32 = 7u32;
pub const IT_MSG_NEW_PAGE: u32 = 5u32;
pub const IT_MSG_STATUS: u32 = 3u32;
pub const IT_MSG_TERMINATION: u32 = 4u32;
pub const IT_STATUS_MASK: u32 = 7u32;
pub const IT_STATUS_PROCESSING_DATA: u32 = 2u32;
pub const IT_STATUS_TRANSFER_FROM_DEVICE: u32 = 1u32;
pub const IT_STATUS_TRANSFER_TO_CLIENT: u32 = 4u32;
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IWiaAppErrorHandler(::windows::runtime::IUnknown);
impl IWiaAppErrorHandler {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindow(&self) -> ::windows::runtime::Result<super::super::Foundation::HWND> {
        let mut result__: <super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::HWND>(result__)
    }
    pub unsafe fn ReportStatus<'a, Param1: ::windows::runtime::IntoParam<'a, IWiaItem2>>(
        &self,
        lflags: i32,
        pwiaitem2: Param1,
        hrstatus: ::windows::runtime::HRESULT,
        lpercentcomplete: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
            pwiaitem2.into_param().abi(),
            ::std::mem::transmute(hrstatus),
            ::std::mem::transmute(lpercentcomplete),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWiaAppErrorHandler {
    type Vtable = IWiaAppErrorHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1813387372,
        53414,
        16396,
        [128, 244, 210, 105, 134, 160, 231, 52],
    );
}
impl ::std::convert::From<IWiaAppErrorHandler> for ::windows::runtime::IUnknown {
    fn from(value: IWiaAppErrorHandler) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWiaAppErrorHandler> for ::windows::runtime::IUnknown {
    fn from(value: &IWiaAppErrorHandler) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWiaAppErrorHandler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWiaAppErrorHandler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaAppErrorHandler_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        phwnd: *mut super::super::Foundation::HWND,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
        pwiaitem2: ::windows::runtime::RawPtr,
        hrstatus: ::windows::runtime::HRESULT,
        lpercentcomplete: i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IWiaDataCallback(::windows::runtime::IUnknown);
impl IWiaDataCallback {
    pub unsafe fn BandedDataCallback(
        &self,
        lmessage: i32,
        lstatus: i32,
        lpercentcomplete: i32,
        loffset: i32,
        llength: i32,
        lreserved: i32,
        lreslength: i32,
        pbbuffer: *mut u8,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lmessage),
            ::std::mem::transmute(lstatus),
            ::std::mem::transmute(lpercentcomplete),
            ::std::mem::transmute(loffset),
            ::std::mem::transmute(llength),
            ::std::mem::transmute(lreserved),
            ::std::mem::transmute(lreslength),
            ::std::mem::transmute(pbbuffer),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWiaDataCallback {
    type Vtable = IWiaDataCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2774050918,
        42416,
        4562,
        [160, 143, 0, 192, 79, 114, 220, 60],
    );
}
impl ::std::convert::From<IWiaDataCallback> for ::windows::runtime::IUnknown {
    fn from(value: IWiaDataCallback) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWiaDataCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IWiaDataCallback) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWiaDataCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWiaDataCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaDataCallback_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lmessage: i32,
        lstatus: i32,
        lpercentcomplete: i32,
        loffset: i32,
        llength: i32,
        lreserved: i32,
        lreslength: i32,
        pbbuffer: *mut u8,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IWiaDataTransfer(::windows::runtime::IUnknown);
impl IWiaDataTransfer {
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Graphics_Gdi",
        feature = "Win32_System_Com",
        feature = "Win32_System_Com_StructuredStorage"
    ))]
    pub unsafe fn idtGetData<'a, Param1: ::windows::runtime::IntoParam<'a, IWiaDataCallback>>(
        &self,
        pmedium: *mut super::super::System::Com::STGMEDIUM,
        piwiadatacallback: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pmedium),
            piwiadatacallback.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn idtGetBandedData<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, IWiaDataCallback>,
    >(
        &self,
        pwiadatatransinfo: *mut WIA_DATA_TRANSFER_INFO,
        piwiadatacallback: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pwiadatatransinfo),
            piwiadatacallback.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn idtQueryGetData(
        &self,
        pfe: *const WIA_FORMAT_INFO,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pfe),
        )
        .ok()
    }
    pub unsafe fn idtEnumWIA_FORMAT_INFO(
        &self,
    ) -> ::windows::runtime::Result<IEnumWIA_FORMAT_INFO> {
        let mut result__: <IEnumWIA_FORMAT_INFO as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IEnumWIA_FORMAT_INFO>(result__)
    }
    pub unsafe fn idtGetExtendedTransferInfo(
        &self,
    ) -> ::windows::runtime::Result<WIA_EXTENDED_TRANSFER_INFO> {
        let mut result__: <WIA_EXTENDED_TRANSFER_INFO as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<WIA_EXTENDED_TRANSFER_INFO>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWiaDataTransfer {
    type Vtable = IWiaDataTransfer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2798582168,
        42416,
        4562,
        [160, 143, 0, 192, 79, 114, 220, 60],
    );
}
impl ::std::convert::From<IWiaDataTransfer> for ::windows::runtime::IUnknown {
    fn from(value: IWiaDataTransfer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWiaDataTransfer> for ::windows::runtime::IUnknown {
    fn from(value: &IWiaDataTransfer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWiaDataTransfer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWiaDataTransfer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaDataTransfer_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_Graphics_Gdi",
        feature = "Win32_System_Com",
        feature = "Win32_System_Com_StructuredStorage"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pmedium: *mut ::std::mem::ManuallyDrop<super::super::System::Com::STGMEDIUM>,
        piwiadatacallback: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_Graphics_Gdi",
        feature = "Win32_System_Com",
        feature = "Win32_System_Com_StructuredStorage"
    )))]
    usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwiadatatransinfo: *mut WIA_DATA_TRANSFER_INFO,
        piwiadatacallback: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pfe: *const WIA_FORMAT_INFO,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pextendedtransferinfo: *mut WIA_EXTENDED_TRANSFER_INFO,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IWiaDevMgr(::windows::runtime::IUnknown);
impl IWiaDevMgr {
    pub unsafe fn EnumDeviceInfo(
        &self,
        lflag: i32,
    ) -> ::windows::runtime::Result<IEnumWIA_DEV_INFO> {
        let mut result__: <IEnumWIA_DEV_INFO as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflag),
            &mut result__,
        )
        .from_abi::<IEnumWIA_DEV_INFO>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateDevice<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrdeviceid: Param0,
    ) -> ::windows::runtime::Result<IWiaItem> {
        let mut result__: <IWiaItem as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            bstrdeviceid.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IWiaItem>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SelectDeviceDlg<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        hwndparent: Param0,
        ldevicetype: i32,
        lflags: i32,
        pbstrdeviceid: *mut super::super::Foundation::BSTR,
        ppitemroot: *mut ::std::option::Option<IWiaItem>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            hwndparent.into_param().abi(),
            ::std::mem::transmute(ldevicetype),
            ::std::mem::transmute(lflags),
            ::std::mem::transmute(pbstrdeviceid),
            ::std::mem::transmute(ppitemroot),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SelectDeviceDlgID<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        hwndparent: Param0,
        ldevicetype: i32,
        lflags: i32,
        pbstrdeviceid: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            hwndparent.into_param().abi(),
            ::std::mem::transmute(ldevicetype),
            ::std::mem::transmute(lflags),
            ::std::mem::transmute(pbstrdeviceid),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetImageDlg<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
        Param4: ::windows::runtime::IntoParam<'a, IWiaItem>,
        Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        hwndparent: Param0,
        ldevicetype: i32,
        lflags: i32,
        lintent: i32,
        pitemroot: Param4,
        bstrfilename: Param5,
        pguidformat: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            hwndparent.into_param().abi(),
            ::std::mem::transmute(ldevicetype),
            ::std::mem::transmute(lflags),
            ::std::mem::transmute(lintent),
            pitemroot.into_param().abi(),
            bstrfilename.into_param().abi(),
            ::std::mem::transmute(pguidformat),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterEventCallbackProgram<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        lflags: i32,
        bstrdeviceid: Param1,
        peventguid: *const ::windows::runtime::GUID,
        bstrcommandline: Param3,
        bstrname: Param4,
        bstrdescription: Param5,
        bstricon: Param6,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
            bstrdeviceid.into_param().abi(),
            ::std::mem::transmute(peventguid),
            bstrcommandline.into_param().abi(),
            bstrname.into_param().abi(),
            bstrdescription.into_param().abi(),
            bstricon.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterEventCallbackInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param3: ::windows::runtime::IntoParam<'a, IWiaEventCallback>,
    >(
        &self,
        lflags: i32,
        bstrdeviceid: Param1,
        peventguid: *const ::windows::runtime::GUID,
        piwiaeventcallback: Param3,
    ) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
            bstrdeviceid.into_param().abi(),
            ::std::mem::transmute(peventguid),
            piwiaeventcallback.into_param().abi(),
            &mut result__,
        )
        .from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterEventCallbackCLSID<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        lflags: i32,
        bstrdeviceid: Param1,
        peventguid: *const ::windows::runtime::GUID,
        pclsid: *const ::windows::runtime::GUID,
        bstrname: Param4,
        bstrdescription: Param5,
        bstricon: Param6,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
            bstrdeviceid.into_param().abi(),
            ::std::mem::transmute(peventguid),
            ::std::mem::transmute(pclsid),
            bstrname.into_param().abi(),
            bstrdescription.into_param().abi(),
            bstricon.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDeviceDlg<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        hwndparent: Param0,
        lflags: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            hwndparent.into_param().abi(),
            ::std::mem::transmute(lflags),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWiaDevMgr {
    type Vtable = IWiaDevMgr_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1588744234,
        36081,
        4561,
        [191, 146, 0, 96, 8, 30, 216, 17],
    );
}
impl ::std::convert::From<IWiaDevMgr> for ::windows::runtime::IUnknown {
    fn from(value: IWiaDevMgr) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWiaDevMgr> for ::windows::runtime::IUnknown {
    fn from(value: &IWiaDevMgr) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWiaDevMgr {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWiaDevMgr {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaDevMgr_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflag: i32,
        ppienum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrdeviceid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        ppwiaitemroot: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hwndparent: super::super::Foundation::HWND,
        ldevicetype: i32,
        lflags: i32,
        pbstrdeviceid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        ppitemroot: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hwndparent: super::super::Foundation::HWND,
        ldevicetype: i32,
        lflags: i32,
        pbstrdeviceid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hwndparent: super::super::Foundation::HWND,
        ldevicetype: i32,
        lflags: i32,
        lintent: i32,
        pitemroot: ::windows::runtime::RawPtr,
        bstrfilename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        pguidformat: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
        bstrdeviceid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        peventguid: *const ::windows::runtime::GUID,
        bstrcommandline: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrdescription: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstricon: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
        bstrdeviceid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        peventguid: *const ::windows::runtime::GUID,
        piwiaeventcallback: ::windows::runtime::RawPtr,
        peventobject: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
        bstrdeviceid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        peventguid: *const ::windows::runtime::GUID,
        pclsid: *const ::windows::runtime::GUID,
        bstrname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrdescription: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstricon: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hwndparent: super::super::Foundation::HWND,
        lflags: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IWiaDevMgr2(::windows::runtime::IUnknown);
impl IWiaDevMgr2 {
    pub unsafe fn EnumDeviceInfo(
        &self,
        lflags: i32,
    ) -> ::windows::runtime::Result<IEnumWIA_DEV_INFO> {
        let mut result__: <IEnumWIA_DEV_INFO as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
            &mut result__,
        )
        .from_abi::<IEnumWIA_DEV_INFO>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateDevice<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        lflags: i32,
        bstrdeviceid: Param1,
    ) -> ::windows::runtime::Result<IWiaItem2> {
        let mut result__: <IWiaItem2 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
            bstrdeviceid.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IWiaItem2>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SelectDeviceDlg<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        hwndparent: Param0,
        ldevicetype: i32,
        lflags: i32,
        pbstrdeviceid: *mut super::super::Foundation::BSTR,
        ppitemroot: *mut ::std::option::Option<IWiaItem2>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            hwndparent.into_param().abi(),
            ::std::mem::transmute(ldevicetype),
            ::std::mem::transmute(lflags),
            ::std::mem::transmute(pbstrdeviceid),
            ::std::mem::transmute(ppitemroot),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SelectDeviceDlgID<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        hwndparent: Param0,
        ldevicetype: i32,
        lflags: i32,
        pbstrdeviceid: *mut super::super::Foundation::BSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            hwndparent.into_param().abi(),
            ::std::mem::transmute(ldevicetype),
            ::std::mem::transmute(lflags),
            ::std::mem::transmute(pbstrdeviceid),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterEventCallbackInterface<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param3: ::windows::runtime::IntoParam<'a, IWiaEventCallback>,
    >(
        &self,
        lflags: i32,
        bstrdeviceid: Param1,
        peventguid: *const ::windows::runtime::GUID,
        piwiaeventcallback: Param3,
    ) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
            bstrdeviceid.into_param().abi(),
            ::std::mem::transmute(peventguid),
            piwiaeventcallback.into_param().abi(),
            &mut result__,
        )
        .from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterEventCallbackProgram<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param7: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        lflags: i32,
        bstrdeviceid: Param1,
        peventguid: *const ::windows::runtime::GUID,
        bstrfullappname: Param3,
        bstrcommandlinearg: Param4,
        bstrname: Param5,
        bstrdescription: Param6,
        bstricon: Param7,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
            bstrdeviceid.into_param().abi(),
            ::std::mem::transmute(peventguid),
            bstrfullappname.into_param().abi(),
            bstrcommandlinearg.into_param().abi(),
            bstrname.into_param().abi(),
            bstrdescription.into_param().abi(),
            bstricon.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterEventCallbackCLSID<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        lflags: i32,
        bstrdeviceid: Param1,
        peventguid: *const ::windows::runtime::GUID,
        pclsid: *const ::windows::runtime::GUID,
        bstrname: Param4,
        bstrdescription: Param5,
        bstricon: Param6,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
            bstrdeviceid.into_param().abi(),
            ::std::mem::transmute(peventguid),
            ::std::mem::transmute(pclsid),
            bstrname.into_param().abi(),
            bstrdescription.into_param().abi(),
            bstricon.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetImageDlg<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        lflags: i32,
        bstrdeviceid: Param1,
        hwndparent: Param2,
        bstrfoldername: Param3,
        bstrfilename: Param4,
        plnumfiles: *mut i32,
        ppbstrfilepaths: *mut *mut super::super::Foundation::BSTR,
        ppitem: *mut ::std::option::Option<IWiaItem2>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
            bstrdeviceid.into_param().abi(),
            hwndparent.into_param().abi(),
            bstrfoldername.into_param().abi(),
            bstrfilename.into_param().abi(),
            ::std::mem::transmute(plnumfiles),
            ::std::mem::transmute(ppbstrfilepaths),
            ::std::mem::transmute(ppitem),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWiaDevMgr2 {
    type Vtable = IWiaDevMgr2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2042658033,
        52189,
        16878,
        [142, 195, 240, 0, 128, 202, 218, 122],
    );
}
impl ::std::convert::From<IWiaDevMgr2> for ::windows::runtime::IUnknown {
    fn from(value: IWiaDevMgr2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWiaDevMgr2> for ::windows::runtime::IUnknown {
    fn from(value: &IWiaDevMgr2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWiaDevMgr2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWiaDevMgr2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaDevMgr2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
        ppienum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
        bstrdeviceid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        ppwiaitem2root: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hwndparent: super::super::Foundation::HWND,
        ldevicetype: i32,
        lflags: i32,
        pbstrdeviceid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        ppitemroot: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hwndparent: super::super::Foundation::HWND,
        ldevicetype: i32,
        lflags: i32,
        pbstrdeviceid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
        bstrdeviceid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        peventguid: *const ::windows::runtime::GUID,
        piwiaeventcallback: ::windows::runtime::RawPtr,
        peventobject: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
        bstrdeviceid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        peventguid: *const ::windows::runtime::GUID,
        bstrfullappname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrcommandlinearg: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrdescription: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstricon: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
        bstrdeviceid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        peventguid: *const ::windows::runtime::GUID,
        pclsid: *const ::windows::runtime::GUID,
        bstrname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrdescription: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstricon: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
        bstrdeviceid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        hwndparent: super::super::Foundation::HWND,
        bstrfoldername: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrfilename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        plnumfiles: *mut i32,
        ppbstrfilepaths: *mut *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        ppitem: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IWiaDrvItem(::windows::runtime::IUnknown);
impl IWiaDrvItem {
    pub unsafe fn GetItemFlags(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn GetDeviceSpecContext(&self) -> ::windows::runtime::Result<*mut u8> {
        let mut result__: <*mut u8 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<*mut u8>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFullItemName(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetItemName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn AddItemToFolder<'a, Param0: ::windows::runtime::IntoParam<'a, IWiaDrvItem>>(
        &self,
        __midl__iwiadrvitem0004: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            __midl__iwiadrvitem0004.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn UnlinkItemTree(
        &self,
        __midl__iwiadrvitem0005: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(__midl__iwiadrvitem0005),
        )
        .ok()
    }
    pub unsafe fn RemoveItemFromFolder(
        &self,
        __midl__iwiadrvitem0006: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(__midl__iwiadrvitem0006),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindItemByName<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        __midl__iwiadrvitem0007: i32,
        __midl__iwiadrvitem0008: Param1,
    ) -> ::windows::runtime::Result<IWiaDrvItem> {
        let mut result__: <IWiaDrvItem as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(__midl__iwiadrvitem0007),
            __midl__iwiadrvitem0008.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IWiaDrvItem>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindChildItemByName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        __midl__iwiadrvitem0010: Param0,
    ) -> ::windows::runtime::Result<IWiaDrvItem> {
        let mut result__: <IWiaDrvItem as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            __midl__iwiadrvitem0010.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IWiaDrvItem>(result__)
    }
    pub unsafe fn GetParentItem(&self) -> ::windows::runtime::Result<IWiaDrvItem> {
        let mut result__: <IWiaDrvItem as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IWiaDrvItem>(result__)
    }
    pub unsafe fn GetFirstChildItem(&self) -> ::windows::runtime::Result<IWiaDrvItem> {
        let mut result__: <IWiaDrvItem as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IWiaDrvItem>(result__)
    }
    pub unsafe fn GetNextSiblingItem(&self) -> ::windows::runtime::Result<IWiaDrvItem> {
        let mut result__: <IWiaDrvItem as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IWiaDrvItem>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DumpItemData(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWiaDrvItem {
    type Vtable = IWiaDrvItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        520271301,
        45068,
        4562,
        [160, 148, 0, 192, 79, 114, 220, 60],
    );
}
impl ::std::convert::From<IWiaDrvItem> for ::windows::runtime::IUnknown {
    fn from(value: IWiaDrvItem) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWiaDrvItem> for ::windows::runtime::IUnknown {
    fn from(value: &IWiaDrvItem) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWiaDrvItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWiaDrvItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaDrvItem_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        __midl__iwiadrvitem0000: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        __midl__iwiadrvitem0001: *mut *mut u8,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        __midl__iwiadrvitem0002: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        __midl__iwiadrvitem0003: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        __midl__iwiadrvitem0004: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        __midl__iwiadrvitem0005: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        __midl__iwiadrvitem0006: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        __midl__iwiadrvitem0007: i32,
        __midl__iwiadrvitem0008: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        __midl__iwiadrvitem0009: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        __midl__iwiadrvitem0010: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        __midl__iwiadrvitem0011: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        __midl__iwiadrvitem0012: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        __midl__iwiadrvitem0013: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        __midl__iwiadrvitem0014: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        __midl__iwiadrvitem0015: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IWiaErrorHandler(::windows::runtime::IUnknown);
impl IWiaErrorHandler {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportStatus<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
        Param2: ::windows::runtime::IntoParam<'a, IWiaItem2>,
    >(
        &self,
        lflags: i32,
        hwndparent: Param1,
        pwiaitem2: Param2,
        hrstatus: ::windows::runtime::HRESULT,
        lpercentcomplete: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
            hwndparent.into_param().abi(),
            pwiaitem2.into_param().abi(),
            ::std::mem::transmute(hrstatus),
            ::std::mem::transmute(lpercentcomplete),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStatusDescription<'a, Param1: ::windows::runtime::IntoParam<'a, IWiaItem2>>(
        &self,
        lflags: i32,
        pwiaitem2: Param1,
        hrstatus: ::windows::runtime::HRESULT,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
            pwiaitem2.into_param().abi(),
            ::std::mem::transmute(hrstatus),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWiaErrorHandler {
    type Vtable = IWiaErrorHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        239751601,
        48159,
        17469,
        [168, 53, 114, 232, 144, 117, 158, 243],
    );
}
impl ::std::convert::From<IWiaErrorHandler> for ::windows::runtime::IUnknown {
    fn from(value: IWiaErrorHandler) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWiaErrorHandler> for ::windows::runtime::IUnknown {
    fn from(value: &IWiaErrorHandler) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWiaErrorHandler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWiaErrorHandler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaErrorHandler_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
        hwndparent: super::super::Foundation::HWND,
        pwiaitem2: ::windows::runtime::RawPtr,
        hrstatus: ::windows::runtime::HRESULT,
        lpercentcomplete: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
        pwiaitem2: ::windows::runtime::RawPtr,
        hrstatus: ::windows::runtime::HRESULT,
        pbstrdescription: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IWiaEventCallback(::windows::runtime::IUnknown);
impl IWiaEventCallback {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImageEventCallback<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        peventguid: *const ::windows::runtime::GUID,
        bstreventdescription: Param1,
        bstrdeviceid: Param2,
        bstrdevicedescription: Param3,
        dwdevicetype: u32,
        bstrfullitemname: Param5,
        puleventtype: *mut u32,
        ulreserved: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(peventguid),
            bstreventdescription.into_param().abi(),
            bstrdeviceid.into_param().abi(),
            bstrdevicedescription.into_param().abi(),
            ::std::mem::transmute(dwdevicetype),
            bstrfullitemname.into_param().abi(),
            ::std::mem::transmute(puleventtype),
            ::std::mem::transmute(ulreserved),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWiaEventCallback {
    type Vtable = IWiaEventCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2925692848,
        132,
        4562,
        [151, 59, 0, 160, 201, 6, 143, 46],
    );
}
impl ::std::convert::From<IWiaEventCallback> for ::windows::runtime::IUnknown {
    fn from(value: IWiaEventCallback) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWiaEventCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IWiaEventCallback) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWiaEventCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWiaEventCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaEventCallback_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        peventguid: *const ::windows::runtime::GUID,
        bstreventdescription: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrdeviceid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrdevicedescription: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        dwdevicetype: u32,
        bstrfullitemname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        puleventtype: *mut u32,
        ulreserved: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IWiaImageFilter(::windows::runtime::IUnknown);
impl IWiaImageFilter {
    pub unsafe fn InitializeFilter<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IWiaItem2>,
        Param1: ::windows::runtime::IntoParam<'a, IWiaTransferCallback>,
    >(
        &self,
        pwiaitem2: Param0,
        pwiatransfercallback: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pwiaitem2.into_param().abi(),
            pwiatransfercallback.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetNewCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IWiaTransferCallback>,
    >(
        &self,
        pwiatransfercallback: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            pwiatransfercallback.into_param().abi(),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn FilterPreviewImage<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, IWiaItem2>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::RECT>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>,
    >(
        &self,
        lflags: i32,
        pwiachilditem2: Param1,
        inputimageextents: Param2,
        pinputstream: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
            pwiachilditem2.into_param().abi(),
            inputimageextents.into_param().abi(),
            pinputstream.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn ApplyProperties<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IWiaPropertyStorage>,
    >(
        &self,
        pwiapropertystorage: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            pwiapropertystorage.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWiaImageFilter {
    type Vtable = IWiaImageFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2829557754,
        17675,
        16881,
        [143, 135, 132, 156, 205, 148, 235, 246],
    );
}
impl ::std::convert::From<IWiaImageFilter> for ::windows::runtime::IUnknown {
    fn from(value: IWiaImageFilter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWiaImageFilter> for ::windows::runtime::IUnknown {
    fn from(value: &IWiaImageFilter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWiaImageFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWiaImageFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaImageFilter_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwiaitem2: ::windows::runtime::RawPtr,
        pwiatransfercallback: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwiatransfercallback: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
        pwiachilditem2: ::windows::runtime::RawPtr,
        inputimageextents: super::super::Foundation::RECT,
        pinputstream: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pwiapropertystorage: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IWiaItem(::windows::runtime::IUnknown);
impl IWiaItem {
    pub unsafe fn GetItemType(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn AnalyzeItem(&self, lflags: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
        )
        .ok()
    }
    pub unsafe fn EnumChildItems(&self) -> ::windows::runtime::Result<IEnumWiaItem> {
        let mut result__: <IEnumWiaItem as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IEnumWiaItem>(result__)
    }
    pub unsafe fn DeleteItem(&self, lflags: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateChildItem<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        lflags: i32,
        bstritemname: Param1,
        bstrfullitemname: Param2,
    ) -> ::windows::runtime::Result<IWiaItem> {
        let mut result__: <IWiaItem as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
            bstritemname.into_param().abi(),
            bstrfullitemname.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IWiaItem>(result__)
    }
    pub unsafe fn EnumRegisterEventInfo(
        &self,
        lflags: i32,
        peventguid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<IEnumWIA_DEV_CAPS> {
        let mut result__: <IEnumWIA_DEV_CAPS as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
            ::std::mem::transmute(peventguid),
            &mut result__,
        )
        .from_abi::<IEnumWIA_DEV_CAPS>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindItemByName<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        lflags: i32,
        bstrfullitemname: Param1,
    ) -> ::windows::runtime::Result<IWiaItem> {
        let mut result__: <IWiaItem as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
            bstrfullitemname.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IWiaItem>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeviceDlg<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        hwndparent: Param0,
        lflags: i32,
        lintent: i32,
        plitemcount: *mut i32,
        ppiwiaitem: *mut *mut ::std::option::Option<IWiaItem>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            hwndparent.into_param().abi(),
            ::std::mem::transmute(lflags),
            ::std::mem::transmute(lintent),
            ::std::mem::transmute(plitemcount),
            ::std::mem::transmute(ppiwiaitem),
        )
        .ok()
    }
    pub unsafe fn DeviceCommand(
        &self,
        lflags: i32,
        pcmdguid: *const ::windows::runtime::GUID,
        piwiaitem: *mut ::std::option::Option<IWiaItem>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
            ::std::mem::transmute(pcmdguid),
            ::std::mem::transmute(piwiaitem),
        )
        .ok()
    }
    pub unsafe fn GetRootItem(&self) -> ::windows::runtime::Result<IWiaItem> {
        let mut result__: <IWiaItem as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IWiaItem>(result__)
    }
    pub unsafe fn EnumDeviceCapabilities(
        &self,
        lflags: i32,
    ) -> ::windows::runtime::Result<IEnumWIA_DEV_CAPS> {
        let mut result__: <IEnumWIA_DEV_CAPS as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
            &mut result__,
        )
        .from_abi::<IEnumWIA_DEV_CAPS>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DumpItemData(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DumpDrvItemData(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DumpTreeItemData(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Diagnostic(
        &self,
        ulsize: u32,
        pbuffer: *const u8,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ulsize),
            ::std::mem::transmute(pbuffer),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWiaItem {
    type Vtable = IWiaItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1303489808,
        13201,
        4562,
        [154, 51, 0, 192, 79, 163, 97, 69],
    );
}
impl ::std::convert::From<IWiaItem> for ::windows::runtime::IUnknown {
    fn from(value: IWiaItem) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWiaItem> for ::windows::runtime::IUnknown {
    fn from(value: &IWiaItem) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWiaItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWiaItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaItem_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pitemtype: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppienumwiaitem: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
        bstritemname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrfullitemname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        ppiwiaitem: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
        peventguid: *const ::windows::runtime::GUID,
        ppienum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
        bstrfullitemname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        ppiwiaitem: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hwndparent: super::super::Foundation::HWND,
        lflags: i32,
        lintent: i32,
        plitemcount: *mut i32,
        ppiwiaitem: *mut *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
        pcmdguid: *const ::windows::runtime::GUID,
        piwiaitem: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppiwiaitem: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
        ppienumwia_dev_caps: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrdata: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrdata: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrdata: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ulsize: u32,
        pbuffer: *const u8,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IWiaItem2(::windows::runtime::IUnknown);
impl IWiaItem2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateChildItem<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        litemflags: i32,
        lcreationflags: i32,
        bstritemname: Param2,
    ) -> ::windows::runtime::Result<IWiaItem2> {
        let mut result__: <IWiaItem2 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(litemflags),
            ::std::mem::transmute(lcreationflags),
            bstritemname.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IWiaItem2>(result__)
    }
    pub unsafe fn DeleteItem(&self, lflags: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
        )
        .ok()
    }
    pub unsafe fn EnumChildItems(
        &self,
        pcategoryguid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<IEnumWiaItem2> {
        let mut result__: <IEnumWiaItem2 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pcategoryguid),
            &mut result__,
        )
        .from_abi::<IEnumWiaItem2>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindItemByName<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        lflags: i32,
        bstrfullitemname: Param1,
    ) -> ::windows::runtime::Result<IWiaItem2> {
        let mut result__: <IWiaItem2 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
            bstrfullitemname.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IWiaItem2>(result__)
    }
    pub unsafe fn GetItemCategory(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::GUID>(result__)
    }
    pub unsafe fn GetItemType(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeviceDlg<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        lflags: i32,
        hwndparent: Param1,
        bstrfoldername: Param2,
        bstrfilename: Param3,
        plnumfiles: *mut i32,
        ppbstrfilepaths: *mut *mut super::super::Foundation::BSTR,
        ppitem: *mut ::std::option::Option<IWiaItem2>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
            hwndparent.into_param().abi(),
            bstrfoldername.into_param().abi(),
            bstrfilename.into_param().abi(),
            ::std::mem::transmute(plnumfiles),
            ::std::mem::transmute(ppbstrfilepaths),
            ::std::mem::transmute(ppitem),
        )
        .ok()
    }
    pub unsafe fn DeviceCommand(
        &self,
        lflags: i32,
        pcmdguid: *const ::windows::runtime::GUID,
        ppiwiaitem2: *mut ::std::option::Option<IWiaItem2>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
            ::std::mem::transmute(pcmdguid),
            ::std::mem::transmute(ppiwiaitem2),
        )
        .ok()
    }
    pub unsafe fn EnumDeviceCapabilities(
        &self,
        lflags: i32,
    ) -> ::windows::runtime::Result<IEnumWIA_DEV_CAPS> {
        let mut result__: <IEnumWIA_DEV_CAPS as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
            &mut result__,
        )
        .from_abi::<IEnumWIA_DEV_CAPS>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CheckExtension<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        lflags: i32,
        bstrname: Param1,
        riidextensioninterface: *const ::windows::runtime::GUID,
        pbextensionexists: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
            bstrname.into_param().abi(),
            ::std::mem::transmute(riidextensioninterface),
            ::std::mem::transmute(pbextensionexists),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetExtension<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        lflags: i32,
        bstrname: Param1,
        riidextensioninterface: *const ::windows::runtime::GUID,
        ppout: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
            bstrname.into_param().abi(),
            ::std::mem::transmute(riidextensioninterface),
            ::std::mem::transmute(ppout),
        )
        .ok()
    }
    pub unsafe fn GetParentItem(&self) -> ::windows::runtime::Result<IWiaItem2> {
        let mut result__: <IWiaItem2 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IWiaItem2>(result__)
    }
    pub unsafe fn GetRootItem(&self) -> ::windows::runtime::Result<IWiaItem2> {
        let mut result__: <IWiaItem2 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IWiaItem2>(result__)
    }
    pub unsafe fn GetPreviewComponent(
        &self,
        lflags: i32,
    ) -> ::windows::runtime::Result<IWiaPreview> {
        let mut result__: <IWiaPreview as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
            &mut result__,
        )
        .from_abi::<IWiaPreview>(result__)
    }
    pub unsafe fn EnumRegisterEventInfo(
        &self,
        lflags: i32,
        peventguid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<IEnumWIA_DEV_CAPS> {
        let mut result__: <IEnumWIA_DEV_CAPS as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
            ::std::mem::transmute(peventguid),
            &mut result__,
        )
        .from_abi::<IEnumWIA_DEV_CAPS>(result__)
    }
    pub unsafe fn Diagnostic(
        &self,
        ulsize: u32,
        pbuffer: *const u8,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ulsize),
            ::std::mem::transmute(pbuffer),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWiaItem2 {
    type Vtable = IWiaItem2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1824129141,
        4743,
        16509,
        [155, 119, 207, 14, 3, 4, 53, 204],
    );
}
impl ::std::convert::From<IWiaItem2> for ::windows::runtime::IUnknown {
    fn from(value: IWiaItem2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWiaItem2> for ::windows::runtime::IUnknown {
    fn from(value: &IWiaItem2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWiaItem2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWiaItem2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaItem2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        litemflags: i32,
        lcreationflags: i32,
        bstritemname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        ppiwiaitem2: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcategoryguid: *const ::windows::runtime::GUID,
        ppienumwiaitem2: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
        bstrfullitemname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        ppiwiaitem2: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pitemcategoryguid: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pitemtype: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
        hwndparent: super::super::Foundation::HWND,
        bstrfoldername: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrfilename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        plnumfiles: *mut i32,
        ppbstrfilepaths: *mut *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        ppitem: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
        pcmdguid: *const ::windows::runtime::GUID,
        ppiwiaitem2: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
        ppienumwia_dev_caps: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
        bstrname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        riidextensioninterface: *const ::windows::runtime::GUID,
        pbextensionexists: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
        bstrname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        riidextensioninterface: *const ::windows::runtime::GUID,
        ppout: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppiwiaitem2: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppiwiaitem2: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
        ppwiapreview: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
        peventguid: *const ::windows::runtime::GUID,
        ppienum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ulsize: u32,
        pbuffer: *const u8,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IWiaItemExtras(::windows::runtime::IUnknown);
impl IWiaItemExtras {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetExtendedErrorInfo(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Escape(
        &self,
        dwescapecode: u32,
        lpindata: *const u8,
        cbindatasize: u32,
        poutdata: *mut u8,
        dwoutdatasize: u32,
        pdwactualdatasize: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dwescapecode),
            ::std::mem::transmute(lpindata),
            ::std::mem::transmute(cbindatasize),
            ::std::mem::transmute(poutdata),
            ::std::mem::transmute(dwoutdatasize),
            ::std::mem::transmute(pdwactualdatasize),
        )
        .ok()
    }
    pub unsafe fn CancelPendingIO(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWiaItemExtras {
    type Vtable = IWiaItemExtras_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1653731116,
        14063,
        17714,
        [135, 106, 142, 19, 37, 147, 119, 141],
    );
}
impl ::std::convert::From<IWiaItemExtras> for ::windows::runtime::IUnknown {
    fn from(value: IWiaItemExtras) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWiaItemExtras> for ::windows::runtime::IUnknown {
    fn from(value: &IWiaItemExtras) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWiaItemExtras {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWiaItemExtras {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaItemExtras_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrerrortext: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dwescapecode: u32,
        lpindata: *const u8,
        cbindatasize: u32,
        poutdata: *mut u8,
        dwoutdatasize: u32,
        pdwactualdatasize: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IWiaLog(::windows::runtime::IUnknown);
impl IWiaLog {
    pub unsafe fn InitializeLog(&self, hinstance: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hinstance),
        )
        .ok()
    }
    pub unsafe fn hResult(
        &self,
        hresult: ::windows::runtime::HRESULT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hresult),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Log<
        'a,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        lflags: i32,
        lresid: i32,
        ldetail: i32,
        bstrtext: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
            ::std::mem::transmute(lresid),
            ::std::mem::transmute(ldetail),
            bstrtext.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWiaLog {
    type Vtable = IWiaLog_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2685145270,
        33441,
        17711,
        [139, 108, 134, 6, 42, 173, 104, 144],
    );
}
impl ::std::convert::From<IWiaLog> for ::windows::runtime::IUnknown {
    fn from(value: IWiaLog) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWiaLog> for ::windows::runtime::IUnknown {
    fn from(value: &IWiaLog) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWiaLog {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWiaLog {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaLog_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hinstance: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hresult: ::windows::runtime::HRESULT,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
        lresid: i32,
        ldetail: i32,
        bstrtext: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IWiaLogEx(::windows::runtime::IUnknown);
impl IWiaLogEx {
    pub unsafe fn InitializeLogEx(&self, hinstance: *const u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hinstance),
        )
        .ok()
    }
    pub unsafe fn hResult(
        &self,
        hresult: ::windows::runtime::HRESULT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hresult),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Log<
        'a,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        lflags: i32,
        lresid: i32,
        ldetail: i32,
        bstrtext: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
            ::std::mem::transmute(lresid),
            ::std::mem::transmute(ldetail),
            bstrtext.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn hResultEx(
        &self,
        lmethodid: i32,
        hresult: ::windows::runtime::HRESULT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lmethodid),
            ::std::mem::transmute(hresult),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LogEx<
        'a,
        Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        lmethodid: i32,
        lflags: i32,
        lresid: i32,
        ldetail: i32,
        bstrtext: Param4,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lmethodid),
            ::std::mem::transmute(lflags),
            ::std::mem::transmute(lresid),
            ::std::mem::transmute(ldetail),
            bstrtext.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWiaLogEx {
    type Vtable = IWiaLogEx_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2938053292,
        31296,
        18311,
        [180, 33, 174, 180, 122, 31, 189, 11],
    );
}
impl ::std::convert::From<IWiaLogEx> for ::windows::runtime::IUnknown {
    fn from(value: IWiaLogEx) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWiaLogEx> for ::windows::runtime::IUnknown {
    fn from(value: &IWiaLogEx) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWiaLogEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWiaLogEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaLogEx_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hinstance: *const u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hresult: ::windows::runtime::HRESULT,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
        lresid: i32,
        ldetail: i32,
        bstrtext: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lmethodid: i32,
        hresult: ::windows::runtime::HRESULT,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lmethodid: i32,
        lflags: i32,
        lresid: i32,
        ldetail: i32,
        bstrtext: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IWiaMiniDrv(::windows::runtime::IUnknown);
impl IWiaMiniDrv {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn drvInitializeWia<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param5: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        __midl__iwiaminidrv0000: *const u8,
        __midl__iwiaminidrv0001: i32,
        __midl__iwiaminidrv0002: Param2,
        __midl__iwiaminidrv0003: Param3,
        __midl__iwiaminidrv0004: Param4,
        __midl__iwiaminidrv0005: Param5,
        __midl__iwiaminidrv0006: *mut ::std::option::Option<IWiaDrvItem>,
        __midl__iwiaminidrv0007: *mut ::std::option::Option<::windows::runtime::IUnknown>,
        __midl__iwiaminidrv0008: *mut i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(__midl__iwiaminidrv0000),
            ::std::mem::transmute(__midl__iwiaminidrv0001),
            __midl__iwiaminidrv0002.into_param().abi(),
            __midl__iwiaminidrv0003.into_param().abi(),
            __midl__iwiaminidrv0004.into_param().abi(),
            __midl__iwiaminidrv0005.into_param().abi(),
            ::std::mem::transmute(__midl__iwiaminidrv0006),
            ::std::mem::transmute(__midl__iwiaminidrv0007),
            ::std::mem::transmute(__midl__iwiaminidrv0008),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn drvAcquireItemData(
        &self,
        __midl__iwiaminidrv0009: *const u8,
        __midl__iwiaminidrv0010: i32,
        __midl__iwiaminidrv0011: *mut MINIDRV_TRANSFER_CONTEXT,
        __midl__iwiaminidrv0012: *mut i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(__midl__iwiaminidrv0009),
            ::std::mem::transmute(__midl__iwiaminidrv0010),
            ::std::mem::transmute(__midl__iwiaminidrv0011),
            ::std::mem::transmute(__midl__iwiaminidrv0012),
        )
        .ok()
    }
    pub unsafe fn drvInitItemProperties(
        &self,
        __midl__iwiaminidrv0013: *const u8,
        __midl__iwiaminidrv0014: i32,
    ) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(__midl__iwiaminidrv0013),
            ::std::mem::transmute(__midl__iwiaminidrv0014),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com_StructuredStorage"
    ))]
    pub unsafe fn drvValidateItemProperties(
        &self,
        __midl__iwiaminidrv0016: *const u8,
        __midl__iwiaminidrv0017: i32,
        __midl__iwiaminidrv0018: u32,
        __midl__iwiaminidrv0019: *const super::super::System::Com::StructuredStorage::PROPSPEC,
    ) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(__midl__iwiaminidrv0016),
            ::std::mem::transmute(__midl__iwiaminidrv0017),
            ::std::mem::transmute(__midl__iwiaminidrv0018),
            ::std::mem::transmute(__midl__iwiaminidrv0019),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn drvWriteItemProperties(
        &self,
        __midl__iwiaminidrv0021: *const u8,
        __midl__iwiaminidrv0022: i32,
        __midl__iwiaminidrv0023: *const MINIDRV_TRANSFER_CONTEXT,
    ) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(__midl__iwiaminidrv0021),
            ::std::mem::transmute(__midl__iwiaminidrv0022),
            ::std::mem::transmute(__midl__iwiaminidrv0023),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com_StructuredStorage"
    ))]
    pub unsafe fn drvReadItemProperties(
        &self,
        __midl__iwiaminidrv0025: *const u8,
        __midl__iwiaminidrv0026: i32,
        __midl__iwiaminidrv0027: u32,
        __midl__iwiaminidrv0028: *const super::super::System::Com::StructuredStorage::PROPSPEC,
    ) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(__midl__iwiaminidrv0025),
            ::std::mem::transmute(__midl__iwiaminidrv0026),
            ::std::mem::transmute(__midl__iwiaminidrv0027),
            ::std::mem::transmute(__midl__iwiaminidrv0028),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn drvLockWiaDevice(
        &self,
        __midl__iwiaminidrv0030: *const u8,
        __midl__iwiaminidrv0031: i32,
    ) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(__midl__iwiaminidrv0030),
            ::std::mem::transmute(__midl__iwiaminidrv0031),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn drvUnLockWiaDevice(
        &self,
        __midl__iwiaminidrv0033: *const u8,
        __midl__iwiaminidrv0034: i32,
    ) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(__midl__iwiaminidrv0033),
            ::std::mem::transmute(__midl__iwiaminidrv0034),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn drvAnalyzeItem(
        &self,
        __midl__iwiaminidrv0036: *const u8,
        __midl__iwiaminidrv0037: i32,
        __midl__iwiaminidrv0038: *const i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(__midl__iwiaminidrv0036),
            ::std::mem::transmute(__midl__iwiaminidrv0037),
            ::std::mem::transmute(__midl__iwiaminidrv0038),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn drvGetDeviceErrorStr(
        &self,
        __midl__iwiaminidrv0039: i32,
        __midl__iwiaminidrv0040: i32,
        __midl__iwiaminidrv0041: *mut super::super::Foundation::PWSTR,
        __midl__iwiaminidrv0042: *mut i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(__midl__iwiaminidrv0039),
            ::std::mem::transmute(__midl__iwiaminidrv0040),
            ::std::mem::transmute(__midl__iwiaminidrv0041),
            ::std::mem::transmute(__midl__iwiaminidrv0042),
        )
        .ok()
    }
    pub unsafe fn drvDeviceCommand(
        &self,
        __midl__iwiaminidrv0043: *const u8,
        __midl__iwiaminidrv0044: i32,
        __midl__iwiaminidrv0045: *const ::windows::runtime::GUID,
        __midl__iwiaminidrv0046: *mut ::std::option::Option<IWiaDrvItem>,
        __midl__iwiaminidrv0047: *mut i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(__midl__iwiaminidrv0043),
            ::std::mem::transmute(__midl__iwiaminidrv0044),
            ::std::mem::transmute(__midl__iwiaminidrv0045),
            ::std::mem::transmute(__midl__iwiaminidrv0046),
            ::std::mem::transmute(__midl__iwiaminidrv0047),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn drvGetCapabilities(
        &self,
        __midl__iwiaminidrv0048: *const u8,
        __midl__iwiaminidrv0049: i32,
        __midl__iwiaminidrv0050: *mut i32,
        __midl__iwiaminidrv0051: *mut *mut WIA_DEV_CAP_DRV,
        __midl__iwiaminidrv0052: *mut i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(__midl__iwiaminidrv0048),
            ::std::mem::transmute(__midl__iwiaminidrv0049),
            ::std::mem::transmute(__midl__iwiaminidrv0050),
            ::std::mem::transmute(__midl__iwiaminidrv0051),
            ::std::mem::transmute(__midl__iwiaminidrv0052),
        )
        .ok()
    }
    pub unsafe fn drvDeleteItem(
        &self,
        __midl__iwiaminidrv0053: *const u8,
        __midl__iwiaminidrv0054: i32,
    ) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(__midl__iwiaminidrv0053),
            ::std::mem::transmute(__midl__iwiaminidrv0054),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn drvFreeDrvItemContext(
        &self,
        __midl__iwiaminidrv0056: i32,
        __midl__iwiaminidrv0057: *const u8,
    ) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(__midl__iwiaminidrv0056),
            ::std::mem::transmute(__midl__iwiaminidrv0057),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    pub unsafe fn drvGetWiaFormatInfo(
        &self,
        __midl__iwiaminidrv0059: *const u8,
        __midl__iwiaminidrv0060: i32,
        __midl__iwiaminidrv0061: *mut i32,
        __midl__iwiaminidrv0062: *mut *mut WIA_FORMAT_INFO,
        __midl__iwiaminidrv0063: *mut i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(__midl__iwiaminidrv0059),
            ::std::mem::transmute(__midl__iwiaminidrv0060),
            ::std::mem::transmute(__midl__iwiaminidrv0061),
            ::std::mem::transmute(__midl__iwiaminidrv0062),
            ::std::mem::transmute(__midl__iwiaminidrv0063),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn drvNotifyPnpEvent<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        peventguid: *const ::windows::runtime::GUID,
        bstrdeviceid: Param1,
        ulreserved: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(peventguid),
            bstrdeviceid.into_param().abi(),
            ::std::mem::transmute(ulreserved),
        )
        .ok()
    }
    pub unsafe fn drvUnInitializeWia(
        &self,
        __midl__iwiaminidrv0064: *const u8,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(__midl__iwiaminidrv0064),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWiaMiniDrv {
    type Vtable = IWiaMiniDrv_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3637374484,
        15468,
        4562,
        [154, 53, 0, 192, 79, 163, 97, 69],
    );
}
impl ::std::convert::From<IWiaMiniDrv> for ::windows::runtime::IUnknown {
    fn from(value: IWiaMiniDrv) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWiaMiniDrv> for ::windows::runtime::IUnknown {
    fn from(value: &IWiaMiniDrv) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWiaMiniDrv {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWiaMiniDrv {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaMiniDrv_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        __midl__iwiaminidrv0000: *const u8,
        __midl__iwiaminidrv0001: i32,
        __midl__iwiaminidrv0002: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        __midl__iwiaminidrv0003: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        __midl__iwiaminidrv0004: ::windows::runtime::RawPtr,
        __midl__iwiaminidrv0005: ::windows::runtime::RawPtr,
        __midl__iwiaminidrv0006: *mut ::windows::runtime::RawPtr,
        __midl__iwiaminidrv0007: *mut ::windows::runtime::RawPtr,
        __midl__iwiaminidrv0008: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        __midl__iwiaminidrv0009: *const u8,
        __midl__iwiaminidrv0010: i32,
        __midl__iwiaminidrv0011: *mut ::std::mem::ManuallyDrop<MINIDRV_TRANSFER_CONTEXT>,
        __midl__iwiaminidrv0012: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        __midl__iwiaminidrv0013: *const u8,
        __midl__iwiaminidrv0014: i32,
        __midl__iwiaminidrv0015: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com_StructuredStorage"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        __midl__iwiaminidrv0016: *const u8,
        __midl__iwiaminidrv0017: i32,
        __midl__iwiaminidrv0018: u32,
        __midl__iwiaminidrv0019: *const super::super::System::Com::StructuredStorage::PROPSPEC,
        __midl__iwiaminidrv0020: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com_StructuredStorage"
    )))]
    usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        __midl__iwiaminidrv0021: *const u8,
        __midl__iwiaminidrv0022: i32,
        __midl__iwiaminidrv0023: *const ::std::mem::ManuallyDrop<MINIDRV_TRANSFER_CONTEXT>,
        __midl__iwiaminidrv0024: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com_StructuredStorage"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        __midl__iwiaminidrv0025: *const u8,
        __midl__iwiaminidrv0026: i32,
        __midl__iwiaminidrv0027: u32,
        __midl__iwiaminidrv0028: *const super::super::System::Com::StructuredStorage::PROPSPEC,
        __midl__iwiaminidrv0029: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com_StructuredStorage"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        __midl__iwiaminidrv0030: *const u8,
        __midl__iwiaminidrv0031: i32,
        __midl__iwiaminidrv0032: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        __midl__iwiaminidrv0033: *const u8,
        __midl__iwiaminidrv0034: i32,
        __midl__iwiaminidrv0035: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        __midl__iwiaminidrv0036: *const u8,
        __midl__iwiaminidrv0037: i32,
        __midl__iwiaminidrv0038: *const i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        __midl__iwiaminidrv0039: i32,
        __midl__iwiaminidrv0040: i32,
        __midl__iwiaminidrv0041: *mut super::super::Foundation::PWSTR,
        __midl__iwiaminidrv0042: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        __midl__iwiaminidrv0043: *const u8,
        __midl__iwiaminidrv0044: i32,
        __midl__iwiaminidrv0045: *const ::windows::runtime::GUID,
        __midl__iwiaminidrv0046: *mut ::windows::runtime::RawPtr,
        __midl__iwiaminidrv0047: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        __midl__iwiaminidrv0048: *const u8,
        __midl__iwiaminidrv0049: i32,
        __midl__iwiaminidrv0050: *mut i32,
        __midl__iwiaminidrv0051: *mut *mut WIA_DEV_CAP_DRV,
        __midl__iwiaminidrv0052: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        __midl__iwiaminidrv0053: *const u8,
        __midl__iwiaminidrv0054: i32,
        __midl__iwiaminidrv0055: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        __midl__iwiaminidrv0056: i32,
        __midl__iwiaminidrv0057: *const u8,
        __midl__iwiaminidrv0058: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        __midl__iwiaminidrv0059: *const u8,
        __midl__iwiaminidrv0060: i32,
        __midl__iwiaminidrv0061: *mut i32,
        __midl__iwiaminidrv0062: *mut *mut WIA_FORMAT_INFO,
        __midl__iwiaminidrv0063: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        peventguid: *const ::windows::runtime::GUID,
        bstrdeviceid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        ulreserved: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        __midl__iwiaminidrv0064: *const u8,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IWiaMiniDrvCallBack(::windows::runtime::IUnknown);
impl IWiaMiniDrvCallBack {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MiniDrvCallback(
        &self,
        lreason: i32,
        lstatus: i32,
        lpercentcomplete: i32,
        loffset: i32,
        llength: i32,
        ptranctx: *const MINIDRV_TRANSFER_CONTEXT,
        lreserved: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lreason),
            ::std::mem::transmute(lstatus),
            ::std::mem::transmute(lpercentcomplete),
            ::std::mem::transmute(loffset),
            ::std::mem::transmute(llength),
            ::std::mem::transmute(ptranctx),
            ::std::mem::transmute(lreserved),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWiaMiniDrvCallBack {
    type Vtable = IWiaMiniDrvCallBack_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        866483546,
        15848,
        4562,
        [154, 54, 0, 192, 79, 163, 97, 69],
    );
}
impl ::std::convert::From<IWiaMiniDrvCallBack> for ::windows::runtime::IUnknown {
    fn from(value: IWiaMiniDrvCallBack) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWiaMiniDrvCallBack> for ::windows::runtime::IUnknown {
    fn from(value: &IWiaMiniDrvCallBack) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWiaMiniDrvCallBack {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWiaMiniDrvCallBack {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaMiniDrvCallBack_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lreason: i32,
        lstatus: i32,
        lpercentcomplete: i32,
        loffset: i32,
        llength: i32,
        ptranctx: *const ::std::mem::ManuallyDrop<MINIDRV_TRANSFER_CONTEXT>,
        lreserved: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IWiaMiniDrvTransferCallback(::windows::runtime::IUnknown);
impl IWiaMiniDrvTransferCallback {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetNextStream<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        lflags: i32,
        bstritemname: Param1,
        bstrfullitemname: Param2,
    ) -> ::windows::runtime::Result<super::super::System::Com::IStream> {
        let mut result__: <super::super::System::Com::IStream as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
            bstritemname.into_param().abi(),
            bstrfullitemname.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::System::Com::IStream>(result__)
    }
    pub unsafe fn SendMessage(
        &self,
        lflags: i32,
        pwiatransferparams: *const WiaTransferParams,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
            ::std::mem::transmute(pwiatransferparams),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWiaMiniDrvTransferCallback {
    type Vtable = IWiaMiniDrvTransferCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2849173129,
        11493,
        20464,
        [138, 219, 201, 97, 209, 215, 116, 202],
    );
}
impl ::std::convert::From<IWiaMiniDrvTransferCallback> for ::windows::runtime::IUnknown {
    fn from(value: IWiaMiniDrvTransferCallback) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWiaMiniDrvTransferCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IWiaMiniDrvTransferCallback) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IWiaMiniDrvTransferCallback
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IWiaMiniDrvTransferCallback
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaMiniDrvTransferCallback_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
        bstritemname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrfullitemname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        ppistream: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
        pwiatransferparams: *const WiaTransferParams,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IWiaNotifyDevMgr(::windows::runtime::IUnknown);
impl IWiaNotifyDevMgr {
    pub unsafe fn NewDeviceArrival(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWiaNotifyDevMgr {
    type Vtable = IWiaNotifyDevMgr_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1885871776,
        59327,
        17041,
        [159, 177, 78, 136, 19, 163, 247, 142],
    );
}
impl ::std::convert::From<IWiaNotifyDevMgr> for ::windows::runtime::IUnknown {
    fn from(value: IWiaNotifyDevMgr) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWiaNotifyDevMgr> for ::windows::runtime::IUnknown {
    fn from(value: &IWiaNotifyDevMgr) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWiaNotifyDevMgr {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWiaNotifyDevMgr {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaNotifyDevMgr_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IWiaPreview(::windows::runtime::IUnknown);
impl IWiaPreview {
    pub unsafe fn GetNewPreview<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, IWiaItem2>,
        Param2: ::windows::runtime::IntoParam<'a, IWiaTransferCallback>,
    >(
        &self,
        lflags: i32,
        pwiaitem2: Param1,
        pwiatransfercallback: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
            pwiaitem2.into_param().abi(),
            pwiatransfercallback.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn UpdatePreview<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, IWiaItem2>,
        Param2: ::windows::runtime::IntoParam<'a, IWiaTransferCallback>,
    >(
        &self,
        lflags: i32,
        pchildwiaitem2: Param1,
        pwiatransfercallback: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
            pchildwiaitem2.into_param().abi(),
            pwiatransfercallback.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn DetectRegions(&self, lflags: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
        )
        .ok()
    }
    pub unsafe fn Clear(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWiaPreview {
    type Vtable = IWiaPreview_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2512565501,
        13298,
        19846,
        [173, 64, 148, 49, 240, 223, 8, 247],
    );
}
impl ::std::convert::From<IWiaPreview> for ::windows::runtime::IUnknown {
    fn from(value: IWiaPreview) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWiaPreview> for ::windows::runtime::IUnknown {
    fn from(value: &IWiaPreview) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWiaPreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWiaPreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaPreview_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
        pwiaitem2: ::windows::runtime::RawPtr,
        pwiatransfercallback: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
        pchildwiaitem2: ::windows::runtime::RawPtr,
        pwiatransfercallback: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IWiaPropertyStorage(::windows::runtime::IUnknown);
impl IWiaPropertyStorage {
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Com_StructuredStorage",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn ReadMultiple(
        &self,
        cpspec: u32,
        rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC,
        rgpropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cpspec),
            ::std::mem::transmute(rgpspec),
            ::std::mem::transmute(rgpropvar),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Com_StructuredStorage",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn WriteMultiple(
        &self,
        cpspec: u32,
        rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC,
        rgpropvar: *const super::super::System::Com::StructuredStorage::PROPVARIANT,
        propidnamefirst: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cpspec),
            ::std::mem::transmute(rgpspec),
            ::std::mem::transmute(rgpropvar),
            ::std::mem::transmute(propidnamefirst),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com_StructuredStorage"
    ))]
    pub unsafe fn DeleteMultiple(
        &self,
        cpspec: u32,
        rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cpspec),
            ::std::mem::transmute(rgpspec),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReadPropertyNames(
        &self,
        cpropid: u32,
        rgpropid: *const u32,
        rglpwstrname: *mut super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cpropid),
            ::std::mem::transmute(rgpropid),
            ::std::mem::transmute(rglpwstrname),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WritePropertyNames(
        &self,
        cpropid: u32,
        rgpropid: *const u32,
        rglpwstrname: *const super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cpropid),
            ::std::mem::transmute(rgpropid),
            ::std::mem::transmute(rglpwstrname),
        )
        .ok()
    }
    pub unsafe fn DeletePropertyNames(
        &self,
        cpropid: u32,
        rgpropid: *const u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cpropid),
            ::std::mem::transmute(rgpropid),
        )
        .ok()
    }
    pub unsafe fn Commit(&self, grfcommitflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(grfcommitflags),
        )
        .ok()
    }
    pub unsafe fn Revert(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Enum(
        &self,
    ) -> ::windows::runtime::Result<super::super::System::Com::StructuredStorage::IEnumSTATPROPSTG>
    {
        let mut result__ : < super::super::System::Com::StructuredStorage:: IEnumSTATPROPSTG as :: windows :: runtime :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::System::Com::StructuredStorage::IEnumSTATPROPSTG>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTimes(
        &self,
        pctime: *const super::super::Foundation::FILETIME,
        patime: *const super::super::Foundation::FILETIME,
        pmtime: *const super::super::Foundation::FILETIME,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pctime),
            ::std::mem::transmute(patime),
            ::std::mem::transmute(pmtime),
        )
        .ok()
    }
    pub unsafe fn SetClass(
        &self,
        clsid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(clsid),
        )
        .ok()
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com_StructuredStorage"
    ))]
    pub unsafe fn Stat(
        &self,
    ) -> ::windows::runtime::Result<super::super::System::Com::StructuredStorage::STATPROPSETSTG>
    {
        let mut result__ : < super::super::System::Com::StructuredStorage:: STATPROPSETSTG as :: windows :: runtime :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::System::Com::StructuredStorage::STATPROPSETSTG>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Com_StructuredStorage",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn GetPropertyAttributes(
        &self,
        cpspec: u32,
        rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC,
        rgflags: *mut u32,
        rgpropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cpspec),
            ::std::mem::transmute(rgpspec),
            ::std::mem::transmute(rgflags),
            ::std::mem::transmute(rgpropvar),
        )
        .ok()
    }
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPropertyStream(
        &self,
        pcompatibilityid: *mut ::windows::runtime::GUID,
        ppistream: *mut ::std::option::Option<super::super::System::Com::IStream>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pcompatibilityid),
            ::std::mem::transmute(ppistream),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPropertyStream<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>,
    >(
        &self,
        pcompatibilityid: *mut ::windows::runtime::GUID,
        pistream: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pcompatibilityid),
            pistream.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWiaPropertyStorage {
    type Vtable = IWiaPropertyStorage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2562058400,
        10700,
        18714,
        [170, 192, 230, 219, 79, 220, 206, 182],
    );
}
impl ::std::convert::From<IWiaPropertyStorage> for ::windows::runtime::IUnknown {
    fn from(value: IWiaPropertyStorage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWiaPropertyStorage> for ::windows::runtime::IUnknown {
    fn from(value: &IWiaPropertyStorage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWiaPropertyStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWiaPropertyStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaPropertyStorage_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Com_StructuredStorage",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cpspec: u32,
        rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC,
        rgpropvar: *mut ::std::mem::ManuallyDrop<
            super::super::System::Com::StructuredStorage::PROPVARIANT,
        >,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Com_StructuredStorage",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Com_StructuredStorage",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cpspec: u32,
        rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC,
        rgpropvar: *const ::std::mem::ManuallyDrop<
            super::super::System::Com::StructuredStorage::PROPVARIANT,
        >,
        propidnamefirst: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Com_StructuredStorage",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com_StructuredStorage"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cpspec: u32,
        rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com_StructuredStorage"
    )))]
    usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cpropid: u32,
        rgpropid: *const u32,
        rglpwstrname: *mut super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cpropid: u32,
        rgpropid: *const u32,
        rglpwstrname: *const super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cpropid: u32,
        rgpropid: *const u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        grfcommitflags: u32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pctime: *const super::super::Foundation::FILETIME,
        patime: *const super::super::Foundation::FILETIME,
        pmtime: *const super::super::Foundation::FILETIME,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        clsid: *const ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com_StructuredStorage"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pstatpsstg: *mut super::super::System::Com::StructuredStorage::STATPROPSETSTG,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com_StructuredStorage"
    )))]
    usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Com_StructuredStorage",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cpspec: u32,
        rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC,
        rgflags: *mut u32,
        rgpropvar: *mut ::std::mem::ManuallyDrop<
            super::super::System::Com::StructuredStorage::PROPVARIANT,
        >,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Com_StructuredStorage",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pulnumprops: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcompatibilityid: *mut ::windows::runtime::GUID,
        ppistream: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pcompatibilityid: *mut ::windows::runtime::GUID,
        pistream: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IWiaSegmentationFilter(::windows::runtime::IUnknown);
impl IWiaSegmentationFilter {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DetectRegions<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>,
        Param2: ::windows::runtime::IntoParam<'a, IWiaItem2>,
    >(
        &self,
        lflags: i32,
        pinputstream: Param1,
        pwiaitem2: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
            pinputstream.into_param().abi(),
            pwiaitem2.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWiaSegmentationFilter {
    type Vtable = IWiaSegmentationFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3964053143,
        44036,
        17479,
        [143, 101, 255, 99, 213, 21, 75, 33],
    );
}
impl ::std::convert::From<IWiaSegmentationFilter> for ::windows::runtime::IUnknown {
    fn from(value: IWiaSegmentationFilter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWiaSegmentationFilter> for ::windows::runtime::IUnknown {
    fn from(value: &IWiaSegmentationFilter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IWiaSegmentationFilter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IWiaSegmentationFilter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaSegmentationFilter_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
        pinputstream: ::windows::runtime::RawPtr,
        pwiaitem2: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IWiaTransfer(::windows::runtime::IUnknown);
impl IWiaTransfer {
    pub unsafe fn Download<'a, Param1: ::windows::runtime::IntoParam<'a, IWiaTransferCallback>>(
        &self,
        lflags: i32,
        piwiatransfercallback: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
            piwiatransfercallback.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Upload<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>,
        Param2: ::windows::runtime::IntoParam<'a, IWiaTransferCallback>,
    >(
        &self,
        lflags: i32,
        psource: Param1,
        piwiatransfercallback: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
            psource.into_param().abi(),
            piwiatransfercallback.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn EnumWIA_FORMAT_INFO(&self) -> ::windows::runtime::Result<IEnumWIA_FORMAT_INFO> {
        let mut result__: <IEnumWIA_FORMAT_INFO as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IEnumWIA_FORMAT_INFO>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWiaTransfer {
    type Vtable = IWiaTransfer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3281873218,
        12110,
        19716,
        [146, 254, 78, 244, 211, 161, 222, 90],
    );
}
impl ::std::convert::From<IWiaTransfer> for ::windows::runtime::IUnknown {
    fn from(value: IWiaTransfer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWiaTransfer> for ::windows::runtime::IUnknown {
    fn from(value: &IWiaTransfer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWiaTransfer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWiaTransfer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaTransfer_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
        piwiatransfercallback: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
        psource: ::windows::runtime::RawPtr,
        piwiatransfercallback: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppenum: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IWiaTransferCallback(::windows::runtime::IUnknown);
impl IWiaTransferCallback {
    pub unsafe fn TransferCallback(
        &self,
        lflags: i32,
        pwiatransferparams: *const WiaTransferParams,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
            ::std::mem::transmute(pwiatransferparams),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetNextStream<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        lflags: i32,
        bstritemname: Param1,
        bstrfullitemname: Param2,
    ) -> ::windows::runtime::Result<super::super::System::Com::IStream> {
        let mut result__: <super::super::System::Com::IStream as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lflags),
            bstritemname.into_param().abi(),
            bstrfullitemname.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::System::Com::IStream>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWiaTransferCallback {
    type Vtable = IWiaTransferCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        668265135,
        10406,
        19621,
        [154, 171, 230, 120, 22, 139, 149, 39],
    );
}
impl ::std::convert::From<IWiaTransferCallback> for ::windows::runtime::IUnknown {
    fn from(value: IWiaTransferCallback) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWiaTransferCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IWiaTransferCallback) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWiaTransferCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWiaTransferCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaTransferCallback_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
        pwiatransferparams: *const WiaTransferParams,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lflags: i32,
        bstritemname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bstrfullitemname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        ppdestination: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IWiaUIExtension(::windows::runtime::IUnknown);
impl IWiaUIExtension {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeviceDialog(
        &self,
        pdevicedialogdata: *const DEVICEDIALOGDATA,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pdevicedialogdata),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn GetDeviceIcon<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrdeviceid: Param0,
        phicon: *mut super::super::UI::WindowsAndMessaging::HICON,
        nsize: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            bstrdeviceid.into_param().abi(),
            ::std::mem::transmute(phicon),
            ::std::mem::transmute(nsize),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDeviceBitmapLogo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrdeviceid: Param0,
        phbitmap: *mut super::super::Graphics::Gdi::HBITMAP,
        nmaxwidth: u32,
        nmaxheight: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            bstrdeviceid.into_param().abi(),
            ::std::mem::transmute(phbitmap),
            ::std::mem::transmute(nmaxwidth),
            ::std::mem::transmute(nmaxheight),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWiaUIExtension {
    type Vtable = IWiaUIExtension_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3660681491,
        20718,
        19584,
        [180, 96, 87, 208, 5, 212, 74, 44],
    );
}
impl ::std::convert::From<IWiaUIExtension> for ::windows::runtime::IUnknown {
    fn from(value: IWiaUIExtension) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWiaUIExtension> for ::windows::runtime::IUnknown {
    fn from(value: &IWiaUIExtension) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWiaUIExtension {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWiaUIExtension {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaUIExtension_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevicedialogdata: *const ::std::mem::ManuallyDrop<DEVICEDIALOGDATA>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrdeviceid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        phicon: *mut super::super::UI::WindowsAndMessaging::HICON,
        nsize: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrdeviceid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        phbitmap: *mut super::super::Graphics::Gdi::HBITMAP,
        nmaxwidth: u32,
        nmaxheight: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IWiaUIExtension2(::windows::runtime::IUnknown);
impl IWiaUIExtension2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeviceDialog(
        &self,
        pdevicedialogdata: *const DEVICEDIALOGDATA2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pdevicedialogdata),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn GetDeviceIcon<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrdeviceid: Param0,
        phicon: *mut super::super::UI::WindowsAndMessaging::HICON,
        nsize: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            bstrdeviceid.into_param().abi(),
            ::std::mem::transmute(phicon),
            ::std::mem::transmute(nsize),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWiaUIExtension2 {
    type Vtable = IWiaUIExtension2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        810942679,
        20616,
        18135,
        [154, 21, 183, 123, 9, 205, 186, 122],
    );
}
impl ::std::convert::From<IWiaUIExtension2> for ::windows::runtime::IUnknown {
    fn from(value: IWiaUIExtension2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWiaUIExtension2> for ::windows::runtime::IUnknown {
    fn from(value: &IWiaUIExtension2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWiaUIExtension2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWiaUIExtension2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaUIExtension2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pdevicedialogdata: *const ::std::mem::ManuallyDrop<DEVICEDIALOGDATA2>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrdeviceid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        phicon: *mut super::super::UI::WindowsAndMessaging::HICON,
        nsize: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IWiaVideo(::windows::runtime::IUnknown);
impl IWiaVideo {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PreviewVisible(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPreviewVisible<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        bpreviewvisible: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            bpreviewvisible.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImagesDirectory(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetImagesDirectory<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrimagedirectory: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            bstrimagedirectory.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVideoByWiaDevID<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        bstrwiadeviceid: Param0,
        hwndparent: Param1,
        bstretchtofitparent: Param2,
        bautobeginplayback: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            bstrwiadeviceid.into_param().abi(),
            hwndparent.into_param().abi(),
            bstretchtofitparent.into_param().abi(),
            bautobeginplayback.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVideoByDevNum<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        uidevicenumber: u32,
        hwndparent: Param1,
        bstretchtofitparent: Param2,
        bautobeginplayback: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(uidevicenumber),
            hwndparent.into_param().abi(),
            bstretchtofitparent.into_param().abi(),
            bautobeginplayback.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVideoByName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        bstrfriendlyname: Param0,
        hwndparent: Param1,
        bstretchtofitparent: Param2,
        bautobeginplayback: Param3,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            bstrfriendlyname.into_param().abi(),
            hwndparent.into_param().abi(),
            bstretchtofitparent.into_param().abi(),
            bautobeginplayback.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn DestroyVideo(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Play(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Pause(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TakePicture(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ResizeVideo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        bstretchtofitparent: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            bstretchtofitparent.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetCurrentState(&self) -> ::windows::runtime::Result<WIAVIDEO_STATE> {
        let mut result__: <WIAVIDEO_STATE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<WIAVIDEO_STATE>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWiaVideo {
    type Vtable = IWiaVideo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3576242346,
        56200,
        16880,
        [148, 108, 224, 13, 192, 161, 156, 250],
    );
}
impl ::std::convert::From<IWiaVideo> for ::windows::runtime::IUnknown {
    fn from(value: IWiaVideo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWiaVideo> for ::windows::runtime::IUnknown {
    fn from(value: &IWiaVideo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWiaVideo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWiaVideo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaVideo_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbpreviewvisible: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bpreviewvisible: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrimagedirectory: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrimagedirectory: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrwiadeviceid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        hwndparent: super::super::Foundation::HWND,
        bstretchtofitparent: super::super::Foundation::BOOL,
        bautobeginplayback: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        uidevicenumber: u32,
        hwndparent: super::super::Foundation::HWND,
        bstretchtofitparent: super::super::Foundation::BOOL,
        bautobeginplayback: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrfriendlyname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        hwndparent: super::super::Foundation::HWND,
        bstretchtofitparent: super::super::Foundation::BOOL,
        bautobeginplayback: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pbstrnewimagefilename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstretchtofitparent: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pstate: *mut WIAVIDEO_STATE,
    ) -> ::windows::runtime::HRESULT,
);
pub const LAMP_ERR: u32 = 2048u32;
pub const LANDSCAPE: u32 = 1u32;
pub const LANSCAPE: u32 = 1u32;
pub const LEFT_JUSTIFIED: u32 = 0u32;
pub const LIGHT_SOURCE_DETECT_READY: u32 = 4u32;
pub const LIGHT_SOURCE_NEGATIVE: u32 = 4u32;
pub const LIGHT_SOURCE_POSITIVE: u32 = 2u32;
pub const LIGHT_SOURCE_PRESENT: u32 = 2u32;
pub const LIGHT_SOURCE_PRESENT_DETECT: u32 = 1u32;
pub const LIGHT_SOURCE_READY: u32 = 8u32;
pub const LIGHT_SOURCE_SELECT: u32 = 1u32;
pub const MAX_ANSI_CHAR: u32 = 255u32;
pub const MAX_IO_HANDLES: u32 = 16u32;
pub const MAX_RESERVED: u32 = 4u32;
pub const MCRO_ERROR_GENERAL_ERROR: u32 = 0u32;
pub const MCRO_ERROR_OFFLINE: u32 = 5u32;
pub const MCRO_ERROR_PAPER_EMPTY: u32 = 4u32;
pub const MCRO_ERROR_PAPER_JAM: u32 = 2u32;
pub const MCRO_ERROR_PAPER_PROBLEM: u32 = 3u32;
pub const MCRO_ERROR_USER_INTERVENTION: u32 = 6u32;
pub const MCRO_STATUS_OK: u32 = 1u32;
pub const MICR_READER: u32 = 1048576u32;
pub const MICR_READER_READY: u32 = 65536u32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MINIDRV_TRANSFER_CONTEXT {
    pub lSize: i32,
    pub lWidthInPixels: i32,
    pub lLines: i32,
    pub lDepth: i32,
    pub lXRes: i32,
    pub lYRes: i32,
    pub lCompression: i32,
    pub guidFormatID: ::windows::runtime::GUID,
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
    pub pIWiaMiniDrvCallBack: ::std::option::Option<IWiaMiniDrvCallBack>,
    pub lImageSize: i32,
    pub lHeaderSize: i32,
    pub lItemSize: i32,
    pub cbWidthInBytes: i32,
    pub lPage: i32,
    pub lCurIfdOffset: i32,
    pub lPrevIfdOffset: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl MINIDRV_TRANSFER_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MINIDRV_TRANSFER_CONTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MINIDRV_TRANSFER_CONTEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MINIDRV_TRANSFER_CONTEXT")
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
impl ::std::cmp::PartialEq for MINIDRV_TRANSFER_CONTEXT {
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
impl ::std::cmp::Eq for MINIDRV_TRANSFER_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MINIDRV_TRANSFER_CONTEXT {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
pub const MIRRORED: u32 = 1u32;
pub const MULTIPLE_FEED: u32 = 512u32;
pub const NEXT_PAGE: u32 = 128u32;
pub const PAPER_JAM: u32 = 32u32;
pub const PATCH_CODE_READER: u32 = 524288u32;
pub const PATCH_CODE_READER_READY: u32 = 32768u32;
pub const PATH_COVER_UP: u32 = 16u32;
pub const PORTRAIT: u32 = 0u32;
pub const POWERMODE_BATTERY: u32 = 2u32;
pub const POWERMODE_LINE: u32 = 1u32;
pub const PREFEED: u32 = 256u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RANGEVALUE {
    pub lMin: i32,
    pub lMax: i32,
    pub lStep: i32,
}
impl RANGEVALUE {}
impl ::std::default::Default for RANGEVALUE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RANGEVALUE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RANGEVALUE")
            .field("lMin", &self.lMin)
            .field("lMax", &self.lMax)
            .field("lStep", &self.lStep)
            .finish()
    }
}
impl ::std::cmp::PartialEq for RANGEVALUE {
    fn eq(&self, other: &Self) -> bool {
        self.lMin == other.lMin && self.lMax == other.lMax && self.lStep == other.lStep
    }
}
impl ::std::cmp::Eq for RANGEVALUE {}
unsafe impl ::windows::runtime::Abi for RANGEVALUE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const RIGHT_JUSTIFIED: u32 = 2u32;
pub const ROT180: u32 = 2u32;
pub const ROT270: u32 = 3u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
    pub pMicroDriverContext: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl SCANINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SCANINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SCANINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SCANINFO")
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
impl ::std::cmp::PartialEq for SCANINFO {
    fn eq(&self, other: &Self) -> bool {
        self.ADF == other.ADF
            && self.TPA == other.TPA
            && self.Endorser == other.Endorser
            && self.OpticalXResolution == other.OpticalXResolution
            && self.OpticalYResolution == other.OpticalYResolution
            && self.BedWidth == other.BedWidth
            && self.BedHeight == other.BedHeight
            && self.IntensityRange == other.IntensityRange
            && self.ContrastRange == other.ContrastRange
            && self.SupportedCompressionType == other.SupportedCompressionType
            && self.SupportedDataTypes == other.SupportedDataTypes
            && self.WidthPixels == other.WidthPixels
            && self.WidthBytes == other.WidthBytes
            && self.Lines == other.Lines
            && self.DataType == other.DataType
            && self.PixelBits == other.PixelBits
            && self.Intensity == other.Intensity
            && self.Contrast == other.Contrast
            && self.Xresolution == other.Xresolution
            && self.Yresolution == other.Yresolution
            && self.Window == other.Window
            && self.DitherPattern == other.DitherPattern
            && self.Negative == other.Negative
            && self.Mirror == other.Mirror
            && self.AutoBack == other.AutoBack
            && self.ColorDitherPattern == other.ColorDitherPattern
            && self.ToneMap == other.ToneMap
            && self.Compression == other.Compression
            && self.RawDataFormat == other.RawDataFormat
            && self.RawPixelOrder == other.RawPixelOrder
            && self.bNeedDataAlignment == other.bNeedDataAlignment
            && self.DelayBetweenRead == other.DelayBetweenRead
            && self.MaxBufferSize == other.MaxBufferSize
            && self.DeviceIOHandles == other.DeviceIOHandles
            && self.lReserved == other.lReserved
            && self.pMicroDriverContext == other.pMicroDriverContext
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SCANINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SCANINFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SCANMODE_FINALSCAN: u32 = 0u32;
pub const SCANMODE_PREVIEWSCAN: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SCANWINDOW {
    pub xPos: i32,
    pub yPos: i32,
    pub xExtent: i32,
    pub yExtent: i32,
}
impl SCANWINDOW {}
impl ::std::default::Default for SCANWINDOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SCANWINDOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SCANWINDOW")
            .field("xPos", &self.xPos)
            .field("yPos", &self.yPos)
            .field("xExtent", &self.xExtent)
            .field("yExtent", &self.yExtent)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SCANWINDOW {
    fn eq(&self, other: &Self) -> bool {
        self.xPos == other.xPos
            && self.yPos == other.yPos
            && self.xExtent == other.xExtent
            && self.yExtent == other.yExtent
    }
}
impl ::std::cmp::Eq for SCANWINDOW {}
unsafe impl ::windows::runtime::Abi for SCANWINDOW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SCAN_FINISHED: u32 = 30u32;
pub const SCAN_FIRST: u32 = 10u32;
pub const SCAN_NEXT: u32 = 20u32;
pub const STOR: u32 = 2048u32;
pub const STORAGE_FULL: u32 = 256u32;
pub const STORAGE_READY: u32 = 128u32;
pub const SUPPORT_BW: u32 = 2u32;
pub const SUPPORT_COLOR: u32 = 1u32;
pub const SUPPORT_GRAYSCALE: u32 = 4u32;
pub const TOP_JUSTIFIED: u32 = 0u32;
pub const TRANSPARENCY_DYNAMIC_FRAME_SUPPORT: u32 = 1u32;
pub const TRANSPARENCY_STATIC_FRAME_SUPPORT: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl TWAIN_CAPABILITY {}
impl ::std::default::Default for TWAIN_CAPABILITY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TWAIN_CAPABILITY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TWAIN_CAPABILITY")
            .field("lSize", &self.lSize)
            .field("lMSG", &self.lMSG)
            .field("lCapID", &self.lCapID)
            .field("lConType", &self.lConType)
            .field("lRC", &self.lRC)
            .field("lCC", &self.lCC)
            .field("lDataSize", &self.lDataSize)
            .field("Data", &self.Data)
            .finish()
    }
}
impl ::std::cmp::PartialEq for TWAIN_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        self.lSize == other.lSize
            && self.lMSG == other.lMSG
            && self.lCapID == other.lCapID
            && self.lConType == other.lConType
            && self.lRC == other.lRC
            && self.lCC == other.lCC
            && self.lDataSize == other.lDataSize
            && self.Data == other.Data
    }
}
impl ::std::cmp::Eq for TWAIN_CAPABILITY {}
unsafe impl ::windows::runtime::Abi for TWAIN_CAPABILITY {
    type Abi = Self;
    type DefaultType = Self;
}
pub const TYMED_CALLBACK: u32 = 128u32;
pub const TYMED_MULTIPAGE_CALLBACK: u32 = 512u32;
pub const TYMED_MULTIPAGE_FILE: u32 = 256u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VAL {
    pub lVal: i32,
    pub dblVal: f64,
    pub pGuid: *mut ::windows::runtime::GUID,
    pub pScanInfo: *mut SCANINFO,
    pub handle: isize,
    pub ppButtonNames: *mut *mut u16,
    pub pHandle: *mut super::super::Foundation::HANDLE,
    pub lReserved: i32,
    pub szVal: [super::super::Foundation::CHAR; 255],
}
#[cfg(feature = "Win32_Foundation")]
impl VAL {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for VAL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for VAL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VAL")
            .field("lVal", &self.lVal)
            .field("dblVal", &self.dblVal)
            .field("pGuid", &self.pGuid)
            .field("pScanInfo", &self.pScanInfo)
            .field("handle", &self.handle)
            .field("ppButtonNames", &self.ppButtonNames)
            .field("pHandle", &self.pHandle)
            .field("lReserved", &self.lReserved)
            .field("szVal", &self.szVal)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for VAL {
    fn eq(&self, other: &Self) -> bool {
        self.lVal == other.lVal
            && self.dblVal == other.dblVal
            && self.pGuid == other.pGuid
            && self.pScanInfo == other.pScanInfo
            && self.handle == other.handle
            && self.ppButtonNames == other.ppButtonNames
            && self.pHandle == other.pHandle
            && self.lReserved == other.lReserved
            && self.szVal == other.szVal
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for VAL {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for VAL {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WHITEBALANCE_AUTO: u32 = 2u32;
pub const WHITEBALANCE_DAYLIGHT: u32 = 4u32;
pub const WHITEBALANCE_FLASH: u32 = 7u32;
pub const WHITEBALANCE_FLORESCENT: u32 = 5u32;
pub const WHITEBALANCE_MANUAL: u32 = 1u32;
pub const WHITEBALANCE_ONEPUSH_AUTO: u32 = 3u32;
pub const WHITEBALANCE_TUNGSTEN: u32 = 6u32;
#[cfg(feature = "Win32_Foundation")]
impl ::std::clone::Clone for WIAS_CHANGED_VALUE_INFO {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WIAS_CHANGED_VALUE_INFO {
    pub bChanged: super::super::Foundation::BOOL,
    pub vt: i32,
    pub Old: WIAS_CHANGED_VALUE_INFO_1,
    pub Current: WIAS_CHANGED_VALUE_INFO_0,
}
#[cfg(feature = "Win32_Foundation")]
impl WIAS_CHANGED_VALUE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WIAS_CHANGED_VALUE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WIAS_CHANGED_VALUE_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WIAS_CHANGED_VALUE_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WIAS_CHANGED_VALUE_INFO {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::clone::Clone for WIAS_CHANGED_VALUE_INFO_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union WIAS_CHANGED_VALUE_INFO_0 {
    pub lVal: i32,
    pub fltVal: f32,
    pub bstrVal: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    pub guidVal: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl WIAS_CHANGED_VALUE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WIAS_CHANGED_VALUE_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WIAS_CHANGED_VALUE_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WIAS_CHANGED_VALUE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WIAS_CHANGED_VALUE_INFO_0 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::clone::Clone for WIAS_CHANGED_VALUE_INFO_1 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union WIAS_CHANGED_VALUE_INFO_1 {
    pub lVal: i32,
    pub fltVal: f32,
    pub bstrVal: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    pub guidVal: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl WIAS_CHANGED_VALUE_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WIAS_CHANGED_VALUE_INFO_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WIAS_CHANGED_VALUE_INFO_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WIAS_CHANGED_VALUE_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WIAS_CHANGED_VALUE_INFO_1 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl WIAS_DOWN_SAMPLE_INFO {}
impl ::std::default::Default for WIAS_DOWN_SAMPLE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WIAS_DOWN_SAMPLE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIAS_DOWN_SAMPLE_INFO")
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
impl ::std::cmp::PartialEq for WIAS_DOWN_SAMPLE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ulOriginalWidth == other.ulOriginalWidth
            && self.ulOriginalHeight == other.ulOriginalHeight
            && self.ulBitsPerPixel == other.ulBitsPerPixel
            && self.ulXRes == other.ulXRes
            && self.ulYRes == other.ulYRes
            && self.ulDownSampledWidth == other.ulDownSampledWidth
            && self.ulDownSampledHeight == other.ulDownSampledHeight
            && self.ulActualSize == other.ulActualSize
            && self.ulDestBufSize == other.ulDestBufSize
            && self.ulSrcBufSize == other.ulSrcBufSize
            && self.pSrcBuffer == other.pSrcBuffer
            && self.pDestBuffer == other.pDestBuffer
    }
}
impl ::std::cmp::Eq for WIAS_DOWN_SAMPLE_INFO {}
unsafe impl ::windows::runtime::Abi for WIAS_DOWN_SAMPLE_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WIAS_ENDORSER_INFO {
    pub ulPageCount: u32,
    pub ulNumEndorserValues: u32,
    pub pEndorserValues: *mut WIAS_ENDORSER_VALUE,
}
#[cfg(feature = "Win32_Foundation")]
impl WIAS_ENDORSER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WIAS_ENDORSER_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WIAS_ENDORSER_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIAS_ENDORSER_INFO")
            .field("ulPageCount", &self.ulPageCount)
            .field("ulNumEndorserValues", &self.ulNumEndorserValues)
            .field("pEndorserValues", &self.pEndorserValues)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WIAS_ENDORSER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ulPageCount == other.ulPageCount
            && self.ulNumEndorserValues == other.ulNumEndorserValues
            && self.pEndorserValues == other.pEndorserValues
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WIAS_ENDORSER_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WIAS_ENDORSER_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WIAS_ENDORSER_VALUE {
    pub wszTokenName: super::super::Foundation::PWSTR,
    pub wszValue: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WIAS_ENDORSER_VALUE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WIAS_ENDORSER_VALUE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WIAS_ENDORSER_VALUE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIAS_ENDORSER_VALUE")
            .field("wszTokenName", &self.wszTokenName)
            .field("wszValue", &self.wszValue)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WIAS_ENDORSER_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.wszTokenName == other.wszTokenName && self.wszValue == other.wszValue
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WIAS_ENDORSER_VALUE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WIAS_ENDORSER_VALUE {
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
pub struct WIAVIDEO_STATE(pub i32);
pub const WIAVIDEO_NO_VIDEO: WIAVIDEO_STATE = WIAVIDEO_STATE(1i32);
pub const WIAVIDEO_CREATING_VIDEO: WIAVIDEO_STATE = WIAVIDEO_STATE(2i32);
pub const WIAVIDEO_VIDEO_CREATED: WIAVIDEO_STATE = WIAVIDEO_STATE(3i32);
pub const WIAVIDEO_VIDEO_PLAYING: WIAVIDEO_STATE = WIAVIDEO_STATE(4i32);
pub const WIAVIDEO_VIDEO_PAUSED: WIAVIDEO_STATE = WIAVIDEO_STATE(5i32);
pub const WIAVIDEO_DESTROYING_VIDEO: WIAVIDEO_STATE = WIAVIDEO_STATE(6i32);
impl ::std::convert::From<i32> for WIAVIDEO_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WIAVIDEO_STATE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WIA_ACTION_EVENT: u32 = 2u32;
pub const WIA_ADVANCED_PREVIEW: u32 = 0u32;
pub const WIA_ALARM_BEEP1: u32 = 1u32;
pub const WIA_ALARM_BEEP10: u32 = 10u32;
pub const WIA_ALARM_BEEP2: u32 = 2u32;
pub const WIA_ALARM_BEEP3: u32 = 3u32;
pub const WIA_ALARM_BEEP4: u32 = 4u32;
pub const WIA_ALARM_BEEP5: u32 = 5u32;
pub const WIA_ALARM_BEEP6: u32 = 6u32;
pub const WIA_ALARM_BEEP7: u32 = 7u32;
pub const WIA_ALARM_BEEP8: u32 = 8u32;
pub const WIA_ALARM_BEEP9: u32 = 9u32;
pub const WIA_ALARM_NONE: u32 = 0u32;
pub const WIA_AUTO_CROP_DISABLED: u32 = 0u32;
pub const WIA_AUTO_CROP_MULTI: u32 = 2u32;
pub const WIA_AUTO_CROP_SINGLE: u32 = 1u32;
pub const WIA_AUTO_DESKEW_OFF: u32 = 1u32;
pub const WIA_AUTO_DESKEW_ON: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WIA_BARCODES {
    pub Tag: u32,
    pub Version: u32,
    pub Size: u32,
    pub Count: u32,
    pub Barcodes: [WIA_BARCODE_INFO; 1],
}
impl WIA_BARCODES {}
impl ::std::default::Default for WIA_BARCODES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WIA_BARCODES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIA_BARCODES")
            .field("Tag", &self.Tag)
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .field("Count", &self.Count)
            .field("Barcodes", &self.Barcodes)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WIA_BARCODES {
    fn eq(&self, other: &Self) -> bool {
        self.Tag == other.Tag
            && self.Version == other.Version
            && self.Size == other.Size
            && self.Count == other.Count
            && self.Barcodes == other.Barcodes
    }
}
impl ::std::cmp::Eq for WIA_BARCODES {}
unsafe impl ::windows::runtime::Abi for WIA_BARCODES {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WIA_BARCODE_AUTO_SEARCH: u32 = 4u32;
pub const WIA_BARCODE_AZTEC: u32 = 36u32;
pub const WIA_BARCODE_CODABAR: u32 = 2u32;
pub const WIA_BARCODE_CODE128: u32 = 9u32;
pub const WIA_BARCODE_CODE128A: u32 = 10u32;
pub const WIA_BARCODE_CODE128B: u32 = 11u32;
pub const WIA_BARCODE_CODE128C: u32 = 12u32;
pub const WIA_BARCODE_CODE39: u32 = 5u32;
pub const WIA_BARCODE_CODE39_FULLASCII: u32 = 7u32;
pub const WIA_BARCODE_CODE39_MOD43: u32 = 6u32;
pub const WIA_BARCODE_CODE93: u32 = 8u32;
pub const WIA_BARCODE_CPCBINARY: u32 = 29u32;
pub const WIA_BARCODE_CUSTOMBASE: u32 = 32768u32;
pub const WIA_BARCODE_DATAMATRIX: u32 = 38u32;
pub const WIA_BARCODE_DATASTRIP: u32 = 39u32;
pub const WIA_BARCODE_EAN13: u32 = 17u32;
pub const WIA_BARCODE_EAN8: u32 = 16u32;
pub const WIA_BARCODE_EZCODE: u32 = 40u32;
pub const WIA_BARCODE_FIM: u32 = 30u32;
pub const WIA_BARCODE_GS1128: u32 = 13u32;
pub const WIA_BARCODE_GS1DATABAR: u32 = 14u32;
pub const WIA_BARCODE_HIGH_CAPACITY_COLOR: u32 = 26u32;
pub const WIA_BARCODE_HORIZONTAL_SEARCH: u32 = 0u32;
pub const WIA_BARCODE_HORIZONTAL_VERTICAL_SEARCH: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl WIA_BARCODE_INFO {}
impl ::std::default::Default for WIA_BARCODE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WIA_BARCODE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIA_BARCODE_INFO")
            .field("Size", &self.Size)
            .field("Type", &self.Type)
            .field("Page", &self.Page)
            .field("Confidence", &self.Confidence)
            .field("XOffset", &self.XOffset)
            .field("YOffset", &self.YOffset)
            .field("Rotation", &self.Rotation)
            .field("Length", &self.Length)
            .field("Text", &self.Text)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WIA_BARCODE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Type == other.Type
            && self.Page == other.Page
            && self.Confidence == other.Confidence
            && self.XOffset == other.XOffset
            && self.YOffset == other.YOffset
            && self.Rotation == other.Rotation
            && self.Length == other.Length
            && self.Text == other.Text
    }
}
impl ::std::cmp::Eq for WIA_BARCODE_INFO {}
unsafe impl ::windows::runtime::Abi for WIA_BARCODE_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WIA_BARCODE_INTELLIGENT_MAIL: u32 = 23u32;
pub const WIA_BARCODE_INTERLEAVED_2OF5: u32 = 4u32;
pub const WIA_BARCODE_ITF14: u32 = 15u32;
pub const WIA_BARCODE_JAN: u32 = 34u32;
pub const WIA_BARCODE_MAXICODE: u32 = 27u32;
pub const WIA_BARCODE_MSI: u32 = 33u32;
pub const WIA_BARCODE_NONINTERLEAVED_2OF5: u32 = 3u32;
pub const WIA_BARCODE_PDF417: u32 = 28u32;
pub const WIA_BARCODE_PHARMACODE: u32 = 31u32;
pub const WIA_BARCODE_PLANET: u32 = 22u32;
pub const WIA_BARCODE_PLESSEY: u32 = 32u32;
pub const WIA_BARCODE_POSTBAR: u32 = 24u32;
pub const WIA_BARCODE_POSTNETA: u32 = 18u32;
pub const WIA_BARCODE_POSTNETB: u32 = 19u32;
pub const WIA_BARCODE_POSTNETC: u32 = 20u32;
pub const WIA_BARCODE_POSTNET_DPBC: u32 = 21u32;
pub const WIA_BARCODE_QRCODE: u32 = 41u32;
pub const WIA_BARCODE_READER_AUTO: u32 = 1u32;
pub const WIA_BARCODE_READER_DISABLED: u32 = 0u32;
pub const WIA_BARCODE_READER_FEEDER_BACK: u32 = 4u32;
pub const WIA_BARCODE_READER_FEEDER_DUPLEX: u32 = 5u32;
pub const WIA_BARCODE_READER_FEEDER_FRONT: u32 = 3u32;
pub const WIA_BARCODE_READER_FLATBED: u32 = 2u32;
pub const WIA_BARCODE_RM4SCC: u32 = 25u32;
pub const WIA_BARCODE_SHOTCODE: u32 = 42u32;
pub const WIA_BARCODE_SMALLAZTEC: u32 = 37u32;
pub const WIA_BARCODE_SPARQCODE: u32 = 43u32;
pub const WIA_BARCODE_TELEPEN: u32 = 35u32;
pub const WIA_BARCODE_UPCA: u32 = 0u32;
pub const WIA_BARCODE_UPCE: u32 = 1u32;
pub const WIA_BARCODE_VERTICAL_HORIZONTAL_SEARCH: u32 = 3u32;
pub const WIA_BARCODE_VERTICAL_SEARCH: u32 = 1u32;
pub const WIA_BASIC_PREVIEW: u32 = 1u32;
pub const WIA_BLANK_PAGE_DETECTION_DISABLED: u32 = 0u32;
pub const WIA_BLANK_PAGE_DISCARD: u32 = 1u32;
pub const WIA_BLANK_PAGE_JOB_SEPARATOR: u32 = 2u32;
pub const WIA_CATEGORY_AUTO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3741212632,
    27799,
    19934,
    [177, 30, 203, 80, 155, 39, 14, 17],
);
pub const WIA_CATEGORY_BARCODE_READER: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        920746144,
        18239,
        18763,
        [175, 143, 108, 63, 109, 116, 134, 252],
    );
pub const WIA_CATEGORY_ENDORSER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1192242371,
    4735,
    18289,
    [173, 252, 153, 26, 184, 238, 30, 151],
);
pub const WIA_CATEGORY_FEEDER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    4262664500,
    63564,
    17069,
    [141, 164, 97, 41, 205, 221, 114, 136],
);
pub const WIA_CATEGORY_FEEDER_BACK: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1640658132,
        14811,
        17066,
        [137, 177, 140, 25, 201, 205, 76, 35],
    );
pub const WIA_CATEGORY_FEEDER_FRONT: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1210259292,
        15144,
        18555,
        [167, 230, 238, 188, 23, 97, 79, 209],
    );
pub const WIA_CATEGORY_FILM: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    4244003815,
    15587,
    17523,
    [175, 133, 245, 211, 125, 33, 182, 138],
);
pub const WIA_CATEGORY_FINISHED_FILE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        4281038794,
        53124,
        17195,
        [167, 53, 58, 19, 13, 222, 42, 136],
    );
pub const WIA_CATEGORY_FLATBED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    4217404191,
    17395,
    18571,
    [133, 91, 251, 112, 62, 195, 66, 166],
);
pub const WIA_CATEGORY_FOLDER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3331499078,
    28506,
    18461,
    [133, 187, 146, 226, 232, 111, 211, 10],
);
pub const WIA_CATEGORY_IMPRINTER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    4234477933,
    37378,
    17373,
    [145, 167, 100, 194, 149, 76, 251, 139],
);
pub const WIA_CATEGORY_MICR_READER: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        998687212,
        29116,
        17989,
        [180, 213, 27, 25, 218, 43, 233, 120],
    );
pub const WIA_CATEGORY_PATCH_CODE_READER: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2410289773,
        40074,
        17101,
        [152, 179, 238, 151, 0, 203, 199, 79],
    );
pub const WIA_CATEGORY_ROOT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    4052963951,
    22968,
    18982,
    [152, 136, 225, 110, 79, 151, 206, 16],
);
pub const WIA_CMD_BUILD_DEVICE_TREE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2629459168,
        56298,
        4562,
        [132, 22, 0, 192, 79, 163, 97, 69],
    );
pub const WIA_CMD_CHANGE_DOCUMENT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    82257328,
    44206,
    4562,
    [160, 147, 0, 192, 79, 114, 220, 60],
);
pub const WIA_CMD_DELETE_ALL_ITEMS: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3792224624,
        44205,
        4562,
        [160, 147, 0, 192, 79, 114, 220, 60],
    );
pub const WIA_CMD_DELETE_DEVICE_TREE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1937856834,
        56298,
        4562,
        [132, 22, 0, 192, 79, 163, 97, 69],
    );
pub const WIA_CMD_DIAGNOSTIC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    285168373,
    56836,
    19696,
    [165, 173, 105, 31, 141, 206, 1, 65],
);
pub const WIA_CMD_FORMAT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3282473898,
    63368,
    19764,
    [165, 176, 190, 113, 144, 117, 154, 36],
);
pub const WIA_CMD_PAUSE_FEEDER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1352162893,
    42418,
    19313,
    [156, 149, 109, 125, 124, 70, 154, 67],
);
pub const WIA_CMD_START_FEEDER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1520301769,
    24365,
    19001,
    [157, 108, 0, 69, 109, 4, 127, 0],
);
pub const WIA_CMD_STOP_FEEDER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3628576877,
    14597,
    17820,
    [149, 9, 155, 41, 205, 182, 145, 231],
);
pub const WIA_CMD_SYNCHRONIZE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2603005874,
    44205,
    4562,
    [160, 147, 0, 192, 79, 114, 220, 60],
);
pub const WIA_CMD_TAKE_PICTURE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2945662124,
    44205,
    4562,
    [160, 147, 0, 192, 79, 114, 220, 60],
);
pub const WIA_CMD_UNLOAD_DOCUMENT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    523976078,
    44206,
    4562,
    [160, 147, 0, 192, 79, 114, 220, 60],
);
pub const WIA_COLOR_DROP_BLUE: u32 = 3u32;
pub const WIA_COLOR_DROP_DISABLED: u32 = 0u32;
pub const WIA_COLOR_DROP_GREEN: u32 = 2u32;
pub const WIA_COLOR_DROP_RED: u32 = 1u32;
pub const WIA_COLOR_DROP_RGB: u32 = 4u32;
pub const WIA_COMPRESSION_AUTO: u32 = 100u32;
pub const WIA_COMPRESSION_BI_RLE4: u32 = 1u32;
pub const WIA_COMPRESSION_BI_RLE8: u32 = 2u32;
pub const WIA_COMPRESSION_G3: u32 = 3u32;
pub const WIA_COMPRESSION_G4: u32 = 4u32;
pub const WIA_COMPRESSION_JBIG: u32 = 6u32;
pub const WIA_COMPRESSION_JPEG: u32 = 5u32;
pub const WIA_COMPRESSION_JPEG2K: u32 = 7u32;
pub const WIA_COMPRESSION_NONE: u32 = 0u32;
pub const WIA_COMPRESSION_PNG: u32 = 8u32;
pub const WIA_DATA_AUTO: u32 = 100u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WIA_DATA_CALLBACK_HEADER {
    pub lSize: i32,
    pub guidFormatID: ::windows::runtime::GUID,
    pub lBufferSize: i32,
    pub lPageCount: i32,
}
impl WIA_DATA_CALLBACK_HEADER {}
impl ::std::default::Default for WIA_DATA_CALLBACK_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WIA_DATA_CALLBACK_HEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIA_DATA_CALLBACK_HEADER")
            .field("lSize", &self.lSize)
            .field("guidFormatID", &self.guidFormatID)
            .field("lBufferSize", &self.lBufferSize)
            .field("lPageCount", &self.lPageCount)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WIA_DATA_CALLBACK_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.lSize == other.lSize
            && self.guidFormatID == other.guidFormatID
            && self.lBufferSize == other.lBufferSize
            && self.lPageCount == other.lPageCount
    }
}
impl ::std::cmp::Eq for WIA_DATA_CALLBACK_HEADER {}
unsafe impl ::windows::runtime::Abi for WIA_DATA_CALLBACK_HEADER {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WIA_DATA_COLOR: u32 = 3u32;
pub const WIA_DATA_COLOR_DITHER: u32 = 5u32;
pub const WIA_DATA_COLOR_THRESHOLD: u32 = 4u32;
pub const WIA_DATA_DITHER: u32 = 1u32;
pub const WIA_DATA_GRAYSCALE: u32 = 2u32;
pub const WIA_DATA_RAW_BGR: u32 = 7u32;
pub const WIA_DATA_RAW_CMY: u32 = 10u32;
pub const WIA_DATA_RAW_CMYK: u32 = 11u32;
pub const WIA_DATA_RAW_RGB: u32 = 6u32;
pub const WIA_DATA_RAW_YUV: u32 = 8u32;
pub const WIA_DATA_RAW_YUVK: u32 = 9u32;
pub const WIA_DATA_THRESHOLD: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl WIA_DATA_TRANSFER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WIA_DATA_TRANSFER_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WIA_DATA_TRANSFER_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIA_DATA_TRANSFER_INFO")
            .field("ulSize", &self.ulSize)
            .field("ulSection", &self.ulSection)
            .field("ulBufferSize", &self.ulBufferSize)
            .field("bDoubleBuffer", &self.bDoubleBuffer)
            .field("ulReserved1", &self.ulReserved1)
            .field("ulReserved2", &self.ulReserved2)
            .field("ulReserved3", &self.ulReserved3)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WIA_DATA_TRANSFER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ulSize == other.ulSize
            && self.ulSection == other.ulSection
            && self.ulBufferSize == other.ulBufferSize
            && self.bDoubleBuffer == other.bDoubleBuffer
            && self.ulReserved1 == other.ulReserved1
            && self.ulReserved2 == other.ulReserved2
            && self.ulReserved3 == other.ulReserved3
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WIA_DATA_TRANSFER_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WIA_DATA_TRANSFER_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WIA_DEPTH_AUTO: u32 = 0u32;
pub const WIA_DEVICE_COMMANDS: u32 = 1u32;
pub const WIA_DEVICE_CONNECTED: u32 = 1u32;
pub const WIA_DEVICE_DIALOG_SINGLE_IMAGE: u32 = 2u32;
pub const WIA_DEVICE_DIALOG_USE_COMMON_UI: u32 = 4u32;
pub const WIA_DEVICE_EVENTS: u32 = 2u32;
pub const WIA_DEVICE_NOT_CONNECTED: u32 = 0u32;
pub const WIA_DEVINFO_ENUM_ALL: u32 = 15u32;
pub const WIA_DEVINFO_ENUM_LOCAL: u32 = 16u32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WIA_DEV_CAP {
    pub guid: ::windows::runtime::GUID,
    pub ulFlags: u32,
    pub bstrName: super::super::Foundation::BSTR,
    pub bstrDescription: super::super::Foundation::BSTR,
    pub bstrIcon: super::super::Foundation::BSTR,
    pub bstrCommandline: super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WIA_DEV_CAP {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WIA_DEV_CAP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WIA_DEV_CAP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIA_DEV_CAP")
            .field("guid", &self.guid)
            .field("ulFlags", &self.ulFlags)
            .field("bstrName", &self.bstrName)
            .field("bstrDescription", &self.bstrDescription)
            .field("bstrIcon", &self.bstrIcon)
            .field("bstrCommandline", &self.bstrCommandline)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WIA_DEV_CAP {
    fn eq(&self, other: &Self) -> bool {
        self.guid == other.guid
            && self.ulFlags == other.ulFlags
            && self.bstrName == other.bstrName
            && self.bstrDescription == other.bstrDescription
            && self.bstrIcon == other.bstrIcon
            && self.bstrCommandline == other.bstrCommandline
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WIA_DEV_CAP {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WIA_DEV_CAP {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WIA_DEV_CAP_DRV {
    pub guid: *mut ::windows::runtime::GUID,
    pub ulFlags: u32,
    pub wszName: super::super::Foundation::PWSTR,
    pub wszDescription: super::super::Foundation::PWSTR,
    pub wszIcon: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WIA_DEV_CAP_DRV {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WIA_DEV_CAP_DRV {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WIA_DEV_CAP_DRV {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIA_DEV_CAP_DRV")
            .field("guid", &self.guid)
            .field("ulFlags", &self.ulFlags)
            .field("wszName", &self.wszName)
            .field("wszDescription", &self.wszDescription)
            .field("wszIcon", &self.wszIcon)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WIA_DEV_CAP_DRV {
    fn eq(&self, other: &Self) -> bool {
        self.guid == other.guid
            && self.ulFlags == other.ulFlags
            && self.wszName == other.wszName
            && self.wszDescription == other.wszDescription
            && self.wszIcon == other.wszIcon
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WIA_DEV_CAP_DRV {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WIA_DEV_CAP_DRV {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WIA_DIP_BAUDRATE: u32 = 12u32;
pub const WIA_DIP_DEV_DESC: u32 = 4u32;
pub const WIA_DIP_DEV_ID: u32 = 2u32;
pub const WIA_DIP_DEV_NAME: u32 = 7u32;
pub const WIA_DIP_DEV_TYPE: u32 = 5u32;
pub const WIA_DIP_DRIVER_VERSION: u32 = 15u32;
pub const WIA_DIP_FIRST: u32 = 2u32;
pub const WIA_DIP_HW_CONFIG: u32 = 11u32;
pub const WIA_DIP_PNP_ID: u32 = 16u32;
pub const WIA_DIP_PORT_NAME: u32 = 6u32;
pub const WIA_DIP_REMOTE_DEV_ID: u32 = 9u32;
pub const WIA_DIP_SERVER_NAME: u32 = 8u32;
pub const WIA_DIP_STI_DRIVER_VERSION: u32 = 17u32;
pub const WIA_DIP_STI_GEN_CAPABILITIES: u32 = 13u32;
pub const WIA_DIP_UI_CLSID: u32 = 10u32;
pub const WIA_DIP_VEND_DESC: u32 = 3u32;
pub const WIA_DIP_WIA_VERSION: u32 = 14u32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
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
impl WIA_DITHER_PATTERN_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WIA_DITHER_PATTERN_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WIA_DITHER_PATTERN_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIA_DITHER_PATTERN_DATA")
            .field("lSize", &self.lSize)
            .field("bstrPatternName", &self.bstrPatternName)
            .field("lPatternWidth", &self.lPatternWidth)
            .field("lPatternLength", &self.lPatternLength)
            .field("cbPattern", &self.cbPattern)
            .field("pbPattern", &self.pbPattern)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WIA_DITHER_PATTERN_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.lSize == other.lSize
            && self.bstrPatternName == other.bstrPatternName
            && self.lPatternWidth == other.lPatternWidth
            && self.lPatternLength == other.lPatternLength
            && self.cbPattern == other.cbPattern
            && self.pbPattern == other.pbPattern
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WIA_DITHER_PATTERN_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WIA_DITHER_PATTERN_DATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
pub const WIA_DONT_SHOW_PREVIEW_CONTROL: u32 = 1u32;
pub const WIA_DONT_USE_SEGMENTATION_FILTER: u32 = 1u32;
pub const WIA_DPA_CONNECT_STATUS: u32 = 1027u32;
pub const WIA_DPA_DEVICE_TIME: u32 = 1028u32;
pub const WIA_DPA_FIRMWARE_VERSION: u32 = 1026u32;
pub const WIA_DPC_ARTIST: u32 = 2091u32;
pub const WIA_DPC_BATTERY_STATUS: u32 = 2065u32;
pub const WIA_DPC_BURST_INTERVAL: u32 = 2075u32;
pub const WIA_DPC_BURST_NUMBER: u32 = 2076u32;
pub const WIA_DPC_CAPTURE_DELAY: u32 = 2082u32;
pub const WIA_DPC_CAPTURE_MODE: u32 = 2081u32;
pub const WIA_DPC_COMPRESSION_SETTING: u32 = 2071u32;
pub const WIA_DPC_CONTRAST: u32 = 2080u32;
pub const WIA_DPC_COPYRIGHT_INFO: u32 = 2092u32;
pub const WIA_DPC_DIGITAL_ZOOM: u32 = 2078u32;
pub const WIA_DPC_DIMENSION: u32 = 2070u32;
pub const WIA_DPC_EFFECT_MODE: u32 = 2077u32;
pub const WIA_DPC_EXPOSURE_COMP: u32 = 2053u32;
pub const WIA_DPC_EXPOSURE_INDEX: u32 = 2083u32;
pub const WIA_DPC_EXPOSURE_METERING_MODE: u32 = 2084u32;
pub const WIA_DPC_EXPOSURE_MODE: u32 = 2052u32;
pub const WIA_DPC_EXPOSURE_TIME: u32 = 2054u32;
pub const WIA_DPC_FLASH_MODE: u32 = 2056u32;
pub const WIA_DPC_FNUMBER: u32 = 2055u32;
pub const WIA_DPC_FOCAL_LENGTH: u32 = 2087u32;
pub const WIA_DPC_FOCUS_DISTANCE: u32 = 2086u32;
pub const WIA_DPC_FOCUS_MANUAL_DIST: u32 = 2058u32;
pub const WIA_DPC_FOCUS_METERING: u32 = 2072u32;
pub const WIA_DPC_FOCUS_METERING_MODE: u32 = 2085u32;
pub const WIA_DPC_FOCUS_MODE: u32 = 2057u32;
pub const WIA_DPC_PAN_POSITION: u32 = 2060u32;
pub const WIA_DPC_PICTURES_REMAINING: u32 = 2051u32;
pub const WIA_DPC_PICTURES_TAKEN: u32 = 2050u32;
pub const WIA_DPC_PICT_HEIGHT: u32 = 2069u32;
pub const WIA_DPC_PICT_WIDTH: u32 = 2068u32;
pub const WIA_DPC_POWER_MODE: u32 = 2064u32;
pub const WIA_DPC_RGB_GAIN: u32 = 2088u32;
pub const WIA_DPC_SHARPNESS: u32 = 2079u32;
pub const WIA_DPC_THUMB_HEIGHT: u32 = 2067u32;
pub const WIA_DPC_THUMB_WIDTH: u32 = 2066u32;
pub const WIA_DPC_TILT_POSITION: u32 = 2061u32;
pub const WIA_DPC_TIMELAPSE_INTERVAL: u32 = 2073u32;
pub const WIA_DPC_TIMELAPSE_NUMBER: u32 = 2074u32;
pub const WIA_DPC_TIMER_MODE: u32 = 2062u32;
pub const WIA_DPC_TIMER_VALUE: u32 = 2063u32;
pub const WIA_DPC_UPLOAD_URL: u32 = 2090u32;
pub const WIA_DPC_WHITE_BALANCE: u32 = 2089u32;
pub const WIA_DPC_ZOOM_POSITION: u32 = 2059u32;
pub const WIA_DPF_FIRST: u32 = 3330u32;
pub const WIA_DPF_MOUNT_POINT: u32 = 3330u32;
pub const WIA_DPS_DEVICE_ID: u32 = 3114u32;
pub const WIA_DPS_DITHER_PATTERN_DATA: u32 = 3085u32;
pub const WIA_DPS_DITHER_SELECT: u32 = 3084u32;
pub const WIA_DPS_DOCUMENT_HANDLING_CAPABILITIES: u32 = 3086u32;
pub const WIA_DPS_DOCUMENT_HANDLING_CAPACITY: u32 = 3089u32;
pub const WIA_DPS_DOCUMENT_HANDLING_SELECT: u32 = 3088u32;
pub const WIA_DPS_DOCUMENT_HANDLING_STATUS: u32 = 3087u32;
pub const WIA_DPS_ENDORSER_CHARACTERS: u32 = 3092u32;
pub const WIA_DPS_ENDORSER_STRING: u32 = 3093u32;
pub const WIA_DPS_FILTER_SELECT: u32 = 3083u32;
pub const WIA_DPS_FIRST: u32 = 3074u32;
pub const WIA_DPS_GLOBAL_IDENTITY: u32 = 3115u32;
pub const WIA_DPS_HORIZONTAL_BED_REGISTRATION: u32 = 3079u32;
pub const WIA_DPS_HORIZONTAL_BED_SIZE: u32 = 3074u32;
pub const WIA_DPS_HORIZONTAL_SHEET_FEED_SIZE: u32 = 3076u32;
pub const WIA_DPS_MAX_SCAN_TIME: u32 = 3095u32;
pub const WIA_DPS_MIN_HORIZONTAL_SHEET_FEED_SIZE: u32 = 3104u32;
pub const WIA_DPS_MIN_VERTICAL_SHEET_FEED_SIZE: u32 = 3105u32;
pub const WIA_DPS_OPTICAL_XRES: u32 = 3090u32;
pub const WIA_DPS_OPTICAL_YRES: u32 = 3091u32;
pub const WIA_DPS_PAD_COLOR: u32 = 3082u32;
pub const WIA_DPS_PAGES: u32 = 3096u32;
pub const WIA_DPS_PAGE_HEIGHT: u32 = 3099u32;
pub const WIA_DPS_PAGE_SIZE: u32 = 3097u32;
pub const WIA_DPS_PAGE_WIDTH: u32 = 3098u32;
pub const WIA_DPS_PLATEN_COLOR: u32 = 3081u32;
pub const WIA_DPS_PREVIEW: u32 = 3100u32;
pub const WIA_DPS_SCAN_AHEAD_PAGES: u32 = 3094u32;
pub const WIA_DPS_SCAN_AVAILABLE_ITEM: u32 = 3116u32;
pub const WIA_DPS_SERVICE_ID: u32 = 3113u32;
pub const WIA_DPS_SHEET_FEEDER_REGISTRATION: u32 = 3078u32;
pub const WIA_DPS_SHOW_PREVIEW_CONTROL: u32 = 3103u32;
pub const WIA_DPS_TRANSPARENCY: u32 = 3101u32;
pub const WIA_DPS_TRANSPARENCY_CAPABILITIES: u32 = 3106u32;
pub const WIA_DPS_TRANSPARENCY_SELECT: u32 = 3102u32;
pub const WIA_DPS_TRANSPARENCY_STATUS: u32 = 3107u32;
pub const WIA_DPS_USER_NAME: u32 = 3112u32;
pub const WIA_DPS_VERTICAL_BED_REGISTRATION: u32 = 3080u32;
pub const WIA_DPS_VERTICAL_BED_SIZE: u32 = 3075u32;
pub const WIA_DPS_VERTICAL_SHEET_FEED_SIZE: u32 = 3077u32;
pub const WIA_DPV_DSHOW_DEVICE_PATH: u32 = 3588u32;
pub const WIA_DPV_IMAGES_DIRECTORY: u32 = 3587u32;
pub const WIA_DPV_LAST_PICTURE_TAKEN: u32 = 3586u32;
pub const WIA_ERROR_BUSY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2145320954i32 as _);
pub const WIA_ERROR_COVER_OPEN: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2145320944i32 as _);
pub const WIA_ERROR_DESTINATION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2145320942i32 as _);
pub const WIA_ERROR_DEVICE_COMMUNICATION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2145320950i32 as _);
pub const WIA_ERROR_DEVICE_LOCKED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2145320947i32 as _);
pub const WIA_ERROR_EXCEPTION_IN_DRIVER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2145320946i32 as _);
pub const WIA_ERROR_GENERAL_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2145320959i32 as _);
pub const WIA_ERROR_INCORRECT_HARDWARE_SETTING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2145320948i32 as _);
pub const WIA_ERROR_INVALID_COMMAND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2145320949i32 as _);
pub const WIA_ERROR_INVALID_DRIVER_RESPONSE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2145320945i32 as _);
pub const WIA_ERROR_ITEM_DELETED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2145320951i32 as _);
pub const WIA_ERROR_LAMP_OFF: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2145320943i32 as _);
pub const WIA_ERROR_MAXIMUM_PRINTER_ENDORSER_COUNTER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2145320939i32 as _);
pub const WIA_ERROR_MULTI_FEED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2145320940i32 as _);
pub const WIA_ERROR_NETWORK_RESERVATION_FAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2145320941i32 as _);
pub const WIA_ERROR_OFFLINE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2145320955i32 as _);
pub const WIA_ERROR_PAPER_EMPTY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2145320957i32 as _);
pub const WIA_ERROR_PAPER_JAM: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2145320958i32 as _);
pub const WIA_ERROR_PAPER_PROBLEM: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2145320956i32 as _);
pub const WIA_ERROR_USER_INTERVENTION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2145320952i32 as _);
pub const WIA_ERROR_WARMING_UP: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2145320953i32 as _);
pub const WIA_EVENT_CANCEL_IO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3361798072,
    40141,
    16874,
    [187, 191, 77, 208, 156, 91, 23, 149],
);
pub const WIA_EVENT_COVER_CLOSED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1729405414,
    57989,
    18060,
    [155, 140, 218, 125, 196, 203, 170, 5],
);
pub const WIA_EVENT_COVER_OPEN: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    429990198,
    64028,
    20326,
    [144, 15, 143, 145, 78, 199, 78, 201],
);
pub const WIA_EVENT_DEVICE_CONNECTED: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2727066334,
        25782,
        4562,
        [162, 49, 0, 192, 79, 163, 24, 9],
    );
pub const WIA_EVENT_DEVICE_DISCONNECTED: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        339627651,
        25751,
        4562,
        [162, 49, 0, 192, 79, 163, 24, 9],
    );
pub const WIA_EVENT_DEVICE_NOT_READY: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3633720702,
        58588,
        19277,
        [186, 41, 102, 138, 135, 244, 46, 111],
    );
pub const WIA_EVENT_DEVICE_READY: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1965288556,
    39051,
    16798,
    [154, 10, 66, 90, 195, 27, 55, 220],
);
pub const WIA_EVENT_FEEDER_EMPTIED: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3876277122,
        28122,
        18107,
        [143, 249, 83, 206, 177, 160, 62, 53],
    );
pub const WIA_EVENT_FEEDER_LOADED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3431821342,
    39610,
    18461,
    [191, 116, 120, 247, 99, 220, 52, 42],
);
pub const WIA_EVENT_FLATBED_LID_CLOSED: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        4168724239,
        39721,
        17027,
        [173, 149, 212, 18, 22, 77, 57, 169],
    );
pub const WIA_EVENT_FLATBED_LID_OPEN: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3121219107,
        17277,
        20227,
        [169, 125, 119, 147, 177, 35, 17, 60],
    );
pub const WIA_EVENT_HANDLER_NO_ACTION: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3761711997,
        57621,
        17701,
        [188, 85, 182, 41, 230, 140, 116, 90],
    );
pub const WIA_EVENT_HANDLER_PROMPT: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1598794448,
        19801,
        20429,
        [178, 19, 120, 60, 231, 169, 47, 34],
    );
pub const WIA_EVENT_ITEM_CREATED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1284460277,
    57679,
    4562,
    [179, 38, 0, 192, 79, 104, 206, 97],
);
pub const WIA_EVENT_ITEM_DELETED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    488809817,
    57679,
    4562,
    [179, 38, 0, 192, 79, 104, 206, 97],
);
pub const WIA_EVENT_POWER_RESUME: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1636767038,
    63110,
    17232,
    [150, 52, 65, 21, 163, 4, 131, 12],
);
pub const WIA_EVENT_POWER_SUSPEND: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2693935097,
    50100,
    16668,
    [158, 41, 3, 166, 105, 147, 210, 190],
);
pub const WIA_EVENT_SCAN_EMAIL_IMAGE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3330727150,
        21746,
        16798,
        [154, 39, 47, 199, 242, 233, 143, 158],
    );
pub const WIA_EVENT_SCAN_FAX_IMAGE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3222189971,
        35950,
        4562,
        [151, 122, 0, 0, 248, 122, 146, 111],
    );
pub const WIA_EVENT_SCAN_FILM_IMAGE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2603312684,
        24965,
        17292,
        [182, 139, 227, 158, 226, 94, 113, 203],
    );
pub const WIA_EVENT_SCAN_IMAGE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2797971221,
    35950,
    4562,
    [151, 122, 0, 0, 248, 122, 146, 111],
);
pub const WIA_EVENT_SCAN_IMAGE2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    4232538049,
    51379,
    18594,
    [156, 250, 46, 144, 203, 61, 53, 144],
);
pub const WIA_EVENT_SCAN_IMAGE3: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    357443518,
    46615,
    18003,
    [172, 197, 15, 215, 189, 76, 101, 206],
);
pub const WIA_EVENT_SCAN_IMAGE4: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2791010378,
    32572,
    17479,
    [167, 93, 138, 38, 223, 202, 31, 223],
);
pub const WIA_EVENT_SCAN_OCR_IMAGE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2634636169,
        14294,
        18551,
        [175, 237, 98, 162, 151, 220, 109, 190],
    );
pub const WIA_EVENT_SCAN_PRINT_IMAGE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3024221221,
        35950,
        4562,
        [151, 122, 0, 0, 248, 122, 146, 111],
    );
pub const WIA_EVENT_STI_PROXY: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3608279071,
    7949,
    16941,
    [134, 65, 146, 125, 27, 147, 229, 229],
);
pub const WIA_EVENT_STORAGE_CREATED: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        892537010,
        65139,
        18120,
        [137, 94, 250, 69, 81, 204, 200, 90],
    );
pub const WIA_EVENT_STORAGE_DELETED: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1581377374,
        37776,
        17605,
        [154, 81, 228, 112, 25, 227, 144, 207],
    );
pub const WIA_EVENT_TREE_UPDATED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3380976529,
    19122,
    19670,
    [161, 252, 88, 46, 236, 85, 229, 133],
);
pub const WIA_EVENT_VOLUME_INSERT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2520300541,
    53693,
    4562,
    [179, 31, 0, 192, 79, 104, 206, 97],
);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WIA_EXTENDED_TRANSFER_INFO {
    pub ulSize: u32,
    pub ulMinBufferSize: u32,
    pub ulOptimalBufferSize: u32,
    pub ulMaxBufferSize: u32,
    pub ulNumBuffers: u32,
}
impl WIA_EXTENDED_TRANSFER_INFO {}
impl ::std::default::Default for WIA_EXTENDED_TRANSFER_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WIA_EXTENDED_TRANSFER_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIA_EXTENDED_TRANSFER_INFO")
            .field("ulSize", &self.ulSize)
            .field("ulMinBufferSize", &self.ulMinBufferSize)
            .field("ulOptimalBufferSize", &self.ulOptimalBufferSize)
            .field("ulMaxBufferSize", &self.ulMaxBufferSize)
            .field("ulNumBuffers", &self.ulNumBuffers)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WIA_EXTENDED_TRANSFER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ulSize == other.ulSize
            && self.ulMinBufferSize == other.ulMinBufferSize
            && self.ulOptimalBufferSize == other.ulOptimalBufferSize
            && self.ulMaxBufferSize == other.ulMaxBufferSize
            && self.ulNumBuffers == other.ulNumBuffers
    }
}
impl ::std::cmp::Eq for WIA_EXTENDED_TRANSFER_INFO {}
unsafe impl ::windows::runtime::Abi for WIA_EXTENDED_TRANSFER_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WIA_FEEDER_CONTROL_AUTO: u32 = 0u32;
pub const WIA_FEEDER_CONTROL_MANUAL: u32 = 1u32;
pub const WIA_FILM_BW_NEGATIVE: u32 = 2u32;
pub const WIA_FILM_COLOR_NEGATIVE: u32 = 1u32;
pub const WIA_FILM_COLOR_SLIDE: u32 = 0u32;
pub const WIA_FINAL_SCAN: u32 = 0u32;
pub const WIA_FLAG_NOM: u32 = 0u32;
pub const WIA_FLAG_NUM_ELEMS: u32 = 2u32;
pub const WIA_FLAG_VALUES: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WIA_FORMAT_INFO {
    pub guidFormatID: ::windows::runtime::GUID,
    pub lTymed: i32,
}
impl WIA_FORMAT_INFO {}
impl ::std::default::Default for WIA_FORMAT_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WIA_FORMAT_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIA_FORMAT_INFO")
            .field("guidFormatID", &self.guidFormatID)
            .field("lTymed", &self.lTymed)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WIA_FORMAT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.guidFormatID == other.guidFormatID && self.lTymed == other.lTymed
    }
}
impl ::std::cmp::Eq for WIA_FORMAT_INFO {}
unsafe impl ::windows::runtime::Abi for WIA_FORMAT_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WIA_INTENT_BEST_PREVIEW: u32 = 262144u32;
pub const WIA_INTENT_IMAGE_TYPE_COLOR: u32 = 1u32;
pub const WIA_INTENT_IMAGE_TYPE_GRAYSCALE: u32 = 2u32;
pub const WIA_INTENT_IMAGE_TYPE_MASK: u32 = 15u32;
pub const WIA_INTENT_IMAGE_TYPE_TEXT: u32 = 4u32;
pub const WIA_INTENT_MAXIMIZE_QUALITY: u32 = 131072u32;
pub const WIA_INTENT_MINIMIZE_SIZE: u32 = 65536u32;
pub const WIA_INTENT_NONE: u32 = 0u32;
pub const WIA_INTENT_SIZE_MASK: u32 = 983040u32;
pub const WIA_IPA_ACCESS_RIGHTS: u32 = 4102u32;
pub const WIA_IPA_APP_COLOR_MAPPING: u32 = 4121u32;
pub const WIA_IPA_BITS_PER_CHANNEL: u32 = 4110u32;
pub const WIA_IPA_BUFFER_SIZE: u32 = 4118u32;
pub const WIA_IPA_BYTES_PER_LINE: u32 = 4113u32;
pub const WIA_IPA_CHANNELS_PER_PIXEL: u32 = 4109u32;
pub const WIA_IPA_COLOR_PROFILE: u32 = 4117u32;
pub const WIA_IPA_COMPRESSION: u32 = 4107u32;
pub const WIA_IPA_DATATYPE: u32 = 4103u32;
pub const WIA_IPA_DEPTH: u32 = 4104u32;
pub const WIA_IPA_FILENAME_EXTENSION: u32 = 4123u32;
pub const WIA_IPA_FIRST: u32 = 4098u32;
pub const WIA_IPA_FORMAT: u32 = 4106u32;
pub const WIA_IPA_FULL_ITEM_NAME: u32 = 4099u32;
pub const WIA_IPA_GAMMA_CURVES: u32 = 4115u32;
pub const WIA_IPA_ICM_PROFILE_NAME: u32 = 4120u32;
pub const WIA_IPA_ITEMS_STORED: u32 = 4127u32;
pub const WIA_IPA_ITEM_CATEGORY: u32 = 4125u32;
pub const WIA_IPA_ITEM_FLAGS: u32 = 4101u32;
pub const WIA_IPA_ITEM_NAME: u32 = 4098u32;
pub const WIA_IPA_ITEM_SIZE: u32 = 4116u32;
pub const WIA_IPA_ITEM_TIME: u32 = 4100u32;
pub const WIA_IPA_MIN_BUFFER_SIZE: u32 = 4118u32;
pub const WIA_IPA_NUMBER_OF_LINES: u32 = 4114u32;
pub const WIA_IPA_PIXELS_PER_LINE: u32 = 4112u32;
pub const WIA_IPA_PLANAR: u32 = 4111u32;
pub const WIA_IPA_PREFERRED_FORMAT: u32 = 4105u32;
pub const WIA_IPA_PROP_STREAM_COMPAT_ID: u32 = 4122u32;
pub const WIA_IPA_RAW_BITS_PER_CHANNEL: u32 = 4128u32;
pub const WIA_IPA_REGION_TYPE: u32 = 4119u32;
pub const WIA_IPA_SUPPRESS_PROPERTY_PAGE: u32 = 4124u32;
pub const WIA_IPA_TYMED: u32 = 4108u32;
pub const WIA_IPA_UPLOAD_ITEM_SIZE: u32 = 4126u32;
pub const WIA_IPC_AUDIO_AVAILABLE: u32 = 5125u32;
pub const WIA_IPC_AUDIO_DATA: u32 = 5127u32;
pub const WIA_IPC_AUDIO_DATA_FORMAT: u32 = 5126u32;
pub const WIA_IPC_FIRST: u32 = 5122u32;
pub const WIA_IPC_NUM_PICT_PER_ROW: u32 = 5128u32;
pub const WIA_IPC_SEQUENCE: u32 = 5129u32;
pub const WIA_IPC_THUMBNAIL: u32 = 5122u32;
pub const WIA_IPC_THUMB_HEIGHT: u32 = 5124u32;
pub const WIA_IPC_THUMB_WIDTH: u32 = 5123u32;
pub const WIA_IPC_TIMEDELAY: u32 = 5130u32;
pub const WIA_IPS_ALARM: u32 = 4185u32;
pub const WIA_IPS_AUTO_CROP: u32 = 4170u32;
pub const WIA_IPS_AUTO_DESKEW: u32 = 3107u32;
pub const WIA_IPS_BARCODE_READER: u32 = 4150u32;
pub const WIA_IPS_BARCODE_SEARCH_DIRECTION: u32 = 4152u32;
pub const WIA_IPS_BARCODE_SEARCH_TIMEOUT: u32 = 4154u32;
pub const WIA_IPS_BLANK_PAGES: u32 = 4167u32;
pub const WIA_IPS_BLANK_PAGES_SENSITIVITY: u32 = 4192u32;
pub const WIA_IPS_BRIGHTNESS: u32 = 6154u32;
pub const WIA_IPS_COLOR_DROP: u32 = 4176u32;
pub const WIA_IPS_COLOR_DROP_BLUE: u32 = 4179u32;
pub const WIA_IPS_COLOR_DROP_GREEN: u32 = 4178u32;
pub const WIA_IPS_COLOR_DROP_MULTI: u32 = 4191u32;
pub const WIA_IPS_COLOR_DROP_RED: u32 = 4177u32;
pub const WIA_IPS_CONTRAST: u32 = 6155u32;
pub const WIA_IPS_CUR_INTENT: u32 = 6146u32;
pub const WIA_IPS_DESKEW_X: u32 = 6162u32;
pub const WIA_IPS_DESKEW_Y: u32 = 6163u32;
pub const WIA_IPS_DOCUMENT_HANDLING_SELECT: u32 = 3088u32;
pub const WIA_IPS_ENABLED_BARCODE_TYPES: u32 = 4156u32;
pub const WIA_IPS_ENABLED_PATCH_CODE_TYPES: u32 = 4163u32;
pub const WIA_IPS_FEEDER_CONTROL: u32 = 4182u32;
pub const WIA_IPS_FILM_NODE_NAME: u32 = 4129u32;
pub const WIA_IPS_FILM_SCAN_MODE: u32 = 3104u32;
pub const WIA_IPS_FIRST: u32 = 6146u32;
pub const WIA_IPS_INVERT: u32 = 6160u32;
pub const WIA_IPS_JOB_SEPARATORS: u32 = 4165u32;
pub const WIA_IPS_LAMP: u32 = 3105u32;
pub const WIA_IPS_LAMP_AUTO_OFF: u32 = 3106u32;
pub const WIA_IPS_LONG_DOCUMENT: u32 = 4166u32;
pub const WIA_IPS_MAXIMUM_BARCODES_PER_PAGE: u32 = 4151u32;
pub const WIA_IPS_MAXIMUM_BARCODE_SEARCH_RETRIES: u32 = 4153u32;
pub const WIA_IPS_MAX_HORIZONTAL_SIZE: u32 = 6165u32;
pub const WIA_IPS_MAX_VERTICAL_SIZE: u32 = 6166u32;
pub const WIA_IPS_MICR_READER: u32 = 4164u32;
pub const WIA_IPS_MIN_HORIZONTAL_SIZE: u32 = 6167u32;
pub const WIA_IPS_MIN_VERTICAL_SIZE: u32 = 6168u32;
pub const WIA_IPS_MIRROR: u32 = 6158u32;
pub const WIA_IPS_MULTI_FEED: u32 = 4168u32;
pub const WIA_IPS_MULTI_FEED_DETECT_METHOD: u32 = 4193u32;
pub const WIA_IPS_MULTI_FEED_SENSITIVITY: u32 = 4169u32;
pub const WIA_IPS_OPTICAL_XRES: u32 = 3090u32;
pub const WIA_IPS_OPTICAL_YRES: u32 = 3091u32;
pub const WIA_IPS_ORIENTATION: u32 = 6156u32;
pub const WIA_IPS_OVER_SCAN: u32 = 4171u32;
pub const WIA_IPS_OVER_SCAN_BOTTOM: u32 = 4175u32;
pub const WIA_IPS_OVER_SCAN_LEFT: u32 = 4172u32;
pub const WIA_IPS_OVER_SCAN_RIGHT: u32 = 4173u32;
pub const WIA_IPS_OVER_SCAN_TOP: u32 = 4174u32;
pub const WIA_IPS_PAGES: u32 = 3096u32;
pub const WIA_IPS_PAGE_HEIGHT: u32 = 3099u32;
pub const WIA_IPS_PAGE_SIZE: u32 = 3097u32;
pub const WIA_IPS_PAGE_WIDTH: u32 = 3098u32;
pub const WIA_IPS_PATCH_CODE_READER: u32 = 4157u32;
pub const WIA_IPS_PHOTOMETRIC_INTERP: u32 = 6153u32;
pub const WIA_IPS_PREVIEW: u32 = 3100u32;
pub const WIA_IPS_PREVIEW_TYPE: u32 = 3111u32;
pub const WIA_IPS_PRINTER_ENDORSER: u32 = 4130u32;
pub const WIA_IPS_PRINTER_ENDORSER_CHARACTER_ROTATION: u32 = 4187u32;
pub const WIA_IPS_PRINTER_ENDORSER_COUNTER: u32 = 4132u32;
pub const WIA_IPS_PRINTER_ENDORSER_COUNTER_DIGITS: u32 = 4190u32;
pub const WIA_IPS_PRINTER_ENDORSER_FONT_TYPE: u32 = 4184u32;
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS: u32 = 4142u32;
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_DOWNLOAD: u32 = 4149u32;
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MAX_HEIGHT: u32 = 4147u32;
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MAX_WIDTH: u32 = 4145u32;
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MIN_HEIGHT: u32 = 4146u32;
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MIN_WIDTH: u32 = 4144u32;
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_POSITION: u32 = 4143u32;
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_UPLOAD: u32 = 4148u32;
pub const WIA_IPS_PRINTER_ENDORSER_INK: u32 = 4186u32;
pub const WIA_IPS_PRINTER_ENDORSER_MAX_CHARACTERS: u32 = 4188u32;
pub const WIA_IPS_PRINTER_ENDORSER_MAX_GRAPHICS: u32 = 4189u32;
pub const WIA_IPS_PRINTER_ENDORSER_NUM_LINES: u32 = 4136u32;
pub const WIA_IPS_PRINTER_ENDORSER_ORDER: u32 = 4131u32;
pub const WIA_IPS_PRINTER_ENDORSER_PADDING: u32 = 4183u32;
pub const WIA_IPS_PRINTER_ENDORSER_STEP: u32 = 4133u32;
pub const WIA_IPS_PRINTER_ENDORSER_STRING: u32 = 4137u32;
pub const WIA_IPS_PRINTER_ENDORSER_TEXT_DOWNLOAD: u32 = 4141u32;
pub const WIA_IPS_PRINTER_ENDORSER_TEXT_UPLOAD: u32 = 4140u32;
pub const WIA_IPS_PRINTER_ENDORSER_VALID_CHARACTERS: u32 = 4138u32;
pub const WIA_IPS_PRINTER_ENDORSER_VALID_FORMAT_SPECIFIERS: u32 = 4139u32;
pub const WIA_IPS_PRINTER_ENDORSER_XOFFSET: u32 = 4134u32;
pub const WIA_IPS_PRINTER_ENDORSER_YOFFSET: u32 = 4135u32;
pub const WIA_IPS_ROTATION: u32 = 6157u32;
pub const WIA_IPS_SCAN_AHEAD: u32 = 4180u32;
pub const WIA_IPS_SCAN_AHEAD_CAPACITY: u32 = 4181u32;
pub const WIA_IPS_SEGMENTATION: u32 = 6164u32;
pub const WIA_IPS_SHEET_FEEDER_REGISTRATION: u32 = 3078u32;
pub const WIA_IPS_SHOW_PREVIEW_CONTROL: u32 = 3103u32;
pub const WIA_IPS_SUPPORTED_BARCODE_TYPES: u32 = 4155u32;
pub const WIA_IPS_SUPPORTED_PATCH_CODE_TYPES: u32 = 4162u32;
pub const WIA_IPS_SUPPORTS_CHILD_ITEM_CREATION: u32 = 3108u32;
pub const WIA_IPS_THRESHOLD: u32 = 6159u32;
pub const WIA_IPS_TRANSFER_CAPABILITIES: u32 = 6169u32;
pub const WIA_IPS_WARM_UP_TIME: u32 = 6161u32;
pub const WIA_IPS_XEXTENT: u32 = 6151u32;
pub const WIA_IPS_XPOS: u32 = 6149u32;
pub const WIA_IPS_XRES: u32 = 6147u32;
pub const WIA_IPS_XSCALING: u32 = 3109u32;
pub const WIA_IPS_YEXTENT: u32 = 6152u32;
pub const WIA_IPS_YPOS: u32 = 6150u32;
pub const WIA_IPS_YRES: u32 = 6148u32;
pub const WIA_IPS_YSCALING: u32 = 3110u32;
pub const WIA_IS_DEFAULT_HANDLER: u32 = 1u32;
pub const WIA_ITEM_CAN_BE_DELETED: u32 = 128u32;
pub const WIA_ITEM_READ: u32 = 1u32;
pub const WIA_ITEM_WRITE: u32 = 2u32;
pub const WIA_LAMP_OFF: u32 = 1u32;
pub const WIA_LAMP_ON: u32 = 0u32;
pub const WIA_LINE_ORDER_BOTTOM_TO_TOP: u32 = 2u32;
pub const WIA_LINE_ORDER_TOP_TO_BOTTOM: u32 = 1u32;
pub const WIA_LIST_COUNT: u32 = 0u32;
pub const WIA_LIST_NOM: u32 = 1u32;
pub const WIA_LIST_NUM_ELEMS: u32 = 2u32;
pub const WIA_LIST_VALUES: u32 = 2u32;
pub const WIA_LONG_DOCUMENT_DISABLED: u32 = 0u32;
pub const WIA_LONG_DOCUMENT_ENABLED: u32 = 1u32;
pub const WIA_LONG_DOCUMENT_SPLIT: u32 = 2u32;
pub const WIA_MAJOR_EVENT_DEVICE_CONNECT: u32 = 1u32;
pub const WIA_MAJOR_EVENT_DEVICE_DISCONNECT: u32 = 2u32;
pub const WIA_MAJOR_EVENT_PICTURE_DELETED: u32 = 4u32;
pub const WIA_MAJOR_EVENT_PICTURE_TAKEN: u32 = 3u32;
pub const WIA_MAX_CTX_SIZE: u32 = 16777216u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WIA_MICR {
    pub Tag: u32,
    pub Version: u32,
    pub Size: u32,
    pub Placeholder: u16,
    pub Reserved: u16,
    pub Count: u32,
    pub Micr: [WIA_MICR_INFO; 1],
}
impl WIA_MICR {}
impl ::std::default::Default for WIA_MICR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WIA_MICR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIA_MICR")
            .field("Tag", &self.Tag)
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .field("Placeholder", &self.Placeholder)
            .field("Reserved", &self.Reserved)
            .field("Count", &self.Count)
            .field("Micr", &self.Micr)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WIA_MICR {
    fn eq(&self, other: &Self) -> bool {
        self.Tag == other.Tag
            && self.Version == other.Version
            && self.Size == other.Size
            && self.Placeholder == other.Placeholder
            && self.Reserved == other.Reserved
            && self.Count == other.Count
            && self.Micr == other.Micr
    }
}
impl ::std::cmp::Eq for WIA_MICR {}
unsafe impl ::windows::runtime::Abi for WIA_MICR {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WIA_MICR_INFO {
    pub Size: u32,
    pub Page: u32,
    pub Length: u32,
    pub Text: [u16; 1],
}
impl WIA_MICR_INFO {}
impl ::std::default::Default for WIA_MICR_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WIA_MICR_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIA_MICR_INFO")
            .field("Size", &self.Size)
            .field("Page", &self.Page)
            .field("Length", &self.Length)
            .field("Text", &self.Text)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WIA_MICR_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Page == other.Page
            && self.Length == other.Length
            && self.Text == other.Text
    }
}
impl ::std::cmp::Eq for WIA_MICR_INFO {}
unsafe impl ::windows::runtime::Abi for WIA_MICR_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WIA_MICR_READER_AUTO: u32 = 1u32;
pub const WIA_MICR_READER_DISABLED: u32 = 0u32;
pub const WIA_MICR_READER_FEEDER_BACK: u32 = 4u32;
pub const WIA_MICR_READER_FEEDER_DUPLEX: u32 = 5u32;
pub const WIA_MICR_READER_FEEDER_FRONT: u32 = 3u32;
pub const WIA_MICR_READER_FLATBED: u32 = 2u32;
pub const WIA_MULTI_FEED_DETECT_CONTINUE: u32 = 3u32;
pub const WIA_MULTI_FEED_DETECT_DISABLED: u32 = 0u32;
pub const WIA_MULTI_FEED_DETECT_METHOD_LENGTH: u32 = 0u32;
pub const WIA_MULTI_FEED_DETECT_METHOD_OVERLAP: u32 = 1u32;
pub const WIA_MULTI_FEED_DETECT_STOP_ERROR: u32 = 1u32;
pub const WIA_MULTI_FEED_DETECT_STOP_SUCCESS: u32 = 2u32;
pub const WIA_NOTIFICATION_EVENT: u32 = 1u32;
pub const WIA_NUM_DIP: u32 = 16u32;
pub const WIA_NUM_IPC: u32 = 9u32;
pub const WIA_ORDER_BGR: u32 = 1u32;
pub const WIA_ORDER_RGB: u32 = 0u32;
pub const WIA_OVER_SCAN_ALL: u32 = 3u32;
pub const WIA_OVER_SCAN_DISABLED: u32 = 0u32;
pub const WIA_OVER_SCAN_LEFT_RIGHT: u32 = 2u32;
pub const WIA_OVER_SCAN_TOP_BOTTOM: u32 = 1u32;
pub const WIA_PACKED_PIXEL: u32 = 0u32;
pub const WIA_PAGE_A4: u32 = 0u32;
pub const WIA_PAGE_AUTO: u32 = 100u32;
pub const WIA_PAGE_BUSINESSCARD: u32 = 6u32;
pub const WIA_PAGE_CUSTOM: u32 = 2u32;
pub const WIA_PAGE_CUSTOM_BASE: u32 = 32768u32;
pub const WIA_PAGE_DIN_2B: u32 = 52u32;
pub const WIA_PAGE_DIN_4B: u32 = 53u32;
pub const WIA_PAGE_ISO_A0: u32 = 7u32;
pub const WIA_PAGE_ISO_A1: u32 = 8u32;
pub const WIA_PAGE_ISO_A10: u32 = 16u32;
pub const WIA_PAGE_ISO_A2: u32 = 9u32;
pub const WIA_PAGE_ISO_A3: u32 = 10u32;
pub const WIA_PAGE_ISO_A4: u32 = 0u32;
pub const WIA_PAGE_ISO_A5: u32 = 11u32;
pub const WIA_PAGE_ISO_A6: u32 = 12u32;
pub const WIA_PAGE_ISO_A7: u32 = 13u32;
pub const WIA_PAGE_ISO_A8: u32 = 14u32;
pub const WIA_PAGE_ISO_A9: u32 = 15u32;
pub const WIA_PAGE_ISO_B0: u32 = 17u32;
pub const WIA_PAGE_ISO_B1: u32 = 18u32;
pub const WIA_PAGE_ISO_B10: u32 = 27u32;
pub const WIA_PAGE_ISO_B2: u32 = 19u32;
pub const WIA_PAGE_ISO_B3: u32 = 20u32;
pub const WIA_PAGE_ISO_B4: u32 = 21u32;
pub const WIA_PAGE_ISO_B5: u32 = 22u32;
pub const WIA_PAGE_ISO_B6: u32 = 23u32;
pub const WIA_PAGE_ISO_B7: u32 = 24u32;
pub const WIA_PAGE_ISO_B8: u32 = 25u32;
pub const WIA_PAGE_ISO_B9: u32 = 26u32;
pub const WIA_PAGE_ISO_C0: u32 = 28u32;
pub const WIA_PAGE_ISO_C1: u32 = 29u32;
pub const WIA_PAGE_ISO_C10: u32 = 38u32;
pub const WIA_PAGE_ISO_C2: u32 = 30u32;
pub const WIA_PAGE_ISO_C3: u32 = 31u32;
pub const WIA_PAGE_ISO_C4: u32 = 32u32;
pub const WIA_PAGE_ISO_C5: u32 = 33u32;
pub const WIA_PAGE_ISO_C6: u32 = 34u32;
pub const WIA_PAGE_ISO_C7: u32 = 35u32;
pub const WIA_PAGE_ISO_C8: u32 = 36u32;
pub const WIA_PAGE_ISO_C9: u32 = 37u32;
pub const WIA_PAGE_JIS_2A: u32 = 50u32;
pub const WIA_PAGE_JIS_4A: u32 = 51u32;
pub const WIA_PAGE_JIS_B0: u32 = 39u32;
pub const WIA_PAGE_JIS_B1: u32 = 40u32;
pub const WIA_PAGE_JIS_B10: u32 = 49u32;
pub const WIA_PAGE_JIS_B2: u32 = 41u32;
pub const WIA_PAGE_JIS_B3: u32 = 42u32;
pub const WIA_PAGE_JIS_B4: u32 = 43u32;
pub const WIA_PAGE_JIS_B5: u32 = 44u32;
pub const WIA_PAGE_JIS_B6: u32 = 45u32;
pub const WIA_PAGE_JIS_B7: u32 = 46u32;
pub const WIA_PAGE_JIS_B8: u32 = 47u32;
pub const WIA_PAGE_JIS_B9: u32 = 48u32;
pub const WIA_PAGE_LETTER: u32 = 1u32;
pub const WIA_PAGE_USLEDGER: u32 = 4u32;
pub const WIA_PAGE_USLEGAL: u32 = 3u32;
pub const WIA_PAGE_USLETTER: u32 = 1u32;
pub const WIA_PAGE_USSTATEMENT: u32 = 5u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WIA_PATCH_CODES {
    pub Tag: u32,
    pub Version: u32,
    pub Size: u32,
    pub Count: u32,
    pub PatchCodes: [WIA_PATCH_CODE_INFO; 1],
}
impl WIA_PATCH_CODES {}
impl ::std::default::Default for WIA_PATCH_CODES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WIA_PATCH_CODES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIA_PATCH_CODES")
            .field("Tag", &self.Tag)
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .field("Count", &self.Count)
            .field("PatchCodes", &self.PatchCodes)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WIA_PATCH_CODES {
    fn eq(&self, other: &Self) -> bool {
        self.Tag == other.Tag
            && self.Version == other.Version
            && self.Size == other.Size
            && self.Count == other.Count
            && self.PatchCodes == other.PatchCodes
    }
}
impl ::std::cmp::Eq for WIA_PATCH_CODES {}
unsafe impl ::windows::runtime::Abi for WIA_PATCH_CODES {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WIA_PATCH_CODE_1: u32 = 1u32;
pub const WIA_PATCH_CODE_10: u32 = 10u32;
pub const WIA_PATCH_CODE_11: u32 = 11u32;
pub const WIA_PATCH_CODE_12: u32 = 12u32;
pub const WIA_PATCH_CODE_13: u32 = 13u32;
pub const WIA_PATCH_CODE_14: u32 = 14u32;
pub const WIA_PATCH_CODE_2: u32 = 2u32;
pub const WIA_PATCH_CODE_3: u32 = 3u32;
pub const WIA_PATCH_CODE_4: u32 = 4u32;
pub const WIA_PATCH_CODE_6: u32 = 6u32;
pub const WIA_PATCH_CODE_7: u32 = 7u32;
pub const WIA_PATCH_CODE_8: u32 = 8u32;
pub const WIA_PATCH_CODE_9: u32 = 9u32;
pub const WIA_PATCH_CODE_CUSTOM_BASE: u32 = 32768u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WIA_PATCH_CODE_INFO {
    pub Type: u32,
}
impl WIA_PATCH_CODE_INFO {}
impl ::std::default::Default for WIA_PATCH_CODE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WIA_PATCH_CODE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIA_PATCH_CODE_INFO")
            .field("Type", &self.Type)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WIA_PATCH_CODE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type
    }
}
impl ::std::cmp::Eq for WIA_PATCH_CODE_INFO {}
unsafe impl ::windows::runtime::Abi for WIA_PATCH_CODE_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WIA_PATCH_CODE_READER_AUTO: u32 = 1u32;
pub const WIA_PATCH_CODE_READER_DISABLED: u32 = 0u32;
pub const WIA_PATCH_CODE_READER_FEEDER_BACK: u32 = 4u32;
pub const WIA_PATCH_CODE_READER_FEEDER_DUPLEX: u32 = 5u32;
pub const WIA_PATCH_CODE_READER_FEEDER_FRONT: u32 = 3u32;
pub const WIA_PATCH_CODE_READER_FLATBED: u32 = 2u32;
pub const WIA_PATCH_CODE_T: u32 = 5u32;
pub const WIA_PATCH_CODE_UNKNOWN: u32 = 0u32;
pub const WIA_PHOTO_WHITE_0: u32 = 1u32;
pub const WIA_PHOTO_WHITE_1: u32 = 0u32;
pub const WIA_PLANAR: u32 = 1u32;
pub const WIA_PREVIEW_SCAN: u32 = 1u32;
pub const WIA_PRINTER_ENDORSER_AFTER_SCAN: u32 = 1u32;
pub const WIA_PRINTER_ENDORSER_AUTO: u32 = 1u32;
pub const WIA_PRINTER_ENDORSER_BEFORE_SCAN: u32 = 0u32;
pub const WIA_PRINTER_ENDORSER_DIGITAL: u32 = 6u32;
pub const WIA_PRINTER_ENDORSER_DISABLED: u32 = 0u32;
pub const WIA_PRINTER_ENDORSER_FEEDER_BACK: u32 = 4u32;
pub const WIA_PRINTER_ENDORSER_FEEDER_DUPLEX: u32 = 5u32;
pub const WIA_PRINTER_ENDORSER_FEEDER_FRONT: u32 = 3u32;
pub const WIA_PRINTER_ENDORSER_FLATBED: u32 = 2u32;
pub const WIA_PRINTER_ENDORSER_GRAPHICS_BACKGROUND: u32 = 8u32;
pub const WIA_PRINTER_ENDORSER_GRAPHICS_BOTTOM: u32 = 3u32;
pub const WIA_PRINTER_ENDORSER_GRAPHICS_BOTTOM_LEFT: u32 = 6u32;
pub const WIA_PRINTER_ENDORSER_GRAPHICS_BOTTOM_RIGHT: u32 = 7u32;
pub const WIA_PRINTER_ENDORSER_GRAPHICS_DEVICE_DEFAULT: u32 = 9u32;
pub const WIA_PRINTER_ENDORSER_GRAPHICS_LEFT: u32 = 0u32;
pub const WIA_PRINTER_ENDORSER_GRAPHICS_RIGHT: u32 = 1u32;
pub const WIA_PRINTER_ENDORSER_GRAPHICS_TOP: u32 = 2u32;
pub const WIA_PRINTER_ENDORSER_GRAPHICS_TOP_LEFT: u32 = 4u32;
pub const WIA_PRINTER_ENDORSER_GRAPHICS_TOP_RIGHT: u32 = 5u32;
pub const WIA_PRINT_AM_PM: u32 = 9u32;
pub const WIA_PRINT_DATE: u32 = 0u32;
pub const WIA_PRINT_DAY: u32 = 3u32;
pub const WIA_PRINT_FONT_BOLD: u32 = 1u32;
pub const WIA_PRINT_FONT_EXTRA_BOLD: u32 = 2u32;
pub const WIA_PRINT_FONT_ITALIC: u32 = 5u32;
pub const WIA_PRINT_FONT_ITALIC_BOLD: u32 = 3u32;
pub const WIA_PRINT_FONT_ITALIC_EXTRA_BOLD: u32 = 4u32;
pub const WIA_PRINT_FONT_LARGE: u32 = 12u32;
pub const WIA_PRINT_FONT_LARGE_BOLD: u32 = 13u32;
pub const WIA_PRINT_FONT_LARGE_EXTRA_BOLD: u32 = 14u32;
pub const WIA_PRINT_FONT_LARGE_ITALIC: u32 = 17u32;
pub const WIA_PRINT_FONT_LARGE_ITALIC_BOLD: u32 = 15u32;
pub const WIA_PRINT_FONT_LARGE_ITALIC_EXTRA_BOLD: u32 = 16u32;
pub const WIA_PRINT_FONT_NORMAL: u32 = 0u32;
pub const WIA_PRINT_FONT_SMALL: u32 = 6u32;
pub const WIA_PRINT_FONT_SMALL_BOLD: u32 = 7u32;
pub const WIA_PRINT_FONT_SMALL_EXTRA_BOLD: u32 = 8u32;
pub const WIA_PRINT_FONT_SMALL_ITALIC: u32 = 11u32;
pub const WIA_PRINT_FONT_SMALL_ITALIC_BOLD: u32 = 9u32;
pub const WIA_PRINT_FONT_SMALL_ITALIC_EXTRA_BOLD: u32 = 10u32;
pub const WIA_PRINT_HOUR_12H: u32 = 8u32;
pub const WIA_PRINT_HOUR_24H: u32 = 7u32;
pub const WIA_PRINT_IMAGE: u32 = 13u32;
pub const WIA_PRINT_MILLISECOND: u32 = 14u32;
pub const WIA_PRINT_MINUTE: u32 = 10u32;
pub const WIA_PRINT_MONTH: u32 = 2u32;
pub const WIA_PRINT_MONTH_NAME: u32 = 15u32;
pub const WIA_PRINT_MONTH_SHORT: u32 = 16u32;
pub const WIA_PRINT_PADDING_BLANK: u32 = 2u32;
pub const WIA_PRINT_PADDING_NONE: u32 = 0u32;
pub const WIA_PRINT_PADDING_ZERO: u32 = 1u32;
pub const WIA_PRINT_PAGE_COUNT: u32 = 12u32;
pub const WIA_PRINT_SECOND: u32 = 11u32;
pub const WIA_PRINT_TIME_12H: u32 = 6u32;
pub const WIA_PRINT_TIME_24H: u32 = 5u32;
pub const WIA_PRINT_WEEK_DAY: u32 = 4u32;
pub const WIA_PRINT_WEEK_DAY_SHORT: u32 = 17u32;
pub const WIA_PRINT_YEAR: u32 = 1u32;
pub const WIA_PRIVATE_DEVPROP: u32 = 38914u32;
pub const WIA_PRIVATE_ITEMPROP: u32 = 71682u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WIA_PROPERTY_CONTEXT {
    pub cProps: u32,
    pub pProps: *mut u32,
    pub pChanged: *mut super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl WIA_PROPERTY_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WIA_PROPERTY_CONTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WIA_PROPERTY_CONTEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIA_PROPERTY_CONTEXT")
            .field("cProps", &self.cProps)
            .field("pProps", &self.pProps)
            .field("pChanged", &self.pChanged)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WIA_PROPERTY_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.cProps == other.cProps
            && self.pProps == other.pProps
            && self.pChanged == other.pChanged
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WIA_PROPERTY_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WIA_PROPERTY_CONTEXT {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::clone::Clone for WIA_PROPERTY_INFO {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WIA_PROPERTY_INFO {
    pub lAccessFlags: u32,
    pub vt: u16,
    pub ValidVal: WIA_PROPERTY_INFO_0,
}
#[cfg(feature = "Win32_Foundation")]
impl WIA_PROPERTY_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WIA_PROPERTY_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WIA_PROPERTY_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WIA_PROPERTY_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WIA_PROPERTY_INFO {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::clone::Clone for WIA_PROPERTY_INFO_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union WIA_PROPERTY_INFO_0 {
    pub Range: WIA_PROPERTY_INFO_0_7,
    pub RangeFloat: WIA_PROPERTY_INFO_0_6,
    pub List: WIA_PROPERTY_INFO_0_4,
    pub ListFloat: WIA_PROPERTY_INFO_0_2,
    pub ListGuid: WIA_PROPERTY_INFO_0_3,
    pub ListBStr: ::std::mem::ManuallyDrop<WIA_PROPERTY_INFO_0_1>,
    pub Flag: WIA_PROPERTY_INFO_0_0,
    pub None: WIA_PROPERTY_INFO_0_5,
}
#[cfg(feature = "Win32_Foundation")]
impl WIA_PROPERTY_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WIA_PROPERTY_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WIA_PROPERTY_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WIA_PROPERTY_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WIA_PROPERTY_INFO_0 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WIA_PROPERTY_INFO_0_0 {
    pub Nom: i32,
    pub ValidBits: i32,
}
impl WIA_PROPERTY_INFO_0_0 {}
impl ::std::default::Default for WIA_PROPERTY_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WIA_PROPERTY_INFO_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Flag_e__Struct")
            .field("Nom", &self.Nom)
            .field("ValidBits", &self.ValidBits)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WIA_PROPERTY_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Nom == other.Nom && self.ValidBits == other.ValidBits
    }
}
impl ::std::cmp::Eq for WIA_PROPERTY_INFO_0_0 {}
unsafe impl ::windows::runtime::Abi for WIA_PROPERTY_INFO_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WIA_PROPERTY_INFO_0_1 {
    pub cNumList: i32,
    pub Nom: super::super::Foundation::BSTR,
    pub pList: *mut super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WIA_PROPERTY_INFO_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WIA_PROPERTY_INFO_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WIA_PROPERTY_INFO_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_ListBStr_e__Struct")
            .field("cNumList", &self.cNumList)
            .field("Nom", &self.Nom)
            .field("pList", &self.pList)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WIA_PROPERTY_INFO_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.cNumList == other.cNumList && self.Nom == other.Nom && self.pList == other.pList
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WIA_PROPERTY_INFO_0_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WIA_PROPERTY_INFO_0_1 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WIA_PROPERTY_INFO_0_2 {
    pub cNumList: i32,
    pub Nom: f64,
    pub pList: *mut u8,
}
impl WIA_PROPERTY_INFO_0_2 {}
impl ::std::default::Default for WIA_PROPERTY_INFO_0_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WIA_PROPERTY_INFO_0_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_ListFloat_e__Struct")
            .field("cNumList", &self.cNumList)
            .field("Nom", &self.Nom)
            .field("pList", &self.pList)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WIA_PROPERTY_INFO_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.cNumList == other.cNumList && self.Nom == other.Nom && self.pList == other.pList
    }
}
impl ::std::cmp::Eq for WIA_PROPERTY_INFO_0_2 {}
unsafe impl ::windows::runtime::Abi for WIA_PROPERTY_INFO_0_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WIA_PROPERTY_INFO_0_3 {
    pub cNumList: i32,
    pub Nom: ::windows::runtime::GUID,
    pub pList: *mut ::windows::runtime::GUID,
}
impl WIA_PROPERTY_INFO_0_3 {}
impl ::std::default::Default for WIA_PROPERTY_INFO_0_3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WIA_PROPERTY_INFO_0_3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_ListGuid_e__Struct")
            .field("cNumList", &self.cNumList)
            .field("Nom", &self.Nom)
            .field("pList", &self.pList)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WIA_PROPERTY_INFO_0_3 {
    fn eq(&self, other: &Self) -> bool {
        self.cNumList == other.cNumList && self.Nom == other.Nom && self.pList == other.pList
    }
}
impl ::std::cmp::Eq for WIA_PROPERTY_INFO_0_3 {}
unsafe impl ::windows::runtime::Abi for WIA_PROPERTY_INFO_0_3 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WIA_PROPERTY_INFO_0_4 {
    pub cNumList: i32,
    pub Nom: i32,
    pub pList: *mut u8,
}
impl WIA_PROPERTY_INFO_0_4 {}
impl ::std::default::Default for WIA_PROPERTY_INFO_0_4 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WIA_PROPERTY_INFO_0_4 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_List_e__Struct")
            .field("cNumList", &self.cNumList)
            .field("Nom", &self.Nom)
            .field("pList", &self.pList)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WIA_PROPERTY_INFO_0_4 {
    fn eq(&self, other: &Self) -> bool {
        self.cNumList == other.cNumList && self.Nom == other.Nom && self.pList == other.pList
    }
}
impl ::std::cmp::Eq for WIA_PROPERTY_INFO_0_4 {}
unsafe impl ::windows::runtime::Abi for WIA_PROPERTY_INFO_0_4 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WIA_PROPERTY_INFO_0_5 {
    pub Dummy: i32,
}
impl WIA_PROPERTY_INFO_0_5 {}
impl ::std::default::Default for WIA_PROPERTY_INFO_0_5 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WIA_PROPERTY_INFO_0_5 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_None_e__Struct")
            .field("Dummy", &self.Dummy)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WIA_PROPERTY_INFO_0_5 {
    fn eq(&self, other: &Self) -> bool {
        self.Dummy == other.Dummy
    }
}
impl ::std::cmp::Eq for WIA_PROPERTY_INFO_0_5 {}
unsafe impl ::windows::runtime::Abi for WIA_PROPERTY_INFO_0_5 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WIA_PROPERTY_INFO_0_6 {
    pub Min: f64,
    pub Nom: f64,
    pub Max: f64,
    pub Inc: f64,
}
impl WIA_PROPERTY_INFO_0_6 {}
impl ::std::default::Default for WIA_PROPERTY_INFO_0_6 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WIA_PROPERTY_INFO_0_6 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_RangeFloat_e__Struct")
            .field("Min", &self.Min)
            .field("Nom", &self.Nom)
            .field("Max", &self.Max)
            .field("Inc", &self.Inc)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WIA_PROPERTY_INFO_0_6 {
    fn eq(&self, other: &Self) -> bool {
        self.Min == other.Min
            && self.Nom == other.Nom
            && self.Max == other.Max
            && self.Inc == other.Inc
    }
}
impl ::std::cmp::Eq for WIA_PROPERTY_INFO_0_6 {}
unsafe impl ::windows::runtime::Abi for WIA_PROPERTY_INFO_0_6 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WIA_PROPERTY_INFO_0_7 {
    pub Min: i32,
    pub Nom: i32,
    pub Max: i32,
    pub Inc: i32,
}
impl WIA_PROPERTY_INFO_0_7 {}
impl ::std::default::Default for WIA_PROPERTY_INFO_0_7 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WIA_PROPERTY_INFO_0_7 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Range_e__Struct")
            .field("Min", &self.Min)
            .field("Nom", &self.Nom)
            .field("Max", &self.Max)
            .field("Inc", &self.Inc)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WIA_PROPERTY_INFO_0_7 {
    fn eq(&self, other: &Self) -> bool {
        self.Min == other.Min
            && self.Nom == other.Nom
            && self.Max == other.Max
            && self.Inc == other.Inc
    }
}
impl ::std::cmp::Eq for WIA_PROPERTY_INFO_0_7 {}
unsafe impl ::windows::runtime::Abi for WIA_PROPERTY_INFO_0_7 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WIA_PROPID_TO_NAME {
    pub propid: u32,
    pub pszName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WIA_PROPID_TO_NAME {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WIA_PROPID_TO_NAME {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WIA_PROPID_TO_NAME {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIA_PROPID_TO_NAME")
            .field("propid", &self.propid)
            .field("pszName", &self.pszName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WIA_PROPID_TO_NAME {
    fn eq(&self, other: &Self) -> bool {
        self.propid == other.propid && self.pszName == other.pszName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WIA_PROPID_TO_NAME {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WIA_PROPID_TO_NAME {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WIA_PROPPAGE_CAMERA_ITEM_GENERAL: u32 = 2u32;
pub const WIA_PROPPAGE_DEVICE_GENERAL: u32 = 4u32;
pub const WIA_PROPPAGE_SCANNER_ITEM_GENERAL: u32 = 1u32;
pub const WIA_PROP_CACHEABLE: u32 = 65536u32;
pub const WIA_PROP_FLAG: u32 = 64u32;
pub const WIA_PROP_LIST: u32 = 32u32;
pub const WIA_PROP_NONE: u32 = 8u32;
pub const WIA_PROP_RANGE: u32 = 16u32;
pub const WIA_PROP_READ: u32 = 1u32;
pub const WIA_PROP_SYNC_REQUIRED: u32 = 4u32;
pub const WIA_PROP_WRITE: u32 = 2u32;
pub const WIA_RANGE_MAX: u32 = 2u32;
pub const WIA_RANGE_MIN: u32 = 0u32;
pub const WIA_RANGE_NOM: u32 = 1u32;
pub const WIA_RANGE_NUM_ELEMS: u32 = 4u32;
pub const WIA_RANGE_STEP: u32 = 3u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl WIA_RAW_HEADER {}
impl ::std::default::Default for WIA_RAW_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WIA_RAW_HEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIA_RAW_HEADER")
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
impl ::std::cmp::PartialEq for WIA_RAW_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Tag == other.Tag
            && self.Version == other.Version
            && self.HeaderSize == other.HeaderSize
            && self.XRes == other.XRes
            && self.YRes == other.YRes
            && self.XExtent == other.XExtent
            && self.YExtent == other.YExtent
            && self.BytesPerLine == other.BytesPerLine
            && self.BitsPerPixel == other.BitsPerPixel
            && self.ChannelsPerPixel == other.ChannelsPerPixel
            && self.DataType == other.DataType
            && self.BitsPerChannel == other.BitsPerChannel
            && self.Compression == other.Compression
            && self.PhotometricInterp == other.PhotometricInterp
            && self.LineOrder == other.LineOrder
            && self.RawDataOffset == other.RawDataOffset
            && self.RawDataSize == other.RawDataSize
            && self.PaletteOffset == other.PaletteOffset
            && self.PaletteSize == other.PaletteSize
    }
}
impl ::std::cmp::Eq for WIA_RAW_HEADER {}
unsafe impl ::windows::runtime::Abi for WIA_RAW_HEADER {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WIA_REGISTER_EVENT_CALLBACK: u32 = 1u32;
pub const WIA_RESERVED_FOR_NEW_PROPS: u32 = 1024u32;
pub const WIA_SCAN_AHEAD_ALL: u32 = 0u32;
pub const WIA_SCAN_AHEAD_DISABLED: u32 = 0u32;
pub const WIA_SCAN_AHEAD_ENABLED: u32 = 1u32;
pub const WIA_SELECT_DEVICE_NODEFAULT: u32 = 1u32;
pub const WIA_SEPARATOR_DETECT_NOSCAN_CONTINUE: u32 = 3u32;
pub const WIA_SEPARATOR_DETECT_NOSCAN_STOP: u32 = 4u32;
pub const WIA_SEPARATOR_DETECT_SCAN_CONTINUE: u32 = 1u32;
pub const WIA_SEPARATOR_DETECT_SCAN_STOP: u32 = 2u32;
pub const WIA_SEPARATOR_DISABLED: u32 = 0u32;
pub const WIA_SET_DEFAULT_HANDLER: u32 = 4u32;
pub const WIA_SHOW_PREVIEW_CONTROL: u32 = 0u32;
pub const WIA_STATUS_CALIBRATING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(2162691i32 as _);
pub const WIA_STATUS_CLEAR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(2162696i32 as _);
pub const WIA_STATUS_END_OF_MEDIA: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(2162689i32 as _);
pub const WIA_STATUS_NETWORK_DEVICE_RESERVED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(2162695i32 as _);
pub const WIA_STATUS_NOT_HANDLED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(2162698i32 as _);
pub const WIA_STATUS_RESERVING_NETWORK_DEVICE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(2162694i32 as _);
pub const WIA_STATUS_SKIP_ITEM: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(2162697i32 as _);
pub const WIA_STATUS_WARMING_UP: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(2162690i32 as _);
pub const WIA_S_CHANGE_DEVICE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(2162699i32 as _);
pub const WIA_S_NO_DEVICE_AVAILABLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2145320939i32 as _);
pub const WIA_TRANSFER_ACQUIRE_CHILDREN: u32 = 1u32;
pub const WIA_TRANSFER_CHILDREN_SINGLE_SCAN: u32 = 1u32;
pub const WIA_TRANSFER_MSG_DEVICE_STATUS: u32 = 5u32;
pub const WIA_TRANSFER_MSG_END_OF_STREAM: u32 = 2u32;
pub const WIA_TRANSFER_MSG_END_OF_TRANSFER: u32 = 3u32;
pub const WIA_TRANSFER_MSG_NEW_PAGE: u32 = 6u32;
pub const WIA_TRANSFER_MSG_STATUS: u32 = 1u32;
pub const WIA_UNREGISTER_EVENT_CALLBACK: u32 = 2u32;
pub const WIA_USE_SEGMENTATION_FILTER: u32 = 0u32;
pub const WIA_WSD_FRIENDLY_NAME: u32 = 38920u32;
pub const WIA_WSD_MANUFACTURER: u32 = 38914u32;
pub const WIA_WSD_MANUFACTURER_URL: u32 = 38915u32;
pub const WIA_WSD_MODEL_NAME: u32 = 38916u32;
pub const WIA_WSD_MODEL_NUMBER: u32 = 38917u32;
pub const WIA_WSD_MODEL_URL: u32 = 38918u32;
pub const WIA_WSD_PRESENTATION_URL: u32 = 38919u32;
pub const WIA_WSD_SCAN_AVAILABLE_ITEM: u32 = 38922u32;
pub const WIA_WSD_SERIAL_NUMBER: u32 = 38921u32;
pub const WiaAudFmt_AIFF: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1726136143,
    46844,
    17471,
    [148, 200, 47, 51, 200, 166, 90, 175],
);
pub const WiaAudFmt_MP3: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    264008187,
    17343,
    18930,
    [145, 144, 230, 254, 207, 243, 126, 84],
);
pub const WiaAudFmt_WAV: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    4162380102,
    1967,
    16639,
    [174, 85, 190, 143, 44, 6, 93, 190],
);
pub const WiaAudFmt_WMA: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3592250387,
    35778,
    17295,
    [147, 173, 33, 189, 72, 77, 182, 161],
);
pub const WiaDevMgr: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2717181734,
    36081,
    4561,
    [191, 146, 0, 96, 8, 30, 216, 17],
);
pub const WiaDevMgr2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3066204860,
    31880,
    16878,
    [139, 84, 142, 201, 38, 23, 229, 153],
);
pub const WiaImgFmt_ASF: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2375323369,
    53418,
    18962,
    [157, 154, 156, 197, 222, 54, 25, 155],
);
pub const WiaImgFmt_AVI: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    855165460,
    2172,
    18696,
    [183, 196, 103, 87, 254, 126, 144, 171],
);
pub const WiaImgFmt_BMP: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3110812843,
    1832,
    4563,
    [157, 123, 0, 0, 248, 30, 243, 46],
);
pub const WiaImgFmt_CIFF: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2552342699,
    14974,
    16917,
    [148, 224, 210, 122, 70, 12, 3, 178],
);
pub const WiaImgFmt_CSV: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    895212068,
    23199,
    17556,
    [128, 220, 190, 117, 44, 236, 188, 140],
);
pub const WiaImgFmt_DPOF: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    916385451,
    41192,
    17866,
    [134, 166, 168, 60, 229, 105, 126, 40],
);
pub const WiaImgFmt_EMF: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3110812844,
    1832,
    4563,
    [157, 123, 0, 0, 248, 30, 243, 46],
);
pub const WiaImgFmt_EXEC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1214095511,
    5150,
    19109,
    [187, 59, 165, 97, 141, 149, 208, 43],
);
pub const WiaImgFmt_EXIF: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3110812850,
    1832,
    4563,
    [157, 123, 0, 0, 248, 30, 243, 46],
);
pub const WiaImgFmt_FLASHPIX: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3110812852,
    1832,
    4563,
    [157, 123, 0, 0, 248, 30, 243, 46],
);
pub const WiaImgFmt_GIF: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3110812848,
    1832,
    4563,
    [157, 123, 0, 0, 248, 30, 243, 46],
);
pub const WiaImgFmt_HTML: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3382333026,
    39390,
    19092,
    [172, 202, 113, 149, 106, 194, 151, 125],
);
pub const WiaImgFmt_ICO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3110812853,
    1832,
    4563,
    [157, 123, 0, 0, 248, 30, 243, 46],
);
pub const WiaImgFmt_JBIG: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1105780114,
    12042,
    17364,
    [134, 54, 241, 97, 75, 161, 30, 70],
);
pub const WiaImgFmt_JBIG2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3146677863,
    10300,
    16949,
    [158, 89, 11, 155, 249, 76, 166, 135],
);
pub const WiaImgFmt_JPEG: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3110812846,
    1832,
    4563,
    [157, 123, 0, 0, 248, 30, 243, 46],
);
pub const WiaImgFmt_JPEG2K: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    877585074,
    14811,
    19934,
    [129, 115, 196, 183, 95, 143, 30, 73],
);
pub const WiaImgFmt_JPEG2KX: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1138837012,
    51210,
    18512,
    [186, 243, 75, 21, 45, 200, 218, 39],
);
pub const WiaImgFmt_MEMORYBMP: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3110812842,
    1832,
    4563,
    [157, 123, 0, 0, 248, 30, 243, 46],
);
pub const WiaImgFmt_MPG: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3973535716,
    53996,
    20311,
    [149, 93, 188, 248, 169, 124, 78, 82],
);
pub const WiaImgFmt_OXPS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    746263104,
    49485,
    16649,
    [151, 85, 4, 184, 144, 37, 21, 58],
);
pub const WiaImgFmt_PDFA: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2575351131,
    13411,
    17351,
    [189, 202, 60, 170, 20, 111, 34, 159],
);
pub const WiaImgFmt_PHOTOCD: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3110812851,
    1832,
    4563,
    [157, 123, 0, 0, 248, 30, 243, 46],
);
pub const WiaImgFmt_PICT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2797372888,
    27454,
    16622,
    [169, 92, 37, 212, 130, 228, 26, 220],
);
pub const WiaImgFmt_PNG: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3110812847,
    1832,
    4563,
    [157, 123, 0, 0, 248, 30, 243, 46],
);
pub const WiaImgFmt_RAW: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1863452441,
    61864,
    19975,
    [154, 222, 155, 100, 198, 58, 61, 204],
);
pub const WiaImgFmt_RAWBAR: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3663984691,
    53870,
    17694,
    [144, 210, 234, 85, 161, 54, 93, 98],
);
pub const WiaImgFmt_RAWMIC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    583331928,
    3464,
    16540,
    [172, 28, 238, 193, 43, 14, 166, 128],
);
pub const WiaImgFmt_RAWPAT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2002800764,
    20580,
    16396,
    [154, 23, 87, 86, 36, 216, 130, 75],
);
pub const WiaImgFmt_RAWRGB: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3164900181,
    62066,
    17265,
    [176, 241, 74, 21, 13, 5, 123, 180],
);
pub const WiaImgFmt_RTF: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1463670435,
    18484,
    17197,
    [169, 181, 225, 152, 221, 158, 137, 13],
);
pub const WiaImgFmt_SCRIPT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    4269632595,
    11692,
    17514,
    [176, 189, 215, 62, 33, 233, 36, 201],
);
pub const WiaImgFmt_TIFF: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3110812849,
    1832,
    4563,
    [157, 123, 0, 0, 248, 30, 243, 46],
);
pub const WiaImgFmt_TXT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    4210904450,
    29247,
    16927,
    [147, 24, 48, 80, 26, 196, 75, 89],
);
pub const WiaImgFmt_UNDEFINED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3110812841,
    1832,
    4563,
    [157, 123, 0, 0, 248, 30, 243, 46],
);
pub const WiaImgFmt_UNICODE16: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    460732854,
    25431,
    18385,
    [154, 7, 18, 69, 45, 192, 115, 233],
);
pub const WiaImgFmt_WMF: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3110812845,
    1832,
    4563,
    [157, 123, 0, 0, 248, 30, 243, 46],
);
pub const WiaImgFmt_XML: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3105297495,
    56008,
    18564,
    [179, 147, 21, 180, 113, 213, 240, 126],
);
pub const WiaImgFmt_XMLBAR: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1647669276,
    15000,
    18508,
    [178, 168, 253, 255, 216, 126, 107, 22],
);
pub const WiaImgFmt_XMLMIC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    756436065,
    47534,
    19235,
    [137, 115, 199, 6, 126, 31, 189, 49],
);
pub const WiaImgFmt_XMLPAT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    4170739541,
    61522,
    17933,
    [149, 35, 58, 125, 254, 219, 179, 60],
);
pub const WiaImgFmt_XPS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1879788047,
    8209,
    16668,
    [180, 48, 209, 224, 178, 225, 11, 40],
);
pub const WiaItemTypeAnalyze: u32 = 16u32;
pub const WiaItemTypeAudio: u32 = 32u32;
pub const WiaItemTypeBurst: u32 = 2048u32;
pub const WiaItemTypeDeleted: u32 = 128u32;
pub const WiaItemTypeDevice: u32 = 64u32;
pub const WiaItemTypeDisconnected: u32 = 256u32;
pub const WiaItemTypeDocument: u32 = 262144u32;
pub const WiaItemTypeFile: u32 = 2u32;
pub const WiaItemTypeFolder: u32 = 4u32;
pub const WiaItemTypeFree: u32 = 0u32;
pub const WiaItemTypeGenerated: u32 = 16384u32;
pub const WiaItemTypeHPanorama: u32 = 512u32;
pub const WiaItemTypeHasAttachments: u32 = 32768u32;
pub const WiaItemTypeImage: u32 = 1u32;
pub const WiaItemTypeMask: u32 = 2148532223u32;
pub const WiaItemTypeProgrammableDataSource: u32 = 524288u32;
pub const WiaItemTypeRemoved: u32 = 2147483648u32;
pub const WiaItemTypeRoot: u32 = 8u32;
pub const WiaItemTypeStorage: u32 = 4096u32;
pub const WiaItemTypeTransfer: u32 = 8192u32;
pub const WiaItemTypeTwainCapabilityPassThrough: u32 = 131072u32;
pub const WiaItemTypeVPanorama: u32 = 1024u32;
pub const WiaItemTypeVideo: u32 = 65536u32;
pub const WiaLog: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2716291927,
    34842,
    16798,
    [131, 226, 187, 22, 219, 25, 124, 104],
);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WiaTransferParams {
    pub lMessage: i32,
    pub lPercentComplete: i32,
    pub ulTransferredBytes: u64,
    pub hrErrorStatus: ::windows::runtime::HRESULT,
}
impl WiaTransferParams {}
impl ::std::default::Default for WiaTransferParams {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WiaTransferParams {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WiaTransferParams")
            .field("lMessage", &self.lMessage)
            .field("lPercentComplete", &self.lPercentComplete)
            .field("ulTransferredBytes", &self.ulTransferredBytes)
            .field("hrErrorStatus", &self.hrErrorStatus)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WiaTransferParams {
    fn eq(&self, other: &Self) -> bool {
        self.lMessage == other.lMessage
            && self.lPercentComplete == other.lPercentComplete
            && self.ulTransferredBytes == other.ulTransferredBytes
            && self.hrErrorStatus == other.hrErrorStatus
    }
}
impl ::std::cmp::Eq for WiaTransferParams {}
unsafe impl ::windows::runtime::Abi for WiaTransferParams {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WiaVideo: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    956875725,
    17528,
    17718,
    [175, 47, 16, 194, 93, 78, 248, 154],
);
