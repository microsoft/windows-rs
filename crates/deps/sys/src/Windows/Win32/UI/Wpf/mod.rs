#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub const CLSID_MILBitmapEffectBevel: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4248182206, data2: 27803, data3: 19936, data4: [130, 144, 246, 64, 12, 39, 55, 237] };
pub const CLSID_MILBitmapEffectBlur: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2837766023, data2: 8797, data3: 17267, data4: [143, 91, 185, 14, 200, 90, 227, 222] };
pub const CLSID_MILBitmapEffectDropShadow: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1167736766, data2: 55468, data3: 18066, data4: [135, 75, 122, 38, 87, 21, 170, 22] };
pub const CLSID_MILBitmapEffectEmboss: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3442055238, data2: 33359, data3: 18412, data4: [160, 7, 18, 170, 118, 127, 40, 22] };
pub const CLSID_MILBitmapEffectGroup: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2895911578, data2: 32280, data3: 20324, data4: [172, 126, 71, 207, 127, 5, 30, 149] };
pub const CLSID_MILBitmapEffectOuterGlow: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3793099741, data2: 32438, data3: 18213, data4: [156, 11, 138, 42, 27, 79, 6, 103] };
pub struct IMILBitmapEffect(pub *mut ::core::ffi::c_void);
pub struct IMILBitmapEffectConnections(pub *mut ::core::ffi::c_void);
pub struct IMILBitmapEffectConnectionsInfo(pub *mut ::core::ffi::c_void);
pub struct IMILBitmapEffectConnector(pub *mut ::core::ffi::c_void);
pub struct IMILBitmapEffectConnectorInfo(pub *mut ::core::ffi::c_void);
pub struct IMILBitmapEffectEvents(pub *mut ::core::ffi::c_void);
pub struct IMILBitmapEffectFactory(pub *mut ::core::ffi::c_void);
pub struct IMILBitmapEffectGroup(pub *mut ::core::ffi::c_void);
pub struct IMILBitmapEffectGroupImpl(pub *mut ::core::ffi::c_void);
pub struct IMILBitmapEffectImpl(pub *mut ::core::ffi::c_void);
pub struct IMILBitmapEffectInputConnector(pub *mut ::core::ffi::c_void);
pub struct IMILBitmapEffectInteriorInputConnector(pub *mut ::core::ffi::c_void);
pub struct IMILBitmapEffectInteriorOutputConnector(pub *mut ::core::ffi::c_void);
pub struct IMILBitmapEffectOutputConnector(pub *mut ::core::ffi::c_void);
pub struct IMILBitmapEffectOutputConnectorImpl(pub *mut ::core::ffi::c_void);
pub struct IMILBitmapEffectPrimitive(pub *mut ::core::ffi::c_void);
pub struct IMILBitmapEffectPrimitiveImpl(pub *mut ::core::ffi::c_void);
pub struct IMILBitmapEffectRenderContext(pub *mut ::core::ffi::c_void);
pub struct IMILBitmapEffectRenderContextImpl(pub *mut ::core::ffi::c_void);
pub struct IMILBitmapEffects(pub *mut ::core::ffi::c_void);
#[doc = "*Required features: `Win32_UI_Wpf`*"]
pub const MILBITMAPEFFECT_SDK_VERSION: u32 = 16777216u32;
pub struct MILMatrixF(i32);
pub struct MilPoint2D(i32);
pub struct MilRectD(i32);
