#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Win32_UI_Input_Ime")]
pub mod Ime;
#[cfg(feature = "Win32_UI_Input_Ink")]
pub mod Ink;
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
pub mod KeyboardAndMouse;
#[cfg(feature = "Win32_UI_Input_Pointer")]
pub mod Pointer;
#[cfg(feature = "Win32_UI_Input_Radial")]
pub mod Radial;
#[cfg(feature = "Win32_UI_Input_Touch")]
pub mod Touch;
#[cfg(feature = "Win32_UI_Input_XboxController")]
pub mod XboxController;
#[doc = "*Required features: `Win32_UI_Input`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DefRawInputProc(parawinput: *const *const RAWINPUT, ninput: i32, cbsizeheader: u32) -> super::super::Foundation::LRESULT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DefRawInputProc(parawinput: *const *const RAWINPUT, ninput: i32, cbsizeheader: u32) -> super::super::Foundation::LRESULT;
        }
        ::std::mem::transmute(DefRawInputProc(::std::mem::transmute(parawinput), ::std::mem::transmute(ninput), ::std::mem::transmute(cbsizeheader)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Input`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCIMSSM(inputmessagesource: *mut INPUT_MESSAGE_SOURCE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCIMSSM(inputmessagesource: *mut INPUT_MESSAGE_SOURCE) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetCIMSSM(::std::mem::transmute(inputmessagesource)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Input`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentInputMessageSource(inputmessagesource: *mut INPUT_MESSAGE_SOURCE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentInputMessageSource(inputmessagesource: *mut INPUT_MESSAGE_SOURCE) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetCurrentInputMessageSource(::std::mem::transmute(inputmessagesource)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Input`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetRawInputBuffer(pdata: *mut RAWINPUT, pcbsize: *mut u32, cbsizeheader: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetRawInputBuffer(pdata: *mut RAWINPUT, pcbsize: *mut u32, cbsizeheader: u32) -> u32;
        }
        ::std::mem::transmute(GetRawInputBuffer(::std::mem::transmute(pdata), ::std::mem::transmute(pcbsize), ::std::mem::transmute(cbsizeheader)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Input`*"]
#[inline]
pub unsafe fn GetRawInputData<'a, Param0: ::windows::runtime::IntoParam<'a, HRAWINPUT>>(hrawinput: Param0, uicommand: RAW_INPUT_DATA_COMMAND_FLAGS, pdata: *mut ::std::ffi::c_void, pcbsize: *mut u32, cbsizeheader: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetRawInputData(hrawinput: HRAWINPUT, uicommand: RAW_INPUT_DATA_COMMAND_FLAGS, pdata: *mut ::std::ffi::c_void, pcbsize: *mut u32, cbsizeheader: u32) -> u32;
        }
        ::std::mem::transmute(GetRawInputData(hrawinput.into_param().abi(), ::std::mem::transmute(uicommand), ::std::mem::transmute(pdata), ::std::mem::transmute(pcbsize), ::std::mem::transmute(cbsizeheader)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Input`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetRawInputDeviceInfoA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hdevice: Param0, uicommand: RAW_INPUT_DEVICE_INFO_COMMAND, pdata: *mut ::std::ffi::c_void, pcbsize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetRawInputDeviceInfoA(hdevice: super::super::Foundation::HANDLE, uicommand: RAW_INPUT_DEVICE_INFO_COMMAND, pdata: *mut ::std::ffi::c_void, pcbsize: *mut u32) -> u32;
        }
        ::std::mem::transmute(GetRawInputDeviceInfoA(hdevice.into_param().abi(), ::std::mem::transmute(uicommand), ::std::mem::transmute(pdata), ::std::mem::transmute(pcbsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Input`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetRawInputDeviceInfoW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hdevice: Param0, uicommand: RAW_INPUT_DEVICE_INFO_COMMAND, pdata: *mut ::std::ffi::c_void, pcbsize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetRawInputDeviceInfoW(hdevice: super::super::Foundation::HANDLE, uicommand: RAW_INPUT_DEVICE_INFO_COMMAND, pdata: *mut ::std::ffi::c_void, pcbsize: *mut u32) -> u32;
        }
        ::std::mem::transmute(GetRawInputDeviceInfoW(hdevice.into_param().abi(), ::std::mem::transmute(uicommand), ::std::mem::transmute(pdata), ::std::mem::transmute(pcbsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Input`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetRawInputDeviceList(prawinputdevicelist: *mut RAWINPUTDEVICELIST, puinumdevices: *mut u32, cbsize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetRawInputDeviceList(prawinputdevicelist: *mut RAWINPUTDEVICELIST, puinumdevices: *mut u32, cbsize: u32) -> u32;
        }
        ::std::mem::transmute(GetRawInputDeviceList(::std::mem::transmute(prawinputdevicelist), ::std::mem::transmute(puinumdevices), ::std::mem::transmute(cbsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Input`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetRegisteredRawInputDevices(prawinputdevices: *mut RAWINPUTDEVICE, puinumdevices: *mut u32, cbsize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetRegisteredRawInputDevices(prawinputdevices: *mut RAWINPUTDEVICE, puinumdevices: *mut u32, cbsize: u32) -> u32;
        }
        ::std::mem::transmute(GetRegisteredRawInputDevices(::std::mem::transmute(prawinputdevices), ::std::mem::transmute(puinumdevices), ::std::mem::transmute(cbsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct HRAWINPUT(pub isize);
impl ::std::default::Default for HRAWINPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HRAWINPUT {}
unsafe impl ::windows::runtime::Abi for HRAWINPUT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Input`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct INPUT_MESSAGE_DEVICE_TYPE(pub i32);
pub const IMDT_UNAVAILABLE: INPUT_MESSAGE_DEVICE_TYPE = INPUT_MESSAGE_DEVICE_TYPE(0i32);
pub const IMDT_KEYBOARD: INPUT_MESSAGE_DEVICE_TYPE = INPUT_MESSAGE_DEVICE_TYPE(1i32);
pub const IMDT_MOUSE: INPUT_MESSAGE_DEVICE_TYPE = INPUT_MESSAGE_DEVICE_TYPE(2i32);
pub const IMDT_TOUCH: INPUT_MESSAGE_DEVICE_TYPE = INPUT_MESSAGE_DEVICE_TYPE(4i32);
pub const IMDT_PEN: INPUT_MESSAGE_DEVICE_TYPE = INPUT_MESSAGE_DEVICE_TYPE(8i32);
pub const IMDT_TOUCHPAD: INPUT_MESSAGE_DEVICE_TYPE = INPUT_MESSAGE_DEVICE_TYPE(16i32);
impl ::std::convert::From<i32> for INPUT_MESSAGE_DEVICE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for INPUT_MESSAGE_DEVICE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Input`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct INPUT_MESSAGE_ORIGIN_ID(pub i32);
pub const IMO_UNAVAILABLE: INPUT_MESSAGE_ORIGIN_ID = INPUT_MESSAGE_ORIGIN_ID(0i32);
pub const IMO_HARDWARE: INPUT_MESSAGE_ORIGIN_ID = INPUT_MESSAGE_ORIGIN_ID(1i32);
pub const IMO_INJECTED: INPUT_MESSAGE_ORIGIN_ID = INPUT_MESSAGE_ORIGIN_ID(2i32);
pub const IMO_SYSTEM: INPUT_MESSAGE_ORIGIN_ID = INPUT_MESSAGE_ORIGIN_ID(4i32);
impl ::std::convert::From<i32> for INPUT_MESSAGE_ORIGIN_ID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for INPUT_MESSAGE_ORIGIN_ID {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Input`*"]
pub struct INPUT_MESSAGE_SOURCE {
    pub deviceType: INPUT_MESSAGE_DEVICE_TYPE,
    pub originId: INPUT_MESSAGE_ORIGIN_ID,
}
impl INPUT_MESSAGE_SOURCE {}
impl ::std::default::Default for INPUT_MESSAGE_SOURCE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for INPUT_MESSAGE_SOURCE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("INPUT_MESSAGE_SOURCE").field("deviceType", &self.deviceType).field("originId", &self.originId).finish()
    }
}
impl ::std::cmp::PartialEq for INPUT_MESSAGE_SOURCE {
    fn eq(&self, other: &Self) -> bool {
        self.deviceType == other.deviceType && self.originId == other.originId
    }
}
impl ::std::cmp::Eq for INPUT_MESSAGE_SOURCE {}
unsafe impl ::windows::runtime::Abi for INPUT_MESSAGE_SOURCE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Input`*"]
pub struct RAWHID {
    pub dwSizeHid: u32,
    pub dwCount: u32,
    pub bRawData: [u8; 1],
}
impl RAWHID {}
impl ::std::default::Default for RAWHID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RAWHID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RAWHID").field("dwSizeHid", &self.dwSizeHid).field("dwCount", &self.dwCount).field("bRawData", &self.bRawData).finish()
    }
}
impl ::std::cmp::PartialEq for RAWHID {
    fn eq(&self, other: &Self) -> bool {
        self.dwSizeHid == other.dwSizeHid && self.dwCount == other.dwCount && self.bRawData == other.bRawData
    }
}
impl ::std::cmp::Eq for RAWHID {}
unsafe impl ::windows::runtime::Abi for RAWHID {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Input`, `Win32_Foundation`*"]
pub struct RAWINPUT {
    pub header: RAWINPUTHEADER,
    pub data: RAWINPUT_0,
}
#[cfg(feature = "Win32_Foundation")]
impl RAWINPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RAWINPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RAWINPUT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RAWINPUT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RAWINPUT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union RAWINPUT_0 {
    pub mouse: RAWMOUSE,
    pub keyboard: RAWKEYBOARD,
    pub hid: RAWHID,
}
#[cfg(feature = "Win32_Foundation")]
impl RAWINPUT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RAWINPUT_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RAWINPUT_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RAWINPUT_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RAWINPUT_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Input`, `Win32_Foundation`*"]
pub struct RAWINPUTDEVICE {
    pub usUsagePage: u16,
    pub usUsage: u16,
    pub dwFlags: RAWINPUTDEVICE_FLAGS,
    pub hwndTarget: super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Foundation")]
impl RAWINPUTDEVICE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RAWINPUTDEVICE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RAWINPUTDEVICE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RAWINPUTDEVICE").field("usUsagePage", &self.usUsagePage).field("usUsage", &self.usUsage).field("dwFlags", &self.dwFlags).field("hwndTarget", &self.hwndTarget).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RAWINPUTDEVICE {
    fn eq(&self, other: &Self) -> bool {
        self.usUsagePage == other.usUsagePage && self.usUsage == other.usUsage && self.dwFlags == other.dwFlags && self.hwndTarget == other.hwndTarget
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RAWINPUTDEVICE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RAWINPUTDEVICE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Input`, `Win32_Foundation`*"]
pub struct RAWINPUTDEVICELIST {
    pub hDevice: super::super::Foundation::HANDLE,
    pub dwType: RID_DEVICE_INFO_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl RAWINPUTDEVICELIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RAWINPUTDEVICELIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RAWINPUTDEVICELIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RAWINPUTDEVICELIST").field("hDevice", &self.hDevice).field("dwType", &self.dwType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RAWINPUTDEVICELIST {
    fn eq(&self, other: &Self) -> bool {
        self.hDevice == other.hDevice && self.dwType == other.dwType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RAWINPUTDEVICELIST {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RAWINPUTDEVICELIST {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Input`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RAWINPUTDEVICE_FLAGS(pub u32);
pub const RIDEV_REMOVE: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(1u32);
pub const RIDEV_EXCLUDE: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(16u32);
pub const RIDEV_PAGEONLY: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(32u32);
pub const RIDEV_NOLEGACY: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(48u32);
pub const RIDEV_INPUTSINK: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(256u32);
pub const RIDEV_CAPTUREMOUSE: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(512u32);
pub const RIDEV_NOHOTKEYS: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(512u32);
pub const RIDEV_APPKEYS: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(1024u32);
pub const RIDEV_EXINPUTSINK: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(4096u32);
pub const RIDEV_DEVNOTIFY: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(8192u32);
impl ::std::convert::From<u32> for RAWINPUTDEVICE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RAWINPUTDEVICE_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for RAWINPUTDEVICE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for RAWINPUTDEVICE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for RAWINPUTDEVICE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for RAWINPUTDEVICE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for RAWINPUTDEVICE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Input`, `Win32_Foundation`*"]
pub struct RAWINPUTHEADER {
    pub dwType: u32,
    pub dwSize: u32,
    pub hDevice: super::super::Foundation::HANDLE,
    pub wParam: super::super::Foundation::WPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl RAWINPUTHEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RAWINPUTHEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RAWINPUTHEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RAWINPUTHEADER").field("dwType", &self.dwType).field("dwSize", &self.dwSize).field("hDevice", &self.hDevice).field("wParam", &self.wParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RAWINPUTHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType && self.dwSize == other.dwSize && self.hDevice == other.hDevice && self.wParam == other.wParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RAWINPUTHEADER {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RAWINPUTHEADER {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Input`*"]
pub struct RAWKEYBOARD {
    pub MakeCode: u16,
    pub Flags: u16,
    pub Reserved: u16,
    pub VKey: u16,
    pub Message: u32,
    pub ExtraInformation: u32,
}
impl RAWKEYBOARD {}
impl ::std::default::Default for RAWKEYBOARD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RAWKEYBOARD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RAWKEYBOARD").field("MakeCode", &self.MakeCode).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("VKey", &self.VKey).field("Message", &self.Message).field("ExtraInformation", &self.ExtraInformation).finish()
    }
}
impl ::std::cmp::PartialEq for RAWKEYBOARD {
    fn eq(&self, other: &Self) -> bool {
        self.MakeCode == other.MakeCode && self.Flags == other.Flags && self.Reserved == other.Reserved && self.VKey == other.VKey && self.Message == other.Message && self.ExtraInformation == other.ExtraInformation
    }
}
impl ::std::cmp::Eq for RAWKEYBOARD {}
unsafe impl ::windows::runtime::Abi for RAWKEYBOARD {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Input`*"]
pub struct RAWMOUSE {
    pub usFlags: u16,
    pub Anonymous: RAWMOUSE_0,
    pub ulRawButtons: u32,
    pub lLastX: i32,
    pub lLastY: i32,
    pub ulExtraInformation: u32,
}
impl RAWMOUSE {}
impl ::std::default::Default for RAWMOUSE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for RAWMOUSE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for RAWMOUSE {}
unsafe impl ::windows::runtime::Abi for RAWMOUSE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Input`*"]
pub union RAWMOUSE_0 {
    pub ulButtons: u32,
    pub Anonymous: RAWMOUSE_0_0,
}
impl RAWMOUSE_0 {}
impl ::std::default::Default for RAWMOUSE_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for RAWMOUSE_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for RAWMOUSE_0 {}
unsafe impl ::windows::runtime::Abi for RAWMOUSE_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Input`*"]
pub struct RAWMOUSE_0_0 {
    pub usButtonFlags: u16,
    pub usButtonData: u16,
}
impl RAWMOUSE_0_0 {}
impl ::std::default::Default for RAWMOUSE_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RAWMOUSE_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("usButtonFlags", &self.usButtonFlags).field("usButtonData", &self.usButtonData).finish()
    }
}
impl ::std::cmp::PartialEq for RAWMOUSE_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.usButtonFlags == other.usButtonFlags && self.usButtonData == other.usButtonData
    }
}
impl ::std::cmp::Eq for RAWMOUSE_0_0 {}
unsafe impl ::windows::runtime::Abi for RAWMOUSE_0_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Input`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RAW_INPUT_DATA_COMMAND_FLAGS(pub u32);
pub const RID_HEADER: RAW_INPUT_DATA_COMMAND_FLAGS = RAW_INPUT_DATA_COMMAND_FLAGS(268435461u32);
pub const RID_INPUT: RAW_INPUT_DATA_COMMAND_FLAGS = RAW_INPUT_DATA_COMMAND_FLAGS(268435459u32);
impl ::std::convert::From<u32> for RAW_INPUT_DATA_COMMAND_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RAW_INPUT_DATA_COMMAND_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for RAW_INPUT_DATA_COMMAND_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for RAW_INPUT_DATA_COMMAND_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for RAW_INPUT_DATA_COMMAND_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for RAW_INPUT_DATA_COMMAND_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for RAW_INPUT_DATA_COMMAND_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Input`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RAW_INPUT_DEVICE_INFO_COMMAND(pub u32);
pub const RIDI_PREPARSEDDATA: RAW_INPUT_DEVICE_INFO_COMMAND = RAW_INPUT_DEVICE_INFO_COMMAND(536870917u32);
pub const RIDI_DEVICENAME: RAW_INPUT_DEVICE_INFO_COMMAND = RAW_INPUT_DEVICE_INFO_COMMAND(536870919u32);
pub const RIDI_DEVICEINFO: RAW_INPUT_DEVICE_INFO_COMMAND = RAW_INPUT_DEVICE_INFO_COMMAND(536870923u32);
impl ::std::convert::From<u32> for RAW_INPUT_DEVICE_INFO_COMMAND {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RAW_INPUT_DEVICE_INFO_COMMAND {
    type Abi = Self;
}
impl ::std::ops::BitOr for RAW_INPUT_DEVICE_INFO_COMMAND {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for RAW_INPUT_DEVICE_INFO_COMMAND {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for RAW_INPUT_DEVICE_INFO_COMMAND {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for RAW_INPUT_DEVICE_INFO_COMMAND {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for RAW_INPUT_DEVICE_INFO_COMMAND {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Input`, `Win32_Foundation`*"]
pub struct RID_DEVICE_INFO {
    pub cbSize: u32,
    pub dwType: RID_DEVICE_INFO_TYPE,
    pub Anonymous: RID_DEVICE_INFO_0,
}
#[cfg(feature = "Win32_Foundation")]
impl RID_DEVICE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RID_DEVICE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RID_DEVICE_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RID_DEVICE_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RID_DEVICE_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union RID_DEVICE_INFO_0 {
    pub mouse: RID_DEVICE_INFO_MOUSE,
    pub keyboard: RID_DEVICE_INFO_KEYBOARD,
    pub hid: RID_DEVICE_INFO_HID,
}
#[cfg(feature = "Win32_Foundation")]
impl RID_DEVICE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RID_DEVICE_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RID_DEVICE_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RID_DEVICE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RID_DEVICE_INFO_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Input`*"]
pub struct RID_DEVICE_INFO_HID {
    pub dwVendorId: u32,
    pub dwProductId: u32,
    pub dwVersionNumber: u32,
    pub usUsagePage: u16,
    pub usUsage: u16,
}
impl RID_DEVICE_INFO_HID {}
impl ::std::default::Default for RID_DEVICE_INFO_HID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RID_DEVICE_INFO_HID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RID_DEVICE_INFO_HID").field("dwVendorId", &self.dwVendorId).field("dwProductId", &self.dwProductId).field("dwVersionNumber", &self.dwVersionNumber).field("usUsagePage", &self.usUsagePage).field("usUsage", &self.usUsage).finish()
    }
}
impl ::std::cmp::PartialEq for RID_DEVICE_INFO_HID {
    fn eq(&self, other: &Self) -> bool {
        self.dwVendorId == other.dwVendorId && self.dwProductId == other.dwProductId && self.dwVersionNumber == other.dwVersionNumber && self.usUsagePage == other.usUsagePage && self.usUsage == other.usUsage
    }
}
impl ::std::cmp::Eq for RID_DEVICE_INFO_HID {}
unsafe impl ::windows::runtime::Abi for RID_DEVICE_INFO_HID {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Input`*"]
pub struct RID_DEVICE_INFO_KEYBOARD {
    pub dwType: u32,
    pub dwSubType: u32,
    pub dwKeyboardMode: u32,
    pub dwNumberOfFunctionKeys: u32,
    pub dwNumberOfIndicators: u32,
    pub dwNumberOfKeysTotal: u32,
}
impl RID_DEVICE_INFO_KEYBOARD {}
impl ::std::default::Default for RID_DEVICE_INFO_KEYBOARD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RID_DEVICE_INFO_KEYBOARD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RID_DEVICE_INFO_KEYBOARD")
            .field("dwType", &self.dwType)
            .field("dwSubType", &self.dwSubType)
            .field("dwKeyboardMode", &self.dwKeyboardMode)
            .field("dwNumberOfFunctionKeys", &self.dwNumberOfFunctionKeys)
            .field("dwNumberOfIndicators", &self.dwNumberOfIndicators)
            .field("dwNumberOfKeysTotal", &self.dwNumberOfKeysTotal)
            .finish()
    }
}
impl ::std::cmp::PartialEq for RID_DEVICE_INFO_KEYBOARD {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType && self.dwSubType == other.dwSubType && self.dwKeyboardMode == other.dwKeyboardMode && self.dwNumberOfFunctionKeys == other.dwNumberOfFunctionKeys && self.dwNumberOfIndicators == other.dwNumberOfIndicators && self.dwNumberOfKeysTotal == other.dwNumberOfKeysTotal
    }
}
impl ::std::cmp::Eq for RID_DEVICE_INFO_KEYBOARD {}
unsafe impl ::windows::runtime::Abi for RID_DEVICE_INFO_KEYBOARD {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Input`, `Win32_Foundation`*"]
pub struct RID_DEVICE_INFO_MOUSE {
    pub dwId: u32,
    pub dwNumberOfButtons: u32,
    pub dwSampleRate: u32,
    pub fHasHorizontalWheel: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl RID_DEVICE_INFO_MOUSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RID_DEVICE_INFO_MOUSE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RID_DEVICE_INFO_MOUSE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RID_DEVICE_INFO_MOUSE").field("dwId", &self.dwId).field("dwNumberOfButtons", &self.dwNumberOfButtons).field("dwSampleRate", &self.dwSampleRate).field("fHasHorizontalWheel", &self.fHasHorizontalWheel).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RID_DEVICE_INFO_MOUSE {
    fn eq(&self, other: &Self) -> bool {
        self.dwId == other.dwId && self.dwNumberOfButtons == other.dwNumberOfButtons && self.dwSampleRate == other.dwSampleRate && self.fHasHorizontalWheel == other.fHasHorizontalWheel
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RID_DEVICE_INFO_MOUSE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RID_DEVICE_INFO_MOUSE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Input`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RID_DEVICE_INFO_TYPE(pub u32);
pub const RIM_TYPEMOUSE: RID_DEVICE_INFO_TYPE = RID_DEVICE_INFO_TYPE(0u32);
pub const RIM_TYPEKEYBOARD: RID_DEVICE_INFO_TYPE = RID_DEVICE_INFO_TYPE(1u32);
pub const RIM_TYPEHID: RID_DEVICE_INFO_TYPE = RID_DEVICE_INFO_TYPE(2u32);
impl ::std::convert::From<u32> for RID_DEVICE_INFO_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RID_DEVICE_INFO_TYPE {
    type Abi = Self;
}
impl ::std::ops::BitOr for RID_DEVICE_INFO_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for RID_DEVICE_INFO_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for RID_DEVICE_INFO_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for RID_DEVICE_INFO_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for RID_DEVICE_INFO_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Input`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterRawInputDevices(prawinputdevices: *const RAWINPUTDEVICE, uinumdevices: u32, cbsize: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterRawInputDevices(prawinputdevices: *const RAWINPUTDEVICE, uinumdevices: u32, cbsize: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RegisterRawInputDevices(::std::mem::transmute(prawinputdevices), ::std::mem::transmute(uinumdevices), ::std::mem::transmute(cbsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
