#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnableMouseInPointer<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(fenable: Param0) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnableMouseInPointer(fenable: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EnableMouseInPointer(fenable.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPointerCursorId(pointerid: u32, cursorid: *mut u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPointerCursorId(pointerid: u32, cursorid: *mut u32) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetPointerCursorId(::core::mem::transmute(pointerid), ::core::mem::transmute(cursorid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_Controls\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls"))]
#[inline]
pub unsafe fn GetPointerDevice<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(device: Param0, pointerdevice: *mut super::super::Controls::POINTER_DEVICE_INFO) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPointerDevice(device: super::super::super::Foundation::HANDLE, pointerdevice: *mut super::super::Controls::POINTER_DEVICE_INFO) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetPointerDevice(device.into_param().abi(), ::core::mem::transmute(pointerdevice)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
#[inline]
pub unsafe fn GetPointerDeviceCursors<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(device: Param0, cursorcount: *mut u32, devicecursors: *mut super::super::Controls::POINTER_DEVICE_CURSOR_INFO) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPointerDeviceCursors(device: super::super::super::Foundation::HANDLE, cursorcount: *mut u32, devicecursors: *mut super::super::Controls::POINTER_DEVICE_CURSOR_INFO) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetPointerDeviceCursors(device.into_param().abi(), ::core::mem::transmute(cursorcount), ::core::mem::transmute(devicecursors)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
#[inline]
pub unsafe fn GetPointerDeviceProperties<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(device: Param0, propertycount: *mut u32, pointerproperties: *mut super::super::Controls::POINTER_DEVICE_PROPERTY) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPointerDeviceProperties(device: super::super::super::Foundation::HANDLE, propertycount: *mut u32, pointerproperties: *mut super::super::Controls::POINTER_DEVICE_PROPERTY) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetPointerDeviceProperties(device.into_param().abi(), ::core::mem::transmute(propertycount), ::core::mem::transmute(pointerproperties)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPointerDeviceRects<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(device: Param0, pointerdevicerect: *mut super::super::super::Foundation::RECT, displayrect: *mut super::super::super::Foundation::RECT) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPointerDeviceRects(device: super::super::super::Foundation::HANDLE, pointerdevicerect: *mut super::super::super::Foundation::RECT, displayrect: *mut super::super::super::Foundation::RECT) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetPointerDeviceRects(device.into_param().abi(), ::core::mem::transmute(pointerdevicerect), ::core::mem::transmute(displayrect)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_Controls\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls"))]
#[inline]
pub unsafe fn GetPointerDevices(devicecount: *mut u32, pointerdevices: *mut super::super::Controls::POINTER_DEVICE_INFO) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPointerDevices(devicecount: *mut u32, pointerdevices: *mut super::super::Controls::POINTER_DEVICE_INFO) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetPointerDevices(::core::mem::transmute(devicecount), ::core::mem::transmute(pointerdevices)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn GetPointerFrameInfo(pointerid: u32, pointercount: *mut u32, pointerinfo: *mut POINTER_INFO) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPointerFrameInfo(pointerid: u32, pointercount: *mut u32, pointerinfo: *mut POINTER_INFO) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetPointerFrameInfo(::core::mem::transmute(pointerid), ::core::mem::transmute(pointercount), ::core::mem::transmute(pointerinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn GetPointerFrameInfoHistory(pointerid: u32, entriescount: *mut u32, pointercount: *mut u32, pointerinfo: *mut POINTER_INFO) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPointerFrameInfoHistory(pointerid: u32, entriescount: *mut u32, pointercount: *mut u32, pointerinfo: *mut POINTER_INFO) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetPointerFrameInfoHistory(::core::mem::transmute(pointerid), ::core::mem::transmute(entriescount), ::core::mem::transmute(pointercount), ::core::mem::transmute(pointerinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn GetPointerFramePenInfo(pointerid: u32, pointercount: *mut u32, peninfo: *mut POINTER_PEN_INFO) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPointerFramePenInfo(pointerid: u32, pointercount: *mut u32, peninfo: *mut POINTER_PEN_INFO) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetPointerFramePenInfo(::core::mem::transmute(pointerid), ::core::mem::transmute(pointercount), ::core::mem::transmute(peninfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn GetPointerFramePenInfoHistory(pointerid: u32, entriescount: *mut u32, pointercount: *mut u32, peninfo: *mut POINTER_PEN_INFO) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPointerFramePenInfoHistory(pointerid: u32, entriescount: *mut u32, pointercount: *mut u32, peninfo: *mut POINTER_PEN_INFO) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetPointerFramePenInfoHistory(::core::mem::transmute(pointerid), ::core::mem::transmute(entriescount), ::core::mem::transmute(pointercount), ::core::mem::transmute(peninfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn GetPointerFrameTouchInfo(pointerid: u32, pointercount: *mut u32, touchinfo: *mut POINTER_TOUCH_INFO) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPointerFrameTouchInfo(pointerid: u32, pointercount: *mut u32, touchinfo: *mut POINTER_TOUCH_INFO) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetPointerFrameTouchInfo(::core::mem::transmute(pointerid), ::core::mem::transmute(pointercount), ::core::mem::transmute(touchinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn GetPointerFrameTouchInfoHistory(pointerid: u32, entriescount: *mut u32, pointercount: *mut u32, touchinfo: *mut POINTER_TOUCH_INFO) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPointerFrameTouchInfoHistory(pointerid: u32, entriescount: *mut u32, pointercount: *mut u32, touchinfo: *mut POINTER_TOUCH_INFO) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetPointerFrameTouchInfoHistory(::core::mem::transmute(pointerid), ::core::mem::transmute(entriescount), ::core::mem::transmute(pointercount), ::core::mem::transmute(touchinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn GetPointerInfo(pointerid: u32, pointerinfo: *mut POINTER_INFO) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPointerInfo(pointerid: u32, pointerinfo: *mut POINTER_INFO) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetPointerInfo(::core::mem::transmute(pointerid), ::core::mem::transmute(pointerinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn GetPointerInfoHistory(pointerid: u32, entriescount: *mut u32, pointerinfo: *mut POINTER_INFO) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPointerInfoHistory(pointerid: u32, entriescount: *mut u32, pointerinfo: *mut POINTER_INFO) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetPointerInfoHistory(::core::mem::transmute(pointerid), ::core::mem::transmute(entriescount), ::core::mem::transmute(pointerinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPointerInputTransform(pointerid: u32, inputtransform: &mut [INPUT_TRANSFORM]) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPointerInputTransform(pointerid: u32, historycount: u32, inputtransform: *mut INPUT_TRANSFORM) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetPointerInputTransform(::core::mem::transmute(pointerid), inputtransform.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(inputtransform))))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn GetPointerPenInfo(pointerid: u32, peninfo: *mut POINTER_PEN_INFO) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPointerPenInfo(pointerid: u32, peninfo: *mut POINTER_PEN_INFO) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetPointerPenInfo(::core::mem::transmute(pointerid), ::core::mem::transmute(peninfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn GetPointerPenInfoHistory(pointerid: u32, entriescount: *mut u32, peninfo: *mut POINTER_PEN_INFO) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPointerPenInfoHistory(pointerid: u32, entriescount: *mut u32, peninfo: *mut POINTER_PEN_INFO) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetPointerPenInfoHistory(::core::mem::transmute(pointerid), ::core::mem::transmute(entriescount), ::core::mem::transmute(peninfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn GetPointerTouchInfo(pointerid: u32, touchinfo: *mut POINTER_TOUCH_INFO) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPointerTouchInfo(pointerid: u32, touchinfo: *mut POINTER_TOUCH_INFO) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetPointerTouchInfo(::core::mem::transmute(pointerid), ::core::mem::transmute(touchinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn GetPointerTouchInfoHistory(pointerid: u32, entriescount: *mut u32, touchinfo: *mut POINTER_TOUCH_INFO) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPointerTouchInfoHistory(pointerid: u32, entriescount: *mut u32, touchinfo: *mut POINTER_TOUCH_INFO) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetPointerTouchInfoHistory(::core::mem::transmute(pointerid), ::core::mem::transmute(entriescount), ::core::mem::transmute(touchinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn GetPointerType(pointerid: u32, pointertype: *mut super::super::WindowsAndMessaging::POINTER_INPUT_TYPE) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPointerType(pointerid: u32, pointertype: *mut super::super::WindowsAndMessaging::POINTER_INPUT_TYPE) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetPointerType(::core::mem::transmute(pointerid), ::core::mem::transmute(pointertype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
#[inline]
pub unsafe fn GetRawPointerDeviceData(pointerid: u32, historycount: u32, pproperties: &[super::super::Controls::POINTER_DEVICE_PROPERTY], pvalues: *mut i32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetRawPointerDeviceData(pointerid: u32, historycount: u32, propertiescount: u32, pproperties: *const super::super::Controls::POINTER_DEVICE_PROPERTY, pvalues: *mut i32) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetRawPointerDeviceData(::core::mem::transmute(pointerid), ::core::mem::transmute(historycount), pproperties.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pproperties)), ::core::mem::transmute(pvalues)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
#[inline]
pub unsafe fn GetUnpredictedMessagePos() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetUnpredictedMessagePos() -> u32;
        }
        ::core::mem::transmute(GetUnpredictedMessagePos())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
pub struct INPUT_INJECTION_VALUE {
    pub page: u16,
    pub usage: u16,
    pub value: i32,
    pub index: u16,
}
impl ::core::marker::Copy for INPUT_INJECTION_VALUE {}
impl ::core::clone::Clone for INPUT_INJECTION_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INPUT_INJECTION_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INPUT_INJECTION_VALUE").field("page", &self.page).field("usage", &self.usage).field("value", &self.value).field("index", &self.index).finish()
    }
}
unsafe impl ::windows::core::Abi for INPUT_INJECTION_VALUE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for INPUT_INJECTION_VALUE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INPUT_INJECTION_VALUE>()) == 0 }
    }
}
impl ::core::cmp::Eq for INPUT_INJECTION_VALUE {}
impl ::core::default::Default for INPUT_INJECTION_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
pub struct INPUT_TRANSFORM {
    pub Anonymous: INPUT_TRANSFORM_0,
}
impl ::core::marker::Copy for INPUT_TRANSFORM {}
impl ::core::clone::Clone for INPUT_TRANSFORM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for INPUT_TRANSFORM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for INPUT_TRANSFORM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INPUT_TRANSFORM>()) == 0 }
    }
}
impl ::core::cmp::Eq for INPUT_TRANSFORM {}
impl ::core::default::Default for INPUT_TRANSFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
pub union INPUT_TRANSFORM_0 {
    pub Anonymous: INPUT_TRANSFORM_0_0,
    pub m: [f32; 16],
}
impl ::core::marker::Copy for INPUT_TRANSFORM_0 {}
impl ::core::clone::Clone for INPUT_TRANSFORM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for INPUT_TRANSFORM_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for INPUT_TRANSFORM_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INPUT_TRANSFORM_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for INPUT_TRANSFORM_0 {}
impl ::core::default::Default for INPUT_TRANSFORM_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
pub struct INPUT_TRANSFORM_0_0 {
    pub _11: f32,
    pub _12: f32,
    pub _13: f32,
    pub _14: f32,
    pub _21: f32,
    pub _22: f32,
    pub _23: f32,
    pub _24: f32,
    pub _31: f32,
    pub _32: f32,
    pub _33: f32,
    pub _34: f32,
    pub _41: f32,
    pub _42: f32,
    pub _43: f32,
    pub _44: f32,
}
impl ::core::marker::Copy for INPUT_TRANSFORM_0_0 {}
impl ::core::clone::Clone for INPUT_TRANSFORM_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INPUT_TRANSFORM_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INPUT_TRANSFORM_0_0").field("_11", &self._11).field("_12", &self._12).field("_13", &self._13).field("_14", &self._14).field("_21", &self._21).field("_22", &self._22).field("_23", &self._23).field("_24", &self._24).field("_31", &self._31).field("_32", &self._32).field("_33", &self._33).field("_34", &self._34).field("_41", &self._41).field("_42", &self._42).field("_43", &self._43).field("_44", &self._44).finish()
    }
}
unsafe impl ::windows::core::Abi for INPUT_TRANSFORM_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for INPUT_TRANSFORM_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INPUT_TRANSFORM_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for INPUT_TRANSFORM_0_0 {}
impl ::core::default::Default for INPUT_TRANSFORM_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InitializeTouchInjection(maxcount: u32, dwmode: TOUCH_FEEDBACK_MODE) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitializeTouchInjection(maxcount: u32, dwmode: TOUCH_FEEDBACK_MODE) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InitializeTouchInjection(::core::mem::transmute(maxcount), ::core::mem::transmute(dwmode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn InjectSyntheticPointerInput<'a, Param0: ::windows::core::IntoParam<'a, super::super::Controls::HSYNTHETICPOINTERDEVICE>>(device: Param0, pointerinfo: &[super::super::Controls::POINTER_TYPE_INFO]) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InjectSyntheticPointerInput(device: super::super::Controls::HSYNTHETICPOINTERDEVICE, pointerinfo: *const super::super::Controls::POINTER_TYPE_INFO, count: u32) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InjectSyntheticPointerInput(device.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(pointerinfo)), pointerinfo.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn InjectTouchInput(contacts: &[POINTER_TOUCH_INFO]) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InjectTouchInput(count: u32, contacts: *const POINTER_TOUCH_INFO) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InjectTouchInput(contacts.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(contacts))))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsMouseInPointerEnabled() -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsMouseInPointerEnabled() -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsMouseInPointerEnabled())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct POINTER_BUTTON_CHANGE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
pub const POINTER_CHANGE_NONE: POINTER_BUTTON_CHANGE_TYPE = POINTER_BUTTON_CHANGE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
pub const POINTER_CHANGE_FIRSTBUTTON_DOWN: POINTER_BUTTON_CHANGE_TYPE = POINTER_BUTTON_CHANGE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
pub const POINTER_CHANGE_FIRSTBUTTON_UP: POINTER_BUTTON_CHANGE_TYPE = POINTER_BUTTON_CHANGE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
pub const POINTER_CHANGE_SECONDBUTTON_DOWN: POINTER_BUTTON_CHANGE_TYPE = POINTER_BUTTON_CHANGE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
pub const POINTER_CHANGE_SECONDBUTTON_UP: POINTER_BUTTON_CHANGE_TYPE = POINTER_BUTTON_CHANGE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
pub const POINTER_CHANGE_THIRDBUTTON_DOWN: POINTER_BUTTON_CHANGE_TYPE = POINTER_BUTTON_CHANGE_TYPE(5i32);
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
pub const POINTER_CHANGE_THIRDBUTTON_UP: POINTER_BUTTON_CHANGE_TYPE = POINTER_BUTTON_CHANGE_TYPE(6i32);
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
pub const POINTER_CHANGE_FOURTHBUTTON_DOWN: POINTER_BUTTON_CHANGE_TYPE = POINTER_BUTTON_CHANGE_TYPE(7i32);
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
pub const POINTER_CHANGE_FOURTHBUTTON_UP: POINTER_BUTTON_CHANGE_TYPE = POINTER_BUTTON_CHANGE_TYPE(8i32);
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
pub const POINTER_CHANGE_FIFTHBUTTON_DOWN: POINTER_BUTTON_CHANGE_TYPE = POINTER_BUTTON_CHANGE_TYPE(9i32);
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
pub const POINTER_CHANGE_FIFTHBUTTON_UP: POINTER_BUTTON_CHANGE_TYPE = POINTER_BUTTON_CHANGE_TYPE(10i32);
impl ::core::marker::Copy for POINTER_BUTTON_CHANGE_TYPE {}
impl ::core::clone::Clone for POINTER_BUTTON_CHANGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for POINTER_BUTTON_CHANGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for POINTER_BUTTON_CHANGE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for POINTER_BUTTON_CHANGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POINTER_BUTTON_CHANGE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct POINTER_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
pub const POINTER_FLAG_NONE: POINTER_FLAGS = POINTER_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
pub const POINTER_FLAG_NEW: POINTER_FLAGS = POINTER_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
pub const POINTER_FLAG_INRANGE: POINTER_FLAGS = POINTER_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
pub const POINTER_FLAG_INCONTACT: POINTER_FLAGS = POINTER_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
pub const POINTER_FLAG_FIRSTBUTTON: POINTER_FLAGS = POINTER_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
pub const POINTER_FLAG_SECONDBUTTON: POINTER_FLAGS = POINTER_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
pub const POINTER_FLAG_THIRDBUTTON: POINTER_FLAGS = POINTER_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
pub const POINTER_FLAG_FOURTHBUTTON: POINTER_FLAGS = POINTER_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
pub const POINTER_FLAG_FIFTHBUTTON: POINTER_FLAGS = POINTER_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
pub const POINTER_FLAG_PRIMARY: POINTER_FLAGS = POINTER_FLAGS(8192u32);
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
pub const POINTER_FLAG_CONFIDENCE: POINTER_FLAGS = POINTER_FLAGS(16384u32);
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
pub const POINTER_FLAG_CANCELED: POINTER_FLAGS = POINTER_FLAGS(32768u32);
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
pub const POINTER_FLAG_DOWN: POINTER_FLAGS = POINTER_FLAGS(65536u32);
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
pub const POINTER_FLAG_UPDATE: POINTER_FLAGS = POINTER_FLAGS(131072u32);
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
pub const POINTER_FLAG_UP: POINTER_FLAGS = POINTER_FLAGS(262144u32);
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
pub const POINTER_FLAG_WHEEL: POINTER_FLAGS = POINTER_FLAGS(524288u32);
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
pub const POINTER_FLAG_HWHEEL: POINTER_FLAGS = POINTER_FLAGS(1048576u32);
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
pub const POINTER_FLAG_CAPTURECHANGED: POINTER_FLAGS = POINTER_FLAGS(2097152u32);
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
pub const POINTER_FLAG_HASTRANSFORM: POINTER_FLAGS = POINTER_FLAGS(4194304u32);
impl ::core::marker::Copy for POINTER_FLAGS {}
impl ::core::clone::Clone for POINTER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for POINTER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for POINTER_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for POINTER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POINTER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for POINTER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for POINTER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for POINTER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for POINTER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for POINTER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct POINTER_INFO {
    pub pointerType: super::super::WindowsAndMessaging::POINTER_INPUT_TYPE,
    pub pointerId: u32,
    pub frameId: u32,
    pub pointerFlags: POINTER_FLAGS,
    pub sourceDevice: super::super::super::Foundation::HANDLE,
    pub hwndTarget: super::super::super::Foundation::HWND,
    pub ptPixelLocation: super::super::super::Foundation::POINT,
    pub ptHimetricLocation: super::super::super::Foundation::POINT,
    pub ptPixelLocationRaw: super::super::super::Foundation::POINT,
    pub ptHimetricLocationRaw: super::super::super::Foundation::POINT,
    pub dwTime: u32,
    pub historyCount: u32,
    pub InputData: i32,
    pub dwKeyStates: u32,
    pub PerformanceCount: u64,
    pub ButtonChangeType: POINTER_BUTTON_CHANGE_TYPE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for POINTER_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for POINTER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for POINTER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POINTER_INFO")
            .field("pointerType", &self.pointerType)
            .field("pointerId", &self.pointerId)
            .field("frameId", &self.frameId)
            .field("pointerFlags", &self.pointerFlags)
            .field("sourceDevice", &self.sourceDevice)
            .field("hwndTarget", &self.hwndTarget)
            .field("ptPixelLocation", &self.ptPixelLocation)
            .field("ptHimetricLocation", &self.ptHimetricLocation)
            .field("ptPixelLocationRaw", &self.ptPixelLocationRaw)
            .field("ptHimetricLocationRaw", &self.ptHimetricLocationRaw)
            .field("dwTime", &self.dwTime)
            .field("historyCount", &self.historyCount)
            .field("InputData", &self.InputData)
            .field("dwKeyStates", &self.dwKeyStates)
            .field("PerformanceCount", &self.PerformanceCount)
            .field("ButtonChangeType", &self.ButtonChangeType)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for POINTER_INFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for POINTER_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<POINTER_INFO>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for POINTER_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for POINTER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct POINTER_PEN_INFO {
    pub pointerInfo: POINTER_INFO,
    pub penFlags: u32,
    pub penMask: u32,
    pub pressure: u32,
    pub rotation: u32,
    pub tiltX: i32,
    pub tiltY: i32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for POINTER_PEN_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for POINTER_PEN_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for POINTER_PEN_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POINTER_PEN_INFO").field("pointerInfo", &self.pointerInfo).field("penFlags", &self.penFlags).field("penMask", &self.penMask).field("pressure", &self.pressure).field("rotation", &self.rotation).field("tiltX", &self.tiltX).field("tiltY", &self.tiltY).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for POINTER_PEN_INFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for POINTER_PEN_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<POINTER_PEN_INFO>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for POINTER_PEN_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for POINTER_PEN_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct POINTER_TOUCH_INFO {
    pub pointerInfo: POINTER_INFO,
    pub touchFlags: u32,
    pub touchMask: u32,
    pub rcContact: super::super::super::Foundation::RECT,
    pub rcContactRaw: super::super::super::Foundation::RECT,
    pub orientation: u32,
    pub pressure: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for POINTER_TOUCH_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for POINTER_TOUCH_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for POINTER_TOUCH_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POINTER_TOUCH_INFO").field("pointerInfo", &self.pointerInfo).field("touchFlags", &self.touchFlags).field("touchMask", &self.touchMask).field("rcContact", &self.rcContact).field("rcContactRaw", &self.rcContactRaw).field("orientation", &self.orientation).field("pressure", &self.pressure).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for POINTER_TOUCH_INFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for POINTER_TOUCH_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<POINTER_TOUCH_INFO>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for POINTER_TOUCH_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for POINTER_TOUCH_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SkipPointerFrameMessages(pointerid: u32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SkipPointerFrameMessages(pointerid: u32) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SkipPointerFrameMessages(::core::mem::transmute(pointerid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TOUCH_FEEDBACK_MODE(pub u32);
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
pub const TOUCH_FEEDBACK_DEFAULT: TOUCH_FEEDBACK_MODE = TOUCH_FEEDBACK_MODE(1u32);
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
pub const TOUCH_FEEDBACK_INDIRECT: TOUCH_FEEDBACK_MODE = TOUCH_FEEDBACK_MODE(2u32);
#[doc = "*Required features: `\"Win32_UI_Input_Pointer\"`*"]
pub const TOUCH_FEEDBACK_NONE: TOUCH_FEEDBACK_MODE = TOUCH_FEEDBACK_MODE(3u32);
impl ::core::marker::Copy for TOUCH_FEEDBACK_MODE {}
impl ::core::clone::Clone for TOUCH_FEEDBACK_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TOUCH_FEEDBACK_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TOUCH_FEEDBACK_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TOUCH_FEEDBACK_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TOUCH_FEEDBACK_MODE").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
