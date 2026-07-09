#[inline]
pub unsafe fn glAccum(op: GLenum, value: f32) {
    windows_core::link!("opengl32.dll" "system" fn glAccum(op : GLenum, value : f32));
    unsafe { glAccum(op, value) }
}
#[inline]
pub unsafe fn glAlphaFunc(func: GLenum, r#ref: f32) {
    windows_core::link!("opengl32.dll" "system" fn glAlphaFunc(func : GLenum, r#ref : f32));
    unsafe { glAlphaFunc(func, r#ref) }
}
#[inline]
pub unsafe fn glAreTexturesResident(n: GLsizei, textures: *const GLuint, residences: *mut GLboolean) -> GLboolean {
    windows_core::link!("opengl32.dll" "system" fn glAreTexturesResident(n : GLsizei, textures : *const GLuint, residences : *mut GLboolean) -> GLboolean);
    unsafe { glAreTexturesResident(n, textures, residences as _) }
}
#[inline]
pub unsafe fn glArrayElement(i: GLint) {
    windows_core::link!("opengl32.dll" "system" fn glArrayElement(i : GLint));
    unsafe { glArrayElement(i) }
}
#[inline]
pub unsafe fn glBegin(mode: GLenum) {
    windows_core::link!("opengl32.dll" "system" fn glBegin(mode : GLenum));
    unsafe { glBegin(mode) }
}
#[inline]
pub unsafe fn glBindTexture(target: GLenum, texture: GLuint) {
    windows_core::link!("opengl32.dll" "system" fn glBindTexture(target : GLenum, texture : GLuint));
    unsafe { glBindTexture(target, texture) }
}
#[inline]
pub unsafe fn glBitmap(width: GLsizei, height: GLsizei, xorig: f32, yorig: f32, xmove: f32, ymove: f32, bitmap: *const GLubyte) {
    windows_core::link!("opengl32.dll" "system" fn glBitmap(width : GLsizei, height : GLsizei, xorig : f32, yorig : f32, xmove : f32, ymove : f32, bitmap : *const GLubyte));
    unsafe { glBitmap(width, height, xorig, yorig, xmove, ymove, bitmap) }
}
#[inline]
pub unsafe fn glBlendFunc(sfactor: GLenum, dfactor: GLenum) {
    windows_core::link!("opengl32.dll" "system" fn glBlendFunc(sfactor : GLenum, dfactor : GLenum));
    unsafe { glBlendFunc(sfactor, dfactor) }
}
#[inline]
pub unsafe fn glCallList(list: GLuint) {
    windows_core::link!("opengl32.dll" "system" fn glCallList(list : GLuint));
    unsafe { glCallList(list) }
}
#[inline]
pub unsafe fn glCallLists(n: GLsizei, r#type: GLenum, lists: *const GLvoid) {
    windows_core::link!("opengl32.dll" "system" fn glCallLists(n : GLsizei, r#type : GLenum, lists : *const GLvoid));
    unsafe { glCallLists(n, r#type, lists) }
}
#[inline]
pub unsafe fn glClear(mask: GLbitfield) {
    windows_core::link!("opengl32.dll" "system" fn glClear(mask : GLbitfield));
    unsafe { glClear(mask) }
}
#[inline]
pub unsafe fn glClearAccum(red: f32, green: f32, blue: f32, alpha: f32) {
    windows_core::link!("opengl32.dll" "system" fn glClearAccum(red : f32, green : f32, blue : f32, alpha : f32));
    unsafe { glClearAccum(red, green, blue, alpha) }
}
#[inline]
pub unsafe fn glClearColor(red: f32, green: f32, blue: f32, alpha: f32) {
    windows_core::link!("opengl32.dll" "system" fn glClearColor(red : f32, green : f32, blue : f32, alpha : f32));
    unsafe { glClearColor(red, green, blue, alpha) }
}
#[inline]
pub unsafe fn glClearDepth(depth: f64) {
    windows_core::link!("opengl32.dll" "system" fn glClearDepth(depth : f64));
    unsafe { glClearDepth(depth) }
}
#[inline]
pub unsafe fn glClearIndex(c: f32) {
    windows_core::link!("opengl32.dll" "system" fn glClearIndex(c : f32));
    unsafe { glClearIndex(c) }
}
#[inline]
pub unsafe fn glClearStencil(s: GLint) {
    windows_core::link!("opengl32.dll" "system" fn glClearStencil(s : GLint));
    unsafe { glClearStencil(s) }
}
#[inline]
pub unsafe fn glClipPlane(plane: GLenum, equation: *const f64) {
    windows_core::link!("opengl32.dll" "system" fn glClipPlane(plane : GLenum, equation : *const f64));
    unsafe { glClipPlane(plane, equation) }
}
#[inline]
pub unsafe fn glColor3b(red: GLbyte, green: GLbyte, blue: GLbyte) {
    windows_core::link!("opengl32.dll" "system" fn glColor3b(red : GLbyte, green : GLbyte, blue : GLbyte));
    unsafe { glColor3b(red, green, blue) }
}
#[inline]
pub unsafe fn glColor3bv(v: *const GLbyte) {
    windows_core::link!("opengl32.dll" "system" fn glColor3bv(v : *const GLbyte));
    unsafe { glColor3bv(v) }
}
#[inline]
pub unsafe fn glColor3d(red: f64, green: f64, blue: f64) {
    windows_core::link!("opengl32.dll" "system" fn glColor3d(red : f64, green : f64, blue : f64));
    unsafe { glColor3d(red, green, blue) }
}
#[inline]
pub unsafe fn glColor3dv(v: *const f64) {
    windows_core::link!("opengl32.dll" "system" fn glColor3dv(v : *const f64));
    unsafe { glColor3dv(v) }
}
#[inline]
pub unsafe fn glColor3f(red: f32, green: f32, blue: f32) {
    windows_core::link!("opengl32.dll" "system" fn glColor3f(red : f32, green : f32, blue : f32));
    unsafe { glColor3f(red, green, blue) }
}
#[inline]
pub unsafe fn glColor3fv(v: *const f32) {
    windows_core::link!("opengl32.dll" "system" fn glColor3fv(v : *const f32));
    unsafe { glColor3fv(v) }
}
#[inline]
pub unsafe fn glColor3i(red: GLint, green: GLint, blue: GLint) {
    windows_core::link!("opengl32.dll" "system" fn glColor3i(red : GLint, green : GLint, blue : GLint));
    unsafe { glColor3i(red, green, blue) }
}
#[inline]
pub unsafe fn glColor3iv(v: *const GLint) {
    windows_core::link!("opengl32.dll" "system" fn glColor3iv(v : *const GLint));
    unsafe { glColor3iv(v) }
}
#[inline]
pub unsafe fn glColor3s(red: GLshort, green: GLshort, blue: GLshort) {
    windows_core::link!("opengl32.dll" "system" fn glColor3s(red : GLshort, green : GLshort, blue : GLshort));
    unsafe { glColor3s(red, green, blue) }
}
#[inline]
pub unsafe fn glColor3sv(v: *const GLshort) {
    windows_core::link!("opengl32.dll" "system" fn glColor3sv(v : *const GLshort));
    unsafe { glColor3sv(v) }
}
#[inline]
pub unsafe fn glColor3ub(red: GLubyte, green: GLubyte, blue: GLubyte) {
    windows_core::link!("opengl32.dll" "system" fn glColor3ub(red : GLubyte, green : GLubyte, blue : GLubyte));
    unsafe { glColor3ub(red, green, blue) }
}
#[inline]
pub unsafe fn glColor3ubv(v: *const GLubyte) {
    windows_core::link!("opengl32.dll" "system" fn glColor3ubv(v : *const GLubyte));
    unsafe { glColor3ubv(v) }
}
#[inline]
pub unsafe fn glColor3ui(red: GLuint, green: GLuint, blue: GLuint) {
    windows_core::link!("opengl32.dll" "system" fn glColor3ui(red : GLuint, green : GLuint, blue : GLuint));
    unsafe { glColor3ui(red, green, blue) }
}
#[inline]
pub unsafe fn glColor3uiv(v: *const GLuint) {
    windows_core::link!("opengl32.dll" "system" fn glColor3uiv(v : *const GLuint));
    unsafe { glColor3uiv(v) }
}
#[inline]
pub unsafe fn glColor3us(red: GLushort, green: GLushort, blue: GLushort) {
    windows_core::link!("opengl32.dll" "system" fn glColor3us(red : GLushort, green : GLushort, blue : GLushort));
    unsafe { glColor3us(red, green, blue) }
}
#[inline]
pub unsafe fn glColor3usv(v: *const GLushort) {
    windows_core::link!("opengl32.dll" "system" fn glColor3usv(v : *const GLushort));
    unsafe { glColor3usv(v) }
}
#[inline]
pub unsafe fn glColor4b(red: GLbyte, green: GLbyte, blue: GLbyte, alpha: GLbyte) {
    windows_core::link!("opengl32.dll" "system" fn glColor4b(red : GLbyte, green : GLbyte, blue : GLbyte, alpha : GLbyte));
    unsafe { glColor4b(red, green, blue, alpha) }
}
#[inline]
pub unsafe fn glColor4bv(v: *const GLbyte) {
    windows_core::link!("opengl32.dll" "system" fn glColor4bv(v : *const GLbyte));
    unsafe { glColor4bv(v) }
}
#[inline]
pub unsafe fn glColor4d(red: f64, green: f64, blue: f64, alpha: f64) {
    windows_core::link!("opengl32.dll" "system" fn glColor4d(red : f64, green : f64, blue : f64, alpha : f64));
    unsafe { glColor4d(red, green, blue, alpha) }
}
#[inline]
pub unsafe fn glColor4dv(v: *const f64) {
    windows_core::link!("opengl32.dll" "system" fn glColor4dv(v : *const f64));
    unsafe { glColor4dv(v) }
}
#[inline]
pub unsafe fn glColor4f(red: f32, green: f32, blue: f32, alpha: f32) {
    windows_core::link!("opengl32.dll" "system" fn glColor4f(red : f32, green : f32, blue : f32, alpha : f32));
    unsafe { glColor4f(red, green, blue, alpha) }
}
#[inline]
pub unsafe fn glColor4fv(v: *const f32) {
    windows_core::link!("opengl32.dll" "system" fn glColor4fv(v : *const f32));
    unsafe { glColor4fv(v) }
}
#[inline]
pub unsafe fn glColor4i(red: GLint, green: GLint, blue: GLint, alpha: GLint) {
    windows_core::link!("opengl32.dll" "system" fn glColor4i(red : GLint, green : GLint, blue : GLint, alpha : GLint));
    unsafe { glColor4i(red, green, blue, alpha) }
}
#[inline]
pub unsafe fn glColor4iv(v: *const GLint) {
    windows_core::link!("opengl32.dll" "system" fn glColor4iv(v : *const GLint));
    unsafe { glColor4iv(v) }
}
#[inline]
pub unsafe fn glColor4s(red: GLshort, green: GLshort, blue: GLshort, alpha: GLshort) {
    windows_core::link!("opengl32.dll" "system" fn glColor4s(red : GLshort, green : GLshort, blue : GLshort, alpha : GLshort));
    unsafe { glColor4s(red, green, blue, alpha) }
}
#[inline]
pub unsafe fn glColor4sv(v: *const GLshort) {
    windows_core::link!("opengl32.dll" "system" fn glColor4sv(v : *const GLshort));
    unsafe { glColor4sv(v) }
}
#[inline]
pub unsafe fn glColor4ub(red: GLubyte, green: GLubyte, blue: GLubyte, alpha: GLubyte) {
    windows_core::link!("opengl32.dll" "system" fn glColor4ub(red : GLubyte, green : GLubyte, blue : GLubyte, alpha : GLubyte));
    unsafe { glColor4ub(red, green, blue, alpha) }
}
#[inline]
pub unsafe fn glColor4ubv(v: *const GLubyte) {
    windows_core::link!("opengl32.dll" "system" fn glColor4ubv(v : *const GLubyte));
    unsafe { glColor4ubv(v) }
}
#[inline]
pub unsafe fn glColor4ui(red: GLuint, green: GLuint, blue: GLuint, alpha: GLuint) {
    windows_core::link!("opengl32.dll" "system" fn glColor4ui(red : GLuint, green : GLuint, blue : GLuint, alpha : GLuint));
    unsafe { glColor4ui(red, green, blue, alpha) }
}
#[inline]
pub unsafe fn glColor4uiv(v: *const GLuint) {
    windows_core::link!("opengl32.dll" "system" fn glColor4uiv(v : *const GLuint));
    unsafe { glColor4uiv(v) }
}
#[inline]
pub unsafe fn glColor4us(red: GLushort, green: GLushort, blue: GLushort, alpha: GLushort) {
    windows_core::link!("opengl32.dll" "system" fn glColor4us(red : GLushort, green : GLushort, blue : GLushort, alpha : GLushort));
    unsafe { glColor4us(red, green, blue, alpha) }
}
#[inline]
pub unsafe fn glColor4usv(v: *const GLushort) {
    windows_core::link!("opengl32.dll" "system" fn glColor4usv(v : *const GLushort));
    unsafe { glColor4usv(v) }
}
#[inline]
pub unsafe fn glColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) {
    windows_core::link!("opengl32.dll" "system" fn glColorMask(red : GLboolean, green : GLboolean, blue : GLboolean, alpha : GLboolean));
    unsafe { glColorMask(red, green, blue, alpha) }
}
#[inline]
pub unsafe fn glColorMaterial(face: GLenum, mode: GLenum) {
    windows_core::link!("opengl32.dll" "system" fn glColorMaterial(face : GLenum, mode : GLenum));
    unsafe { glColorMaterial(face, mode) }
}
#[inline]
pub unsafe fn glColorPointer(size: GLint, r#type: GLenum, stride: GLsizei, pointer: *const GLvoid) {
    windows_core::link!("opengl32.dll" "system" fn glColorPointer(size : GLint, r#type : GLenum, stride : GLsizei, pointer : *const GLvoid));
    unsafe { glColorPointer(size, r#type, stride, pointer) }
}
#[inline]
pub unsafe fn glCopyPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, r#type: GLenum) {
    windows_core::link!("opengl32.dll" "system" fn glCopyPixels(x : GLint, y : GLint, width : GLsizei, height : GLsizei, r#type : GLenum));
    unsafe { glCopyPixels(x, y, width, height, r#type) }
}
#[inline]
pub unsafe fn glCopyTexImage1D(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint) {
    windows_core::link!("opengl32.dll" "system" fn glCopyTexImage1D(target : GLenum, level : GLint, internalformat : GLenum, x : GLint, y : GLint, width : GLsizei, border : GLint));
    unsafe { glCopyTexImage1D(target, level, internalformat, x, y, width, border) }
}
#[inline]
pub unsafe fn glCopyTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint) {
    windows_core::link!("opengl32.dll" "system" fn glCopyTexImage2D(target : GLenum, level : GLint, internalformat : GLenum, x : GLint, y : GLint, width : GLsizei, height : GLsizei, border : GLint));
    unsafe { glCopyTexImage2D(target, level, internalformat, x, y, width, height, border) }
}
#[inline]
pub unsafe fn glCopyTexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei) {
    windows_core::link!("opengl32.dll" "system" fn glCopyTexSubImage1D(target : GLenum, level : GLint, xoffset : GLint, x : GLint, y : GLint, width : GLsizei));
    unsafe { glCopyTexSubImage1D(target, level, xoffset, x, y, width) }
}
#[inline]
pub unsafe fn glCopyTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    windows_core::link!("opengl32.dll" "system" fn glCopyTexSubImage2D(target : GLenum, level : GLint, xoffset : GLint, yoffset : GLint, x : GLint, y : GLint, width : GLsizei, height : GLsizei));
    unsafe { glCopyTexSubImage2D(target, level, xoffset, yoffset, x, y, width, height) }
}
#[inline]
pub unsafe fn glCullFace(mode: GLenum) {
    windows_core::link!("opengl32.dll" "system" fn glCullFace(mode : GLenum));
    unsafe { glCullFace(mode) }
}
#[inline]
pub unsafe fn glDeleteLists(list: GLuint, range: GLsizei) {
    windows_core::link!("opengl32.dll" "system" fn glDeleteLists(list : GLuint, range : GLsizei));
    unsafe { glDeleteLists(list, range) }
}
#[inline]
pub unsafe fn glDeleteTextures(n: GLsizei, textures: *const GLuint) {
    windows_core::link!("opengl32.dll" "system" fn glDeleteTextures(n : GLsizei, textures : *const GLuint));
    unsafe { glDeleteTextures(n, textures) }
}
#[inline]
pub unsafe fn glDepthFunc(func: GLenum) {
    windows_core::link!("opengl32.dll" "system" fn glDepthFunc(func : GLenum));
    unsafe { glDepthFunc(func) }
}
#[inline]
pub unsafe fn glDepthMask(flag: GLboolean) {
    windows_core::link!("opengl32.dll" "system" fn glDepthMask(flag : GLboolean));
    unsafe { glDepthMask(flag) }
}
#[inline]
pub unsafe fn glDepthRange(znear: f64, zfar: f64) {
    windows_core::link!("opengl32.dll" "system" fn glDepthRange(znear : f64, zfar : f64));
    unsafe { glDepthRange(znear, zfar) }
}
#[inline]
pub unsafe fn glDisable(cap: GLenum) {
    windows_core::link!("opengl32.dll" "system" fn glDisable(cap : GLenum));
    unsafe { glDisable(cap) }
}
#[inline]
pub unsafe fn glDisableClientState(array: GLenum) {
    windows_core::link!("opengl32.dll" "system" fn glDisableClientState(array : GLenum));
    unsafe { glDisableClientState(array) }
}
#[inline]
pub unsafe fn glDrawArrays(mode: GLenum, first: GLint, count: GLsizei) {
    windows_core::link!("opengl32.dll" "system" fn glDrawArrays(mode : GLenum, first : GLint, count : GLsizei));
    unsafe { glDrawArrays(mode, first, count) }
}
#[inline]
pub unsafe fn glDrawBuffer(mode: GLenum) {
    windows_core::link!("opengl32.dll" "system" fn glDrawBuffer(mode : GLenum));
    unsafe { glDrawBuffer(mode) }
}
#[inline]
pub unsafe fn glDrawElements(mode: GLenum, count: GLsizei, r#type: GLenum, indices: *const GLvoid) {
    windows_core::link!("opengl32.dll" "system" fn glDrawElements(mode : GLenum, count : GLsizei, r#type : GLenum, indices : *const GLvoid));
    unsafe { glDrawElements(mode, count, r#type, indices) }
}
#[inline]
pub unsafe fn glDrawPixels(width: GLsizei, height: GLsizei, format: GLenum, r#type: GLenum, pixels: *const GLvoid) {
    windows_core::link!("opengl32.dll" "system" fn glDrawPixels(width : GLsizei, height : GLsizei, format : GLenum, r#type : GLenum, pixels : *const GLvoid));
    unsafe { glDrawPixels(width, height, format, r#type, pixels) }
}
#[inline]
pub unsafe fn glEdgeFlag(flag: GLboolean) {
    windows_core::link!("opengl32.dll" "system" fn glEdgeFlag(flag : GLboolean));
    unsafe { glEdgeFlag(flag) }
}
#[inline]
pub unsafe fn glEdgeFlagPointer(stride: GLsizei, pointer: *const GLvoid) {
    windows_core::link!("opengl32.dll" "system" fn glEdgeFlagPointer(stride : GLsizei, pointer : *const GLvoid));
    unsafe { glEdgeFlagPointer(stride, pointer) }
}
#[inline]
pub unsafe fn glEdgeFlagv(flag: *const GLboolean) {
    windows_core::link!("opengl32.dll" "system" fn glEdgeFlagv(flag : *const GLboolean));
    unsafe { glEdgeFlagv(flag) }
}
#[inline]
pub unsafe fn glEnable(cap: GLenum) {
    windows_core::link!("opengl32.dll" "system" fn glEnable(cap : GLenum));
    unsafe { glEnable(cap) }
}
#[inline]
pub unsafe fn glEnableClientState(array: GLenum) {
    windows_core::link!("opengl32.dll" "system" fn glEnableClientState(array : GLenum));
    unsafe { glEnableClientState(array) }
}
#[inline]
pub unsafe fn glEnd() {
    windows_core::link!("opengl32.dll" "system" fn glEnd());
    unsafe { glEnd() }
}
#[inline]
pub unsafe fn glEndList() {
    windows_core::link!("opengl32.dll" "system" fn glEndList());
    unsafe { glEndList() }
}
#[inline]
pub unsafe fn glEvalCoord1d(u: f64) {
    windows_core::link!("opengl32.dll" "system" fn glEvalCoord1d(u : f64));
    unsafe { glEvalCoord1d(u) }
}
#[inline]
pub unsafe fn glEvalCoord1dv(u: *const f64) {
    windows_core::link!("opengl32.dll" "system" fn glEvalCoord1dv(u : *const f64));
    unsafe { glEvalCoord1dv(u) }
}
#[inline]
pub unsafe fn glEvalCoord1f(u: f32) {
    windows_core::link!("opengl32.dll" "system" fn glEvalCoord1f(u : f32));
    unsafe { glEvalCoord1f(u) }
}
#[inline]
pub unsafe fn glEvalCoord1fv(u: *const f32) {
    windows_core::link!("opengl32.dll" "system" fn glEvalCoord1fv(u : *const f32));
    unsafe { glEvalCoord1fv(u) }
}
#[inline]
pub unsafe fn glEvalCoord2d(u: f64, v: f64) {
    windows_core::link!("opengl32.dll" "system" fn glEvalCoord2d(u : f64, v : f64));
    unsafe { glEvalCoord2d(u, v) }
}
#[inline]
pub unsafe fn glEvalCoord2dv(u: *const f64) {
    windows_core::link!("opengl32.dll" "system" fn glEvalCoord2dv(u : *const f64));
    unsafe { glEvalCoord2dv(u) }
}
#[inline]
pub unsafe fn glEvalCoord2f(u: f32, v: f32) {
    windows_core::link!("opengl32.dll" "system" fn glEvalCoord2f(u : f32, v : f32));
    unsafe { glEvalCoord2f(u, v) }
}
#[inline]
pub unsafe fn glEvalCoord2fv(u: *const f32) {
    windows_core::link!("opengl32.dll" "system" fn glEvalCoord2fv(u : *const f32));
    unsafe { glEvalCoord2fv(u) }
}
#[inline]
pub unsafe fn glEvalMesh1(mode: GLenum, i1: GLint, i2: GLint) {
    windows_core::link!("opengl32.dll" "system" fn glEvalMesh1(mode : GLenum, i1 : GLint, i2 : GLint));
    unsafe { glEvalMesh1(mode, i1, i2) }
}
#[inline]
pub unsafe fn glEvalMesh2(mode: GLenum, i1: GLint, i2: GLint, j1: GLint, j2: GLint) {
    windows_core::link!("opengl32.dll" "system" fn glEvalMesh2(mode : GLenum, i1 : GLint, i2 : GLint, j1 : GLint, j2 : GLint));
    unsafe { glEvalMesh2(mode, i1, i2, j1, j2) }
}
#[inline]
pub unsafe fn glEvalPoint1(i: GLint) {
    windows_core::link!("opengl32.dll" "system" fn glEvalPoint1(i : GLint));
    unsafe { glEvalPoint1(i) }
}
#[inline]
pub unsafe fn glEvalPoint2(i: GLint, j: GLint) {
    windows_core::link!("opengl32.dll" "system" fn glEvalPoint2(i : GLint, j : GLint));
    unsafe { glEvalPoint2(i, j) }
}
#[inline]
pub unsafe fn glFeedbackBuffer(size: GLsizei, r#type: GLenum) -> f32 {
    windows_core::link!("opengl32.dll" "system" fn glFeedbackBuffer(size : GLsizei, r#type : GLenum, buffer : *mut f32));
    unsafe {
        let mut result__ = core::mem::zeroed();
        glFeedbackBuffer(size, r#type, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn glFinish() {
    windows_core::link!("opengl32.dll" "system" fn glFinish());
    unsafe { glFinish() }
}
#[inline]
pub unsafe fn glFlush() {
    windows_core::link!("opengl32.dll" "system" fn glFlush());
    unsafe { glFlush() }
}
#[inline]
pub unsafe fn glFogf(pname: GLenum, param: f32) {
    windows_core::link!("opengl32.dll" "system" fn glFogf(pname : GLenum, param : f32));
    unsafe { glFogf(pname, param) }
}
#[inline]
pub unsafe fn glFogfv(pname: GLenum, params: *const f32) {
    windows_core::link!("opengl32.dll" "system" fn glFogfv(pname : GLenum, params : *const f32));
    unsafe { glFogfv(pname, params) }
}
#[inline]
pub unsafe fn glFogi(pname: GLenum, param: GLint) {
    windows_core::link!("opengl32.dll" "system" fn glFogi(pname : GLenum, param : GLint));
    unsafe { glFogi(pname, param) }
}
#[inline]
pub unsafe fn glFogiv(pname: GLenum, params: *const GLint) {
    windows_core::link!("opengl32.dll" "system" fn glFogiv(pname : GLenum, params : *const GLint));
    unsafe { glFogiv(pname, params) }
}
#[inline]
pub unsafe fn glFrontFace(mode: GLenum) {
    windows_core::link!("opengl32.dll" "system" fn glFrontFace(mode : GLenum));
    unsafe { glFrontFace(mode) }
}
#[inline]
pub unsafe fn glFrustum(left: f64, right: f64, bottom: f64, top: f64, znear: f64, zfar: f64) {
    windows_core::link!("opengl32.dll" "system" fn glFrustum(left : f64, right : f64, bottom : f64, top : f64, znear : f64, zfar : f64));
    unsafe { glFrustum(left, right, bottom, top, znear, zfar) }
}
#[inline]
pub unsafe fn glGenLists(range: GLsizei) -> GLuint {
    windows_core::link!("opengl32.dll" "system" fn glGenLists(range : GLsizei) -> GLuint);
    unsafe { glGenLists(range) }
}
#[inline]
pub unsafe fn glGenTextures(n: GLsizei) -> GLuint {
    windows_core::link!("opengl32.dll" "system" fn glGenTextures(n : GLsizei, textures : *mut GLuint));
    unsafe {
        let mut result__ = core::mem::zeroed();
        glGenTextures(n, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn glGetBooleanv(pname: GLenum) -> GLboolean {
    windows_core::link!("opengl32.dll" "system" fn glGetBooleanv(pname : GLenum, params : *mut GLboolean));
    unsafe {
        let mut result__ = core::mem::zeroed();
        glGetBooleanv(pname, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn glGetClipPlane(plane: GLenum) -> f64 {
    windows_core::link!("opengl32.dll" "system" fn glGetClipPlane(plane : GLenum, equation : *mut f64));
    unsafe {
        let mut result__ = core::mem::zeroed();
        glGetClipPlane(plane, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn glGetDoublev(pname: GLenum) -> f64 {
    windows_core::link!("opengl32.dll" "system" fn glGetDoublev(pname : GLenum, params : *mut f64));
    unsafe {
        let mut result__ = core::mem::zeroed();
        glGetDoublev(pname, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn glGetError() -> GLenum {
    windows_core::link!("opengl32.dll" "system" fn glGetError() -> GLenum);
    unsafe { glGetError() }
}
#[inline]
pub unsafe fn glGetFloatv(pname: GLenum) -> f32 {
    windows_core::link!("opengl32.dll" "system" fn glGetFloatv(pname : GLenum, params : *mut f32));
    unsafe {
        let mut result__ = core::mem::zeroed();
        glGetFloatv(pname, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn glGetIntegerv(pname: GLenum) -> GLint {
    windows_core::link!("opengl32.dll" "system" fn glGetIntegerv(pname : GLenum, params : *mut GLint));
    unsafe {
        let mut result__ = core::mem::zeroed();
        glGetIntegerv(pname, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn glGetLightfv(light: GLenum, pname: GLenum) -> f32 {
    windows_core::link!("opengl32.dll" "system" fn glGetLightfv(light : GLenum, pname : GLenum, params : *mut f32));
    unsafe {
        let mut result__ = core::mem::zeroed();
        glGetLightfv(light, pname, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn glGetLightiv(light: GLenum, pname: GLenum) -> GLint {
    windows_core::link!("opengl32.dll" "system" fn glGetLightiv(light : GLenum, pname : GLenum, params : *mut GLint));
    unsafe {
        let mut result__ = core::mem::zeroed();
        glGetLightiv(light, pname, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn glGetMapdv(target: GLenum, query: GLenum) -> f64 {
    windows_core::link!("opengl32.dll" "system" fn glGetMapdv(target : GLenum, query : GLenum, v : *mut f64));
    unsafe {
        let mut result__ = core::mem::zeroed();
        glGetMapdv(target, query, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn glGetMapfv(target: GLenum, query: GLenum) -> f32 {
    windows_core::link!("opengl32.dll" "system" fn glGetMapfv(target : GLenum, query : GLenum, v : *mut f32));
    unsafe {
        let mut result__ = core::mem::zeroed();
        glGetMapfv(target, query, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn glGetMapiv(target: GLenum, query: GLenum) -> GLint {
    windows_core::link!("opengl32.dll" "system" fn glGetMapiv(target : GLenum, query : GLenum, v : *mut GLint));
    unsafe {
        let mut result__ = core::mem::zeroed();
        glGetMapiv(target, query, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn glGetMaterialfv(face: GLenum, pname: GLenum) -> f32 {
    windows_core::link!("opengl32.dll" "system" fn glGetMaterialfv(face : GLenum, pname : GLenum, params : *mut f32));
    unsafe {
        let mut result__ = core::mem::zeroed();
        glGetMaterialfv(face, pname, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn glGetMaterialiv(face: GLenum, pname: GLenum) -> GLint {
    windows_core::link!("opengl32.dll" "system" fn glGetMaterialiv(face : GLenum, pname : GLenum, params : *mut GLint));
    unsafe {
        let mut result__ = core::mem::zeroed();
        glGetMaterialiv(face, pname, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn glGetPixelMapfv(map: GLenum) -> f32 {
    windows_core::link!("opengl32.dll" "system" fn glGetPixelMapfv(map : GLenum, values : *mut f32));
    unsafe {
        let mut result__ = core::mem::zeroed();
        glGetPixelMapfv(map, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn glGetPixelMapuiv(map: GLenum) -> GLuint {
    windows_core::link!("opengl32.dll" "system" fn glGetPixelMapuiv(map : GLenum, values : *mut GLuint));
    unsafe {
        let mut result__ = core::mem::zeroed();
        glGetPixelMapuiv(map, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn glGetPixelMapusv(map: GLenum) -> GLushort {
    windows_core::link!("opengl32.dll" "system" fn glGetPixelMapusv(map : GLenum, values : *mut GLushort));
    unsafe {
        let mut result__ = core::mem::zeroed();
        glGetPixelMapusv(map, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn glGetPointerv(pname: GLenum) -> *mut GLvoid {
    windows_core::link!("opengl32.dll" "system" fn glGetPointerv(pname : GLenum, params : *mut *mut GLvoid));
    unsafe {
        let mut result__ = core::mem::zeroed();
        glGetPointerv(pname, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn glGetPolygonStipple() -> GLubyte {
    windows_core::link!("opengl32.dll" "system" fn glGetPolygonStipple(mask : *mut GLubyte));
    unsafe {
        let mut result__ = core::mem::zeroed();
        glGetPolygonStipple(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn glGetString(name: GLenum) -> *const GLubyte {
    windows_core::link!("opengl32.dll" "system" fn glGetString(name : GLenum) -> *const GLubyte);
    unsafe { glGetString(name) }
}
#[inline]
pub unsafe fn glGetTexEnvfv(target: GLenum, pname: GLenum) -> f32 {
    windows_core::link!("opengl32.dll" "system" fn glGetTexEnvfv(target : GLenum, pname : GLenum, params : *mut f32));
    unsafe {
        let mut result__ = core::mem::zeroed();
        glGetTexEnvfv(target, pname, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn glGetTexEnviv(target: GLenum, pname: GLenum) -> GLint {
    windows_core::link!("opengl32.dll" "system" fn glGetTexEnviv(target : GLenum, pname : GLenum, params : *mut GLint));
    unsafe {
        let mut result__ = core::mem::zeroed();
        glGetTexEnviv(target, pname, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn glGetTexGendv(coord: GLenum, pname: GLenum) -> f64 {
    windows_core::link!("opengl32.dll" "system" fn glGetTexGendv(coord : GLenum, pname : GLenum, params : *mut f64));
    unsafe {
        let mut result__ = core::mem::zeroed();
        glGetTexGendv(coord, pname, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn glGetTexGenfv(coord: GLenum, pname: GLenum) -> f32 {
    windows_core::link!("opengl32.dll" "system" fn glGetTexGenfv(coord : GLenum, pname : GLenum, params : *mut f32));
    unsafe {
        let mut result__ = core::mem::zeroed();
        glGetTexGenfv(coord, pname, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn glGetTexGeniv(coord: GLenum, pname: GLenum) -> GLint {
    windows_core::link!("opengl32.dll" "system" fn glGetTexGeniv(coord : GLenum, pname : GLenum, params : *mut GLint));
    unsafe {
        let mut result__ = core::mem::zeroed();
        glGetTexGeniv(coord, pname, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn glGetTexImage(target: GLenum, level: GLint, format: GLenum, r#type: GLenum) -> GLvoid {
    windows_core::link!("opengl32.dll" "system" fn glGetTexImage(target : GLenum, level : GLint, format : GLenum, r#type : GLenum, pixels : *mut GLvoid));
    unsafe {
        let mut result__ = core::mem::zeroed();
        glGetTexImage(target, level, format, r#type, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn glGetTexLevelParameterfv(target: GLenum, level: GLint, pname: GLenum) -> f32 {
    windows_core::link!("opengl32.dll" "system" fn glGetTexLevelParameterfv(target : GLenum, level : GLint, pname : GLenum, params : *mut f32));
    unsafe {
        let mut result__ = core::mem::zeroed();
        glGetTexLevelParameterfv(target, level, pname, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn glGetTexLevelParameteriv(target: GLenum, level: GLint, pname: GLenum) -> GLint {
    windows_core::link!("opengl32.dll" "system" fn glGetTexLevelParameteriv(target : GLenum, level : GLint, pname : GLenum, params : *mut GLint));
    unsafe {
        let mut result__ = core::mem::zeroed();
        glGetTexLevelParameteriv(target, level, pname, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn glGetTexParameterfv(target: GLenum, pname: GLenum) -> f32 {
    windows_core::link!("opengl32.dll" "system" fn glGetTexParameterfv(target : GLenum, pname : GLenum, params : *mut f32));
    unsafe {
        let mut result__ = core::mem::zeroed();
        glGetTexParameterfv(target, pname, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn glGetTexParameteriv(target: GLenum, pname: GLenum) -> GLint {
    windows_core::link!("opengl32.dll" "system" fn glGetTexParameteriv(target : GLenum, pname : GLenum, params : *mut GLint));
    unsafe {
        let mut result__ = core::mem::zeroed();
        glGetTexParameteriv(target, pname, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn glHint(target: GLenum, mode: GLenum) {
    windows_core::link!("opengl32.dll" "system" fn glHint(target : GLenum, mode : GLenum));
    unsafe { glHint(target, mode) }
}
#[inline]
pub unsafe fn glIndexMask(mask: GLuint) {
    windows_core::link!("opengl32.dll" "system" fn glIndexMask(mask : GLuint));
    unsafe { glIndexMask(mask) }
}
#[inline]
pub unsafe fn glIndexPointer(r#type: GLenum, stride: GLsizei, pointer: *const GLvoid) {
    windows_core::link!("opengl32.dll" "system" fn glIndexPointer(r#type : GLenum, stride : GLsizei, pointer : *const GLvoid));
    unsafe { glIndexPointer(r#type, stride, pointer) }
}
#[inline]
pub unsafe fn glIndexd(c: f64) {
    windows_core::link!("opengl32.dll" "system" fn glIndexd(c : f64));
    unsafe { glIndexd(c) }
}
#[inline]
pub unsafe fn glIndexdv(c: *const f64) {
    windows_core::link!("opengl32.dll" "system" fn glIndexdv(c : *const f64));
    unsafe { glIndexdv(c) }
}
#[inline]
pub unsafe fn glIndexf(c: f32) {
    windows_core::link!("opengl32.dll" "system" fn glIndexf(c : f32));
    unsafe { glIndexf(c) }
}
#[inline]
pub unsafe fn glIndexfv(c: *const f32) {
    windows_core::link!("opengl32.dll" "system" fn glIndexfv(c : *const f32));
    unsafe { glIndexfv(c) }
}
#[inline]
pub unsafe fn glIndexi(c: GLint) {
    windows_core::link!("opengl32.dll" "system" fn glIndexi(c : GLint));
    unsafe { glIndexi(c) }
}
#[inline]
pub unsafe fn glIndexiv(c: *const GLint) {
    windows_core::link!("opengl32.dll" "system" fn glIndexiv(c : *const GLint));
    unsafe { glIndexiv(c) }
}
#[inline]
pub unsafe fn glIndexs(c: GLshort) {
    windows_core::link!("opengl32.dll" "system" fn glIndexs(c : GLshort));
    unsafe { glIndexs(c) }
}
#[inline]
pub unsafe fn glIndexsv(c: *const GLshort) {
    windows_core::link!("opengl32.dll" "system" fn glIndexsv(c : *const GLshort));
    unsafe { glIndexsv(c) }
}
#[inline]
pub unsafe fn glIndexub(c: GLubyte) {
    windows_core::link!("opengl32.dll" "system" fn glIndexub(c : GLubyte));
    unsafe { glIndexub(c) }
}
#[inline]
pub unsafe fn glIndexubv(c: *const GLubyte) {
    windows_core::link!("opengl32.dll" "system" fn glIndexubv(c : *const GLubyte));
    unsafe { glIndexubv(c) }
}
#[inline]
pub unsafe fn glInitNames() {
    windows_core::link!("opengl32.dll" "system" fn glInitNames());
    unsafe { glInitNames() }
}
#[inline]
pub unsafe fn glInterleavedArrays(format: GLenum, stride: GLsizei, pointer: *const GLvoid) {
    windows_core::link!("opengl32.dll" "system" fn glInterleavedArrays(format : GLenum, stride : GLsizei, pointer : *const GLvoid));
    unsafe { glInterleavedArrays(format, stride, pointer) }
}
#[inline]
pub unsafe fn glIsEnabled(cap: GLenum) -> GLboolean {
    windows_core::link!("opengl32.dll" "system" fn glIsEnabled(cap : GLenum) -> GLboolean);
    unsafe { glIsEnabled(cap) }
}
#[inline]
pub unsafe fn glIsList(list: GLuint) -> GLboolean {
    windows_core::link!("opengl32.dll" "system" fn glIsList(list : GLuint) -> GLboolean);
    unsafe { glIsList(list) }
}
#[inline]
pub unsafe fn glIsTexture(texture: GLuint) -> GLboolean {
    windows_core::link!("opengl32.dll" "system" fn glIsTexture(texture : GLuint) -> GLboolean);
    unsafe { glIsTexture(texture) }
}
#[inline]
pub unsafe fn glLightModelf(pname: GLenum, param: f32) {
    windows_core::link!("opengl32.dll" "system" fn glLightModelf(pname : GLenum, param : f32));
    unsafe { glLightModelf(pname, param) }
}
#[inline]
pub unsafe fn glLightModelfv(pname: GLenum, params: *const f32) {
    windows_core::link!("opengl32.dll" "system" fn glLightModelfv(pname : GLenum, params : *const f32));
    unsafe { glLightModelfv(pname, params) }
}
#[inline]
pub unsafe fn glLightModeli(pname: GLenum, param: GLint) {
    windows_core::link!("opengl32.dll" "system" fn glLightModeli(pname : GLenum, param : GLint));
    unsafe { glLightModeli(pname, param) }
}
#[inline]
pub unsafe fn glLightModeliv(pname: GLenum, params: *const GLint) {
    windows_core::link!("opengl32.dll" "system" fn glLightModeliv(pname : GLenum, params : *const GLint));
    unsafe { glLightModeliv(pname, params) }
}
#[inline]
pub unsafe fn glLightf(light: GLenum, pname: GLenum, param: f32) {
    windows_core::link!("opengl32.dll" "system" fn glLightf(light : GLenum, pname : GLenum, param : f32));
    unsafe { glLightf(light, pname, param) }
}
#[inline]
pub unsafe fn glLightfv(light: GLenum, pname: GLenum, params: *const f32) {
    windows_core::link!("opengl32.dll" "system" fn glLightfv(light : GLenum, pname : GLenum, params : *const f32));
    unsafe { glLightfv(light, pname, params) }
}
#[inline]
pub unsafe fn glLighti(light: GLenum, pname: GLenum, param: GLint) {
    windows_core::link!("opengl32.dll" "system" fn glLighti(light : GLenum, pname : GLenum, param : GLint));
    unsafe { glLighti(light, pname, param) }
}
#[inline]
pub unsafe fn glLightiv(light: GLenum, pname: GLenum, params: *const GLint) {
    windows_core::link!("opengl32.dll" "system" fn glLightiv(light : GLenum, pname : GLenum, params : *const GLint));
    unsafe { glLightiv(light, pname, params) }
}
#[inline]
pub unsafe fn glLineStipple(factor: GLint, pattern: GLushort) {
    windows_core::link!("opengl32.dll" "system" fn glLineStipple(factor : GLint, pattern : GLushort));
    unsafe { glLineStipple(factor, pattern) }
}
#[inline]
pub unsafe fn glLineWidth(width: f32) {
    windows_core::link!("opengl32.dll" "system" fn glLineWidth(width : f32));
    unsafe { glLineWidth(width) }
}
#[inline]
pub unsafe fn glListBase(base: GLuint) {
    windows_core::link!("opengl32.dll" "system" fn glListBase(base : GLuint));
    unsafe { glListBase(base) }
}
#[inline]
pub unsafe fn glLoadIdentity() {
    windows_core::link!("opengl32.dll" "system" fn glLoadIdentity());
    unsafe { glLoadIdentity() }
}
#[inline]
pub unsafe fn glLoadMatrixd(m: *const f64) {
    windows_core::link!("opengl32.dll" "system" fn glLoadMatrixd(m : *const f64));
    unsafe { glLoadMatrixd(m) }
}
#[inline]
pub unsafe fn glLoadMatrixf(m: *const f32) {
    windows_core::link!("opengl32.dll" "system" fn glLoadMatrixf(m : *const f32));
    unsafe { glLoadMatrixf(m) }
}
#[inline]
pub unsafe fn glLoadName(name: GLuint) {
    windows_core::link!("opengl32.dll" "system" fn glLoadName(name : GLuint));
    unsafe { glLoadName(name) }
}
#[inline]
pub unsafe fn glLogicOp(opcode: GLenum) {
    windows_core::link!("opengl32.dll" "system" fn glLogicOp(opcode : GLenum));
    unsafe { glLogicOp(opcode) }
}
#[inline]
pub unsafe fn glMap1d(target: GLenum, u1: f64, u2: f64, stride: GLint, order: GLint, points: *const f64) {
    windows_core::link!("opengl32.dll" "system" fn glMap1d(target : GLenum, u1 : f64, u2 : f64, stride : GLint, order : GLint, points : *const f64));
    unsafe { glMap1d(target, u1, u2, stride, order, points) }
}
#[inline]
pub unsafe fn glMap1f(target: GLenum, u1: f32, u2: f32, stride: GLint, order: GLint, points: *const f32) {
    windows_core::link!("opengl32.dll" "system" fn glMap1f(target : GLenum, u1 : f32, u2 : f32, stride : GLint, order : GLint, points : *const f32));
    unsafe { glMap1f(target, u1, u2, stride, order, points) }
}
#[inline]
pub unsafe fn glMap2d(target: GLenum, u1: f64, u2: f64, ustride: GLint, uorder: GLint, v1: f64, v2: f64, vstride: GLint, vorder: GLint, points: *const f64) {
    windows_core::link!("opengl32.dll" "system" fn glMap2d(target : GLenum, u1 : f64, u2 : f64, ustride : GLint, uorder : GLint, v1 : f64, v2 : f64, vstride : GLint, vorder : GLint, points : *const f64));
    unsafe { glMap2d(target, u1, u2, ustride, uorder, v1, v2, vstride, vorder, points) }
}
#[inline]
pub unsafe fn glMap2f(target: GLenum, u1: f32, u2: f32, ustride: GLint, uorder: GLint, v1: f32, v2: f32, vstride: GLint, vorder: GLint, points: *const f32) {
    windows_core::link!("opengl32.dll" "system" fn glMap2f(target : GLenum, u1 : f32, u2 : f32, ustride : GLint, uorder : GLint, v1 : f32, v2 : f32, vstride : GLint, vorder : GLint, points : *const f32));
    unsafe { glMap2f(target, u1, u2, ustride, uorder, v1, v2, vstride, vorder, points) }
}
#[inline]
pub unsafe fn glMapGrid1d(un: GLint, u1: f64, u2: f64) {
    windows_core::link!("opengl32.dll" "system" fn glMapGrid1d(un : GLint, u1 : f64, u2 : f64));
    unsafe { glMapGrid1d(un, u1, u2) }
}
#[inline]
pub unsafe fn glMapGrid1f(un: GLint, u1: f32, u2: f32) {
    windows_core::link!("opengl32.dll" "system" fn glMapGrid1f(un : GLint, u1 : f32, u2 : f32));
    unsafe { glMapGrid1f(un, u1, u2) }
}
#[inline]
pub unsafe fn glMapGrid2d(un: GLint, u1: f64, u2: f64, vn: GLint, v1: f64, v2: f64) {
    windows_core::link!("opengl32.dll" "system" fn glMapGrid2d(un : GLint, u1 : f64, u2 : f64, vn : GLint, v1 : f64, v2 : f64));
    unsafe { glMapGrid2d(un, u1, u2, vn, v1, v2) }
}
#[inline]
pub unsafe fn glMapGrid2f(un: GLint, u1: f32, u2: f32, vn: GLint, v1: f32, v2: f32) {
    windows_core::link!("opengl32.dll" "system" fn glMapGrid2f(un : GLint, u1 : f32, u2 : f32, vn : GLint, v1 : f32, v2 : f32));
    unsafe { glMapGrid2f(un, u1, u2, vn, v1, v2) }
}
#[inline]
pub unsafe fn glMaterialf(face: GLenum, pname: GLenum, param: f32) {
    windows_core::link!("opengl32.dll" "system" fn glMaterialf(face : GLenum, pname : GLenum, param : f32));
    unsafe { glMaterialf(face, pname, param) }
}
#[inline]
pub unsafe fn glMaterialfv(face: GLenum, pname: GLenum, params: *const f32) {
    windows_core::link!("opengl32.dll" "system" fn glMaterialfv(face : GLenum, pname : GLenum, params : *const f32));
    unsafe { glMaterialfv(face, pname, params) }
}
#[inline]
pub unsafe fn glMateriali(face: GLenum, pname: GLenum, param: GLint) {
    windows_core::link!("opengl32.dll" "system" fn glMateriali(face : GLenum, pname : GLenum, param : GLint));
    unsafe { glMateriali(face, pname, param) }
}
#[inline]
pub unsafe fn glMaterialiv(face: GLenum, pname: GLenum, params: *const GLint) {
    windows_core::link!("opengl32.dll" "system" fn glMaterialiv(face : GLenum, pname : GLenum, params : *const GLint));
    unsafe { glMaterialiv(face, pname, params) }
}
#[inline]
pub unsafe fn glMatrixMode(mode: GLenum) {
    windows_core::link!("opengl32.dll" "system" fn glMatrixMode(mode : GLenum));
    unsafe { glMatrixMode(mode) }
}
#[inline]
pub unsafe fn glMultMatrixd(m: *const f64) {
    windows_core::link!("opengl32.dll" "system" fn glMultMatrixd(m : *const f64));
    unsafe { glMultMatrixd(m) }
}
#[inline]
pub unsafe fn glMultMatrixf(m: *const f32) {
    windows_core::link!("opengl32.dll" "system" fn glMultMatrixf(m : *const f32));
    unsafe { glMultMatrixf(m) }
}
#[inline]
pub unsafe fn glNewList(list: GLuint, mode: GLenum) {
    windows_core::link!("opengl32.dll" "system" fn glNewList(list : GLuint, mode : GLenum));
    unsafe { glNewList(list, mode) }
}
#[inline]
pub unsafe fn glNormal3b(nx: GLbyte, ny: GLbyte, nz: GLbyte) {
    windows_core::link!("opengl32.dll" "system" fn glNormal3b(nx : GLbyte, ny : GLbyte, nz : GLbyte));
    unsafe { glNormal3b(nx, ny, nz) }
}
#[inline]
pub unsafe fn glNormal3bv(v: *const GLbyte) {
    windows_core::link!("opengl32.dll" "system" fn glNormal3bv(v : *const GLbyte));
    unsafe { glNormal3bv(v) }
}
#[inline]
pub unsafe fn glNormal3d(nx: f64, ny: f64, nz: f64) {
    windows_core::link!("opengl32.dll" "system" fn glNormal3d(nx : f64, ny : f64, nz : f64));
    unsafe { glNormal3d(nx, ny, nz) }
}
#[inline]
pub unsafe fn glNormal3dv(v: *const f64) {
    windows_core::link!("opengl32.dll" "system" fn glNormal3dv(v : *const f64));
    unsafe { glNormal3dv(v) }
}
#[inline]
pub unsafe fn glNormal3f(nx: f32, ny: f32, nz: f32) {
    windows_core::link!("opengl32.dll" "system" fn glNormal3f(nx : f32, ny : f32, nz : f32));
    unsafe { glNormal3f(nx, ny, nz) }
}
#[inline]
pub unsafe fn glNormal3fv(v: *const f32) {
    windows_core::link!("opengl32.dll" "system" fn glNormal3fv(v : *const f32));
    unsafe { glNormal3fv(v) }
}
#[inline]
pub unsafe fn glNormal3i(nx: GLint, ny: GLint, nz: GLint) {
    windows_core::link!("opengl32.dll" "system" fn glNormal3i(nx : GLint, ny : GLint, nz : GLint));
    unsafe { glNormal3i(nx, ny, nz) }
}
#[inline]
pub unsafe fn glNormal3iv(v: *const GLint) {
    windows_core::link!("opengl32.dll" "system" fn glNormal3iv(v : *const GLint));
    unsafe { glNormal3iv(v) }
}
#[inline]
pub unsafe fn glNormal3s(nx: GLshort, ny: GLshort, nz: GLshort) {
    windows_core::link!("opengl32.dll" "system" fn glNormal3s(nx : GLshort, ny : GLshort, nz : GLshort));
    unsafe { glNormal3s(nx, ny, nz) }
}
#[inline]
pub unsafe fn glNormal3sv(v: *const GLshort) {
    windows_core::link!("opengl32.dll" "system" fn glNormal3sv(v : *const GLshort));
    unsafe { glNormal3sv(v) }
}
#[inline]
pub unsafe fn glNormalPointer(r#type: GLenum, stride: GLsizei, pointer: *const GLvoid) {
    windows_core::link!("opengl32.dll" "system" fn glNormalPointer(r#type : GLenum, stride : GLsizei, pointer : *const GLvoid));
    unsafe { glNormalPointer(r#type, stride, pointer) }
}
#[inline]
pub unsafe fn glOrtho(left: f64, right: f64, bottom: f64, top: f64, znear: f64, zfar: f64) {
    windows_core::link!("opengl32.dll" "system" fn glOrtho(left : f64, right : f64, bottom : f64, top : f64, znear : f64, zfar : f64));
    unsafe { glOrtho(left, right, bottom, top, znear, zfar) }
}
#[inline]
pub unsafe fn glPassThrough(token: f32) {
    windows_core::link!("opengl32.dll" "system" fn glPassThrough(token : f32));
    unsafe { glPassThrough(token) }
}
#[inline]
pub unsafe fn glPixelMapfv(map: GLenum, mapsize: GLsizei, values: *const f32) {
    windows_core::link!("opengl32.dll" "system" fn glPixelMapfv(map : GLenum, mapsize : GLsizei, values : *const f32));
    unsafe { glPixelMapfv(map, mapsize, values) }
}
#[inline]
pub unsafe fn glPixelMapuiv(map: GLenum, mapsize: GLsizei, values: *const GLuint) {
    windows_core::link!("opengl32.dll" "system" fn glPixelMapuiv(map : GLenum, mapsize : GLsizei, values : *const GLuint));
    unsafe { glPixelMapuiv(map, mapsize, values) }
}
#[inline]
pub unsafe fn glPixelMapusv(map: GLenum, mapsize: GLsizei, values: *const GLushort) {
    windows_core::link!("opengl32.dll" "system" fn glPixelMapusv(map : GLenum, mapsize : GLsizei, values : *const GLushort));
    unsafe { glPixelMapusv(map, mapsize, values) }
}
#[inline]
pub unsafe fn glPixelStoref(pname: GLenum, param: f32) {
    windows_core::link!("opengl32.dll" "system" fn glPixelStoref(pname : GLenum, param : f32));
    unsafe { glPixelStoref(pname, param) }
}
#[inline]
pub unsafe fn glPixelStorei(pname: GLenum, param: GLint) {
    windows_core::link!("opengl32.dll" "system" fn glPixelStorei(pname : GLenum, param : GLint));
    unsafe { glPixelStorei(pname, param) }
}
#[inline]
pub unsafe fn glPixelTransferf(pname: GLenum, param: f32) {
    windows_core::link!("opengl32.dll" "system" fn glPixelTransferf(pname : GLenum, param : f32));
    unsafe { glPixelTransferf(pname, param) }
}
#[inline]
pub unsafe fn glPixelTransferi(pname: GLenum, param: GLint) {
    windows_core::link!("opengl32.dll" "system" fn glPixelTransferi(pname : GLenum, param : GLint));
    unsafe { glPixelTransferi(pname, param) }
}
#[inline]
pub unsafe fn glPixelZoom(xfactor: f32, yfactor: f32) {
    windows_core::link!("opengl32.dll" "system" fn glPixelZoom(xfactor : f32, yfactor : f32));
    unsafe { glPixelZoom(xfactor, yfactor) }
}
#[inline]
pub unsafe fn glPointSize(size: f32) {
    windows_core::link!("opengl32.dll" "system" fn glPointSize(size : f32));
    unsafe { glPointSize(size) }
}
#[inline]
pub unsafe fn glPolygonMode(face: GLenum, mode: GLenum) {
    windows_core::link!("opengl32.dll" "system" fn glPolygonMode(face : GLenum, mode : GLenum));
    unsafe { glPolygonMode(face, mode) }
}
#[inline]
pub unsafe fn glPolygonOffset(factor: f32, units: f32) {
    windows_core::link!("opengl32.dll" "system" fn glPolygonOffset(factor : f32, units : f32));
    unsafe { glPolygonOffset(factor, units) }
}
#[inline]
pub unsafe fn glPolygonStipple(mask: *const GLubyte) {
    windows_core::link!("opengl32.dll" "system" fn glPolygonStipple(mask : *const GLubyte));
    unsafe { glPolygonStipple(mask) }
}
#[inline]
pub unsafe fn glPopAttrib() {
    windows_core::link!("opengl32.dll" "system" fn glPopAttrib());
    unsafe { glPopAttrib() }
}
#[inline]
pub unsafe fn glPopClientAttrib() {
    windows_core::link!("opengl32.dll" "system" fn glPopClientAttrib());
    unsafe { glPopClientAttrib() }
}
#[inline]
pub unsafe fn glPopMatrix() {
    windows_core::link!("opengl32.dll" "system" fn glPopMatrix());
    unsafe { glPopMatrix() }
}
#[inline]
pub unsafe fn glPopName() {
    windows_core::link!("opengl32.dll" "system" fn glPopName());
    unsafe { glPopName() }
}
#[inline]
pub unsafe fn glPrioritizeTextures(n: GLsizei, textures: *const GLuint, priorities: *const f32) {
    windows_core::link!("opengl32.dll" "system" fn glPrioritizeTextures(n : GLsizei, textures : *const GLuint, priorities : *const f32));
    unsafe { glPrioritizeTextures(n, textures, priorities) }
}
#[inline]
pub unsafe fn glPushAttrib(mask: GLbitfield) {
    windows_core::link!("opengl32.dll" "system" fn glPushAttrib(mask : GLbitfield));
    unsafe { glPushAttrib(mask) }
}
#[inline]
pub unsafe fn glPushClientAttrib(mask: GLbitfield) {
    windows_core::link!("opengl32.dll" "system" fn glPushClientAttrib(mask : GLbitfield));
    unsafe { glPushClientAttrib(mask) }
}
#[inline]
pub unsafe fn glPushMatrix() {
    windows_core::link!("opengl32.dll" "system" fn glPushMatrix());
    unsafe { glPushMatrix() }
}
#[inline]
pub unsafe fn glPushName(name: GLuint) {
    windows_core::link!("opengl32.dll" "system" fn glPushName(name : GLuint));
    unsafe { glPushName(name) }
}
#[inline]
pub unsafe fn glRasterPos2d(x: f64, y: f64) {
    windows_core::link!("opengl32.dll" "system" fn glRasterPos2d(x : f64, y : f64));
    unsafe { glRasterPos2d(x, y) }
}
#[inline]
pub unsafe fn glRasterPos2dv(v: *const f64) {
    windows_core::link!("opengl32.dll" "system" fn glRasterPos2dv(v : *const f64));
    unsafe { glRasterPos2dv(v) }
}
#[inline]
pub unsafe fn glRasterPos2f(x: f32, y: f32) {
    windows_core::link!("opengl32.dll" "system" fn glRasterPos2f(x : f32, y : f32));
    unsafe { glRasterPos2f(x, y) }
}
#[inline]
pub unsafe fn glRasterPos2fv(v: *const f32) {
    windows_core::link!("opengl32.dll" "system" fn glRasterPos2fv(v : *const f32));
    unsafe { glRasterPos2fv(v) }
}
#[inline]
pub unsafe fn glRasterPos2i(x: GLint, y: GLint) {
    windows_core::link!("opengl32.dll" "system" fn glRasterPos2i(x : GLint, y : GLint));
    unsafe { glRasterPos2i(x, y) }
}
#[inline]
pub unsafe fn glRasterPos2iv(v: *const GLint) {
    windows_core::link!("opengl32.dll" "system" fn glRasterPos2iv(v : *const GLint));
    unsafe { glRasterPos2iv(v) }
}
#[inline]
pub unsafe fn glRasterPos2s(x: GLshort, y: GLshort) {
    windows_core::link!("opengl32.dll" "system" fn glRasterPos2s(x : GLshort, y : GLshort));
    unsafe { glRasterPos2s(x, y) }
}
#[inline]
pub unsafe fn glRasterPos2sv(v: *const GLshort) {
    windows_core::link!("opengl32.dll" "system" fn glRasterPos2sv(v : *const GLshort));
    unsafe { glRasterPos2sv(v) }
}
#[inline]
pub unsafe fn glRasterPos3d(x: f64, y: f64, z: f64) {
    windows_core::link!("opengl32.dll" "system" fn glRasterPos3d(x : f64, y : f64, z : f64));
    unsafe { glRasterPos3d(x, y, z) }
}
#[inline]
pub unsafe fn glRasterPos3dv(v: *const f64) {
    windows_core::link!("opengl32.dll" "system" fn glRasterPos3dv(v : *const f64));
    unsafe { glRasterPos3dv(v) }
}
#[inline]
pub unsafe fn glRasterPos3f(x: f32, y: f32, z: f32) {
    windows_core::link!("opengl32.dll" "system" fn glRasterPos3f(x : f32, y : f32, z : f32));
    unsafe { glRasterPos3f(x, y, z) }
}
#[inline]
pub unsafe fn glRasterPos3fv(v: *const f32) {
    windows_core::link!("opengl32.dll" "system" fn glRasterPos3fv(v : *const f32));
    unsafe { glRasterPos3fv(v) }
}
#[inline]
pub unsafe fn glRasterPos3i(x: GLint, y: GLint, z: GLint) {
    windows_core::link!("opengl32.dll" "system" fn glRasterPos3i(x : GLint, y : GLint, z : GLint));
    unsafe { glRasterPos3i(x, y, z) }
}
#[inline]
pub unsafe fn glRasterPos3iv(v: *const GLint) {
    windows_core::link!("opengl32.dll" "system" fn glRasterPos3iv(v : *const GLint));
    unsafe { glRasterPos3iv(v) }
}
#[inline]
pub unsafe fn glRasterPos3s(x: GLshort, y: GLshort, z: GLshort) {
    windows_core::link!("opengl32.dll" "system" fn glRasterPos3s(x : GLshort, y : GLshort, z : GLshort));
    unsafe { glRasterPos3s(x, y, z) }
}
#[inline]
pub unsafe fn glRasterPos3sv(v: *const GLshort) {
    windows_core::link!("opengl32.dll" "system" fn glRasterPos3sv(v : *const GLshort));
    unsafe { glRasterPos3sv(v) }
}
#[inline]
pub unsafe fn glRasterPos4d(x: f64, y: f64, z: f64, w: f64) {
    windows_core::link!("opengl32.dll" "system" fn glRasterPos4d(x : f64, y : f64, z : f64, w : f64));
    unsafe { glRasterPos4d(x, y, z, w) }
}
#[inline]
pub unsafe fn glRasterPos4dv(v: *const f64) {
    windows_core::link!("opengl32.dll" "system" fn glRasterPos4dv(v : *const f64));
    unsafe { glRasterPos4dv(v) }
}
#[inline]
pub unsafe fn glRasterPos4f(x: f32, y: f32, z: f32, w: f32) {
    windows_core::link!("opengl32.dll" "system" fn glRasterPos4f(x : f32, y : f32, z : f32, w : f32));
    unsafe { glRasterPos4f(x, y, z, w) }
}
#[inline]
pub unsafe fn glRasterPos4fv(v: *const f32) {
    windows_core::link!("opengl32.dll" "system" fn glRasterPos4fv(v : *const f32));
    unsafe { glRasterPos4fv(v) }
}
#[inline]
pub unsafe fn glRasterPos4i(x: GLint, y: GLint, z: GLint, w: GLint) {
    windows_core::link!("opengl32.dll" "system" fn glRasterPos4i(x : GLint, y : GLint, z : GLint, w : GLint));
    unsafe { glRasterPos4i(x, y, z, w) }
}
#[inline]
pub unsafe fn glRasterPos4iv(v: *const GLint) {
    windows_core::link!("opengl32.dll" "system" fn glRasterPos4iv(v : *const GLint));
    unsafe { glRasterPos4iv(v) }
}
#[inline]
pub unsafe fn glRasterPos4s(x: GLshort, y: GLshort, z: GLshort, w: GLshort) {
    windows_core::link!("opengl32.dll" "system" fn glRasterPos4s(x : GLshort, y : GLshort, z : GLshort, w : GLshort));
    unsafe { glRasterPos4s(x, y, z, w) }
}
#[inline]
pub unsafe fn glRasterPos4sv(v: *const GLshort) {
    windows_core::link!("opengl32.dll" "system" fn glRasterPos4sv(v : *const GLshort));
    unsafe { glRasterPos4sv(v) }
}
#[inline]
pub unsafe fn glReadBuffer(mode: GLenum) {
    windows_core::link!("opengl32.dll" "system" fn glReadBuffer(mode : GLenum));
    unsafe { glReadBuffer(mode) }
}
#[inline]
pub unsafe fn glReadPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, r#type: GLenum) -> GLvoid {
    windows_core::link!("opengl32.dll" "system" fn glReadPixels(x : GLint, y : GLint, width : GLsizei, height : GLsizei, format : GLenum, r#type : GLenum, pixels : *mut GLvoid));
    unsafe {
        let mut result__ = core::mem::zeroed();
        glReadPixels(x, y, width, height, format, r#type, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn glRectd(x1: f64, y1: f64, x2: f64, y2: f64) {
    windows_core::link!("opengl32.dll" "system" fn glRectd(x1 : f64, y1 : f64, x2 : f64, y2 : f64));
    unsafe { glRectd(x1, y1, x2, y2) }
}
#[inline]
pub unsafe fn glRectdv(v1: *const f64, v2: *const f64) {
    windows_core::link!("opengl32.dll" "system" fn glRectdv(v1 : *const f64, v2 : *const f64));
    unsafe { glRectdv(v1, v2) }
}
#[inline]
pub unsafe fn glRectf(x1: f32, y1: f32, x2: f32, y2: f32) {
    windows_core::link!("opengl32.dll" "system" fn glRectf(x1 : f32, y1 : f32, x2 : f32, y2 : f32));
    unsafe { glRectf(x1, y1, x2, y2) }
}
#[inline]
pub unsafe fn glRectfv(v1: *const f32, v2: *const f32) {
    windows_core::link!("opengl32.dll" "system" fn glRectfv(v1 : *const f32, v2 : *const f32));
    unsafe { glRectfv(v1, v2) }
}
#[inline]
pub unsafe fn glRecti(x1: GLint, y1: GLint, x2: GLint, y2: GLint) {
    windows_core::link!("opengl32.dll" "system" fn glRecti(x1 : GLint, y1 : GLint, x2 : GLint, y2 : GLint));
    unsafe { glRecti(x1, y1, x2, y2) }
}
#[inline]
pub unsafe fn glRectiv(v1: *const GLint, v2: *const GLint) {
    windows_core::link!("opengl32.dll" "system" fn glRectiv(v1 : *const GLint, v2 : *const GLint));
    unsafe { glRectiv(v1, v2) }
}
#[inline]
pub unsafe fn glRects(x1: GLshort, y1: GLshort, x2: GLshort, y2: GLshort) {
    windows_core::link!("opengl32.dll" "system" fn glRects(x1 : GLshort, y1 : GLshort, x2 : GLshort, y2 : GLshort));
    unsafe { glRects(x1, y1, x2, y2) }
}
#[inline]
pub unsafe fn glRectsv(v1: *const GLshort, v2: *const GLshort) {
    windows_core::link!("opengl32.dll" "system" fn glRectsv(v1 : *const GLshort, v2 : *const GLshort));
    unsafe { glRectsv(v1, v2) }
}
#[inline]
pub unsafe fn glRenderMode(mode: GLenum) -> GLint {
    windows_core::link!("opengl32.dll" "system" fn glRenderMode(mode : GLenum) -> GLint);
    unsafe { glRenderMode(mode) }
}
#[inline]
pub unsafe fn glRotated(angle: f64, x: f64, y: f64, z: f64) {
    windows_core::link!("opengl32.dll" "system" fn glRotated(angle : f64, x : f64, y : f64, z : f64));
    unsafe { glRotated(angle, x, y, z) }
}
#[inline]
pub unsafe fn glRotatef(angle: f32, x: f32, y: f32, z: f32) {
    windows_core::link!("opengl32.dll" "system" fn glRotatef(angle : f32, x : f32, y : f32, z : f32));
    unsafe { glRotatef(angle, x, y, z) }
}
#[inline]
pub unsafe fn glScaled(x: f64, y: f64, z: f64) {
    windows_core::link!("opengl32.dll" "system" fn glScaled(x : f64, y : f64, z : f64));
    unsafe { glScaled(x, y, z) }
}
#[inline]
pub unsafe fn glScalef(x: f32, y: f32, z: f32) {
    windows_core::link!("opengl32.dll" "system" fn glScalef(x : f32, y : f32, z : f32));
    unsafe { glScalef(x, y, z) }
}
#[inline]
pub unsafe fn glScissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    windows_core::link!("opengl32.dll" "system" fn glScissor(x : GLint, y : GLint, width : GLsizei, height : GLsizei));
    unsafe { glScissor(x, y, width, height) }
}
#[inline]
pub unsafe fn glSelectBuffer(size: GLsizei) -> GLuint {
    windows_core::link!("opengl32.dll" "system" fn glSelectBuffer(size : GLsizei, buffer : *mut GLuint));
    unsafe {
        let mut result__ = core::mem::zeroed();
        glSelectBuffer(size, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn glShadeModel(mode: GLenum) {
    windows_core::link!("opengl32.dll" "system" fn glShadeModel(mode : GLenum));
    unsafe { glShadeModel(mode) }
}
#[inline]
pub unsafe fn glStencilFunc(func: GLenum, r#ref: GLint, mask: GLuint) {
    windows_core::link!("opengl32.dll" "system" fn glStencilFunc(func : GLenum, r#ref : GLint, mask : GLuint));
    unsafe { glStencilFunc(func, r#ref, mask) }
}
#[inline]
pub unsafe fn glStencilMask(mask: GLuint) {
    windows_core::link!("opengl32.dll" "system" fn glStencilMask(mask : GLuint));
    unsafe { glStencilMask(mask) }
}
#[inline]
pub unsafe fn glStencilOp(fail: GLenum, zfail: GLenum, zpass: GLenum) {
    windows_core::link!("opengl32.dll" "system" fn glStencilOp(fail : GLenum, zfail : GLenum, zpass : GLenum));
    unsafe { glStencilOp(fail, zfail, zpass) }
}
#[inline]
pub unsafe fn glTexCoord1d(s: f64) {
    windows_core::link!("opengl32.dll" "system" fn glTexCoord1d(s : f64));
    unsafe { glTexCoord1d(s) }
}
#[inline]
pub unsafe fn glTexCoord1dv(v: *const f64) {
    windows_core::link!("opengl32.dll" "system" fn glTexCoord1dv(v : *const f64));
    unsafe { glTexCoord1dv(v) }
}
#[inline]
pub unsafe fn glTexCoord1f(s: f32) {
    windows_core::link!("opengl32.dll" "system" fn glTexCoord1f(s : f32));
    unsafe { glTexCoord1f(s) }
}
#[inline]
pub unsafe fn glTexCoord1fv(v: *const f32) {
    windows_core::link!("opengl32.dll" "system" fn glTexCoord1fv(v : *const f32));
    unsafe { glTexCoord1fv(v) }
}
#[inline]
pub unsafe fn glTexCoord1i(s: GLint) {
    windows_core::link!("opengl32.dll" "system" fn glTexCoord1i(s : GLint));
    unsafe { glTexCoord1i(s) }
}
#[inline]
pub unsafe fn glTexCoord1iv(v: *const GLint) {
    windows_core::link!("opengl32.dll" "system" fn glTexCoord1iv(v : *const GLint));
    unsafe { glTexCoord1iv(v) }
}
#[inline]
pub unsafe fn glTexCoord1s(s: GLshort) {
    windows_core::link!("opengl32.dll" "system" fn glTexCoord1s(s : GLshort));
    unsafe { glTexCoord1s(s) }
}
#[inline]
pub unsafe fn glTexCoord1sv(v: *const GLshort) {
    windows_core::link!("opengl32.dll" "system" fn glTexCoord1sv(v : *const GLshort));
    unsafe { glTexCoord1sv(v) }
}
#[inline]
pub unsafe fn glTexCoord2d(s: f64, t: f64) {
    windows_core::link!("opengl32.dll" "system" fn glTexCoord2d(s : f64, t : f64));
    unsafe { glTexCoord2d(s, t) }
}
#[inline]
pub unsafe fn glTexCoord2dv(v: *const f64) {
    windows_core::link!("opengl32.dll" "system" fn glTexCoord2dv(v : *const f64));
    unsafe { glTexCoord2dv(v) }
}
#[inline]
pub unsafe fn glTexCoord2f(s: f32, t: f32) {
    windows_core::link!("opengl32.dll" "system" fn glTexCoord2f(s : f32, t : f32));
    unsafe { glTexCoord2f(s, t) }
}
#[inline]
pub unsafe fn glTexCoord2fv(v: *const f32) {
    windows_core::link!("opengl32.dll" "system" fn glTexCoord2fv(v : *const f32));
    unsafe { glTexCoord2fv(v) }
}
#[inline]
pub unsafe fn glTexCoord2i(s: GLint, t: GLint) {
    windows_core::link!("opengl32.dll" "system" fn glTexCoord2i(s : GLint, t : GLint));
    unsafe { glTexCoord2i(s, t) }
}
#[inline]
pub unsafe fn glTexCoord2iv(v: *const GLint) {
    windows_core::link!("opengl32.dll" "system" fn glTexCoord2iv(v : *const GLint));
    unsafe { glTexCoord2iv(v) }
}
#[inline]
pub unsafe fn glTexCoord2s(s: GLshort, t: GLshort) {
    windows_core::link!("opengl32.dll" "system" fn glTexCoord2s(s : GLshort, t : GLshort));
    unsafe { glTexCoord2s(s, t) }
}
#[inline]
pub unsafe fn glTexCoord2sv(v: *const GLshort) {
    windows_core::link!("opengl32.dll" "system" fn glTexCoord2sv(v : *const GLshort));
    unsafe { glTexCoord2sv(v) }
}
#[inline]
pub unsafe fn glTexCoord3d(s: f64, t: f64, r: f64) {
    windows_core::link!("opengl32.dll" "system" fn glTexCoord3d(s : f64, t : f64, r : f64));
    unsafe { glTexCoord3d(s, t, r) }
}
#[inline]
pub unsafe fn glTexCoord3dv(v: *const f64) {
    windows_core::link!("opengl32.dll" "system" fn glTexCoord3dv(v : *const f64));
    unsafe { glTexCoord3dv(v) }
}
#[inline]
pub unsafe fn glTexCoord3f(s: f32, t: f32, r: f32) {
    windows_core::link!("opengl32.dll" "system" fn glTexCoord3f(s : f32, t : f32, r : f32));
    unsafe { glTexCoord3f(s, t, r) }
}
#[inline]
pub unsafe fn glTexCoord3fv(v: *const f32) {
    windows_core::link!("opengl32.dll" "system" fn glTexCoord3fv(v : *const f32));
    unsafe { glTexCoord3fv(v) }
}
#[inline]
pub unsafe fn glTexCoord3i(s: GLint, t: GLint, r: GLint) {
    windows_core::link!("opengl32.dll" "system" fn glTexCoord3i(s : GLint, t : GLint, r : GLint));
    unsafe { glTexCoord3i(s, t, r) }
}
#[inline]
pub unsafe fn glTexCoord3iv(v: *const GLint) {
    windows_core::link!("opengl32.dll" "system" fn glTexCoord3iv(v : *const GLint));
    unsafe { glTexCoord3iv(v) }
}
#[inline]
pub unsafe fn glTexCoord3s(s: GLshort, t: GLshort, r: GLshort) {
    windows_core::link!("opengl32.dll" "system" fn glTexCoord3s(s : GLshort, t : GLshort, r : GLshort));
    unsafe { glTexCoord3s(s, t, r) }
}
#[inline]
pub unsafe fn glTexCoord3sv(v: *const GLshort) {
    windows_core::link!("opengl32.dll" "system" fn glTexCoord3sv(v : *const GLshort));
    unsafe { glTexCoord3sv(v) }
}
#[inline]
pub unsafe fn glTexCoord4d(s: f64, t: f64, r: f64, q: f64) {
    windows_core::link!("opengl32.dll" "system" fn glTexCoord4d(s : f64, t : f64, r : f64, q : f64));
    unsafe { glTexCoord4d(s, t, r, q) }
}
#[inline]
pub unsafe fn glTexCoord4dv(v: *const f64) {
    windows_core::link!("opengl32.dll" "system" fn glTexCoord4dv(v : *const f64));
    unsafe { glTexCoord4dv(v) }
}
#[inline]
pub unsafe fn glTexCoord4f(s: f32, t: f32, r: f32, q: f32) {
    windows_core::link!("opengl32.dll" "system" fn glTexCoord4f(s : f32, t : f32, r : f32, q : f32));
    unsafe { glTexCoord4f(s, t, r, q) }
}
#[inline]
pub unsafe fn glTexCoord4fv(v: *const f32) {
    windows_core::link!("opengl32.dll" "system" fn glTexCoord4fv(v : *const f32));
    unsafe { glTexCoord4fv(v) }
}
#[inline]
pub unsafe fn glTexCoord4i(s: GLint, t: GLint, r: GLint, q: GLint) {
    windows_core::link!("opengl32.dll" "system" fn glTexCoord4i(s : GLint, t : GLint, r : GLint, q : GLint));
    unsafe { glTexCoord4i(s, t, r, q) }
}
#[inline]
pub unsafe fn glTexCoord4iv(v: *const GLint) {
    windows_core::link!("opengl32.dll" "system" fn glTexCoord4iv(v : *const GLint));
    unsafe { glTexCoord4iv(v) }
}
#[inline]
pub unsafe fn glTexCoord4s(s: GLshort, t: GLshort, r: GLshort, q: GLshort) {
    windows_core::link!("opengl32.dll" "system" fn glTexCoord4s(s : GLshort, t : GLshort, r : GLshort, q : GLshort));
    unsafe { glTexCoord4s(s, t, r, q) }
}
#[inline]
pub unsafe fn glTexCoord4sv(v: *const GLshort) {
    windows_core::link!("opengl32.dll" "system" fn glTexCoord4sv(v : *const GLshort));
    unsafe { glTexCoord4sv(v) }
}
#[inline]
pub unsafe fn glTexCoordPointer(size: GLint, r#type: GLenum, stride: GLsizei, pointer: *const GLvoid) {
    windows_core::link!("opengl32.dll" "system" fn glTexCoordPointer(size : GLint, r#type : GLenum, stride : GLsizei, pointer : *const GLvoid));
    unsafe { glTexCoordPointer(size, r#type, stride, pointer) }
}
#[inline]
pub unsafe fn glTexEnvf(target: GLenum, pname: GLenum, param: f32) {
    windows_core::link!("opengl32.dll" "system" fn glTexEnvf(target : GLenum, pname : GLenum, param : f32));
    unsafe { glTexEnvf(target, pname, param) }
}
#[inline]
pub unsafe fn glTexEnvfv(target: GLenum, pname: GLenum, params: *const f32) {
    windows_core::link!("opengl32.dll" "system" fn glTexEnvfv(target : GLenum, pname : GLenum, params : *const f32));
    unsafe { glTexEnvfv(target, pname, params) }
}
#[inline]
pub unsafe fn glTexEnvi(target: GLenum, pname: GLenum, param: GLint) {
    windows_core::link!("opengl32.dll" "system" fn glTexEnvi(target : GLenum, pname : GLenum, param : GLint));
    unsafe { glTexEnvi(target, pname, param) }
}
#[inline]
pub unsafe fn glTexEnviv(target: GLenum, pname: GLenum, params: *const GLint) {
    windows_core::link!("opengl32.dll" "system" fn glTexEnviv(target : GLenum, pname : GLenum, params : *const GLint));
    unsafe { glTexEnviv(target, pname, params) }
}
#[inline]
pub unsafe fn glTexGend(coord: GLenum, pname: GLenum, param: f64) {
    windows_core::link!("opengl32.dll" "system" fn glTexGend(coord : GLenum, pname : GLenum, param : f64));
    unsafe { glTexGend(coord, pname, param) }
}
#[inline]
pub unsafe fn glTexGendv(coord: GLenum, pname: GLenum, params: *const f64) {
    windows_core::link!("opengl32.dll" "system" fn glTexGendv(coord : GLenum, pname : GLenum, params : *const f64));
    unsafe { glTexGendv(coord, pname, params) }
}
#[inline]
pub unsafe fn glTexGenf(coord: GLenum, pname: GLenum, param: f32) {
    windows_core::link!("opengl32.dll" "system" fn glTexGenf(coord : GLenum, pname : GLenum, param : f32));
    unsafe { glTexGenf(coord, pname, param) }
}
#[inline]
pub unsafe fn glTexGenfv(coord: GLenum, pname: GLenum, params: *const f32) {
    windows_core::link!("opengl32.dll" "system" fn glTexGenfv(coord : GLenum, pname : GLenum, params : *const f32));
    unsafe { glTexGenfv(coord, pname, params) }
}
#[inline]
pub unsafe fn glTexGeni(coord: GLenum, pname: GLenum, param: GLint) {
    windows_core::link!("opengl32.dll" "system" fn glTexGeni(coord : GLenum, pname : GLenum, param : GLint));
    unsafe { glTexGeni(coord, pname, param) }
}
#[inline]
pub unsafe fn glTexGeniv(coord: GLenum, pname: GLenum, params: *const GLint) {
    windows_core::link!("opengl32.dll" "system" fn glTexGeniv(coord : GLenum, pname : GLenum, params : *const GLint));
    unsafe { glTexGeniv(coord, pname, params) }
}
#[inline]
pub unsafe fn glTexImage1D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, r#type: GLenum, pixels: *const GLvoid) {
    windows_core::link!("opengl32.dll" "system" fn glTexImage1D(target : GLenum, level : GLint, internalformat : GLint, width : GLsizei, border : GLint, format : GLenum, r#type : GLenum, pixels : *const GLvoid));
    unsafe { glTexImage1D(target, level, internalformat, width, border, format, r#type, pixels) }
}
#[inline]
pub unsafe fn glTexImage2D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, r#type: GLenum, pixels: *const GLvoid) {
    windows_core::link!("opengl32.dll" "system" fn glTexImage2D(target : GLenum, level : GLint, internalformat : GLint, width : GLsizei, height : GLsizei, border : GLint, format : GLenum, r#type : GLenum, pixels : *const GLvoid));
    unsafe { glTexImage2D(target, level, internalformat, width, height, border, format, r#type, pixels) }
}
#[inline]
pub unsafe fn glTexParameterf(target: GLenum, pname: GLenum, param: f32) {
    windows_core::link!("opengl32.dll" "system" fn glTexParameterf(target : GLenum, pname : GLenum, param : f32));
    unsafe { glTexParameterf(target, pname, param) }
}
#[inline]
pub unsafe fn glTexParameterfv(target: GLenum, pname: GLenum, params: *const f32) {
    windows_core::link!("opengl32.dll" "system" fn glTexParameterfv(target : GLenum, pname : GLenum, params : *const f32));
    unsafe { glTexParameterfv(target, pname, params) }
}
#[inline]
pub unsafe fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint) {
    windows_core::link!("opengl32.dll" "system" fn glTexParameteri(target : GLenum, pname : GLenum, param : GLint));
    unsafe { glTexParameteri(target, pname, param) }
}
#[inline]
pub unsafe fn glTexParameteriv(target: GLenum, pname: GLenum, params: *const GLint) {
    windows_core::link!("opengl32.dll" "system" fn glTexParameteriv(target : GLenum, pname : GLenum, params : *const GLint));
    unsafe { glTexParameteriv(target, pname, params) }
}
#[inline]
pub unsafe fn glTexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, r#type: GLenum, pixels: *const GLvoid) {
    windows_core::link!("opengl32.dll" "system" fn glTexSubImage1D(target : GLenum, level : GLint, xoffset : GLint, width : GLsizei, format : GLenum, r#type : GLenum, pixels : *const GLvoid));
    unsafe { glTexSubImage1D(target, level, xoffset, width, format, r#type, pixels) }
}
#[inline]
pub unsafe fn glTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, r#type: GLenum, pixels: *const GLvoid) {
    windows_core::link!("opengl32.dll" "system" fn glTexSubImage2D(target : GLenum, level : GLint, xoffset : GLint, yoffset : GLint, width : GLsizei, height : GLsizei, format : GLenum, r#type : GLenum, pixels : *const GLvoid));
    unsafe { glTexSubImage2D(target, level, xoffset, yoffset, width, height, format, r#type, pixels) }
}
#[inline]
pub unsafe fn glTranslated(x: f64, y: f64, z: f64) {
    windows_core::link!("opengl32.dll" "system" fn glTranslated(x : f64, y : f64, z : f64));
    unsafe { glTranslated(x, y, z) }
}
#[inline]
pub unsafe fn glTranslatef(x: f32, y: f32, z: f32) {
    windows_core::link!("opengl32.dll" "system" fn glTranslatef(x : f32, y : f32, z : f32));
    unsafe { glTranslatef(x, y, z) }
}
#[inline]
pub unsafe fn glVertex2d(x: f64, y: f64) {
    windows_core::link!("opengl32.dll" "system" fn glVertex2d(x : f64, y : f64));
    unsafe { glVertex2d(x, y) }
}
#[inline]
pub unsafe fn glVertex2dv(v: *const f64) {
    windows_core::link!("opengl32.dll" "system" fn glVertex2dv(v : *const f64));
    unsafe { glVertex2dv(v) }
}
#[inline]
pub unsafe fn glVertex2f(x: f32, y: f32) {
    windows_core::link!("opengl32.dll" "system" fn glVertex2f(x : f32, y : f32));
    unsafe { glVertex2f(x, y) }
}
#[inline]
pub unsafe fn glVertex2fv(v: *const f32) {
    windows_core::link!("opengl32.dll" "system" fn glVertex2fv(v : *const f32));
    unsafe { glVertex2fv(v) }
}
#[inline]
pub unsafe fn glVertex2i(x: GLint, y: GLint) {
    windows_core::link!("opengl32.dll" "system" fn glVertex2i(x : GLint, y : GLint));
    unsafe { glVertex2i(x, y) }
}
#[inline]
pub unsafe fn glVertex2iv(v: *const GLint) {
    windows_core::link!("opengl32.dll" "system" fn glVertex2iv(v : *const GLint));
    unsafe { glVertex2iv(v) }
}
#[inline]
pub unsafe fn glVertex2s(x: GLshort, y: GLshort) {
    windows_core::link!("opengl32.dll" "system" fn glVertex2s(x : GLshort, y : GLshort));
    unsafe { glVertex2s(x, y) }
}
#[inline]
pub unsafe fn glVertex2sv(v: *const GLshort) {
    windows_core::link!("opengl32.dll" "system" fn glVertex2sv(v : *const GLshort));
    unsafe { glVertex2sv(v) }
}
#[inline]
pub unsafe fn glVertex3d(x: f64, y: f64, z: f64) {
    windows_core::link!("opengl32.dll" "system" fn glVertex3d(x : f64, y : f64, z : f64));
    unsafe { glVertex3d(x, y, z) }
}
#[inline]
pub unsafe fn glVertex3dv(v: *const f64) {
    windows_core::link!("opengl32.dll" "system" fn glVertex3dv(v : *const f64));
    unsafe { glVertex3dv(v) }
}
#[inline]
pub unsafe fn glVertex3f(x: f32, y: f32, z: f32) {
    windows_core::link!("opengl32.dll" "system" fn glVertex3f(x : f32, y : f32, z : f32));
    unsafe { glVertex3f(x, y, z) }
}
#[inline]
pub unsafe fn glVertex3fv(v: *const f32) {
    windows_core::link!("opengl32.dll" "system" fn glVertex3fv(v : *const f32));
    unsafe { glVertex3fv(v) }
}
#[inline]
pub unsafe fn glVertex3i(x: GLint, y: GLint, z: GLint) {
    windows_core::link!("opengl32.dll" "system" fn glVertex3i(x : GLint, y : GLint, z : GLint));
    unsafe { glVertex3i(x, y, z) }
}
#[inline]
pub unsafe fn glVertex3iv(v: *const GLint) {
    windows_core::link!("opengl32.dll" "system" fn glVertex3iv(v : *const GLint));
    unsafe { glVertex3iv(v) }
}
#[inline]
pub unsafe fn glVertex3s(x: GLshort, y: GLshort, z: GLshort) {
    windows_core::link!("opengl32.dll" "system" fn glVertex3s(x : GLshort, y : GLshort, z : GLshort));
    unsafe { glVertex3s(x, y, z) }
}
#[inline]
pub unsafe fn glVertex3sv(v: *const GLshort) {
    windows_core::link!("opengl32.dll" "system" fn glVertex3sv(v : *const GLshort));
    unsafe { glVertex3sv(v) }
}
#[inline]
pub unsafe fn glVertex4d(x: f64, y: f64, z: f64, w: f64) {
    windows_core::link!("opengl32.dll" "system" fn glVertex4d(x : f64, y : f64, z : f64, w : f64));
    unsafe { glVertex4d(x, y, z, w) }
}
#[inline]
pub unsafe fn glVertex4dv(v: *const f64) {
    windows_core::link!("opengl32.dll" "system" fn glVertex4dv(v : *const f64));
    unsafe { glVertex4dv(v) }
}
#[inline]
pub unsafe fn glVertex4f(x: f32, y: f32, z: f32, w: f32) {
    windows_core::link!("opengl32.dll" "system" fn glVertex4f(x : f32, y : f32, z : f32, w : f32));
    unsafe { glVertex4f(x, y, z, w) }
}
#[inline]
pub unsafe fn glVertex4fv(v: *const f32) {
    windows_core::link!("opengl32.dll" "system" fn glVertex4fv(v : *const f32));
    unsafe { glVertex4fv(v) }
}
#[inline]
pub unsafe fn glVertex4i(x: GLint, y: GLint, z: GLint, w: GLint) {
    windows_core::link!("opengl32.dll" "system" fn glVertex4i(x : GLint, y : GLint, z : GLint, w : GLint));
    unsafe { glVertex4i(x, y, z, w) }
}
#[inline]
pub unsafe fn glVertex4iv(v: *const GLint) {
    windows_core::link!("opengl32.dll" "system" fn glVertex4iv(v : *const GLint));
    unsafe { glVertex4iv(v) }
}
#[inline]
pub unsafe fn glVertex4s(x: GLshort, y: GLshort, z: GLshort, w: GLshort) {
    windows_core::link!("opengl32.dll" "system" fn glVertex4s(x : GLshort, y : GLshort, z : GLshort, w : GLshort));
    unsafe { glVertex4s(x, y, z, w) }
}
#[inline]
pub unsafe fn glVertex4sv(v: *const GLshort) {
    windows_core::link!("opengl32.dll" "system" fn glVertex4sv(v : *const GLshort));
    unsafe { glVertex4sv(v) }
}
#[inline]
pub unsafe fn glVertexPointer(size: GLint, r#type: GLenum, stride: GLsizei, pointer: *const GLvoid) {
    windows_core::link!("opengl32.dll" "system" fn glVertexPointer(size : GLint, r#type : GLenum, stride : GLsizei, pointer : *const GLvoid));
    unsafe { glVertexPointer(size, r#type, stride, pointer) }
}
#[inline]
pub unsafe fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    windows_core::link!("opengl32.dll" "system" fn glViewport(x : GLint, y : GLint, width : GLsizei, height : GLsizei));
    unsafe { glViewport(x, y, width, height) }
}
pub const GL_2D: u32 = 1536;
pub const GL_2_BYTES: u32 = 5127;
pub const GL_3D: u32 = 1537;
pub const GL_3D_COLOR: u32 = 1538;
pub const GL_3D_COLOR_TEXTURE: u32 = 1539;
pub const GL_3_BYTES: u32 = 5128;
pub const GL_4D_COLOR_TEXTURE: u32 = 1540;
pub const GL_4_BYTES: u32 = 5129;
pub const GL_ACCUM: u32 = 256;
pub const GL_ACCUM_ALPHA_BITS: u32 = 3419;
pub const GL_ACCUM_BLUE_BITS: u32 = 3418;
pub const GL_ACCUM_BUFFER_BIT: u32 = 512;
pub const GL_ACCUM_CLEAR_VALUE: u32 = 2944;
pub const GL_ACCUM_GREEN_BITS: u32 = 3417;
pub const GL_ACCUM_RED_BITS: u32 = 3416;
pub const GL_ADD: u32 = 260;
pub const GL_ALL_ATTRIB_BITS: u32 = 1048575;
pub const GL_ALPHA: u32 = 6406;
pub const GL_ALPHA12: u32 = 32829;
pub const GL_ALPHA16: u32 = 32830;
pub const GL_ALPHA4: u32 = 32827;
pub const GL_ALPHA8: u32 = 32828;
pub const GL_ALPHA_BIAS: u32 = 3357;
pub const GL_ALPHA_BITS: u32 = 3413;
pub const GL_ALPHA_SCALE: u32 = 3356;
pub const GL_ALPHA_TEST: u32 = 3008;
pub const GL_ALPHA_TEST_FUNC: u32 = 3009;
pub const GL_ALPHA_TEST_REF: u32 = 3010;
pub const GL_ALWAYS: u32 = 519;
pub const GL_AMBIENT: u32 = 4608;
pub const GL_AMBIENT_AND_DIFFUSE: u32 = 5634;
pub const GL_AND: u32 = 5377;
pub const GL_AND_INVERTED: u32 = 5380;
pub const GL_AND_REVERSE: u32 = 5378;
pub const GL_ATTRIB_STACK_DEPTH: u32 = 2992;
pub const GL_AUTO_NORMAL: u32 = 3456;
pub const GL_AUX0: u32 = 1033;
pub const GL_AUX1: u32 = 1034;
pub const GL_AUX2: u32 = 1035;
pub const GL_AUX3: u32 = 1036;
pub const GL_AUX_BUFFERS: u32 = 3072;
pub const GL_BACK: u32 = 1029;
pub const GL_BACK_LEFT: u32 = 1026;
pub const GL_BACK_RIGHT: u32 = 1027;
pub const GL_BGRA_EXT: u32 = 32993;
pub const GL_BGR_EXT: u32 = 32992;
pub const GL_BITMAP: u32 = 6656;
pub const GL_BITMAP_TOKEN: u32 = 1796;
pub const GL_BLEND: u32 = 3042;
pub const GL_BLEND_DST: u32 = 3040;
pub const GL_BLEND_SRC: u32 = 3041;
pub const GL_BLUE: u32 = 6405;
pub const GL_BLUE_BIAS: u32 = 3355;
pub const GL_BLUE_BITS: u32 = 3412;
pub const GL_BLUE_SCALE: u32 = 3354;
pub const GL_BYTE: u32 = 5120;
pub const GL_C3F_V3F: u32 = 10788;
pub const GL_C4F_N3F_V3F: u32 = 10790;
pub const GL_C4UB_V2F: u32 = 10786;
pub const GL_C4UB_V3F: u32 = 10787;
pub const GL_CCW: u32 = 2305;
pub const GL_CLAMP: u32 = 10496;
pub const GL_CLEAR: u32 = 5376;
pub const GL_CLIENT_ALL_ATTRIB_BITS: u32 = 4294967295;
pub const GL_CLIENT_ATTRIB_STACK_DEPTH: u32 = 2993;
pub const GL_CLIENT_PIXEL_STORE_BIT: u32 = 1;
pub const GL_CLIENT_VERTEX_ARRAY_BIT: u32 = 2;
pub const GL_CLIP_PLANE0: u32 = 12288;
pub const GL_CLIP_PLANE1: u32 = 12289;
pub const GL_CLIP_PLANE2: u32 = 12290;
pub const GL_CLIP_PLANE3: u32 = 12291;
pub const GL_CLIP_PLANE4: u32 = 12292;
pub const GL_CLIP_PLANE5: u32 = 12293;
pub const GL_COEFF: u32 = 2560;
pub const GL_COLOR: u32 = 6144;
pub const GL_COLOR_ARRAY: u32 = 32886;
pub const GL_COLOR_ARRAY_COUNT_EXT: u32 = 32900;
pub const GL_COLOR_ARRAY_EXT: u32 = 32886;
pub const GL_COLOR_ARRAY_POINTER: u32 = 32912;
pub const GL_COLOR_ARRAY_POINTER_EXT: u32 = 32912;
pub const GL_COLOR_ARRAY_SIZE: u32 = 32897;
pub const GL_COLOR_ARRAY_SIZE_EXT: u32 = 32897;
pub const GL_COLOR_ARRAY_STRIDE: u32 = 32899;
pub const GL_COLOR_ARRAY_STRIDE_EXT: u32 = 32899;
pub const GL_COLOR_ARRAY_TYPE: u32 = 32898;
pub const GL_COLOR_ARRAY_TYPE_EXT: u32 = 32898;
pub const GL_COLOR_BUFFER_BIT: u32 = 16384;
pub const GL_COLOR_CLEAR_VALUE: u32 = 3106;
pub const GL_COLOR_INDEX: u32 = 6400;
pub const GL_COLOR_INDEX12_EXT: u32 = 32998;
pub const GL_COLOR_INDEX16_EXT: u32 = 32999;
pub const GL_COLOR_INDEX1_EXT: u32 = 32994;
pub const GL_COLOR_INDEX2_EXT: u32 = 32995;
pub const GL_COLOR_INDEX4_EXT: u32 = 32996;
pub const GL_COLOR_INDEX8_EXT: u32 = 32997;
pub const GL_COLOR_INDEXES: u32 = 5635;
pub const GL_COLOR_LOGIC_OP: u32 = 3058;
pub const GL_COLOR_MATERIAL: u32 = 2903;
pub const GL_COLOR_MATERIAL_FACE: u32 = 2901;
pub const GL_COLOR_MATERIAL_PARAMETER: u32 = 2902;
pub const GL_COLOR_TABLE_ALPHA_SIZE_EXT: u32 = 32989;
pub const GL_COLOR_TABLE_BLUE_SIZE_EXT: u32 = 32988;
pub const GL_COLOR_TABLE_FORMAT_EXT: u32 = 32984;
pub const GL_COLOR_TABLE_GREEN_SIZE_EXT: u32 = 32987;
pub const GL_COLOR_TABLE_INTENSITY_SIZE_EXT: u32 = 32991;
pub const GL_COLOR_TABLE_LUMINANCE_SIZE_EXT: u32 = 32990;
pub const GL_COLOR_TABLE_RED_SIZE_EXT: u32 = 32986;
pub const GL_COLOR_TABLE_WIDTH_EXT: u32 = 32985;
pub const GL_COLOR_WRITEMASK: u32 = 3107;
pub const GL_COMPILE: u32 = 4864;
pub const GL_COMPILE_AND_EXECUTE: u32 = 4865;
pub const GL_CONSTANT_ATTENUATION: u32 = 4615;
pub const GL_COPY: u32 = 5379;
pub const GL_COPY_INVERTED: u32 = 5388;
pub const GL_COPY_PIXEL_TOKEN: u32 = 1798;
pub const GL_CULL_FACE: u32 = 2884;
pub const GL_CULL_FACE_MODE: u32 = 2885;
pub const GL_CURRENT_BIT: u32 = 1;
pub const GL_CURRENT_COLOR: u32 = 2816;
pub const GL_CURRENT_INDEX: u32 = 2817;
pub const GL_CURRENT_NORMAL: u32 = 2818;
pub const GL_CURRENT_RASTER_COLOR: u32 = 2820;
pub const GL_CURRENT_RASTER_DISTANCE: u32 = 2825;
pub const GL_CURRENT_RASTER_INDEX: u32 = 2821;
pub const GL_CURRENT_RASTER_POSITION: u32 = 2823;
pub const GL_CURRENT_RASTER_POSITION_VALID: u32 = 2824;
pub const GL_CURRENT_RASTER_TEXTURE_COORDS: u32 = 2822;
pub const GL_CURRENT_TEXTURE_COORDS: u32 = 2819;
pub const GL_CW: u32 = 2304;
pub const GL_DECAL: u32 = 8449;
pub const GL_DECR: u32 = 7683;
pub const GL_DEPTH: u32 = 6145;
pub const GL_DEPTH_BIAS: u32 = 3359;
pub const GL_DEPTH_BITS: u32 = 3414;
pub const GL_DEPTH_BUFFER_BIT: u32 = 256;
pub const GL_DEPTH_CLEAR_VALUE: u32 = 2931;
pub const GL_DEPTH_COMPONENT: u32 = 6402;
pub const GL_DEPTH_FUNC: u32 = 2932;
pub const GL_DEPTH_RANGE: u32 = 2928;
pub const GL_DEPTH_SCALE: u32 = 3358;
pub const GL_DEPTH_TEST: u32 = 2929;
pub const GL_DEPTH_WRITEMASK: u32 = 2930;
pub const GL_DIFFUSE: u32 = 4609;
pub const GL_DITHER: u32 = 3024;
pub const GL_DOMAIN: u32 = 2562;
pub const GL_DONT_CARE: u32 = 4352;
pub const GL_DOUBLE: u32 = 5130;
pub const GL_DOUBLEBUFFER: u32 = 3122;
pub const GL_DOUBLE_EXT: u32 = 5130;
pub const GL_DRAW_BUFFER: u32 = 3073;
pub const GL_DRAW_PIXEL_TOKEN: u32 = 1797;
pub const GL_DST_ALPHA: u32 = 772;
pub const GL_DST_COLOR: u32 = 774;
pub const GL_EDGE_FLAG: u32 = 2883;
pub const GL_EDGE_FLAG_ARRAY: u32 = 32889;
pub const GL_EDGE_FLAG_ARRAY_COUNT_EXT: u32 = 32909;
pub const GL_EDGE_FLAG_ARRAY_EXT: u32 = 32889;
pub const GL_EDGE_FLAG_ARRAY_POINTER: u32 = 32915;
pub const GL_EDGE_FLAG_ARRAY_POINTER_EXT: u32 = 32915;
pub const GL_EDGE_FLAG_ARRAY_STRIDE: u32 = 32908;
pub const GL_EDGE_FLAG_ARRAY_STRIDE_EXT: u32 = 32908;
pub const GL_EMISSION: u32 = 5632;
pub const GL_ENABLE_BIT: u32 = 8192;
pub const GL_EQUAL: u32 = 514;
pub const GL_EQUIV: u32 = 5385;
pub const GL_EVAL_BIT: u32 = 65536;
pub const GL_EXP: u32 = 2048;
pub const GL_EXP2: u32 = 2049;
pub const GL_EXTENSIONS: u32 = 7939;
pub const GL_EXT_bgra: u32 = 1;
pub const GL_EXT_paletted_texture: u32 = 1;
pub const GL_EXT_vertex_array: u32 = 1;
pub const GL_EYE_LINEAR: u32 = 9216;
pub const GL_EYE_PLANE: u32 = 9474;
pub const GL_FALSE: u32 = 0;
pub const GL_FASTEST: u32 = 4353;
pub const GL_FEEDBACK: u32 = 7169;
pub const GL_FEEDBACK_BUFFER_POINTER: u32 = 3568;
pub const GL_FEEDBACK_BUFFER_SIZE: u32 = 3569;
pub const GL_FEEDBACK_BUFFER_TYPE: u32 = 3570;
pub const GL_FILL: u32 = 6914;
pub const GL_FLAT: u32 = 7424;
pub const GL_FLOAT: u32 = 5126;
pub const GL_FOG: u32 = 2912;
pub const GL_FOG_BIT: u32 = 128;
pub const GL_FOG_COLOR: u32 = 2918;
pub const GL_FOG_DENSITY: u32 = 2914;
pub const GL_FOG_END: u32 = 2916;
pub const GL_FOG_HINT: u32 = 3156;
pub const GL_FOG_INDEX: u32 = 2913;
pub const GL_FOG_MODE: u32 = 2917;
pub const GL_FOG_SPECULAR_TEXTURE_WIN: u32 = 33004;
pub const GL_FOG_START: u32 = 2915;
pub const GL_FRONT: u32 = 1028;
pub const GL_FRONT_AND_BACK: u32 = 1032;
pub const GL_FRONT_FACE: u32 = 2886;
pub const GL_FRONT_LEFT: u32 = 1024;
pub const GL_FRONT_RIGHT: u32 = 1025;
pub const GL_GEQUAL: u32 = 518;
pub const GL_GREATER: u32 = 516;
pub const GL_GREEN: u32 = 6404;
pub const GL_GREEN_BIAS: u32 = 3353;
pub const GL_GREEN_BITS: u32 = 3411;
pub const GL_GREEN_SCALE: u32 = 3352;
pub const GL_HINT_BIT: u32 = 32768;
pub const GL_INCR: u32 = 7682;
pub const GL_INDEX_ARRAY: u32 = 32887;
pub const GL_INDEX_ARRAY_COUNT_EXT: u32 = 32903;
pub const GL_INDEX_ARRAY_EXT: u32 = 32887;
pub const GL_INDEX_ARRAY_POINTER: u32 = 32913;
pub const GL_INDEX_ARRAY_POINTER_EXT: u32 = 32913;
pub const GL_INDEX_ARRAY_STRIDE: u32 = 32902;
pub const GL_INDEX_ARRAY_STRIDE_EXT: u32 = 32902;
pub const GL_INDEX_ARRAY_TYPE: u32 = 32901;
pub const GL_INDEX_ARRAY_TYPE_EXT: u32 = 32901;
pub const GL_INDEX_BITS: u32 = 3409;
pub const GL_INDEX_CLEAR_VALUE: u32 = 3104;
pub const GL_INDEX_LOGIC_OP: u32 = 3057;
pub const GL_INDEX_MODE: u32 = 3120;
pub const GL_INDEX_OFFSET: u32 = 3347;
pub const GL_INDEX_SHIFT: u32 = 3346;
pub const GL_INDEX_WRITEMASK: u32 = 3105;
pub const GL_INT: u32 = 5124;
pub const GL_INTENSITY: u32 = 32841;
pub const GL_INTENSITY12: u32 = 32844;
pub const GL_INTENSITY16: u32 = 32845;
pub const GL_INTENSITY4: u32 = 32842;
pub const GL_INTENSITY8: u32 = 32843;
pub const GL_INVALID_ENUM: u32 = 1280;
pub const GL_INVALID_OPERATION: u32 = 1282;
pub const GL_INVALID_VALUE: u32 = 1281;
pub const GL_INVERT: u32 = 5386;
pub const GL_KEEP: u32 = 7680;
pub const GL_LEFT: u32 = 1030;
pub const GL_LEQUAL: u32 = 515;
pub const GL_LESS: u32 = 513;
pub const GL_LIGHT0: u32 = 16384;
pub const GL_LIGHT1: u32 = 16385;
pub const GL_LIGHT2: u32 = 16386;
pub const GL_LIGHT3: u32 = 16387;
pub const GL_LIGHT4: u32 = 16388;
pub const GL_LIGHT5: u32 = 16389;
pub const GL_LIGHT6: u32 = 16390;
pub const GL_LIGHT7: u32 = 16391;
pub const GL_LIGHTING: u32 = 2896;
pub const GL_LIGHTING_BIT: u32 = 64;
pub const GL_LIGHT_MODEL_AMBIENT: u32 = 2899;
pub const GL_LIGHT_MODEL_LOCAL_VIEWER: u32 = 2897;
pub const GL_LIGHT_MODEL_TWO_SIDE: u32 = 2898;
pub const GL_LINE: u32 = 6913;
pub const GL_LINEAR: u32 = 9729;
pub const GL_LINEAR_ATTENUATION: u32 = 4616;
pub const GL_LINEAR_MIPMAP_LINEAR: u32 = 9987;
pub const GL_LINEAR_MIPMAP_NEAREST: u32 = 9985;
pub const GL_LINES: u32 = 1;
pub const GL_LINE_BIT: u32 = 4;
pub const GL_LINE_LOOP: u32 = 2;
pub const GL_LINE_RESET_TOKEN: u32 = 1799;
pub const GL_LINE_SMOOTH: u32 = 2848;
pub const GL_LINE_SMOOTH_HINT: u32 = 3154;
pub const GL_LINE_STIPPLE: u32 = 2852;
pub const GL_LINE_STIPPLE_PATTERN: u32 = 2853;
pub const GL_LINE_STIPPLE_REPEAT: u32 = 2854;
pub const GL_LINE_STRIP: u32 = 3;
pub const GL_LINE_TOKEN: u32 = 1794;
pub const GL_LINE_WIDTH: u32 = 2849;
pub const GL_LINE_WIDTH_GRANULARITY: u32 = 2851;
pub const GL_LINE_WIDTH_RANGE: u32 = 2850;
pub const GL_LIST_BASE: u32 = 2866;
pub const GL_LIST_BIT: u32 = 131072;
pub const GL_LIST_INDEX: u32 = 2867;
pub const GL_LIST_MODE: u32 = 2864;
pub const GL_LOAD: u32 = 257;
pub const GL_LOGIC_OP: u32 = 3057;
pub const GL_LOGIC_OP_MODE: u32 = 3056;
pub const GL_LUMINANCE: u32 = 6409;
pub const GL_LUMINANCE12: u32 = 32833;
pub const GL_LUMINANCE12_ALPHA12: u32 = 32839;
pub const GL_LUMINANCE12_ALPHA4: u32 = 32838;
pub const GL_LUMINANCE16: u32 = 32834;
pub const GL_LUMINANCE16_ALPHA16: u32 = 32840;
pub const GL_LUMINANCE4: u32 = 32831;
pub const GL_LUMINANCE4_ALPHA4: u32 = 32835;
pub const GL_LUMINANCE6_ALPHA2: u32 = 32836;
pub const GL_LUMINANCE8: u32 = 32832;
pub const GL_LUMINANCE8_ALPHA8: u32 = 32837;
pub const GL_LUMINANCE_ALPHA: u32 = 6410;
pub const GL_MAP1_COLOR_4: u32 = 3472;
pub const GL_MAP1_GRID_DOMAIN: u32 = 3536;
pub const GL_MAP1_GRID_SEGMENTS: u32 = 3537;
pub const GL_MAP1_INDEX: u32 = 3473;
pub const GL_MAP1_NORMAL: u32 = 3474;
pub const GL_MAP1_TEXTURE_COORD_1: u32 = 3475;
pub const GL_MAP1_TEXTURE_COORD_2: u32 = 3476;
pub const GL_MAP1_TEXTURE_COORD_3: u32 = 3477;
pub const GL_MAP1_TEXTURE_COORD_4: u32 = 3478;
pub const GL_MAP1_VERTEX_3: u32 = 3479;
pub const GL_MAP1_VERTEX_4: u32 = 3480;
pub const GL_MAP2_COLOR_4: u32 = 3504;
pub const GL_MAP2_GRID_DOMAIN: u32 = 3538;
pub const GL_MAP2_GRID_SEGMENTS: u32 = 3539;
pub const GL_MAP2_INDEX: u32 = 3505;
pub const GL_MAP2_NORMAL: u32 = 3506;
pub const GL_MAP2_TEXTURE_COORD_1: u32 = 3507;
pub const GL_MAP2_TEXTURE_COORD_2: u32 = 3508;
pub const GL_MAP2_TEXTURE_COORD_3: u32 = 3509;
pub const GL_MAP2_TEXTURE_COORD_4: u32 = 3510;
pub const GL_MAP2_VERTEX_3: u32 = 3511;
pub const GL_MAP2_VERTEX_4: u32 = 3512;
pub const GL_MAP_COLOR: u32 = 3344;
pub const GL_MAP_STENCIL: u32 = 3345;
pub const GL_MATRIX_MODE: u32 = 2976;
pub const GL_MAX_ATTRIB_STACK_DEPTH: u32 = 3381;
pub const GL_MAX_CLIENT_ATTRIB_STACK_DEPTH: u32 = 3387;
pub const GL_MAX_CLIP_PLANES: u32 = 3378;
pub const GL_MAX_ELEMENTS_INDICES_WIN: u32 = 33001;
pub const GL_MAX_ELEMENTS_VERTICES_WIN: u32 = 33000;
pub const GL_MAX_EVAL_ORDER: u32 = 3376;
pub const GL_MAX_LIGHTS: u32 = 3377;
pub const GL_MAX_LIST_NESTING: u32 = 2865;
pub const GL_MAX_MODELVIEW_STACK_DEPTH: u32 = 3382;
pub const GL_MAX_NAME_STACK_DEPTH: u32 = 3383;
pub const GL_MAX_PIXEL_MAP_TABLE: u32 = 3380;
pub const GL_MAX_PROJECTION_STACK_DEPTH: u32 = 3384;
pub const GL_MAX_TEXTURE_SIZE: u32 = 3379;
pub const GL_MAX_TEXTURE_STACK_DEPTH: u32 = 3385;
pub const GL_MAX_VIEWPORT_DIMS: u32 = 3386;
pub const GL_MODELVIEW: u32 = 5888;
pub const GL_MODELVIEW_MATRIX: u32 = 2982;
pub const GL_MODELVIEW_STACK_DEPTH: u32 = 2979;
pub const GL_MODULATE: u32 = 8448;
pub const GL_MULT: u32 = 259;
pub const GL_N3F_V3F: u32 = 10789;
pub const GL_NAME_STACK_DEPTH: u32 = 3440;
pub const GL_NAND: u32 = 5390;
pub const GL_NEAREST: u32 = 9728;
pub const GL_NEAREST_MIPMAP_LINEAR: u32 = 9986;
pub const GL_NEAREST_MIPMAP_NEAREST: u32 = 9984;
pub const GL_NEVER: u32 = 512;
pub const GL_NICEST: u32 = 4354;
pub const GL_NONE: u32 = 0;
pub const GL_NOOP: u32 = 5381;
pub const GL_NOR: u32 = 5384;
pub const GL_NORMALIZE: u32 = 2977;
pub const GL_NORMAL_ARRAY: u32 = 32885;
pub const GL_NORMAL_ARRAY_COUNT_EXT: u32 = 32896;
pub const GL_NORMAL_ARRAY_EXT: u32 = 32885;
pub const GL_NORMAL_ARRAY_POINTER: u32 = 32911;
pub const GL_NORMAL_ARRAY_POINTER_EXT: u32 = 32911;
pub const GL_NORMAL_ARRAY_STRIDE: u32 = 32895;
pub const GL_NORMAL_ARRAY_STRIDE_EXT: u32 = 32895;
pub const GL_NORMAL_ARRAY_TYPE: u32 = 32894;
pub const GL_NORMAL_ARRAY_TYPE_EXT: u32 = 32894;
pub const GL_NOTEQUAL: u32 = 517;
pub const GL_NO_ERROR: u32 = 0;
pub const GL_OBJECT_LINEAR: u32 = 9217;
pub const GL_OBJECT_PLANE: u32 = 9473;
pub const GL_ONE: u32 = 1;
pub const GL_ONE_MINUS_DST_ALPHA: u32 = 773;
pub const GL_ONE_MINUS_DST_COLOR: u32 = 775;
pub const GL_ONE_MINUS_SRC_ALPHA: u32 = 771;
pub const GL_ONE_MINUS_SRC_COLOR: u32 = 769;
pub const GL_OR: u32 = 5383;
pub const GL_ORDER: u32 = 2561;
pub const GL_OR_INVERTED: u32 = 5389;
pub const GL_OR_REVERSE: u32 = 5387;
pub const GL_OUT_OF_MEMORY: u32 = 1285;
pub const GL_PACK_ALIGNMENT: u32 = 3333;
pub const GL_PACK_LSB_FIRST: u32 = 3329;
pub const GL_PACK_ROW_LENGTH: u32 = 3330;
pub const GL_PACK_SKIP_PIXELS: u32 = 3332;
pub const GL_PACK_SKIP_ROWS: u32 = 3331;
pub const GL_PACK_SWAP_BYTES: u32 = 3328;
pub const GL_PASS_THROUGH_TOKEN: u32 = 1792;
pub const GL_PERSPECTIVE_CORRECTION_HINT: u32 = 3152;
pub const GL_PHONG_HINT_WIN: u32 = 33003;
pub const GL_PHONG_WIN: u32 = 33002;
pub const GL_PIXEL_MAP_A_TO_A: u32 = 3193;
pub const GL_PIXEL_MAP_A_TO_A_SIZE: u32 = 3257;
pub const GL_PIXEL_MAP_B_TO_B: u32 = 3192;
pub const GL_PIXEL_MAP_B_TO_B_SIZE: u32 = 3256;
pub const GL_PIXEL_MAP_G_TO_G: u32 = 3191;
pub const GL_PIXEL_MAP_G_TO_G_SIZE: u32 = 3255;
pub const GL_PIXEL_MAP_I_TO_A: u32 = 3189;
pub const GL_PIXEL_MAP_I_TO_A_SIZE: u32 = 3253;
pub const GL_PIXEL_MAP_I_TO_B: u32 = 3188;
pub const GL_PIXEL_MAP_I_TO_B_SIZE: u32 = 3252;
pub const GL_PIXEL_MAP_I_TO_G: u32 = 3187;
pub const GL_PIXEL_MAP_I_TO_G_SIZE: u32 = 3251;
pub const GL_PIXEL_MAP_I_TO_I: u32 = 3184;
pub const GL_PIXEL_MAP_I_TO_I_SIZE: u32 = 3248;
pub const GL_PIXEL_MAP_I_TO_R: u32 = 3186;
pub const GL_PIXEL_MAP_I_TO_R_SIZE: u32 = 3250;
pub const GL_PIXEL_MAP_R_TO_R: u32 = 3190;
pub const GL_PIXEL_MAP_R_TO_R_SIZE: u32 = 3254;
pub const GL_PIXEL_MAP_S_TO_S: u32 = 3185;
pub const GL_PIXEL_MAP_S_TO_S_SIZE: u32 = 3249;
pub const GL_PIXEL_MODE_BIT: u32 = 32;
pub const GL_POINT: u32 = 6912;
pub const GL_POINTS: u32 = 0;
pub const GL_POINT_BIT: u32 = 2;
pub const GL_POINT_SIZE: u32 = 2833;
pub const GL_POINT_SIZE_GRANULARITY: u32 = 2835;
pub const GL_POINT_SIZE_RANGE: u32 = 2834;
pub const GL_POINT_SMOOTH: u32 = 2832;
pub const GL_POINT_SMOOTH_HINT: u32 = 3153;
pub const GL_POINT_TOKEN: u32 = 1793;
pub const GL_POLYGON: u32 = 9;
pub const GL_POLYGON_BIT: u32 = 8;
pub const GL_POLYGON_MODE: u32 = 2880;
pub const GL_POLYGON_OFFSET_FACTOR: u32 = 32824;
pub const GL_POLYGON_OFFSET_FILL: u32 = 32823;
pub const GL_POLYGON_OFFSET_LINE: u32 = 10754;
pub const GL_POLYGON_OFFSET_POINT: u32 = 10753;
pub const GL_POLYGON_OFFSET_UNITS: u32 = 10752;
pub const GL_POLYGON_SMOOTH: u32 = 2881;
pub const GL_POLYGON_SMOOTH_HINT: u32 = 3155;
pub const GL_POLYGON_STIPPLE: u32 = 2882;
pub const GL_POLYGON_STIPPLE_BIT: u32 = 16;
pub const GL_POLYGON_TOKEN: u32 = 1795;
pub const GL_POSITION: u32 = 4611;
pub const GL_PROJECTION: u32 = 5889;
pub const GL_PROJECTION_MATRIX: u32 = 2983;
pub const GL_PROJECTION_STACK_DEPTH: u32 = 2980;
pub const GL_PROXY_TEXTURE_1D: u32 = 32867;
pub const GL_PROXY_TEXTURE_2D: u32 = 32868;
pub const GL_Q: u32 = 8195;
pub const GL_QUADRATIC_ATTENUATION: u32 = 4617;
pub const GL_QUADS: u32 = 7;
pub const GL_QUAD_STRIP: u32 = 8;
pub const GL_R: u32 = 8194;
pub const GL_R3_G3_B2: u32 = 10768;
pub const GL_READ_BUFFER: u32 = 3074;
pub const GL_RED: u32 = 6403;
pub const GL_RED_BIAS: u32 = 3349;
pub const GL_RED_BITS: u32 = 3410;
pub const GL_RED_SCALE: u32 = 3348;
pub const GL_RENDER: u32 = 7168;
pub const GL_RENDERER: u32 = 7937;
pub const GL_RENDER_MODE: u32 = 3136;
pub const GL_REPEAT: u32 = 10497;
pub const GL_REPLACE: u32 = 7681;
pub const GL_RETURN: u32 = 258;
pub const GL_RGB: u32 = 6407;
pub const GL_RGB10: u32 = 32850;
pub const GL_RGB10_A2: u32 = 32857;
pub const GL_RGB12: u32 = 32851;
pub const GL_RGB16: u32 = 32852;
pub const GL_RGB4: u32 = 32847;
pub const GL_RGB5: u32 = 32848;
pub const GL_RGB5_A1: u32 = 32855;
pub const GL_RGB8: u32 = 32849;
pub const GL_RGBA: u32 = 6408;
pub const GL_RGBA12: u32 = 32858;
pub const GL_RGBA16: u32 = 32859;
pub const GL_RGBA2: u32 = 32853;
pub const GL_RGBA4: u32 = 32854;
pub const GL_RGBA8: u32 = 32856;
pub const GL_RGBA_MODE: u32 = 3121;
pub const GL_RIGHT: u32 = 1031;
pub const GL_S: u32 = 8192;
pub const GL_SCISSOR_BIT: u32 = 524288;
pub const GL_SCISSOR_BOX: u32 = 3088;
pub const GL_SCISSOR_TEST: u32 = 3089;
pub const GL_SELECT: u32 = 7170;
pub const GL_SELECTION_BUFFER_POINTER: u32 = 3571;
pub const GL_SELECTION_BUFFER_SIZE: u32 = 3572;
pub const GL_SET: u32 = 5391;
pub const GL_SHADE_MODEL: u32 = 2900;
pub const GL_SHININESS: u32 = 5633;
pub const GL_SHORT: u32 = 5122;
pub const GL_SMOOTH: u32 = 7425;
pub const GL_SPECULAR: u32 = 4610;
pub const GL_SPHERE_MAP: u32 = 9218;
pub const GL_SPOT_CUTOFF: u32 = 4614;
pub const GL_SPOT_DIRECTION: u32 = 4612;
pub const GL_SPOT_EXPONENT: u32 = 4613;
pub const GL_SRC_ALPHA: u32 = 770;
pub const GL_SRC_ALPHA_SATURATE: u32 = 776;
pub const GL_SRC_COLOR: u32 = 768;
pub const GL_STACK_OVERFLOW: u32 = 1283;
pub const GL_STACK_UNDERFLOW: u32 = 1284;
pub const GL_STENCIL: u32 = 6146;
pub const GL_STENCIL_BITS: u32 = 3415;
pub const GL_STENCIL_BUFFER_BIT: u32 = 1024;
pub const GL_STENCIL_CLEAR_VALUE: u32 = 2961;
pub const GL_STENCIL_FAIL: u32 = 2964;
pub const GL_STENCIL_FUNC: u32 = 2962;
pub const GL_STENCIL_INDEX: u32 = 6401;
pub const GL_STENCIL_PASS_DEPTH_FAIL: u32 = 2965;
pub const GL_STENCIL_PASS_DEPTH_PASS: u32 = 2966;
pub const GL_STENCIL_REF: u32 = 2967;
pub const GL_STENCIL_TEST: u32 = 2960;
pub const GL_STENCIL_VALUE_MASK: u32 = 2963;
pub const GL_STENCIL_WRITEMASK: u32 = 2968;
pub const GL_STEREO: u32 = 3123;
pub const GL_SUBPIXEL_BITS: u32 = 3408;
pub const GL_T: u32 = 8193;
pub const GL_T2F_C3F_V3F: u32 = 10794;
pub const GL_T2F_C4F_N3F_V3F: u32 = 10796;
pub const GL_T2F_C4UB_V3F: u32 = 10793;
pub const GL_T2F_N3F_V3F: u32 = 10795;
pub const GL_T2F_V3F: u32 = 10791;
pub const GL_T4F_C4F_N3F_V4F: u32 = 10797;
pub const GL_T4F_V4F: u32 = 10792;
pub const GL_TEXTURE: u32 = 5890;
pub const GL_TEXTURE_1D: u32 = 3552;
pub const GL_TEXTURE_2D: u32 = 3553;
pub const GL_TEXTURE_ALPHA_SIZE: u32 = 32863;
pub const GL_TEXTURE_BINDING_1D: u32 = 32872;
pub const GL_TEXTURE_BINDING_2D: u32 = 32873;
pub const GL_TEXTURE_BIT: u32 = 262144;
pub const GL_TEXTURE_BLUE_SIZE: u32 = 32862;
pub const GL_TEXTURE_BORDER: u32 = 4101;
pub const GL_TEXTURE_BORDER_COLOR: u32 = 4100;
pub const GL_TEXTURE_COMPONENTS: u32 = 4099;
pub const GL_TEXTURE_COORD_ARRAY: u32 = 32888;
pub const GL_TEXTURE_COORD_ARRAY_COUNT_EXT: u32 = 32907;
pub const GL_TEXTURE_COORD_ARRAY_EXT: u32 = 32888;
pub const GL_TEXTURE_COORD_ARRAY_POINTER: u32 = 32914;
pub const GL_TEXTURE_COORD_ARRAY_POINTER_EXT: u32 = 32914;
pub const GL_TEXTURE_COORD_ARRAY_SIZE: u32 = 32904;
pub const GL_TEXTURE_COORD_ARRAY_SIZE_EXT: u32 = 32904;
pub const GL_TEXTURE_COORD_ARRAY_STRIDE: u32 = 32906;
pub const GL_TEXTURE_COORD_ARRAY_STRIDE_EXT: u32 = 32906;
pub const GL_TEXTURE_COORD_ARRAY_TYPE: u32 = 32905;
pub const GL_TEXTURE_COORD_ARRAY_TYPE_EXT: u32 = 32905;
pub const GL_TEXTURE_ENV: u32 = 8960;
pub const GL_TEXTURE_ENV_COLOR: u32 = 8705;
pub const GL_TEXTURE_ENV_MODE: u32 = 8704;
pub const GL_TEXTURE_GEN_MODE: u32 = 9472;
pub const GL_TEXTURE_GEN_Q: u32 = 3171;
pub const GL_TEXTURE_GEN_R: u32 = 3170;
pub const GL_TEXTURE_GEN_S: u32 = 3168;
pub const GL_TEXTURE_GEN_T: u32 = 3169;
pub const GL_TEXTURE_GREEN_SIZE: u32 = 32861;
pub const GL_TEXTURE_HEIGHT: u32 = 4097;
pub const GL_TEXTURE_INTENSITY_SIZE: u32 = 32865;
pub const GL_TEXTURE_INTERNAL_FORMAT: u32 = 4099;
pub const GL_TEXTURE_LUMINANCE_SIZE: u32 = 32864;
pub const GL_TEXTURE_MAG_FILTER: u32 = 10240;
pub const GL_TEXTURE_MATRIX: u32 = 2984;
pub const GL_TEXTURE_MIN_FILTER: u32 = 10241;
pub const GL_TEXTURE_PRIORITY: u32 = 32870;
pub const GL_TEXTURE_RED_SIZE: u32 = 32860;
pub const GL_TEXTURE_RESIDENT: u32 = 32871;
pub const GL_TEXTURE_STACK_DEPTH: u32 = 2981;
pub const GL_TEXTURE_WIDTH: u32 = 4096;
pub const GL_TEXTURE_WRAP_S: u32 = 10242;
pub const GL_TEXTURE_WRAP_T: u32 = 10243;
pub const GL_TRANSFORM_BIT: u32 = 4096;
pub const GL_TRIANGLES: u32 = 4;
pub const GL_TRIANGLE_FAN: u32 = 6;
pub const GL_TRIANGLE_STRIP: u32 = 5;
pub const GL_TRUE: u32 = 1;
pub const GL_UNPACK_ALIGNMENT: u32 = 3317;
pub const GL_UNPACK_LSB_FIRST: u32 = 3313;
pub const GL_UNPACK_ROW_LENGTH: u32 = 3314;
pub const GL_UNPACK_SKIP_PIXELS: u32 = 3316;
pub const GL_UNPACK_SKIP_ROWS: u32 = 3315;
pub const GL_UNPACK_SWAP_BYTES: u32 = 3312;
pub const GL_UNSIGNED_BYTE: u32 = 5121;
pub const GL_UNSIGNED_INT: u32 = 5125;
pub const GL_UNSIGNED_SHORT: u32 = 5123;
pub const GL_V2F: u32 = 10784;
pub const GL_V3F: u32 = 10785;
pub const GL_VENDOR: u32 = 7936;
pub const GL_VERSION: u32 = 7938;
pub const GL_VERSION_1_1: u32 = 1;
pub const GL_VERTEX_ARRAY: u32 = 32884;
pub const GL_VERTEX_ARRAY_COUNT_EXT: u32 = 32893;
pub const GL_VERTEX_ARRAY_EXT: u32 = 32884;
pub const GL_VERTEX_ARRAY_POINTER: u32 = 32910;
pub const GL_VERTEX_ARRAY_POINTER_EXT: u32 = 32910;
pub const GL_VERTEX_ARRAY_SIZE: u32 = 32890;
pub const GL_VERTEX_ARRAY_SIZE_EXT: u32 = 32890;
pub const GL_VERTEX_ARRAY_STRIDE: u32 = 32892;
pub const GL_VERTEX_ARRAY_STRIDE_EXT: u32 = 32892;
pub const GL_VERTEX_ARRAY_TYPE: u32 = 32891;
pub const GL_VERTEX_ARRAY_TYPE_EXT: u32 = 32891;
pub const GL_VIEWPORT: u32 = 2978;
pub const GL_VIEWPORT_BIT: u32 = 2048;
pub const GL_WIN_draw_range_elements: u32 = 1;
pub const GL_WIN_swap_hint: u32 = 1;
pub const GL_XOR: u32 = 5382;
pub const GL_ZERO: u32 = 0;
pub const GL_ZOOM_X: u32 = 3350;
pub const GL_ZOOM_Y: u32 = 3351;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct GLbitfield(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct GLboolean(pub u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct GLbyte(pub i8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct GLenum(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct GLint(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct GLshort(pub i16);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct GLsizei(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct GLubyte(pub u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct GLuint(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct GLushort(pub u16);
pub type GLvoid = core::ffi::c_void;
pub type PFNGLADDSWAPHINTRECTWINPROC = Option<unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei)>;
pub type PFNGLARRAYELEMENTARRAYEXTPROC = Option<unsafe extern "system" fn(mode: GLenum, count: GLsizei, pi: *const GLvoid)>;
pub type PFNGLARRAYELEMENTEXTPROC = Option<unsafe extern "system" fn(i: GLint)>;
pub type PFNGLCOLORPOINTEREXTPROC = Option<unsafe extern "system" fn(size: GLint, r#type: GLenum, stride: GLsizei, count: GLsizei, pointer: *const GLvoid)>;
pub type PFNGLCOLORSUBTABLEEXTPROC = Option<unsafe extern "system" fn(target: GLenum, start: GLsizei, count: GLsizei, format: GLenum, r#type: GLenum, data: *const GLvoid)>;
pub type PFNGLCOLORTABLEEXTPROC = Option<unsafe extern "system" fn(target: GLenum, internalformat: GLenum, width: GLsizei, format: GLenum, r#type: GLenum, data: *const GLvoid)>;
pub type PFNGLDRAWARRAYSEXTPROC = Option<unsafe extern "system" fn(mode: GLenum, first: GLint, count: GLsizei)>;
pub type PFNGLDRAWRANGEELEMENTSWINPROC = Option<unsafe extern "system" fn(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, r#type: GLenum, indices: *const GLvoid)>;
pub type PFNGLEDGEFLAGPOINTEREXTPROC = Option<unsafe extern "system" fn(stride: GLsizei, count: GLsizei, pointer: *const GLboolean)>;
pub type PFNGLGETCOLORTABLEEXTPROC = Option<unsafe extern "system" fn(target: GLenum, format: GLenum, r#type: GLenum, data: *mut GLvoid)>;
pub type PFNGLGETCOLORTABLEPARAMETERFVEXTPROC = Option<unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut f32)>;
pub type PFNGLGETCOLORTABLEPARAMETERIVEXTPROC = Option<unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint)>;
pub type PFNGLGETPOINTERVEXTPROC = Option<unsafe extern "system" fn(pname: GLenum, params: *mut *mut GLvoid)>;
pub type PFNGLINDEXPOINTEREXTPROC = Option<unsafe extern "system" fn(r#type: GLenum, stride: GLsizei, count: GLsizei, pointer: *const GLvoid)>;
pub type PFNGLNORMALPOINTEREXTPROC = Option<unsafe extern "system" fn(r#type: GLenum, stride: GLsizei, count: GLsizei, pointer: *const GLvoid)>;
pub type PFNGLTEXCOORDPOINTEREXTPROC = Option<unsafe extern "system" fn(size: GLint, r#type: GLenum, stride: GLsizei, count: GLsizei, pointer: *const GLvoid)>;
pub type PFNGLVERTEXPOINTEREXTPROC = Option<unsafe extern "system" fn(size: GLint, r#type: GLenum, stride: GLsizei, count: GLsizei, pointer: *const GLvoid)>;
