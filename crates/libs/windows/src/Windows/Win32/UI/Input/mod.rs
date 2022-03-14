#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
#[doc = "*Required features: `\"Win32_UI_Input\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DefRawInputProc(parawinput: &[*const RAWINPUT], cbsizeheader: u32) -> super::super::Foundation::LRESULT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DefRawInputProc(parawinput: *const *const RAWINPUT, ninput: i32, cbsizeheader: u32) -> super::super::Foundation::LRESULT;
        }
        ::core::mem::transmute(DefRawInputProc(::core::mem::transmute(::windows::core::as_ptr_or_null(parawinput)), parawinput.len() as _, ::core::mem::transmute(cbsizeheader)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_UI_Input\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCIMSSM(inputmessagesource: *mut INPUT_MESSAGE_SOURCE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCIMSSM(inputmessagesource: *mut INPUT_MESSAGE_SOURCE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetCIMSSM(::core::mem::transmute(inputmessagesource)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_UI_Input\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentInputMessageSource(inputmessagesource: *mut INPUT_MESSAGE_SOURCE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentInputMessageSource(inputmessagesource: *mut INPUT_MESSAGE_SOURCE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetCurrentInputMessageSource(::core::mem::transmute(inputmessagesource)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_UI_Input\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetRawInputBuffer(pdata: *mut RAWINPUT, pcbsize: *mut u32, cbsizeheader: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetRawInputBuffer(pdata: *mut RAWINPUT, pcbsize: *mut u32, cbsizeheader: u32) -> u32;
        }
        ::core::mem::transmute(GetRawInputBuffer(::core::mem::transmute(pdata), ::core::mem::transmute(pcbsize), ::core::mem::transmute(cbsizeheader)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
#[inline]
pub unsafe fn GetRawInputData<'a, Param0: ::windows::core::IntoParam<'a, HRAWINPUT>>(hrawinput: Param0, uicommand: RAW_INPUT_DATA_COMMAND_FLAGS, pdata: *mut ::core::ffi::c_void, pcbsize: *mut u32, cbsizeheader: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetRawInputData(hrawinput: HRAWINPUT, uicommand: RAW_INPUT_DATA_COMMAND_FLAGS, pdata: *mut ::core::ffi::c_void, pcbsize: *mut u32, cbsizeheader: u32) -> u32;
        }
        ::core::mem::transmute(GetRawInputData(hrawinput.into_param().abi(), ::core::mem::transmute(uicommand), ::core::mem::transmute(pdata), ::core::mem::transmute(pcbsize), ::core::mem::transmute(cbsizeheader)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_UI_Input\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetRawInputDeviceInfoA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hdevice: Param0, uicommand: RAW_INPUT_DEVICE_INFO_COMMAND, pdata: *mut ::core::ffi::c_void, pcbsize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetRawInputDeviceInfoA(hdevice: super::super::Foundation::HANDLE, uicommand: RAW_INPUT_DEVICE_INFO_COMMAND, pdata: *mut ::core::ffi::c_void, pcbsize: *mut u32) -> u32;
        }
        ::core::mem::transmute(GetRawInputDeviceInfoA(hdevice.into_param().abi(), ::core::mem::transmute(uicommand), ::core::mem::transmute(pdata), ::core::mem::transmute(pcbsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_UI_Input\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetRawInputDeviceInfoW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hdevice: Param0, uicommand: RAW_INPUT_DEVICE_INFO_COMMAND, pdata: *mut ::core::ffi::c_void, pcbsize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetRawInputDeviceInfoW(hdevice: super::super::Foundation::HANDLE, uicommand: RAW_INPUT_DEVICE_INFO_COMMAND, pdata: *mut ::core::ffi::c_void, pcbsize: *mut u32) -> u32;
        }
        ::core::mem::transmute(GetRawInputDeviceInfoW(hdevice.into_param().abi(), ::core::mem::transmute(uicommand), ::core::mem::transmute(pdata), ::core::mem::transmute(pcbsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_UI_Input\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetRawInputDeviceList(prawinputdevicelist: *mut RAWINPUTDEVICELIST, puinumdevices: *mut u32, cbsize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetRawInputDeviceList(prawinputdevicelist: *mut RAWINPUTDEVICELIST, puinumdevices: *mut u32, cbsize: u32) -> u32;
        }
        ::core::mem::transmute(GetRawInputDeviceList(::core::mem::transmute(prawinputdevicelist), ::core::mem::transmute(puinumdevices), ::core::mem::transmute(cbsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_UI_Input\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetRegisteredRawInputDevices(prawinputdevices: *mut RAWINPUTDEVICE, puinumdevices: *mut u32, cbsize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetRegisteredRawInputDevices(prawinputdevices: *mut RAWINPUTDEVICE, puinumdevices: *mut u32, cbsize: u32) -> u32;
        }
        ::core::mem::transmute(GetRegisteredRawInputDevices(::core::mem::transmute(prawinputdevices), ::core::mem::transmute(puinumdevices), ::core::mem::transmute(cbsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HRAWINPUT(pub isize);
impl HRAWINPUT {
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
impl ::core::default::Default for HRAWINPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HRAWINPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HRAWINPUT {}
impl ::core::fmt::Debug for HRAWINPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HRAWINPUT").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for HRAWINPUT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct INPUT_MESSAGE_DEVICE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
pub const IMDT_UNAVAILABLE: INPUT_MESSAGE_DEVICE_TYPE = INPUT_MESSAGE_DEVICE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
pub const IMDT_KEYBOARD: INPUT_MESSAGE_DEVICE_TYPE = INPUT_MESSAGE_DEVICE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
pub const IMDT_MOUSE: INPUT_MESSAGE_DEVICE_TYPE = INPUT_MESSAGE_DEVICE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
pub const IMDT_TOUCH: INPUT_MESSAGE_DEVICE_TYPE = INPUT_MESSAGE_DEVICE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
pub const IMDT_PEN: INPUT_MESSAGE_DEVICE_TYPE = INPUT_MESSAGE_DEVICE_TYPE(8i32);
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
pub const IMDT_TOUCHPAD: INPUT_MESSAGE_DEVICE_TYPE = INPUT_MESSAGE_DEVICE_TYPE(16i32);
impl ::core::marker::Copy for INPUT_MESSAGE_DEVICE_TYPE {}
impl ::core::clone::Clone for INPUT_MESSAGE_DEVICE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INPUT_MESSAGE_DEVICE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for INPUT_MESSAGE_DEVICE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for INPUT_MESSAGE_DEVICE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INPUT_MESSAGE_DEVICE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct INPUT_MESSAGE_ORIGIN_ID(pub i32);
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
pub const IMO_UNAVAILABLE: INPUT_MESSAGE_ORIGIN_ID = INPUT_MESSAGE_ORIGIN_ID(0i32);
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
pub const IMO_HARDWARE: INPUT_MESSAGE_ORIGIN_ID = INPUT_MESSAGE_ORIGIN_ID(1i32);
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
pub const IMO_INJECTED: INPUT_MESSAGE_ORIGIN_ID = INPUT_MESSAGE_ORIGIN_ID(2i32);
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
pub const IMO_SYSTEM: INPUT_MESSAGE_ORIGIN_ID = INPUT_MESSAGE_ORIGIN_ID(4i32);
impl ::core::marker::Copy for INPUT_MESSAGE_ORIGIN_ID {}
impl ::core::clone::Clone for INPUT_MESSAGE_ORIGIN_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INPUT_MESSAGE_ORIGIN_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for INPUT_MESSAGE_ORIGIN_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for INPUT_MESSAGE_ORIGIN_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INPUT_MESSAGE_ORIGIN_ID").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
pub struct INPUT_MESSAGE_SOURCE {
    pub deviceType: INPUT_MESSAGE_DEVICE_TYPE,
    pub originId: INPUT_MESSAGE_ORIGIN_ID,
}
impl ::core::marker::Copy for INPUT_MESSAGE_SOURCE {}
impl ::core::clone::Clone for INPUT_MESSAGE_SOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INPUT_MESSAGE_SOURCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INPUT_MESSAGE_SOURCE").field("deviceType", &self.deviceType).field("originId", &self.originId).finish()
    }
}
unsafe impl ::windows::core::Abi for INPUT_MESSAGE_SOURCE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for INPUT_MESSAGE_SOURCE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INPUT_MESSAGE_SOURCE>()) == 0 }
    }
}
impl ::core::cmp::Eq for INPUT_MESSAGE_SOURCE {}
impl ::core::default::Default for INPUT_MESSAGE_SOURCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
pub struct RAWHID {
    pub dwSizeHid: u32,
    pub dwCount: u32,
    pub bRawData: [u8; 1],
}
impl ::core::marker::Copy for RAWHID {}
impl ::core::clone::Clone for RAWHID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RAWHID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAWHID").field("dwSizeHid", &self.dwSizeHid).field("dwCount", &self.dwCount).field("bRawData", &self.bRawData).finish()
    }
}
unsafe impl ::windows::core::Abi for RAWHID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RAWHID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RAWHID>()) == 0 }
    }
}
impl ::core::cmp::Eq for RAWHID {}
impl ::core::default::Default for RAWHID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RAWINPUT {
    pub header: RAWINPUTHEADER,
    pub data: RAWINPUT_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RAWINPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RAWINPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RAWINPUT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RAWINPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RAWINPUT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RAWINPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RAWINPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union RAWINPUT_0 {
    pub mouse: RAWMOUSE,
    pub keyboard: RAWKEYBOARD,
    pub hid: RAWHID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RAWINPUT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RAWINPUT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RAWINPUT_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RAWINPUT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RAWINPUT_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RAWINPUT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RAWINPUT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RAWINPUTDEVICE {
    pub usUsagePage: u16,
    pub usUsage: u16,
    pub dwFlags: RAWINPUTDEVICE_FLAGS,
    pub hwndTarget: super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RAWINPUTDEVICE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RAWINPUTDEVICE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RAWINPUTDEVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAWINPUTDEVICE").field("usUsagePage", &self.usUsagePage).field("usUsage", &self.usUsage).field("dwFlags", &self.dwFlags).field("hwndTarget", &self.hwndTarget).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RAWINPUTDEVICE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RAWINPUTDEVICE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RAWINPUTDEVICE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RAWINPUTDEVICE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RAWINPUTDEVICE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RAWINPUTDEVICELIST {
    pub hDevice: super::super::Foundation::HANDLE,
    pub dwType: RID_DEVICE_INFO_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RAWINPUTDEVICELIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RAWINPUTDEVICELIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RAWINPUTDEVICELIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAWINPUTDEVICELIST").field("hDevice", &self.hDevice).field("dwType", &self.dwType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RAWINPUTDEVICELIST {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RAWINPUTDEVICELIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RAWINPUTDEVICELIST>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RAWINPUTDEVICELIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RAWINPUTDEVICELIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RAWINPUTDEVICE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
pub const RIDEV_REMOVE: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
pub const RIDEV_EXCLUDE: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
pub const RIDEV_PAGEONLY: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
pub const RIDEV_NOLEGACY: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(48u32);
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
pub const RIDEV_INPUTSINK: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
pub const RIDEV_CAPTUREMOUSE: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
pub const RIDEV_NOHOTKEYS: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
pub const RIDEV_APPKEYS: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
pub const RIDEV_EXINPUTSINK: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
pub const RIDEV_DEVNOTIFY: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(8192u32);
impl ::core::marker::Copy for RAWINPUTDEVICE_FLAGS {}
impl ::core::clone::Clone for RAWINPUTDEVICE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RAWINPUTDEVICE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RAWINPUTDEVICE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for RAWINPUTDEVICE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RAWINPUTDEVICE_FLAGS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RAWINPUTHEADER {
    pub dwType: u32,
    pub dwSize: u32,
    pub hDevice: super::super::Foundation::HANDLE,
    pub wParam: super::super::Foundation::WPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RAWINPUTHEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RAWINPUTHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RAWINPUTHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAWINPUTHEADER").field("dwType", &self.dwType).field("dwSize", &self.dwSize).field("hDevice", &self.hDevice).field("wParam", &self.wParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RAWINPUTHEADER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RAWINPUTHEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RAWINPUTHEADER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RAWINPUTHEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RAWINPUTHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
pub struct RAWKEYBOARD {
    pub MakeCode: u16,
    pub Flags: u16,
    pub Reserved: u16,
    pub VKey: u16,
    pub Message: u32,
    pub ExtraInformation: u32,
}
impl ::core::marker::Copy for RAWKEYBOARD {}
impl ::core::clone::Clone for RAWKEYBOARD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RAWKEYBOARD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAWKEYBOARD").field("MakeCode", &self.MakeCode).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("VKey", &self.VKey).field("Message", &self.Message).field("ExtraInformation", &self.ExtraInformation).finish()
    }
}
unsafe impl ::windows::core::Abi for RAWKEYBOARD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RAWKEYBOARD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RAWKEYBOARD>()) == 0 }
    }
}
impl ::core::cmp::Eq for RAWKEYBOARD {}
impl ::core::default::Default for RAWKEYBOARD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
pub struct RAWMOUSE {
    pub usFlags: u16,
    pub Anonymous: RAWMOUSE_0,
    pub ulRawButtons: u32,
    pub lLastX: i32,
    pub lLastY: i32,
    pub ulExtraInformation: u32,
}
impl ::core::marker::Copy for RAWMOUSE {}
impl ::core::clone::Clone for RAWMOUSE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RAWMOUSE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RAWMOUSE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RAWMOUSE>()) == 0 }
    }
}
impl ::core::cmp::Eq for RAWMOUSE {}
impl ::core::default::Default for RAWMOUSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
pub union RAWMOUSE_0 {
    pub ulButtons: u32,
    pub Anonymous: RAWMOUSE_0_0,
}
impl ::core::marker::Copy for RAWMOUSE_0 {}
impl ::core::clone::Clone for RAWMOUSE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RAWMOUSE_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RAWMOUSE_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RAWMOUSE_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for RAWMOUSE_0 {}
impl ::core::default::Default for RAWMOUSE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
pub struct RAWMOUSE_0_0 {
    pub usButtonFlags: u16,
    pub usButtonData: u16,
}
impl ::core::marker::Copy for RAWMOUSE_0_0 {}
impl ::core::clone::Clone for RAWMOUSE_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RAWMOUSE_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAWMOUSE_0_0").field("usButtonFlags", &self.usButtonFlags).field("usButtonData", &self.usButtonData).finish()
    }
}
unsafe impl ::windows::core::Abi for RAWMOUSE_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RAWMOUSE_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RAWMOUSE_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for RAWMOUSE_0_0 {}
impl ::core::default::Default for RAWMOUSE_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RAW_INPUT_DATA_COMMAND_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
pub const RID_HEADER: RAW_INPUT_DATA_COMMAND_FLAGS = RAW_INPUT_DATA_COMMAND_FLAGS(268435461u32);
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
pub const RID_INPUT: RAW_INPUT_DATA_COMMAND_FLAGS = RAW_INPUT_DATA_COMMAND_FLAGS(268435459u32);
impl ::core::marker::Copy for RAW_INPUT_DATA_COMMAND_FLAGS {}
impl ::core::clone::Clone for RAW_INPUT_DATA_COMMAND_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RAW_INPUT_DATA_COMMAND_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RAW_INPUT_DATA_COMMAND_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for RAW_INPUT_DATA_COMMAND_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RAW_INPUT_DATA_COMMAND_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RAW_INPUT_DEVICE_INFO_COMMAND(pub u32);
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
pub const RIDI_PREPARSEDDATA: RAW_INPUT_DEVICE_INFO_COMMAND = RAW_INPUT_DEVICE_INFO_COMMAND(536870917u32);
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
pub const RIDI_DEVICENAME: RAW_INPUT_DEVICE_INFO_COMMAND = RAW_INPUT_DEVICE_INFO_COMMAND(536870919u32);
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
pub const RIDI_DEVICEINFO: RAW_INPUT_DEVICE_INFO_COMMAND = RAW_INPUT_DEVICE_INFO_COMMAND(536870923u32);
impl ::core::marker::Copy for RAW_INPUT_DEVICE_INFO_COMMAND {}
impl ::core::clone::Clone for RAW_INPUT_DEVICE_INFO_COMMAND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RAW_INPUT_DEVICE_INFO_COMMAND {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RAW_INPUT_DEVICE_INFO_COMMAND {
    type Abi = Self;
}
impl ::core::fmt::Debug for RAW_INPUT_DEVICE_INFO_COMMAND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RAW_INPUT_DEVICE_INFO_COMMAND").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RID_DEVICE_INFO {
    pub cbSize: u32,
    pub dwType: RID_DEVICE_INFO_TYPE,
    pub Anonymous: RID_DEVICE_INFO_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RID_DEVICE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RID_DEVICE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RID_DEVICE_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RID_DEVICE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RID_DEVICE_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RID_DEVICE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RID_DEVICE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union RID_DEVICE_INFO_0 {
    pub mouse: RID_DEVICE_INFO_MOUSE,
    pub keyboard: RID_DEVICE_INFO_KEYBOARD,
    pub hid: RID_DEVICE_INFO_HID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RID_DEVICE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RID_DEVICE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RID_DEVICE_INFO_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RID_DEVICE_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RID_DEVICE_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RID_DEVICE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RID_DEVICE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
pub struct RID_DEVICE_INFO_HID {
    pub dwVendorId: u32,
    pub dwProductId: u32,
    pub dwVersionNumber: u32,
    pub usUsagePage: u16,
    pub usUsage: u16,
}
impl ::core::marker::Copy for RID_DEVICE_INFO_HID {}
impl ::core::clone::Clone for RID_DEVICE_INFO_HID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RID_DEVICE_INFO_HID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RID_DEVICE_INFO_HID").field("dwVendorId", &self.dwVendorId).field("dwProductId", &self.dwProductId).field("dwVersionNumber", &self.dwVersionNumber).field("usUsagePage", &self.usUsagePage).field("usUsage", &self.usUsage).finish()
    }
}
unsafe impl ::windows::core::Abi for RID_DEVICE_INFO_HID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RID_DEVICE_INFO_HID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RID_DEVICE_INFO_HID>()) == 0 }
    }
}
impl ::core::cmp::Eq for RID_DEVICE_INFO_HID {}
impl ::core::default::Default for RID_DEVICE_INFO_HID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
pub struct RID_DEVICE_INFO_KEYBOARD {
    pub dwType: u32,
    pub dwSubType: u32,
    pub dwKeyboardMode: u32,
    pub dwNumberOfFunctionKeys: u32,
    pub dwNumberOfIndicators: u32,
    pub dwNumberOfKeysTotal: u32,
}
impl ::core::marker::Copy for RID_DEVICE_INFO_KEYBOARD {}
impl ::core::clone::Clone for RID_DEVICE_INFO_KEYBOARD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RID_DEVICE_INFO_KEYBOARD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RID_DEVICE_INFO_KEYBOARD").field("dwType", &self.dwType).field("dwSubType", &self.dwSubType).field("dwKeyboardMode", &self.dwKeyboardMode).field("dwNumberOfFunctionKeys", &self.dwNumberOfFunctionKeys).field("dwNumberOfIndicators", &self.dwNumberOfIndicators).field("dwNumberOfKeysTotal", &self.dwNumberOfKeysTotal).finish()
    }
}
unsafe impl ::windows::core::Abi for RID_DEVICE_INFO_KEYBOARD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RID_DEVICE_INFO_KEYBOARD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RID_DEVICE_INFO_KEYBOARD>()) == 0 }
    }
}
impl ::core::cmp::Eq for RID_DEVICE_INFO_KEYBOARD {}
impl ::core::default::Default for RID_DEVICE_INFO_KEYBOARD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RID_DEVICE_INFO_MOUSE {
    pub dwId: u32,
    pub dwNumberOfButtons: u32,
    pub dwSampleRate: u32,
    pub fHasHorizontalWheel: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RID_DEVICE_INFO_MOUSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RID_DEVICE_INFO_MOUSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RID_DEVICE_INFO_MOUSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RID_DEVICE_INFO_MOUSE").field("dwId", &self.dwId).field("dwNumberOfButtons", &self.dwNumberOfButtons).field("dwSampleRate", &self.dwSampleRate).field("fHasHorizontalWheel", &self.fHasHorizontalWheel).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RID_DEVICE_INFO_MOUSE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RID_DEVICE_INFO_MOUSE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RID_DEVICE_INFO_MOUSE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RID_DEVICE_INFO_MOUSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RID_DEVICE_INFO_MOUSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RID_DEVICE_INFO_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
pub const RIM_TYPEMOUSE: RID_DEVICE_INFO_TYPE = RID_DEVICE_INFO_TYPE(0u32);
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
pub const RIM_TYPEKEYBOARD: RID_DEVICE_INFO_TYPE = RID_DEVICE_INFO_TYPE(1u32);
#[doc = "*Required features: `\"Win32_UI_Input\"`*"]
pub const RIM_TYPEHID: RID_DEVICE_INFO_TYPE = RID_DEVICE_INFO_TYPE(2u32);
impl ::core::marker::Copy for RID_DEVICE_INFO_TYPE {}
impl ::core::clone::Clone for RID_DEVICE_INFO_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RID_DEVICE_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RID_DEVICE_INFO_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RID_DEVICE_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RID_DEVICE_INFO_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Input\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterRawInputDevices(prawinputdevices: &[RAWINPUTDEVICE], cbsize: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterRawInputDevices(prawinputdevices: *const RAWINPUTDEVICE, uinumdevices: u32, cbsize: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(RegisterRawInputDevices(::core::mem::transmute(::windows::core::as_ptr_or_null(prawinputdevices)), prawinputdevices.len() as _, ::core::mem::transmute(cbsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
