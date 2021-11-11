#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ChoosePixelFormat(hdc: super::Gdi::HDC, ppfd: *const PIXELFORMATDESCRIPTOR) -> i32;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn DescribePixelFormat(hdc: super::Gdi::HDC, ipixelformat: i32, nbytes: u32, ppfd: *mut PIXELFORMATDESCRIPTOR) -> i32;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetEnhMetaFilePixelFormat(hemf: super::Gdi::HENHMETAFILE, cbbuffer: u32, ppfd: *mut PIXELFORMATDESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetPixelFormat(hdc: super::Gdi::HDC) -> i32;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn SetPixelFormat(hdc: super::Gdi::HDC, format: i32, ppfd: *const PIXELFORMATDESCRIPTOR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn SwapBuffers(param0: super::Gdi::HDC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glAccum(op: u32, value: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glAlphaFunc(func: u32, r#ref: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glAreTexturesResident(n: i32, textures: *const u32, residences: *mut u8) -> u8;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glArrayElement(i: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glBegin(mode: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glBindTexture(target: u32, texture: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glBitmap(width: i32, height: i32, xorig: f32, yorig: f32, xmove: f32, ymove: f32, bitmap: *const u8);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glBlendFunc(sfactor: u32, dfactor: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glCallList(list: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glCallLists(n: i32, r#type: u32, lists: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glClear(mask: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glClearAccum(red: f32, green: f32, blue: f32, alpha: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glClearColor(red: f32, green: f32, blue: f32, alpha: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glClearDepth(depth: f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glClearIndex(c: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glClearStencil(s: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glClipPlane(plane: u32, equation: *const f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor3b(red: i8, green: i8, blue: i8);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor3bv(v: *const i8);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor3d(red: f64, green: f64, blue: f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor3dv(v: *const f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor3f(red: f32, green: f32, blue: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor3fv(v: *const f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor3i(red: i32, green: i32, blue: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor3iv(v: *const i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor3s(red: i16, green: i16, blue: i16);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor3sv(v: *const i16);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor3ub(red: u8, green: u8, blue: u8);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor3ubv(v: *const u8);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor3ui(red: u32, green: u32, blue: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor3uiv(v: *const u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor3us(red: u16, green: u16, blue: u16);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor3usv(v: *const u16);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor4b(red: i8, green: i8, blue: i8, alpha: i8);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor4bv(v: *const i8);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor4d(red: f64, green: f64, blue: f64, alpha: f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor4dv(v: *const f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor4f(red: f32, green: f32, blue: f32, alpha: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor4fv(v: *const f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor4i(red: i32, green: i32, blue: i32, alpha: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor4iv(v: *const i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor4s(red: i16, green: i16, blue: i16, alpha: i16);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor4sv(v: *const i16);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor4ub(red: u8, green: u8, blue: u8, alpha: u8);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor4ubv(v: *const u8);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor4ui(red: u32, green: u32, blue: u32, alpha: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor4uiv(v: *const u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor4us(red: u16, green: u16, blue: u16, alpha: u16);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor4usv(v: *const u16);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColorMask(red: u8, green: u8, blue: u8, alpha: u8);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColorMaterial(face: u32, mode: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColorPointer(size: i32, r#type: u32, stride: i32, pointer: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glCopyPixels(x: i32, y: i32, width: i32, height: i32, r#type: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glCopyTexImage1D(target: u32, level: i32, internalformat: u32, x: i32, y: i32, width: i32, border: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glCopyTexImage2D(target: u32, level: i32, internalformat: u32, x: i32, y: i32, width: i32, height: i32, border: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glCopyTexSubImage1D(target: u32, level: i32, xoffset: i32, x: i32, y: i32, width: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glCopyTexSubImage2D(target: u32, level: i32, xoffset: i32, yoffset: i32, x: i32, y: i32, width: i32, height: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glCullFace(mode: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glDeleteLists(list: u32, range: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glDeleteTextures(n: i32, textures: *const u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glDepthFunc(func: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glDepthMask(flag: u8);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glDepthRange(znear: f64, zfar: f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glDisable(cap: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glDisableClientState(array: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glDrawArrays(mode: u32, first: i32, count: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glDrawBuffer(mode: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glDrawElements(mode: u32, count: i32, r#type: u32, indices: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glDrawPixels(width: i32, height: i32, format: u32, r#type: u32, pixels: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEdgeFlag(flag: u8);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEdgeFlagPointer(stride: i32, pointer: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEdgeFlagv(flag: *const u8);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEnable(cap: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEnableClientState(array: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEnd();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEndList();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEvalCoord1d(u: f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEvalCoord1dv(u: *const f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEvalCoord1f(u: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEvalCoord1fv(u: *const f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEvalCoord2d(u: f64, v: f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEvalCoord2dv(u: *const f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEvalCoord2f(u: f32, v: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEvalCoord2fv(u: *const f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEvalMesh1(mode: u32, i1: i32, i2: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEvalMesh2(mode: u32, i1: i32, i2: i32, j1: i32, j2: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEvalPoint1(i: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEvalPoint2(i: i32, j: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glFeedbackBuffer(size: i32, r#type: u32, buffer: *mut f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glFinish();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glFlush();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glFogf(pname: u32, param1: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glFogfv(pname: u32, params: *const f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glFogi(pname: u32, param1: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glFogiv(pname: u32, params: *const i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glFrontFace(mode: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glFrustum(left: f64, right: f64, bottom: f64, top: f64, znear: f64, zfar: f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGenLists(range: i32) -> u32;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGenTextures(n: i32, textures: *mut u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetBooleanv(pname: u32, params: *mut u8);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetClipPlane(plane: u32, equation: *mut f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetDoublev(pname: u32, params: *mut f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetError() -> u32;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetFloatv(pname: u32, params: *mut f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetIntegerv(pname: u32, params: *mut i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetLightfv(light: u32, pname: u32, params: *mut f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetLightiv(light: u32, pname: u32, params: *mut i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetMapdv(target: u32, query: u32, v: *mut f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetMapfv(target: u32, query: u32, v: *mut f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetMapiv(target: u32, query: u32, v: *mut i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetMaterialfv(face: u32, pname: u32, params: *mut f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetMaterialiv(face: u32, pname: u32, params: *mut i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetPixelMapfv(map: u32, values: *mut f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetPixelMapuiv(map: u32, values: *mut u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetPixelMapusv(map: u32, values: *mut u16);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetPointerv(pname: u32, params: *mut *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetPolygonStipple(mask: *mut u8);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetString(name: u32) -> *mut u8;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetTexEnvfv(target: u32, pname: u32, params: *mut f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetTexEnviv(target: u32, pname: u32, params: *mut i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetTexGendv(coord: u32, pname: u32, params: *mut f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetTexGenfv(coord: u32, pname: u32, params: *mut f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetTexGeniv(coord: u32, pname: u32, params: *mut i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetTexImage(target: u32, level: i32, format: u32, r#type: u32, pixels: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetTexLevelParameterfv(target: u32, level: i32, pname: u32, params: *mut f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetTexLevelParameteriv(target: u32, level: i32, pname: u32, params: *mut i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetTexParameterfv(target: u32, pname: u32, params: *mut f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetTexParameteriv(target: u32, pname: u32, params: *mut i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glHint(target: u32, mode: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glIndexMask(mask: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glIndexPointer(r#type: u32, stride: i32, pointer: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glIndexd(c: f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glIndexdv(c: *const f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glIndexf(c: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glIndexfv(c: *const f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glIndexi(c: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glIndexiv(c: *const i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glIndexs(c: i16);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glIndexsv(c: *const i16);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glIndexub(c: u8);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glIndexubv(c: *const u8);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glInitNames();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glInterleavedArrays(format: u32, stride: i32, pointer: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glIsEnabled(cap: u32) -> u8;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glIsList(list: u32) -> u8;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glIsTexture(texture: u32) -> u8;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glLightModelf(pname: u32, param1: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glLightModelfv(pname: u32, params: *const f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glLightModeli(pname: u32, param1: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glLightModeliv(pname: u32, params: *const i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glLightf(light: u32, pname: u32, param2: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glLightfv(light: u32, pname: u32, params: *const f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glLighti(light: u32, pname: u32, param2: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glLightiv(light: u32, pname: u32, params: *const i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glLineStipple(factor: i32, pattern: u16);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glLineWidth(width: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glListBase(base: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glLoadIdentity();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glLoadMatrixd(m: *const f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glLoadMatrixf(m: *const f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glLoadName(name: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glLogicOp(opcode: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glMap1d(target: u32, u1: f64, u2: f64, stride: i32, order: i32, points: *const f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glMap1f(target: u32, u1: f32, u2: f32, stride: i32, order: i32, points: *const f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glMap2d(target: u32, u1: f64, u2: f64, ustride: i32, uorder: i32, v1: f64, v2: f64, vstride: i32, vorder: i32, points: *const f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glMap2f(target: u32, u1: f32, u2: f32, ustride: i32, uorder: i32, v1: f32, v2: f32, vstride: i32, vorder: i32, points: *const f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glMapGrid1d(un: i32, u1: f64, u2: f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glMapGrid1f(un: i32, u1: f32, u2: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glMapGrid2d(un: i32, u1: f64, u2: f64, vn: i32, v1: f64, v2: f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glMapGrid2f(un: i32, u1: f32, u2: f32, vn: i32, v1: f32, v2: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glMaterialf(face: u32, pname: u32, param2: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glMaterialfv(face: u32, pname: u32, params: *const f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glMateriali(face: u32, pname: u32, param2: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glMaterialiv(face: u32, pname: u32, params: *const i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glMatrixMode(mode: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glMultMatrixd(m: *const f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glMultMatrixf(m: *const f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glNewList(list: u32, mode: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glNormal3b(nx: i8, ny: i8, nz: i8);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glNormal3bv(v: *const i8);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glNormal3d(nx: f64, ny: f64, nz: f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glNormal3dv(v: *const f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glNormal3f(nx: f32, ny: f32, nz: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glNormal3fv(v: *const f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glNormal3i(nx: i32, ny: i32, nz: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glNormal3iv(v: *const i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glNormal3s(nx: i16, ny: i16, nz: i16);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glNormal3sv(v: *const i16);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glNormalPointer(r#type: u32, stride: i32, pointer: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glOrtho(left: f64, right: f64, bottom: f64, top: f64, znear: f64, zfar: f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPassThrough(token: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPixelMapfv(map: u32, mapsize: i32, values: *const f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPixelMapuiv(map: u32, mapsize: i32, values: *const u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPixelMapusv(map: u32, mapsize: i32, values: *const u16);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPixelStoref(pname: u32, param1: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPixelStorei(pname: u32, param1: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPixelTransferf(pname: u32, param1: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPixelTransferi(pname: u32, param1: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPixelZoom(xfactor: f32, yfactor: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPointSize(size: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPolygonMode(face: u32, mode: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPolygonOffset(factor: f32, units: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPolygonStipple(mask: *const u8);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPopAttrib();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPopClientAttrib();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPopMatrix();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPopName();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPrioritizeTextures(n: i32, textures: *const u32, priorities: *const f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPushAttrib(mask: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPushClientAttrib(mask: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPushMatrix();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPushName(name: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos2d(x: f64, y: f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos2dv(v: *const f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos2f(x: f32, y: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos2fv(v: *const f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos2i(x: i32, y: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos2iv(v: *const i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos2s(x: i16, y: i16);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos2sv(v: *const i16);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos3d(x: f64, y: f64, z: f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos3dv(v: *const f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos3f(x: f32, y: f32, z: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos3fv(v: *const f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos3i(x: i32, y: i32, z: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos3iv(v: *const i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos3s(x: i16, y: i16, z: i16);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos3sv(v: *const i16);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos4d(x: f64, y: f64, z: f64, w: f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos4dv(v: *const f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos4f(x: f32, y: f32, z: f32, w: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos4fv(v: *const f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos4i(x: i32, y: i32, z: i32, w: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos4iv(v: *const i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos4s(x: i16, y: i16, z: i16, w: i16);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos4sv(v: *const i16);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glReadBuffer(mode: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glReadPixels(x: i32, y: i32, width: i32, height: i32, format: u32, r#type: u32, pixels: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRectd(x1: f64, y1: f64, x2: f64, y2: f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRectdv(v1: *const f64, v2: *const f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRectf(x1: f32, y1: f32, x2: f32, y2: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRectfv(v1: *const f32, v2: *const f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRecti(x1: i32, y1: i32, x2: i32, y2: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRectiv(v1: *const i32, v2: *const i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRects(x1: i16, y1: i16, x2: i16, y2: i16);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRectsv(v1: *const i16, v2: *const i16);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRenderMode(mode: u32) -> i32;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRotated(angle: f64, x: f64, y: f64, z: f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRotatef(angle: f32, x: f32, y: f32, z: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glScaled(x: f64, y: f64, z: f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glScalef(x: f32, y: f32, z: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glScissor(x: i32, y: i32, width: i32, height: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glSelectBuffer(size: i32, buffer: *mut u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glShadeModel(mode: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glStencilFunc(func: u32, r#ref: i32, mask: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glStencilMask(mask: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glStencilOp(fail: u32, zfail: u32, zpass: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord1d(s: f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord1dv(v: *const f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord1f(s: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord1fv(v: *const f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord1i(s: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord1iv(v: *const i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord1s(s: i16);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord1sv(v: *const i16);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord2d(s: f64, t: f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord2dv(v: *const f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord2f(s: f32, t: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord2fv(v: *const f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord2i(s: i32, t: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord2iv(v: *const i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord2s(s: i16, t: i16);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord2sv(v: *const i16);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord3d(s: f64, t: f64, r: f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord3dv(v: *const f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord3f(s: f32, t: f32, r: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord3fv(v: *const f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord3i(s: i32, t: i32, r: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord3iv(v: *const i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord3s(s: i16, t: i16, r: i16);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord3sv(v: *const i16);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord4d(s: f64, t: f64, r: f64, q: f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord4dv(v: *const f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord4f(s: f32, t: f32, r: f32, q: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord4fv(v: *const f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord4i(s: i32, t: i32, r: i32, q: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord4iv(v: *const i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord4s(s: i16, t: i16, r: i16, q: i16);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord4sv(v: *const i16);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoordPointer(size: i32, r#type: u32, stride: i32, pointer: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexEnvf(target: u32, pname: u32, param2: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexEnvfv(target: u32, pname: u32, params: *const f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexEnvi(target: u32, pname: u32, param2: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexEnviv(target: u32, pname: u32, params: *const i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexGend(coord: u32, pname: u32, param2: f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexGendv(coord: u32, pname: u32, params: *const f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexGenf(coord: u32, pname: u32, param2: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexGenfv(coord: u32, pname: u32, params: *const f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexGeni(coord: u32, pname: u32, param2: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexGeniv(coord: u32, pname: u32, params: *const i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexImage1D(target: u32, level: i32, internalformat: i32, width: i32, border: i32, format: u32, r#type: u32, pixels: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexImage2D(target: u32, level: i32, internalformat: i32, width: i32, height: i32, border: i32, format: u32, r#type: u32, pixels: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexParameterf(target: u32, pname: u32, param2: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexParameterfv(target: u32, pname: u32, params: *const f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexParameteri(target: u32, pname: u32, param2: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexParameteriv(target: u32, pname: u32, params: *const i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexSubImage1D(target: u32, level: i32, xoffset: i32, width: i32, format: u32, r#type: u32, pixels: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexSubImage2D(target: u32, level: i32, xoffset: i32, yoffset: i32, width: i32, height: i32, format: u32, r#type: u32, pixels: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTranslated(x: f64, y: f64, z: f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTranslatef(x: f32, y: f32, z: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex2d(x: f64, y: f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex2dv(v: *const f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex2f(x: f32, y: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex2fv(v: *const f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex2i(x: i32, y: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex2iv(v: *const i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex2s(x: i16, y: i16);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex2sv(v: *const i16);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex3d(x: f64, y: f64, z: f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex3dv(v: *const f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex3f(x: f32, y: f32, z: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex3fv(v: *const f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex3i(x: i32, y: i32, z: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex3iv(v: *const i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex3s(x: i16, y: i16, z: i16);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex3sv(v: *const i16);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex4d(x: f64, y: f64, z: f64, w: f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex4dv(v: *const f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex4f(x: f32, y: f32, z: f32, w: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex4fv(v: *const f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex4i(x: i32, y: i32, z: i32, w: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex4iv(v: *const i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex4s(x: i16, y: i16, z: i16, w: i16);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex4sv(v: *const i16);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertexPointer(size: i32, r#type: u32, stride: i32, pointer: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glViewport(x: i32, y: i32, width: i32, height: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluBeginCurve(nobj: *mut GLUnurbs);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluBeginPolygon(tess: *mut GLUtesselator);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluBeginSurface(nobj: *mut GLUnurbs);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluBeginTrim(nobj: *mut GLUnurbs);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluBuild1DMipmaps(target: u32, components: i32, width: i32, format: u32, r#type: u32, data: *const ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluBuild2DMipmaps(target: u32, components: i32, width: i32, height: i32, format: u32, r#type: u32, data: *const ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluCylinder(qobj: *mut GLUquadric, baseradius: f64, topradius: f64, height: f64, slices: i32, stacks: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluDeleteNurbsRenderer(nobj: *mut GLUnurbs);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluDeleteQuadric(state: *mut GLUquadric);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluDeleteTess(tess: *mut GLUtesselator);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluDisk(qobj: *mut GLUquadric, innerradius: f64, outerradius: f64, slices: i32, loops: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluEndCurve(nobj: *mut GLUnurbs);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluEndPolygon(tess: *mut GLUtesselator);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluEndSurface(nobj: *mut GLUnurbs);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluEndTrim(nobj: *mut GLUnurbs);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluErrorString(errcode: u32) -> *mut u8;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn gluErrorUnicodeStringEXT(errcode: u32) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluGetNurbsProperty(nobj: *mut GLUnurbs, property: u32, value: *mut f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluGetString(name: u32) -> *mut u8;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluGetTessProperty(tess: *mut GLUtesselator, which: u32, value: *mut f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluLoadSamplingMatrices(nobj: *mut GLUnurbs, modelmatrix: *const f32, projmatrix: *const f32, viewport: *const i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluLookAt(eyex: f64, eyey: f64, eyez: f64, centerx: f64, centery: f64, centerz: f64, upx: f64, upy: f64, upz: f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluNewNurbsRenderer() -> *mut GLUnurbs;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluNewQuadric() -> *mut GLUquadric;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluNewTess() -> *mut GLUtesselator;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluNextContour(tess: *mut GLUtesselator, r#type: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluNurbsCallback(nobj: *mut GLUnurbs, which: u32, r#fn: isize);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluNurbsCurve(nobj: *mut GLUnurbs, nknots: i32, knot: *mut f32, stride: i32, ctlarray: *mut f32, order: i32, r#type: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluNurbsProperty(nobj: *mut GLUnurbs, property: u32, value: f32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluNurbsSurface(nobj: *mut GLUnurbs, sknot_count: i32, sknot: *mut f32, tknot_count: i32, tknot: *mut f32, s_stride: i32, t_stride: i32, ctlarray: *mut f32, sorder: i32, torder: i32, r#type: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluOrtho2D(left: f64, right: f64, bottom: f64, top: f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluPartialDisk(qobj: *mut GLUquadric, innerradius: f64, outerradius: f64, slices: i32, loops: i32, startangle: f64, sweepangle: f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluPerspective(fovy: f64, aspect: f64, znear: f64, zfar: f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluPickMatrix(x: f64, y: f64, width: f64, height: f64, viewport: *mut i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluProject(objx: f64, objy: f64, objz: f64, modelmatrix: *const f64, projmatrix: *const f64, viewport: *const i32, winx: *mut f64, winy: *mut f64, winz: *mut f64) -> i32;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluPwlCurve(nobj: *mut GLUnurbs, count: i32, array: *mut f32, stride: i32, r#type: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluQuadricCallback(qobj: *mut GLUquadric, which: u32, r#fn: isize);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluQuadricDrawStyle(quadobject: *mut GLUquadric, drawstyle: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluQuadricNormals(quadobject: *mut GLUquadric, normals: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluQuadricOrientation(quadobject: *mut GLUquadric, orientation: u32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluQuadricTexture(quadobject: *mut GLUquadric, texturecoords: u8);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluScaleImage(format: u32, widthin: i32, heightin: i32, typein: u32, datain: *const ::core::ffi::c_void, widthout: i32, heightout: i32, typeout: u32, dataout: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluSphere(qobj: *mut GLUquadric, radius: f64, slices: i32, stacks: i32);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluTessBeginContour(tess: *mut GLUtesselator);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluTessBeginPolygon(tess: *mut GLUtesselator, polygon_data: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluTessCallback(tess: *mut GLUtesselator, which: u32, r#fn: isize);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluTessEndContour(tess: *mut GLUtesselator);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluTessEndPolygon(tess: *mut GLUtesselator);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluTessNormal(tess: *mut GLUtesselator, x: f64, y: f64, z: f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluTessProperty(tess: *mut GLUtesselator, which: u32, value: f64);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluTessVertex(tess: *mut GLUtesselator, coords: *mut f64, data: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluUnProject(winx: f64, winy: f64, winz: f64, modelmatrix: *const f64, projmatrix: *const f64, viewport: *const i32, objx: *mut f64, objy: *mut f64, objz: *mut f64) -> i32;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn wglCopyContext(param0: HGLRC, param1: HGLRC, param2: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn wglCreateContext(param0: super::Gdi::HDC) -> HGLRC;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn wglCreateLayerContext(param0: super::Gdi::HDC, param1: i32) -> HGLRC;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn wglDeleteContext(param0: HGLRC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn wglDescribeLayerPlane(param0: super::Gdi::HDC, param1: i32, param2: i32, param3: u32, param4: *mut LAYERPLANEDESCRIPTOR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn wglGetCurrentContext() -> HGLRC;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn wglGetCurrentDC() -> super::Gdi::HDC;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn wglGetLayerPaletteEntries(param0: super::Gdi::HDC, param1: i32, param2: i32, param3: i32, param4: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn wglGetProcAddress(param0: super::super::Foundation::PSTR) -> ::core::option::Option<super::super::Foundation::PROC>;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn wglMakeCurrent(param0: super::Gdi::HDC, param1: HGLRC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn wglRealizeLayerPalette(param0: super::Gdi::HDC, param1: i32, param2: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn wglSetLayerPaletteEntries(param0: super::Gdi::HDC, param1: i32, param2: i32, param3: i32, param4: *const u32) -> i32;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn wglShareLists(param0: HGLRC, param1: HGLRC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn wglSwapLayerBuffers(param0: super::Gdi::HDC, param1: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn wglUseFontBitmapsA(param0: super::Gdi::HDC, param1: u32, param2: u32, param3: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn wglUseFontBitmapsW(param0: super::Gdi::HDC, param1: u32, param2: u32, param3: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn wglUseFontOutlinesA(param0: super::Gdi::HDC, param1: u32, param2: u32, param3: u32, param4: f32, param5: f32, param6: i32, param7: *mut GLYPHMETRICSFLOAT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn wglUseFontOutlinesW(param0: super::Gdi::HDC, param1: u32, param2: u32, param3: u32, param4: f32, param5: f32, param6: i32, param7: *mut GLYPHMETRICSFLOAT) -> super::super::Foundation::BOOL;
}
