#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
pub const CLSID_MILBitmapEffectBevel: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4248182206, data2: 27803, data3: 19936, data4: [130, 144, 246, 64, 12, 39, 55, 237] };
pub const CLSID_MILBitmapEffectBlur: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2837766023, data2: 8797, data3: 17267, data4: [143, 91, 185, 14, 200, 90, 227, 222] };
pub const CLSID_MILBitmapEffectDropShadow: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1167736766, data2: 55468, data3: 18066, data4: [135, 75, 122, 38, 87, 21, 170, 22] };
pub const CLSID_MILBitmapEffectEmboss: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3442055238, data2: 33359, data3: 18412, data4: [160, 7, 18, 170, 118, 127, 40, 22] };
pub const CLSID_MILBitmapEffectGroup: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2895911578, data2: 32280, data3: 20324, data4: [172, 126, 71, 207, 127, 5, 30, 149] };
pub const CLSID_MILBitmapEffectOuterGlow: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3793099741, data2: 32438, data3: 18213, data4: [156, 11, 138, 42, 27, 79, 6, 103] };
#[repr(transparent)]
pub struct IMILBitmapEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMILBitmapEffectConnections(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMILBitmapEffectConnectionsInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMILBitmapEffectConnector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMILBitmapEffectConnectorInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMILBitmapEffectEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMILBitmapEffectFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMILBitmapEffectGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMILBitmapEffectGroupImpl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMILBitmapEffectImpl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMILBitmapEffectInputConnector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMILBitmapEffectInteriorInputConnector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMILBitmapEffectInteriorOutputConnector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMILBitmapEffectOutputConnector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMILBitmapEffectOutputConnectorImpl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMILBitmapEffectPrimitive(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMILBitmapEffectPrimitiveImpl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMILBitmapEffectRenderContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMILBitmapEffectRenderContextImpl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMILBitmapEffects(pub *mut ::core::ffi::c_void);
pub const MILBITMAPEFFECT_SDK_VERSION: u32 = 16777216u32;
#[repr(C)]
pub struct MILMatrixF {
    pub _11: f64,
    pub _12: f64,
    pub _13: f64,
    pub _14: f64,
    pub _21: f64,
    pub _22: f64,
    pub _23: f64,
    pub _24: f64,
    pub _31: f64,
    pub _32: f64,
    pub _33: f64,
    pub _34: f64,
    pub _41: f64,
    pub _42: f64,
    pub _43: f64,
    pub _44: f64,
}
impl ::core::marker::Copy for MILMatrixF {}
impl ::core::clone::Clone for MILMatrixF {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MilPoint2D {
    pub X: f64,
    pub Y: f64,
}
impl ::core::marker::Copy for MilPoint2D {}
impl ::core::clone::Clone for MilPoint2D {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MilRectD {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}
impl ::core::marker::Copy for MilRectD {}
impl ::core::clone::Clone for MilRectD {
    fn clone(&self) -> Self {
        *self
    }
}
