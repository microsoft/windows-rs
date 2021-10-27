#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn EnableMouseInPointer<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    fenable: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnableMouseInPointer(
                fenable: super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EnableMouseInPointer(fenable.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetPointerCursorId(
    pointerid: u32,
    cursorid: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPointerCursorId(
                pointerid: u32,
                cursorid: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetPointerCursorId(
            ::std::mem::transmute(pointerid),
            ::std::mem::transmute(cursorid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub unsafe fn GetPointerFrameInfo(
    pointerid: u32,
    pointercount: *mut u32,
    pointerinfo: *mut POINTER_INFO,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPointerFrameInfo(
                pointerid: u32,
                pointercount: *mut u32,
                pointerinfo: *mut POINTER_INFO,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetPointerFrameInfo(
            ::std::mem::transmute(pointerid),
            ::std::mem::transmute(pointercount),
            ::std::mem::transmute(pointerinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub unsafe fn GetPointerFrameInfoHistory(
    pointerid: u32,
    entriescount: *mut u32,
    pointercount: *mut u32,
    pointerinfo: *mut POINTER_INFO,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPointerFrameInfoHistory(
                pointerid: u32,
                entriescount: *mut u32,
                pointercount: *mut u32,
                pointerinfo: *mut POINTER_INFO,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetPointerFrameInfoHistory(
            ::std::mem::transmute(pointerid),
            ::std::mem::transmute(entriescount),
            ::std::mem::transmute(pointercount),
            ::std::mem::transmute(pointerinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub unsafe fn GetPointerFramePenInfo(
    pointerid: u32,
    pointercount: *mut u32,
    peninfo: *mut POINTER_PEN_INFO,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPointerFramePenInfo(
                pointerid: u32,
                pointercount: *mut u32,
                peninfo: *mut POINTER_PEN_INFO,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetPointerFramePenInfo(
            ::std::mem::transmute(pointerid),
            ::std::mem::transmute(pointercount),
            ::std::mem::transmute(peninfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub unsafe fn GetPointerFramePenInfoHistory(
    pointerid: u32,
    entriescount: *mut u32,
    pointercount: *mut u32,
    peninfo: *mut POINTER_PEN_INFO,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPointerFramePenInfoHistory(
                pointerid: u32,
                entriescount: *mut u32,
                pointercount: *mut u32,
                peninfo: *mut POINTER_PEN_INFO,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetPointerFramePenInfoHistory(
            ::std::mem::transmute(pointerid),
            ::std::mem::transmute(entriescount),
            ::std::mem::transmute(pointercount),
            ::std::mem::transmute(peninfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub unsafe fn GetPointerFrameTouchInfo(
    pointerid: u32,
    pointercount: *mut u32,
    touchinfo: *mut POINTER_TOUCH_INFO,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPointerFrameTouchInfo(
                pointerid: u32,
                pointercount: *mut u32,
                touchinfo: *mut POINTER_TOUCH_INFO,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetPointerFrameTouchInfo(
            ::std::mem::transmute(pointerid),
            ::std::mem::transmute(pointercount),
            ::std::mem::transmute(touchinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub unsafe fn GetPointerFrameTouchInfoHistory(
    pointerid: u32,
    entriescount: *mut u32,
    pointercount: *mut u32,
    touchinfo: *mut POINTER_TOUCH_INFO,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPointerFrameTouchInfoHistory(
                pointerid: u32,
                entriescount: *mut u32,
                pointercount: *mut u32,
                touchinfo: *mut POINTER_TOUCH_INFO,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetPointerFrameTouchInfoHistory(
            ::std::mem::transmute(pointerid),
            ::std::mem::transmute(entriescount),
            ::std::mem::transmute(pointercount),
            ::std::mem::transmute(touchinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub unsafe fn GetPointerInfo(
    pointerid: u32,
    pointerinfo: *mut POINTER_INFO,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPointerInfo(
                pointerid: u32,
                pointerinfo: *mut POINTER_INFO,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetPointerInfo(
            ::std::mem::transmute(pointerid),
            ::std::mem::transmute(pointerinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub unsafe fn GetPointerInfoHistory(
    pointerid: u32,
    entriescount: *mut u32,
    pointerinfo: *mut POINTER_INFO,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPointerInfoHistory(
                pointerid: u32,
                entriescount: *mut u32,
                pointerinfo: *mut POINTER_INFO,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetPointerInfoHistory(
            ::std::mem::transmute(pointerid),
            ::std::mem::transmute(entriescount),
            ::std::mem::transmute(pointerinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetPointerInputTransform(
    pointerid: u32,
    historycount: u32,
    inputtransform: *mut INPUT_TRANSFORM,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPointerInputTransform(
                pointerid: u32,
                historycount: u32,
                inputtransform: *mut INPUT_TRANSFORM,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetPointerInputTransform(
            ::std::mem::transmute(pointerid),
            ::std::mem::transmute(historycount),
            ::std::mem::transmute(inputtransform),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub unsafe fn GetPointerPenInfo(
    pointerid: u32,
    peninfo: *mut POINTER_PEN_INFO,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPointerPenInfo(
                pointerid: u32,
                peninfo: *mut POINTER_PEN_INFO,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetPointerPenInfo(
            ::std::mem::transmute(pointerid),
            ::std::mem::transmute(peninfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub unsafe fn GetPointerPenInfoHistory(
    pointerid: u32,
    entriescount: *mut u32,
    peninfo: *mut POINTER_PEN_INFO,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPointerPenInfoHistory(
                pointerid: u32,
                entriescount: *mut u32,
                peninfo: *mut POINTER_PEN_INFO,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetPointerPenInfoHistory(
            ::std::mem::transmute(pointerid),
            ::std::mem::transmute(entriescount),
            ::std::mem::transmute(peninfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub unsafe fn GetPointerTouchInfo(
    pointerid: u32,
    touchinfo: *mut POINTER_TOUCH_INFO,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPointerTouchInfo(
                pointerid: u32,
                touchinfo: *mut POINTER_TOUCH_INFO,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetPointerTouchInfo(
            ::std::mem::transmute(pointerid),
            ::std::mem::transmute(touchinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub unsafe fn GetPointerTouchInfoHistory(
    pointerid: u32,
    entriescount: *mut u32,
    touchinfo: *mut POINTER_TOUCH_INFO,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPointerTouchInfoHistory(
                pointerid: u32,
                entriescount: *mut u32,
                touchinfo: *mut POINTER_TOUCH_INFO,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetPointerTouchInfoHistory(
            ::std::mem::transmute(pointerid),
            ::std::mem::transmute(entriescount),
            ::std::mem::transmute(touchinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub unsafe fn GetPointerType(
    pointerid: u32,
    pointertype: *mut super::WindowsAndMessaging::POINTER_INPUT_TYPE,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPointerType(
                pointerid: u32,
                pointertype: *mut super::WindowsAndMessaging::POINTER_INPUT_TYPE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetPointerType(
            ::std::mem::transmute(pointerid),
            ::std::mem::transmute(pointertype),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn GetUnpredictedMessagePos() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetUnpredictedMessagePos() -> u32;
        }
        ::std::mem::transmute(GetUnpredictedMessagePos())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct INPUT_TRANSFORM {
    pub Anonymous: INPUT_TRANSFORM_0,
}
impl INPUT_TRANSFORM {}
impl ::std::default::Default for INPUT_TRANSFORM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for INPUT_TRANSFORM {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for INPUT_TRANSFORM {}
unsafe impl ::windows::runtime::Abi for INPUT_TRANSFORM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union INPUT_TRANSFORM_0 {
    pub Anonymous: INPUT_TRANSFORM_0_0,
    pub m: [f32; 16],
}
impl INPUT_TRANSFORM_0 {}
impl ::std::default::Default for INPUT_TRANSFORM_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for INPUT_TRANSFORM_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for INPUT_TRANSFORM_0 {}
unsafe impl ::windows::runtime::Abi for INPUT_TRANSFORM_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl INPUT_TRANSFORM_0_0 {}
impl ::std::default::Default for INPUT_TRANSFORM_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for INPUT_TRANSFORM_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_11", &self._11)
            .field("_12", &self._12)
            .field("_13", &self._13)
            .field("_14", &self._14)
            .field("_21", &self._21)
            .field("_22", &self._22)
            .field("_23", &self._23)
            .field("_24", &self._24)
            .field("_31", &self._31)
            .field("_32", &self._32)
            .field("_33", &self._33)
            .field("_34", &self._34)
            .field("_41", &self._41)
            .field("_42", &self._42)
            .field("_43", &self._43)
            .field("_44", &self._44)
            .finish()
    }
}
impl ::std::cmp::PartialEq for INPUT_TRANSFORM_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._11 == other._11
            && self._12 == other._12
            && self._13 == other._13
            && self._14 == other._14
            && self._21 == other._21
            && self._22 == other._22
            && self._23 == other._23
            && self._24 == other._24
            && self._31 == other._31
            && self._32 == other._32
            && self._33 == other._33
            && self._34 == other._34
            && self._41 == other._41
            && self._42 == other._42
            && self._43 == other._43
            && self._44 == other._44
    }
}
impl ::std::cmp::Eq for INPUT_TRANSFORM_0_0 {}
unsafe impl ::windows::runtime::Abi for INPUT_TRANSFORM_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn IsMouseInPointerEnabled() -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsMouseInPointerEnabled() -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsMouseInPointerEnabled())
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
pub struct POINTER_BUTTON_CHANGE_TYPE(pub i32);
pub const POINTER_CHANGE_NONE: POINTER_BUTTON_CHANGE_TYPE = POINTER_BUTTON_CHANGE_TYPE(0i32);
pub const POINTER_CHANGE_FIRSTBUTTON_DOWN: POINTER_BUTTON_CHANGE_TYPE =
    POINTER_BUTTON_CHANGE_TYPE(1i32);
pub const POINTER_CHANGE_FIRSTBUTTON_UP: POINTER_BUTTON_CHANGE_TYPE =
    POINTER_BUTTON_CHANGE_TYPE(2i32);
pub const POINTER_CHANGE_SECONDBUTTON_DOWN: POINTER_BUTTON_CHANGE_TYPE =
    POINTER_BUTTON_CHANGE_TYPE(3i32);
pub const POINTER_CHANGE_SECONDBUTTON_UP: POINTER_BUTTON_CHANGE_TYPE =
    POINTER_BUTTON_CHANGE_TYPE(4i32);
pub const POINTER_CHANGE_THIRDBUTTON_DOWN: POINTER_BUTTON_CHANGE_TYPE =
    POINTER_BUTTON_CHANGE_TYPE(5i32);
pub const POINTER_CHANGE_THIRDBUTTON_UP: POINTER_BUTTON_CHANGE_TYPE =
    POINTER_BUTTON_CHANGE_TYPE(6i32);
pub const POINTER_CHANGE_FOURTHBUTTON_DOWN: POINTER_BUTTON_CHANGE_TYPE =
    POINTER_BUTTON_CHANGE_TYPE(7i32);
pub const POINTER_CHANGE_FOURTHBUTTON_UP: POINTER_BUTTON_CHANGE_TYPE =
    POINTER_BUTTON_CHANGE_TYPE(8i32);
pub const POINTER_CHANGE_FIFTHBUTTON_DOWN: POINTER_BUTTON_CHANGE_TYPE =
    POINTER_BUTTON_CHANGE_TYPE(9i32);
pub const POINTER_CHANGE_FIFTHBUTTON_UP: POINTER_BUTTON_CHANGE_TYPE =
    POINTER_BUTTON_CHANGE_TYPE(10i32);
impl ::std::convert::From<i32> for POINTER_BUTTON_CHANGE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for POINTER_BUTTON_CHANGE_TYPE {
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
pub struct POINTER_FLAGS(pub u32);
pub const POINTER_FLAG_NONE: POINTER_FLAGS = POINTER_FLAGS(0u32);
pub const POINTER_FLAG_NEW: POINTER_FLAGS = POINTER_FLAGS(1u32);
pub const POINTER_FLAG_INRANGE: POINTER_FLAGS = POINTER_FLAGS(2u32);
pub const POINTER_FLAG_INCONTACT: POINTER_FLAGS = POINTER_FLAGS(4u32);
pub const POINTER_FLAG_FIRSTBUTTON: POINTER_FLAGS = POINTER_FLAGS(16u32);
pub const POINTER_FLAG_SECONDBUTTON: POINTER_FLAGS = POINTER_FLAGS(32u32);
pub const POINTER_FLAG_THIRDBUTTON: POINTER_FLAGS = POINTER_FLAGS(64u32);
pub const POINTER_FLAG_FOURTHBUTTON: POINTER_FLAGS = POINTER_FLAGS(128u32);
pub const POINTER_FLAG_FIFTHBUTTON: POINTER_FLAGS = POINTER_FLAGS(256u32);
pub const POINTER_FLAG_PRIMARY: POINTER_FLAGS = POINTER_FLAGS(8192u32);
pub const POINTER_FLAG_CONFIDENCE: POINTER_FLAGS = POINTER_FLAGS(16384u32);
pub const POINTER_FLAG_CANCELED: POINTER_FLAGS = POINTER_FLAGS(32768u32);
pub const POINTER_FLAG_DOWN: POINTER_FLAGS = POINTER_FLAGS(65536u32);
pub const POINTER_FLAG_UPDATE: POINTER_FLAGS = POINTER_FLAGS(131072u32);
pub const POINTER_FLAG_UP: POINTER_FLAGS = POINTER_FLAGS(262144u32);
pub const POINTER_FLAG_WHEEL: POINTER_FLAGS = POINTER_FLAGS(524288u32);
pub const POINTER_FLAG_HWHEEL: POINTER_FLAGS = POINTER_FLAGS(1048576u32);
pub const POINTER_FLAG_CAPTURECHANGED: POINTER_FLAGS = POINTER_FLAGS(2097152u32);
pub const POINTER_FLAG_HASTRANSFORM: POINTER_FLAGS = POINTER_FLAGS(4194304u32);
impl ::std::convert::From<u32> for POINTER_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for POINTER_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for POINTER_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for POINTER_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for POINTER_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for POINTER_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for POINTER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct POINTER_INFO {
    pub pointerType: super::WindowsAndMessaging::POINTER_INPUT_TYPE,
    pub pointerId: u32,
    pub frameId: u32,
    pub pointerFlags: POINTER_FLAGS,
    pub sourceDevice: super::super::Foundation::HANDLE,
    pub hwndTarget: super::super::Foundation::HWND,
    pub ptPixelLocation: super::super::Foundation::POINT,
    pub ptHimetricLocation: super::super::Foundation::POINT,
    pub ptPixelLocationRaw: super::super::Foundation::POINT,
    pub ptHimetricLocationRaw: super::super::Foundation::POINT,
    pub dwTime: u32,
    pub historyCount: u32,
    pub InputData: i32,
    pub dwKeyStates: u32,
    pub PerformanceCount: u64,
    pub ButtonChangeType: POINTER_BUTTON_CHANGE_TYPE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl POINTER_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for POINTER_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::fmt::Debug for POINTER_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("POINTER_INFO")
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
impl ::std::cmp::PartialEq for POINTER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pointerType == other.pointerType
            && self.pointerId == other.pointerId
            && self.frameId == other.frameId
            && self.pointerFlags == other.pointerFlags
            && self.sourceDevice == other.sourceDevice
            && self.hwndTarget == other.hwndTarget
            && self.ptPixelLocation == other.ptPixelLocation
            && self.ptHimetricLocation == other.ptHimetricLocation
            && self.ptPixelLocationRaw == other.ptPixelLocationRaw
            && self.ptHimetricLocationRaw == other.ptHimetricLocationRaw
            && self.dwTime == other.dwTime
            && self.historyCount == other.historyCount
            && self.InputData == other.InputData
            && self.dwKeyStates == other.dwKeyStates
            && self.PerformanceCount == other.PerformanceCount
            && self.ButtonChangeType == other.ButtonChangeType
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for POINTER_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for POINTER_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl POINTER_PEN_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for POINTER_PEN_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::fmt::Debug for POINTER_PEN_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("POINTER_PEN_INFO")
            .field("pointerInfo", &self.pointerInfo)
            .field("penFlags", &self.penFlags)
            .field("penMask", &self.penMask)
            .field("pressure", &self.pressure)
            .field("rotation", &self.rotation)
            .field("tiltX", &self.tiltX)
            .field("tiltY", &self.tiltY)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for POINTER_PEN_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pointerInfo == other.pointerInfo
            && self.penFlags == other.penFlags
            && self.penMask == other.penMask
            && self.pressure == other.pressure
            && self.rotation == other.rotation
            && self.tiltX == other.tiltX
            && self.tiltY == other.tiltY
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for POINTER_PEN_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for POINTER_PEN_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct POINTER_TOUCH_INFO {
    pub pointerInfo: POINTER_INFO,
    pub touchFlags: u32,
    pub touchMask: u32,
    pub rcContact: super::super::Foundation::RECT,
    pub rcContactRaw: super::super::Foundation::RECT,
    pub orientation: u32,
    pub pressure: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl POINTER_TOUCH_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for POINTER_TOUCH_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::fmt::Debug for POINTER_TOUCH_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("POINTER_TOUCH_INFO")
            .field("pointerInfo", &self.pointerInfo)
            .field("touchFlags", &self.touchFlags)
            .field("touchMask", &self.touchMask)
            .field("rcContact", &self.rcContact)
            .field("rcContactRaw", &self.rcContactRaw)
            .field("orientation", &self.orientation)
            .field("pressure", &self.pressure)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for POINTER_TOUCH_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pointerInfo == other.pointerInfo
            && self.touchFlags == other.touchFlags
            && self.touchMask == other.touchMask
            && self.rcContact == other.rcContact
            && self.rcContactRaw == other.rcContactRaw
            && self.orientation == other.orientation
            && self.pressure == other.pressure
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for POINTER_TOUCH_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for POINTER_TOUCH_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SkipPointerFrameMessages(pointerid: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SkipPointerFrameMessages(pointerid: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SkipPointerFrameMessages(::std::mem::transmute(pointerid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
