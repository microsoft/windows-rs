#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub const CLSID_MILBitmapEffectBevel: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4248182206, data2: 27803, data3: 19936, data4: [130, 144, 246, 64, 12, 39, 55, 237] };
pub const CLSID_MILBitmapEffectBlur: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2837766023, data2: 8797, data3: 17267, data4: [143, 91, 185, 14, 200, 90, 227, 222] };
pub const CLSID_MILBitmapEffectDropShadow: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1167736766, data2: 55468, data3: 18066, data4: [135, 75, 122, 38, 87, 21, 170, 22] };
pub const CLSID_MILBitmapEffectEmboss: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3442055238, data2: 33359, data3: 18412, data4: [160, 7, 18, 170, 118, 127, 40, 22] };
pub const CLSID_MILBitmapEffectGroup: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2895911578, data2: 32280, data3: 20324, data4: [172, 126, 71, 207, 127, 5, 30, 149] };
pub const CLSID_MILBitmapEffectOuterGlow: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3793099741, data2: 32438, data3: 18213, data4: [156, 11, 138, 42, 27, 79, 6, 103] };
pub struct IMILBitmapEffect(i32);
pub struct IMILBitmapEffectConnections(i32);
pub struct IMILBitmapEffectConnectionsInfo(i32);
pub struct IMILBitmapEffectConnector(i32);
pub struct IMILBitmapEffectConnectorInfo(i32);
pub struct IMILBitmapEffectEvents(i32);
pub struct IMILBitmapEffectFactory(i32);
pub struct IMILBitmapEffectGroup(i32);
pub struct IMILBitmapEffectGroupImpl(i32);
pub struct IMILBitmapEffectImpl(i32);
pub struct IMILBitmapEffectInputConnector(i32);
pub struct IMILBitmapEffectInteriorInputConnector(i32);
pub struct IMILBitmapEffectInteriorOutputConnector(i32);
pub struct IMILBitmapEffectOutputConnector(i32);
pub struct IMILBitmapEffectOutputConnectorImpl(i32);
pub struct IMILBitmapEffectPrimitive(i32);
pub struct IMILBitmapEffectPrimitiveImpl(i32);
pub struct IMILBitmapEffectRenderContext(i32);
pub struct IMILBitmapEffectRenderContextImpl(i32);
pub struct IMILBitmapEffects(i32);
#[doc = "*Required features: `Win32_UI_Wpf`*"]
pub const MILBITMAPEFFECT_SDK_VERSION: u32 = 16777216u32;
pub struct MILMatrixF(i32);
pub struct MilPoint2D(i32);
pub struct MilRectD(i32);
