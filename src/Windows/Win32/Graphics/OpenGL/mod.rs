#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ChoosePixelFormat<'a, Param0: ::windows::core::IntoParam<'a, super::Gdi::HDC>>(hdc: Param0, ppfd: *const PIXELFORMATDESCRIPTOR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ChoosePixelFormat(hdc: super::Gdi::HDC, ppfd: *const PIXELFORMATDESCRIPTOR) -> i32;
        }
        ::core::mem::transmute(ChoosePixelFormat(hdc.into_param().abi(), ::core::mem::transmute(ppfd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DescribePixelFormat<'a, Param0: ::windows::core::IntoParam<'a, super::Gdi::HDC>>(hdc: Param0, ipixelformat: i32, nbytes: u32, ppfd: *mut PIXELFORMATDESCRIPTOR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DescribePixelFormat(hdc: super::Gdi::HDC, ipixelformat: i32, nbytes: u32, ppfd: *mut PIXELFORMATDESCRIPTOR) -> i32;
        }
        ::core::mem::transmute(DescribePixelFormat(hdc.into_param().abi(), ::core::mem::transmute(ipixelformat), ::core::mem::transmute(nbytes), ::core::mem::transmute(ppfd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct EMRPIXELFORMAT {
    pub emr: super::Gdi::EMR,
    pub pfd: PIXELFORMATDESCRIPTOR,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl EMRPIXELFORMAT {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for EMRPIXELFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for EMRPIXELFORMAT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("EMRPIXELFORMAT").field("emr", &self.emr).field("pfd", &self.pfd).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for EMRPIXELFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.pfd == other.pfd
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for EMRPIXELFORMAT {}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows::core::Abi for EMRPIXELFORMAT {
    type Abi = Self;
}
pub const GLU_AUTO_LOAD_MATRIX: u32 = 100200u32;
pub const GLU_BEGIN: u32 = 100100u32;
pub const GLU_CCW: u32 = 100121u32;
pub const GLU_CULLING: u32 = 100201u32;
pub const GLU_CW: u32 = 100120u32;
pub const GLU_DISPLAY_MODE: u32 = 100204u32;
pub const GLU_DOMAIN_DISTANCE: u32 = 100217u32;
pub const GLU_EDGE_FLAG: u32 = 100104u32;
pub const GLU_END: u32 = 100102u32;
pub const GLU_ERROR: u32 = 100103u32;
pub const GLU_EXTENSIONS: u32 = 100801u32;
pub const GLU_EXTERIOR: u32 = 100123u32;
pub const GLU_FALSE: u32 = 0u32;
pub const GLU_FILL: u32 = 100012u32;
pub const GLU_FLAT: u32 = 100001u32;
pub const GLU_INCOMPATIBLE_GL_VERSION: u32 = 100903u32;
pub const GLU_INSIDE: u32 = 100021u32;
pub const GLU_INTERIOR: u32 = 100122u32;
pub const GLU_INVALID_ENUM: u32 = 100900u32;
pub const GLU_INVALID_VALUE: u32 = 100901u32;
pub const GLU_LINE: u32 = 100011u32;
pub const GLU_MAP1_TRIM_2: u32 = 100210u32;
pub const GLU_MAP1_TRIM_3: u32 = 100211u32;
pub const GLU_NONE: u32 = 100002u32;
pub const GLU_NURBS_ERROR1: u32 = 100251u32;
pub const GLU_NURBS_ERROR10: u32 = 100260u32;
pub const GLU_NURBS_ERROR11: u32 = 100261u32;
pub const GLU_NURBS_ERROR12: u32 = 100262u32;
pub const GLU_NURBS_ERROR13: u32 = 100263u32;
pub const GLU_NURBS_ERROR14: u32 = 100264u32;
pub const GLU_NURBS_ERROR15: u32 = 100265u32;
pub const GLU_NURBS_ERROR16: u32 = 100266u32;
pub const GLU_NURBS_ERROR17: u32 = 100267u32;
pub const GLU_NURBS_ERROR18: u32 = 100268u32;
pub const GLU_NURBS_ERROR19: u32 = 100269u32;
pub const GLU_NURBS_ERROR2: u32 = 100252u32;
pub const GLU_NURBS_ERROR20: u32 = 100270u32;
pub const GLU_NURBS_ERROR21: u32 = 100271u32;
pub const GLU_NURBS_ERROR22: u32 = 100272u32;
pub const GLU_NURBS_ERROR23: u32 = 100273u32;
pub const GLU_NURBS_ERROR24: u32 = 100274u32;
pub const GLU_NURBS_ERROR25: u32 = 100275u32;
pub const GLU_NURBS_ERROR26: u32 = 100276u32;
pub const GLU_NURBS_ERROR27: u32 = 100277u32;
pub const GLU_NURBS_ERROR28: u32 = 100278u32;
pub const GLU_NURBS_ERROR29: u32 = 100279u32;
pub const GLU_NURBS_ERROR3: u32 = 100253u32;
pub const GLU_NURBS_ERROR30: u32 = 100280u32;
pub const GLU_NURBS_ERROR31: u32 = 100281u32;
pub const GLU_NURBS_ERROR32: u32 = 100282u32;
pub const GLU_NURBS_ERROR33: u32 = 100283u32;
pub const GLU_NURBS_ERROR34: u32 = 100284u32;
pub const GLU_NURBS_ERROR35: u32 = 100285u32;
pub const GLU_NURBS_ERROR36: u32 = 100286u32;
pub const GLU_NURBS_ERROR37: u32 = 100287u32;
pub const GLU_NURBS_ERROR4: u32 = 100254u32;
pub const GLU_NURBS_ERROR5: u32 = 100255u32;
pub const GLU_NURBS_ERROR6: u32 = 100256u32;
pub const GLU_NURBS_ERROR7: u32 = 100257u32;
pub const GLU_NURBS_ERROR8: u32 = 100258u32;
pub const GLU_NURBS_ERROR9: u32 = 100259u32;
pub const GLU_OUTLINE_PATCH: u32 = 100241u32;
pub const GLU_OUTLINE_POLYGON: u32 = 100240u32;
pub const GLU_OUTSIDE: u32 = 100020u32;
pub const GLU_OUT_OF_MEMORY: u32 = 100902u32;
pub const GLU_PARAMETRIC_ERROR: u32 = 100216u32;
pub const GLU_PARAMETRIC_TOLERANCE: u32 = 100202u32;
pub const GLU_PATH_LENGTH: u32 = 100215u32;
pub const GLU_POINT: u32 = 100010u32;
pub const GLU_SAMPLING_METHOD: u32 = 100205u32;
pub const GLU_SAMPLING_TOLERANCE: u32 = 100203u32;
pub const GLU_SILHOUETTE: u32 = 100013u32;
pub const GLU_SMOOTH: u32 = 100000u32;
pub const GLU_TESS_BEGIN: u32 = 100100u32;
pub const GLU_TESS_BEGIN_DATA: u32 = 100106u32;
pub const GLU_TESS_BOUNDARY_ONLY: u32 = 100141u32;
pub const GLU_TESS_COMBINE: u32 = 100105u32;
pub const GLU_TESS_COMBINE_DATA: u32 = 100111u32;
pub const GLU_TESS_COORD_TOO_LARGE: u32 = 100155u32;
pub const GLU_TESS_EDGE_FLAG: u32 = 100104u32;
pub const GLU_TESS_EDGE_FLAG_DATA: u32 = 100110u32;
pub const GLU_TESS_END: u32 = 100102u32;
pub const GLU_TESS_END_DATA: u32 = 100108u32;
pub const GLU_TESS_ERROR: u32 = 100103u32;
pub const GLU_TESS_ERROR1: u32 = 100151u32;
pub const GLU_TESS_ERROR2: u32 = 100152u32;
pub const GLU_TESS_ERROR3: u32 = 100153u32;
pub const GLU_TESS_ERROR4: u32 = 100154u32;
pub const GLU_TESS_ERROR5: u32 = 100155u32;
pub const GLU_TESS_ERROR6: u32 = 100156u32;
pub const GLU_TESS_ERROR7: u32 = 100157u32;
pub const GLU_TESS_ERROR8: u32 = 100158u32;
pub const GLU_TESS_ERROR_DATA: u32 = 100109u32;
pub const GLU_TESS_MISSING_BEGIN_CONTOUR: u32 = 100152u32;
pub const GLU_TESS_MISSING_BEGIN_POLYGON: u32 = 100151u32;
pub const GLU_TESS_MISSING_END_CONTOUR: u32 = 100154u32;
pub const GLU_TESS_MISSING_END_POLYGON: u32 = 100153u32;
pub const GLU_TESS_NEED_COMBINE_CALLBACK: u32 = 100156u32;
pub const GLU_TESS_TOLERANCE: u32 = 100142u32;
pub const GLU_TESS_VERTEX: u32 = 100101u32;
pub const GLU_TESS_VERTEX_DATA: u32 = 100107u32;
pub const GLU_TESS_WINDING_ABS_GEQ_TWO: u32 = 100134u32;
pub const GLU_TESS_WINDING_NEGATIVE: u32 = 100133u32;
pub const GLU_TESS_WINDING_NONZERO: u32 = 100131u32;
pub const GLU_TESS_WINDING_ODD: u32 = 100130u32;
pub const GLU_TESS_WINDING_POSITIVE: u32 = 100132u32;
pub const GLU_TESS_WINDING_RULE: u32 = 100140u32;
pub const GLU_TRUE: u32 = 1u32;
pub const GLU_UNKNOWN: u32 = 100124u32;
pub const GLU_U_STEP: u32 = 100206u32;
pub const GLU_VERSION: u32 = 100800u32;
pub const GLU_VERSION_1_1: u32 = 1u32;
pub const GLU_VERSION_1_2: u32 = 1u32;
pub const GLU_VERTEX: u32 = 100101u32;
pub const GLU_V_STEP: u32 = 100207u32;
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct GLUnurbs(pub u8);
pub type GLUnurbsErrorProc = ::core::option::Option<unsafe extern "system" fn(param0: u32)>;
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct GLUquadric(pub u8);
pub type GLUquadricErrorProc = ::core::option::Option<unsafe extern "system" fn(param0: u32)>;
pub type GLUtessBeginDataProc = ::core::option::Option<unsafe extern "system" fn(param0: u32, param1: *mut ::core::ffi::c_void)>;
pub type GLUtessBeginProc = ::core::option::Option<unsafe extern "system" fn(param0: u32)>;
pub type GLUtessCombineDataProc = ::core::option::Option<unsafe extern "system" fn(param0: *mut f64, param1: *mut *mut ::core::ffi::c_void, param2: *mut f32, param3: *mut *mut ::core::ffi::c_void, param4: *mut ::core::ffi::c_void)>;
pub type GLUtessCombineProc = ::core::option::Option<unsafe extern "system" fn(param0: *mut f64, param1: *mut *mut ::core::ffi::c_void, param2: *mut f32, param3: *mut *mut ::core::ffi::c_void)>;
pub type GLUtessEdgeFlagDataProc = ::core::option::Option<unsafe extern "system" fn(param0: u8, param1: *mut ::core::ffi::c_void)>;
pub type GLUtessEdgeFlagProc = ::core::option::Option<unsafe extern "system" fn(param0: u8)>;
pub type GLUtessEndDataProc = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void)>;
pub type GLUtessEndProc = ::core::option::Option<unsafe extern "system" fn()>;
pub type GLUtessErrorDataProc = ::core::option::Option<unsafe extern "system" fn(param0: u32, param1: *mut ::core::ffi::c_void)>;
pub type GLUtessErrorProc = ::core::option::Option<unsafe extern "system" fn(param0: u32)>;
pub type GLUtessVertexDataProc = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: *mut ::core::ffi::c_void)>;
pub type GLUtessVertexProc = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void)>;
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct GLUtesselator(pub u8);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct GLYPHMETRICSFLOAT {
    pub gmfBlackBoxX: f32,
    pub gmfBlackBoxY: f32,
    pub gmfptGlyphOrigin: POINTFLOAT,
    pub gmfCellIncX: f32,
    pub gmfCellIncY: f32,
}
impl GLYPHMETRICSFLOAT {}
impl ::core::default::Default for GLYPHMETRICSFLOAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for GLYPHMETRICSFLOAT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GLYPHMETRICSFLOAT").field("gmfBlackBoxX", &self.gmfBlackBoxX).field("gmfBlackBoxY", &self.gmfBlackBoxY).field("gmfptGlyphOrigin", &self.gmfptGlyphOrigin).field("gmfCellIncX", &self.gmfCellIncX).field("gmfCellIncY", &self.gmfCellIncY).finish()
    }
}
impl ::core::cmp::PartialEq for GLYPHMETRICSFLOAT {
    fn eq(&self, other: &Self) -> bool {
        self.gmfBlackBoxX == other.gmfBlackBoxX && self.gmfBlackBoxY == other.gmfBlackBoxY && self.gmfptGlyphOrigin == other.gmfptGlyphOrigin && self.gmfCellIncX == other.gmfCellIncX && self.gmfCellIncY == other.gmfCellIncY
    }
}
impl ::core::cmp::Eq for GLYPHMETRICSFLOAT {}
unsafe impl ::windows::core::Abi for GLYPHMETRICSFLOAT {
    type Abi = Self;
}
pub const GL_2D: u32 = 1536u32;
pub const GL_2_BYTES: u32 = 5127u32;
pub const GL_3D: u32 = 1537u32;
pub const GL_3D_COLOR: u32 = 1538u32;
pub const GL_3D_COLOR_TEXTURE: u32 = 1539u32;
pub const GL_3_BYTES: u32 = 5128u32;
pub const GL_4D_COLOR_TEXTURE: u32 = 1540u32;
pub const GL_4_BYTES: u32 = 5129u32;
pub const GL_ACCUM: u32 = 256u32;
pub const GL_ACCUM_ALPHA_BITS: u32 = 3419u32;
pub const GL_ACCUM_BLUE_BITS: u32 = 3418u32;
pub const GL_ACCUM_BUFFER_BIT: u32 = 512u32;
pub const GL_ACCUM_CLEAR_VALUE: u32 = 2944u32;
pub const GL_ACCUM_GREEN_BITS: u32 = 3417u32;
pub const GL_ACCUM_RED_BITS: u32 = 3416u32;
pub const GL_ADD: u32 = 260u32;
pub const GL_ALL_ATTRIB_BITS: u32 = 1048575u32;
pub const GL_ALPHA: u32 = 6406u32;
pub const GL_ALPHA12: u32 = 32829u32;
pub const GL_ALPHA16: u32 = 32830u32;
pub const GL_ALPHA4: u32 = 32827u32;
pub const GL_ALPHA8: u32 = 32828u32;
pub const GL_ALPHA_BIAS: u32 = 3357u32;
pub const GL_ALPHA_BITS: u32 = 3413u32;
pub const GL_ALPHA_SCALE: u32 = 3356u32;
pub const GL_ALPHA_TEST: u32 = 3008u32;
pub const GL_ALPHA_TEST_FUNC: u32 = 3009u32;
pub const GL_ALPHA_TEST_REF: u32 = 3010u32;
pub const GL_ALWAYS: u32 = 519u32;
pub const GL_AMBIENT: u32 = 4608u32;
pub const GL_AMBIENT_AND_DIFFUSE: u32 = 5634u32;
pub const GL_AND: u32 = 5377u32;
pub const GL_AND_INVERTED: u32 = 5380u32;
pub const GL_AND_REVERSE: u32 = 5378u32;
pub const GL_ATTRIB_STACK_DEPTH: u32 = 2992u32;
pub const GL_AUTO_NORMAL: u32 = 3456u32;
pub const GL_AUX0: u32 = 1033u32;
pub const GL_AUX1: u32 = 1034u32;
pub const GL_AUX2: u32 = 1035u32;
pub const GL_AUX3: u32 = 1036u32;
pub const GL_AUX_BUFFERS: u32 = 3072u32;
pub const GL_BACK: u32 = 1029u32;
pub const GL_BACK_LEFT: u32 = 1026u32;
pub const GL_BACK_RIGHT: u32 = 1027u32;
pub const GL_BGRA_EXT: u32 = 32993u32;
pub const GL_BGR_EXT: u32 = 32992u32;
pub const GL_BITMAP: u32 = 6656u32;
pub const GL_BITMAP_TOKEN: u32 = 1796u32;
pub const GL_BLEND: u32 = 3042u32;
pub const GL_BLEND_DST: u32 = 3040u32;
pub const GL_BLEND_SRC: u32 = 3041u32;
pub const GL_BLUE: u32 = 6405u32;
pub const GL_BLUE_BIAS: u32 = 3355u32;
pub const GL_BLUE_BITS: u32 = 3412u32;
pub const GL_BLUE_SCALE: u32 = 3354u32;
pub const GL_BYTE: u32 = 5120u32;
pub const GL_C3F_V3F: u32 = 10788u32;
pub const GL_C4F_N3F_V3F: u32 = 10790u32;
pub const GL_C4UB_V2F: u32 = 10786u32;
pub const GL_C4UB_V3F: u32 = 10787u32;
pub const GL_CCW: u32 = 2305u32;
pub const GL_CLAMP: u32 = 10496u32;
pub const GL_CLEAR: u32 = 5376u32;
pub const GL_CLIENT_ALL_ATTRIB_BITS: u32 = 4294967295u32;
pub const GL_CLIENT_ATTRIB_STACK_DEPTH: u32 = 2993u32;
pub const GL_CLIENT_PIXEL_STORE_BIT: u32 = 1u32;
pub const GL_CLIENT_VERTEX_ARRAY_BIT: u32 = 2u32;
pub const GL_CLIP_PLANE0: u32 = 12288u32;
pub const GL_CLIP_PLANE1: u32 = 12289u32;
pub const GL_CLIP_PLANE2: u32 = 12290u32;
pub const GL_CLIP_PLANE3: u32 = 12291u32;
pub const GL_CLIP_PLANE4: u32 = 12292u32;
pub const GL_CLIP_PLANE5: u32 = 12293u32;
pub const GL_COEFF: u32 = 2560u32;
pub const GL_COLOR: u32 = 6144u32;
pub const GL_COLOR_ARRAY: u32 = 32886u32;
pub const GL_COLOR_ARRAY_COUNT_EXT: u32 = 32900u32;
pub const GL_COLOR_ARRAY_EXT: u32 = 32886u32;
pub const GL_COLOR_ARRAY_POINTER: u32 = 32912u32;
pub const GL_COLOR_ARRAY_POINTER_EXT: u32 = 32912u32;
pub const GL_COLOR_ARRAY_SIZE: u32 = 32897u32;
pub const GL_COLOR_ARRAY_SIZE_EXT: u32 = 32897u32;
pub const GL_COLOR_ARRAY_STRIDE: u32 = 32899u32;
pub const GL_COLOR_ARRAY_STRIDE_EXT: u32 = 32899u32;
pub const GL_COLOR_ARRAY_TYPE: u32 = 32898u32;
pub const GL_COLOR_ARRAY_TYPE_EXT: u32 = 32898u32;
pub const GL_COLOR_BUFFER_BIT: u32 = 16384u32;
pub const GL_COLOR_CLEAR_VALUE: u32 = 3106u32;
pub const GL_COLOR_INDEX: u32 = 6400u32;
pub const GL_COLOR_INDEX12_EXT: u32 = 32998u32;
pub const GL_COLOR_INDEX16_EXT: u32 = 32999u32;
pub const GL_COLOR_INDEX1_EXT: u32 = 32994u32;
pub const GL_COLOR_INDEX2_EXT: u32 = 32995u32;
pub const GL_COLOR_INDEX4_EXT: u32 = 32996u32;
pub const GL_COLOR_INDEX8_EXT: u32 = 32997u32;
pub const GL_COLOR_INDEXES: u32 = 5635u32;
pub const GL_COLOR_LOGIC_OP: u32 = 3058u32;
pub const GL_COLOR_MATERIAL: u32 = 2903u32;
pub const GL_COLOR_MATERIAL_FACE: u32 = 2901u32;
pub const GL_COLOR_MATERIAL_PARAMETER: u32 = 2902u32;
pub const GL_COLOR_TABLE_ALPHA_SIZE_EXT: u32 = 32989u32;
pub const GL_COLOR_TABLE_BLUE_SIZE_EXT: u32 = 32988u32;
pub const GL_COLOR_TABLE_FORMAT_EXT: u32 = 32984u32;
pub const GL_COLOR_TABLE_GREEN_SIZE_EXT: u32 = 32987u32;
pub const GL_COLOR_TABLE_INTENSITY_SIZE_EXT: u32 = 32991u32;
pub const GL_COLOR_TABLE_LUMINANCE_SIZE_EXT: u32 = 32990u32;
pub const GL_COLOR_TABLE_RED_SIZE_EXT: u32 = 32986u32;
pub const GL_COLOR_TABLE_WIDTH_EXT: u32 = 32985u32;
pub const GL_COLOR_WRITEMASK: u32 = 3107u32;
pub const GL_COMPILE: u32 = 4864u32;
pub const GL_COMPILE_AND_EXECUTE: u32 = 4865u32;
pub const GL_CONSTANT_ATTENUATION: u32 = 4615u32;
pub const GL_COPY: u32 = 5379u32;
pub const GL_COPY_INVERTED: u32 = 5388u32;
pub const GL_COPY_PIXEL_TOKEN: u32 = 1798u32;
pub const GL_CULL_FACE: u32 = 2884u32;
pub const GL_CULL_FACE_MODE: u32 = 2885u32;
pub const GL_CURRENT_BIT: u32 = 1u32;
pub const GL_CURRENT_COLOR: u32 = 2816u32;
pub const GL_CURRENT_INDEX: u32 = 2817u32;
pub const GL_CURRENT_NORMAL: u32 = 2818u32;
pub const GL_CURRENT_RASTER_COLOR: u32 = 2820u32;
pub const GL_CURRENT_RASTER_DISTANCE: u32 = 2825u32;
pub const GL_CURRENT_RASTER_INDEX: u32 = 2821u32;
pub const GL_CURRENT_RASTER_POSITION: u32 = 2823u32;
pub const GL_CURRENT_RASTER_POSITION_VALID: u32 = 2824u32;
pub const GL_CURRENT_RASTER_TEXTURE_COORDS: u32 = 2822u32;
pub const GL_CURRENT_TEXTURE_COORDS: u32 = 2819u32;
pub const GL_CW: u32 = 2304u32;
pub const GL_DECAL: u32 = 8449u32;
pub const GL_DECR: u32 = 7683u32;
pub const GL_DEPTH: u32 = 6145u32;
pub const GL_DEPTH_BIAS: u32 = 3359u32;
pub const GL_DEPTH_BITS: u32 = 3414u32;
pub const GL_DEPTH_BUFFER_BIT: u32 = 256u32;
pub const GL_DEPTH_CLEAR_VALUE: u32 = 2931u32;
pub const GL_DEPTH_COMPONENT: u32 = 6402u32;
pub const GL_DEPTH_FUNC: u32 = 2932u32;
pub const GL_DEPTH_RANGE: u32 = 2928u32;
pub const GL_DEPTH_SCALE: u32 = 3358u32;
pub const GL_DEPTH_TEST: u32 = 2929u32;
pub const GL_DEPTH_WRITEMASK: u32 = 2930u32;
pub const GL_DIFFUSE: u32 = 4609u32;
pub const GL_DITHER: u32 = 3024u32;
pub const GL_DOMAIN: u32 = 2562u32;
pub const GL_DONT_CARE: u32 = 4352u32;
pub const GL_DOUBLE: u32 = 5130u32;
pub const GL_DOUBLEBUFFER: u32 = 3122u32;
pub const GL_DOUBLE_EXT: u32 = 5130u32;
pub const GL_DRAW_BUFFER: u32 = 3073u32;
pub const GL_DRAW_PIXEL_TOKEN: u32 = 1797u32;
pub const GL_DST_ALPHA: u32 = 772u32;
pub const GL_DST_COLOR: u32 = 774u32;
pub const GL_EDGE_FLAG: u32 = 2883u32;
pub const GL_EDGE_FLAG_ARRAY: u32 = 32889u32;
pub const GL_EDGE_FLAG_ARRAY_COUNT_EXT: u32 = 32909u32;
pub const GL_EDGE_FLAG_ARRAY_EXT: u32 = 32889u32;
pub const GL_EDGE_FLAG_ARRAY_POINTER: u32 = 32915u32;
pub const GL_EDGE_FLAG_ARRAY_POINTER_EXT: u32 = 32915u32;
pub const GL_EDGE_FLAG_ARRAY_STRIDE: u32 = 32908u32;
pub const GL_EDGE_FLAG_ARRAY_STRIDE_EXT: u32 = 32908u32;
pub const GL_EMISSION: u32 = 5632u32;
pub const GL_ENABLE_BIT: u32 = 8192u32;
pub const GL_EQUAL: u32 = 514u32;
pub const GL_EQUIV: u32 = 5385u32;
pub const GL_EVAL_BIT: u32 = 65536u32;
pub const GL_EXP: u32 = 2048u32;
pub const GL_EXP2: u32 = 2049u32;
pub const GL_EXTENSIONS: u32 = 7939u32;
pub const GL_EXT_bgra: u32 = 1u32;
pub const GL_EXT_paletted_texture: u32 = 1u32;
pub const GL_EXT_vertex_array: u32 = 1u32;
pub const GL_EYE_LINEAR: u32 = 9216u32;
pub const GL_EYE_PLANE: u32 = 9474u32;
pub const GL_FALSE: u32 = 0u32;
pub const GL_FASTEST: u32 = 4353u32;
pub const GL_FEEDBACK: u32 = 7169u32;
pub const GL_FEEDBACK_BUFFER_POINTER: u32 = 3568u32;
pub const GL_FEEDBACK_BUFFER_SIZE: u32 = 3569u32;
pub const GL_FEEDBACK_BUFFER_TYPE: u32 = 3570u32;
pub const GL_FILL: u32 = 6914u32;
pub const GL_FLAT: u32 = 7424u32;
pub const GL_FLOAT: u32 = 5126u32;
pub const GL_FOG: u32 = 2912u32;
pub const GL_FOG_BIT: u32 = 128u32;
pub const GL_FOG_COLOR: u32 = 2918u32;
pub const GL_FOG_DENSITY: u32 = 2914u32;
pub const GL_FOG_END: u32 = 2916u32;
pub const GL_FOG_HINT: u32 = 3156u32;
pub const GL_FOG_INDEX: u32 = 2913u32;
pub const GL_FOG_MODE: u32 = 2917u32;
pub const GL_FOG_SPECULAR_TEXTURE_WIN: u32 = 33004u32;
pub const GL_FOG_START: u32 = 2915u32;
pub const GL_FRONT: u32 = 1028u32;
pub const GL_FRONT_AND_BACK: u32 = 1032u32;
pub const GL_FRONT_FACE: u32 = 2886u32;
pub const GL_FRONT_LEFT: u32 = 1024u32;
pub const GL_FRONT_RIGHT: u32 = 1025u32;
pub const GL_GEQUAL: u32 = 518u32;
pub const GL_GREATER: u32 = 516u32;
pub const GL_GREEN: u32 = 6404u32;
pub const GL_GREEN_BIAS: u32 = 3353u32;
pub const GL_GREEN_BITS: u32 = 3411u32;
pub const GL_GREEN_SCALE: u32 = 3352u32;
pub const GL_HINT_BIT: u32 = 32768u32;
pub const GL_INCR: u32 = 7682u32;
pub const GL_INDEX_ARRAY: u32 = 32887u32;
pub const GL_INDEX_ARRAY_COUNT_EXT: u32 = 32903u32;
pub const GL_INDEX_ARRAY_EXT: u32 = 32887u32;
pub const GL_INDEX_ARRAY_POINTER: u32 = 32913u32;
pub const GL_INDEX_ARRAY_POINTER_EXT: u32 = 32913u32;
pub const GL_INDEX_ARRAY_STRIDE: u32 = 32902u32;
pub const GL_INDEX_ARRAY_STRIDE_EXT: u32 = 32902u32;
pub const GL_INDEX_ARRAY_TYPE: u32 = 32901u32;
pub const GL_INDEX_ARRAY_TYPE_EXT: u32 = 32901u32;
pub const GL_INDEX_BITS: u32 = 3409u32;
pub const GL_INDEX_CLEAR_VALUE: u32 = 3104u32;
pub const GL_INDEX_LOGIC_OP: u32 = 3057u32;
pub const GL_INDEX_MODE: u32 = 3120u32;
pub const GL_INDEX_OFFSET: u32 = 3347u32;
pub const GL_INDEX_SHIFT: u32 = 3346u32;
pub const GL_INDEX_WRITEMASK: u32 = 3105u32;
pub const GL_INT: u32 = 5124u32;
pub const GL_INTENSITY: u32 = 32841u32;
pub const GL_INTENSITY12: u32 = 32844u32;
pub const GL_INTENSITY16: u32 = 32845u32;
pub const GL_INTENSITY4: u32 = 32842u32;
pub const GL_INTENSITY8: u32 = 32843u32;
pub const GL_INVALID_ENUM: u32 = 1280u32;
pub const GL_INVALID_OPERATION: u32 = 1282u32;
pub const GL_INVALID_VALUE: u32 = 1281u32;
pub const GL_INVERT: u32 = 5386u32;
pub const GL_KEEP: u32 = 7680u32;
pub const GL_LEFT: u32 = 1030u32;
pub const GL_LEQUAL: u32 = 515u32;
pub const GL_LESS: u32 = 513u32;
pub const GL_LIGHT0: u32 = 16384u32;
pub const GL_LIGHT1: u32 = 16385u32;
pub const GL_LIGHT2: u32 = 16386u32;
pub const GL_LIGHT3: u32 = 16387u32;
pub const GL_LIGHT4: u32 = 16388u32;
pub const GL_LIGHT5: u32 = 16389u32;
pub const GL_LIGHT6: u32 = 16390u32;
pub const GL_LIGHT7: u32 = 16391u32;
pub const GL_LIGHTING: u32 = 2896u32;
pub const GL_LIGHTING_BIT: u32 = 64u32;
pub const GL_LIGHT_MODEL_AMBIENT: u32 = 2899u32;
pub const GL_LIGHT_MODEL_LOCAL_VIEWER: u32 = 2897u32;
pub const GL_LIGHT_MODEL_TWO_SIDE: u32 = 2898u32;
pub const GL_LINE: u32 = 6913u32;
pub const GL_LINEAR: u32 = 9729u32;
pub const GL_LINEAR_ATTENUATION: u32 = 4616u32;
pub const GL_LINEAR_MIPMAP_LINEAR: u32 = 9987u32;
pub const GL_LINEAR_MIPMAP_NEAREST: u32 = 9985u32;
pub const GL_LINES: u32 = 1u32;
pub const GL_LINE_BIT: u32 = 4u32;
pub const GL_LINE_LOOP: u32 = 2u32;
pub const GL_LINE_RESET_TOKEN: u32 = 1799u32;
pub const GL_LINE_SMOOTH: u32 = 2848u32;
pub const GL_LINE_SMOOTH_HINT: u32 = 3154u32;
pub const GL_LINE_STIPPLE: u32 = 2852u32;
pub const GL_LINE_STIPPLE_PATTERN: u32 = 2853u32;
pub const GL_LINE_STIPPLE_REPEAT: u32 = 2854u32;
pub const GL_LINE_STRIP: u32 = 3u32;
pub const GL_LINE_TOKEN: u32 = 1794u32;
pub const GL_LINE_WIDTH: u32 = 2849u32;
pub const GL_LINE_WIDTH_GRANULARITY: u32 = 2851u32;
pub const GL_LINE_WIDTH_RANGE: u32 = 2850u32;
pub const GL_LIST_BASE: u32 = 2866u32;
pub const GL_LIST_BIT: u32 = 131072u32;
pub const GL_LIST_INDEX: u32 = 2867u32;
pub const GL_LIST_MODE: u32 = 2864u32;
pub const GL_LOAD: u32 = 257u32;
pub const GL_LOGIC_OP: u32 = 3057u32;
pub const GL_LOGIC_OP_MODE: u32 = 3056u32;
pub const GL_LUMINANCE: u32 = 6409u32;
pub const GL_LUMINANCE12: u32 = 32833u32;
pub const GL_LUMINANCE12_ALPHA12: u32 = 32839u32;
pub const GL_LUMINANCE12_ALPHA4: u32 = 32838u32;
pub const GL_LUMINANCE16: u32 = 32834u32;
pub const GL_LUMINANCE16_ALPHA16: u32 = 32840u32;
pub const GL_LUMINANCE4: u32 = 32831u32;
pub const GL_LUMINANCE4_ALPHA4: u32 = 32835u32;
pub const GL_LUMINANCE6_ALPHA2: u32 = 32836u32;
pub const GL_LUMINANCE8: u32 = 32832u32;
pub const GL_LUMINANCE8_ALPHA8: u32 = 32837u32;
pub const GL_LUMINANCE_ALPHA: u32 = 6410u32;
pub const GL_MAP1_COLOR_4: u32 = 3472u32;
pub const GL_MAP1_GRID_DOMAIN: u32 = 3536u32;
pub const GL_MAP1_GRID_SEGMENTS: u32 = 3537u32;
pub const GL_MAP1_INDEX: u32 = 3473u32;
pub const GL_MAP1_NORMAL: u32 = 3474u32;
pub const GL_MAP1_TEXTURE_COORD_1: u32 = 3475u32;
pub const GL_MAP1_TEXTURE_COORD_2: u32 = 3476u32;
pub const GL_MAP1_TEXTURE_COORD_3: u32 = 3477u32;
pub const GL_MAP1_TEXTURE_COORD_4: u32 = 3478u32;
pub const GL_MAP1_VERTEX_3: u32 = 3479u32;
pub const GL_MAP1_VERTEX_4: u32 = 3480u32;
pub const GL_MAP2_COLOR_4: u32 = 3504u32;
pub const GL_MAP2_GRID_DOMAIN: u32 = 3538u32;
pub const GL_MAP2_GRID_SEGMENTS: u32 = 3539u32;
pub const GL_MAP2_INDEX: u32 = 3505u32;
pub const GL_MAP2_NORMAL: u32 = 3506u32;
pub const GL_MAP2_TEXTURE_COORD_1: u32 = 3507u32;
pub const GL_MAP2_TEXTURE_COORD_2: u32 = 3508u32;
pub const GL_MAP2_TEXTURE_COORD_3: u32 = 3509u32;
pub const GL_MAP2_TEXTURE_COORD_4: u32 = 3510u32;
pub const GL_MAP2_VERTEX_3: u32 = 3511u32;
pub const GL_MAP2_VERTEX_4: u32 = 3512u32;
pub const GL_MAP_COLOR: u32 = 3344u32;
pub const GL_MAP_STENCIL: u32 = 3345u32;
pub const GL_MATRIX_MODE: u32 = 2976u32;
pub const GL_MAX_ATTRIB_STACK_DEPTH: u32 = 3381u32;
pub const GL_MAX_CLIENT_ATTRIB_STACK_DEPTH: u32 = 3387u32;
pub const GL_MAX_CLIP_PLANES: u32 = 3378u32;
pub const GL_MAX_ELEMENTS_INDICES_WIN: u32 = 33001u32;
pub const GL_MAX_ELEMENTS_VERTICES_WIN: u32 = 33000u32;
pub const GL_MAX_EVAL_ORDER: u32 = 3376u32;
pub const GL_MAX_LIGHTS: u32 = 3377u32;
pub const GL_MAX_LIST_NESTING: u32 = 2865u32;
pub const GL_MAX_MODELVIEW_STACK_DEPTH: u32 = 3382u32;
pub const GL_MAX_NAME_STACK_DEPTH: u32 = 3383u32;
pub const GL_MAX_PIXEL_MAP_TABLE: u32 = 3380u32;
pub const GL_MAX_PROJECTION_STACK_DEPTH: u32 = 3384u32;
pub const GL_MAX_TEXTURE_SIZE: u32 = 3379u32;
pub const GL_MAX_TEXTURE_STACK_DEPTH: u32 = 3385u32;
pub const GL_MAX_VIEWPORT_DIMS: u32 = 3386u32;
pub const GL_MODELVIEW: u32 = 5888u32;
pub const GL_MODELVIEW_MATRIX: u32 = 2982u32;
pub const GL_MODELVIEW_STACK_DEPTH: u32 = 2979u32;
pub const GL_MODULATE: u32 = 8448u32;
pub const GL_MULT: u32 = 259u32;
pub const GL_N3F_V3F: u32 = 10789u32;
pub const GL_NAME_STACK_DEPTH: u32 = 3440u32;
pub const GL_NAND: u32 = 5390u32;
pub const GL_NEAREST: u32 = 9728u32;
pub const GL_NEAREST_MIPMAP_LINEAR: u32 = 9986u32;
pub const GL_NEAREST_MIPMAP_NEAREST: u32 = 9984u32;
pub const GL_NEVER: u32 = 512u32;
pub const GL_NICEST: u32 = 4354u32;
pub const GL_NONE: u32 = 0u32;
pub const GL_NOOP: u32 = 5381u32;
pub const GL_NOR: u32 = 5384u32;
pub const GL_NORMALIZE: u32 = 2977u32;
pub const GL_NORMAL_ARRAY: u32 = 32885u32;
pub const GL_NORMAL_ARRAY_COUNT_EXT: u32 = 32896u32;
pub const GL_NORMAL_ARRAY_EXT: u32 = 32885u32;
pub const GL_NORMAL_ARRAY_POINTER: u32 = 32911u32;
pub const GL_NORMAL_ARRAY_POINTER_EXT: u32 = 32911u32;
pub const GL_NORMAL_ARRAY_STRIDE: u32 = 32895u32;
pub const GL_NORMAL_ARRAY_STRIDE_EXT: u32 = 32895u32;
pub const GL_NORMAL_ARRAY_TYPE: u32 = 32894u32;
pub const GL_NORMAL_ARRAY_TYPE_EXT: u32 = 32894u32;
pub const GL_NOTEQUAL: u32 = 517u32;
pub const GL_NO_ERROR: u32 = 0u32;
pub const GL_OBJECT_LINEAR: u32 = 9217u32;
pub const GL_OBJECT_PLANE: u32 = 9473u32;
pub const GL_ONE: u32 = 1u32;
pub const GL_ONE_MINUS_DST_ALPHA: u32 = 773u32;
pub const GL_ONE_MINUS_DST_COLOR: u32 = 775u32;
pub const GL_ONE_MINUS_SRC_ALPHA: u32 = 771u32;
pub const GL_ONE_MINUS_SRC_COLOR: u32 = 769u32;
pub const GL_OR: u32 = 5383u32;
pub const GL_ORDER: u32 = 2561u32;
pub const GL_OR_INVERTED: u32 = 5389u32;
pub const GL_OR_REVERSE: u32 = 5387u32;
pub const GL_OUT_OF_MEMORY: u32 = 1285u32;
pub const GL_PACK_ALIGNMENT: u32 = 3333u32;
pub const GL_PACK_LSB_FIRST: u32 = 3329u32;
pub const GL_PACK_ROW_LENGTH: u32 = 3330u32;
pub const GL_PACK_SKIP_PIXELS: u32 = 3332u32;
pub const GL_PACK_SKIP_ROWS: u32 = 3331u32;
pub const GL_PACK_SWAP_BYTES: u32 = 3328u32;
pub const GL_PASS_THROUGH_TOKEN: u32 = 1792u32;
pub const GL_PERSPECTIVE_CORRECTION_HINT: u32 = 3152u32;
pub const GL_PHONG_HINT_WIN: u32 = 33003u32;
pub const GL_PHONG_WIN: u32 = 33002u32;
pub const GL_PIXEL_MAP_A_TO_A: u32 = 3193u32;
pub const GL_PIXEL_MAP_A_TO_A_SIZE: u32 = 3257u32;
pub const GL_PIXEL_MAP_B_TO_B: u32 = 3192u32;
pub const GL_PIXEL_MAP_B_TO_B_SIZE: u32 = 3256u32;
pub const GL_PIXEL_MAP_G_TO_G: u32 = 3191u32;
pub const GL_PIXEL_MAP_G_TO_G_SIZE: u32 = 3255u32;
pub const GL_PIXEL_MAP_I_TO_A: u32 = 3189u32;
pub const GL_PIXEL_MAP_I_TO_A_SIZE: u32 = 3253u32;
pub const GL_PIXEL_MAP_I_TO_B: u32 = 3188u32;
pub const GL_PIXEL_MAP_I_TO_B_SIZE: u32 = 3252u32;
pub const GL_PIXEL_MAP_I_TO_G: u32 = 3187u32;
pub const GL_PIXEL_MAP_I_TO_G_SIZE: u32 = 3251u32;
pub const GL_PIXEL_MAP_I_TO_I: u32 = 3184u32;
pub const GL_PIXEL_MAP_I_TO_I_SIZE: u32 = 3248u32;
pub const GL_PIXEL_MAP_I_TO_R: u32 = 3186u32;
pub const GL_PIXEL_MAP_I_TO_R_SIZE: u32 = 3250u32;
pub const GL_PIXEL_MAP_R_TO_R: u32 = 3190u32;
pub const GL_PIXEL_MAP_R_TO_R_SIZE: u32 = 3254u32;
pub const GL_PIXEL_MAP_S_TO_S: u32 = 3185u32;
pub const GL_PIXEL_MAP_S_TO_S_SIZE: u32 = 3249u32;
pub const GL_PIXEL_MODE_BIT: u32 = 32u32;
pub const GL_POINT: u32 = 6912u32;
pub const GL_POINTS: u32 = 0u32;
pub const GL_POINT_BIT: u32 = 2u32;
pub const GL_POINT_SIZE: u32 = 2833u32;
pub const GL_POINT_SIZE_GRANULARITY: u32 = 2835u32;
pub const GL_POINT_SIZE_RANGE: u32 = 2834u32;
pub const GL_POINT_SMOOTH: u32 = 2832u32;
pub const GL_POINT_SMOOTH_HINT: u32 = 3153u32;
pub const GL_POINT_TOKEN: u32 = 1793u32;
pub const GL_POLYGON: u32 = 9u32;
pub const GL_POLYGON_BIT: u32 = 8u32;
pub const GL_POLYGON_MODE: u32 = 2880u32;
pub const GL_POLYGON_OFFSET_FACTOR: u32 = 32824u32;
pub const GL_POLYGON_OFFSET_FILL: u32 = 32823u32;
pub const GL_POLYGON_OFFSET_LINE: u32 = 10754u32;
pub const GL_POLYGON_OFFSET_POINT: u32 = 10753u32;
pub const GL_POLYGON_OFFSET_UNITS: u32 = 10752u32;
pub const GL_POLYGON_SMOOTH: u32 = 2881u32;
pub const GL_POLYGON_SMOOTH_HINT: u32 = 3155u32;
pub const GL_POLYGON_STIPPLE: u32 = 2882u32;
pub const GL_POLYGON_STIPPLE_BIT: u32 = 16u32;
pub const GL_POLYGON_TOKEN: u32 = 1795u32;
pub const GL_POSITION: u32 = 4611u32;
pub const GL_PROJECTION: u32 = 5889u32;
pub const GL_PROJECTION_MATRIX: u32 = 2983u32;
pub const GL_PROJECTION_STACK_DEPTH: u32 = 2980u32;
pub const GL_PROXY_TEXTURE_1D: u32 = 32867u32;
pub const GL_PROXY_TEXTURE_2D: u32 = 32868u32;
pub const GL_Q: u32 = 8195u32;
pub const GL_QUADRATIC_ATTENUATION: u32 = 4617u32;
pub const GL_QUADS: u32 = 7u32;
pub const GL_QUAD_STRIP: u32 = 8u32;
pub const GL_R: u32 = 8194u32;
pub const GL_R3_G3_B2: u32 = 10768u32;
pub const GL_READ_BUFFER: u32 = 3074u32;
pub const GL_RED: u32 = 6403u32;
pub const GL_RED_BIAS: u32 = 3349u32;
pub const GL_RED_BITS: u32 = 3410u32;
pub const GL_RED_SCALE: u32 = 3348u32;
pub const GL_RENDER: u32 = 7168u32;
pub const GL_RENDERER: u32 = 7937u32;
pub const GL_RENDER_MODE: u32 = 3136u32;
pub const GL_REPEAT: u32 = 10497u32;
pub const GL_REPLACE: u32 = 7681u32;
pub const GL_RETURN: u32 = 258u32;
pub const GL_RGB: u32 = 6407u32;
pub const GL_RGB10: u32 = 32850u32;
pub const GL_RGB10_A2: u32 = 32857u32;
pub const GL_RGB12: u32 = 32851u32;
pub const GL_RGB16: u32 = 32852u32;
pub const GL_RGB4: u32 = 32847u32;
pub const GL_RGB5: u32 = 32848u32;
pub const GL_RGB5_A1: u32 = 32855u32;
pub const GL_RGB8: u32 = 32849u32;
pub const GL_RGBA: u32 = 6408u32;
pub const GL_RGBA12: u32 = 32858u32;
pub const GL_RGBA16: u32 = 32859u32;
pub const GL_RGBA2: u32 = 32853u32;
pub const GL_RGBA4: u32 = 32854u32;
pub const GL_RGBA8: u32 = 32856u32;
pub const GL_RGBA_MODE: u32 = 3121u32;
pub const GL_RIGHT: u32 = 1031u32;
pub const GL_S: u32 = 8192u32;
pub const GL_SCISSOR_BIT: u32 = 524288u32;
pub const GL_SCISSOR_BOX: u32 = 3088u32;
pub const GL_SCISSOR_TEST: u32 = 3089u32;
pub const GL_SELECT: u32 = 7170u32;
pub const GL_SELECTION_BUFFER_POINTER: u32 = 3571u32;
pub const GL_SELECTION_BUFFER_SIZE: u32 = 3572u32;
pub const GL_SET: u32 = 5391u32;
pub const GL_SHADE_MODEL: u32 = 2900u32;
pub const GL_SHININESS: u32 = 5633u32;
pub const GL_SHORT: u32 = 5122u32;
pub const GL_SMOOTH: u32 = 7425u32;
pub const GL_SPECULAR: u32 = 4610u32;
pub const GL_SPHERE_MAP: u32 = 9218u32;
pub const GL_SPOT_CUTOFF: u32 = 4614u32;
pub const GL_SPOT_DIRECTION: u32 = 4612u32;
pub const GL_SPOT_EXPONENT: u32 = 4613u32;
pub const GL_SRC_ALPHA: u32 = 770u32;
pub const GL_SRC_ALPHA_SATURATE: u32 = 776u32;
pub const GL_SRC_COLOR: u32 = 768u32;
pub const GL_STACK_OVERFLOW: u32 = 1283u32;
pub const GL_STACK_UNDERFLOW: u32 = 1284u32;
pub const GL_STENCIL: u32 = 6146u32;
pub const GL_STENCIL_BITS: u32 = 3415u32;
pub const GL_STENCIL_BUFFER_BIT: u32 = 1024u32;
pub const GL_STENCIL_CLEAR_VALUE: u32 = 2961u32;
pub const GL_STENCIL_FAIL: u32 = 2964u32;
pub const GL_STENCIL_FUNC: u32 = 2962u32;
pub const GL_STENCIL_INDEX: u32 = 6401u32;
pub const GL_STENCIL_PASS_DEPTH_FAIL: u32 = 2965u32;
pub const GL_STENCIL_PASS_DEPTH_PASS: u32 = 2966u32;
pub const GL_STENCIL_REF: u32 = 2967u32;
pub const GL_STENCIL_TEST: u32 = 2960u32;
pub const GL_STENCIL_VALUE_MASK: u32 = 2963u32;
pub const GL_STENCIL_WRITEMASK: u32 = 2968u32;
pub const GL_STEREO: u32 = 3123u32;
pub const GL_SUBPIXEL_BITS: u32 = 3408u32;
pub const GL_T: u32 = 8193u32;
pub const GL_T2F_C3F_V3F: u32 = 10794u32;
pub const GL_T2F_C4F_N3F_V3F: u32 = 10796u32;
pub const GL_T2F_C4UB_V3F: u32 = 10793u32;
pub const GL_T2F_N3F_V3F: u32 = 10795u32;
pub const GL_T2F_V3F: u32 = 10791u32;
pub const GL_T4F_C4F_N3F_V4F: u32 = 10797u32;
pub const GL_T4F_V4F: u32 = 10792u32;
pub const GL_TEXTURE: u32 = 5890u32;
pub const GL_TEXTURE_1D: u32 = 3552u32;
pub const GL_TEXTURE_2D: u32 = 3553u32;
pub const GL_TEXTURE_ALPHA_SIZE: u32 = 32863u32;
pub const GL_TEXTURE_BINDING_1D: u32 = 32872u32;
pub const GL_TEXTURE_BINDING_2D: u32 = 32873u32;
pub const GL_TEXTURE_BIT: u32 = 262144u32;
pub const GL_TEXTURE_BLUE_SIZE: u32 = 32862u32;
pub const GL_TEXTURE_BORDER: u32 = 4101u32;
pub const GL_TEXTURE_BORDER_COLOR: u32 = 4100u32;
pub const GL_TEXTURE_COMPONENTS: u32 = 4099u32;
pub const GL_TEXTURE_COORD_ARRAY: u32 = 32888u32;
pub const GL_TEXTURE_COORD_ARRAY_COUNT_EXT: u32 = 32907u32;
pub const GL_TEXTURE_COORD_ARRAY_EXT: u32 = 32888u32;
pub const GL_TEXTURE_COORD_ARRAY_POINTER: u32 = 32914u32;
pub const GL_TEXTURE_COORD_ARRAY_POINTER_EXT: u32 = 32914u32;
pub const GL_TEXTURE_COORD_ARRAY_SIZE: u32 = 32904u32;
pub const GL_TEXTURE_COORD_ARRAY_SIZE_EXT: u32 = 32904u32;
pub const GL_TEXTURE_COORD_ARRAY_STRIDE: u32 = 32906u32;
pub const GL_TEXTURE_COORD_ARRAY_STRIDE_EXT: u32 = 32906u32;
pub const GL_TEXTURE_COORD_ARRAY_TYPE: u32 = 32905u32;
pub const GL_TEXTURE_COORD_ARRAY_TYPE_EXT: u32 = 32905u32;
pub const GL_TEXTURE_ENV: u32 = 8960u32;
pub const GL_TEXTURE_ENV_COLOR: u32 = 8705u32;
pub const GL_TEXTURE_ENV_MODE: u32 = 8704u32;
pub const GL_TEXTURE_GEN_MODE: u32 = 9472u32;
pub const GL_TEXTURE_GEN_Q: u32 = 3171u32;
pub const GL_TEXTURE_GEN_R: u32 = 3170u32;
pub const GL_TEXTURE_GEN_S: u32 = 3168u32;
pub const GL_TEXTURE_GEN_T: u32 = 3169u32;
pub const GL_TEXTURE_GREEN_SIZE: u32 = 32861u32;
pub const GL_TEXTURE_HEIGHT: u32 = 4097u32;
pub const GL_TEXTURE_INTENSITY_SIZE: u32 = 32865u32;
pub const GL_TEXTURE_INTERNAL_FORMAT: u32 = 4099u32;
pub const GL_TEXTURE_LUMINANCE_SIZE: u32 = 32864u32;
pub const GL_TEXTURE_MAG_FILTER: u32 = 10240u32;
pub const GL_TEXTURE_MATRIX: u32 = 2984u32;
pub const GL_TEXTURE_MIN_FILTER: u32 = 10241u32;
pub const GL_TEXTURE_PRIORITY: u32 = 32870u32;
pub const GL_TEXTURE_RED_SIZE: u32 = 32860u32;
pub const GL_TEXTURE_RESIDENT: u32 = 32871u32;
pub const GL_TEXTURE_STACK_DEPTH: u32 = 2981u32;
pub const GL_TEXTURE_WIDTH: u32 = 4096u32;
pub const GL_TEXTURE_WRAP_S: u32 = 10242u32;
pub const GL_TEXTURE_WRAP_T: u32 = 10243u32;
pub const GL_TRANSFORM_BIT: u32 = 4096u32;
pub const GL_TRIANGLES: u32 = 4u32;
pub const GL_TRIANGLE_FAN: u32 = 6u32;
pub const GL_TRIANGLE_STRIP: u32 = 5u32;
pub const GL_TRUE: u32 = 1u32;
pub const GL_UNPACK_ALIGNMENT: u32 = 3317u32;
pub const GL_UNPACK_LSB_FIRST: u32 = 3313u32;
pub const GL_UNPACK_ROW_LENGTH: u32 = 3314u32;
pub const GL_UNPACK_SKIP_PIXELS: u32 = 3316u32;
pub const GL_UNPACK_SKIP_ROWS: u32 = 3315u32;
pub const GL_UNPACK_SWAP_BYTES: u32 = 3312u32;
pub const GL_UNSIGNED_BYTE: u32 = 5121u32;
pub const GL_UNSIGNED_INT: u32 = 5125u32;
pub const GL_UNSIGNED_SHORT: u32 = 5123u32;
pub const GL_V2F: u32 = 10784u32;
pub const GL_V3F: u32 = 10785u32;
pub const GL_VENDOR: u32 = 7936u32;
pub const GL_VERSION: u32 = 7938u32;
pub const GL_VERSION_1_1: u32 = 1u32;
pub const GL_VERTEX_ARRAY: u32 = 32884u32;
pub const GL_VERTEX_ARRAY_COUNT_EXT: u32 = 32893u32;
pub const GL_VERTEX_ARRAY_EXT: u32 = 32884u32;
pub const GL_VERTEX_ARRAY_POINTER: u32 = 32910u32;
pub const GL_VERTEX_ARRAY_POINTER_EXT: u32 = 32910u32;
pub const GL_VERTEX_ARRAY_SIZE: u32 = 32890u32;
pub const GL_VERTEX_ARRAY_SIZE_EXT: u32 = 32890u32;
pub const GL_VERTEX_ARRAY_STRIDE: u32 = 32892u32;
pub const GL_VERTEX_ARRAY_STRIDE_EXT: u32 = 32892u32;
pub const GL_VERTEX_ARRAY_TYPE: u32 = 32891u32;
pub const GL_VERTEX_ARRAY_TYPE_EXT: u32 = 32891u32;
pub const GL_VIEWPORT: u32 = 2978u32;
pub const GL_VIEWPORT_BIT: u32 = 2048u32;
pub const GL_WIN_draw_range_elements: u32 = 1u32;
pub const GL_WIN_swap_hint: u32 = 1u32;
pub const GL_XOR: u32 = 5382u32;
pub const GL_ZERO: u32 = 0u32;
pub const GL_ZOOM_X: u32 = 3350u32;
pub const GL_ZOOM_Y: u32 = 3351u32;
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetEnhMetaFilePixelFormat<'a, Param0: ::windows::core::IntoParam<'a, super::Gdi::HENHMETAFILE>>(hemf: Param0, cbbuffer: u32, ppfd: *mut PIXELFORMATDESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetEnhMetaFilePixelFormat(hemf: super::Gdi::HENHMETAFILE, cbbuffer: u32, ppfd: *mut PIXELFORMATDESCRIPTOR) -> u32;
        }
        ::core::mem::transmute(GetEnhMetaFilePixelFormat(hemf.into_param().abi(), ::core::mem::transmute(cbbuffer), ::core::mem::transmute(ppfd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetPixelFormat<'a, Param0: ::windows::core::IntoParam<'a, super::Gdi::HDC>>(hdc: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPixelFormat(hdc: super::Gdi::HDC) -> i32;
        }
        ::core::mem::transmute(GetPixelFormat(hdc.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct HGLRC(pub isize);
impl ::core::default::Default for HGLRC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for HGLRC {}
unsafe impl ::windows::core::Abi for HGLRC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct LAYERPLANEDESCRIPTOR {
    pub nSize: u16,
    pub nVersion: u16,
    pub dwFlags: u32,
    pub iPixelType: u8,
    pub cColorBits: u8,
    pub cRedBits: u8,
    pub cRedShift: u8,
    pub cGreenBits: u8,
    pub cGreenShift: u8,
    pub cBlueBits: u8,
    pub cBlueShift: u8,
    pub cAlphaBits: u8,
    pub cAlphaShift: u8,
    pub cAccumBits: u8,
    pub cAccumRedBits: u8,
    pub cAccumGreenBits: u8,
    pub cAccumBlueBits: u8,
    pub cAccumAlphaBits: u8,
    pub cDepthBits: u8,
    pub cStencilBits: u8,
    pub cAuxBuffers: u8,
    pub iLayerPlane: u8,
    pub bReserved: u8,
    pub crTransparent: u32,
}
impl LAYERPLANEDESCRIPTOR {}
impl ::core::default::Default for LAYERPLANEDESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for LAYERPLANEDESCRIPTOR {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("LAYERPLANEDESCRIPTOR")
            .field("nSize", &self.nSize)
            .field("nVersion", &self.nVersion)
            .field("dwFlags", &self.dwFlags)
            .field("iPixelType", &self.iPixelType)
            .field("cColorBits", &self.cColorBits)
            .field("cRedBits", &self.cRedBits)
            .field("cRedShift", &self.cRedShift)
            .field("cGreenBits", &self.cGreenBits)
            .field("cGreenShift", &self.cGreenShift)
            .field("cBlueBits", &self.cBlueBits)
            .field("cBlueShift", &self.cBlueShift)
            .field("cAlphaBits", &self.cAlphaBits)
            .field("cAlphaShift", &self.cAlphaShift)
            .field("cAccumBits", &self.cAccumBits)
            .field("cAccumRedBits", &self.cAccumRedBits)
            .field("cAccumGreenBits", &self.cAccumGreenBits)
            .field("cAccumBlueBits", &self.cAccumBlueBits)
            .field("cAccumAlphaBits", &self.cAccumAlphaBits)
            .field("cDepthBits", &self.cDepthBits)
            .field("cStencilBits", &self.cStencilBits)
            .field("cAuxBuffers", &self.cAuxBuffers)
            .field("iLayerPlane", &self.iLayerPlane)
            .field("bReserved", &self.bReserved)
            .field("crTransparent", &self.crTransparent)
            .finish()
    }
}
impl ::core::cmp::PartialEq for LAYERPLANEDESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.nSize == other.nSize
            && self.nVersion == other.nVersion
            && self.dwFlags == other.dwFlags
            && self.iPixelType == other.iPixelType
            && self.cColorBits == other.cColorBits
            && self.cRedBits == other.cRedBits
            && self.cRedShift == other.cRedShift
            && self.cGreenBits == other.cGreenBits
            && self.cGreenShift == other.cGreenShift
            && self.cBlueBits == other.cBlueBits
            && self.cBlueShift == other.cBlueShift
            && self.cAlphaBits == other.cAlphaBits
            && self.cAlphaShift == other.cAlphaShift
            && self.cAccumBits == other.cAccumBits
            && self.cAccumRedBits == other.cAccumRedBits
            && self.cAccumGreenBits == other.cAccumGreenBits
            && self.cAccumBlueBits == other.cAccumBlueBits
            && self.cAccumAlphaBits == other.cAccumAlphaBits
            && self.cDepthBits == other.cDepthBits
            && self.cStencilBits == other.cStencilBits
            && self.cAuxBuffers == other.cAuxBuffers
            && self.iLayerPlane == other.iLayerPlane
            && self.bReserved == other.bReserved
            && self.crTransparent == other.crTransparent
    }
}
impl ::core::cmp::Eq for LAYERPLANEDESCRIPTOR {}
unsafe impl ::windows::core::Abi for LAYERPLANEDESCRIPTOR {
    type Abi = Self;
}
pub type PFNGLADDSWAPHINTRECTWINPROC = ::core::option::Option<unsafe extern "system" fn(x: i32, y: i32, width: i32, height: i32)>;
pub type PFNGLARRAYELEMENTARRAYEXTPROC = ::core::option::Option<unsafe extern "system" fn(mode: u32, count: i32, pi: *const ::core::ffi::c_void)>;
pub type PFNGLARRAYELEMENTEXTPROC = ::core::option::Option<unsafe extern "system" fn(i: i32)>;
pub type PFNGLCOLORPOINTEREXTPROC = ::core::option::Option<unsafe extern "system" fn(size: i32, r#type: u32, stride: i32, count: i32, pointer: *const ::core::ffi::c_void)>;
pub type PFNGLCOLORSUBTABLEEXTPROC = ::core::option::Option<unsafe extern "system" fn(target: u32, start: i32, count: i32, format: u32, r#type: u32, data: *const ::core::ffi::c_void)>;
pub type PFNGLCOLORTABLEEXTPROC = ::core::option::Option<unsafe extern "system" fn(target: u32, internalformat: u32, width: i32, format: u32, r#type: u32, data: *const ::core::ffi::c_void)>;
pub type PFNGLDRAWARRAYSEXTPROC = ::core::option::Option<unsafe extern "system" fn(mode: u32, first: i32, count: i32)>;
pub type PFNGLDRAWRANGEELEMENTSWINPROC = ::core::option::Option<unsafe extern "system" fn(mode: u32, start: u32, end: u32, count: i32, r#type: u32, indices: *const ::core::ffi::c_void)>;
pub type PFNGLEDGEFLAGPOINTEREXTPROC = ::core::option::Option<unsafe extern "system" fn(stride: i32, count: i32, pointer: *const u8)>;
pub type PFNGLGETCOLORTABLEEXTPROC = ::core::option::Option<unsafe extern "system" fn(target: u32, format: u32, r#type: u32, data: *mut ::core::ffi::c_void)>;
pub type PFNGLGETCOLORTABLEPARAMETERFVEXTPROC = ::core::option::Option<unsafe extern "system" fn(target: u32, pname: u32, params: *mut f32)>;
pub type PFNGLGETCOLORTABLEPARAMETERIVEXTPROC = ::core::option::Option<unsafe extern "system" fn(target: u32, pname: u32, params: *mut i32)>;
pub type PFNGLGETPOINTERVEXTPROC = ::core::option::Option<unsafe extern "system" fn(pname: u32, params: *mut *mut ::core::ffi::c_void)>;
pub type PFNGLINDEXPOINTEREXTPROC = ::core::option::Option<unsafe extern "system" fn(r#type: u32, stride: i32, count: i32, pointer: *const ::core::ffi::c_void)>;
pub type PFNGLNORMALPOINTEREXTPROC = ::core::option::Option<unsafe extern "system" fn(r#type: u32, stride: i32, count: i32, pointer: *const ::core::ffi::c_void)>;
pub type PFNGLTEXCOORDPOINTEREXTPROC = ::core::option::Option<unsafe extern "system" fn(size: i32, r#type: u32, stride: i32, count: i32, pointer: *const ::core::ffi::c_void)>;
pub type PFNGLVERTEXPOINTEREXTPROC = ::core::option::Option<unsafe extern "system" fn(size: i32, r#type: u32, stride: i32, count: i32, pointer: *const ::core::ffi::c_void)>;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PIXELFORMATDESCRIPTOR {
    pub nSize: u16,
    pub nVersion: u16,
    pub dwFlags: u32,
    pub iPixelType: u8,
    pub cColorBits: u8,
    pub cRedBits: u8,
    pub cRedShift: u8,
    pub cGreenBits: u8,
    pub cGreenShift: u8,
    pub cBlueBits: u8,
    pub cBlueShift: u8,
    pub cAlphaBits: u8,
    pub cAlphaShift: u8,
    pub cAccumBits: u8,
    pub cAccumRedBits: u8,
    pub cAccumGreenBits: u8,
    pub cAccumBlueBits: u8,
    pub cAccumAlphaBits: u8,
    pub cDepthBits: u8,
    pub cStencilBits: u8,
    pub cAuxBuffers: u8,
    pub iLayerType: u8,
    pub bReserved: u8,
    pub dwLayerMask: u32,
    pub dwVisibleMask: u32,
    pub dwDamageMask: u32,
}
impl PIXELFORMATDESCRIPTOR {}
impl ::core::default::Default for PIXELFORMATDESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PIXELFORMATDESCRIPTOR {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PIXELFORMATDESCRIPTOR")
            .field("nSize", &self.nSize)
            .field("nVersion", &self.nVersion)
            .field("dwFlags", &self.dwFlags)
            .field("iPixelType", &self.iPixelType)
            .field("cColorBits", &self.cColorBits)
            .field("cRedBits", &self.cRedBits)
            .field("cRedShift", &self.cRedShift)
            .field("cGreenBits", &self.cGreenBits)
            .field("cGreenShift", &self.cGreenShift)
            .field("cBlueBits", &self.cBlueBits)
            .field("cBlueShift", &self.cBlueShift)
            .field("cAlphaBits", &self.cAlphaBits)
            .field("cAlphaShift", &self.cAlphaShift)
            .field("cAccumBits", &self.cAccumBits)
            .field("cAccumRedBits", &self.cAccumRedBits)
            .field("cAccumGreenBits", &self.cAccumGreenBits)
            .field("cAccumBlueBits", &self.cAccumBlueBits)
            .field("cAccumAlphaBits", &self.cAccumAlphaBits)
            .field("cDepthBits", &self.cDepthBits)
            .field("cStencilBits", &self.cStencilBits)
            .field("cAuxBuffers", &self.cAuxBuffers)
            .field("iLayerType", &self.iLayerType)
            .field("bReserved", &self.bReserved)
            .field("dwLayerMask", &self.dwLayerMask)
            .field("dwVisibleMask", &self.dwVisibleMask)
            .field("dwDamageMask", &self.dwDamageMask)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PIXELFORMATDESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.nSize == other.nSize
            && self.nVersion == other.nVersion
            && self.dwFlags == other.dwFlags
            && self.iPixelType == other.iPixelType
            && self.cColorBits == other.cColorBits
            && self.cRedBits == other.cRedBits
            && self.cRedShift == other.cRedShift
            && self.cGreenBits == other.cGreenBits
            && self.cGreenShift == other.cGreenShift
            && self.cBlueBits == other.cBlueBits
            && self.cBlueShift == other.cBlueShift
            && self.cAlphaBits == other.cAlphaBits
            && self.cAlphaShift == other.cAlphaShift
            && self.cAccumBits == other.cAccumBits
            && self.cAccumRedBits == other.cAccumRedBits
            && self.cAccumGreenBits == other.cAccumGreenBits
            && self.cAccumBlueBits == other.cAccumBlueBits
            && self.cAccumAlphaBits == other.cAccumAlphaBits
            && self.cDepthBits == other.cDepthBits
            && self.cStencilBits == other.cStencilBits
            && self.cAuxBuffers == other.cAuxBuffers
            && self.iLayerType == other.iLayerType
            && self.bReserved == other.bReserved
            && self.dwLayerMask == other.dwLayerMask
            && self.dwVisibleMask == other.dwVisibleMask
            && self.dwDamageMask == other.dwDamageMask
    }
}
impl ::core::cmp::Eq for PIXELFORMATDESCRIPTOR {}
unsafe impl ::windows::core::Abi for PIXELFORMATDESCRIPTOR {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct POINTFLOAT {
    pub x: f32,
    pub y: f32,
}
impl POINTFLOAT {}
impl ::core::default::Default for POINTFLOAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for POINTFLOAT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("POINTFLOAT").field("x", &self.x).field("y", &self.y).finish()
    }
}
impl ::core::cmp::PartialEq for POINTFLOAT {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl ::core::cmp::Eq for POINTFLOAT {}
unsafe impl ::windows::core::Abi for POINTFLOAT {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn SetPixelFormat<'a, Param0: ::windows::core::IntoParam<'a, super::Gdi::HDC>>(hdc: Param0, format: i32, ppfd: *const PIXELFORMATDESCRIPTOR) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetPixelFormat(hdc: super::Gdi::HDC, format: i32, ppfd: *const PIXELFORMATDESCRIPTOR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetPixelFormat(hdc.into_param().abi(), ::core::mem::transmute(format), ::core::mem::transmute(ppfd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn SwapBuffers<'a, Param0: ::windows::core::IntoParam<'a, super::Gdi::HDC>>(param0: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwapBuffers(param0: super::Gdi::HDC) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SwapBuffers(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glAccum(op: u32, value: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glAccum(op: u32, value: f32);
        }
        ::core::mem::transmute(glAccum(::core::mem::transmute(op), ::core::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glAlphaFunc(func: u32, r#ref: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glAlphaFunc(func: u32, r#ref: f32);
        }
        ::core::mem::transmute(glAlphaFunc(::core::mem::transmute(func), ::core::mem::transmute(r#ref)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glAreTexturesResident(n: i32, textures: *const u32, residences: *mut u8) -> u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glAreTexturesResident(n: i32, textures: *const u32, residences: *mut u8) -> u8;
        }
        ::core::mem::transmute(glAreTexturesResident(::core::mem::transmute(n), ::core::mem::transmute(textures), ::core::mem::transmute(residences)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glArrayElement(i: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glArrayElement(i: i32);
        }
        ::core::mem::transmute(glArrayElement(::core::mem::transmute(i)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glBegin(mode: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glBegin(mode: u32);
        }
        ::core::mem::transmute(glBegin(::core::mem::transmute(mode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glBindTexture(target: u32, texture: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glBindTexture(target: u32, texture: u32);
        }
        ::core::mem::transmute(glBindTexture(::core::mem::transmute(target), ::core::mem::transmute(texture)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glBitmap(width: i32, height: i32, xorig: f32, yorig: f32, xmove: f32, ymove: f32, bitmap: *const u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glBitmap(width: i32, height: i32, xorig: f32, yorig: f32, xmove: f32, ymove: f32, bitmap: *const u8);
        }
        ::core::mem::transmute(glBitmap(::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(xorig), ::core::mem::transmute(yorig), ::core::mem::transmute(xmove), ::core::mem::transmute(ymove), ::core::mem::transmute(bitmap)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glBlendFunc(sfactor: u32, dfactor: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glBlendFunc(sfactor: u32, dfactor: u32);
        }
        ::core::mem::transmute(glBlendFunc(::core::mem::transmute(sfactor), ::core::mem::transmute(dfactor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glCallList(list: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glCallList(list: u32);
        }
        ::core::mem::transmute(glCallList(::core::mem::transmute(list)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glCallLists(n: i32, r#type: u32, lists: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glCallLists(n: i32, r#type: u32, lists: *const ::core::ffi::c_void);
        }
        ::core::mem::transmute(glCallLists(::core::mem::transmute(n), ::core::mem::transmute(r#type), ::core::mem::transmute(lists)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glClear(mask: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glClear(mask: u32);
        }
        ::core::mem::transmute(glClear(::core::mem::transmute(mask)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glClearAccum(red: f32, green: f32, blue: f32, alpha: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glClearAccum(red: f32, green: f32, blue: f32, alpha: f32);
        }
        ::core::mem::transmute(glClearAccum(::core::mem::transmute(red), ::core::mem::transmute(green), ::core::mem::transmute(blue), ::core::mem::transmute(alpha)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glClearColor(red: f32, green: f32, blue: f32, alpha: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glClearColor(red: f32, green: f32, blue: f32, alpha: f32);
        }
        ::core::mem::transmute(glClearColor(::core::mem::transmute(red), ::core::mem::transmute(green), ::core::mem::transmute(blue), ::core::mem::transmute(alpha)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glClearDepth(depth: f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glClearDepth(depth: f64);
        }
        ::core::mem::transmute(glClearDepth(::core::mem::transmute(depth)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glClearIndex(c: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glClearIndex(c: f32);
        }
        ::core::mem::transmute(glClearIndex(::core::mem::transmute(c)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glClearStencil(s: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glClearStencil(s: i32);
        }
        ::core::mem::transmute(glClearStencil(::core::mem::transmute(s)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glClipPlane(plane: u32, equation: *const f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glClipPlane(plane: u32, equation: *const f64);
        }
        ::core::mem::transmute(glClipPlane(::core::mem::transmute(plane), ::core::mem::transmute(equation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glColor3b(red: i8, green: i8, blue: i8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glColor3b(red: i8, green: i8, blue: i8);
        }
        ::core::mem::transmute(glColor3b(::core::mem::transmute(red), ::core::mem::transmute(green), ::core::mem::transmute(blue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glColor3bv(v: *const i8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glColor3bv(v: *const i8);
        }
        ::core::mem::transmute(glColor3bv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glColor3d(red: f64, green: f64, blue: f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glColor3d(red: f64, green: f64, blue: f64);
        }
        ::core::mem::transmute(glColor3d(::core::mem::transmute(red), ::core::mem::transmute(green), ::core::mem::transmute(blue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glColor3dv(v: *const f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glColor3dv(v: *const f64);
        }
        ::core::mem::transmute(glColor3dv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glColor3f(red: f32, green: f32, blue: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glColor3f(red: f32, green: f32, blue: f32);
        }
        ::core::mem::transmute(glColor3f(::core::mem::transmute(red), ::core::mem::transmute(green), ::core::mem::transmute(blue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glColor3fv(v: *const f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glColor3fv(v: *const f32);
        }
        ::core::mem::transmute(glColor3fv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glColor3i(red: i32, green: i32, blue: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glColor3i(red: i32, green: i32, blue: i32);
        }
        ::core::mem::transmute(glColor3i(::core::mem::transmute(red), ::core::mem::transmute(green), ::core::mem::transmute(blue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glColor3iv(v: *const i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glColor3iv(v: *const i32);
        }
        ::core::mem::transmute(glColor3iv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glColor3s(red: i16, green: i16, blue: i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glColor3s(red: i16, green: i16, blue: i16);
        }
        ::core::mem::transmute(glColor3s(::core::mem::transmute(red), ::core::mem::transmute(green), ::core::mem::transmute(blue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glColor3sv(v: *const i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glColor3sv(v: *const i16);
        }
        ::core::mem::transmute(glColor3sv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glColor3ub(red: u8, green: u8, blue: u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glColor3ub(red: u8, green: u8, blue: u8);
        }
        ::core::mem::transmute(glColor3ub(::core::mem::transmute(red), ::core::mem::transmute(green), ::core::mem::transmute(blue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glColor3ubv(v: *const u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glColor3ubv(v: *const u8);
        }
        ::core::mem::transmute(glColor3ubv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glColor3ui(red: u32, green: u32, blue: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glColor3ui(red: u32, green: u32, blue: u32);
        }
        ::core::mem::transmute(glColor3ui(::core::mem::transmute(red), ::core::mem::transmute(green), ::core::mem::transmute(blue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glColor3uiv(v: *const u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glColor3uiv(v: *const u32);
        }
        ::core::mem::transmute(glColor3uiv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glColor3us(red: u16, green: u16, blue: u16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glColor3us(red: u16, green: u16, blue: u16);
        }
        ::core::mem::transmute(glColor3us(::core::mem::transmute(red), ::core::mem::transmute(green), ::core::mem::transmute(blue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glColor3usv(v: *const u16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glColor3usv(v: *const u16);
        }
        ::core::mem::transmute(glColor3usv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glColor4b(red: i8, green: i8, blue: i8, alpha: i8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glColor4b(red: i8, green: i8, blue: i8, alpha: i8);
        }
        ::core::mem::transmute(glColor4b(::core::mem::transmute(red), ::core::mem::transmute(green), ::core::mem::transmute(blue), ::core::mem::transmute(alpha)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glColor4bv(v: *const i8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glColor4bv(v: *const i8);
        }
        ::core::mem::transmute(glColor4bv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glColor4d(red: f64, green: f64, blue: f64, alpha: f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glColor4d(red: f64, green: f64, blue: f64, alpha: f64);
        }
        ::core::mem::transmute(glColor4d(::core::mem::transmute(red), ::core::mem::transmute(green), ::core::mem::transmute(blue), ::core::mem::transmute(alpha)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glColor4dv(v: *const f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glColor4dv(v: *const f64);
        }
        ::core::mem::transmute(glColor4dv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glColor4f(red: f32, green: f32, blue: f32, alpha: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glColor4f(red: f32, green: f32, blue: f32, alpha: f32);
        }
        ::core::mem::transmute(glColor4f(::core::mem::transmute(red), ::core::mem::transmute(green), ::core::mem::transmute(blue), ::core::mem::transmute(alpha)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glColor4fv(v: *const f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glColor4fv(v: *const f32);
        }
        ::core::mem::transmute(glColor4fv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glColor4i(red: i32, green: i32, blue: i32, alpha: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glColor4i(red: i32, green: i32, blue: i32, alpha: i32);
        }
        ::core::mem::transmute(glColor4i(::core::mem::transmute(red), ::core::mem::transmute(green), ::core::mem::transmute(blue), ::core::mem::transmute(alpha)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glColor4iv(v: *const i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glColor4iv(v: *const i32);
        }
        ::core::mem::transmute(glColor4iv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glColor4s(red: i16, green: i16, blue: i16, alpha: i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glColor4s(red: i16, green: i16, blue: i16, alpha: i16);
        }
        ::core::mem::transmute(glColor4s(::core::mem::transmute(red), ::core::mem::transmute(green), ::core::mem::transmute(blue), ::core::mem::transmute(alpha)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glColor4sv(v: *const i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glColor4sv(v: *const i16);
        }
        ::core::mem::transmute(glColor4sv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glColor4ub(red: u8, green: u8, blue: u8, alpha: u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glColor4ub(red: u8, green: u8, blue: u8, alpha: u8);
        }
        ::core::mem::transmute(glColor4ub(::core::mem::transmute(red), ::core::mem::transmute(green), ::core::mem::transmute(blue), ::core::mem::transmute(alpha)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glColor4ubv(v: *const u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glColor4ubv(v: *const u8);
        }
        ::core::mem::transmute(glColor4ubv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glColor4ui(red: u32, green: u32, blue: u32, alpha: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glColor4ui(red: u32, green: u32, blue: u32, alpha: u32);
        }
        ::core::mem::transmute(glColor4ui(::core::mem::transmute(red), ::core::mem::transmute(green), ::core::mem::transmute(blue), ::core::mem::transmute(alpha)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glColor4uiv(v: *const u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glColor4uiv(v: *const u32);
        }
        ::core::mem::transmute(glColor4uiv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glColor4us(red: u16, green: u16, blue: u16, alpha: u16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glColor4us(red: u16, green: u16, blue: u16, alpha: u16);
        }
        ::core::mem::transmute(glColor4us(::core::mem::transmute(red), ::core::mem::transmute(green), ::core::mem::transmute(blue), ::core::mem::transmute(alpha)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glColor4usv(v: *const u16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glColor4usv(v: *const u16);
        }
        ::core::mem::transmute(glColor4usv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glColorMask(red: u8, green: u8, blue: u8, alpha: u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glColorMask(red: u8, green: u8, blue: u8, alpha: u8);
        }
        ::core::mem::transmute(glColorMask(::core::mem::transmute(red), ::core::mem::transmute(green), ::core::mem::transmute(blue), ::core::mem::transmute(alpha)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glColorMaterial(face: u32, mode: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glColorMaterial(face: u32, mode: u32);
        }
        ::core::mem::transmute(glColorMaterial(::core::mem::transmute(face), ::core::mem::transmute(mode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glColorPointer(size: i32, r#type: u32, stride: i32, pointer: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glColorPointer(size: i32, r#type: u32, stride: i32, pointer: *const ::core::ffi::c_void);
        }
        ::core::mem::transmute(glColorPointer(::core::mem::transmute(size), ::core::mem::transmute(r#type), ::core::mem::transmute(stride), ::core::mem::transmute(pointer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glCopyPixels(x: i32, y: i32, width: i32, height: i32, r#type: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glCopyPixels(x: i32, y: i32, width: i32, height: i32, r#type: u32);
        }
        ::core::mem::transmute(glCopyPixels(::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(r#type)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glCopyTexImage1D(target: u32, level: i32, internalformat: u32, x: i32, y: i32, width: i32, border: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glCopyTexImage1D(target: u32, level: i32, internalformat: u32, x: i32, y: i32, width: i32, border: i32);
        }
        ::core::mem::transmute(glCopyTexImage1D(::core::mem::transmute(target), ::core::mem::transmute(level), ::core::mem::transmute(internalformat), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(width), ::core::mem::transmute(border)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glCopyTexImage2D(target: u32, level: i32, internalformat: u32, x: i32, y: i32, width: i32, height: i32, border: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glCopyTexImage2D(target: u32, level: i32, internalformat: u32, x: i32, y: i32, width: i32, height: i32, border: i32);
        }
        ::core::mem::transmute(glCopyTexImage2D(::core::mem::transmute(target), ::core::mem::transmute(level), ::core::mem::transmute(internalformat), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(border)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glCopyTexSubImage1D(target: u32, level: i32, xoffset: i32, x: i32, y: i32, width: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glCopyTexSubImage1D(target: u32, level: i32, xoffset: i32, x: i32, y: i32, width: i32);
        }
        ::core::mem::transmute(glCopyTexSubImage1D(::core::mem::transmute(target), ::core::mem::transmute(level), ::core::mem::transmute(xoffset), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(width)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glCopyTexSubImage2D(target: u32, level: i32, xoffset: i32, yoffset: i32, x: i32, y: i32, width: i32, height: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glCopyTexSubImage2D(target: u32, level: i32, xoffset: i32, yoffset: i32, x: i32, y: i32, width: i32, height: i32);
        }
        ::core::mem::transmute(glCopyTexSubImage2D(::core::mem::transmute(target), ::core::mem::transmute(level), ::core::mem::transmute(xoffset), ::core::mem::transmute(yoffset), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(width), ::core::mem::transmute(height)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glCullFace(mode: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glCullFace(mode: u32);
        }
        ::core::mem::transmute(glCullFace(::core::mem::transmute(mode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glDeleteLists(list: u32, range: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glDeleteLists(list: u32, range: i32);
        }
        ::core::mem::transmute(glDeleteLists(::core::mem::transmute(list), ::core::mem::transmute(range)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glDeleteTextures(n: i32, textures: *const u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glDeleteTextures(n: i32, textures: *const u32);
        }
        ::core::mem::transmute(glDeleteTextures(::core::mem::transmute(n), ::core::mem::transmute(textures)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glDepthFunc(func: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glDepthFunc(func: u32);
        }
        ::core::mem::transmute(glDepthFunc(::core::mem::transmute(func)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glDepthMask(flag: u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glDepthMask(flag: u8);
        }
        ::core::mem::transmute(glDepthMask(::core::mem::transmute(flag)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glDepthRange(znear: f64, zfar: f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glDepthRange(znear: f64, zfar: f64);
        }
        ::core::mem::transmute(glDepthRange(::core::mem::transmute(znear), ::core::mem::transmute(zfar)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glDisable(cap: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glDisable(cap: u32);
        }
        ::core::mem::transmute(glDisable(::core::mem::transmute(cap)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glDisableClientState(array: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glDisableClientState(array: u32);
        }
        ::core::mem::transmute(glDisableClientState(::core::mem::transmute(array)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glDrawArrays(mode: u32, first: i32, count: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glDrawArrays(mode: u32, first: i32, count: i32);
        }
        ::core::mem::transmute(glDrawArrays(::core::mem::transmute(mode), ::core::mem::transmute(first), ::core::mem::transmute(count)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glDrawBuffer(mode: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glDrawBuffer(mode: u32);
        }
        ::core::mem::transmute(glDrawBuffer(::core::mem::transmute(mode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glDrawElements(mode: u32, count: i32, r#type: u32, indices: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glDrawElements(mode: u32, count: i32, r#type: u32, indices: *const ::core::ffi::c_void);
        }
        ::core::mem::transmute(glDrawElements(::core::mem::transmute(mode), ::core::mem::transmute(count), ::core::mem::transmute(r#type), ::core::mem::transmute(indices)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glDrawPixels(width: i32, height: i32, format: u32, r#type: u32, pixels: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glDrawPixels(width: i32, height: i32, format: u32, r#type: u32, pixels: *const ::core::ffi::c_void);
        }
        ::core::mem::transmute(glDrawPixels(::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(format), ::core::mem::transmute(r#type), ::core::mem::transmute(pixels)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glEdgeFlag(flag: u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glEdgeFlag(flag: u8);
        }
        ::core::mem::transmute(glEdgeFlag(::core::mem::transmute(flag)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glEdgeFlagPointer(stride: i32, pointer: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glEdgeFlagPointer(stride: i32, pointer: *const ::core::ffi::c_void);
        }
        ::core::mem::transmute(glEdgeFlagPointer(::core::mem::transmute(stride), ::core::mem::transmute(pointer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glEdgeFlagv(flag: *const u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glEdgeFlagv(flag: *const u8);
        }
        ::core::mem::transmute(glEdgeFlagv(::core::mem::transmute(flag)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glEnable(cap: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glEnable(cap: u32);
        }
        ::core::mem::transmute(glEnable(::core::mem::transmute(cap)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glEnableClientState(array: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glEnableClientState(array: u32);
        }
        ::core::mem::transmute(glEnableClientState(::core::mem::transmute(array)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glEnd() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glEnd();
        }
        ::core::mem::transmute(glEnd())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glEndList() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glEndList();
        }
        ::core::mem::transmute(glEndList())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glEvalCoord1d(u: f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glEvalCoord1d(u: f64);
        }
        ::core::mem::transmute(glEvalCoord1d(::core::mem::transmute(u)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glEvalCoord1dv(u: *const f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glEvalCoord1dv(u: *const f64);
        }
        ::core::mem::transmute(glEvalCoord1dv(::core::mem::transmute(u)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glEvalCoord1f(u: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glEvalCoord1f(u: f32);
        }
        ::core::mem::transmute(glEvalCoord1f(::core::mem::transmute(u)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glEvalCoord1fv(u: *const f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glEvalCoord1fv(u: *const f32);
        }
        ::core::mem::transmute(glEvalCoord1fv(::core::mem::transmute(u)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glEvalCoord2d(u: f64, v: f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glEvalCoord2d(u: f64, v: f64);
        }
        ::core::mem::transmute(glEvalCoord2d(::core::mem::transmute(u), ::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glEvalCoord2dv(u: *const f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glEvalCoord2dv(u: *const f64);
        }
        ::core::mem::transmute(glEvalCoord2dv(::core::mem::transmute(u)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glEvalCoord2f(u: f32, v: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glEvalCoord2f(u: f32, v: f32);
        }
        ::core::mem::transmute(glEvalCoord2f(::core::mem::transmute(u), ::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glEvalCoord2fv(u: *const f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glEvalCoord2fv(u: *const f32);
        }
        ::core::mem::transmute(glEvalCoord2fv(::core::mem::transmute(u)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glEvalMesh1(mode: u32, i1: i32, i2: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glEvalMesh1(mode: u32, i1: i32, i2: i32);
        }
        ::core::mem::transmute(glEvalMesh1(::core::mem::transmute(mode), ::core::mem::transmute(i1), ::core::mem::transmute(i2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glEvalMesh2(mode: u32, i1: i32, i2: i32, j1: i32, j2: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glEvalMesh2(mode: u32, i1: i32, i2: i32, j1: i32, j2: i32);
        }
        ::core::mem::transmute(glEvalMesh2(::core::mem::transmute(mode), ::core::mem::transmute(i1), ::core::mem::transmute(i2), ::core::mem::transmute(j1), ::core::mem::transmute(j2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glEvalPoint1(i: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glEvalPoint1(i: i32);
        }
        ::core::mem::transmute(glEvalPoint1(::core::mem::transmute(i)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glEvalPoint2(i: i32, j: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glEvalPoint2(i: i32, j: i32);
        }
        ::core::mem::transmute(glEvalPoint2(::core::mem::transmute(i), ::core::mem::transmute(j)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glFeedbackBuffer(size: i32, r#type: u32, buffer: *mut f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glFeedbackBuffer(size: i32, r#type: u32, buffer: *mut f32);
        }
        ::core::mem::transmute(glFeedbackBuffer(::core::mem::transmute(size), ::core::mem::transmute(r#type), ::core::mem::transmute(buffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glFinish() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glFinish();
        }
        ::core::mem::transmute(glFinish())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glFlush() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glFlush();
        }
        ::core::mem::transmute(glFlush())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glFogf(pname: u32, param1: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glFogf(pname: u32, param1: f32);
        }
        ::core::mem::transmute(glFogf(::core::mem::transmute(pname), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glFogfv(pname: u32, params: *const f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glFogfv(pname: u32, params: *const f32);
        }
        ::core::mem::transmute(glFogfv(::core::mem::transmute(pname), ::core::mem::transmute(params)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glFogi(pname: u32, param1: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glFogi(pname: u32, param1: i32);
        }
        ::core::mem::transmute(glFogi(::core::mem::transmute(pname), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glFogiv(pname: u32, params: *const i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glFogiv(pname: u32, params: *const i32);
        }
        ::core::mem::transmute(glFogiv(::core::mem::transmute(pname), ::core::mem::transmute(params)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glFrontFace(mode: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glFrontFace(mode: u32);
        }
        ::core::mem::transmute(glFrontFace(::core::mem::transmute(mode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glFrustum(left: f64, right: f64, bottom: f64, top: f64, znear: f64, zfar: f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glFrustum(left: f64, right: f64, bottom: f64, top: f64, znear: f64, zfar: f64);
        }
        ::core::mem::transmute(glFrustum(::core::mem::transmute(left), ::core::mem::transmute(right), ::core::mem::transmute(bottom), ::core::mem::transmute(top), ::core::mem::transmute(znear), ::core::mem::transmute(zfar)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glGenLists(range: i32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glGenLists(range: i32) -> u32;
        }
        ::core::mem::transmute(glGenLists(::core::mem::transmute(range)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glGenTextures(n: i32, textures: *mut u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glGenTextures(n: i32, textures: *mut u32);
        }
        ::core::mem::transmute(glGenTextures(::core::mem::transmute(n), ::core::mem::transmute(textures)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glGetBooleanv(pname: u32, params: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glGetBooleanv(pname: u32, params: *mut u8);
        }
        ::core::mem::transmute(glGetBooleanv(::core::mem::transmute(pname), ::core::mem::transmute(params)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glGetClipPlane(plane: u32, equation: *mut f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glGetClipPlane(plane: u32, equation: *mut f64);
        }
        ::core::mem::transmute(glGetClipPlane(::core::mem::transmute(plane), ::core::mem::transmute(equation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glGetDoublev(pname: u32, params: *mut f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glGetDoublev(pname: u32, params: *mut f64);
        }
        ::core::mem::transmute(glGetDoublev(::core::mem::transmute(pname), ::core::mem::transmute(params)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glGetError() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glGetError() -> u32;
        }
        ::core::mem::transmute(glGetError())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glGetFloatv(pname: u32, params: *mut f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glGetFloatv(pname: u32, params: *mut f32);
        }
        ::core::mem::transmute(glGetFloatv(::core::mem::transmute(pname), ::core::mem::transmute(params)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glGetIntegerv(pname: u32, params: *mut i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glGetIntegerv(pname: u32, params: *mut i32);
        }
        ::core::mem::transmute(glGetIntegerv(::core::mem::transmute(pname), ::core::mem::transmute(params)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glGetLightfv(light: u32, pname: u32, params: *mut f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glGetLightfv(light: u32, pname: u32, params: *mut f32);
        }
        ::core::mem::transmute(glGetLightfv(::core::mem::transmute(light), ::core::mem::transmute(pname), ::core::mem::transmute(params)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glGetLightiv(light: u32, pname: u32, params: *mut i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glGetLightiv(light: u32, pname: u32, params: *mut i32);
        }
        ::core::mem::transmute(glGetLightiv(::core::mem::transmute(light), ::core::mem::transmute(pname), ::core::mem::transmute(params)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glGetMapdv(target: u32, query: u32, v: *mut f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glGetMapdv(target: u32, query: u32, v: *mut f64);
        }
        ::core::mem::transmute(glGetMapdv(::core::mem::transmute(target), ::core::mem::transmute(query), ::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glGetMapfv(target: u32, query: u32, v: *mut f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glGetMapfv(target: u32, query: u32, v: *mut f32);
        }
        ::core::mem::transmute(glGetMapfv(::core::mem::transmute(target), ::core::mem::transmute(query), ::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glGetMapiv(target: u32, query: u32, v: *mut i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glGetMapiv(target: u32, query: u32, v: *mut i32);
        }
        ::core::mem::transmute(glGetMapiv(::core::mem::transmute(target), ::core::mem::transmute(query), ::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glGetMaterialfv(face: u32, pname: u32, params: *mut f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glGetMaterialfv(face: u32, pname: u32, params: *mut f32);
        }
        ::core::mem::transmute(glGetMaterialfv(::core::mem::transmute(face), ::core::mem::transmute(pname), ::core::mem::transmute(params)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glGetMaterialiv(face: u32, pname: u32, params: *mut i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glGetMaterialiv(face: u32, pname: u32, params: *mut i32);
        }
        ::core::mem::transmute(glGetMaterialiv(::core::mem::transmute(face), ::core::mem::transmute(pname), ::core::mem::transmute(params)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glGetPixelMapfv(map: u32, values: *mut f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glGetPixelMapfv(map: u32, values: *mut f32);
        }
        ::core::mem::transmute(glGetPixelMapfv(::core::mem::transmute(map), ::core::mem::transmute(values)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glGetPixelMapuiv(map: u32, values: *mut u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glGetPixelMapuiv(map: u32, values: *mut u32);
        }
        ::core::mem::transmute(glGetPixelMapuiv(::core::mem::transmute(map), ::core::mem::transmute(values)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glGetPixelMapusv(map: u32, values: *mut u16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glGetPixelMapusv(map: u32, values: *mut u16);
        }
        ::core::mem::transmute(glGetPixelMapusv(::core::mem::transmute(map), ::core::mem::transmute(values)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glGetPointerv(pname: u32, params: *mut *mut ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glGetPointerv(pname: u32, params: *mut *mut ::core::ffi::c_void);
        }
        ::core::mem::transmute(glGetPointerv(::core::mem::transmute(pname), ::core::mem::transmute(params)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glGetPolygonStipple(mask: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glGetPolygonStipple(mask: *mut u8);
        }
        ::core::mem::transmute(glGetPolygonStipple(::core::mem::transmute(mask)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glGetString(name: u32) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glGetString(name: u32) -> *mut u8;
        }
        ::core::mem::transmute(glGetString(::core::mem::transmute(name)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glGetTexEnvfv(target: u32, pname: u32, params: *mut f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glGetTexEnvfv(target: u32, pname: u32, params: *mut f32);
        }
        ::core::mem::transmute(glGetTexEnvfv(::core::mem::transmute(target), ::core::mem::transmute(pname), ::core::mem::transmute(params)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glGetTexEnviv(target: u32, pname: u32, params: *mut i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glGetTexEnviv(target: u32, pname: u32, params: *mut i32);
        }
        ::core::mem::transmute(glGetTexEnviv(::core::mem::transmute(target), ::core::mem::transmute(pname), ::core::mem::transmute(params)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glGetTexGendv(coord: u32, pname: u32, params: *mut f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glGetTexGendv(coord: u32, pname: u32, params: *mut f64);
        }
        ::core::mem::transmute(glGetTexGendv(::core::mem::transmute(coord), ::core::mem::transmute(pname), ::core::mem::transmute(params)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glGetTexGenfv(coord: u32, pname: u32, params: *mut f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glGetTexGenfv(coord: u32, pname: u32, params: *mut f32);
        }
        ::core::mem::transmute(glGetTexGenfv(::core::mem::transmute(coord), ::core::mem::transmute(pname), ::core::mem::transmute(params)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glGetTexGeniv(coord: u32, pname: u32, params: *mut i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glGetTexGeniv(coord: u32, pname: u32, params: *mut i32);
        }
        ::core::mem::transmute(glGetTexGeniv(::core::mem::transmute(coord), ::core::mem::transmute(pname), ::core::mem::transmute(params)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glGetTexImage(target: u32, level: i32, format: u32, r#type: u32, pixels: *mut ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glGetTexImage(target: u32, level: i32, format: u32, r#type: u32, pixels: *mut ::core::ffi::c_void);
        }
        ::core::mem::transmute(glGetTexImage(::core::mem::transmute(target), ::core::mem::transmute(level), ::core::mem::transmute(format), ::core::mem::transmute(r#type), ::core::mem::transmute(pixels)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glGetTexLevelParameterfv(target: u32, level: i32, pname: u32, params: *mut f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glGetTexLevelParameterfv(target: u32, level: i32, pname: u32, params: *mut f32);
        }
        ::core::mem::transmute(glGetTexLevelParameterfv(::core::mem::transmute(target), ::core::mem::transmute(level), ::core::mem::transmute(pname), ::core::mem::transmute(params)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glGetTexLevelParameteriv(target: u32, level: i32, pname: u32, params: *mut i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glGetTexLevelParameteriv(target: u32, level: i32, pname: u32, params: *mut i32);
        }
        ::core::mem::transmute(glGetTexLevelParameteriv(::core::mem::transmute(target), ::core::mem::transmute(level), ::core::mem::transmute(pname), ::core::mem::transmute(params)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glGetTexParameterfv(target: u32, pname: u32, params: *mut f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glGetTexParameterfv(target: u32, pname: u32, params: *mut f32);
        }
        ::core::mem::transmute(glGetTexParameterfv(::core::mem::transmute(target), ::core::mem::transmute(pname), ::core::mem::transmute(params)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glGetTexParameteriv(target: u32, pname: u32, params: *mut i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glGetTexParameteriv(target: u32, pname: u32, params: *mut i32);
        }
        ::core::mem::transmute(glGetTexParameteriv(::core::mem::transmute(target), ::core::mem::transmute(pname), ::core::mem::transmute(params)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glHint(target: u32, mode: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glHint(target: u32, mode: u32);
        }
        ::core::mem::transmute(glHint(::core::mem::transmute(target), ::core::mem::transmute(mode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glIndexMask(mask: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glIndexMask(mask: u32);
        }
        ::core::mem::transmute(glIndexMask(::core::mem::transmute(mask)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glIndexPointer(r#type: u32, stride: i32, pointer: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glIndexPointer(r#type: u32, stride: i32, pointer: *const ::core::ffi::c_void);
        }
        ::core::mem::transmute(glIndexPointer(::core::mem::transmute(r#type), ::core::mem::transmute(stride), ::core::mem::transmute(pointer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glIndexd(c: f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glIndexd(c: f64);
        }
        ::core::mem::transmute(glIndexd(::core::mem::transmute(c)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glIndexdv(c: *const f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glIndexdv(c: *const f64);
        }
        ::core::mem::transmute(glIndexdv(::core::mem::transmute(c)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glIndexf(c: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glIndexf(c: f32);
        }
        ::core::mem::transmute(glIndexf(::core::mem::transmute(c)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glIndexfv(c: *const f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glIndexfv(c: *const f32);
        }
        ::core::mem::transmute(glIndexfv(::core::mem::transmute(c)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glIndexi(c: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glIndexi(c: i32);
        }
        ::core::mem::transmute(glIndexi(::core::mem::transmute(c)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glIndexiv(c: *const i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glIndexiv(c: *const i32);
        }
        ::core::mem::transmute(glIndexiv(::core::mem::transmute(c)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glIndexs(c: i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glIndexs(c: i16);
        }
        ::core::mem::transmute(glIndexs(::core::mem::transmute(c)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glIndexsv(c: *const i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glIndexsv(c: *const i16);
        }
        ::core::mem::transmute(glIndexsv(::core::mem::transmute(c)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glIndexub(c: u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glIndexub(c: u8);
        }
        ::core::mem::transmute(glIndexub(::core::mem::transmute(c)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glIndexubv(c: *const u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glIndexubv(c: *const u8);
        }
        ::core::mem::transmute(glIndexubv(::core::mem::transmute(c)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glInitNames() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glInitNames();
        }
        ::core::mem::transmute(glInitNames())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glInterleavedArrays(format: u32, stride: i32, pointer: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glInterleavedArrays(format: u32, stride: i32, pointer: *const ::core::ffi::c_void);
        }
        ::core::mem::transmute(glInterleavedArrays(::core::mem::transmute(format), ::core::mem::transmute(stride), ::core::mem::transmute(pointer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glIsEnabled(cap: u32) -> u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glIsEnabled(cap: u32) -> u8;
        }
        ::core::mem::transmute(glIsEnabled(::core::mem::transmute(cap)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glIsList(list: u32) -> u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glIsList(list: u32) -> u8;
        }
        ::core::mem::transmute(glIsList(::core::mem::transmute(list)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glIsTexture(texture: u32) -> u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glIsTexture(texture: u32) -> u8;
        }
        ::core::mem::transmute(glIsTexture(::core::mem::transmute(texture)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glLightModelf(pname: u32, param1: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glLightModelf(pname: u32, param1: f32);
        }
        ::core::mem::transmute(glLightModelf(::core::mem::transmute(pname), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glLightModelfv(pname: u32, params: *const f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glLightModelfv(pname: u32, params: *const f32);
        }
        ::core::mem::transmute(glLightModelfv(::core::mem::transmute(pname), ::core::mem::transmute(params)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glLightModeli(pname: u32, param1: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glLightModeli(pname: u32, param1: i32);
        }
        ::core::mem::transmute(glLightModeli(::core::mem::transmute(pname), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glLightModeliv(pname: u32, params: *const i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glLightModeliv(pname: u32, params: *const i32);
        }
        ::core::mem::transmute(glLightModeliv(::core::mem::transmute(pname), ::core::mem::transmute(params)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glLightf(light: u32, pname: u32, param2: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glLightf(light: u32, pname: u32, param2: f32);
        }
        ::core::mem::transmute(glLightf(::core::mem::transmute(light), ::core::mem::transmute(pname), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glLightfv(light: u32, pname: u32, params: *const f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glLightfv(light: u32, pname: u32, params: *const f32);
        }
        ::core::mem::transmute(glLightfv(::core::mem::transmute(light), ::core::mem::transmute(pname), ::core::mem::transmute(params)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glLighti(light: u32, pname: u32, param2: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glLighti(light: u32, pname: u32, param2: i32);
        }
        ::core::mem::transmute(glLighti(::core::mem::transmute(light), ::core::mem::transmute(pname), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glLightiv(light: u32, pname: u32, params: *const i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glLightiv(light: u32, pname: u32, params: *const i32);
        }
        ::core::mem::transmute(glLightiv(::core::mem::transmute(light), ::core::mem::transmute(pname), ::core::mem::transmute(params)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glLineStipple(factor: i32, pattern: u16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glLineStipple(factor: i32, pattern: u16);
        }
        ::core::mem::transmute(glLineStipple(::core::mem::transmute(factor), ::core::mem::transmute(pattern)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glLineWidth(width: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glLineWidth(width: f32);
        }
        ::core::mem::transmute(glLineWidth(::core::mem::transmute(width)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glListBase(base: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glListBase(base: u32);
        }
        ::core::mem::transmute(glListBase(::core::mem::transmute(base)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glLoadIdentity() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glLoadIdentity();
        }
        ::core::mem::transmute(glLoadIdentity())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glLoadMatrixd(m: *const f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glLoadMatrixd(m: *const f64);
        }
        ::core::mem::transmute(glLoadMatrixd(::core::mem::transmute(m)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glLoadMatrixf(m: *const f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glLoadMatrixf(m: *const f32);
        }
        ::core::mem::transmute(glLoadMatrixf(::core::mem::transmute(m)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glLoadName(name: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glLoadName(name: u32);
        }
        ::core::mem::transmute(glLoadName(::core::mem::transmute(name)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glLogicOp(opcode: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glLogicOp(opcode: u32);
        }
        ::core::mem::transmute(glLogicOp(::core::mem::transmute(opcode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glMap1d(target: u32, u1: f64, u2: f64, stride: i32, order: i32, points: *const f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glMap1d(target: u32, u1: f64, u2: f64, stride: i32, order: i32, points: *const f64);
        }
        ::core::mem::transmute(glMap1d(::core::mem::transmute(target), ::core::mem::transmute(u1), ::core::mem::transmute(u2), ::core::mem::transmute(stride), ::core::mem::transmute(order), ::core::mem::transmute(points)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glMap1f(target: u32, u1: f32, u2: f32, stride: i32, order: i32, points: *const f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glMap1f(target: u32, u1: f32, u2: f32, stride: i32, order: i32, points: *const f32);
        }
        ::core::mem::transmute(glMap1f(::core::mem::transmute(target), ::core::mem::transmute(u1), ::core::mem::transmute(u2), ::core::mem::transmute(stride), ::core::mem::transmute(order), ::core::mem::transmute(points)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glMap2d(target: u32, u1: f64, u2: f64, ustride: i32, uorder: i32, v1: f64, v2: f64, vstride: i32, vorder: i32, points: *const f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glMap2d(target: u32, u1: f64, u2: f64, ustride: i32, uorder: i32, v1: f64, v2: f64, vstride: i32, vorder: i32, points: *const f64);
        }
        ::core::mem::transmute(glMap2d(::core::mem::transmute(target), ::core::mem::transmute(u1), ::core::mem::transmute(u2), ::core::mem::transmute(ustride), ::core::mem::transmute(uorder), ::core::mem::transmute(v1), ::core::mem::transmute(v2), ::core::mem::transmute(vstride), ::core::mem::transmute(vorder), ::core::mem::transmute(points)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glMap2f(target: u32, u1: f32, u2: f32, ustride: i32, uorder: i32, v1: f32, v2: f32, vstride: i32, vorder: i32, points: *const f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glMap2f(target: u32, u1: f32, u2: f32, ustride: i32, uorder: i32, v1: f32, v2: f32, vstride: i32, vorder: i32, points: *const f32);
        }
        ::core::mem::transmute(glMap2f(::core::mem::transmute(target), ::core::mem::transmute(u1), ::core::mem::transmute(u2), ::core::mem::transmute(ustride), ::core::mem::transmute(uorder), ::core::mem::transmute(v1), ::core::mem::transmute(v2), ::core::mem::transmute(vstride), ::core::mem::transmute(vorder), ::core::mem::transmute(points)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glMapGrid1d(un: i32, u1: f64, u2: f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glMapGrid1d(un: i32, u1: f64, u2: f64);
        }
        ::core::mem::transmute(glMapGrid1d(::core::mem::transmute(un), ::core::mem::transmute(u1), ::core::mem::transmute(u2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glMapGrid1f(un: i32, u1: f32, u2: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glMapGrid1f(un: i32, u1: f32, u2: f32);
        }
        ::core::mem::transmute(glMapGrid1f(::core::mem::transmute(un), ::core::mem::transmute(u1), ::core::mem::transmute(u2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glMapGrid2d(un: i32, u1: f64, u2: f64, vn: i32, v1: f64, v2: f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glMapGrid2d(un: i32, u1: f64, u2: f64, vn: i32, v1: f64, v2: f64);
        }
        ::core::mem::transmute(glMapGrid2d(::core::mem::transmute(un), ::core::mem::transmute(u1), ::core::mem::transmute(u2), ::core::mem::transmute(vn), ::core::mem::transmute(v1), ::core::mem::transmute(v2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glMapGrid2f(un: i32, u1: f32, u2: f32, vn: i32, v1: f32, v2: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glMapGrid2f(un: i32, u1: f32, u2: f32, vn: i32, v1: f32, v2: f32);
        }
        ::core::mem::transmute(glMapGrid2f(::core::mem::transmute(un), ::core::mem::transmute(u1), ::core::mem::transmute(u2), ::core::mem::transmute(vn), ::core::mem::transmute(v1), ::core::mem::transmute(v2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glMaterialf(face: u32, pname: u32, param2: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glMaterialf(face: u32, pname: u32, param2: f32);
        }
        ::core::mem::transmute(glMaterialf(::core::mem::transmute(face), ::core::mem::transmute(pname), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glMaterialfv(face: u32, pname: u32, params: *const f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glMaterialfv(face: u32, pname: u32, params: *const f32);
        }
        ::core::mem::transmute(glMaterialfv(::core::mem::transmute(face), ::core::mem::transmute(pname), ::core::mem::transmute(params)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glMateriali(face: u32, pname: u32, param2: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glMateriali(face: u32, pname: u32, param2: i32);
        }
        ::core::mem::transmute(glMateriali(::core::mem::transmute(face), ::core::mem::transmute(pname), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glMaterialiv(face: u32, pname: u32, params: *const i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glMaterialiv(face: u32, pname: u32, params: *const i32);
        }
        ::core::mem::transmute(glMaterialiv(::core::mem::transmute(face), ::core::mem::transmute(pname), ::core::mem::transmute(params)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glMatrixMode(mode: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glMatrixMode(mode: u32);
        }
        ::core::mem::transmute(glMatrixMode(::core::mem::transmute(mode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glMultMatrixd(m: *const f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glMultMatrixd(m: *const f64);
        }
        ::core::mem::transmute(glMultMatrixd(::core::mem::transmute(m)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glMultMatrixf(m: *const f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glMultMatrixf(m: *const f32);
        }
        ::core::mem::transmute(glMultMatrixf(::core::mem::transmute(m)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glNewList(list: u32, mode: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glNewList(list: u32, mode: u32);
        }
        ::core::mem::transmute(glNewList(::core::mem::transmute(list), ::core::mem::transmute(mode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glNormal3b(nx: i8, ny: i8, nz: i8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glNormal3b(nx: i8, ny: i8, nz: i8);
        }
        ::core::mem::transmute(glNormal3b(::core::mem::transmute(nx), ::core::mem::transmute(ny), ::core::mem::transmute(nz)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glNormal3bv(v: *const i8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glNormal3bv(v: *const i8);
        }
        ::core::mem::transmute(glNormal3bv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glNormal3d(nx: f64, ny: f64, nz: f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glNormal3d(nx: f64, ny: f64, nz: f64);
        }
        ::core::mem::transmute(glNormal3d(::core::mem::transmute(nx), ::core::mem::transmute(ny), ::core::mem::transmute(nz)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glNormal3dv(v: *const f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glNormal3dv(v: *const f64);
        }
        ::core::mem::transmute(glNormal3dv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glNormal3f(nx: f32, ny: f32, nz: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glNormal3f(nx: f32, ny: f32, nz: f32);
        }
        ::core::mem::transmute(glNormal3f(::core::mem::transmute(nx), ::core::mem::transmute(ny), ::core::mem::transmute(nz)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glNormal3fv(v: *const f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glNormal3fv(v: *const f32);
        }
        ::core::mem::transmute(glNormal3fv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glNormal3i(nx: i32, ny: i32, nz: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glNormal3i(nx: i32, ny: i32, nz: i32);
        }
        ::core::mem::transmute(glNormal3i(::core::mem::transmute(nx), ::core::mem::transmute(ny), ::core::mem::transmute(nz)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glNormal3iv(v: *const i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glNormal3iv(v: *const i32);
        }
        ::core::mem::transmute(glNormal3iv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glNormal3s(nx: i16, ny: i16, nz: i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glNormal3s(nx: i16, ny: i16, nz: i16);
        }
        ::core::mem::transmute(glNormal3s(::core::mem::transmute(nx), ::core::mem::transmute(ny), ::core::mem::transmute(nz)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glNormal3sv(v: *const i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glNormal3sv(v: *const i16);
        }
        ::core::mem::transmute(glNormal3sv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glNormalPointer(r#type: u32, stride: i32, pointer: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glNormalPointer(r#type: u32, stride: i32, pointer: *const ::core::ffi::c_void);
        }
        ::core::mem::transmute(glNormalPointer(::core::mem::transmute(r#type), ::core::mem::transmute(stride), ::core::mem::transmute(pointer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glOrtho(left: f64, right: f64, bottom: f64, top: f64, znear: f64, zfar: f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glOrtho(left: f64, right: f64, bottom: f64, top: f64, znear: f64, zfar: f64);
        }
        ::core::mem::transmute(glOrtho(::core::mem::transmute(left), ::core::mem::transmute(right), ::core::mem::transmute(bottom), ::core::mem::transmute(top), ::core::mem::transmute(znear), ::core::mem::transmute(zfar)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glPassThrough(token: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glPassThrough(token: f32);
        }
        ::core::mem::transmute(glPassThrough(::core::mem::transmute(token)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glPixelMapfv(map: u32, mapsize: i32, values: *const f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glPixelMapfv(map: u32, mapsize: i32, values: *const f32);
        }
        ::core::mem::transmute(glPixelMapfv(::core::mem::transmute(map), ::core::mem::transmute(mapsize), ::core::mem::transmute(values)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glPixelMapuiv(map: u32, mapsize: i32, values: *const u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glPixelMapuiv(map: u32, mapsize: i32, values: *const u32);
        }
        ::core::mem::transmute(glPixelMapuiv(::core::mem::transmute(map), ::core::mem::transmute(mapsize), ::core::mem::transmute(values)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glPixelMapusv(map: u32, mapsize: i32, values: *const u16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glPixelMapusv(map: u32, mapsize: i32, values: *const u16);
        }
        ::core::mem::transmute(glPixelMapusv(::core::mem::transmute(map), ::core::mem::transmute(mapsize), ::core::mem::transmute(values)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glPixelStoref(pname: u32, param1: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glPixelStoref(pname: u32, param1: f32);
        }
        ::core::mem::transmute(glPixelStoref(::core::mem::transmute(pname), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glPixelStorei(pname: u32, param1: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glPixelStorei(pname: u32, param1: i32);
        }
        ::core::mem::transmute(glPixelStorei(::core::mem::transmute(pname), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glPixelTransferf(pname: u32, param1: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glPixelTransferf(pname: u32, param1: f32);
        }
        ::core::mem::transmute(glPixelTransferf(::core::mem::transmute(pname), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glPixelTransferi(pname: u32, param1: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glPixelTransferi(pname: u32, param1: i32);
        }
        ::core::mem::transmute(glPixelTransferi(::core::mem::transmute(pname), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glPixelZoom(xfactor: f32, yfactor: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glPixelZoom(xfactor: f32, yfactor: f32);
        }
        ::core::mem::transmute(glPixelZoom(::core::mem::transmute(xfactor), ::core::mem::transmute(yfactor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glPointSize(size: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glPointSize(size: f32);
        }
        ::core::mem::transmute(glPointSize(::core::mem::transmute(size)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glPolygonMode(face: u32, mode: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glPolygonMode(face: u32, mode: u32);
        }
        ::core::mem::transmute(glPolygonMode(::core::mem::transmute(face), ::core::mem::transmute(mode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glPolygonOffset(factor: f32, units: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glPolygonOffset(factor: f32, units: f32);
        }
        ::core::mem::transmute(glPolygonOffset(::core::mem::transmute(factor), ::core::mem::transmute(units)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glPolygonStipple(mask: *const u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glPolygonStipple(mask: *const u8);
        }
        ::core::mem::transmute(glPolygonStipple(::core::mem::transmute(mask)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glPopAttrib() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glPopAttrib();
        }
        ::core::mem::transmute(glPopAttrib())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glPopClientAttrib() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glPopClientAttrib();
        }
        ::core::mem::transmute(glPopClientAttrib())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glPopMatrix() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glPopMatrix();
        }
        ::core::mem::transmute(glPopMatrix())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glPopName() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glPopName();
        }
        ::core::mem::transmute(glPopName())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glPrioritizeTextures(n: i32, textures: *const u32, priorities: *const f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glPrioritizeTextures(n: i32, textures: *const u32, priorities: *const f32);
        }
        ::core::mem::transmute(glPrioritizeTextures(::core::mem::transmute(n), ::core::mem::transmute(textures), ::core::mem::transmute(priorities)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glPushAttrib(mask: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glPushAttrib(mask: u32);
        }
        ::core::mem::transmute(glPushAttrib(::core::mem::transmute(mask)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glPushClientAttrib(mask: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glPushClientAttrib(mask: u32);
        }
        ::core::mem::transmute(glPushClientAttrib(::core::mem::transmute(mask)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glPushMatrix() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glPushMatrix();
        }
        ::core::mem::transmute(glPushMatrix())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glPushName(name: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glPushName(name: u32);
        }
        ::core::mem::transmute(glPushName(::core::mem::transmute(name)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glRasterPos2d(x: f64, y: f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glRasterPos2d(x: f64, y: f64);
        }
        ::core::mem::transmute(glRasterPos2d(::core::mem::transmute(x), ::core::mem::transmute(y)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glRasterPos2dv(v: *const f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glRasterPos2dv(v: *const f64);
        }
        ::core::mem::transmute(glRasterPos2dv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glRasterPos2f(x: f32, y: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glRasterPos2f(x: f32, y: f32);
        }
        ::core::mem::transmute(glRasterPos2f(::core::mem::transmute(x), ::core::mem::transmute(y)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glRasterPos2fv(v: *const f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glRasterPos2fv(v: *const f32);
        }
        ::core::mem::transmute(glRasterPos2fv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glRasterPos2i(x: i32, y: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glRasterPos2i(x: i32, y: i32);
        }
        ::core::mem::transmute(glRasterPos2i(::core::mem::transmute(x), ::core::mem::transmute(y)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glRasterPos2iv(v: *const i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glRasterPos2iv(v: *const i32);
        }
        ::core::mem::transmute(glRasterPos2iv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glRasterPos2s(x: i16, y: i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glRasterPos2s(x: i16, y: i16);
        }
        ::core::mem::transmute(glRasterPos2s(::core::mem::transmute(x), ::core::mem::transmute(y)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glRasterPos2sv(v: *const i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glRasterPos2sv(v: *const i16);
        }
        ::core::mem::transmute(glRasterPos2sv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glRasterPos3d(x: f64, y: f64, z: f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glRasterPos3d(x: f64, y: f64, z: f64);
        }
        ::core::mem::transmute(glRasterPos3d(::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glRasterPos3dv(v: *const f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glRasterPos3dv(v: *const f64);
        }
        ::core::mem::transmute(glRasterPos3dv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glRasterPos3f(x: f32, y: f32, z: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glRasterPos3f(x: f32, y: f32, z: f32);
        }
        ::core::mem::transmute(glRasterPos3f(::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glRasterPos3fv(v: *const f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glRasterPos3fv(v: *const f32);
        }
        ::core::mem::transmute(glRasterPos3fv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glRasterPos3i(x: i32, y: i32, z: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glRasterPos3i(x: i32, y: i32, z: i32);
        }
        ::core::mem::transmute(glRasterPos3i(::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glRasterPos3iv(v: *const i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glRasterPos3iv(v: *const i32);
        }
        ::core::mem::transmute(glRasterPos3iv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glRasterPos3s(x: i16, y: i16, z: i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glRasterPos3s(x: i16, y: i16, z: i16);
        }
        ::core::mem::transmute(glRasterPos3s(::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glRasterPos3sv(v: *const i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glRasterPos3sv(v: *const i16);
        }
        ::core::mem::transmute(glRasterPos3sv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glRasterPos4d(x: f64, y: f64, z: f64, w: f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glRasterPos4d(x: f64, y: f64, z: f64, w: f64);
        }
        ::core::mem::transmute(glRasterPos4d(::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z), ::core::mem::transmute(w)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glRasterPos4dv(v: *const f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glRasterPos4dv(v: *const f64);
        }
        ::core::mem::transmute(glRasterPos4dv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glRasterPos4f(x: f32, y: f32, z: f32, w: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glRasterPos4f(x: f32, y: f32, z: f32, w: f32);
        }
        ::core::mem::transmute(glRasterPos4f(::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z), ::core::mem::transmute(w)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glRasterPos4fv(v: *const f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glRasterPos4fv(v: *const f32);
        }
        ::core::mem::transmute(glRasterPos4fv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glRasterPos4i(x: i32, y: i32, z: i32, w: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glRasterPos4i(x: i32, y: i32, z: i32, w: i32);
        }
        ::core::mem::transmute(glRasterPos4i(::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z), ::core::mem::transmute(w)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glRasterPos4iv(v: *const i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glRasterPos4iv(v: *const i32);
        }
        ::core::mem::transmute(glRasterPos4iv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glRasterPos4s(x: i16, y: i16, z: i16, w: i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glRasterPos4s(x: i16, y: i16, z: i16, w: i16);
        }
        ::core::mem::transmute(glRasterPos4s(::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z), ::core::mem::transmute(w)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glRasterPos4sv(v: *const i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glRasterPos4sv(v: *const i16);
        }
        ::core::mem::transmute(glRasterPos4sv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glReadBuffer(mode: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glReadBuffer(mode: u32);
        }
        ::core::mem::transmute(glReadBuffer(::core::mem::transmute(mode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glReadPixels(x: i32, y: i32, width: i32, height: i32, format: u32, r#type: u32, pixels: *mut ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glReadPixels(x: i32, y: i32, width: i32, height: i32, format: u32, r#type: u32, pixels: *mut ::core::ffi::c_void);
        }
        ::core::mem::transmute(glReadPixels(::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(format), ::core::mem::transmute(r#type), ::core::mem::transmute(pixels)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glRectd(x1: f64, y1: f64, x2: f64, y2: f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glRectd(x1: f64, y1: f64, x2: f64, y2: f64);
        }
        ::core::mem::transmute(glRectd(::core::mem::transmute(x1), ::core::mem::transmute(y1), ::core::mem::transmute(x2), ::core::mem::transmute(y2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glRectdv(v1: *const f64, v2: *const f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glRectdv(v1: *const f64, v2: *const f64);
        }
        ::core::mem::transmute(glRectdv(::core::mem::transmute(v1), ::core::mem::transmute(v2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glRectf(x1: f32, y1: f32, x2: f32, y2: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glRectf(x1: f32, y1: f32, x2: f32, y2: f32);
        }
        ::core::mem::transmute(glRectf(::core::mem::transmute(x1), ::core::mem::transmute(y1), ::core::mem::transmute(x2), ::core::mem::transmute(y2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glRectfv(v1: *const f32, v2: *const f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glRectfv(v1: *const f32, v2: *const f32);
        }
        ::core::mem::transmute(glRectfv(::core::mem::transmute(v1), ::core::mem::transmute(v2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glRecti(x1: i32, y1: i32, x2: i32, y2: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glRecti(x1: i32, y1: i32, x2: i32, y2: i32);
        }
        ::core::mem::transmute(glRecti(::core::mem::transmute(x1), ::core::mem::transmute(y1), ::core::mem::transmute(x2), ::core::mem::transmute(y2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glRectiv(v1: *const i32, v2: *const i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glRectiv(v1: *const i32, v2: *const i32);
        }
        ::core::mem::transmute(glRectiv(::core::mem::transmute(v1), ::core::mem::transmute(v2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glRects(x1: i16, y1: i16, x2: i16, y2: i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glRects(x1: i16, y1: i16, x2: i16, y2: i16);
        }
        ::core::mem::transmute(glRects(::core::mem::transmute(x1), ::core::mem::transmute(y1), ::core::mem::transmute(x2), ::core::mem::transmute(y2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glRectsv(v1: *const i16, v2: *const i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glRectsv(v1: *const i16, v2: *const i16);
        }
        ::core::mem::transmute(glRectsv(::core::mem::transmute(v1), ::core::mem::transmute(v2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glRenderMode(mode: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glRenderMode(mode: u32) -> i32;
        }
        ::core::mem::transmute(glRenderMode(::core::mem::transmute(mode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glRotated(angle: f64, x: f64, y: f64, z: f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glRotated(angle: f64, x: f64, y: f64, z: f64);
        }
        ::core::mem::transmute(glRotated(::core::mem::transmute(angle), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glRotatef(angle: f32, x: f32, y: f32, z: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glRotatef(angle: f32, x: f32, y: f32, z: f32);
        }
        ::core::mem::transmute(glRotatef(::core::mem::transmute(angle), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glScaled(x: f64, y: f64, z: f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glScaled(x: f64, y: f64, z: f64);
        }
        ::core::mem::transmute(glScaled(::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glScalef(x: f32, y: f32, z: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glScalef(x: f32, y: f32, z: f32);
        }
        ::core::mem::transmute(glScalef(::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glScissor(x: i32, y: i32, width: i32, height: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glScissor(x: i32, y: i32, width: i32, height: i32);
        }
        ::core::mem::transmute(glScissor(::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(width), ::core::mem::transmute(height)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glSelectBuffer(size: i32, buffer: *mut u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glSelectBuffer(size: i32, buffer: *mut u32);
        }
        ::core::mem::transmute(glSelectBuffer(::core::mem::transmute(size), ::core::mem::transmute(buffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glShadeModel(mode: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glShadeModel(mode: u32);
        }
        ::core::mem::transmute(glShadeModel(::core::mem::transmute(mode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glStencilFunc(func: u32, r#ref: i32, mask: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glStencilFunc(func: u32, r#ref: i32, mask: u32);
        }
        ::core::mem::transmute(glStencilFunc(::core::mem::transmute(func), ::core::mem::transmute(r#ref), ::core::mem::transmute(mask)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glStencilMask(mask: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glStencilMask(mask: u32);
        }
        ::core::mem::transmute(glStencilMask(::core::mem::transmute(mask)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glStencilOp(fail: u32, zfail: u32, zpass: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glStencilOp(fail: u32, zfail: u32, zpass: u32);
        }
        ::core::mem::transmute(glStencilOp(::core::mem::transmute(fail), ::core::mem::transmute(zfail), ::core::mem::transmute(zpass)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexCoord1d(s: f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexCoord1d(s: f64);
        }
        ::core::mem::transmute(glTexCoord1d(::core::mem::transmute(s)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexCoord1dv(v: *const f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexCoord1dv(v: *const f64);
        }
        ::core::mem::transmute(glTexCoord1dv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexCoord1f(s: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexCoord1f(s: f32);
        }
        ::core::mem::transmute(glTexCoord1f(::core::mem::transmute(s)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexCoord1fv(v: *const f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexCoord1fv(v: *const f32);
        }
        ::core::mem::transmute(glTexCoord1fv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexCoord1i(s: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexCoord1i(s: i32);
        }
        ::core::mem::transmute(glTexCoord1i(::core::mem::transmute(s)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexCoord1iv(v: *const i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexCoord1iv(v: *const i32);
        }
        ::core::mem::transmute(glTexCoord1iv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexCoord1s(s: i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexCoord1s(s: i16);
        }
        ::core::mem::transmute(glTexCoord1s(::core::mem::transmute(s)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexCoord1sv(v: *const i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexCoord1sv(v: *const i16);
        }
        ::core::mem::transmute(glTexCoord1sv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexCoord2d(s: f64, t: f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexCoord2d(s: f64, t: f64);
        }
        ::core::mem::transmute(glTexCoord2d(::core::mem::transmute(s), ::core::mem::transmute(t)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexCoord2dv(v: *const f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexCoord2dv(v: *const f64);
        }
        ::core::mem::transmute(glTexCoord2dv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexCoord2f(s: f32, t: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexCoord2f(s: f32, t: f32);
        }
        ::core::mem::transmute(glTexCoord2f(::core::mem::transmute(s), ::core::mem::transmute(t)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexCoord2fv(v: *const f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexCoord2fv(v: *const f32);
        }
        ::core::mem::transmute(glTexCoord2fv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexCoord2i(s: i32, t: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexCoord2i(s: i32, t: i32);
        }
        ::core::mem::transmute(glTexCoord2i(::core::mem::transmute(s), ::core::mem::transmute(t)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexCoord2iv(v: *const i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexCoord2iv(v: *const i32);
        }
        ::core::mem::transmute(glTexCoord2iv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexCoord2s(s: i16, t: i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexCoord2s(s: i16, t: i16);
        }
        ::core::mem::transmute(glTexCoord2s(::core::mem::transmute(s), ::core::mem::transmute(t)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexCoord2sv(v: *const i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexCoord2sv(v: *const i16);
        }
        ::core::mem::transmute(glTexCoord2sv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexCoord3d(s: f64, t: f64, r: f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexCoord3d(s: f64, t: f64, r: f64);
        }
        ::core::mem::transmute(glTexCoord3d(::core::mem::transmute(s), ::core::mem::transmute(t), ::core::mem::transmute(r)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexCoord3dv(v: *const f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexCoord3dv(v: *const f64);
        }
        ::core::mem::transmute(glTexCoord3dv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexCoord3f(s: f32, t: f32, r: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexCoord3f(s: f32, t: f32, r: f32);
        }
        ::core::mem::transmute(glTexCoord3f(::core::mem::transmute(s), ::core::mem::transmute(t), ::core::mem::transmute(r)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexCoord3fv(v: *const f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexCoord3fv(v: *const f32);
        }
        ::core::mem::transmute(glTexCoord3fv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexCoord3i(s: i32, t: i32, r: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexCoord3i(s: i32, t: i32, r: i32);
        }
        ::core::mem::transmute(glTexCoord3i(::core::mem::transmute(s), ::core::mem::transmute(t), ::core::mem::transmute(r)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexCoord3iv(v: *const i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexCoord3iv(v: *const i32);
        }
        ::core::mem::transmute(glTexCoord3iv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexCoord3s(s: i16, t: i16, r: i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexCoord3s(s: i16, t: i16, r: i16);
        }
        ::core::mem::transmute(glTexCoord3s(::core::mem::transmute(s), ::core::mem::transmute(t), ::core::mem::transmute(r)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexCoord3sv(v: *const i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexCoord3sv(v: *const i16);
        }
        ::core::mem::transmute(glTexCoord3sv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexCoord4d(s: f64, t: f64, r: f64, q: f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexCoord4d(s: f64, t: f64, r: f64, q: f64);
        }
        ::core::mem::transmute(glTexCoord4d(::core::mem::transmute(s), ::core::mem::transmute(t), ::core::mem::transmute(r), ::core::mem::transmute(q)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexCoord4dv(v: *const f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexCoord4dv(v: *const f64);
        }
        ::core::mem::transmute(glTexCoord4dv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexCoord4f(s: f32, t: f32, r: f32, q: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexCoord4f(s: f32, t: f32, r: f32, q: f32);
        }
        ::core::mem::transmute(glTexCoord4f(::core::mem::transmute(s), ::core::mem::transmute(t), ::core::mem::transmute(r), ::core::mem::transmute(q)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexCoord4fv(v: *const f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexCoord4fv(v: *const f32);
        }
        ::core::mem::transmute(glTexCoord4fv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexCoord4i(s: i32, t: i32, r: i32, q: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexCoord4i(s: i32, t: i32, r: i32, q: i32);
        }
        ::core::mem::transmute(glTexCoord4i(::core::mem::transmute(s), ::core::mem::transmute(t), ::core::mem::transmute(r), ::core::mem::transmute(q)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexCoord4iv(v: *const i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexCoord4iv(v: *const i32);
        }
        ::core::mem::transmute(glTexCoord4iv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexCoord4s(s: i16, t: i16, r: i16, q: i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexCoord4s(s: i16, t: i16, r: i16, q: i16);
        }
        ::core::mem::transmute(glTexCoord4s(::core::mem::transmute(s), ::core::mem::transmute(t), ::core::mem::transmute(r), ::core::mem::transmute(q)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexCoord4sv(v: *const i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexCoord4sv(v: *const i16);
        }
        ::core::mem::transmute(glTexCoord4sv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexCoordPointer(size: i32, r#type: u32, stride: i32, pointer: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexCoordPointer(size: i32, r#type: u32, stride: i32, pointer: *const ::core::ffi::c_void);
        }
        ::core::mem::transmute(glTexCoordPointer(::core::mem::transmute(size), ::core::mem::transmute(r#type), ::core::mem::transmute(stride), ::core::mem::transmute(pointer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexEnvf(target: u32, pname: u32, param2: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexEnvf(target: u32, pname: u32, param2: f32);
        }
        ::core::mem::transmute(glTexEnvf(::core::mem::transmute(target), ::core::mem::transmute(pname), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexEnvfv(target: u32, pname: u32, params: *const f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexEnvfv(target: u32, pname: u32, params: *const f32);
        }
        ::core::mem::transmute(glTexEnvfv(::core::mem::transmute(target), ::core::mem::transmute(pname), ::core::mem::transmute(params)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexEnvi(target: u32, pname: u32, param2: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexEnvi(target: u32, pname: u32, param2: i32);
        }
        ::core::mem::transmute(glTexEnvi(::core::mem::transmute(target), ::core::mem::transmute(pname), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexEnviv(target: u32, pname: u32, params: *const i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexEnviv(target: u32, pname: u32, params: *const i32);
        }
        ::core::mem::transmute(glTexEnviv(::core::mem::transmute(target), ::core::mem::transmute(pname), ::core::mem::transmute(params)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexGend(coord: u32, pname: u32, param2: f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexGend(coord: u32, pname: u32, param2: f64);
        }
        ::core::mem::transmute(glTexGend(::core::mem::transmute(coord), ::core::mem::transmute(pname), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexGendv(coord: u32, pname: u32, params: *const f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexGendv(coord: u32, pname: u32, params: *const f64);
        }
        ::core::mem::transmute(glTexGendv(::core::mem::transmute(coord), ::core::mem::transmute(pname), ::core::mem::transmute(params)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexGenf(coord: u32, pname: u32, param2: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexGenf(coord: u32, pname: u32, param2: f32);
        }
        ::core::mem::transmute(glTexGenf(::core::mem::transmute(coord), ::core::mem::transmute(pname), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexGenfv(coord: u32, pname: u32, params: *const f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexGenfv(coord: u32, pname: u32, params: *const f32);
        }
        ::core::mem::transmute(glTexGenfv(::core::mem::transmute(coord), ::core::mem::transmute(pname), ::core::mem::transmute(params)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexGeni(coord: u32, pname: u32, param2: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexGeni(coord: u32, pname: u32, param2: i32);
        }
        ::core::mem::transmute(glTexGeni(::core::mem::transmute(coord), ::core::mem::transmute(pname), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexGeniv(coord: u32, pname: u32, params: *const i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexGeniv(coord: u32, pname: u32, params: *const i32);
        }
        ::core::mem::transmute(glTexGeniv(::core::mem::transmute(coord), ::core::mem::transmute(pname), ::core::mem::transmute(params)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexImage1D(target: u32, level: i32, internalformat: i32, width: i32, border: i32, format: u32, r#type: u32, pixels: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexImage1D(target: u32, level: i32, internalformat: i32, width: i32, border: i32, format: u32, r#type: u32, pixels: *const ::core::ffi::c_void);
        }
        ::core::mem::transmute(glTexImage1D(::core::mem::transmute(target), ::core::mem::transmute(level), ::core::mem::transmute(internalformat), ::core::mem::transmute(width), ::core::mem::transmute(border), ::core::mem::transmute(format), ::core::mem::transmute(r#type), ::core::mem::transmute(pixels)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexImage2D(target: u32, level: i32, internalformat: i32, width: i32, height: i32, border: i32, format: u32, r#type: u32, pixels: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexImage2D(target: u32, level: i32, internalformat: i32, width: i32, height: i32, border: i32, format: u32, r#type: u32, pixels: *const ::core::ffi::c_void);
        }
        ::core::mem::transmute(glTexImage2D(::core::mem::transmute(target), ::core::mem::transmute(level), ::core::mem::transmute(internalformat), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(border), ::core::mem::transmute(format), ::core::mem::transmute(r#type), ::core::mem::transmute(pixels)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexParameterf(target: u32, pname: u32, param2: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexParameterf(target: u32, pname: u32, param2: f32);
        }
        ::core::mem::transmute(glTexParameterf(::core::mem::transmute(target), ::core::mem::transmute(pname), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexParameterfv(target: u32, pname: u32, params: *const f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexParameterfv(target: u32, pname: u32, params: *const f32);
        }
        ::core::mem::transmute(glTexParameterfv(::core::mem::transmute(target), ::core::mem::transmute(pname), ::core::mem::transmute(params)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexParameteri(target: u32, pname: u32, param2: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexParameteri(target: u32, pname: u32, param2: i32);
        }
        ::core::mem::transmute(glTexParameteri(::core::mem::transmute(target), ::core::mem::transmute(pname), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexParameteriv(target: u32, pname: u32, params: *const i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexParameteriv(target: u32, pname: u32, params: *const i32);
        }
        ::core::mem::transmute(glTexParameteriv(::core::mem::transmute(target), ::core::mem::transmute(pname), ::core::mem::transmute(params)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexSubImage1D(target: u32, level: i32, xoffset: i32, width: i32, format: u32, r#type: u32, pixels: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexSubImage1D(target: u32, level: i32, xoffset: i32, width: i32, format: u32, r#type: u32, pixels: *const ::core::ffi::c_void);
        }
        ::core::mem::transmute(glTexSubImage1D(::core::mem::transmute(target), ::core::mem::transmute(level), ::core::mem::transmute(xoffset), ::core::mem::transmute(width), ::core::mem::transmute(format), ::core::mem::transmute(r#type), ::core::mem::transmute(pixels)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTexSubImage2D(target: u32, level: i32, xoffset: i32, yoffset: i32, width: i32, height: i32, format: u32, r#type: u32, pixels: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTexSubImage2D(target: u32, level: i32, xoffset: i32, yoffset: i32, width: i32, height: i32, format: u32, r#type: u32, pixels: *const ::core::ffi::c_void);
        }
        ::core::mem::transmute(glTexSubImage2D(::core::mem::transmute(target), ::core::mem::transmute(level), ::core::mem::transmute(xoffset), ::core::mem::transmute(yoffset), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(format), ::core::mem::transmute(r#type), ::core::mem::transmute(pixels)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTranslated(x: f64, y: f64, z: f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTranslated(x: f64, y: f64, z: f64);
        }
        ::core::mem::transmute(glTranslated(::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glTranslatef(x: f32, y: f32, z: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glTranslatef(x: f32, y: f32, z: f32);
        }
        ::core::mem::transmute(glTranslatef(::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glVertex2d(x: f64, y: f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glVertex2d(x: f64, y: f64);
        }
        ::core::mem::transmute(glVertex2d(::core::mem::transmute(x), ::core::mem::transmute(y)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glVertex2dv(v: *const f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glVertex2dv(v: *const f64);
        }
        ::core::mem::transmute(glVertex2dv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glVertex2f(x: f32, y: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glVertex2f(x: f32, y: f32);
        }
        ::core::mem::transmute(glVertex2f(::core::mem::transmute(x), ::core::mem::transmute(y)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glVertex2fv(v: *const f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glVertex2fv(v: *const f32);
        }
        ::core::mem::transmute(glVertex2fv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glVertex2i(x: i32, y: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glVertex2i(x: i32, y: i32);
        }
        ::core::mem::transmute(glVertex2i(::core::mem::transmute(x), ::core::mem::transmute(y)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glVertex2iv(v: *const i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glVertex2iv(v: *const i32);
        }
        ::core::mem::transmute(glVertex2iv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glVertex2s(x: i16, y: i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glVertex2s(x: i16, y: i16);
        }
        ::core::mem::transmute(glVertex2s(::core::mem::transmute(x), ::core::mem::transmute(y)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glVertex2sv(v: *const i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glVertex2sv(v: *const i16);
        }
        ::core::mem::transmute(glVertex2sv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glVertex3d(x: f64, y: f64, z: f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glVertex3d(x: f64, y: f64, z: f64);
        }
        ::core::mem::transmute(glVertex3d(::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glVertex3dv(v: *const f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glVertex3dv(v: *const f64);
        }
        ::core::mem::transmute(glVertex3dv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glVertex3f(x: f32, y: f32, z: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glVertex3f(x: f32, y: f32, z: f32);
        }
        ::core::mem::transmute(glVertex3f(::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glVertex3fv(v: *const f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glVertex3fv(v: *const f32);
        }
        ::core::mem::transmute(glVertex3fv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glVertex3i(x: i32, y: i32, z: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glVertex3i(x: i32, y: i32, z: i32);
        }
        ::core::mem::transmute(glVertex3i(::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glVertex3iv(v: *const i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glVertex3iv(v: *const i32);
        }
        ::core::mem::transmute(glVertex3iv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glVertex3s(x: i16, y: i16, z: i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glVertex3s(x: i16, y: i16, z: i16);
        }
        ::core::mem::transmute(glVertex3s(::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glVertex3sv(v: *const i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glVertex3sv(v: *const i16);
        }
        ::core::mem::transmute(glVertex3sv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glVertex4d(x: f64, y: f64, z: f64, w: f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glVertex4d(x: f64, y: f64, z: f64, w: f64);
        }
        ::core::mem::transmute(glVertex4d(::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z), ::core::mem::transmute(w)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glVertex4dv(v: *const f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glVertex4dv(v: *const f64);
        }
        ::core::mem::transmute(glVertex4dv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glVertex4f(x: f32, y: f32, z: f32, w: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glVertex4f(x: f32, y: f32, z: f32, w: f32);
        }
        ::core::mem::transmute(glVertex4f(::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z), ::core::mem::transmute(w)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glVertex4fv(v: *const f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glVertex4fv(v: *const f32);
        }
        ::core::mem::transmute(glVertex4fv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glVertex4i(x: i32, y: i32, z: i32, w: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glVertex4i(x: i32, y: i32, z: i32, w: i32);
        }
        ::core::mem::transmute(glVertex4i(::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z), ::core::mem::transmute(w)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glVertex4iv(v: *const i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glVertex4iv(v: *const i32);
        }
        ::core::mem::transmute(glVertex4iv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glVertex4s(x: i16, y: i16, z: i16, w: i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glVertex4s(x: i16, y: i16, z: i16, w: i16);
        }
        ::core::mem::transmute(glVertex4s(::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z), ::core::mem::transmute(w)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glVertex4sv(v: *const i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glVertex4sv(v: *const i16);
        }
        ::core::mem::transmute(glVertex4sv(::core::mem::transmute(v)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glVertexPointer(size: i32, r#type: u32, stride: i32, pointer: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glVertexPointer(size: i32, r#type: u32, stride: i32, pointer: *const ::core::ffi::c_void);
        }
        ::core::mem::transmute(glVertexPointer(::core::mem::transmute(size), ::core::mem::transmute(r#type), ::core::mem::transmute(stride), ::core::mem::transmute(pointer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn glViewport(x: i32, y: i32, width: i32, height: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn glViewport(x: i32, y: i32, width: i32, height: i32);
        }
        ::core::mem::transmute(glViewport(::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(width), ::core::mem::transmute(height)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluBeginCurve(nobj: *mut GLUnurbs) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluBeginCurve(nobj: *mut GLUnurbs);
        }
        ::core::mem::transmute(gluBeginCurve(::core::mem::transmute(nobj)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluBeginPolygon(tess: *mut GLUtesselator) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluBeginPolygon(tess: *mut GLUtesselator);
        }
        ::core::mem::transmute(gluBeginPolygon(::core::mem::transmute(tess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluBeginSurface(nobj: *mut GLUnurbs) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluBeginSurface(nobj: *mut GLUnurbs);
        }
        ::core::mem::transmute(gluBeginSurface(::core::mem::transmute(nobj)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluBeginTrim(nobj: *mut GLUnurbs) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluBeginTrim(nobj: *mut GLUnurbs);
        }
        ::core::mem::transmute(gluBeginTrim(::core::mem::transmute(nobj)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluBuild1DMipmaps(target: u32, components: i32, width: i32, format: u32, r#type: u32, data: *const ::core::ffi::c_void) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluBuild1DMipmaps(target: u32, components: i32, width: i32, format: u32, r#type: u32, data: *const ::core::ffi::c_void) -> i32;
        }
        ::core::mem::transmute(gluBuild1DMipmaps(::core::mem::transmute(target), ::core::mem::transmute(components), ::core::mem::transmute(width), ::core::mem::transmute(format), ::core::mem::transmute(r#type), ::core::mem::transmute(data)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluBuild2DMipmaps(target: u32, components: i32, width: i32, height: i32, format: u32, r#type: u32, data: *const ::core::ffi::c_void) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluBuild2DMipmaps(target: u32, components: i32, width: i32, height: i32, format: u32, r#type: u32, data: *const ::core::ffi::c_void) -> i32;
        }
        ::core::mem::transmute(gluBuild2DMipmaps(::core::mem::transmute(target), ::core::mem::transmute(components), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(format), ::core::mem::transmute(r#type), ::core::mem::transmute(data)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluCylinder(qobj: *mut GLUquadric, baseradius: f64, topradius: f64, height: f64, slices: i32, stacks: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluCylinder(qobj: *mut GLUquadric, baseradius: f64, topradius: f64, height: f64, slices: i32, stacks: i32);
        }
        ::core::mem::transmute(gluCylinder(::core::mem::transmute(qobj), ::core::mem::transmute(baseradius), ::core::mem::transmute(topradius), ::core::mem::transmute(height), ::core::mem::transmute(slices), ::core::mem::transmute(stacks)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluDeleteNurbsRenderer(nobj: *mut GLUnurbs) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluDeleteNurbsRenderer(nobj: *mut GLUnurbs);
        }
        ::core::mem::transmute(gluDeleteNurbsRenderer(::core::mem::transmute(nobj)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluDeleteQuadric(state: *mut GLUquadric) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluDeleteQuadric(state: *mut GLUquadric);
        }
        ::core::mem::transmute(gluDeleteQuadric(::core::mem::transmute(state)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluDeleteTess(tess: *mut GLUtesselator) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluDeleteTess(tess: *mut GLUtesselator);
        }
        ::core::mem::transmute(gluDeleteTess(::core::mem::transmute(tess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluDisk(qobj: *mut GLUquadric, innerradius: f64, outerradius: f64, slices: i32, loops: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluDisk(qobj: *mut GLUquadric, innerradius: f64, outerradius: f64, slices: i32, loops: i32);
        }
        ::core::mem::transmute(gluDisk(::core::mem::transmute(qobj), ::core::mem::transmute(innerradius), ::core::mem::transmute(outerradius), ::core::mem::transmute(slices), ::core::mem::transmute(loops)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluEndCurve(nobj: *mut GLUnurbs) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluEndCurve(nobj: *mut GLUnurbs);
        }
        ::core::mem::transmute(gluEndCurve(::core::mem::transmute(nobj)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluEndPolygon(tess: *mut GLUtesselator) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluEndPolygon(tess: *mut GLUtesselator);
        }
        ::core::mem::transmute(gluEndPolygon(::core::mem::transmute(tess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluEndSurface(nobj: *mut GLUnurbs) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluEndSurface(nobj: *mut GLUnurbs);
        }
        ::core::mem::transmute(gluEndSurface(::core::mem::transmute(nobj)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluEndTrim(nobj: *mut GLUnurbs) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluEndTrim(nobj: *mut GLUnurbs);
        }
        ::core::mem::transmute(gluEndTrim(::core::mem::transmute(nobj)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluErrorString(errcode: u32) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluErrorString(errcode: u32) -> *mut u8;
        }
        ::core::mem::transmute(gluErrorString(::core::mem::transmute(errcode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn gluErrorUnicodeStringEXT(errcode: u32) -> super::super::Foundation::PWSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluErrorUnicodeStringEXT(errcode: u32) -> super::super::Foundation::PWSTR;
        }
        ::core::mem::transmute(gluErrorUnicodeStringEXT(::core::mem::transmute(errcode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluGetNurbsProperty(nobj: *mut GLUnurbs, property: u32, value: *mut f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluGetNurbsProperty(nobj: *mut GLUnurbs, property: u32, value: *mut f32);
        }
        ::core::mem::transmute(gluGetNurbsProperty(::core::mem::transmute(nobj), ::core::mem::transmute(property), ::core::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluGetString(name: u32) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluGetString(name: u32) -> *mut u8;
        }
        ::core::mem::transmute(gluGetString(::core::mem::transmute(name)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluGetTessProperty(tess: *mut GLUtesselator, which: u32, value: *mut f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluGetTessProperty(tess: *mut GLUtesselator, which: u32, value: *mut f64);
        }
        ::core::mem::transmute(gluGetTessProperty(::core::mem::transmute(tess), ::core::mem::transmute(which), ::core::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluLoadSamplingMatrices(nobj: *mut GLUnurbs, modelmatrix: *const f32, projmatrix: *const f32, viewport: *const i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluLoadSamplingMatrices(nobj: *mut GLUnurbs, modelmatrix: *const f32, projmatrix: *const f32, viewport: *const i32);
        }
        ::core::mem::transmute(gluLoadSamplingMatrices(::core::mem::transmute(nobj), ::core::mem::transmute(modelmatrix), ::core::mem::transmute(projmatrix), ::core::mem::transmute(viewport)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluLookAt(eyex: f64, eyey: f64, eyez: f64, centerx: f64, centery: f64, centerz: f64, upx: f64, upy: f64, upz: f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluLookAt(eyex: f64, eyey: f64, eyez: f64, centerx: f64, centery: f64, centerz: f64, upx: f64, upy: f64, upz: f64);
        }
        ::core::mem::transmute(gluLookAt(::core::mem::transmute(eyex), ::core::mem::transmute(eyey), ::core::mem::transmute(eyez), ::core::mem::transmute(centerx), ::core::mem::transmute(centery), ::core::mem::transmute(centerz), ::core::mem::transmute(upx), ::core::mem::transmute(upy), ::core::mem::transmute(upz)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluNewNurbsRenderer() -> *mut GLUnurbs {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluNewNurbsRenderer() -> *mut GLUnurbs;
        }
        ::core::mem::transmute(gluNewNurbsRenderer())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluNewQuadric() -> *mut GLUquadric {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluNewQuadric() -> *mut GLUquadric;
        }
        ::core::mem::transmute(gluNewQuadric())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluNewTess() -> *mut GLUtesselator {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluNewTess() -> *mut GLUtesselator;
        }
        ::core::mem::transmute(gluNewTess())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluNextContour(tess: *mut GLUtesselator, r#type: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluNextContour(tess: *mut GLUtesselator, r#type: u32);
        }
        ::core::mem::transmute(gluNextContour(::core::mem::transmute(tess), ::core::mem::transmute(r#type)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluNurbsCallback(nobj: *mut GLUnurbs, which: u32, r#fn: isize) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluNurbsCallback(nobj: *mut GLUnurbs, which: u32, r#fn: isize);
        }
        ::core::mem::transmute(gluNurbsCallback(::core::mem::transmute(nobj), ::core::mem::transmute(which), ::core::mem::transmute(r#fn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluNurbsCurve(nobj: *mut GLUnurbs, nknots: i32, knot: *mut f32, stride: i32, ctlarray: *mut f32, order: i32, r#type: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluNurbsCurve(nobj: *mut GLUnurbs, nknots: i32, knot: *mut f32, stride: i32, ctlarray: *mut f32, order: i32, r#type: u32);
        }
        ::core::mem::transmute(gluNurbsCurve(::core::mem::transmute(nobj), ::core::mem::transmute(nknots), ::core::mem::transmute(knot), ::core::mem::transmute(stride), ::core::mem::transmute(ctlarray), ::core::mem::transmute(order), ::core::mem::transmute(r#type)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluNurbsProperty(nobj: *mut GLUnurbs, property: u32, value: f32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluNurbsProperty(nobj: *mut GLUnurbs, property: u32, value: f32);
        }
        ::core::mem::transmute(gluNurbsProperty(::core::mem::transmute(nobj), ::core::mem::transmute(property), ::core::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluNurbsSurface(nobj: *mut GLUnurbs, sknot_count: i32, sknot: *mut f32, tknot_count: i32, tknot: *mut f32, s_stride: i32, t_stride: i32, ctlarray: *mut f32, sorder: i32, torder: i32, r#type: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluNurbsSurface(nobj: *mut GLUnurbs, sknot_count: i32, sknot: *mut f32, tknot_count: i32, tknot: *mut f32, s_stride: i32, t_stride: i32, ctlarray: *mut f32, sorder: i32, torder: i32, r#type: u32);
        }
        ::core::mem::transmute(gluNurbsSurface(::core::mem::transmute(nobj), ::core::mem::transmute(sknot_count), ::core::mem::transmute(sknot), ::core::mem::transmute(tknot_count), ::core::mem::transmute(tknot), ::core::mem::transmute(s_stride), ::core::mem::transmute(t_stride), ::core::mem::transmute(ctlarray), ::core::mem::transmute(sorder), ::core::mem::transmute(torder), ::core::mem::transmute(r#type)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluOrtho2D(left: f64, right: f64, bottom: f64, top: f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluOrtho2D(left: f64, right: f64, bottom: f64, top: f64);
        }
        ::core::mem::transmute(gluOrtho2D(::core::mem::transmute(left), ::core::mem::transmute(right), ::core::mem::transmute(bottom), ::core::mem::transmute(top)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluPartialDisk(qobj: *mut GLUquadric, innerradius: f64, outerradius: f64, slices: i32, loops: i32, startangle: f64, sweepangle: f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluPartialDisk(qobj: *mut GLUquadric, innerradius: f64, outerradius: f64, slices: i32, loops: i32, startangle: f64, sweepangle: f64);
        }
        ::core::mem::transmute(gluPartialDisk(::core::mem::transmute(qobj), ::core::mem::transmute(innerradius), ::core::mem::transmute(outerradius), ::core::mem::transmute(slices), ::core::mem::transmute(loops), ::core::mem::transmute(startangle), ::core::mem::transmute(sweepangle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluPerspective(fovy: f64, aspect: f64, znear: f64, zfar: f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluPerspective(fovy: f64, aspect: f64, znear: f64, zfar: f64);
        }
        ::core::mem::transmute(gluPerspective(::core::mem::transmute(fovy), ::core::mem::transmute(aspect), ::core::mem::transmute(znear), ::core::mem::transmute(zfar)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluPickMatrix(x: f64, y: f64, width: f64, height: f64, viewport: *mut i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluPickMatrix(x: f64, y: f64, width: f64, height: f64, viewport: *mut i32);
        }
        ::core::mem::transmute(gluPickMatrix(::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(viewport)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluProject(objx: f64, objy: f64, objz: f64, modelmatrix: *const f64, projmatrix: *const f64, viewport: *const i32, winx: *mut f64, winy: *mut f64, winz: *mut f64) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluProject(objx: f64, objy: f64, objz: f64, modelmatrix: *const f64, projmatrix: *const f64, viewport: *const i32, winx: *mut f64, winy: *mut f64, winz: *mut f64) -> i32;
        }
        ::core::mem::transmute(gluProject(::core::mem::transmute(objx), ::core::mem::transmute(objy), ::core::mem::transmute(objz), ::core::mem::transmute(modelmatrix), ::core::mem::transmute(projmatrix), ::core::mem::transmute(viewport), ::core::mem::transmute(winx), ::core::mem::transmute(winy), ::core::mem::transmute(winz)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluPwlCurve(nobj: *mut GLUnurbs, count: i32, array: *mut f32, stride: i32, r#type: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluPwlCurve(nobj: *mut GLUnurbs, count: i32, array: *mut f32, stride: i32, r#type: u32);
        }
        ::core::mem::transmute(gluPwlCurve(::core::mem::transmute(nobj), ::core::mem::transmute(count), ::core::mem::transmute(array), ::core::mem::transmute(stride), ::core::mem::transmute(r#type)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluQuadricCallback(qobj: *mut GLUquadric, which: u32, r#fn: isize) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluQuadricCallback(qobj: *mut GLUquadric, which: u32, r#fn: isize);
        }
        ::core::mem::transmute(gluQuadricCallback(::core::mem::transmute(qobj), ::core::mem::transmute(which), ::core::mem::transmute(r#fn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluQuadricDrawStyle(quadobject: *mut GLUquadric, drawstyle: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluQuadricDrawStyle(quadobject: *mut GLUquadric, drawstyle: u32);
        }
        ::core::mem::transmute(gluQuadricDrawStyle(::core::mem::transmute(quadobject), ::core::mem::transmute(drawstyle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluQuadricNormals(quadobject: *mut GLUquadric, normals: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluQuadricNormals(quadobject: *mut GLUquadric, normals: u32);
        }
        ::core::mem::transmute(gluQuadricNormals(::core::mem::transmute(quadobject), ::core::mem::transmute(normals)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluQuadricOrientation(quadobject: *mut GLUquadric, orientation: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluQuadricOrientation(quadobject: *mut GLUquadric, orientation: u32);
        }
        ::core::mem::transmute(gluQuadricOrientation(::core::mem::transmute(quadobject), ::core::mem::transmute(orientation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluQuadricTexture(quadobject: *mut GLUquadric, texturecoords: u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluQuadricTexture(quadobject: *mut GLUquadric, texturecoords: u8);
        }
        ::core::mem::transmute(gluQuadricTexture(::core::mem::transmute(quadobject), ::core::mem::transmute(texturecoords)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluScaleImage(format: u32, widthin: i32, heightin: i32, typein: u32, datain: *const ::core::ffi::c_void, widthout: i32, heightout: i32, typeout: u32, dataout: *mut ::core::ffi::c_void) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluScaleImage(format: u32, widthin: i32, heightin: i32, typein: u32, datain: *const ::core::ffi::c_void, widthout: i32, heightout: i32, typeout: u32, dataout: *mut ::core::ffi::c_void) -> i32;
        }
        ::core::mem::transmute(gluScaleImage(::core::mem::transmute(format), ::core::mem::transmute(widthin), ::core::mem::transmute(heightin), ::core::mem::transmute(typein), ::core::mem::transmute(datain), ::core::mem::transmute(widthout), ::core::mem::transmute(heightout), ::core::mem::transmute(typeout), ::core::mem::transmute(dataout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluSphere(qobj: *mut GLUquadric, radius: f64, slices: i32, stacks: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluSphere(qobj: *mut GLUquadric, radius: f64, slices: i32, stacks: i32);
        }
        ::core::mem::transmute(gluSphere(::core::mem::transmute(qobj), ::core::mem::transmute(radius), ::core::mem::transmute(slices), ::core::mem::transmute(stacks)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluTessBeginContour(tess: *mut GLUtesselator) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluTessBeginContour(tess: *mut GLUtesselator);
        }
        ::core::mem::transmute(gluTessBeginContour(::core::mem::transmute(tess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluTessBeginPolygon(tess: *mut GLUtesselator, polygon_data: *mut ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluTessBeginPolygon(tess: *mut GLUtesselator, polygon_data: *mut ::core::ffi::c_void);
        }
        ::core::mem::transmute(gluTessBeginPolygon(::core::mem::transmute(tess), ::core::mem::transmute(polygon_data)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluTessCallback(tess: *mut GLUtesselator, which: u32, r#fn: isize) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluTessCallback(tess: *mut GLUtesselator, which: u32, r#fn: isize);
        }
        ::core::mem::transmute(gluTessCallback(::core::mem::transmute(tess), ::core::mem::transmute(which), ::core::mem::transmute(r#fn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluTessEndContour(tess: *mut GLUtesselator) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluTessEndContour(tess: *mut GLUtesselator);
        }
        ::core::mem::transmute(gluTessEndContour(::core::mem::transmute(tess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluTessEndPolygon(tess: *mut GLUtesselator) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluTessEndPolygon(tess: *mut GLUtesselator);
        }
        ::core::mem::transmute(gluTessEndPolygon(::core::mem::transmute(tess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluTessNormal(tess: *mut GLUtesselator, x: f64, y: f64, z: f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluTessNormal(tess: *mut GLUtesselator, x: f64, y: f64, z: f64);
        }
        ::core::mem::transmute(gluTessNormal(::core::mem::transmute(tess), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluTessProperty(tess: *mut GLUtesselator, which: u32, value: f64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluTessProperty(tess: *mut GLUtesselator, which: u32, value: f64);
        }
        ::core::mem::transmute(gluTessProperty(::core::mem::transmute(tess), ::core::mem::transmute(which), ::core::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluTessVertex(tess: *mut GLUtesselator, coords: *mut f64, data: *mut ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluTessVertex(tess: *mut GLUtesselator, coords: *mut f64, data: *mut ::core::ffi::c_void);
        }
        ::core::mem::transmute(gluTessVertex(::core::mem::transmute(tess), ::core::mem::transmute(coords), ::core::mem::transmute(data)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn gluUnProject(winx: f64, winy: f64, winz: f64, modelmatrix: *const f64, projmatrix: *const f64, viewport: *const i32, objx: *mut f64, objy: *mut f64, objz: *mut f64) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn gluUnProject(winx: f64, winy: f64, winz: f64, modelmatrix: *const f64, projmatrix: *const f64, viewport: *const i32, objx: *mut f64, objy: *mut f64, objz: *mut f64) -> i32;
        }
        ::core::mem::transmute(gluUnProject(::core::mem::transmute(winx), ::core::mem::transmute(winy), ::core::mem::transmute(winz), ::core::mem::transmute(modelmatrix), ::core::mem::transmute(projmatrix), ::core::mem::transmute(viewport), ::core::mem::transmute(objx), ::core::mem::transmute(objy), ::core::mem::transmute(objz)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn wglCopyContext<'a, Param0: ::windows::core::IntoParam<'a, HGLRC>, Param1: ::windows::core::IntoParam<'a, HGLRC>>(param0: Param0, param1: Param1, param2: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn wglCopyContext(param0: HGLRC, param1: HGLRC, param2: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(wglCopyContext(param0.into_param().abi(), param1.into_param().abi(), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn wglCreateContext<'a, Param0: ::windows::core::IntoParam<'a, super::Gdi::HDC>>(param0: Param0) -> HGLRC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn wglCreateContext(param0: super::Gdi::HDC) -> HGLRC;
        }
        ::core::mem::transmute(wglCreateContext(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn wglCreateLayerContext<'a, Param0: ::windows::core::IntoParam<'a, super::Gdi::HDC>>(param0: Param0, param1: i32) -> HGLRC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn wglCreateLayerContext(param0: super::Gdi::HDC, param1: i32) -> HGLRC;
        }
        ::core::mem::transmute(wglCreateLayerContext(param0.into_param().abi(), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn wglDeleteContext<'a, Param0: ::windows::core::IntoParam<'a, HGLRC>>(param0: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn wglDeleteContext(param0: HGLRC) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(wglDeleteContext(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn wglDescribeLayerPlane<'a, Param0: ::windows::core::IntoParam<'a, super::Gdi::HDC>>(param0: Param0, param1: i32, param2: i32, param3: u32, param4: *mut LAYERPLANEDESCRIPTOR) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn wglDescribeLayerPlane(param0: super::Gdi::HDC, param1: i32, param2: i32, param3: u32, param4: *mut LAYERPLANEDESCRIPTOR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(wglDescribeLayerPlane(param0.into_param().abi(), ::core::mem::transmute(param1), ::core::mem::transmute(param2), ::core::mem::transmute(param3), ::core::mem::transmute(param4)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn wglGetCurrentContext() -> HGLRC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn wglGetCurrentContext() -> HGLRC;
        }
        ::core::mem::transmute(wglGetCurrentContext())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn wglGetCurrentDC() -> super::Gdi::HDC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn wglGetCurrentDC() -> super::Gdi::HDC;
        }
        ::core::mem::transmute(wglGetCurrentDC())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn wglGetLayerPaletteEntries<'a, Param0: ::windows::core::IntoParam<'a, super::Gdi::HDC>>(param0: Param0, param1: i32, param2: i32, param3: i32, param4: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn wglGetLayerPaletteEntries(param0: super::Gdi::HDC, param1: i32, param2: i32, param3: i32, param4: *mut u32) -> i32;
        }
        ::core::mem::transmute(wglGetLayerPaletteEntries(param0.into_param().abi(), ::core::mem::transmute(param1), ::core::mem::transmute(param2), ::core::mem::transmute(param3), ::core::mem::transmute(param4)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn wglGetProcAddress<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(param0: Param0) -> super::super::Foundation::PROC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn wglGetProcAddress(param0: super::super::Foundation::PSTR) -> super::super::Foundation::PROC;
        }
        ::core::mem::transmute(wglGetProcAddress(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn wglMakeCurrent<'a, Param0: ::windows::core::IntoParam<'a, super::Gdi::HDC>, Param1: ::windows::core::IntoParam<'a, HGLRC>>(param0: Param0, param1: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn wglMakeCurrent(param0: super::Gdi::HDC, param1: HGLRC) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(wglMakeCurrent(param0.into_param().abi(), param1.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn wglRealizeLayerPalette<'a, Param0: ::windows::core::IntoParam<'a, super::Gdi::HDC>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(param0: Param0, param1: i32, param2: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn wglRealizeLayerPalette(param0: super::Gdi::HDC, param1: i32, param2: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(wglRealizeLayerPalette(param0.into_param().abi(), ::core::mem::transmute(param1), param2.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn wglSetLayerPaletteEntries<'a, Param0: ::windows::core::IntoParam<'a, super::Gdi::HDC>>(param0: Param0, param1: i32, param2: i32, param3: i32, param4: *const u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn wglSetLayerPaletteEntries(param0: super::Gdi::HDC, param1: i32, param2: i32, param3: i32, param4: *const u32) -> i32;
        }
        ::core::mem::transmute(wglSetLayerPaletteEntries(param0.into_param().abi(), ::core::mem::transmute(param1), ::core::mem::transmute(param2), ::core::mem::transmute(param3), ::core::mem::transmute(param4)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn wglShareLists<'a, Param0: ::windows::core::IntoParam<'a, HGLRC>, Param1: ::windows::core::IntoParam<'a, HGLRC>>(param0: Param0, param1: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn wglShareLists(param0: HGLRC, param1: HGLRC) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(wglShareLists(param0.into_param().abi(), param1.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn wglSwapLayerBuffers<'a, Param0: ::windows::core::IntoParam<'a, super::Gdi::HDC>>(param0: Param0, param1: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn wglSwapLayerBuffers(param0: super::Gdi::HDC, param1: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(wglSwapLayerBuffers(param0.into_param().abi(), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn wglUseFontBitmapsA<'a, Param0: ::windows::core::IntoParam<'a, super::Gdi::HDC>>(param0: Param0, param1: u32, param2: u32, param3: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn wglUseFontBitmapsA(param0: super::Gdi::HDC, param1: u32, param2: u32, param3: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(wglUseFontBitmapsA(param0.into_param().abi(), ::core::mem::transmute(param1), ::core::mem::transmute(param2), ::core::mem::transmute(param3)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn wglUseFontBitmapsW<'a, Param0: ::windows::core::IntoParam<'a, super::Gdi::HDC>>(param0: Param0, param1: u32, param2: u32, param3: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn wglUseFontBitmapsW(param0: super::Gdi::HDC, param1: u32, param2: u32, param3: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(wglUseFontBitmapsW(param0.into_param().abi(), ::core::mem::transmute(param1), ::core::mem::transmute(param2), ::core::mem::transmute(param3)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn wglUseFontOutlinesA<'a, Param0: ::windows::core::IntoParam<'a, super::Gdi::HDC>>(param0: Param0, param1: u32, param2: u32, param3: u32, param4: f32, param5: f32, param6: i32, param7: *mut GLYPHMETRICSFLOAT) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn wglUseFontOutlinesA(param0: super::Gdi::HDC, param1: u32, param2: u32, param3: u32, param4: f32, param5: f32, param6: i32, param7: *mut GLYPHMETRICSFLOAT) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(wglUseFontOutlinesA(param0.into_param().abi(), ::core::mem::transmute(param1), ::core::mem::transmute(param2), ::core::mem::transmute(param3), ::core::mem::transmute(param4), ::core::mem::transmute(param5), ::core::mem::transmute(param6), ::core::mem::transmute(param7)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn wglUseFontOutlinesW<'a, Param0: ::windows::core::IntoParam<'a, super::Gdi::HDC>>(param0: Param0, param1: u32, param2: u32, param3: u32, param4: f32, param5: f32, param6: i32, param7: *mut GLYPHMETRICSFLOAT) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn wglUseFontOutlinesW(param0: super::Gdi::HDC, param1: u32, param2: u32, param3: u32, param4: f32, param5: f32, param6: i32, param7: *mut GLYPHMETRICSFLOAT) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(wglUseFontOutlinesW(param0.into_param().abi(), ::core::mem::transmute(param1), ::core::mem::transmute(param2), ::core::mem::transmute(param3), ::core::mem::transmute(param4), ::core::mem::transmute(param5), ::core::mem::transmute(param6), ::core::mem::transmute(param7)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
